use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::{FixedHeader, WrapperMqttBytesError};

/// Acknowledgement to QoS1 publish.
#[pyclass(module = "mqttbytes.v4")]
pub struct PubComp(::mqttbytes::v4::PubComp);

#[pymethods]
impl PubComp {
    #[new]
    fn new(pkid: u16) -> Self {
        ::mqttbytes::v4::PubComp::new(pkid).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: Vec<u8>) -> Result<Self, WrapperMqttBytesError> {
        ::mqttbytes::v4::PubComp::read(fixed_header.0, bytes.into())
            .map(Into::into)
            .map_err(Into::into)
    }

    fn write(&self, _py: Python) -> Result<Py<PyBytes>, WrapperMqttBytesError> {
        let mut buffer: BytesMut = BytesMut::new();
        self.0
            .write(&mut buffer)
            .map_err(WrapperMqttBytesError::from)?;
        Ok(PyBytes::new(_py, &buffer).into())
    }

    #[getter]
    fn get_pkid(&self) -> u16 {
        self.0.pkid
    }

    #[setter]
    fn set_pkid(&mut self, pkid: u16) {
        self.0.pkid = pkid;
    }
}

impl From<::mqttbytes::v4::PubComp> for PubComp {
    fn from(p: ::mqttbytes::v4::PubComp) -> Self {
        Self(p)
    }
}
