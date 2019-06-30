extern crate failure;

use crate::error::KvError::{IoError, SerdeError};
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
    #[fail(display = "Serde error: {}", error)]
    SerdeError {
        #[cause]
        error: serde_json::Error,
    },
}

impl From<io::Error> for KvError {
    fn from(io_error: io::Error) -> KvError {
        IoError { error: io_error }
    }
}

impl From<serde_json::Error> for KvError {
    fn from(serde_error: serde_json::Error) -> KvError {
        SerdeError { error: serde_error }
    }
}

pub type Result<T> = result::Result<T, KvError>;
