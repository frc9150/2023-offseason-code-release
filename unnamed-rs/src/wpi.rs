/*use core::slice;
use std::{mem, time::Duration};

use anyhow::Result;
use nng::{
    options::{protocol::reqrep::ResendTime, Options},
    Protocol, Socket,
};

use crate::{
    ffi,
    util::{arecv, asend},
};

pub struct WPI {
    pub inner: Socket,
}

#[repr(C)]
pub struct WPIRequest {
    pub typ: WPIRequestType,
}

#[repr(C)]
pub enum WPIRequestType {
    GetAlliance,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub enum Alliance {
    Red,
    Blue,
    #[default]
    Invalid,
}

impl WPI {
    pub fn new(url: &str) -> nng::Result<Self> {
        let inner = Socket::new(Protocol::Req0)?;
        inner.set_opt::<ResendTime>(Some(Duration::from_millis(2000)))?;
        inner.dial(url)?;
        Ok(Self { inner })
    }

    pub async fn get_alliance(&mut self) -> Result<Alliance> {
        let ffi_alliance: crate::ffi::Alliance = crate::ffi::DriverStation_GetAlliance();
        Ok(match ffi_alliance {
            ffi::Alliance::kRed => Alliance::Red,
            ffi::Alliance::kBlue => Alliance::Blue,
            _ => Alliance::Invalid,
        })
        /*let req = WPIRequest {
            typ: WPIRequestType::GetAlliance,
        };
        let mut msg = nng::Message::new();
        msg.push_back(unsafe {
            slice::from_raw_parts(&req as *const _ as *const u8, mem::size_of::<WPIRequest>())
        });
        asend(&mut self.inner, msg).await?;
        let msg = arecv(&mut self.inner).await?;
        let color = match msg.get(0) {
            Some(0) => Alliance::Red,
            Some(1) => Alliance::Blue,
            _ => Alliance::Invalid,
        };
        Ok(color)*/
    }
}*/
