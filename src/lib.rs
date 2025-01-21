#![doc = include_str!("../README.md")]

pub use color_profile::*;
pub use context::HeifContext;
pub use decoder::*;
pub use encoder::*;
pub use enums::*;
pub use errors::{HeifError, HeifErrorCode, HeifErrorSubCode, Result};
pub use heif::*;
pub use image::*;
pub use image_handle::{ImageHandle, ItemId};
pub use metadata::ImageMetadata;
pub use reader::{Reader, StreamReader};
pub use utils::check_file_type;
pub use auxiliary_image_handle::AuxiliaryImageHandle;

mod color_profile;
mod context;
mod decoder;
mod encoder;
mod enums;
mod errors;
mod heif;
mod image;
mod image_handle;
mod metadata;
mod reader;
mod utils;
mod auxiliary_image_handle;
