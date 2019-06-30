extern crate failure;

use crate::error::KvError::IoError;
use core::result;
use std::io;

#[derive(Fail, Debug)]
pub enum KvError {
    #[fail(display = "IO error: {}", error)]
    IoError {
        #[cause]
        error: io::Error,
    },
    #[fail(display = "An unknown error has occurred.")]
    UnknownError,
}

impl From<io::Error> for KvError {
    fn from(io_error: io::Error) -> KvError {
        IoError { error: io_error }
    }
}

pub type Result<T> = result::Result<T, KvError>;
