use std::collections::HashMap;
use std::ffi::CString;

use libheif_sys::*;

use crate::enums::{EncoderParameterType, EncoderParameterValue};
use crate::utils::cstr_to_str;
use crate::{EncoderQuality, HeifError, HeifErrorCode, HeifErrorSubCode};

pub type EncoderParametersTypes = HashMap<String, EncoderParameterType>;

fn parameters_types(c_encoder: *mut heif_encoder) -> Result<EncoderParametersTypes, HeifError> {
    let mut res = EncoderParametersTypes::new();
    unsafe {
        let mut param_pointers = heif_encoder_list_parameters(c_encoder);
        if !param_pointers.is_null() {
            while let Some(raw_param) = (*param_pointers).as_ref() {
                let c_param_type = heif_encoder_parameter_get_type(raw_param);
                let param_type: EncoderParameterType;
                match num::FromPrimitive::from_u32(c_param_type) {
                    Some(res) => {
                        param_type = res;
                    }
                    None => {
                        return Err(HeifError {
                            code: HeifErrorCode::EncoderPluginError,
                            sub_code: HeifErrorSubCode::UnsupportedParameter,
                            message: format!("{} is unknown type of parameter", c_param_type),
                        });
                    }
                }
                let c_param_name = heif_encoder_parameter_get_name(raw_param);
                let name = cstr_to_str(c_param_name).unwrap_or("").to_string();
                res.insert(name, param_type);
                param_pointers = param_pointers.offset(1);
            }
        }
    }
    Ok(res)
}

pub struct Encoder {
    pub(crate) inner: *mut heif_encoder,
    pub(crate) parameters_types: EncoderParametersTypes,
}

impl Encoder {
    pub(crate) fn new(c_encoder: *mut heif_encoder) -> Result<Encoder, HeifError> {
        Ok(Encoder {
            inner: c_encoder,
            parameters_types: parameters_types(c_encoder)?,
        })
    }

    pub fn name(&self) -> &str {
        let res = unsafe { heif_encoder_get_name(self.inner) };
        cstr_to_str(res).unwrap_or("")
    }

    pub fn set_quality(&mut self, quality: EncoderQuality) -> Result<(), HeifError> {
        let err;
        match quality {
            EncoderQuality::LossLess => {
                err = unsafe { heif_encoder_set_lossless(self.inner, 1) };
            }
            EncoderQuality::Lossy(value) => {
                unsafe {
                    let middle_err = heif_encoder_set_lossless(self.inner, 0);
                    HeifError::from_heif_error(middle_err)?;
                    err = heif_encoder_set_lossy_quality(self.inner, i32::from(value))
                };
            }
        }
        HeifError::from_heif_error(err)
    }

    fn parameter_value(
        &self,
        name: &str,
        parameter_type: EncoderParameterType,
    ) -> Result<EncoderParameterValue, HeifError> {
        let c_param_name = CString::new(name).unwrap();
        let mut param_value;
        match parameter_type {
            EncoderParameterType::Int => {
                let mut value = 0;
                let err = unsafe {
                    heif_encoder_get_parameter_integer(
                        self.inner,
                        c_param_name.as_ptr(),
                        &mut value as _,
                    )
                };
                HeifError::from_heif_error(err)?;
                param_value = EncoderParameterValue::Int(value);
            }
            EncoderParameterType::Bool => {
                let mut value = 0;
                let err = unsafe {
                    heif_encoder_get_parameter_boolean(
                        self.inner,
                        c_param_name.as_ptr(),
                        &mut value as _,
                    )
                };
                HeifError::from_heif_error(err)?;
                param_value = EncoderParameterValue::Bool(value > 0);
            }
            EncoderParameterType::String => {
                let value: Vec<u8> = vec![0; 51];
                let err = unsafe {
                    heif_encoder_get_parameter_string(
                        self.inner,
                        c_param_name.as_ptr(),
                        value.as_ptr() as _,
                        50,
                    )
                };
                HeifError::from_heif_error(err)?;
                param_value = EncoderParameterValue::String(
                    cstr_to_str(value.as_ptr() as _).unwrap_or("").to_string(),
                );
            }
        };

        Ok(param_value)
    }

    pub fn parameters_names(&self) -> Vec<String> {
        self.parameters_types.keys().cloned().collect()
    }

    pub fn parameter(&self, name: &str) -> Result<Option<EncoderParameterValue>, HeifError> {
        match self.parameters_types.get(name) {
            Some(param_type) => {
                let value = self.parameter_value(name, *param_type)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { heif_encoder_release(self.inner) };
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EncodingOptions {
    pub version: u8,
    pub save_alpha_channel: bool,
}

impl Default for EncodingOptions {
    fn default() -> Self {
        unsafe {
            let heif_opt = heif_encoding_options_alloc();
            let res = EncodingOptions {
                version: (*heif_opt).version,
                save_alpha_channel: (*heif_opt).save_alpha_channel != 0,
            };
            heif_encoding_options_free(heif_opt);
            res
        }
    }
}

impl EncodingOptions {
    pub(crate) fn heif_encoding_options(self) -> heif_encoding_options {
        heif_encoding_options {
            version: self.version,
            save_alpha_channel: if self.save_alpha_channel { 1 } else { 0 },
        }
    }
}
