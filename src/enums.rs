use libheif_sys as lh;
use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum ColorSpace {
    Undefined = lh::heif_colorspace_heif_colorspace_undefined as isize,
    YCbCr = lh::heif_colorspace_heif_colorspace_YCbCr as isize,
    RGB = lh::heif_colorspace_heif_colorspace_RGB as isize,
    Monochrome = lh::heif_colorspace_heif_colorspace_monochrome as isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Chroma {
    Undefined = lh::heif_chroma_heif_chroma_undefined as _,
    Monochrome = lh::heif_chroma_heif_chroma_monochrome as _,
    C420 = lh::heif_chroma_heif_chroma_420 as _,
    C422 = lh::heif_chroma_heif_chroma_422 as _,
    C444 = lh::heif_chroma_heif_chroma_444 as _,
    InterleavedRgb = lh::heif_chroma_heif_chroma_interleaved_RGB as _,
    InterleavedRgba = lh::heif_chroma_heif_chroma_interleaved_RGBA as _,
    InterleavedHdrRgbBe = lh::heif_chroma_heif_chroma_interleaved_RRGGBB_BE as _,
    InterleavedHdrRgbaBe = lh::heif_chroma_heif_chroma_interleaved_RRGGBBAA_BE as _,
    InterleavedHdrRgbLe = lh::heif_chroma_heif_chroma_interleaved_RRGGBB_LE as _,
    InterleavedHdrRgbaLe = lh::heif_chroma_heif_chroma_interleaved_RRGGBBAA_LE as _,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Channel {
    Y = 0,
    Cb = 1,
    Cr = 2,
    R = 3,
    G = 4,
    B = 5,
    Alpha = 6,
    Interleaved = 10,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum CompressionFormat {
    Undefined = 0,
    Hevc = 1,
    Avc = 2,
    Jpeg = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum EncoderParameterType {
    Int = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_integer as _,
    Bool = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_boolean as _,
    String = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_string as _,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderParameterValue {
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderQuality {
    LossLess,
    Lossy(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ReaderGrowStatus {
    SizeReached = lh::heif_reader_grow_status_heif_reader_grow_status_size_reached as _,
    Timeout = lh::heif_reader_grow_status_heif_reader_grow_status_timeout as _,
    SizeBeyondEof = lh::heif_reader_grow_status_heif_reader_grow_status_size_beyond_eof as _,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum FileTypeResult {
    No = lh::heif_filetype_result_heif_filetype_no as _,
    /// It is HEIF and can be read by libheif
    Supported = lh::heif_filetype_result_heif_filetype_yes_supported as _,
    /// It is HEIF, but cannot be read by libheif
    Unsupported = lh::heif_filetype_result_heif_filetype_yes_unsupported as _,
    /// Not sure whether it is an HEIF, try detection with more input data
    MayBe = lh::heif_filetype_result_heif_filetype_maybe as _,
}
