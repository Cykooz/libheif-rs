use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr;

use four_cc::FourCC;
use libheif_sys as lh;

use crate::utils::cstr_to_str;
use crate::{
    AuxiliaryImageHandle, ColorProfileNCLX, ColorProfileRaw, ColorProfileType, ColorSpace,
    HeifError, HeifErrorCode, HeifErrorSubCode, ImageMetadata, Result,
};

const LIBHEIF_AUX_IMAGE_FILTER_OMIT_ALPHA: i32 = 1 << 1;
const LIBHEIF_AUX_IMAGE_FILTER_OMIT_DEPTH: i32 = 2 << 1;

/// Encoded image.
#[derive(Debug)]
pub struct ImageHandle {
    pub(crate) inner: *mut lh::heif_image_handle,
}

pub type ItemId = lh::heif_item_id;

impl ImageHandle {
    pub(crate) fn new(handle: *mut lh::heif_image_handle) -> Self {
        ImageHandle { inner: handle }
    }

    pub fn get_number_of_auxiliary_images(&self) -> i32 {
        let aux_filter = LIBHEIF_AUX_IMAGE_FILTER_OMIT_ALPHA | LIBHEIF_AUX_IMAGE_FILTER_OMIT_DEPTH;
        unsafe { lh::heif_image_handle_get_number_of_auxiliary_images(self.inner, aux_filter) }
    }

    pub fn get_auxiliary_image_ids(&self) -> Vec<lh::heif_item_id> {
        let aux_filter = LIBHEIF_AUX_IMAGE_FILTER_OMIT_ALPHA | LIBHEIF_AUX_IMAGE_FILTER_OMIT_DEPTH;
        let mut n_aux = self.get_number_of_auxiliary_images();
        if n_aux == 0 {
            return vec![];
        }
        let mut ids = vec![0; n_aux as usize];
        n_aux = unsafe {
            lh::heif_image_handle_get_list_of_auxiliary_image_IDs(
                self.inner,
                aux_filter,
                ids.as_mut_ptr(),
                n_aux,
            )
        };
        ids.truncate(n_aux as usize);
        ids
    }

    pub fn get_auxiliary_image_handle(
        &self,
        aux_id: lh::heif_item_id,
    ) -> Result<AuxiliaryImageHandle> {
        let mut handle = core::ptr::null_mut();
        let err = unsafe {
            lh::heif_image_handle_get_auxiliary_image_handle(self.inner, aux_id, &mut handle)
        };
        HeifError::from_heif_error(err)?;
        let aux_handle = unsafe { AuxiliaryImageHandle::new(handle) };
        Ok(aux_handle)
    }

