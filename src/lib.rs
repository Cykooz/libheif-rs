// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
extern crate core;

pub use color_profile::*;
pub use context::HeifContext;
pub use decoder::*;
pub use encoder::*;
pub use enums::*;
pub use errors::{HeifError, HeifErrorCode, HeifErrorSubCode, Result};
pub use heif::*;
pub use image::*;
pub use image_handle::{AuxiliaryImagesFilter, ImageHandle, ItemId};
pub use metadata::ImageMetadata;
pub use reader::{Reader, StreamReader};
#[cfg(feature = "v1_19")]
pub use security_limits::*;
pub use utils::check_file_type;
mod color_profile;
mod context;
mod decoder;
mod encoder;
mod enums;
mod errors;
mod heif;
mod image;
mod image_handle;
#[cfg(feature = "image")]
pub mod integration;
mod metadata;
mod reader;
#[cfg(feature = "v1_18")]
pub mod regions;
#[cfg(feature = "v1_19")]
mod security_limits;
mod utils;
