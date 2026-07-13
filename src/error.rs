//! Error types.

use std::{
    io,
    num::{ParseIntError, TryFromIntError},
};

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
    /// A [`TryFromIntError`] occureed.
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
}

/// An error ocurred while parsing a Guid
#[derive(Debug, Error)]
pub enum ParseGuidError {
    #[error("invalid guid length {0}")]
    InvalidLength(usize),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

/// An error ocurred while decribing a PropertyTag.
#[derive(Debug, Error)]
pub enum PropertyTagError {
    #[error("Unsupported method PropertyTag::{0} for {1}")]
    Unsupported(Box<str>, String),
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
