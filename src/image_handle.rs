use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr;

use libheif_sys as lh;

use crate::utils::cstr_to_str;
use crate::{ColorSpace, HeifContext, HeifError, HeifErrorCode, HeifErrorSubCode, Image, Result};

pub struct ImageHandle<'a> {
    context: &'a HeifContext,
    pub(crate) inner: *mut lh::heif_image_handle,
}

pub type ItemId = lh::heif_item_id;

//pub struct DepthRepresentationInfo {
//    pub version: u8,
//    pub z_near: Option<f64>,
//    pub z_far: Option<f64>,
//    pub d_min: Option<f64>,
//    pub d_max: Option<f64>,
//    pub depth_representation_type: DepthRepresentationType,
//    pub disparity_reference_view: u32,
//    pub depth_nonlinear_representation_model_size: u32,
//    pub depth_nonlinear_representation_model: *mut u8,
//}

impl<'a> ImageHandle<'a> {
    pub(crate) fn new(context: &HeifContext, handle: *mut lh::heif_image_handle) -> ImageHandle {
        ImageHandle {
            context,
            inner: handle,
        }
    }

    pub fn decode(&self, color_space: ColorSpace, ignore_transformations: bool) -> Result<Image> {
        let mut c_image = MaybeUninit::<_>::uninit();
        let err;
        unsafe {
            let mut options = lh::heif_decoding_options_alloc();
            (*options).ignore_transformations = if ignore_transformations { 1 } else { 0 };
            err = lh::heif_decode_image(
                self.inner,
                c_image.as_mut_ptr(),
                color_space.heif_color_space(),
                color_space.heif_chroma(),
                options,
            );
            lh::heif_decoding_options_free(options);
        }
        HeifError::from_heif_error(err)?;
        let c_image = unsafe { c_image.assume_init() };
        Ok(Image::from_heif_image(c_image))
    }

    pub fn width(&self) -> u32 {
        unsafe { lh::heif_image_handle_get_width(self.inner) as _ }
    }

    pub fn height(&self) -> u32 {
        unsafe { lh::heif_image_handle_get_height(self.inner) as _ }
    }

    pub fn has_alpha_channel(&self) -> bool {
        unsafe { lh::heif_image_handle_has_alpha_channel(self.inner) != 0 }
    }

    pub fn is_primary(&self) -> bool {
        unsafe { lh::heif_image_handle_is_primary_image(self.inner) != 0 }
    }

    pub fn luma_bits_per_pixel(&self) -> u8 {
        unsafe { lh::heif_image_handle_get_luma_bits_per_pixel(self.inner) as _ }
    }

    pub fn chroma_bits_per_pixel(&self) -> u8 {
        unsafe { lh::heif_image_handle_get_chroma_bits_per_pixel(self.inner) as _ }
    }

    /// Get the image width from the 'ispe' box. This is the original image size without
    /// any transformations applied to it. Do not use this unless you know exactly what
    /// you are doing.
    pub fn ispe_width(&self) -> i32 {
        unsafe { lh::heif_image_handle_get_ispe_width(self.inner) as _ }
    }

    /// Get the image height from the 'ispe' box. This is the original image size without
    /// any transformations applied to it. Do not use this unless you know exactly what
    /// you are doing.
    pub fn ispe_height(&self) -> i32 {
        unsafe { lh::heif_image_handle_get_ispe_height(self.inner) as _ }
    }

    // Depth images

    pub fn has_depth_image(&self) -> bool {
        unsafe { lh::heif_image_handle_has_depth_image(self.inner) != 0 }
    }

    pub fn number_of_depth_images(&self) -> i32 {
        unsafe { lh::heif_image_handle_get_number_of_depth_images(self.inner) }
    }