    pub fn item_id(&self) -> ItemId {
        unsafe { lh::heif_image_handle_get_item_id(self.inner) }
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

    pub fn is_premultiplied_alpha(&self) -> bool {
        unsafe { lh::heif_image_handle_is_premultiplied_alpha(self.inner) != 0 }
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

    pub fn depth_image_handle(&self, depth_image_id: ItemId) -> Result<Self> {
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
        Ok(ImageHandle {
            inner: out_depth_handler,
        })
    }

    // Thumbnails

    pub fn number_of_thumbnails(&self) -> usize {
        unsafe { lh::heif_image_handle_get_number_of_thumbnails(self.inner) as _ }
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

    pub fn thumbnail(&self, thumbnail_id: ItemId) -> Result<Self> {
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
        Ok(ImageHandle {
            inner: out_thumbnail_handler,
        })
    }

    // Metadata

    fn convert_type_filter<T>(type_filter: T) -> Option<CString>
    where
        T: Into<FourCC>,
    {
        let type_filter = type_filter.into();
        if type_filter.0.contains(&0) {
            // We can't convert FourCC with zero byte into valid C-string
            None
        } else {
            CString::new(type_filter.to_string()).ok()
        }
    }

    pub fn number_of_metadata_blocks<T>(&self, type_filter: T) -> i32
    where
        T: Into<FourCC>,
    {
        let c_type_filter = Self::convert_type_filter(type_filter);
        let filter_ptr: *const c_char = match &c_type_filter {
            Some(s) => s.as_ptr(),
            None => ptr::null(),
        };
        unsafe { lh::heif_image_handle_get_number_of_metadata_blocks(self.inner, filter_ptr) }
    }

    pub fn metadata_block_ids<T>(&self, item_ids: &mut [ItemId], type_filter: T) -> usize
    where
        T: Into<FourCC>,
    {
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

    /// Return a string indicating the type of the metadata, as specified in the HEIF file.
    /// Exif data will have the type string "Exif".
    /// This string will be valid until the next call to a libheif function.
    pub fn metadata_type(&self, metadata_id: ItemId) -> Option<&str> {
        let c_type: *const c_char =
            unsafe { lh::heif_image_handle_get_metadata_type(self.inner, metadata_id) };
        cstr_to_str(c_type)
    }

    /// For EXIF, the content type is `Some("")`.
    ///
    /// For XMP, the content type is `Some("application/rdf+xml")`.
    pub fn metadata_content_type(&self, metadata_id: ItemId) -> Option<&str> {
        let c_type =
            unsafe { lh::heif_image_handle_get_metadata_content_type(self.inner, metadata_id) };
        cstr_to_str(c_type)
    }

    /// Get the size of the raw metadata, as stored in the HEIF file.
    pub fn metadata_size(&self, metadata_id: ItemId) -> usize {
        unsafe { lh::heif_image_handle_get_metadata_size(self.inner, metadata_id) }
    }

    /// Only valid for item type == "uri ", an absolute URI.
    pub fn metadata_item_uri_type(&self, metadata_id: ItemId) -> Option<&str> {
        let c_type =
            unsafe { lh::heif_image_handle_get_metadata_item_uri_type(self.inner, metadata_id) };
        cstr_to_str(c_type)
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
            let err =
                lh::heif_image_handle_get_metadata(self.inner, metadata_id, result.as_ptr() as _);
            HeifError::from_heif_error(err)?;
            result.set_len(size);
        }
        Ok(result)
    }

    /// Return vector with all image's metadata items.
    pub fn all_metadata(&self) -> Vec<ImageMetadata> {
        let count = self.number_of_metadata_blocks(0).max(0) as usize;
        let mut item_ids = vec![0; count];
        let count = self.metadata_block_ids(&mut item_ids, 0);
        let mut result = Vec::with_capacity(count);
        for item_id in item_ids {
            if item_id == 0 {
                continue;
            }
            if let Some(item) = self.item_metadata(item_id) {
                result.push(item);
            }
        }
        result
    }

    fn item_metadata(&self, item_id: ItemId) -> Option<ImageMetadata> {
        let item_type = self
            .metadata_type(item_id)
            .filter(|t| t.len() == 4)
            .map(|t| FourCC::from(t.as_bytes()))?;
        let content_type = self.metadata_content_type(item_id).map(|s| s.to_string())?;
        let uri_type = self
            .metadata_item_uri_type(item_id)
            .map(|s| s.to_string())?;
        let raw_data = self.metadata(item_id).ok()?;
        Some(ImageMetadata {
            item_type,
            content_type,
            uri_type,
            raw_data,
        })
    }

    /// Return the colorspace that `libheif` proposes to use for decoding.
    /// Usually, these will be either [YCbCr](ColorSpace::YCbCr)
    /// or [Monochrome](ColorSpace::Monochrome), but it may also
    /// propose [Rgb](ColorSpace::Rgb) for images encoded with `matrix_coefficients=0`.
    /// It may also return [Undefined](ColorSpace::Undefined) if the file misses
    /// relevant information to determine this without decoding.
    pub fn preferred_decoding_colorspace(&self) -> Result<ColorSpace> {
        let mut lh_colorspace = lh::heif_colorspace_heif_colorspace_undefined;
        let mut lh_chroma = lh::heif_chroma_heif_chroma_undefined;
        let err = unsafe {
            lh::heif_image_handle_get_preferred_decoding_colorspace(
                self.inner,
                &mut lh_colorspace,
                &mut lh_chroma,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(ColorSpace::from_libheif(lh_colorspace, lh_chroma).unwrap_or(ColorSpace::Undefined))
    }

    pub fn color_profile_raw(&self) -> Option<ColorProfileRaw> {
        let size = unsafe { lh::heif_image_handle_get_raw_color_profile_size(self.inner) };
        if size == 0 {
            return None;
        }
        let mut result: Vec<u8> = Vec::with_capacity(size);
        let err = unsafe {
            lh::heif_image_handle_get_raw_color_profile(self.inner, result.as_ptr() as _)
        };
        if err.code != 0 {
            // Only one error is possible inside `libheif` - `ColorProfileDoesNotExist`
            return None;
        }
        unsafe {
            result.set_len(size);
        }
        let c_profile_type = unsafe { lh::heif_image_handle_get_color_profile_type(self.inner) };
        // `c_profile_type` on Windows will be i32, so we need to cast it to u32
        let profile_type = ColorProfileType::from(c_profile_type as u32);

        Some(ColorProfileRaw {
            typ: profile_type,
            data: result,
        })
    }

    /// NOTE: This function does currently not return an NCLX profile if it is
    /// stored in the image bitstream. Only NCLX profiles stored as colr boxes
    /// are returned. This may change in the future.
    pub fn color_profile_nclx(&self) -> Option<ColorProfileNCLX> {
        let mut profile_ptr = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_image_handle_get_nclx_color_profile(self.inner, profile_ptr.as_mut_ptr())
        };
        if err.code != 0 {
            // Only one error is possible inside `libheif` - `ColorProfileDoesNotExist`
            return None;
        }
        let profile_ptr = unsafe { profile_ptr.assume_init() };
        if profile_ptr.is_null() {
            return None;
        }
        Some(ColorProfileNCLX { inner: profile_ptr })
    }
}

impl Drop for ImageHandle {
    fn drop(&mut self) {
        unsafe { lh::heif_image_handle_release(self.inner) };
    }
}
