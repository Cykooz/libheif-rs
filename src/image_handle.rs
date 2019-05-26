use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::ptr;

use libheif_sys::*;

use crate::enums::{Chroma, ColorSpace};
use crate::errors::{HeifError, HeifErrorCode, HeifErrorSubCode};
use crate::utils::cstr_to_str;
use crate::{image, Image};

pub struct ImageHandle {
    inner: *mut heif_image_handle,
}

pub type ItemId = heif_item_id;

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

impl ImageHandle {
    pub fn decode(&self, colorspace: ColorSpace, chroma: Chroma) -> Result<Image, HeifError> {
        let mut image = unsafe { mem::uninitialized() };
        let options = unsafe { heif_decoding_options_alloc() };
        let err = unsafe {
            heif_decode_image(
                self.inner,
                &mut image,
                colorspace as _,
                chroma as _,
                options,
            )
        };
        unsafe { heif_decoding_options_free(options) };
        HeifError::from_heif_error(err)?;
        Ok(image::heif_image_2_rs_image(image))
    }

    pub fn width(&self) -> u32 {
        unsafe { heif_image_handle_get_width(self.inner) as _ }
    }

    pub fn height(&self) -> u32 {
        unsafe { heif_image_handle_get_height(self.inner) as _ }
    }

    pub fn has_alpha_channel(&self) -> bool {
        unsafe { heif_image_handle_has_alpha_channel(self.inner) != 0 }
    }

    pub fn is_primary_image(&self) -> bool {
        unsafe { heif_image_handle_is_primary_image(self.inner) != 0 }
    }

    pub fn luma_bits_per_pixel(&self) -> u8 {
        unsafe { heif_image_handle_get_luma_bits_per_pixel(self.inner) as _ }
    }

    pub fn chroma_bits_per_pixel(&self) -> u8 {
        unsafe { heif_image_handle_get_chroma_bits_per_pixel(self.inner) as _ }
    }

    /// Get the image width from the 'ispe' box. This is the original image size without
    /// any transformations applied to it. Do not use this unless you know exactly what
    /// you are doing.
    pub fn ispe_width(&self) -> i32 {
        unsafe { heif_image_handle_get_ispe_width(self.inner) as _ }
    }

    /// Get the image height from the 'ispe' box. This is the original image size without
    /// any transformations applied to it. Do not use this unless you know exactly what
    /// you are doing.
    pub fn ispe_height(&self) -> i32 {
        unsafe { heif_image_handle_get_ispe_height(self.inner) as _ }
    }

    // Depth images

    pub fn has_depth_image(&self) -> bool {
        unsafe { heif_image_handle_has_depth_image(self.inner) != 0 }
    }

    pub fn number_of_depth_images(&self) -> i32 {
        unsafe { heif_image_handle_get_number_of_depth_images(self.inner) }
    }

    pub fn list_of_depth_image_ids(&self, count: usize) -> Vec<ItemId> {
        let mut item_ids: Vec<ItemId> = vec![0; count];
        let res_count = unsafe {
            heif_image_handle_get_list_of_depth_image_IDs(
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

    pub fn depth_image_handle(&self, depth_image_id: ItemId) -> Result<ImageHandle, HeifError> {
        let mut out_depth_handler = unsafe { mem::uninitialized() };
        let err = unsafe {
            heif_image_handle_get_depth_image_handle(
                self.inner,
                depth_image_id,
                &mut out_depth_handler,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(heif_image_handle_2_rs_image_handle(out_depth_handler))
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
        unsafe { heif_image_handle_get_number_of_thumbnails(self.inner) as _ }
    }

    pub fn list_of_thumbnail_ids(&self, count: usize) -> Vec<ItemId> {
        let mut item_ids: Vec<ItemId> = vec![0; count];
        let res_count = unsafe {
            heif_image_handle_get_list_of_thumbnail_IDs(
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

    pub fn thumbnail(&self, thumbnail_id: ItemId) -> Result<ImageHandle, HeifError> {
        let mut out_thumbnail_handler = unsafe { mem::uninitialized() };
        let err = unsafe {
            heif_image_handle_get_thumbnail(self.inner, thumbnail_id, &mut out_thumbnail_handler)
        };
        HeifError::from_heif_error(err)?;
        Ok(heif_image_handle_2_rs_image_handle(out_thumbnail_handler))
    }

    // Metadata

    #[inline]
    fn convert_type_filter(type_filter: &str) -> *const c_char {
        match type_filter {
            "" => ptr::null(),
            _ => match CString::new(type_filter) {
                Ok(s) => s.as_ptr(),
                _ => ptr::null(),
            },
        }
    }

    pub fn number_of_metadata_blocks(&self, type_filter: &str) -> i32 {
        let c_type_filter = Self::convert_type_filter(type_filter);
        unsafe { heif_image_handle_get_number_of_metadata_blocks(self.inner, c_type_filter) }
    }

    pub fn list_of_metadata_block_ids(&self, type_filter: &str, count: usize) -> Vec<ItemId> {
        let mut item_ids: Vec<ItemId> = vec![0; count];
        let c_type_filter = Self::convert_type_filter(type_filter);
        let res_count = unsafe {
            heif_image_handle_get_list_of_metadata_block_IDs(
                self.inner,
                c_type_filter,
                item_ids.as_mut_ptr(),
                count as _,
            ) as usize
        };
        if count != res_count {
            item_ids.resize(res_count, 0);
        }
        item_ids
    }

    pub fn metadata_type(&self, metadata_id: ItemId) -> Option<&str> {
        let c_type = unsafe { heif_image_handle_get_metadata_type(self.inner, metadata_id) };
        cstr_to_str(c_type)
    }

    pub fn metadata_content_type(&self, metadata_id: ItemId) -> Option<&str> {
        let c_type =
            unsafe { heif_image_handle_get_metadata_content_type(self.inner, metadata_id) };
        cstr_to_str(c_type)
    }

    pub fn metadata_size(&self, metadata_id: ItemId) -> usize {
        unsafe { heif_image_handle_get_metadata_size(self.inner, metadata_id) }
    }

    pub fn metadata(&self, metadata_id: ItemId) -> Result<Vec<u8>, HeifError> {
        let size = self.metadata_size(metadata_id);
        if size == 0 {
            return Err(HeifError {
                code: HeifErrorCode::UsageError,
                sub_code: HeifErrorSubCode::NonexistingItemReferenced,
                message: "".to_string(),
            });
        }
        let mut result: Vec<u8> = Vec::with_capacity(size);
        unsafe {
            result.set_len(size);
            let err = heif_image_handle_get_metadata(self.inner, metadata_id, result.as_ptr() as _);
            HeifError::from_heif_error(err)?;
        }
        Ok(result)
    }
}

impl Drop for ImageHandle {
    fn drop(&mut self) {
        unsafe { heif_image_handle_release(self.inner) };
    }
}

#[inline]
pub fn heif_image_handle_2_rs_image_handle(handle: *mut heif_image_handle) -> ImageHandle {
    ImageHandle { inner: handle }
}
