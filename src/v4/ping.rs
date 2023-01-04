use pyo3::prelude::*;
use pyo3::types::PyByteArray;

use crate::convert::wrap_packet_write;

#[pyclass]
pub struct PingReq(::mqttbytes::v4::PingReq);

#[pymethods]
impl PingReq {
    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::PingReq::write)
    }
}

#[pyclass]
pub struct PingResp(::mqttbytes::v4::PingResp);

#[pymethods]
impl PingResp {
    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::PingResp::write)
    }
}
