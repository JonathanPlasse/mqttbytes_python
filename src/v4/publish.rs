use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

use crate::convert::{wrap_packet_read, wrap_packet_write};
use crate::{FixedHeader, QoS, WrapperMqttBytesError};

/// Publish packet
#[pyclass]
pub struct Publish(::mqttbytes::v4::Publish);

#[pymethods]
impl Publish {
    #[new]
    fn new(topic: String, qos: QoS, payload: Vec<u8>) -> Self {
        ::mqttbytes::v4::Publish::new(topic, qos.into(), payload).into()
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::Publish::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::Publish::write)
    }
}

impl From<::mqttbytes::v4::Publish> for Publish {
    fn from(p: ::mqttbytes::v4::Publish) -> Self {
        Self(p)
    }
}
