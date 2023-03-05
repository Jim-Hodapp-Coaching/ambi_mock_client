use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostSchedulerError {
    #[error("Invalid value \"{value}\" for argument {argument_name}. {message}")]
    InvalidArgument {
        argument_name: String,
        value: String,
        message: String,
    },
}
