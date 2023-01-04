use pyo3::{
    prelude::*,
    types::{PyByteArray, PyBytes},
};

use crate::convert::{wrap_packet_read, wrap_packet_write};
use crate::{FixedHeader, WrapperMqttBytesError};

/// Acknowledgement to connect packet
#[pyclass]
pub struct ConnAck(::mqttbytes::v4::ConnAck);

#[pymethods]
impl ConnAck {
    #[new]
    fn new(code: ConnectReturnCode, session_present: bool) -> Self {
        ::mqttbytes::v4::ConnAck::new(code.into(), session_present).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::ConnAck::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::ConnAck::write)
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

/// Return code in connack
#[pyclass]
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
