// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::ObjectId;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;

/// The attributes for an object node.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectAttributes {
    pub specified_attributes: UInt32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: UInt32,
    pub user_write_mask: UInt32,
    pub event_notifier: Byte,
}

impl BinaryEncoder<ObjectAttributes> for ObjectAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.event_notifier.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.event_notifier.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let specified_attributes = UInt32::decode(stream)?;
        let display_name = LocalizedText::decode(stream)?;
        let description = LocalizedText::decode(stream)?;
        let write_mask = UInt32::decode(stream)?;
        let user_write_mask = UInt32::decode(stream)?;
        let event_notifier = Byte::decode(stream)?;
        Ok(ObjectAttributes {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            event_notifier,
        })
    }
}
