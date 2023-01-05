use pyo3::prelude::*;
use pyo3::types::PyByteArray;

use crate::convert::wrap_packet_write;

#[pyclass]
pub struct PingReq(pub ::mqttbytes::v4::PingReq);

#[pymethods]
impl PingReq {
    #[new]
    fn new() -> Self {
        ::mqttbytes::v4::PingReq.into()
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::PingReq::write)
    }
}

impl From<::mqttbytes::v4::PingReq> for PingReq {
    fn from(ping_req: ::mqttbytes::v4::PingReq) -> Self {
        Self(ping_req)
    }
}

#[pyclass]
pub struct PingResp(pub ::mqttbytes::v4::PingResp);

#[pymethods]
impl PingResp {
    #[new]
    fn new() -> Self {
        ::mqttbytes::v4::PingResp.into()
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::PingResp::write)
    }
}

impl From<::mqttbytes::v4::PingResp> for PingResp {
    fn from(ping_resp: ::mqttbytes::v4::PingResp) -> Self {
        Self(ping_resp)
    }
}
