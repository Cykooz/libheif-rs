use std;
use std::mem;
use std::ptr;
use std::slice;

use libheif_sys::*;

use crate::enums::*;
use crate::errors::{HeifError, HeifErrorCode, HeifErrorSubCode};
use std::os::raw::c_int;

const MAX_IMAGE_SIZE: u32 = std::i32::MAX as _;

pub struct Image {
    pub(crate) inner: *mut heif_image,
}

pub struct ScalingOptions {}

impl Image {
    pub fn new(
        width: u32,
        height: u32,
        colorspace: ColorSpace,
        chroma: Chroma,
    ) -> Result<Image, HeifError> {
        if width > MAX_IMAGE_SIZE || height > MAX_IMAGE_SIZE {
            return Err(HeifError {
                code: HeifErrorCode::UsageError,
                sub_code: HeifErrorSubCode::InvalidBoxSize,
                message: "width or height is greater than MAX_IMAGE_SIZE".to_string(),
            });
        }

        let mut image = Image {
            inner: unsafe { mem::uninitialized() },
        };
        let err = unsafe {
            heif_image_create(
                width as _,
                height as _,
                colorspace as _,
                chroma as _,
                &mut image.inner,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(image)
    }

    pub fn width(&self, channel: Channel) -> Result<u32, HeifError> {
        let value = unsafe { heif_image_get_width(self.inner, channel as _) };
        if value >= 0 {
            return Ok(value as _);
        }
        Err(HeifError {
            code: HeifErrorCode::UsageError,
            sub_code: HeifErrorSubCode::NonexistingImageChannelReferenced,
            message: "".to_string(),
        })
    }

    pub fn height(&self, channel: Channel) -> Result<u32, HeifError> {
        let value = unsafe { heif_image_get_height(self.inner, channel as _) };
        if value >= 0 {
            return Ok(value as _);
        }
        Err(HeifError {
            code: HeifErrorCode::UsageError,
            sub_code: HeifErrorSubCode::NonexistingImageChannelReferenced,
            message: "".to_string(),
        })
    }

    pub fn bits_per_pixel(&self, channel: Channel) -> Result<u8, HeifError> {
        let value = unsafe { heif_image_get_bits_per_pixel(self.inner, channel as _) };
        if value >= 0 {
            return Ok(value as _);
        }
        Err(HeifError {
            code: HeifErrorCode::UsageError,
            sub_code: HeifErrorSubCode::NonexistingImageChannelReferenced,
            message: "".to_string(),
        })
    }

    pub fn has_channel(&self, channel: Channel) -> bool {
        unsafe { heif_image_has_channel(self.inner, channel as _) != 0 }
    }

    pub fn chroma_format(&self) -> Chroma {
        unsafe { mem::transmute(heif_image_get_chroma_format(self.inner)) }
    }

    pub fn color_space(&self) -> ColorSpace {
        unsafe { mem::transmute(heif_image_get_colorspace(self.inner)) }
    }

    /// Scale image by "nearest neighbor" method.
    pub fn scale(
        &self,
        width: u32,
        height: u32,
        _scaling_options: Option<ScalingOptions>,
    ) -> Result<Image, HeifError> {
        let mut image = unsafe { mem::uninitialized() };
        let err = unsafe {
            heif_image_scale_image(self.inner, &mut image, width as _, height as _, ptr::null())
        };
        HeifError::from_heif_error(err)?;
        Ok(Image { inner: image })
    }

    pub fn add_plane(
        &mut self,
        channel: Channel,
        width: u32,
        height: u32,
        bit_depth: u8,
    ) -> Result<(), HeifError> {
        let err = unsafe {
            heif_image_add_plane(
                self.inner,
                channel as _,
                width as _,
                height as _,
                c_int::from(bit_depth),
            )
        };
        HeifError::from_heif_error(err)
    }

    pub fn plane_mut(&mut self, channel: Channel) -> Result<(&mut [u8], usize), HeifError> {
        let height = self.height(channel)? as usize;
        let mut stride: i32 = 1;
        let data = unsafe { heif_image_get_plane(self.inner, channel as _, &mut stride) };
        let size = height * (stride as usize);
        let bytes = unsafe { slice::from_raw_parts_mut(data, size) };
        Ok((bytes, stride as usize))
    }

    pub fn plane(&self, channel: Channel) -> Result<(&[u8], usize), HeifError> {
        let height = self.height(channel)? as usize;
        let mut stride: i32 = 1;
        let data = unsafe { heif_image_get_plane_readonly(self.inner, channel as _, &mut stride) };
        let size = height * (stride as usize);
        let bytes = unsafe { slice::from_raw_parts(data, size) };
        Ok((bytes, stride as usize))
    }

    //    TODO: need implement
    //    pub fn set_raw_color_profile(&self) -> Result<(), HeifError> {
    //        let err = unsafe {
    //            heif_image_set_raw_color_profile(self.inner)
    //        };
    //        HeifError::from_heif_error(err)
    //    }
    //
    //    pub fn set_nclx_color_profile(&self) -> Result<(), HeifError> {
    //        let err = unsafe {
    //            heif_image_set_nclx_color_profile(self.inner)
    //        };
    //        HeifError::from_heif_error(err)
    //    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { heif_image_release(self.inner) };
    }
}

#[inline]
pub fn heif_image_2_rs_image(image: *mut heif_image) -> Image {
    Image { inner: image }
}
