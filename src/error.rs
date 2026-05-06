use std::io;

use thiserror::Error;

/// A wrapper for the various error types this crate can emit
#[derive(Error, Debug)]
pub enum Error {
    /// A `binrw::Error` occurred
    #[error(transparent)]
    Binrw(#[from] binrw::Error),

    /// An `std::io::Error` occured
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
