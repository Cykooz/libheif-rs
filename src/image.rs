use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::ptr;
use std::slice;

use libheif_sys as lh;

use crate::{
    Channel, ColorProfileNCLX, ColorProfileRaw, ColorProfileType, ColorSpace, HeifError,
    HeifErrorCode, HeifErrorSubCode, Result,
};

const MAX_IMAGE_SIZE: u32 = i32::MAX as _;

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
    pub(crate) inner: *mut lh::heif_image,
}

pub struct ScalingOptions {}

impl Image {
    /// Create a new image of the specified resolution and colorspace.
    /// Note: no memory for the actual image data is reserved yet. You have to use
    /// [`Image::create_plane`] method to add image planes required by your colorspace.    
    pub fn new(width: u32, height: u32, color_space: ColorSpace) -> Result<Image> {
        if width > MAX_IMAGE_SIZE || height > MAX_IMAGE_SIZE {
            return Err(HeifError {
                code: HeifErrorCode::UsageError,
                sub_code: HeifErrorSubCode::InvalidBoxSize,
                message: "width or height is greater than MAX_IMAGE_SIZE".to_string(),
            });
        }

        let mut c_image = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_image_create(
                width as _,
                height as _,
                color_space.heif_color_space(),
                color_space.heif_chroma(),
                c_image.as_mut_ptr(),
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Image {
            inner: unsafe { c_image.assume_init() },
        })
    }

    #[inline]
    pub(crate) fn from_heif_image(image: *mut lh::heif_image) -> Image {
        Image { inner: image }
    }

    pub fn width(&self, channel: Channel) -> Result<u32> {
        let value = unsafe { lh::heif_image_get_width(self.inner, channel as _) };
        if value >= 0 {
            return Ok(value as _);
        }
        Err(HeifError {
            code: HeifErrorCode::UsageError,
            sub_code: HeifErrorSubCode::NonExistingImageChannelReferenced,
            message: "".to_string(),
        })
    }

    pub fn height(&self, channel: Channel) -> Result<u32> {
        let value = unsafe { lh::heif_image_get_height(self.inner, channel as _) };
        if value >= 0 {
            return Ok(value as _);
        }
        Err(HeifError {
            code: HeifErrorCode::UsageError,
            sub_code: HeifErrorSubCode::NonExistingImageChannelReferenced,
            message: "".to_string(),
        })
    }

    pub fn bits_per_pixel(&self, channel: Channel) -> Result<u8> {
        let value = unsafe { lh::heif_image_get_bits_per_pixel(self.inner, channel as _) };
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
        let data = unsafe { lh::heif_image_get_plane(self.inner, channel as _, &mut stride) };
        assert!(!data.is_null());
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
        let data = unsafe { lh::heif_image_get_plane(self.inner, channel as _, &mut stride) };
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
        unsafe { lh::heif_image_has_channel(self.inner, channel as _) != 0 }
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

    pub fn color_space(&self) -> Option<ColorSpace> {
        unsafe {
            ColorSpace::from_libheif(
                lh::heif_image_get_colorspace(self.inner),
                lh::heif_image_get_chroma_format(self.inner),
            )
        }
    }

    /// Scale image by "nearest neighbor" method.
    pub fn scale(
        &self,
        width: u32,
        height: u32,
        _scaling_options: Option<ScalingOptions>,
    ) -> Result<Image> {
        let mut c_image = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_image_scale_image(
                self.inner,
                c_image.as_mut_ptr(),
                width as _,
                height as _,
                ptr::null(),
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Image {
            inner: unsafe { c_image.assume_init() },
        })
    }

    /// The indicated bit_depth corresponds to the bit depth per channel.
    /// I.e. for interleaved formats like RRGGBB, the bit_depth would be, e.g., 10 bit instead
    /// of 30 bits or 3*16=48 bits.
    /// For backward compatibility, one can also specify 24bits for RGB and 32bits for RGBA,
    /// instead of the preferred 8 bits.
    pub fn create_plane(
        &mut self,
        channel: Channel,
        width: u32,
        height: u32,
        bit_depth: u8,
    ) -> Result<()> {
        let err = unsafe {
            lh::heif_image_add_plane(
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

    pub fn set_premultiplied_alpha(&self, is_premultiplied_alpha: bool) {
        unsafe { lh::heif_image_set_premultiplied_alpha(self.inner, is_premultiplied_alpha as _) };
    }

    pub fn is_premultiplied_alpha(&self) -> bool {
        unsafe { lh::heif_image_is_premultiplied_alpha(self.inner) != 0 }
    }

    pub fn color_profile_raw(&self) -> Option<ColorProfileRaw> {
        let size = unsafe { lh::heif_image_get_raw_color_profile_size(self.inner) };
        if size == 0 {
            return None;
        }
        let mut result: Vec<u8> = Vec::with_capacity(size);
        let err = unsafe { lh::heif_image_get_raw_color_profile(self.inner, result.as_ptr() as _) };
        if err.code != 0 {
            // Only one error is possible inside `libheif` - `ColorProfileDoesNotExist`
            return None;
        }
        unsafe {
            result.set_len(size);
        }
        let c_profile_type = unsafe { lh::heif_image_get_color_profile_type(self.inner) };
        let profile_type = ColorProfileType::from(c_profile_type);

        Some(ColorProfileRaw {
            typ: profile_type,
            data: result,
        })
    }

    pub fn color_profile_nclx(&self) -> Option<ColorProfileNCLX> {
        let mut profile_ptr = MaybeUninit::<_>::uninit();
        let err =
            unsafe { lh::heif_image_get_nclx_color_profile(self.inner, profile_ptr.as_mut_ptr()) };
        if err.code != 0 {
            // Only one error is possible inside `libheif` - `ColorProfileDoesNotExist`
            return None;
        }
        let profile_ptr = unsafe { profile_ptr.assume_init() };
        Some(ColorProfileNCLX { inner: profile_ptr })
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { lh::heif_image_release(self.inner) };
    }
}

unsafe impl Send for Image {}
