//! Error types.

use std::{io, num::ParseIntError};

use thiserror::Error;

/// A type alias for [`core::result::Result<T, Error>`].
pub type Result<T> = core::result::Result<T, Error>;

/// A wrapper for the various error types this crate can emit
#[derive(Debug, Error)]
pub enum Error {
    /// A [`ParseGuidError`] occureed.
    #[error(transparent)]
    ParseGuidError(#[from] ParseGuidError),
    /// An [`std::io::Error`] occured
    #[error(transparent)]
    Io(#[from] io::Error),
    /// A [`binrw::Error`] occured
    #[error(transparent)]
    Binrw(#[from] binrw::Error),
}

/// An error ocurred while parsing a Guid
#[derive(Debug, Error)]
pub enum ParseGuidError {
    #[error("invalid guid length {0}")]
    InvalidLength(usize),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

pub(crate) fn binrw_custom<T>(pos: u64) -> impl Fn(T) -> binrw::Error
where
    T: binrw::error::CustomError + 'static,
{
    move |e| binrw::Error::Custom {
        pos,
        err: Box::new(e),
    }
}
