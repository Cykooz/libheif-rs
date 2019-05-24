use std::mem;
use std::ptr;
use std::slice;

use libheif_sys::*;

use crate::enums::*;
use crate::HeifError;

pub struct Image {
    pub(crate) inner: *mut heif_image,
}

pub struct ScalingOptions {}

impl Image {
    pub fn new(
        width: i32,
        height: i32,
        colorspace: ColorSpace,
        chroma: Chroma,
    ) -> Result<Image, HeifError> {
        let mut image = Image {
            inner: unsafe { mem::uninitialized() },
        };
        let err = unsafe {
            heif_image_create(
                width,
                height,
                colorspace as _,
                chroma as _,
                &mut image.inner,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(image)
    }

    pub fn width(&self, channel: Channel) -> i32 {
        unsafe { heif_image_get_width(self.inner, channel as _) }
    }

    pub fn height(&self, channel: Channel) -> i32 {
        unsafe { heif_image_get_height(self.inner, channel as _) }
    }

    pub fn get_bits_per_pixel(&self, channel: Channel) -> i32 {
        unsafe { heif_image_get_bits_per_pixel(self.inner, channel as _) }
    }

    pub fn has_channel(&self, channel: Channel) -> bool {
        unsafe { heif_image_has_channel(self.inner, channel as _) != 0 }
    }

    pub fn get_chroma_format(&self) -> Chroma {
        unsafe { mem::transmute(heif_image_get_chroma_format(self.inner)) }
    }

    pub fn get_color_space(&self) -> ColorSpace {
        unsafe { mem::transmute(heif_image_get_colorspace(self.inner)) }
    }

    /// Scale image by "nearest neighbor" method.
    pub fn scale(
        &self,
        width: i32,
        height: i32,
        _scaling_options: Option<ScalingOptions>,
    ) -> Result<Image, HeifError> {
        let mut image = unsafe { mem::uninitialized() };
        let err =
            unsafe { heif_image_scale_image(self.inner, &mut image, width, height, ptr::null()) };
        HeifError::from_heif_error(err)?;
        Ok(Image { inner: image })
    }

    pub fn add_plane(
        &mut self,
        channel: Channel,
        width: i32,
        height: i32,
        bit_depth: i32,
    ) -> Result<(), HeifError> {
        let err = unsafe {
            heif_image_add_plane(
                self.inner,
                channel as _,
                width as _,
                height as _,
                bit_depth as _,
            )
        };
        HeifError::from_heif_error(err)
    }

    pub fn get_plane_mut(&mut self, channel: Channel) -> (&mut [u8], i32) {
        let mut stride: i32 = 1;
        let data = unsafe { heif_image_get_plane(self.inner, channel as _, &mut stride) };
        let height = self.height(channel) as usize;
        let size = height * (stride as usize);
        let bytes = unsafe { slice::from_raw_parts_mut(data, size) };
        (bytes, stride)
    }

    pub fn get_plane(&self, channel: Channel) -> (&[u8], i32) {
        let mut stride: i32 = 1;
        let data = unsafe { heif_image_get_plane_readonly(self.inner, channel as _, &mut stride) };
        let height = self.height(channel) as usize;
        let size = height * (stride as usize);
        let bytes = unsafe { slice::from_raw_parts(data, size) };
        (bytes, stride)
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
