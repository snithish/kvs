#![crate_type = "lib"]

#[macro_use]
extern crate failure_derive;
extern crate serde;

mod error;
mod kv;

pub use error::{KvError, Result};
pub use kv::KvStore;
