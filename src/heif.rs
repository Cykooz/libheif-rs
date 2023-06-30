use std::ffi::CString;
use std::path::Path;
use std::ptr;

use libheif_sys as lh;

use crate::utils::path_to_cstring;
use crate::{
    ColorSpace, CompressionFormat, DecoderDescriptor, DecodingOptions, Encoder, EncoderDescriptor,
    HeifError, Image, ImageHandle, Result,
};

/// Guard structure used for `libheif` initialization, working with plugins,
/// getting encoders, decode images and etc.
///
/// You may use one instance of this structure for all parts of your code.
/// Methods of the structure use static variables inside `libheif`.
/// So any changes what they will make in internals of `libheif`
/// will have side effect to all instances of the structure.
///
/// For example, if you load some plugins by one `LibHeif` instance,
/// all those plugins will be available to all other instances.
pub struct LibHeif(());

impl LibHeif {
    pub fn new() -> Self {
        let mut init_params = lh::heif_init_params { version: 0 };
        unsafe {
            lh::heif_init(&mut init_params as _);
        }
        Self(())
    }

    /// Use this method if you need to make sure that external plugins
    /// have loaded successfully.
    ///
    /// Default paths of directories with external plugins may be
    /// changed with help of environment variable `LIBHEIF_PLUGIN_PATH`.
    pub fn new_checked() -> Result<Self> {
        let mut init_params = lh::heif_init_params { version: 0 };
        let error = unsafe { lh::heif_init(&mut init_params as _) };
        HeifError::from_heif_error(error)?;
        Ok(Self(()))
    }
}

impl Drop for LibHeif {
    fn drop(&mut self) {
        unsafe {
            lh::heif_deinit();
        }
    }
}

impl Default for LibHeif {
    fn default() -> Self {
        Self::new()
    }
}

impl LibHeif {
    /// Version of linked `libheif` as an array of numbers in format:
    ///
    /// `[<major>, <minor>, <maintenance>]`
    pub fn version(&self) -> [u8; 3] {
        // Numeric version of linked libheif library, encoded as 0xHHMMLL00 = HH.MM.LL.
        let version: u32 = unsafe { lh::heif_get_version_number() };
        let parts = version.to_be_bytes();
        [parts[0], parts[1], parts[2]]
    }

    /// Load all plugins from given path of directory.
    ///
    /// Returns number of loaded plugins.
    pub fn load_plugins(&self, dir_path: impl AsRef<Path>) -> Result<usize> {
        self._load_plugins(dir_path.as_ref())
    }

    // TODO: Consider using 'momo' crate in the future
    fn _load_plugins(&self, dir_path: &Path) -> Result<usize> {
        let dir_path = path_to_cstring(dir_path);
        let mut plugins_loaded: libc::c_int = 0;
        let err = unsafe {
            lh::heif_load_plugins(
                dir_path.as_ptr(),
                ptr::null_mut() as _,
                &mut plugins_loaded as _,
                0,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(plugins_loaded as usize)
    }

    /// Decode an image handle into the actual pixel image and also carry out
    /// all geometric transformations specified in the HEIF file (rotation, cropping, mirroring).
    ///
    /// If `color_space` is set to [`ColorSpace::Undefined`],
    /// respectively, the original colorspace is taken.
    pub fn decode(
        &self,
        image_handle: &ImageHandle,
        color_space: ColorSpace,
        decoding_options: Option<DecodingOptions>,
    ) -> Result<Image> {
        let decoding_options_ptr = decoding_options
            .map(|o| o.inner)
            .unwrap_or_else(ptr::null_mut);
        let mut c_image: *mut lh::heif_image = ptr::null_mut();
        let err = unsafe {
            lh::heif_decode_image(
                image_handle.inner,
                &mut c_image,
                color_space.heif_color_space(),
                color_space.heif_chroma(),
                decoding_options_ptr,
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Image::from_heif_image(c_image))
    }

    /// Get a list of available decoders.
    /// You can filter the decoders by compression format.
    ///
    /// The returned list of decoders is sorted by their priority
    /// (which is a plugin property).
    pub fn decoder_descriptors(
        &self,
        max_count: usize,
        format_filter: Option<CompressionFormat>,
    ) -> Vec<DecoderDescriptor> {
        let format_filter = format_filter.unwrap_or(CompressionFormat::Undefined);
        let max_count = max_count.min(libc::c_int::MAX as usize);

        let mut descriptors_ptr = Vec::with_capacity(max_count);
        unsafe {
            let count = lh::heif_get_decoder_descriptors(
                format_filter as _,
                descriptors_ptr.as_mut_ptr(),
                max_count as _,
            );
            descriptors_ptr.set_len(count as usize);
        }

        descriptors_ptr
            .into_iter()
            .filter_map(|d_ptr| unsafe { d_ptr.as_ref().map(DecoderDescriptor::new) })
            .collect()
    }

    /// Get a list of available encoders.
    /// You can filter the encoders by compression format and name.
    ///
    /// The returned list of encoders is sorted by their priority
    /// (which is a plugin property).
    ///
    /// Note: to get the actual encoder from the descriptors returned here,
    /// use [`LibHeif::encoder`] method.
    pub fn encoder_descriptors(
        &self,
        max_count: usize,
        format_filter: Option<CompressionFormat>,
        name_filter: Option<&str>,
    ) -> Vec<EncoderDescriptor> {
        let format_filter = format_filter.unwrap_or(CompressionFormat::Undefined);
        let max_count = max_count.min(libc::c_int::MAX as usize);
        let name_filter = name_filter
            .map(|s| CString::new(s).ok())
            .unwrap_or_default();
        let name_filter_ptr = name_filter.map(|s| s.as_ptr()).unwrap_or(ptr::null());

        let mut descriptors_ptr = Vec::with_capacity(max_count);
        unsafe {
            let count = lh::heif_get_encoder_descriptors(
                format_filter as _,
                name_filter_ptr,
                descriptors_ptr.as_mut_ptr(),
                max_count as _,
            );
            descriptors_ptr.set_len(count as usize);
        }

        descriptors_ptr
            .into_iter()
            .filter_map(|d_ptr| unsafe { d_ptr.as_ref().map(EncoderDescriptor::new) })
            .collect()
    }

    /// Get an encoder instance that can be used to actually
    /// encode images from a descriptor.
    pub fn encoder(&self, descriptor: EncoderDescriptor) -> Result<Encoder> {
        let mut c_encoder: *mut lh::heif_encoder = ptr::null_mut();
        let err = unsafe {
            lh::heif_context_get_encoder(ptr::null_mut(), descriptor.inner, &mut c_encoder)
        };
        HeifError::from_heif_error(err)?;
        let encoder = Encoder::new(unsafe { &mut *c_encoder })?;
        Ok(encoder)
    }

    /// Get an encoder for the given compression format.
    /// If there are several encoder plugins for this format,
    /// the encoder with the highest plugin priority will be returned.
    pub fn encoder_for_format(&self, format: CompressionFormat) -> Result<Encoder> {
        let mut c_encoder: *mut lh::heif_encoder = ptr::null_mut();
        let err = unsafe {
            lh::heif_context_get_encoder_for_format(
                ptr::null_mut() as _,
                format as _,
                &mut c_encoder,
            )
        };
        HeifError::from_heif_error(err)?;
        let encoder = Encoder::new(unsafe { &mut *c_encoder })?;
        Ok(encoder)
    }
}
