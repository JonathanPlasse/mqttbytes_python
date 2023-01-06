use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::WrapperMqttBytesError;

#[pyclass(module = "mqttbytes.v4")]
pub struct Disconnect(pub ::mqttbytes::v4::Disconnect);

#[pymethods]
impl Disconnect {
    #[new]
    fn new() -> Self {
        ::mqttbytes::v4::Disconnect.into()
    }

    fn write(&self, _py: Python) -> Result<Py<PyBytes>, WrapperMqttBytesError> {
        let mut buffer: BytesMut = BytesMut::new();
        self.0
            .write(&mut buffer)
            .map_err(WrapperMqttBytesError::from)?;
        Ok(PyBytes::new(_py, &buffer).into())
    }
}

impl From<::mqttbytes::v4::Disconnect> for Disconnect {
    fn from(disconnect: ::mqttbytes::v4::Disconnect) -> Self {
        Self(disconnect)
    }
}
