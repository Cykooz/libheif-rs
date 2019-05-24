use libheif_sys::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum ColorSpace {
    Undefined = heif_colorspace_heif_colorspace_undefined as isize,
    YCbCr = heif_colorspace_heif_colorspace_YCbCr as isize,
    Rgb = heif_colorspace_heif_colorspace_RGB as isize,
    Monochrome = heif_colorspace_heif_colorspace_monochrome as isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Chroma {
    Undefined = 99,
    Monochrome = 0,
    C420 = 1,
    C422 = 2,
    C444 = 3,
    InterleavedRgb = 10,
    InterleavedRgba = 11,
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
