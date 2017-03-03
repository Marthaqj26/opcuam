// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MonitoredItemNotification {
    pub client_handle: UInt32,
    pub value: DataValue,
}

impl MessageInfo for MonitoredItemNotification {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoredItemNotification_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoredItemNotification> for MonitoredItemNotification {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.client_handle.byte_len();
        size += self.value.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.client_handle.encode(stream)?;
        size += self.value.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let client_handle = UInt32::decode(stream)?;
        let value = DataValue::decode(stream)?;
        Ok(MonitoredItemNotification {
            client_handle: client_handle,
            value: value,
        })
    }
}
