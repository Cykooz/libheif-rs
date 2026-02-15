use four_cc::FourCC;
use libheif_sys as lh;
use std::ptr;

use crate::{
    ColorSpace, HeifError, Result, DecodingOptions, Image
};
use crate::decoder::get_decoding_options_ptr;

pub type TrackType = FourCC;

pub mod track_types {
    use super::{TrackType, FourCC};

    pub const VIDEO: TrackType = FourCC(*b"vide");
    pub const IMAGE_SEQUENCE: TrackType = FourCC(*b"pict");
    pub const AUXILIARY: TrackType = FourCC(*b"auxv");
    pub const METADATA: TrackType = FourCC(*b"meta");
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ImageResolution {
    pub width: u16,
    pub height: u16,
}

pub struct Track {
    pub(crate) inner: *mut lh::heif_track,
}

impl Track {
    #[inline]
    pub(crate) fn from_heif_track(track: *mut lh::heif_track) -> Track {
        Track { inner: track }
    }

    pub fn id(&self) -> u32 {
        unsafe { lh::heif_track_get_id(self.inner) }
    }

    pub fn track_handler_type(&self) -> TrackType {
        let c_track_type = unsafe { lh::heif_track_get_track_handler_type(self.inner) };
        TrackType::from(c_track_type as u32)
    }

    #[cfg(feature = "v1_21")]
    pub fn has_alpha_channel(&self) -> bool {
        unsafe { lh::heif_track_has_alpha_channel(self.inner) != 0 }
    }

    pub fn timescale(&self) -> u32 {
        unsafe { lh::heif_track_get_timescale(self.inner) }
    }

    pub fn image_resolution(&self) -> Result<ImageResolution> {
        let mut res = ImageResolution::default();

        let err = unsafe {
            lh::heif_track_get_image_resolution(self.inner, &mut res.width, &mut res.height)
        };

        HeifError::from_heif_error(err)?;
        Ok(res)
    }

    pub fn decode_next_image(
        &self,
        color_space: ColorSpace,
        decoding_options: Option<DecodingOptions>
    ) -> Result<Image> {
        let mut c_image: *mut lh::heif_image = ptr::null_mut();
        let err = unsafe {
            lh::heif_track_decode_next_image(
                self.inner,
                &mut c_image,
                color_space.heif_color_space(),
                color_space.heif_chroma(),
                get_decoding_options_ptr(&decoding_options),
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Image::from_heif_image(c_image))
    }

}

impl Drop for Track {
    fn drop(&mut self) {
        unsafe { lh::heif_track_release(self.inner) };
    }
}