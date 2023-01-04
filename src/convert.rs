use bytes::{Bytes, BytesMut};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

use crate::{FixedHeader, WrapperMqttBytesError};

pub fn wrap_packet_read<T>(
    fixed_header: FixedHeader,
    bytes: &PyBytes,
    read: impl FnOnce(::mqttbytes::FixedHeader, Bytes) -> Result<T, ::mqttbytes::Error>,
) -> Result<T, WrapperMqttBytesError> {
    let bytes = from_pybytes_to_bytes(bytes);
    read(fixed_header.0, bytes).map_err(Into::into)
}

pub fn wrap_packet_write<T>(
    packet: &T,
    buffer: &PyByteArray,
    write: impl FnOnce(&T, &mut BytesMut) -> Result<usize, ::mqttbytes::Error>,
) -> PyResult<usize> {
    let mut bytes = from_pybytearray_to_bytes_mut(buffer);
    let result = write(packet, &mut bytes).map_err(|err| WrapperMqttBytesError(err).into());
    write_pybytearray_from_bytes_mut(buffer, &bytes)?;
    result
}

pub fn from_pybytes_to_bytes(bytes: &PyBytes) -> Bytes {
    Bytes::from(bytes.as_bytes().to_owned())
}

pub fn from_pybytearray_to_bytes_mut(bytes: &PyByteArray) -> BytesMut {
    unsafe { BytesMut::from(bytes.as_bytes()) }
}

pub fn write_pybytearray_from_bytes_mut(buffer: &PyByteArray, bytes: &BytesMut) -> PyResult<()> {
    buffer.resize(bytes.len())?;
    unsafe {
        buffer.as_bytes_mut().copy_from_slice(bytes);
    }
    Ok(())
}
