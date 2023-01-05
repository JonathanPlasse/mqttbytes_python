use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

use crate::convert::{wrap_packet_read, wrap_packet_write};
use crate::{FixedHeader, WrapperMqttBytesError};

/// Unsubscribe packet
#[pyclass]
pub struct Unsubscribe(::mqttbytes::v4::Unsubscribe);

#[pymethods]
impl Unsubscribe {
    #[new]
    fn new(topic: String) -> Self {
        ::mqttbytes::v4::Unsubscribe::new(topic).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::Unsubscribe::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::Unsubscribe::write)
    }

    #[getter]
    fn get_pkid(&self) -> u16 {
        self.0.pkid
    }

    #[setter]
    fn set_pkid(&mut self, pkid: u16) {
        self.0.pkid = pkid;
    }

    #[getter]
    fn get_topics(&self) -> Vec<String> {
        self.0.topics.clone()
    }

    #[setter]
    fn set_topics(&mut self, topics: Vec<String>) {
        self.0.topics = topics;
    }
}

impl From<::mqttbytes::v4::Unsubscribe> for Unsubscribe {
    fn from(p: ::mqttbytes::v4::Unsubscribe) -> Self {
        Self(p)
    }
}
