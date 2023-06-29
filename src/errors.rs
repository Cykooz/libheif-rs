use std::ffi::CStr;
use std::fmt;

use libheif_sys as lh;

#[derive(Debug, Copy, Clone, enumn::N)]
#[non_exhaustive]
#[repr(C)]
pub enum HeifErrorCode {
    InputDoesNotExist = lh::heif_error_code_heif_error_Input_does_not_exist as _,
    InvalidInput = lh::heif_error_code_heif_error_Invalid_input as _,
    UnsupportedFileType = lh::heif_error_code_heif_error_Unsupported_filetype as _,
    UnsupportedFeature = lh::heif_error_code_heif_error_Unsupported_feature as _,
    UsageError = lh::heif_error_code_heif_error_Usage_error as _,
    MemoryAllocationError = lh::heif_error_code_heif_error_Memory_allocation_error as _,
    DecoderPluginError = lh::heif_error_code_heif_error_Decoder_plugin_error as _,
    EncoderPluginError = lh::heif_error_code_heif_error_Encoder_plugin_error as _,
    EncodingError = lh::heif_error_code_heif_error_Encoding_error as _,
    ColorProfileDoesNotExist = lh::heif_error_code_heif_error_Color_profile_does_not_exist as _,
    PluginLoadingError = lh::heif_error_code_heif_error_Plugin_loading_error as _,
    ContextCreateFailed,
    /// This value is used when library `libheif` returns unknown value of error code.
    Unknown,
}

