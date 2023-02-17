#![allow(dead_code)]

use std::{
    thread,
    time::{Duration, SystemTime},
};

use reqwest::blocking::Client;

const DEFAULT_REQUEST_AMOUNT: u32 = 1;
const DEFAULT_TIME_PER_REQUEST: Duration = Duration::from_secs(10);
const DEFAULT_NUM_THREADS: u32 = 1;

#[derive(Clone, Copy)]
struct RequestSchedulerBuilder {
    request_amount: Option<u32>,
    time_per_request: Option<Duration>,
    total_time: Option<Duration>,
    num_threads: Option<u32>,
}

impl RequestSchedulerBuilder {
    fn default() -> Self {
        RequestSchedulerBuilder {
            request_amount: None,
            time_per_request: None,
            total_time: None,
            num_threads: None,
        }
    }

    fn with_request_amount(&mut self, request_amount: u32) -> &mut Self {
        self.request_amount = Some(request_amount);
        self
    }

    fn with_time_per_request(&mut self, time_per_request: Duration) -> &mut Self {
        self.time_per_request = Some(time_per_request);
        self
    }

    fn with_total_time(&mut self, total_time: Duration) -> &mut Self {
        self.total_time = Some(total_time);
        self
    }

    fn with_num_threads(&mut self, num_threads: u32) -> &mut Self {
        self.num_threads = Some(num_threads);
        self
    }

    fn build(self) -> RequestScheduler {
        let request_amount = self.request_amount.unwrap_or(DEFAULT_REQUEST_AMOUNT);

        let time_per_request = match (&self.time_per_request, &self.total_time) {
            (None, None) => DEFAULT_TIME_PER_REQUEST,
            (None, Some(total_time)) => *total_time / request_amount,
            (Some(time_per_request), None) => *time_per_request,
            (Some(time_per_request), Some(_)) => *time_per_request,
        };

        // TODO: Come up with a reasonable upper limit.
        let num_threads = self.num_threads.unwrap_or(DEFAULT_NUM_THREADS);

        RequestScheduler {
            request_amount,
            time_per_request,
            num_threads,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RequestScheduler {
    request_amount: u32,
    time_per_request: Duration,
    num_threads: u32,
}

/// -s 5: 1 req every 5 seconds.
pub(crate) fn send_data(req_scheduler: RequestScheduler, json: String) {
    // TODO: https://github.com/Jim-Hodapp-Coaching/ambi_mock_client/pull/8#pullrequestreview-932531277
    // TODO: Debug logging?
    let handles = (0..req_scheduler.num_threads)
        .map(|_| {
            let json_clone = json.clone();
            thread::spawn(move || send_data_internal(req_scheduler, json_clone))
        })
        .collect::<Vec<_>>();

    let _result: Vec<_> = handles.into_iter().map(|x| x.join()).collect();
}

fn send_data_internal(req_scheduler: RequestScheduler, json: String) {
    // TODO: Actually make the API calls.
    let client = Client::new();
    let start = SystemTime::now();

    for _ in 0..req_scheduler.request_amount {
        // let res = client
        //     .post(URL)
        //     .header(CONTENT_TYPE, "application/json")
        //     .body(json.clone())
        //     .send();

        println!("{:?}", SystemTime::elapsed(&start));

        thread::sleep(req_scheduler.time_per_request)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{send_data, RequestSchedulerBuilder};

    // Used for manual testing, will be removed/edited later
    // cargo t -- send_data_test --nocapture
    #[test]
    fn send_data_test() {
        let req_scheduler = RequestSchedulerBuilder::default()
            .with_num_threads(2)
            .with_request_amount(5)
            .with_time_per_request(Duration::from_secs(1))
            // .with_total_time(Duration::from_secs(1))
            .build();

        send_data(req_scheduler, "{}".to_owned());
    }
}
