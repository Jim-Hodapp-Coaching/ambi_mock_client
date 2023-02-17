use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestSchedulerError {
    #[error("Invalid value \"{value}\" for argument {argument_name}. {message}")]
    InvalidArgument {
        argument_name: String,
        value: String,
        message: String,
    },
}
