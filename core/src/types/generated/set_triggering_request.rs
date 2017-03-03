// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SetTriggeringRequest {
    pub request_header: RequestHeader,
    pub subscription_id: UInt32,
    pub triggering_item_id: UInt32,
    pub links_to_add: Option<Vec<UInt32>>,
    pub links_to_remove: Option<Vec<UInt32>>,
}

impl MessageInfo for SetTriggeringRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::SetTriggeringRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SetTriggeringRequest> for SetTriggeringRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.triggering_item_id.byte_len();
        size += byte_len_array(&self.links_to_add);
        size += byte_len_array(&self.links_to_remove);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.triggering_item_id.encode(stream)?;
        size += write_array(stream, &self.links_to_add)?;
        size += write_array(stream, &self.links_to_remove)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let subscription_id = UInt32::decode(stream)?;
        let triggering_item_id = UInt32::decode(stream)?;
        let links_to_add: Option<Vec<UInt32>> = read_array(stream)?;
        let links_to_remove: Option<Vec<UInt32>> = read_array(stream)?;
        Ok(SetTriggeringRequest {
            request_header: request_header,
            subscription_id: subscription_id,
            triggering_item_id: triggering_item_id,
            links_to_add: links_to_add,
            links_to_remove: links_to_remove,
        })
    }
}
