use std::ffi;
use std::mem;
use std::ptr;

use libheif_sys::*;

use crate::encoder::{Encoder, EncodingOptions};
use crate::enums::CompressionFormat;
use crate::image::Image;
use crate::{HeifError, HeifErrorCode, HeifErrorSubCode, ImageHandle};

//#[derive(Debug)]
//pub struct DecodeOptions {
//    inner: *mut heif_decoding_options,
//}
//
//impl DecodeOptions {
//    pub fn new() -> DecodeOptions {
//        DecodeOptions {
//            inner: unsafe { heif_decoding_options_alloc() },
//        }
//    }
//}
//
//impl Drop for DecodeOptions {
//    fn drop(&mut self) {
//        unsafe { heif_decoding_options_free(self.inner) };
//    }
//}

pub struct HeifContext {
    inner: *mut heif_context,
}

impl HeifContext {
    /// Create a new empty context.
    pub fn new() -> Result<HeifContext, HeifError> {
        let ctx = unsafe { heif_context_alloc() };
        if ctx.is_null() {
            Err(HeifError {
                code: HeifErrorCode::ContextCreateFailed,
                sub_code: HeifErrorSubCode::Unspecified,
                message: String::from(""),
            })
        } else {
            Ok(HeifContext { inner: ctx })
        }
    }

    /// Create a new context from bytes.
    pub fn read_from_bytes(bytes: &[u8]) -> Result<HeifContext, HeifError> {
        let context = HeifContext::new()?;
        let err = unsafe {
            heif_context_read_from_memory_without_copy(
                context.inner,
                bytes.as_ptr() as _,
                bytes.len(),
                ptr::null(),
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(context)
    }

    /// Create a new context from file.
    pub fn read_from_file(name: &str) -> Result<HeifContext, HeifError> {
        let context = HeifContext::new()?;
        let c_name = ffi::CString::new(name).unwrap();
        let err =
            unsafe { heif_context_read_from_file(context.inner, c_name.as_ptr(), ptr::null()) };
        HeifError::from_heif_error(err)?;
        Ok(context)
    }

    unsafe extern "C" fn vector_writer(
        _ctx: *mut heif_context,
        data: *const ::std::os::raw::c_void,
        size: usize,
        user_data: *mut ::std::os::raw::c_void,
    ) -> heif_error {
        let vec: &mut Vec<u8> = &mut *(user_data as *mut Vec<u8>);
        vec.reserve(size);
        vec.set_len(size);
        ptr::copy_nonoverlapping::<u8>(data as _, vec.as_mut_ptr(), size);

        heif_error {
            code: 0,
            subcode: heif_suberror_code_heif_suberror_Unspecified,
            message: ptr::null(),
        }
    }

    pub fn write_to_bytes(&self) -> Result<Vec<u8>, HeifError> {
        let mut res = Vec::<u8>::new();
        let pointer_to_res = &mut res as *mut _ as *mut ::std::os::raw::c_void;

        let mut writer = heif_writer {
            writer_api_version: 1,
            write: Some(Self::vector_writer),
        };

        let err = unsafe { heif_context_write(self.inner, &mut writer, pointer_to_res) };
        HeifError::from_heif_error(err)?;
        Ok(res)
    }

    pub fn write_to_file(&self, name: &str) -> Result<(), HeifError> {
        let c_name = ffi::CString::new(name).unwrap();
        let err = unsafe { heif_context_write_to_file(self.inner, c_name.as_ptr()) };
        HeifError::from_heif_error(err)
    }

    pub fn number_of_top_level_images(&self) -> usize {
        unsafe { heif_context_get_number_of_top_level_images(self.inner) as _ }
    }

    pub fn primary_image_handle(&self) -> Result<ImageHandle, HeifError> {
        let mut handle = unsafe { mem::uninitialized() };
        let err = unsafe { heif_context_get_primary_image_handle(self.inner, &mut handle) };
        HeifError::from_heif_error(err)?;
        Ok(ImageHandle::new(self, handle))
    }

    pub fn encoder_for_format(&self, format: CompressionFormat) -> Result<Encoder, HeifError> {
        let mut c_encoder = Box::new(unsafe { mem::uninitialized() });
        let err = unsafe {
            heif_context_get_encoder_for_format(self.inner, format as _, &mut *c_encoder)
        };
        HeifError::from_heif_error(err)?;
        let encoder = Encoder::new(*c_encoder)?;
        Ok(encoder)
    }

    pub fn encode_image(
        &mut self,
        image: &Image,
        encoder: &mut Encoder,
        encoding_options: Option<EncodingOptions>,
    ) -> Result<(), HeifError> {
        let encoding_options_ptr = match encoding_options {
            Some(options) => &(options.heif_encoding_options()),
            None => ptr::null(),
        };

        unsafe {
            let err = heif_context_encode_image(
                self.inner,
                image.inner,
                encoder.inner,
                encoding_options_ptr,
                ptr::null_mut(),
            );
            HeifError::from_heif_error(err)?;
        }
        Ok(())
    }
}

impl Drop for HeifContext {
    fn drop(&mut self) {
        unsafe { heif_context_free(self.inner) };
    }
}

unsafe impl Send for HeifContext {}
