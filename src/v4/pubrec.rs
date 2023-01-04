use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

use crate::convert::{wrap_packet_read, wrap_packet_write};
use crate::{FixedHeader, WrapperMqttBytesError};

/// Acknowledgement to QoS1 publish
#[pyclass]
pub struct PubRec(::mqttbytes::v4::PubRec);

#[pymethods]
impl PubRec {
    #[new]
    fn new(pkid: u16) -> Self {
        ::mqttbytes::v4::PubRec::new(pkid).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::PubRec::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::PubRec::write)
    }
}

impl From<::mqttbytes::v4::PubRec> for PubRec {
    fn from(p: ::mqttbytes::v4::PubRec) -> Self {
        Self(p)
    }
}
