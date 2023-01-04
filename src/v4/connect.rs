use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

use crate::convert::{wrap_packet_read, wrap_packet_write};
use crate::{FixedHeader, Protocol, QoS, WrapperMqttBytesError};

/// Connection packet initiated by the client
#[pyclass]
pub struct Connect(::mqttbytes::v4::Connect);

#[pymethods]
impl Connect {
    #[new]
    fn new(id: String) -> Self {
        ::mqttbytes::v4::Connect::new(id).into()
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    #[staticmethod]
    fn read(fixed_header: FixedHeader, bytes: &PyBytes) -> Result<Self, WrapperMqttBytesError> {
        wrap_packet_read(fixed_header, bytes, ::mqttbytes::v4::Connect::read).map(Into::into)
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::Connect::write)
    }

    #[getter]
    fn get_protocol(&self) -> Protocol {
        self.0.protocol.into()
    }

    #[setter]
    fn set_protocol(&mut self, protocol: Protocol) {
        self.0.protocol = protocol.into();
    }

    #[getter]
    fn get_keep_alive(&self) -> u16 {
        self.0.keep_alive
    }

    #[setter]
    fn set_keep_alive(&mut self, keep_alive: u16) {
        self.0.keep_alive = keep_alive;
    }

    #[getter]
    fn get_client_id(&self) -> String {
        self.0.client_id.clone()
    }

    #[setter]
    fn set_client_id(&mut self, client_id: String) {
        self.0.client_id = client_id;
    }

    #[getter]
    fn get_clean_session(&self) -> bool {
        self.0.clean_session
    }

    #[setter]
    fn set_clean_session(&mut self, clean_session: bool) {
        self.0.clean_session = clean_session;
    }

    #[getter]
    fn get_last_will(&self) -> Option<LastWill> {
        self.0.last_will.clone().map(Into::into)
    }

    #[setter]
    fn set_last_will(&mut self, last_will: Option<LastWill>) {
        self.0.last_will = last_will.map(|last_will| last_will.0);
    }

    #[getter]
    fn get_login(&self) -> Option<Login> {
        self.0.login.clone().map(Into::into)
    }

    #[setter]
    fn set_login(&mut self, login: Option<Login>) {
        self.0.login = login.map(|login| login.0);
    }
}

impl From<::mqttbytes::v4::Connect> for Connect {
    fn from(connect: ::mqttbytes::v4::Connect) -> Self {
        Self(connect)
    }
}

/// LastWill that broker forwards on behalf of the client
#[pyclass]
#[derive(Clone)]
pub struct LastWill(::mqttbytes::v4::LastWill);

#[pymethods]
impl LastWill {
    #[new]
    fn new(topic: String, payload: Vec<u8>, qos: QoS, retain: bool) -> Self {
        ::mqttbytes::v4::LastWill::new(topic, payload, qos.into(), retain).into()
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
    fn get_message(&self) -> Vec<u8> {
        self.0.message.to_vec()
    }

    #[setter]
    fn set_message(&mut self, message: Vec<u8>) {
        self.0.message = Bytes::from(message);
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
}

impl From<::mqttbytes::v4::LastWill> for LastWill {
    fn from(last_will: ::mqttbytes::v4::LastWill) -> Self {
        Self(last_will)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Login(::mqttbytes::v4::Login);

#[pymethods]
impl Login {
    #[new]
    fn new(u: String, p: String) -> Self {
        ::mqttbytes::v4::Login::new(u, p).into()
    }

    fn validate(&self, username: String, password: String) -> bool {
        self.0.validate(&username, &password)
    }

    #[getter]
    fn get_username(&self) -> String {
        self.0.username.clone()
    }

    #[setter]
    fn set_username(&mut self, username: String) {
        self.0.username = username;
    }

    #[getter]
    fn get_password(&self) -> String {
        self.0.password.clone()
    }

    #[setter]
    fn set_password(&mut self, password: String) {
        self.0.password = password;
    }
}

impl From<::mqttbytes::v4::Login> for Login {
    fn from(login: ::mqttbytes::v4::Login) -> Self {
        Self(login)
    }
}
