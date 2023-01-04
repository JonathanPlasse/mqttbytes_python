use bytes::Bytes;
use pyo3::{
    prelude::*,
    types::{PyByteArray, PyBytes},
};

use crate::convert::{wrap_packet_read, wrap_packet_write};
use crate::{FixedHeader, Protocol, QoS, WrapperMqttBytesError};

/// Connection packet initiated by the client
#[pyclass]
pub struct Connect {
    /// Mqtt protocol version
    #[pyo3(get, set)]
    pub protocol: Protocol,
    /// Mqtt keep alive time
    #[pyo3(get, set)]
    pub keep_alive: u16,
    /// Client Id
    #[pyo3(get, set)]
    pub client_id: String,
    /// Clean session. Asks the broker to clear previous state
    #[pyo3(get, set)]
    pub clean_session: bool,
    /// Will that broker needs to publish when the client disconnects
    #[pyo3(get, set)]
    pub last_will: Option<LastWill>,
    /// Login credentials
    #[pyo3(get)]
    pub login: Option<Login>,
}

#[pymethods]
impl Connect {
    #[new]
    fn new(id: String) -> Self {
        ::mqttbytes::v4::Connect::new(id).into()
    }

    fn set_login(&mut self, u: String, p: String) {
        *self = ::mqttbytes::v4::Connect::set_login(&mut self.into(), u, p).into();
    }

    fn __len__(&self) -> usize {
        ::mqttbytes::v4::Connect::len(&self.into())
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::Connect::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.into(), buffer, ::mqttbytes::v4::Connect::write)
    }
}

impl From<::mqttbytes::v4::Connect> for Connect {
    fn from(connect: ::mqttbytes::v4::Connect) -> Self {
        Connect {
            protocol: connect.protocol.into(),
            keep_alive: connect.keep_alive,
            client_id: connect.client_id,
            clean_session: connect.clean_session,
            last_will: connect.last_will.map(Into::into),
            login: connect.login.map(Into::into),
        }
    }
}

impl From<&mut ::mqttbytes::v4::Connect> for Connect {
    fn from(connect: &mut ::mqttbytes::v4::Connect) -> Self {
        Connect {
            protocol: connect.protocol.into(),
            keep_alive: connect.keep_alive,
            client_id: connect.client_id.clone(),
            clean_session: connect.clean_session,
            last_will: connect.last_will.as_ref().map(Into::into),
            login: connect.login.as_ref().map(Into::into),
        }
    }
}

impl From<&Connect> for ::mqttbytes::v4::Connect {
    fn from(connect: &Connect) -> Self {
        ::mqttbytes::v4::Connect {
            protocol: connect.protocol.into(),
            keep_alive: connect.keep_alive,
            client_id: connect.client_id.clone(),
            clean_session: connect.clean_session,
            last_will: connect.last_will.as_ref().map(Into::into),
            login: connect.login.as_ref().map(Into::into),
        }
    }
}

impl From<&mut Connect> for ::mqttbytes::v4::Connect {
    fn from(connect: &mut Connect) -> Self {
        ::mqttbytes::v4::Connect {
            protocol: connect.protocol.into(),
            keep_alive: connect.keep_alive,
            client_id: connect.client_id.clone(),
            clean_session: connect.clean_session,
            last_will: connect.last_will.as_ref().map(Into::into),
            login: connect.login.as_ref().map(Into::into),
        }
    }
}

/// LastWill that broker forwards on behalf of the client
#[pyclass]
#[derive(Clone)]
pub struct LastWill {
    #[pyo3(get, set)]
    pub topic: String,
    #[pyo3(get, set)]
    pub message: Vec<u8>,
    #[pyo3(get, set)]
    pub qos: QoS,
    #[pyo3(get, set)]
    pub retain: bool,
}

#[pymethods]
impl LastWill {
    #[new]
    fn new(topic: String, payload: Vec<u8>, qos: QoS, retain: bool) -> Self {
        ::mqttbytes::v4::LastWill::new(topic, payload, qos.into(), retain).into()
    }
}

impl From<::mqttbytes::v4::LastWill> for LastWill {
    fn from(last_will: ::mqttbytes::v4::LastWill) -> Self {
        LastWill {
            topic: last_will.topic,
            message: last_will.message.to_vec(),
            qos: last_will.qos.into(),
            retain: last_will.retain,
        }
    }
}

impl From<&::mqttbytes::v4::LastWill> for LastWill {
    fn from(last_will: &::mqttbytes::v4::LastWill) -> Self {
        LastWill {
            topic: last_will.topic.clone(),
            message: last_will.message.to_vec(),
            qos: last_will.qos.into(),
            retain: last_will.retain,
        }
    }
}

impl From<&LastWill> for ::mqttbytes::v4::LastWill {
    fn from(last_will: &LastWill) -> Self {
        ::mqttbytes::v4::LastWill {
            topic: last_will.topic.clone(),
            message: Bytes::from(last_will.message.clone()),
            qos: last_will.qos.into(),
            retain: last_will.retain,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Login {
    #[pyo3(get, set)]
    pub username: String,
    #[pyo3(get, set)]
    pub password: String,
}

#[pymethods]
impl Login {
    #[new]
    fn new(u: String, p: String) -> Self {
        ::mqttbytes::v4::Login::new(u, p).into()
    }

    fn validate(&self, username: String, password: String) -> bool {
        ::mqttbytes::v4::Login::validate(&self.into(), &username, &password)
    }
}

impl From<::mqttbytes::v4::Login> for Login {
    fn from(login: ::mqttbytes::v4::Login) -> Self {
        Login {
            username: login.username,
            password: login.password,
        }
    }
}

impl From<&::mqttbytes::v4::Login> for Login {
    fn from(login: &::mqttbytes::v4::Login) -> Self {
        Login {
            username: login.username.clone(),
            password: login.password.clone(),
        }
    }
}

impl From<&Login> for ::mqttbytes::v4::Login {
    fn from(login: &Login) -> Self {
        ::mqttbytes::v4::Login {
            username: login.username.clone(),
            password: login.password.clone(),
        }
    }
}
