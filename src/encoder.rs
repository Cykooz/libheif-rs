use std::collections::HashMap;
use std::ffi::CString;

use libheif_sys as lh;

use crate::utils::cstr_to_str;
use crate::{
    EncoderParameterType, EncoderParameterValue, EncoderQuality, HeifError, HeifErrorCode,
    HeifErrorSubCode, Result,
};

pub type EncoderParametersTypes = HashMap<String, EncoderParameterType>;

fn parameters_types(c_encoder: *mut lh::heif_encoder) -> Result<EncoderParametersTypes> {
    let mut res = EncoderParametersTypes::new();
    unsafe {
        let mut param_pointers = lh::heif_encoder_list_parameters(c_encoder);
        if !param_pointers.is_null() {
            while let Some(raw_param) = (*param_pointers).as_ref() {
                let c_param_type = lh::heif_encoder_parameter_get_type(raw_param);
                let param_type = match EncoderParameterType::n(c_param_type) {
                    Some(res) => res,
                    None => {
                        return Err(HeifError {
                            code: HeifErrorCode::EncoderPluginError,
                            sub_code: HeifErrorSubCode::UnsupportedParameter,
                            message: format!("{} is unknown type of parameter", c_param_type),
                        });
                    }
                };
                let c_param_name = lh::heif_encoder_parameter_get_name(raw_param);
                let name = cstr_to_str(c_param_name).unwrap_or("").to_string();
                res.insert(name, param_type);
                param_pointers = param_pointers.offset(1);
            }
        }
    }
    Ok(res)
}

pub struct Encoder {
    pub(crate) inner: *mut lh::heif_encoder,
    pub(crate) parameters_types: EncoderParametersTypes,
}

impl Encoder {
    pub(crate) fn new(c_encoder: *mut lh::heif_encoder) -> Result<Encoder> {
        Ok(Encoder {
            inner: c_encoder,
            parameters_types: parameters_types(c_encoder)?,
        })
    }

    pub fn name(&self) -> &str {
        let res = unsafe { lh::heif_encoder_get_name(self.inner) };
        cstr_to_str(res).unwrap_or("")
    }

    pub fn set_quality(&mut self, quality: EncoderQuality) -> Result<()> {
        let err;
        match quality {
            EncoderQuality::LossLess => {
                err = unsafe { lh::heif_encoder_set_lossless(self.inner, 1) };
            }
            EncoderQuality::Lossy(value) => {
                unsafe {
                    let middle_err = lh::heif_encoder_set_lossless(self.inner, 0);
                    HeifError::from_heif_error(middle_err)?;
                    err = lh::heif_encoder_set_lossy_quality(self.inner, i32::from(value))
                };
            }
        }
        HeifError::from_heif_error(err)
    }

    fn parameter_value(
        &self,
        name: &str,
        parameter_type: EncoderParameterType,
    ) -> Result<EncoderParameterValue> {
        let c_param_name = CString::new(name).unwrap();
        let param_value = match parameter_type {
            EncoderParameterType::Int => {
                let mut value = 0;
                let err = unsafe {
                    lh::heif_encoder_get_parameter_integer(
                        self.inner,
                        c_param_name.as_ptr(),
                        &mut value as _,
                    )
                };
                HeifError::from_heif_error(err)?;
                EncoderParameterValue::Int(value)
            }
            EncoderParameterType::Bool => {
                let mut value = 0;
                let err = unsafe {
                    lh::heif_encoder_get_parameter_boolean(
                        self.inner,
                        c_param_name.as_ptr(),
                        &mut value as _,
                    )
                };
                HeifError::from_heif_error(err)?;
                EncoderParameterValue::Bool(value > 0)
            }
            EncoderParameterType::String => {
                let value: Vec<u8> = vec![0; 51];
                let err = unsafe {
                    lh::heif_encoder_get_parameter_string(
                        self.inner,
                        c_param_name.as_ptr(),
                        value.as_ptr() as _,
                        50,
                    )
                };
                HeifError::from_heif_error(err)?;
                EncoderParameterValue::String(
                    cstr_to_str(value.as_ptr() as _).unwrap_or("").to_string(),
                )
            }
        };

        Ok(param_value)
    }

