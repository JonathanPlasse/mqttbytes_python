use pyo3::prelude::*;

use connack::{ConnAck, ConnectReturnCode};
use connect::{Connect, LastWill, Login};

mod connack;
mod connect;

#[pymodule]
pub fn v4(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ConnAck>()?;
    m.add_class::<Connect>()?;
    m.add_class::<LastWill>()?;
    m.add_class::<Login>()?;
    m.add_class::<ConnectReturnCode>()?;
    Ok(())
}
