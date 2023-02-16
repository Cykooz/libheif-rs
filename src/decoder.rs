use libheif_sys as lh;

#[derive(Debug)]
pub struct DecodingOptions {
    pub(crate) inner: *mut lh::heif_decoding_options,
}

impl DecodingOptions {
    pub fn new() -> Option<Self> {
        let inner = unsafe { lh::heif_decoding_options_alloc() };
        if inner.is_null() {
            return None;
        }
        Some(Self { inner })
    }
}

impl Drop for DecodingOptions {
    fn drop(&mut self) {
        unsafe {
            lh::heif_decoding_options_free(self.inner);
        }
    }
}

impl DecodingOptions {
    #[inline(always)]
    fn inner_ref(&self) -> &lh::heif_decoding_options {
        unsafe { &(*self.inner) }
    }

    #[inline(always)]
    fn inner_mut(&mut self) -> &mut lh::heif_decoding_options {
        unsafe { &mut (*self.inner) }
    }

    #[inline]
    pub fn version(&self) -> u8 {
        self.inner_ref().version
    }

    #[inline]
    pub fn ignore_transformations(&self) -> bool {
        self.inner_ref().ignore_transformations != 0
    }

    #[inline]
    pub fn set_ignore_transformations(&mut self, enable: bool) {
        self.inner_mut().ignore_transformations = if enable { 1 } else { 0 }
    }

    #[inline]
    pub fn convert_hdr_to_8bit(&self) -> bool {
        self.inner_ref().convert_hdr_to_8bit != 0
    }

    #[inline]
    pub fn set_convert_hdr_to_8bit(&mut self, enable: bool) {
        self.inner_mut().convert_hdr_to_8bit = if enable { 1 } else { 0 }
    }

    #[inline]
    pub fn strict_decoding(&self) -> bool {
        self.inner_ref().strict_decoding != 0
    }

    #[inline]
    pub fn set_strict_decoding(&mut self, enable: bool) {
        self.inner_mut().strict_decoding = if enable { 1 } else { 0 }
    }
}
