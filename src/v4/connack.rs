use pyo3::{
    prelude::*,
    types::{PyByteArray, PyBytes},
};

use crate::convert::{
    from_pybytearray_to_bytes_mut, from_pybytes_to_bytes, write_pybytearray_from_bytes_mut,
};
use crate::{FixedHeader, WrapperMqttBytesError};

/// Acknowledgement to connect packet
#[pyclass]
pub struct ConnAck {
    #[pyo3(get, set)]
    session_present: bool,
    #[pyo3(get, set)]
    code: ConnectReturnCode,
}

#[pymethods]
impl ConnAck {
    #[new]
    fn new(code: ConnectReturnCode, session_present: bool) -> ConnAck {
        ::mqttbytes::v4::ConnAck::new(code.into(), session_present).into()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        let bytes = from_pybytes_to_bytes(bytes);
        ::mqttbytes::v4::ConnAck::read(fixed_header.0, bytes)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        let mut bytes = from_pybytearray_to_bytes_mut(buffer);
        let result = ::mqttbytes::v4::ConnAck::from(self)
            .write(&mut bytes)
            .map_err(|err| WrapperMqttBytesError(err).into());
        write_pybytearray_from_bytes_mut(buffer, &bytes)?;
        result
    }
}

impl From<::mqttbytes::v4::ConnAck> for ConnAck {
    fn from(conn_ack: ::mqttbytes::v4::ConnAck) -> Self {
        ConnAck {
            session_present: conn_ack.session_present,
            code: conn_ack.code.into(),
        }
    }
}

impl From<&ConnAck> for ::mqttbytes::v4::ConnAck {
    fn from(conn_ack: &ConnAck) -> Self {
        ::mqttbytes::v4::ConnAck {
            session_present: conn_ack.session_present,
            code: conn_ack.code.into(),
        }
    }
}

/// Return code in connack
#[pyclass]
#[derive(Clone, Copy)]
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
