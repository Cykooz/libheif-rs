use libheif_sys::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum ColorSpace {
    Undefined = heif_colorspace_heif_colorspace_undefined as isize,
    YCbCr = heif_colorspace_heif_colorspace_YCbCr as isize,
    RGB = heif_colorspace_heif_colorspace_RGB as isize,
    Monochrome = heif_colorspace_heif_colorspace_monochrome as isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Chroma {
    Undefined = heif_chroma_heif_chroma_undefined as _,
    Monochrome = heif_chroma_heif_chroma_monochrome as _,
    C420 = heif_chroma_heif_chroma_420 as _,
    C422 = heif_chroma_heif_chroma_422 as _,
    C444 = heif_chroma_heif_chroma_444 as _,
    InterleavedRgb = heif_chroma_heif_chroma_interleaved_RGB as _,
    InterleavedRgba = heif_chroma_heif_chroma_interleaved_RGBA as _,
    InterleavedHdrRgbBe = heif_chroma_heif_chroma_interleaved_RRGGBB_BE as _,
    InterleavedHdrRgbaBe = heif_chroma_heif_chroma_interleaved_RRGGBBAA_BE as _,
    InterleavedHdrRgbLe = heif_chroma_heif_chroma_interleaved_RRGGBB_LE as _,
    InterleavedHdrRgbaLe = heif_chroma_heif_chroma_interleaved_RRGGBBAA_LE as _,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
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
    Int = heif_encoder_parameter_type_heif_encoder_parameter_type_integer as _,
    Bool = heif_encoder_parameter_type_heif_encoder_parameter_type_boolean as _,
    String = heif_encoder_parameter_type_heif_encoder_parameter_type_string as _,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderParameterValue {
    Int(i32),
    Bool(bool),
    String(String),
}

//#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
//pub enum DepthRepresentationType {
//    UniformInverseZ =
//        heif_depth_representation_type_heif_depth_representation_type_uniform_inverse_Z as _,
//    UniformDisparity =
//        heif_depth_representation_type_heif_depth_representation_type_uniform_disparity as _,
//    UniformZ = heif_depth_representation_type_heif_depth_representation_type_uniform_Z as _,
//    NonUniformDisparity =
//        heif_depth_representation_type_heif_depth_representation_type_nonuniform_disparity as _,
//}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderQuality {
    LossLess,
    Lossy(u8),
}
