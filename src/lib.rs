#![warn(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::use_self)]
// #![warn(missing_docs)]
#![warn(unreachable_pub)]

//! Gvas
//!
//! UE4+ save game object parsing library.
//!
//! This library includes faithful recrations in rust of the named structs needed to serialize and
//! deserialize Unreal Engine USaveGame objects.
//!
//! # Examples
//!
//! The example code below demonstrates how to use the gvas crate to read a gvas save file. The
//! `USaveGame` struct's `BinRead::read` implementation is used to deserialize a data stream into a
//! USaveGame struct.
//!
//! ```
//! # use std::{fs::File, io::{Cursor, Read}, path::Path};
//! #
//! # use binrw::BinRead;
//! # use gvas::types::USaveGame;
//! # use gvas::error::{Error, Result};
//! #
//! let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot1.sav");
//! let mut file = File::open(path)?;
//! let result = USaveGame::read(&mut file)?;
//!
//! println!("{:#?}", result);
//! # Ok::<(), Error>(())
//! ```
//!
//! To enable support for Palworld compressed save files, enable the `palworld` feature by running
//! `cargo add gvas --features palworld`, and use the `PalworldSaveGame` wrapper to access the
//! compressed `USaveGame` object.
//!
//! ```
//! # use gvas::error::Error;
//! # #[cfg(feature = "palworld")]
//! # {
//! # use binrw::BinRead;
//! # use gvas::palworld::PalworldSaveGame;
//! # use std::fs::File;
//! # use std::path::Path;
//! #
//! # let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/palworld_zlib.sav");
//! let mut file = File::open(path)?;
//! let palworld_save_game = PalworldSaveGame::read_le(&mut file)?;
//! let save_game = palworld_save_game.content;
//!
//! println!("{:#?}", save_game);
//! # }
//! # Ok::<(), Error>(())
//! ```
//!
//! Alternatively, use the `AutoDetectFile` struct to support any known `USaveGame`
//! object container type.
//!
//! ```
//! # use gvas::error::Error;
//! # #[cfg(feature = "palworld")]
//! # {
//! # use binrw::BinRead;
//! # use gvas::detect::AutoDetectFile;
//! # use std::fs::File;
//! # use std::path::Path;
//! #
//! # let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/palworld_zlib.sav");
//! let mut file = File::open(path)?;
//! let auto_detect_file = AutoDetectFile::read(&mut file)?;
//!
//! std::assert_matches!(auto_detect_file, AutoDetectFile::Palworld(_));
//! # }
//! # Ok::<(), Error>(())
//! ```
//!
//! ## Hints
//!
//! Prior to Unreal Engine 5.4, save game files contained limited type information about container
//! sub-types, preventing this library from inferring types for old save game objects under certain
//! conditions, such as when as `MapProperty` contains a `StructProperty`. In previous versions of
//! this library, type hints could be provided to allow transparent type inference. Support for this
//! feature was removed to make way for complete types, but the functionality may be restored in a
//! future version. In the event that a type hint is missing, the opaque type [`FProperty::Unknown`]
//! will be used.

pub mod detect;
pub mod error;
pub mod format;
pub mod hints;
/// Palworld save wrapper support.
///
/// Palworld save files contain a 12-byte header followed by a [`USaveGame`]
/// which may be Zlib-deflated.
#[cfg(feature = "palworld")]
pub mod palworld;
#[cfg(feature = "serde")]
pub(crate) mod serde;
#[cfg(test)]
mod test;
pub mod types;
