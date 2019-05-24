use std::ffi::CStr;
use std::fmt;

use failure::Fail;
use libheif_sys::*;
use num;

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum HeifErrorCode {
    InputDoesNotExist = heif_error_code_heif_error_Input_does_not_exist as _,
    InvalidInput = heif_error_code_heif_error_Invalid_input as _,
    UnsupportedFiletype = heif_error_code_heif_error_Unsupported_filetype as _,
    UnsupportedFeature = heif_error_code_heif_error_Unsupported_feature as _,
    UsageError = heif_error_code_heif_error_Usage_error as _,
    MemoryAllocationError = heif_error_code_heif_error_Memory_allocation_error as _,
    DecoderPluginError = heif_error_code_heif_error_Decoder_plugin_error as _,
    EncoderPluginError = heif_error_code_heif_error_Encoder_plugin_error as _,
    EncodingError = heif_error_code_heif_error_Encoding_error as _,
    ContextCreateFailed,
    Unknown,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum HeifErrorSubCode {
    Unspecified = heif_suberror_code_heif_suberror_Unspecified as _,
    EndOfData = heif_suberror_code_heif_suberror_End_of_data as _,
    InvalidBoxSize = heif_suberror_code_heif_suberror_Invalid_box_size as _,
    NoFtypBox = heif_suberror_code_heif_suberror_No_ftyp_box as _,
    NoIdatBox = heif_suberror_code_heif_suberror_No_idat_box as _,
    NoMetaBox = heif_suberror_code_heif_suberror_No_meta_box as _,
    NoHdlrBox = heif_suberror_code_heif_suberror_No_hdlr_box as _,
    NoHvccBox = heif_suberror_code_heif_suberror_No_hvcC_box as _,
    NoPitmBox = heif_suberror_code_heif_suberror_No_pitm_box as _,
    NoIpcoBox = heif_suberror_code_heif_suberror_No_ipco_box as _,
    NoIpmaBox = heif_suberror_code_heif_suberror_No_ipma_box as _,
    NoIlocBox = heif_suberror_code_heif_suberror_No_iloc_box as _,
    NoIinfBox = heif_suberror_code_heif_suberror_No_iinf_box as _,
    NoIprpBox = heif_suberror_code_heif_suberror_No_iprp_box as _,
    NoIrefBox = heif_suberror_code_heif_suberror_No_iref_box as _,
    NoPictHandler = heif_suberror_code_heif_suberror_No_pict_handler as _,
    IpmaBoxReferencesNonexistingProperty =
        heif_suberror_code_heif_suberror_Ipma_box_references_nonexisting_property as _,
    NoPropertiesAssignedToItem =
        heif_suberror_code_heif_suberror_No_properties_assigned_to_item as _,
    NoItemData = heif_suberror_code_heif_suberror_No_item_data as _,
    InvalidGridData = heif_suberror_code_heif_suberror_Invalid_grid_data as _,
    MissingGridImages = heif_suberror_code_heif_suberror_Missing_grid_images as _,
    InvalidCleanAperture = heif_suberror_code_heif_suberror_Invalid_clean_aperture as _,
    InvalidOverlayData = heif_suberror_code_heif_suberror_Invalid_overlay_data as _,
    OverlayImageOutsideOfCanvas =
        heif_suberror_code_heif_suberror_Overlay_image_outside_of_canvas as _,
    AuxiliaryImageTypeUnspecified =
        heif_suberror_code_heif_suberror_Auxiliary_image_type_unspecified as _,
    NoOrInvalidPrimaryItem = heif_suberror_code_heif_suberror_No_or_invalid_primary_item as _,
    NoInfeBox = heif_suberror_code_heif_suberror_No_infe_box as _,
    UnknownColorProfileType = heif_suberror_code_heif_suberror_Unknown_color_profile_type as _,
    WrongTileImageChromaFormat =
        heif_suberror_code_heif_suberror_Wrong_tile_image_chroma_format as _,
    SecurityLimitExceeded = heif_suberror_code_heif_suberror_Security_limit_exceeded as _,
    NonexistingItemReferenced = heif_suberror_code_heif_suberror_Nonexisting_item_referenced as _,
    NullPointerArgument = heif_suberror_code_heif_suberror_Null_pointer_argument as _,
    NonexistingImageChannelReferenced =
        heif_suberror_code_heif_suberror_Nonexisting_image_channel_referenced as _,
    UnsupportedPluginVersion = heif_suberror_code_heif_suberror_Unsupported_plugin_version as _,
    UnsupportedWriterVersion = heif_suberror_code_heif_suberror_Unsupported_writer_version as _,
    UnsupportedParameter = heif_suberror_code_heif_suberror_Unsupported_parameter as _,
    InvalidParameterValue = heif_suberror_code_heif_suberror_Invalid_parameter_value as _,
    UnsupportedCodec = heif_suberror_code_heif_suberror_Unsupported_codec as _,
    UnsupportedImageType = heif_suberror_code_heif_suberror_Unsupported_image_type as _,
    UnsupportedDataVersion = heif_suberror_code_heif_suberror_Unsupported_data_version as _,
    UnsupportedColorConversion = heif_suberror_code_heif_suberror_Unsupported_color_conversion as _,
    UnsupportedItemConstructionMethod =
        heif_suberror_code_heif_suberror_Unsupported_item_construction_method as _,
    UnsupportedBitDepth = heif_suberror_code_heif_suberror_Unsupported_bit_depth as _,
    CannotWriteOutputData = heif_suberror_code_heif_suberror_Cannot_write_output_data as _,
}

#[derive(Debug, Clone, Fail)]
pub struct HeifError {
    pub code: HeifErrorCode,
    pub sub_code: HeifErrorSubCode,
    pub message: String,
}

impl fmt::Display for HeifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {:?}-{:?} {}",
            self.code, self.sub_code, self.message
        )
    }
}

impl HeifError {
    pub fn from_heif_error(err: heif_error) -> Result<(), HeifError> {
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
            code: num::FromPrimitive::from_u32(err.code).unwrap_or(HeifErrorCode::Unknown),
            sub_code: num::FromPrimitive::from_u32(err.subcode)
                .unwrap_or(HeifErrorSubCode::Unspecified),
            message: String::from(message),
        })
    }
}
