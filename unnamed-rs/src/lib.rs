//#![feature(async_fn_in_trait)]
//#![feature(return_position_impl_trait_in_trait)]
//#![feature(type_alias_impl_trait)]
//#![feature(impl_trait_in_assoc_type)]
#![feature(type_name_of_val)]

use num_traits::{NumCast, ToPrimitive};

pub mod can;
pub mod rev;
pub mod subsystem;
pub mod swerve;
pub mod wpi;

mod server;

#[cxx::bridge(namespace = "frc")]
//#[namespace="frc::DriverStation"]
mod ffi {
    #[repr(u32)]
    #[derive(Debug, PartialEq, Eq)]
    //#[namespace="frc::DriverStation"]
    #[cxx_name = "DriverStation_Alliance"]
    enum Alliance {
        kRed,
        kBlue,
        kInvalid,
    }

    unsafe extern "C++" {
        include!("frc/cpp/include/frc/DriverStation.h");
        include!("frc/cpp/shims/frc/DriverStation.h");

        #[cxx_name = "DriverStation_Alliance"]
        type Alliance;

        fn DriverStation_GetAlliance() -> Alliance;
    }
}
