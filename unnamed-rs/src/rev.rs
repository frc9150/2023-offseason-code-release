mod spark_max;

pub use spark_max::{MotorType, SparkMax, SparkMaxView};
use thiserror::Error;

mod ffi;

#[derive(Error, Debug)]
pub enum RevError {
    // TODO: come up with better error descriptions (me when revlib docs :()
    #[error("REVLib error")]
    Error,
    #[error("timeout")]
    Timeout,
    #[error("not implemented in REVLib")]
    NotImplemented,
    #[error("hal error")]
    HALError,
    #[error("can't find firmware")]
    CantFindFirmware,
    #[error("firmware version is too old")]
    FirmwareTooOld,
    #[error("firmware version is too new")]
    FirmwareTooNew,
    #[error("parameter has invalid id")]
    ParamInvalidID,
    #[error("parameter has mismatched type")]
    ParamMismatchType,
    #[error("parameter access mode")]
    ParamAccessMode,
    #[error("invalid parameter")]
    ParamInvalid,
    #[error("parameter has been removed")]
    ParamNotImplementedDeprecated,
    #[error("mismatched follow config")]
    FollowConfigMismatch,
    #[error("invalid")]
    Invalid,
    #[error("setpoint is out of range")]
    SetpointOutOfRange,
    #[error("unknown")]
    Unknown,
    #[error("CAN disconnected")]
    CANDisconnected,
    #[error("duplicate CAN id")]
    DuplicateCANId,
    #[error("invalid CAN id")]
    InvalidCANId,
    #[error("SparkMax data port is already configured differently")]
    SparkMaxDataPortAlreadyConfiguredDifferently,
    #[error("invalid error code returned from REVLib")]
    InvalidFFIError,
}

impl From<ffi::REVLibError> for Result<(), RevError> {
    fn from(value: ffi::REVLibError) -> Self {
        match value {
            ffi::REVLibError::kOk => Ok(()),
            ffi::REVLibError::kError => Err(RevError::Error),
            ffi::REVLibError::kTimeout => Err(RevError::Timeout),
            ffi::REVLibError::kNotImplemented => Err(RevError::NotImplemented),
            ffi::REVLibError::kHALError => Err(RevError::HALError),
            ffi::REVLibError::kCantFindFirmware => Err(RevError::CantFindFirmware),
            ffi::REVLibError::kFirmwareTooOld => Err(RevError::FirmwareTooOld),
            ffi::REVLibError::kFirmwareTooNew => Err(RevError::FirmwareTooNew),
            ffi::REVLibError::kParamInvalidID => Err(RevError::ParamInvalidID),
            ffi::REVLibError::kParamMismatchType => Err(RevError::ParamMismatchType),
            ffi::REVLibError::kParamAccessMode => Err(RevError::ParamAccessMode),
            ffi::REVLibError::kParamInvalid => Err(RevError::ParamInvalid),
            ffi::REVLibError::kParamNotImplementedDeprecated => {
                Err(RevError::ParamNotImplementedDeprecated)
            }
            ffi::REVLibError::kFollowConfigMismatch => Err(RevError::FollowConfigMismatch),
            ffi::REVLibError::kInvalid => Err(RevError::Invalid),
            ffi::REVLibError::kSetpointOutOfRange => Err(RevError::SetpointOutOfRange),
            ffi::REVLibError::kUnknown => Err(RevError::Unknown),
            ffi::REVLibError::kCANDisconnected => Err(RevError::CANDisconnected),
            ffi::REVLibError::kDuplicateCANId => Err(RevError::DuplicateCANId),
            ffi::REVLibError::kInvalidCANId => Err(RevError::InvalidCANId),
            ffi::REVLibError::kSparkMaxDataPortAlreadyConfiguredDifferently => {
                Err(RevError::SparkMaxDataPortAlreadyConfiguredDifferently)
            }
            _ => Err(RevError::InvalidFFIError),
        }
    }
}
