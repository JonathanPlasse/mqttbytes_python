use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::WrapperMqttBytesError;

#[pyclass(module = "mqttbytes.v4")]
pub struct PingReq(pub ::mqttbytes::v4::PingReq);

#[pymethods]
impl PingReq {
    #[new]
    fn new() -> Self {
        ::mqttbytes::v4::PingReq.into()
    }

    fn write(&self, _py: Python) -> Result<Py<PyBytes>, WrapperMqttBytesError> {
        let mut buffer: BytesMut = BytesMut::new();
        self.0
            .write(&mut buffer)
            .map_err(WrapperMqttBytesError::from)?;
        Ok(PyBytes::new(_py, &buffer).into())
    }
}

impl From<::mqttbytes::v4::PingReq> for PingReq {
    fn from(ping_req: ::mqttbytes::v4::PingReq) -> Self {
        Self(ping_req)
    }
}

#[pyclass(module = "mqttbytes.v4")]
pub struct PingResp(pub ::mqttbytes::v4::PingResp);

#[pymethods]
impl PingResp {
    #[new]
    fn new() -> Self {
        ::mqttbytes::v4::PingResp.into()
    }

    fn write(&self, _py: Python) -> Result<Py<PyBytes>, WrapperMqttBytesError> {
        let mut buffer: BytesMut = BytesMut::new();
        self.0
            .write(&mut buffer)
            .map_err(WrapperMqttBytesError::from)?;
        Ok(PyBytes::new(_py, &buffer).into())
    }
}

impl From<::mqttbytes::v4::PingResp> for PingResp {
    fn from(ping_resp: ::mqttbytes::v4::PingResp) -> Self {
        Self(ping_resp)
    }
}
