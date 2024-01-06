use std::{sync::Arc, time::Duration, future::Future};

use anyhow::Result;
use nalgebra::{Matrix2x4, Vector2, U2, U4, U1, Matrix2xX, Dyn, Matrix, Dim, RawStorage, Scalar};
use num_traits::Zero;
use tokio::sync::Mutex;

use crate::{
    can::CANId,
    ffi,
    rev::{MotorType, SparkMax},
    server::subsystem::ExampleSubsystem,
};

mod subsystem;

/*#[frc_derive::subsystem]
#[frc(path = crate)]
mod a {
    #[subsystem(type)]
    pub struct A {
        pub b: i32,
        pub c: i32,
    }

    impl A {
        pub fn hi(&self) {
            // hi
            let abcd: i32 = 1;
            let bcde: i64 = 2;
            let cdef: i32 = 3;
        }
    }
}*/

#[no_mangle]
pub extern "C" fn main_rs() {
    //let s = a::A { b: 1, c: 3 };
    main().unwrap();
}



trait Command: Future {
    type F: Future<Output=()>;
    fn interrupt(self) -> Self::F;
}

struct Cmd<T, Int: Future<Output = ()>> {
    future: T,
    interrupted: Int,
}

struct A<T: Future<Output=()>> {
    inner: T,
}

impl<T: Future<Output=()>> Future for A<T> {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

impl<T: Future<Output=()>> Command for A<T> {
    type F = T;

    fn interrupt(self) -> Self::F {
        async { self.inner.await }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    #[rustfmt::skip]
    let mod_translations = Matrix2xX::from_column_slice(&[
        -1.0, -1.0,
        -1.0, 1.0,
        1.0, 1.0,
        1.0, -1.0,
    ]);
    let drivetrain = crate::swerve::SwerveKinematics::new(&mod_translations);
    let wheel_vels = drivetrain.to_module_states(Vector2::new(1.0, 0.0), 1.0);
    dbg!(wheel_vels);
    println!("hi");
    dbg!("hi");
    panic!();
    let mut neo = SparkMax::new(CANId(7), MotorType::Brushless);
    let (subs, subs_view) = {
        let handle = ExampleSubsystem::new();
        let view = handle.make_view();
        (Arc::new(Mutex::new(handle)), view)
    };
    let neo_view = neo.get_view();
    tokio::join!(
        async {
            loop {
                if let Ok(rate) = neo.set(0.001).await {
                    rate.await;
                } else {
                    break;
                }
            }
        },
        async {
            loop {
                println!("output: {}", neo_view.get().await.unwrap());
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        },
        async {
            loop {
                dbg!(ffi::DriverStation_GetAlliance());
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        },
        async {
            let mut subsys = subs.lock().await;
            loop {
                let mut subs = subsys.write().await;
                subs.do_the_thing().await;
                subs.set_condition(true).await;
                drop(subs);
                tokio::time::sleep(Duration::from_secs(1)).await;
                let mut subs = subsys.write().await;
                println!("cond0: {:?}", subs.example_condition().await);
                subs.set_condition(false).await;
                drop(subs);
                tokio::time::sleep(Duration::from_secs(1)).await;
                let subs = subsys.read().await;
                println!("cond1: {:?}", subs.example_condition().await);
            }
        },
        async {
            loop {
                let subs = subs_view.read().await;
                println!("AMONGUS: {}", subs.example_condition().await);
                tokio::time::sleep(Duration::from_millis(50)).await;
                tokio::task::yield_now().await;
            }
        }
    );
    Ok(())
}
