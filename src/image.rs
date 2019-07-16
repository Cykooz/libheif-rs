use libheif_sys::*;
use std;
use std::mem;
use std::ptr;
use std::slice;

use libheif_sys as lh;

use crate::enums::{Channel, Chroma, ColorSpace};
use crate::errors::{HeifError, HeifErrorCode, HeifErrorSubCode};

const MAX_IMAGE_SIZE: u32 = std::i32::MAX as _;

pub struct Plane<T> {
    pub data: T,
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub bits_pre_pixel: u8,
}

pub struct Planes<T> {
    pub y: Option<Plane<T>>,
    pub cb: Option<Plane<T>>,
    pub cr: Option<Plane<T>>,
    pub r: Option<Plane<T>>,
    pub g: Option<Plane<T>>,
    pub b: Option<Plane<T>>,
    pub a: Option<Plane<T>>,
    pub interleaved: Option<Plane<T>>,
}

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
            sub_code: HeifErrorSubCode::NonExistingImageChannelReferenced,
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
            sub_code: HeifErrorSubCode::NonExistingImageChannelReferenced,
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
            sub_code: HeifErrorSubCode::NonExistingImageChannelReferenced,
            message: "".to_string(),
        })
    }

    fn plane(&self, channel: Channel) -> Option<Plane<&[u8]>> {
        if !self.has_channel(channel) {
            return None;
        }

        let width = self.width(channel).unwrap();
        let height = self.height(channel).unwrap();
        let bits_pre_pixel = self.bits_per_pixel(channel).unwrap();
        let mut stride: i32 = 1;
        let data = unsafe { heif_image_get_plane(self.inner, channel as _, &mut stride) };
        let size = height as usize * stride as usize;
        let bytes = unsafe { slice::from_raw_parts(data, size) };
        Some(Plane {
            data: bytes,
            bits_pre_pixel,
            width,
            height,
            stride: stride as _,
        })
    }

    fn plane_mut(&self, channel: Channel) -> Option<Plane<&mut [u8]>> {
        if !self.has_channel(channel) {
            return None;
        }

        let width = self.width(channel).unwrap();
        let height = self.height(channel).unwrap();
        let bits_pre_pixel = self.bits_per_pixel(channel).unwrap();
        let mut stride: i32 = 1;
        let data = unsafe { heif_image_get_plane(self.inner, channel as _, &mut stride) };
        let size = height as usize * stride as usize;
        let bytes = unsafe { slice::from_raw_parts_mut(data, size) };
        Some(Plane {
            data: bytes,
            bits_pre_pixel,
            width,
            height,
            stride: stride as _,
        })
    }

    pub fn planes(&self) -> Planes<&[u8]> {
        Planes {
            y: self.plane(Channel::Y),
            cb: self.plane(Channel::Cb),
            cr: self.plane(Channel::Cr),
            r: self.plane(Channel::R),
            g: self.plane(Channel::G),
            b: self.plane(Channel::B),
            a: self.plane(Channel::Alpha),
            interleaved: self.plane(Channel::Interleaved),
        }
    }

    pub fn planes_mut(&mut self) -> Planes<&mut [u8]> {
        Planes {
            y: self.plane_mut(Channel::Y),
            cb: self.plane_mut(Channel::Cb),
            cr: self.plane_mut(Channel::Cr),
            r: self.plane_mut(Channel::R),
            g: self.plane_mut(Channel::G),
            b: self.plane_mut(Channel::B),
            a: self.plane_mut(Channel::Alpha),
            interleaved: self.plane_mut(Channel::Interleaved),
        }
    }

    pub fn has_channel(&self, channel: Channel) -> bool {
        unsafe { heif_image_has_channel(self.inner, channel as _) != 0 }
    }

    //    pub fn channels(&self) -> Vec<Channel> {
    //        let mut res = Vec::from_iter();
    //        for channel in Channel::iter() {
    //            if self.has_channel(channel) {
    //                res.insert(channel);
    //            }
    //        }
    //        res
    //    }

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

    pub fn create_plane(
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
