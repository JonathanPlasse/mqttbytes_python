use pyo3::prelude::*;

use connack::{ConnAck, ConnectReturnCode};
use connect::{Connect, LastWill, Login};
use ping::{PingReq, PingResp};
use puback::PubAck;
use pubcomp::PubComp;
use pubrec::PubRec;
use pubrel::PubRel;

mod connack;
mod connect;
mod ping;
mod puback;
mod pubcomp;
mod pubrec;
mod pubrel;

#[pymodule]
pub fn v4(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ConnAck>()?;
    m.add_class::<Connect>()?;
    m.add_class::<LastWill>()?;
    m.add_class::<Login>()?;
    m.add_class::<PingReq>()?;
    m.add_class::<PingResp>()?;
    m.add_class::<PubAck>()?;
    m.add_class::<PubComp>()?;
    m.add_class::<PubRec>()?;
    m.add_class::<PubRel>()?;
    m.add_class::<ConnectReturnCode>()?;
    Ok(())
}
