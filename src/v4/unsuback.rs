use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

use crate::convert::{wrap_packet_read, wrap_packet_write};
use crate::{FixedHeader, WrapperMqttBytesError};

/// Acknowledgement to unsubscribe.
#[pyclass(module = "mqttbytes.v4")]
pub struct UnsubAck(::mqttbytes::v4::UnsubAck);

#[pymethods]
impl UnsubAck {
    #[new]
    fn new(pkid: u16) -> Self {
        ::mqttbytes::v4::UnsubAck::new(pkid).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::UnsubAck::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::UnsubAck::write)
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

impl From<::mqttbytes::v4::UnsubAck> for UnsubAck {
    fn from(p: ::mqttbytes::v4::UnsubAck) -> Self {
        Self(p)
    }
}
