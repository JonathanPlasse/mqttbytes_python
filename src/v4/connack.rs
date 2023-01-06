use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::{FixedHeader, WrapperMqttBytesError};

/// Acknowledgement to connect packet.
#[pyclass(module = "mqttbytes.v4")]
pub struct ConnAck(::mqttbytes::v4::ConnAck);

#[pymethods]
impl ConnAck {
    #[new]
    fn new(code: ConnectReturnCode, session_present: bool) -> Self {
        ::mqttbytes::v4::ConnAck::new(code.into(), session_present).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: Vec<u8>) -> Result<Self, WrapperMqttBytesError> {
        ::mqttbytes::v4::ConnAck::read(fixed_header.0, bytes.into())
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
    fn get_session_present(&self) -> bool {
        self.0.session_present
    }

    #[setter]
    fn set_session_present(&mut self, session_present: bool) {
        self.0.session_present = session_present;
    }

    #[getter]
    fn get_code(&self) -> ConnectReturnCode {
        self.0.code.into()
    }

    #[setter]
    fn set_code(&mut self, code: ConnectReturnCode) {
        self.0.code = code.into();
    }
}

impl From<::mqttbytes::v4::ConnAck> for ConnAck {
    fn from(connack: ::mqttbytes::v4::ConnAck) -> Self {
        Self(connack)
    }
}

/// Return code in connack.
#[pyclass(module = "mqttbytes.v4")]
#[derive(Clone)]
#[repr(u8)]
pub enum ConnectReturnCode {
    Success = 0,
    RefusedProtocolVersion,
    BadClientId,
    ServiceUnavailable,
    BadUserNamePassword,
    NotAuthorized,
}

impl From<::mqttbytes::v4::ConnectReturnCode> for ConnectReturnCode {
    fn from(code: ::mqttbytes::v4::ConnectReturnCode) -> Self {
        match code {
            ::mqttbytes::v4::ConnectReturnCode::Success => ConnectReturnCode::Success,
            ::mqttbytes::v4::ConnectReturnCode::RefusedProtocolVersion => {
                ConnectReturnCode::RefusedProtocolVersion
            }
            ::mqttbytes::v4::ConnectReturnCode::BadClientId => ConnectReturnCode::BadClientId,
            ::mqttbytes::v4::ConnectReturnCode::ServiceUnavailable => {
                ConnectReturnCode::ServiceUnavailable
            }
            ::mqttbytes::v4::ConnectReturnCode::BadUserNamePassword => {
                ConnectReturnCode::BadUserNamePassword
            }
            ::mqttbytes::v4::ConnectReturnCode::NotAuthorized => ConnectReturnCode::NotAuthorized,
        }
    }
}

impl From<ConnectReturnCode> for ::mqttbytes::v4::ConnectReturnCode {
    fn from(code: ConnectReturnCode) -> Self {
        match code {
            ConnectReturnCode::Success => ::mqttbytes::v4::ConnectReturnCode::Success,
            ConnectReturnCode::RefusedProtocolVersion => {
                ::mqttbytes::v4::ConnectReturnCode::RefusedProtocolVersion
            }
            ConnectReturnCode::BadClientId => ::mqttbytes::v4::ConnectReturnCode::BadClientId,
            ConnectReturnCode::ServiceUnavailable => {
                ::mqttbytes::v4::ConnectReturnCode::ServiceUnavailable
            }
            ConnectReturnCode::BadUserNamePassword => {
                ::mqttbytes::v4::ConnectReturnCode::BadUserNamePassword
            }
            ConnectReturnCode::NotAuthorized => ::mqttbytes::v4::ConnectReturnCode::NotAuthorized,
        }
    }
}
