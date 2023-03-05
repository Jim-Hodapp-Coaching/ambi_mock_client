use std::{thread, time::Duration};

use log::{debug, error, info};
use rand::rngs::ThreadRng;
use reqwest::{blocking::Client, header::CONTENT_TYPE};

use crate::{
    data::{
        random_gen_dust_concentration, random_gen_humidity, random_gen_pressure,
        random_gen_temperature, AirPurity, Reading,
    },
    error::PostSchedulerError,
    URL,
};

const DEFAULT_REQUEST_AMOUNT: u32 = 1;
const DEFAULT_TIME_PER_REQUEST: Duration = Duration::from_secs(10);
const DEFAULT_NUM_THREADS: u32 = 1;
pub const MAX_NUM_THREADS: u32 = 10;

#[derive(Clone, Copy)]
pub struct PostSchedulerBuilder {
    request_amount: Option<u32>,
    time_per_request: Option<Duration>,
    total_time: Option<Duration>,
    num_threads: Option<u32>,
}

impl PostSchedulerBuilder {
    pub fn default() -> Self {
        PostSchedulerBuilder {
            request_amount: None,
            time_per_request: None,
            total_time: None,
            num_threads: None,
        }
    }

    pub fn with_some_request_amount(mut self, request_amount: &Option<u32>) -> Self {
        self.request_amount = *request_amount;
        self
    }

    pub fn with_some_time_per_request(mut self, time_per_request: &Option<Duration>) -> Self {
        self.time_per_request = *time_per_request;
        self
    }

    pub fn with_some_total_time(mut self, total_time: &Option<Duration>) -> Self {
        self.total_time = *total_time;
        self
    }

    pub fn with_some_num_threads(mut self, num_threads: &Option<u32>) -> Self {
        self.num_threads = *num_threads;
        self
    }

    pub fn build(self) -> Result<PostScheduler, PostSchedulerError> {
        // Loop indefinitely if no req amt is set. If time per req is also not set then don't loop.
        let loop_indefinitely = self.request_amount.is_none() && self.time_per_request.is_some();

        let request_amount = self.request_amount.unwrap_or(DEFAULT_REQUEST_AMOUNT);

        let time_per_request = match (&self.time_per_request, &self.total_time) {
            (None, None) => DEFAULT_TIME_PER_REQUEST,
            (None, Some(total_time)) => *total_time / request_amount,
            (Some(time_per_request), None) => *time_per_request,
            (Some(time_per_request), Some(_)) => *time_per_request,
        };

        let num_threads = self.num_threads.unwrap_or(DEFAULT_NUM_THREADS);
        // At this point we know that the number of threads is in [1, `MAX_NUM_THREADS`].
        // Validation is done in `lib::is_valid_num_of_threads`.

        Ok(PostScheduler {
            request_amount,
            time_per_request,
            num_threads,
            loop_indefinitely,
        })
    }
}

#[derive(Clone, Copy)]
pub struct PostScheduler {
    request_amount: u32,
    time_per_request: Duration,
    num_threads: u32,
    loop_indefinitely: bool,
}

pub fn send_data(req_scheduler: PostScheduler) {
    // If 1 thread is specified, we can use the current thread.
    if req_scheduler.num_threads == 1 {
        debug!("num_threads is set to 1, use current thread.");

        send_data_internal(req_scheduler, 0, Client::new(), &mut rand::thread_rng());
        return;
    }

    debug!("Spawning {} threads.", req_scheduler.num_threads);

    let handles = (0..req_scheduler.num_threads)
        .map(|thread_id| {
            thread::spawn(move || {
                send_data_internal(
                    req_scheduler,
                    thread_id,
                    Client::new(),
                    &mut rand::thread_rng(),
                )
            })
        })
        .collect::<Vec<_>>();

    debug!("Threads spawned.");

    let _result: Vec<_> = handles.into_iter().map(|x| x.join()).collect();

    debug!("Threads joined.");
}

/// This code might run on separate threads, any logs should be prefixed by the thread id
/// for easier debugging.
///
/// You can log the thread id by prepending `[Thread {thread_id}]: ` to your logs.
fn send_data_internal(
    req_scheduler: PostScheduler,
    thread_id: u32,
    client: Client,
    rng: &mut ThreadRng,
) {
    if req_scheduler.loop_indefinitely {
        loop {
            make_request(thread_id, &client, rng);
            thread::sleep(req_scheduler.time_per_request)
        }
    }

    for i in 0..req_scheduler.request_amount {
        make_request(thread_id, &client, rng);

        // Only use thread.sleep if we are not on the last request
        if i != req_scheduler.request_amount - 1 {
            thread::sleep(req_scheduler.time_per_request)
        }
    }
}

/// This also can run in parallel, refer to [`send_data_internal`].
fn make_request(thread_id: u32, client: &Client, rng: &mut ThreadRng) {
    let json = generate_random_reading(rng);
    info!("[Thread {thread_id}]: Sending POST request to {}", URL);
    debug!("[Thread {thread_id}]: Request JSON: {}", json);

    let res = client
        .post(URL)
        .header(CONTENT_TYPE, "application/json")
        .body(json)
        .send();

    match res {
        Ok(response) => {
            info!(
                "[Thread {thread_id}]: Response from Ambi backend: {}",
                response.status().as_str()
            );
            debug!(
                "[Thread {thread_id}]: Response from Ambi backend: {:#?}",
                response
            );
        }
        Err(e) => {
            if e.is_request() {
                error!("[Thread {thread_id}]: Response error from Ambi backend: request error");
            } else if e.is_timeout() {
                error!("[Thread {thread_id}]: Response error from Ambi backend: request timed out");
            } else {
                error!("[Thread {thread_id}]: Response error from Ambi backend: specific error type unknown");
            }

            debug!("[Thread {thread_id}]: {}", e.to_string());
            debug!(
                "[Thread {thread_id}]: Response error from Ambi backend: {:?}",
                e
            );
        }
    }
}

fn generate_random_reading(rng: &mut ThreadRng) -> String {
    let dust_concentration = random_gen_dust_concentration(rng);
    let air_purity = AirPurity::from_value(dust_concentration).to_string();
    let reading = Reading::new(
        random_gen_temperature(rng),
        random_gen_humidity(rng),
        random_gen_pressure(rng),
        dust_concentration,
        air_purity,
    );

    serde_json::to_string(&reading).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{PostSchedulerBuilder, MAX_NUM_THREADS};

    // TODO: Add some tests for the parameter logic.

    #[test]
    fn test_invalid_num_threads_low() {
        let req_scheduler = PostSchedulerBuilder::default()
            .with_some_num_threads(&Some(0))
            .build();

        assert!(req_scheduler.is_err())
    }

    #[test]
    fn test_invalid_num_threads_high() {
        let req_scheduler = PostSchedulerBuilder::default()
            .with_some_num_threads(&Some(MAX_NUM_THREADS + 1))
            .build();

        assert!(req_scheduler.is_err())
    }
}
