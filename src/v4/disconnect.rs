use pyo3::prelude::*;
use pyo3::types::PyByteArray;

use crate::convert::wrap_packet_write;

#[pyclass(module = "mqttbytes.v4")]
pub struct Disconnect(pub ::mqttbytes::v4::Disconnect);

#[pymethods]
impl Disconnect {
    #[new]
    fn new() -> Self {
        ::mqttbytes::v4::Disconnect.into()
    }

    fn write(&self, buffer: &PyByteArray) -> PyResult<usize> {
        wrap_packet_write(&self.0, buffer, ::mqttbytes::v4::Disconnect::write)
    }
}

impl From<::mqttbytes::v4::Disconnect> for Disconnect {
    fn from(disconnect: ::mqttbytes::v4::Disconnect) -> Self {
        Self(disconnect)
    }
}
