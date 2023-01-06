use bytes::BytesMut;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::{FixedHeader, QoS, WrapperMqttBytesError};

/// Subscription packet.
#[pyclass(module = "mqttbytes.v4")]
pub struct Subscribe(::mqttbytes::v4::Subscribe);

#[pymethods]
impl Subscribe {
    #[new]
    fn new(path: Option<&PyAny>, qos: Option<QoS>) -> PyResult<Self> {
        match (path, qos) {
            (Some(path), Some(qos)) => {
                let path = path.extract::<String>()?;
                Ok(::mqttbytes::v4::Subscribe::new(path, qos.into()).into())
            }
            (Some(path), None) => {
                let topics = path.extract::<Vec<SubscribeFilter>>()?;
                Ok(
                    ::mqttbytes::v4::Subscribe::new_many(topics.into_iter().map(|topic| topic.0))
                        .into(),
                )
            }
            (None, None) => Ok(::mqttbytes::v4::Subscribe::empty_subscribe().into()),
            _ => Err(PyTypeError::new_err("Invalid arguments")),
        }
    }

    fn add(&mut self, path: String, qos: QoS) {
        self.0.add(path, qos.into());
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: Vec<u8>) -> Result<Self, WrapperMqttBytesError> {
        ::mqttbytes::v4::Subscribe::read(fixed_header.0, bytes.into())
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

    #[getter]
    fn get_filters(&self) -> Vec<SubscribeFilter> {
        self.0.filters.iter().cloned().map(Into::into).collect()
    }

    #[setter]
    fn set_filters(&mut self, filters: Vec<SubscribeFilter>) {
        self.0.filters = filters.into_iter().map(|filter| filter.0).collect();
    }
}

impl From<::mqttbytes::v4::Subscribe> for Subscribe {
    fn from(p: ::mqttbytes::v4::Subscribe) -> Self {
        Self(p)
    }
}

/// Subscription filter.
#[pyclass(module = "mqttbytes.v4")]
#[derive(Clone)]
pub struct SubscribeFilter(::mqttbytes::v4::SubscribeFilter);

#[pymethods]
impl SubscribeFilter {
    #[new]
    fn new(path: String, qos: QoS) -> Self {
        ::mqttbytes::v4::SubscribeFilter::new(path, qos.into()).into()
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    #[getter]
    fn get_path(&self) -> String {
        self.0.path.clone()
    }

    #[setter]
    fn set_path(&mut self, path: String) {
        self.0.path = path;
    }

    #[getter]
    fn get_qos(&self) -> QoS {
        self.0.qos.into()
    }

    #[setter]
    fn set_qos(&mut self, qos: QoS) {
        self.0.qos = qos.into();
    }
}

impl From<::mqttbytes::v4::SubscribeFilter> for SubscribeFilter {
    fn from(p: ::mqttbytes::v4::SubscribeFilter) -> Self {
        Self(p)
    }
}

#[pyclass(module = "mqttbytes.v4")]
pub enum RetainForwardRule {
    OnEverySubscribe,
    OnNewSubscribe,
    Never,
}
