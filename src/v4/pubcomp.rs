use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

use crate::convert::{wrap_packet_read, wrap_packet_write};
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
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::PubComp::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::PubComp::write)
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
