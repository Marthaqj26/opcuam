// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeZoneDataType {
    pub offset: Int16,
    pub daylight_saving_in_offset: Boolean,
}

impl MessageInfo for TimeZoneDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::TimeZoneDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TimeZoneDataType> for TimeZoneDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.offset.byte_len();
        size += self.daylight_saving_in_offset.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.offset.encode(stream)?;
        size += self.daylight_saving_in_offset.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let offset = Int16::decode(stream)?;
        let daylight_saving_in_offset = Boolean::decode(stream)?;
        Ok(TimeZoneDataType {
            offset: offset,
            daylight_saving_in_offset: daylight_saving_in_offset,
        })
    }
}