    #[deprecated(
        since = "0.14.0",
        note = "Please use the ``depth_image_ids`` method instead"
    )]
    pub fn list_of_depth_image_ids(&self, count: usize) -> Vec<ItemId> {
        let mut item_ids: Vec<ItemId> = vec![0; count];
        let res_count = unsafe {
            lh::heif_image_handle_get_list_of_depth_image_IDs(
                self.inner,
                item_ids.as_mut_ptr(),
                count as _,
            ) as usize
        };
        if count != res_count {
            item_ids.resize(res_count, 0);
        }
        item_ids
    }

    pub fn depth_image_ids(&self, item_ids: &mut [ItemId]) -> usize {
        if item_ids.is_empty() {
            0
        } else {
            unsafe {
                lh::heif_image_handle_get_list_of_depth_image_IDs(
                    self.inner,
                    item_ids.as_mut_ptr(),
                    item_ids.len() as _,
                ) as usize
            }
        }
    }

    pub fn depth_image_handle(&self, depth_image_id: ItemId) -> Result<ImageHandle> {
        let mut out_depth_handler = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_image_handle_get_depth_image_handle(
                self.inner,
                depth_image_id,
                out_depth_handler.as_mut_ptr(),
            )
        };
        HeifError::from_heif_error(err)?;
        let out_depth_handler = unsafe { out_depth_handler.assume_init() };
        Ok(ImageHandle::new(self.context, out_depth_handler))
    }

    //    pub fn get_depth_image_representation_info(&self, depth_image_id: ItemId) {
    //        let mut out = unsafe { mem::uninitialized() };
    //        let res = unsafe {
    //            heif_image_handle_get_depth_image_representation_info(
    //                self.inner, depth_image_id,
    //                &mut out,
    //            )
    //        };
    //    }

    // Thumbnails

    pub fn number_of_thumbnails(&self) -> usize {
        unsafe { lh::heif_image_handle_get_number_of_thumbnails(self.inner) as _ }
    }

    #[deprecated(
        since = "0.14.0",
        note = "Please use the ``thumbnail_ids`` method instead"
    )]
    pub fn list_of_thumbnail_ids(&self, count: usize) -> Vec<ItemId> {
        let mut item_ids: Vec<ItemId> = vec![0; count];
        let res_count = unsafe {
            lh::heif_image_handle_get_list_of_thumbnail_IDs(
                self.inner,
                item_ids.as_mut_ptr(),
                count as _,
            ) as usize
        };
        if count != res_count {
            item_ids.resize(res_count, 0);
        }
        item_ids
    }

    pub fn thumbnail_ids(&self, item_ids: &mut [ItemId]) -> usize {
        if item_ids.is_empty() {
            0
        } else {
            unsafe {
                lh::heif_image_handle_get_list_of_thumbnail_IDs(
                    self.inner,
                    item_ids.as_mut_ptr(),
                    item_ids.len() as _,
                ) as usize
            }
        }
    }

    pub fn thumbnail(&self, thumbnail_id: ItemId) -> Result<ImageHandle> {
        let mut out_thumbnail_handler = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_image_handle_get_thumbnail(
                self.inner,
                thumbnail_id,
                out_thumbnail_handler.as_mut_ptr(),
            )
        };
        HeifError::from_heif_error(err)?;
        let out_thumbnail_handler = unsafe { out_thumbnail_handler.assume_init() };
        Ok(ImageHandle::new(self.context, out_thumbnail_handler))
    }

    // Metadata

    #[inline]
    fn convert_type_filter(type_filter: &str) -> Option<CString> {
        match type_filter {
            "" => None,
            _ => Some(CString::new(type_filter).unwrap()),
        }
    }

    pub fn number_of_metadata_blocks(&self, type_filter: &str) -> i32 {
        let c_type_filter = Self::convert_type_filter(type_filter);
        let filter_ptr: *const c_char = match &c_type_filter {
            Some(s) => s.as_ptr(),
            None => ptr::null(),
        };
        unsafe { lh::heif_image_handle_get_number_of_metadata_blocks(self.inner, filter_ptr) }
    }

    #[deprecated(
        since = "0.14.0",
        note = "Please use the ``metadata_block_ids`` method instead"
    )]
    pub fn list_of_metadata_block_ids(&self, type_filter: &str, count: usize) -> Vec<ItemId> {
        let mut item_ids: Vec<ItemId> = vec![0; count];

        let c_type_filter = Self::convert_type_filter(type_filter);
        let filter_ptr: *const c_char = match &c_type_filter {
            Some(s) => s.as_ptr(),
            None => ptr::null(),
        };

        let res_count = unsafe {
            lh::heif_image_handle_get_list_of_metadata_block_IDs(
                self.inner,
                filter_ptr,
                item_ids.as_mut_ptr(),
                count as _,
            ) as usize
        };
        if count != res_count {
            item_ids.resize(res_count, 0);
        }
        item_ids
    }

    pub fn metadata_block_ids(&self, type_filter: &str, item_ids: &mut [ItemId]) -> usize {
        if item_ids.is_empty() {
            0
        } else {
            let c_type_filter = Self::convert_type_filter(type_filter);
            let filter_ptr: *const c_char = match &c_type_filter {
                Some(s) => s.as_ptr(),
                None => ptr::null(),
            };
            unsafe {
                lh::heif_image_handle_get_list_of_metadata_block_IDs(
                    self.inner,
                    filter_ptr,
                    item_ids.as_mut_ptr(),
                    item_ids.len() as _,
                ) as usize
            }
        }
    }

    pub fn metadata_type(&self, metadata_id: ItemId) -> Option<&str> {
        let c_type: *const c_char =
            unsafe { lh::heif_image_handle_get_metadata_type(self.inner, metadata_id) };
        cstr_to_str(c_type)
    }

    pub fn metadata_content_type(&self, metadata_id: ItemId) -> Option<&str> {
        let c_type =
            unsafe { lh::heif_image_handle_get_metadata_content_type(self.inner, metadata_id) };
        cstr_to_str(c_type)
    }

    pub fn metadata_size(&self, metadata_id: ItemId) -> usize {
        unsafe { lh::heif_image_handle_get_metadata_size(self.inner, metadata_id) }
    }

    pub fn metadata(&self, metadata_id: ItemId) -> Result<Vec<u8>> {
        let size = self.metadata_size(metadata_id);
        if size == 0 {
            return Err(HeifError {
                code: HeifErrorCode::UsageError,
                sub_code: HeifErrorSubCode::NonExistingItemReferenced,
                message: "".to_string(),
            });
        }
        let mut result: Vec<u8> = Vec::with_capacity(size);
        unsafe {
            result.set_len(size);
            let err =
                lh::heif_image_handle_get_metadata(self.inner, metadata_id, result.as_ptr() as _);
            HeifError::from_heif_error(err)?;
        }
        Ok(result)
    }
}

impl<'a> Drop for ImageHandle<'a> {
    fn drop(&mut self) {
        unsafe { lh::heif_image_handle_release(self.inner) };
    }
}
