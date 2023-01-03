use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{create_exception, wrap_pymodule};

mod convert;
mod v4;

create_exception!(
    mqttbytes,
    MqttBytesError,
    PyException,
    "Error during serialization and deserialization."
);

struct WrapperMqttBytesError(::mqttbytes::Error);

impl From<::mqttbytes::Error> for WrapperMqttBytesError {
    fn from(err: ::mqttbytes::Error) -> Self {
        Self(err)
    }
}

impl From<WrapperMqttBytesError> for PyErr {
    fn from(err: WrapperMqttBytesError) -> PyErr {
        MqttBytesError::new_err(err.0.to_string())
    }
}

/// Packet type from a byte
///
/// ```ignore
///          7                          3                          0
///          +--------------------------+--------------------------+
/// byte 1   | MQTT Control Packet Type | Flags for each type      |
///          +--------------------------+--------------------------+
///          |         Remaining Bytes Len  (1/2/3/4 bytes)        |
///          +-----------------------------------------------------+
///
/// http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Figure_2.2_-
/// ```
#[pyclass]
#[derive(Clone, Copy)]
struct FixedHeader(::mqttbytes::FixedHeader);

#[pymethods]
impl FixedHeader {
    #[new]
    fn new(byte1: u8, remaining_len_len: usize, remaining_len: usize) -> FixedHeader {
        ::mqttbytes::FixedHeader::new(byte1, remaining_len_len, remaining_len).into()
    }

    fn packet_type(&self) -> Result<PacketType, WrapperMqttBytesError> {
        self.0.packet_type().map(Into::into).map_err(Into::into)
    }

    /// Returns the size of full packet (fixed header + variable header + payload)
    /// Fixed header is enough to get the size of a frame in the stream
    fn frame_length(&self) -> usize {
        self.0.frame_length()
    }
}

impl From<::mqttbytes::FixedHeader> for FixedHeader {
    fn from(fixed_header: ::mqttbytes::FixedHeader) -> Self {
        Self(fixed_header)
    }
}

/// MQTT packet type.
#[pyclass]
#[repr(u8)]
enum PacketType {
    Connect = 1,
    ConnAck,
    Publish,
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    Unsubscribe,
    UnsubAck,
    PingReq,
    PingResp,
    Disconnect,
}

impl From<::mqttbytes::PacketType> for PacketType {
    fn from(packet_type: ::mqttbytes::PacketType) -> Self {
        match packet_type {
            ::mqttbytes::PacketType::Connect => PacketType::Connect,
            ::mqttbytes::PacketType::ConnAck => PacketType::ConnAck,
            ::mqttbytes::PacketType::Publish => PacketType::Publish,
            ::mqttbytes::PacketType::PubAck => PacketType::PubAck,
            ::mqttbytes::PacketType::PubRec => PacketType::PubRec,
            ::mqttbytes::PacketType::PubRel => PacketType::PubRel,
            ::mqttbytes::PacketType::PubComp => PacketType::PubComp,
            ::mqttbytes::PacketType::Subscribe => PacketType::Subscribe,
            ::mqttbytes::PacketType::SubAck => PacketType::SubAck,
            ::mqttbytes::PacketType::Unsubscribe => PacketType::Unsubscribe,
            ::mqttbytes::PacketType::UnsubAck => PacketType::UnsubAck,
            ::mqttbytes::PacketType::PingReq => PacketType::PingReq,
            ::mqttbytes::PacketType::PingResp => PacketType::PingResp,
            ::mqttbytes::PacketType::Disconnect => PacketType::Disconnect,
        }
    }
}

#[pyclass]
enum Protocol {
    V4,
    V5,
}

/// Quality of service.
#[allow(clippy::enum_variant_names)]
#[pyclass]
#[repr(u8)]
enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl From<::mqttbytes::QoS> for QoS {
    fn from(qos: ::mqttbytes::QoS) -> Self {
        match qos {
            ::mqttbytes::QoS::AtMostOnce => QoS::AtMostOnce,
            ::mqttbytes::QoS::AtLeastOnce => QoS::AtLeastOnce,
            ::mqttbytes::QoS::ExactlyOnce => QoS::ExactlyOnce,
        }
    }
}

/// Checks if the stream has enough bytes to frame a packet and returns fixed header
/// only if a packet can be framed with existing bytes in the `stream`.
/// The passed stream doesn't modify parent stream's cursor. If this function
/// returned an error, next `check` on the same parent stream is forced start
/// with cursor at 0 again (Iter is owned. Only Iter's cursor is changed internally)
#[pyfunction]
fn check(stream: &PyBytes, max_packet_size: usize) -> Result<FixedHeader, WrapperMqttBytesError> {
    ::mqttbytes::check(stream.as_bytes().iter(), max_packet_size)
        .map(Into::into)
        .map_err(Into::into)
}

/// Checks if a topic or topic filter has wildcards
#[pyfunction]
fn has_wildcards(s: &str) -> bool {
    ::mqttbytes::has_wildcards(s)
}

/// Checks if topic matches a filter. topic and filter validation isn't done here.
///
/// **NOTE**: 'topic' is a misnomer in the arg. this can also be used to match 2 wild subscriptions
/// **NOTE**: make sure a topic is validated during a publish and filter is validated
/// during a subscribe
#[pyfunction]
fn matches(topic: &str, filter: &str) -> bool {
    ::mqttbytes::matches(topic, filter)
}

/// Maps a number to QoS
#[pyfunction]
fn qos(num: u8) -> Result<QoS, WrapperMqttBytesError> {
    ::mqttbytes::qos(num).map(Into::into).map_err(Into::into)
}

/// Checks if the filter is valid
///
/// https://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718106
#[pyfunction]
fn valid_filter(filter: &str) -> bool {
    ::mqttbytes::valid_filter(filter)
}

/// Checks if a topic is valid
#[pyfunction]
fn valid_topic(topic: &str) -> bool {
    ::mqttbytes::valid_topic(topic)
}

#[pymodule]
fn mqttbytes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(v4::v4))?;
    m.add_class::<FixedHeader>()?;
    m.add("MqttBytesError", _py.get_type::<MqttBytesError>())?;
    m.add_class::<PacketType>()?;
    m.add_class::<Protocol>()?;
    m.add_class::<QoS>()?;
    m.add_function(wrap_pyfunction!(check, m)?)?;
    m.add_function(wrap_pyfunction!(has_wildcards, m)?)?;
    m.add_function(wrap_pyfunction!(matches, m)?)?;
    m.add_function(wrap_pyfunction!(qos, m)?)?;
    m.add_function(wrap_pyfunction!(valid_filter, m)?)?;
    m.add_function(wrap_pyfunction!(valid_topic, m)?)?;
    Ok(())
}
