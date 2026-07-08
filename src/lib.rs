#![warn(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
#![warn(clippy::use_self)]
// #![warn(missing_docs)]

//! Gvas
//!
//! UE4+ Save File parsing library
//!
//! # Examples
//!
//! ```
//! use std::{fs::File, io::{Cursor, Read}, path::Path};
//!
//! use binrw::BinRead;
//! use gvas::types::USaveGame;
//! use gvas::error::{Error, Result};
//!
//! let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot1.sav");
//! let mut file = File::open(path)?;
//! let mut buf = Vec::new();
//! let len = file.read_to_end(&mut buf)?;
//! let mut cursor = Cursor::new(buf);
//! let result = USaveGame::read(&mut cursor)?;
//!
//! println!("{:#?}", result);
//! # Ok::<(), Error>(())
//! ```

pub mod detect;
pub mod error;
pub mod format;
pub mod palworld;
pub(crate) mod serde;
pub mod types;

mod test;