#[derive(Debug, Copy, Clone, enumn::N)]
#[non_exhaustive]
#[repr(C)]
pub enum HeifErrorSubCode {
    Unspecified = lh::heif_suberror_code_heif_suberror_Unspecified as _,
    EndOfData = lh::heif_suberror_code_heif_suberror_End_of_data as _,
    InvalidBoxSize = lh::heif_suberror_code_heif_suberror_Invalid_box_size as _,
    NoFtypBox = lh::heif_suberror_code_heif_suberror_No_ftyp_box as _,
    NoIdatBox = lh::heif_suberror_code_heif_suberror_No_idat_box as _,
    NoMetaBox = lh::heif_suberror_code_heif_suberror_No_meta_box as _,
    NoHdlrBox = lh::heif_suberror_code_heif_suberror_No_hdlr_box as _,
    NoHvccBox = lh::heif_suberror_code_heif_suberror_No_hvcC_box as _,
    NoPitmBox = lh::heif_suberror_code_heif_suberror_No_pitm_box as _,
    NoIpcoBox = lh::heif_suberror_code_heif_suberror_No_ipco_box as _,
    NoIpmaBox = lh::heif_suberror_code_heif_suberror_No_ipma_box as _,
    NoIlocBox = lh::heif_suberror_code_heif_suberror_No_iloc_box as _,
    NoIinfBox = lh::heif_suberror_code_heif_suberror_No_iinf_box as _,
    NoIprpBox = lh::heif_suberror_code_heif_suberror_No_iprp_box as _,
    NoIrefBox = lh::heif_suberror_code_heif_suberror_No_iref_box as _,
    NoPictHandler = lh::heif_suberror_code_heif_suberror_No_pict_handler as _,
    IpmaBoxReferencesNonExistingProperty =
        lh::heif_suberror_code_heif_suberror_Ipma_box_references_nonexisting_property as _,
    NoPropertiesAssignedToItem =
        lh::heif_suberror_code_heif_suberror_No_properties_assigned_to_item as _,
    NoItemData = lh::heif_suberror_code_heif_suberror_No_item_data as _,
    InvalidGridData = lh::heif_suberror_code_heif_suberror_Invalid_grid_data as _,
    MissingGridImages = lh::heif_suberror_code_heif_suberror_Missing_grid_images as _,
    InvalidCleanAperture = lh::heif_suberror_code_heif_suberror_Invalid_clean_aperture as _,
    InvalidOverlayData = lh::heif_suberror_code_heif_suberror_Invalid_overlay_data as _,
    OverlayImageOutsideOfCanvas =
        lh::heif_suberror_code_heif_suberror_Overlay_image_outside_of_canvas as _,
    AuxiliaryImageTypeUnspecified =
        lh::heif_suberror_code_heif_suberror_Auxiliary_image_type_unspecified as _,
    NoOrInvalidPrimaryItem = lh::heif_suberror_code_heif_suberror_No_or_invalid_primary_item as _,
    NoInfeBox = lh::heif_suberror_code_heif_suberror_No_infe_box as _,
    UnknownColorProfileType = lh::heif_suberror_code_heif_suberror_Unknown_color_profile_type as _,
    WrongTileImageChromaFormat =
        lh::heif_suberror_code_heif_suberror_Wrong_tile_image_chroma_format as _,
    InvalidFractionalNumber = lh::heif_suberror_code_heif_suberror_Invalid_fractional_number as _,
    InvalidImageSize = lh::heif_suberror_code_heif_suberror_Invalid_image_size as _,
    InvalidPixiBox = lh::heif_suberror_code_heif_suberror_Invalid_pixi_box as _,
    NoAv1cBox = lh::heif_suberror_code_heif_suberror_No_av1C_box as _,
    WrongTileImagePixelDepth =
        lh::heif_suberror_code_heif_suberror_Wrong_tile_image_pixel_depth as _,
    SecurityLimitExceeded = lh::heif_suberror_code_heif_suberror_Security_limit_exceeded as _,
    NonExistingItemReferenced =
        lh::heif_suberror_code_heif_suberror_Nonexisting_item_referenced as _,
    NullPointerArgument = lh::heif_suberror_code_heif_suberror_Null_pointer_argument as _,
    NonExistingImageChannelReferenced =
        lh::heif_suberror_code_heif_suberror_Nonexisting_image_channel_referenced as _,
    UnsupportedPluginVersion = lh::heif_suberror_code_heif_suberror_Unsupported_plugin_version as _,
    UnsupportedWriterVersion = lh::heif_suberror_code_heif_suberror_Unsupported_writer_version as _,
    UnsupportedParameter = lh::heif_suberror_code_heif_suberror_Unsupported_parameter as _,
    InvalidParameterValue = lh::heif_suberror_code_heif_suberror_Invalid_parameter_value as _,
    UnsupportedCodec = lh::heif_suberror_code_heif_suberror_Unsupported_codec as _,
    UnsupportedImageType = lh::heif_suberror_code_heif_suberror_Unsupported_image_type as _,
    UnsupportedDataVersion = lh::heif_suberror_code_heif_suberror_Unsupported_data_version as _,
    UnsupportedColorConversion =
        lh::heif_suberror_code_heif_suberror_Unsupported_color_conversion as _,
    UnsupportedItemConstructionMethod =
        lh::heif_suberror_code_heif_suberror_Unsupported_item_construction_method as _,
    UnsupportedBitDepth = lh::heif_suberror_code_heif_suberror_Unsupported_bit_depth as _,
    CannotWriteOutputData = lh::heif_suberror_code_heif_suberror_Cannot_write_output_data as _,
    UnknownNclxColorPrimaries =
        lh::heif_suberror_code_heif_suberror_Unknown_NCLX_color_primaries as _,
    UnknownNclxTransferCharacteristics =
        lh::heif_suberror_code_heif_suberror_Unknown_NCLX_transfer_characteristics as _,
    UnknownNclxMatrixCoefficients =
        lh::heif_suberror_code_heif_suberror_Unknown_NCLX_matrix_coefficients as _,
    UnsupportedHeaderCompressionMethod =
        lh::heif_suberror_code_heif_suberror_Unsupported_header_compression_method as _,
    PluginLoadingError = lh::heif_suberror_code_heif_suberror_Plugin_loading_error as _,
    PluginIsNotLoaded = lh::heif_suberror_code_heif_suberror_Plugin_is_not_loaded as _,
    CannotReadPluginDirectory =
        lh::heif_suberror_code_heif_suberror_Cannot_read_plugin_directory as _,
    InvalidRegionData = lh::heif_suberror_code_heif_suberror_Invalid_region_data as _,
    InvalidProperty = lh::heif_suberror_code_heif_suberror_Invalid_property as _,
    ItemReferenceCycle = lh::heif_suberror_code_heif_suberror_Item_reference_cycle as _,
    EncoderInitialization = lh::heif_suberror_code_heif_suberror_Encoder_initialization as _,
    EncoderEncoding = lh::heif_suberror_code_heif_suberror_Encoder_encoding as _,
    EncoderCleanup = lh::heif_suberror_code_heif_suberror_Encoder_cleanup as _,
    TooManyRegions = lh::heif_suberror_code_heif_suberror_Too_many_regions as _,
    /// This value is used when library `libheif` returns unknown value of error sub-code.
    Unknown,
}

#[derive(Debug, Clone)]
pub struct HeifError {
    pub code: HeifErrorCode,
    pub sub_code: HeifErrorSubCode,
    pub message: String,
}

pub type Result<T> = std::result::Result<T, HeifError>;

impl std::error::Error for HeifError {}

impl fmt::Display for HeifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({:?}) {}", self.code, self.sub_code, self.message)
    }
}

impl HeifError {
    pub fn from_heif_error(err: lh::heif_error) -> Result<()> {
        if err.code == 0 {
            return Ok(());
        }

        let message = if err.message.is_null() {
            ""
        } else {
            let res = unsafe { CStr::from_ptr(err.message).to_str() };
            res.unwrap_or("")
        };

        Err(HeifError {
            code: HeifErrorCode::n(err.code).unwrap_or(HeifErrorCode::Unknown),
            sub_code: HeifErrorSubCode::n(err.subcode).unwrap_or(HeifErrorSubCode::Unknown),
            message: String::from(message),
        })
    }
}
