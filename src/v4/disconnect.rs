use pyo3::prelude::*;
use pyo3::types::PyByteArray;

use crate::convert::wrap_packet_write;

#[pyclass]
pub struct Disconnect(::mqttbytes::v4::Disconnect);

#[pymethods]
impl Disconnect {
    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::Disconnect::write)
    }
}
