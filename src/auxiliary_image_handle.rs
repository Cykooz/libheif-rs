use std::ffi::{CStr, CString};

use libheif_sys as lh;

use crate::{HeifError, Image, Result};

#[derive(Debug)]
pub struct AuxiliaryImageHandle {
    pub(crate) inner: *mut lh::heif_image_handle,
}

impl Drop for AuxiliaryImageHandle {
    fn drop(&mut self) {
        unsafe { lh::heif_image_handle_release(self.inner) };
    }
}

impl AuxiliaryImageHandle {
    pub(crate) unsafe fn new(handle: *mut lh::heif_image_handle) -> Self {
        AuxiliaryImageHandle { inner: handle }
    }

    pub fn get_type(&self) -> Result<CString> {
        let mut output = core::ptr::null();
        let err = unsafe { lh::heif_image_handle_get_auxiliary_type(self.inner, &mut output) };
        HeifError::from_heif_error(err)?;
        // copy to rust owned string
        let s = unsafe { CStr::from_ptr(output) }.to_owned();

        unsafe { lh::heif_image_handle_release_auxiliary_type(self.inner, &mut output) };

        Ok(s)
    }

    pub fn luma_bits_per_pixel(&self) -> u8 {
        unsafe { lh::heif_image_handle_get_luma_bits_per_pixel(self.inner) as _ }
    }
    pub fn chroma_bits_per_pixel(&self) -> u8 {
        unsafe { lh::heif_image_handle_get_chroma_bits_per_pixel(self.inner) as _ }
    }

    pub fn get_image(&self) -> Result<Image> {
        let mut out_img = core::ptr::null_mut();
        let err = unsafe {
            lh::heif_decode_image(
                self.inner,
                &mut out_img,
                lh::heif_colorspace_heif_colorspace_undefined,
                lh::heif_chroma_heif_chroma_undefined,
                core::ptr::null(),
            )
        };
        HeifError::from_heif_error(err)?;
        let image = Image::from_heif_image(out_img);
        Ok(image)
    }

    // Seems problematic. File created by `aux_handler.get_context().write_to_file()` DOES NOT open in mac preview
    pub fn get_context(&self) -> crate::HeifContext {
        let ptr = unsafe { lh::heif_image_handle_get_context(self.inner) };
        crate::HeifContext {
            inner: ptr,
            source: crate::context::Source::Memory(&()),
        }
    }
}
