#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate strum_macros;

pub use context::HeifContext;
pub use encoder::{EncoderParametersTypes, EncodingOptions};
pub use enums::*;
pub use errors::{HeifError, HeifErrorCode, HeifErrorSubCode};
pub use image::Image;
pub use image_handle::ImageHandle;

mod context;
mod encoder;
mod enums;
mod errors;
mod image;
mod image_handle;
mod utils;
