use std::{
    future::Future,
    ops::{Deref, DerefMut},
    time::Duration,
};

use tokio::{
    spawn,
    sync::{mpsc, oneshot},
    time::Instant,
};

use super::ffi;
use crate::can::CANId;

enum SparkMaxCommand {
    Set(f64),
    Get(oneshot::Sender<f64>),
}

#[derive(Clone)]
pub struct SparkMaxView {
    cmd: mpsc::UnboundedSender<SparkMaxCommand>,
}

pub struct SparkMax {
    inner: SparkMaxView,
    time_last_set: Instant,
}

impl Deref for SparkMax {
    type Target = SparkMaxView;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SparkMax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl SparkMax {
    pub fn new(id: CANId, typ: MotorType) -> Self {
        // TODO: Channel bounds?
        let (tx, mut rx) = mpsc::unbounded_channel::<SparkMaxCommand>();
        let _ = spawn(async move {
            let mut ffi_uptr = ffi::CANSparkMax_ctor(id.0, typ.into());
            // TODO: Error handling?
            assert!(!ffi_uptr.is_null());
            while let Some(cmd) = rx.recv().await {
                // SAFETY: I think?
                // pin_mut once before loop won't work b/c Pin<&mut T> isn't copy,
                // is that an impl detail or a restriction of the borrow system?
                // Regardless, this should (?) be safe because the ptr is never modified in this
                // code, and can't be modified by other code.
                let ptr = unsafe { ffi_uptr.as_mut().unwrap_unchecked() };
                match cmd {
                    SparkMaxCommand::Set(output) => {
                        ptr.Set(output);
                    }
                    SparkMaxCommand::Get(res) => {
                        let _ = res.send(ptr.Get());
                    }
                }
            }
        });
        let v = SparkMaxView { cmd: tx };
        Self {
            inner: v.clone(),
            time_last_set: Instant::now(),
        }
    }

    pub async fn set(
        &mut self,
        output: f64,
    ) -> Result<impl Future<Output = ()> + 'static, mpsc::error::SendError<f64>> {
        // TODO: does map_err maintain trace?
        self.cmd
            .send(SparkMaxCommand::Set(output))
            .map_err(|_| mpsc::error::SendError(output))?;
        // Set instantly, then wait for rate limiting
        // If we make there be x ms between the time that we last returned and the time that we
        // will return, the total cycle will take x ms.
        let timer = tokio::time::sleep_until(self.time_last_set + Duration::from_millis(10));
        // The time that we expect to return is either the time that we're sleeping until, or, if
        // that time is in the past, the time that it is right now.
        self.time_last_set = Instant::max(
            self.time_last_set + Duration::from_millis(10),
            Instant::now(),
        );
        // We could just await the timer here, but we instead return it so that the rate limiting
        // part of this call can be passed around as a future with a static lifetime.
        Ok(timer)
    }

    pub fn get_view(&self) -> SparkMaxView {
        return self.inner.clone();
    }
}

impl SparkMaxView {
    pub async fn get(&self) -> Result<f64, ()> {
        let (tx, rx) = oneshot::channel();
        self.cmd.send(SparkMaxCommand::Get(tx)).unwrap();
        let output = rx.await;
        Ok(output.unwrap())
    }
}

pub enum MotorType {
    Brushed,
    Brushless,
}

impl From<ffi::MotorType> for MotorType {
    fn from(value: ffi::MotorType) -> Self {
        match value {
            ffi::MotorType::kBrushed => MotorType::Brushed,
            ffi::MotorType::kBrushless => MotorType::Brushless,
            _ => panic!("Invalid CANSparkMaxLowLevel::MotorType from ffi"),
        }
    }
}

impl From<MotorType> for ffi::MotorType {
    fn from(value: MotorType) -> Self {
        match value {
            MotorType::Brushed => Self::kBrushed,
            MotorType::Brushless => Self::kBrushless,
        }
    }
}
