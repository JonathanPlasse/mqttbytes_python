use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::{FixedHeader, WrapperMqttBytesError};

/// Unsubscribe packet.
#[pyclass(module = "mqttbytes.v4")]
pub struct Unsubscribe(::mqttbytes::v4::Unsubscribe);

#[pymethods]
impl Unsubscribe {
    #[new]
    fn new(topic: String) -> Self {
        ::mqttbytes::v4::Unsubscribe::new(topic).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: Vec<u8>) -> Result<Self, WrapperMqttBytesError> {
        ::mqttbytes::v4::Unsubscribe::read(fixed_header.0, bytes.into())
            .map(Into::into)
            .map_err(Into::into)
    }

    // Rewrite write method to return Python bytes instead of Vec<u8>
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
