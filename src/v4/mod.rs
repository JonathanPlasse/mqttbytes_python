use pyo3::prelude::*;

use connack::{ConnAck, ConnectReturnCode};

mod connack;

#[pymodule]
pub fn v4(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ConnAck>()?;
    m.add_class::<ConnectReturnCode>()?;
    Ok(())
}
