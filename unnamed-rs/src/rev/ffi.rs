pub use inner::*;

#[cxx::bridge(namespace = "rev")]
mod inner {
    #[repr(u32)]
    #[cxx_name = "CANSparkMaxLowLevel_MotorType"]
    pub enum MotorType {
        kBrushed,
        kBrushless,
    }

    #[repr(u32)]
    #[cxx_name = "CANSparkMaxLowLevel_ControlType"]
    pub enum ControlType {
        kDutyCycle,
        kVelocity,
        kVoltage,
        kPosition,
        kSmartMotion,
        kCurrent,
        kSmartVelocity,
    }

    #[repr(u32)]
    #[cxx_name = "SparkMaxPIDController_ArbFFUnits"]
    pub enum ArbFFUnits {
        kVoltage,
        kPercentOut,
    }

    #[repr(u32)]
    pub enum REVLibError {
        kOk,
        kError,
        kTimeout,
        kNotImplemented,
        kHALError,
        kCantFindFirmware,
        kFirmwareTooOld,
        kFirmwareTooNew,
        kParamInvalidID,
        kParamMismatchType,
        kParamAccessMode,
        kParamInvalid,
        kParamNotImplementedDeprecated,
        kFollowConfigMismatch,
        kInvalid,
        kSetpointOutOfRange,
        kUnknown,
        kCANDisconnected,
        kDuplicateCANId,
        kInvalidCANId,
        kSparkMaxDataPortAlreadyConfiguredDifferently,
    }

    // SAFETY: probably not
    unsafe extern "C++" {
        include!("frc/cpp/include/rev/REVLibError.h");
        type REVLibError;

        include!("frc/cpp/include/rev/CANSparkMax.h");
        include!("frc/cpp/shims/rev/CANSparkMax.h");
        type CANSparkMax;

        include!("frc/cpp/shims/rev/CANSparkMaxLowLevel.h");
        #[cxx_name = "CANSparkMaxLowLevel_MotorType"]
        type MotorType;
        #[cxx_name = "CANSparkMaxLowLevel_ControlType"]
        type ControlType;

        pub fn CANSparkMax_ctor(deviceID: i32, typ: MotorType) -> UniquePtr<CANSparkMax>;

        pub fn Set(self: Pin<&mut CANSparkMax>, output: f64);
        pub fn Get(self: &CANSparkMax) -> f64;
        pub fn CANSparkMax_GetPIDController(
            motor: Pin<&mut CANSparkMax>,
        ) -> UniquePtr<SparkMaxPIDController>;

        include!("frc/cpp/include/rev/SparkMaxPIDController.h");
        include!("frc/cpp/shims/rev/SparkMaxPIDController.h");
        type SparkMaxPIDController;
        #[cxx_name = "SparkMaxPIDController_ArbFFUnits"]
        type ArbFFUnits;

        pub fn SetReference(
            self: Pin<&mut SparkMaxPIDController>,
            value: f64,
            ctrl: ControlType,
            pidSlot: i32,
            arbFF: f64,
            arbFFUnits: ArbFFUnits,
        ) -> REVLibError;
    }
}

// TODO: Is this correct?
unsafe impl Send for CANSparkMax {}
unsafe impl Sync for CANSparkMax {}
