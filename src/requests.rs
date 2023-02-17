// TODO: Remove this
#![allow(dead_code, unused_variables)]

use std::{
    cmp::min,
    thread,
    time::{Duration, SystemTime},
};

use crate::error::RequestSchedulerError;

const DEFAULT_REQUEST_AMOUNT: u32 = 1;
const DEFAULT_TIME_PER_REQUEST: Duration = Duration::from_secs(10);
const DEFAULT_NUM_THREADS: u32 = 1;
const MAX_NUM_THREADS: u32 = 10;

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

    fn build(self) -> Result<RequestScheduler, RequestSchedulerError> {
        let request_amount = self.request_amount.unwrap_or(DEFAULT_REQUEST_AMOUNT);

        let time_per_request = match (&self.time_per_request, &self.total_time) {
            (None, None) => DEFAULT_TIME_PER_REQUEST,
            (None, Some(total_time)) => *total_time / request_amount,
            (Some(time_per_request), None) => *time_per_request,
            (Some(time_per_request), Some(_)) => *time_per_request,
        };

        let num_threads = match self.num_threads {
            Some(num_threads) => num_threads,
            None => DEFAULT_NUM_THREADS,
        };

        // Make sure that the number of threads is in [1, `MAX_NUM_THREADS`].
        match num_threads {
            0 => {
                return Err(RequestSchedulerError::InvalidArgument {
                    argument_name: "num_threads".to_owned(),
                    value: "0".to_owned(),
                    message: "You must use at least 1 thread.".to_owned(),
                })
            }
            1..=MAX_NUM_THREADS => (),
            _ => {
                return Err(RequestSchedulerError::InvalidArgument {
                    argument_name: "num_threads".to_owned(),
                    value: format!("{num_threads}"),
                    message: format!("You can't use more than {MAX_NUM_THREADS} threads."),
                })
            }
        }

        Ok(RequestScheduler {
            request_amount,
            time_per_request,
            num_threads,
        })
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RequestScheduler {
    request_amount: u32,
    time_per_request: Duration,
    num_threads: u32,
}

pub(crate) fn send_data(req_scheduler: RequestScheduler, json: String, debug: bool) {
    // TODO: https://github.com/Jim-Hodapp-Coaching/ambi_mock_client/pull/8#pullrequestreview-932531277
    // TODO: Debug logging?

    // If 1 thread is specified, we can use the current thread.
    if req_scheduler.num_threads == 1 {
        send_data_internal(req_scheduler, json, debug, 0);
        return;
    }

    let handles = (0..req_scheduler.num_threads)
        .map(|thread_id| {
            let json_clone = json.clone();
            thread::spawn(move || send_data_internal(req_scheduler, json_clone, debug, thread_id))
        })
        .collect::<Vec<_>>();

    let _result: Vec<_> = handles.into_iter().map(|x| x.join()).collect();
}

fn send_data_internal(req_scheduler: RequestScheduler, json: String, debug: bool, thread_id: u32) {
    // TODO: Actually make the API calls.
    // let client = Client::new();
    let start = SystemTime::now();

    for _ in 0..req_scheduler.request_amount {
        // let res = client
        //     .post(URL)
        //     .header(CONTENT_TYPE, "application/json")
        //     .body(json.clone())
        //     .send();

        // TODO: Discuss what should be logged (verbose or not) and whether we should use a logger.
        //       A logger would make the info/verbose level separation a bit cleaner. I've used
        //       <https://github.com/rust-cli/env_logger> in the past.
        println!("[Thread {}]: {:?}", thread_id, SystemTime::elapsed(&start));

        thread::sleep(req_scheduler.time_per_request)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{send_data, RequestSchedulerBuilder, MAX_NUM_THREADS};

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

        assert!(req_scheduler.is_ok());
        let req_scheduler = req_scheduler.unwrap();

        send_data(req_scheduler, "{}".to_owned(), true);
    }

    #[test]
    fn test_invalid_num_threads_low() {
        let req_scheduler = RequestSchedulerBuilder::default()
            .with_num_threads(0)
            .build();

        assert!(req_scheduler.is_err())
    }

    #[test]
    fn test_invalid_num_threads_high() {
        let req_scheduler = RequestSchedulerBuilder::default()
            .with_num_threads(MAX_NUM_THREADS + 1)
            .build();

        assert!(req_scheduler.is_err())
    }
}
