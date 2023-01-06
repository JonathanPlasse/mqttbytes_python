use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::{FixedHeader, QoS, WrapperMqttBytesError};

/// Publish packet.
#[pyclass(module = "mqttbytes.v4")]
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
    fn read(fixed_header: FixedHeader, bytes: Vec<u8>) -> Result<Self, WrapperMqttBytesError> {
        ::mqttbytes::v4::Publish::read(fixed_header.0, bytes.into())
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
    fn get_dup(&self) -> bool {
        self.0.dup
    }

    #[setter]
    fn set_dup(&mut self, dup: bool) {
        self.0.dup = dup;
    }

    #[getter]
    fn get_qos(&self) -> QoS {
        self.0.qos.into()
    }

    #[setter]
    fn set_qos(&mut self, qos: QoS) {
        self.0.qos = qos.into();
    }

    #[getter]
    fn get_retain(&self) -> bool {
        self.0.retain
    }

    #[setter]
    fn set_retain(&mut self, retain: bool) {
        self.0.retain = retain;
    }

    #[getter]
    fn get_topic(&self) -> String {
        self.0.topic.clone()
    }

    #[setter]
    fn set_topic(&mut self, topic: String) {
        self.0.topic = topic;
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
    fn get_payload(&self) -> Vec<u8> {
        self.0.payload.to_vec()
    }

    #[setter]
    fn set_payload(&mut self, payload: Vec<u8>) {
        self.0.payload = payload.into();
    }
}

impl From<::mqttbytes::v4::Publish> for Publish {
    fn from(p: ::mqttbytes::v4::Publish) -> Self {
        Self(p)
    }
}