    pub fn parameters_names(&self) -> Vec<String> {
        self.parameters_types.keys().cloned().collect()
    }

    pub fn parameter(&self, name: &str) -> Result<Option<EncoderParameterValue>> {
        match self.parameters_types.get(name) {
            Some(param_type) => {
                let value = self.parameter_value(name, *param_type)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub fn set_parameter_value(&self, name: &str, value: EncoderParameterValue) -> Result<()> {
        let c_param_name = CString::new(name).unwrap();
        let err = match value {
            EncoderParameterValue::Bool(v) => unsafe {
                lh::heif_encoder_set_parameter_boolean(self.inner, c_param_name.as_ptr(), v.into())
            },
            EncoderParameterValue::Int(v) => unsafe {
                lh::heif_encoder_set_parameter_integer(self.inner, c_param_name.as_ptr(), v)
            },
            EncoderParameterValue::String(v) => unsafe {
                let c_param_value = CString::new(v).unwrap();
                lh::heif_encoder_set_parameter_string(
                    self.inner,
                    c_param_name.as_ptr(),
                    c_param_value.as_ptr(),
                )
            },
        };
        HeifError::from_heif_error(err)?;
        Ok(())
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { lh::heif_encoder_release(self.inner) };
    }
}

#[derive(Debug)]
pub struct EncodingOptions {
    pub(crate) inner: *mut lh::heif_encoding_options,
}

impl Default for EncodingOptions {
    fn default() -> Self {
        let inner = unsafe { lh::heif_encoding_options_alloc() };
        if inner.is_null() {
            panic!("heif_encoding_options_alloc() returns a null pointer")
        }
        Self { inner }
    }
}

impl Drop for EncodingOptions {
    fn drop(&mut self) {
        unsafe {
            lh::heif_encoding_options_free(self.inner);
        }
    }
}

impl EncodingOptions {
    #[inline]
    pub fn version(&self) -> u8 {
        unsafe { (*self.inner).version }
    }

    #[inline]
    pub fn save_alpha_channel(&self) -> bool {
        unsafe { (*self.inner).save_alpha_channel != 0 }
    }

    #[inline]
    pub fn set_save_alpha_channel(&mut self, enable: bool) {
        unsafe { (*self.inner).save_alpha_channel = if enable { 1 } else { 0 } }
    }

    #[inline]
    pub fn mac_os_compatibility_workaround(&self) -> bool {
        unsafe { (*self.inner).macOS_compatibility_workaround != 0 }
    }

    #[inline]
    pub fn set_mac_os_compatibility_workaround(&mut self, enable: bool) {
        unsafe { (*self.inner).macOS_compatibility_workaround = if enable { 1 } else { 0 } }
    }

    #[inline]
    pub fn save_two_colr_boxes_when_icc_and_nclx_available(&self) -> bool {
        unsafe { (*self.inner).save_two_colr_boxes_when_ICC_and_nclx_available != 0 }
    }

    #[inline]
    pub fn set_save_two_colr_boxes_when_icc_and_nclx_available(&mut self, enable: bool) {
        unsafe {
            (*self.inner).save_two_colr_boxes_when_ICC_and_nclx_available =
                if enable { 1 } else { 0 }
        }
    }

    #[inline]
    pub fn mac_os_compatibility_workaround_no_nclx_profile(&self) -> bool {
        unsafe { (*self.inner).macOS_compatibility_workaround_no_nclx_profile != 0 }
    }

    #[inline]
    pub fn set_mac_os_compatibility_workaround_no_nclx_profile(&mut self, enable: bool) {
        unsafe {
            (*self.inner).macOS_compatibility_workaround_no_nclx_profile =
                if enable { 1 } else { 0 }
        }
    }
}
