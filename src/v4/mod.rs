use pyo3::prelude::*;

use connack::*;
use connect::*;
use disconnect::*;
use ping::*;
use puback::*;
use pubcomp::*;
use publish::*;
use pubrec::*;
use pubrel::*;
use suback::*;
use subscribe::*;
use unsuback::*;
use unsubscribe::*;

use crate::WrapperMqttBytesError;

mod connack;
mod connect;
mod disconnect;
mod ping;
mod puback;
mod pubcomp;
mod publish;
mod pubrec;
mod pubrel;
mod suback;
mod subscribe;
mod unsuback;
mod unsubscribe;

/// Reads a stream of bytes and extracts next MQTT packet out of it.
#[pyfunction]
fn read(_py: Python, bytes: Vec<u8>, max_size: usize) -> Result<PyObject, WrapperMqttBytesError> {
    let bytes: &[u8] = &bytes;
    ::mqttbytes::v4::read(&mut bytes.into(), max_size)
        .map(|packet| match packet {
            ::mqttbytes::v4::Packet::Connect(packet) => Connect::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::ConnAck(packet) => ConnAck::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::Publish(packet) => Publish::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::PubAck(packet) => PubAck::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::PubRec(packet) => PubRec::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::PubRel(packet) => PubRel::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::PubComp(packet) => PubComp::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::Subscribe(packet) => Subscribe::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::SubAck(packet) => SubAck::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::Unsubscribe(packet) => Unsubscribe::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::UnsubAck(packet) => UnsubAck::from(packet).into_py(_py),
            ::mqttbytes::v4::Packet::PingReq => PingReq(::mqttbytes::v4::PingReq).into_py(_py),
            ::mqttbytes::v4::Packet::PingResp => PingResp(::mqttbytes::v4::PingResp).into_py(_py),
            ::mqttbytes::v4::Packet::Disconnect => {
                Disconnect(::mqttbytes::v4::Disconnect).into_py(_py)
            }
        })
        .map_err(Into::into)
}

#[pymodule]
pub fn v4(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ConnAck>()?;
    m.add_class::<Connect>()?;
    m.add_class::<Disconnect>()?;
    m.add_class::<LastWill>()?;
    m.add_class::<Login>()?;
    m.add_class::<PingReq>()?;
    m.add_class::<PingResp>()?;
    m.add_class::<PubAck>()?;
    m.add_class::<PubComp>()?;
    m.add_class::<PubRec>()?;
    m.add_class::<PubRel>()?;
    m.add_class::<Publish>()?;
    m.add_class::<SubAck>()?;
    m.add_class::<Subscribe>()?;
    m.add_class::<UnsubAck>()?;
    m.add_class::<Unsubscribe>()?;
    m.add_class::<ConnectReturnCode>()?;
    m.add_class::<RetainForwardRule>()?;
    m.add_class::<SubscribeFilter>()?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
