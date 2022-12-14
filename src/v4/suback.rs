use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::{FixedHeader, QoS, WrapperMqttBytesError};

/// Acknowledgement to subscribe.
#[pyclass(module = "mqttbytes.v4")]
pub struct SubAck(::mqttbytes::v4::SubAck);

#[pymethods]
impl SubAck {
    #[new]
    fn new(pkid: u16, return_codes: Vec<Option<QoS>>) -> Self {
        ::mqttbytes::v4::SubAck::new(
            pkid,
            from_vec_option_qos_to_vec_subscribe_reason_code(return_codes),
        )
        .into()
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: Vec<u8>) -> Result<Self, WrapperMqttBytesError> {
        ::mqttbytes::v4::SubAck::read(fixed_header.0, bytes.into())
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
    fn get_return_codes(&self) -> Vec<Option<QoS>> {
        from_vec_subscribe_reason_code_to_vec_option_qos(self.0.return_codes.clone())
    }

    #[setter]
    fn set_return_codes(&mut self, return_codes: Vec<Option<QoS>>) {
        self.0.return_codes = from_vec_option_qos_to_vec_subscribe_reason_code(return_codes);
    }
}

impl From<::mqttbytes::v4::SubAck> for SubAck {
    fn from(p: ::mqttbytes::v4::SubAck) -> Self {
        Self(p)
    }
}

fn from_option_qos_to_subscribe_reason_code(
    return_code: Option<QoS>,
) -> ::mqttbytes::v4::SubscribeReasonCode {
    match return_code {
        Some(qos) => ::mqttbytes::v4::SubscribeReasonCode::Success(qos.into()),
        None => ::mqttbytes::v4::SubscribeReasonCode::Failure,
    }
}

fn from_vec_option_qos_to_vec_subscribe_reason_code(
    return_codes: Vec<Option<QoS>>,
) -> Vec<::mqttbytes::v4::SubscribeReasonCode> {
    return_codes
        .into_iter()
        .map(from_option_qos_to_subscribe_reason_code)
        .collect()
}

fn from_subscribe_reason_code_to_option_qos(
    return_code: ::mqttbytes::v4::SubscribeReasonCode,
) -> Option<QoS> {
    match return_code {
        ::mqttbytes::v4::SubscribeReasonCode::Success(qos) => Some(qos.into()),
        ::mqttbytes::v4::SubscribeReasonCode::Failure => None,
    }
}

fn from_vec_subscribe_reason_code_to_vec_option_qos(
    return_codes: Vec<::mqttbytes::v4::SubscribeReasonCode>,
) -> Vec<Option<QoS>> {
    return_codes
        .into_iter()
        .map(from_subscribe_reason_code_to_option_qos)
        .collect()
}
