// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MonitoredItemModifyRequest {
    pub monitored_item_id: UInt32,
    pub requested_parameters: MonitoringParameters,
}

impl MessageInfo for MonitoredItemModifyRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoredItemModifyRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoredItemModifyRequest> for MonitoredItemModifyRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.monitored_item_id.byte_len();
        size += self.requested_parameters.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.monitored_item_id.encode(stream)?;
        size += self.requested_parameters.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let monitored_item_id = UInt32::decode(stream)?;
        let requested_parameters = MonitoringParameters::decode(stream)?;
        Ok(MonitoredItemModifyRequest {
            monitored_item_id: monitored_item_id,
            requested_parameters: requested_parameters,
        })
    }
}
