use bytes::{Bytes, BytesMut};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

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
