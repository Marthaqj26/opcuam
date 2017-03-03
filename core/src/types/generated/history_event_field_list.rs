// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEventFieldList {
    pub event_fields: Option<Vec<Variant>>,
}

impl MessageInfo for HistoryEventFieldList {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryEventFieldList_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryEventFieldList> for HistoryEventFieldList {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.event_fields);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.event_fields)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let event_fields: Option<Vec<Variant>> = read_array(stream)?;
        Ok(HistoryEventFieldList {
            event_fields: event_fields,
        })
    }
}
