#[macro_use]
extern crate serde;

use std::io::Write;

use failure::{Error, ResultExt};
use protobuf::descriptor::FileDescriptorSet;
use protobuf::CodedInputStream;
use serde::{Deserialize, Serialize};
use serde_json::Serializer;
use serde_protobuf::descriptor::Descriptors;
use serde_value::Value;

pub mod formatter;
pub mod serde_protov;

use formatter::CustomFormatter;
use serde_protov::de::Deserializer;

pub struct ProtovDecoder<'a> {
    pub descriptors: Descriptors,
    pub message_type: &'a str,
}

impl<'a> ProtovDecoder<'a> {
    pub fn new(loaded_descs: Vec<FileDescriptorSet>, message_type: &str) -> ProtovDecoder<'_> {
        let mut descriptors = Descriptors::new();
        for fdset in loaded_descs {
            descriptors.add_file_set_proto(&fdset);
        }
        descriptors.resolve_refs();
        ProtovDecoder {
            descriptors,
            message_type,
        }
    }

    pub fn decode_message(&self, data: &[u8]) -> Result<Value, Error> {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer =
            Deserializer::for_named_message(&self.descriptors, self.message_type, stream)
                .with_context(|_| "Couldn't initialize deserializer")?;
        let value =
            Value::deserialize(&mut deserializer).with_context(|_| "Couldn't decode message.")?;
        Ok(value)
    }
    pub fn write_message(
        &self,
        v: Value,
        out: &mut dyn Write,
        formatter: &'a mut CustomFormatter<'a>,
    ) {
        if let Err(e) = v.serialize(&mut Serializer::with_formatter(out, formatter)) {
            panic!("Couldn't serialize message: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
