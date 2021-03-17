use std::ffi;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::ptr;

use libheif_sys as lh;

use crate::encoder::{Encoder, EncodingOptions};
use crate::enums::CompressionFormat;
use crate::image::Image;
use crate::reader::{Reader, HEIF_READER};
use crate::{HeifError, HeifErrorCode, HeifErrorSubCode, ImageHandle, ItemId, Result};

pub struct HeifContext {
    pub(crate) inner: *mut lh::heif_context,
    reader: Option<Box<Box<dyn Reader>>>,
}

impl HeifContext {
    /// Create a new empty context.
    pub fn new() -> Result<HeifContext> {
        let ctx = unsafe { lh::heif_context_alloc() };
        if ctx.is_null() {
            Err(HeifError {
                code: HeifErrorCode::ContextCreateFailed,
                sub_code: HeifErrorSubCode::Unspecified,
                message: String::from(""),
            })
        } else {
            Ok(HeifContext {
                inner: ctx,
                reader: None,
            })
        }
    }

    /// Create a new context from bytes.
    pub fn read_from_bytes(bytes: &[u8]) -> Result<HeifContext> {
        let context = HeifContext::new()?;
        let err = unsafe {
            lh::heif_context_read_from_memory_without_copy(
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
    pub fn read_from_file(name: &str) -> Result<HeifContext> {
        let context = HeifContext::new()?;
        let c_name = ffi::CString::new(name).unwrap();
        let err =
            unsafe { lh::heif_context_read_from_file(context.inner, c_name.as_ptr(), ptr::null()) };
        HeifError::from_heif_error(err)?;
        Ok(context)
    }

    /// Create a new context from reader.
    pub fn read_from_reader(reader: Box<dyn Reader>) -> Result<HeifContext> {
        let mut context = HeifContext::new()?;
        let mut reader_box = Box::new(reader);
        let user_data = &mut *reader_box as *mut _ as *mut c_void;
        let err = unsafe {
            lh::heif_context_read_from_reader(context.inner, &HEIF_READER, user_data, ptr::null())
        };
        HeifError::from_heif_error(err)?;
        context.reader = Some(reader_box);
        Ok(context)
    }

    unsafe extern "C" fn vector_writer(
        _ctx: *mut lh::heif_context,
        data: *const c_void,
        size: usize,
        user_data: *mut c_void,
    ) -> lh::heif_error {
        let vec: &mut Vec<u8> = &mut *(user_data as *mut Vec<u8>);
        vec.reserve(size);
        ptr::copy_nonoverlapping::<u8>(data as _, vec.as_mut_ptr(), size);
        vec.set_len(size);

        lh::heif_error {
            code: lh::heif_error_code_heif_error_Ok,
            subcode: lh::heif_suberror_code_heif_suberror_Unspecified,
            message: ptr::null(),
        }
    }

    pub fn write_to_bytes(&self) -> Result<Vec<u8>> {
        let mut res = Vec::<u8>::new();
        let pointer_to_res = &mut res as *mut _ as *mut c_void;

        let mut writer = lh::heif_writer {
            writer_api_version: 1,
            write: Some(Self::vector_writer),
        };

        let err = unsafe { lh::heif_context_write(self.inner, &mut writer, pointer_to_res) };
        HeifError::from_heif_error(err)?;
        Ok(res)
    }

    pub fn write_to_file(&self, name: &str) -> Result<()> {
        let c_name = ffi::CString::new(name).unwrap();
        let err = unsafe { lh::heif_context_write_to_file(self.inner, c_name.as_ptr()) };
        HeifError::from_heif_error(err)
    }

    pub fn number_of_top_level_images(&self) -> usize {
        unsafe { lh::heif_context_get_number_of_top_level_images(self.inner) as _ }
    }

    pub fn top_level_image_ids(&self, item_ids: &mut [ItemId]) -> usize {
        if item_ids.is_empty() {
            0
        } else {
            unsafe {
                lh::heif_context_get_list_of_top_level_image_IDs(
                    self.inner,
                    item_ids.as_mut_ptr(),
                    item_ids.len() as _,
                ) as usize
            }
        }
    }

    pub fn image_handle(&self, item_id: ItemId) -> Result<ImageHandle> {
        let mut handle = MaybeUninit::<_>::uninit();
        let err =
            unsafe { lh::heif_context_get_image_handle(self.inner, item_id, handle.as_mut_ptr()) };
        HeifError::from_heif_error(err)?;
        let handle = unsafe { handle.assume_init() };
        Ok(ImageHandle::new(self, handle))
    }

    pub fn primary_image_handle(&self) -> Result<ImageHandle> {
        let mut handle = MaybeUninit::<_>::uninit();
        let err =
            unsafe { lh::heif_context_get_primary_image_handle(self.inner, handle.as_mut_ptr()) };
        HeifError::from_heif_error(err)?;
        let handle = unsafe { handle.assume_init() };
        Ok(ImageHandle::new(self, handle))
    }

    pub fn encoder_for_format(&self, format: CompressionFormat) -> Result<Encoder> {
        let mut c_encoder = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_context_get_encoder_for_format(self.inner, format as _, c_encoder.as_mut_ptr())
        };
        HeifError::from_heif_error(err)?;
        let c_encoder = unsafe { c_encoder.assume_init() };
        let encoder = Encoder::new(c_encoder)?;
        Ok(encoder)
    }

    /// Compress the input image.
    /// The first image added to the context is also automatically set the primary image, but
    /// you can change the primary image later with [`set_primary_image`] method.
    /// [`set_primary_image`]: #method.set_primary_image
    pub fn encode_image(
        &mut self,
        image: &Image,
        encoder: &mut Encoder,
        encoding_options: Option<EncodingOptions>,
    ) -> Result<ImageHandle> {
        let encoding_options_ptr = match encoding_options {
            Some(options) => options.inner,
            None => ptr::null(),
        };
        let mut handle = MaybeUninit::<_>::uninit();
        unsafe {
            let err = lh::heif_context_encode_image(
                self.inner,
                image.inner,
                encoder.inner,
                encoding_options_ptr,
                handle.as_mut_ptr(),
            );
            HeifError::from_heif_error(err)?;
        }
        let handle = unsafe { handle.assume_init() };
        Ok(ImageHandle::new(self, handle))
    }

    pub fn set_primary_image(&mut self, image_handle: &mut ImageHandle) -> Result<()> {
        unsafe {
            let err = lh::heif_context_set_primary_image(self.inner, image_handle.inner);
            HeifError::from_heif_error(err)
        }
    }
}

impl Drop for HeifContext {
    fn drop(&mut self) {
        unsafe { lh::heif_context_free(self.inner) };
    }
}

unsafe impl Send for HeifContext {}
