// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// Cancels an outstanding request.
#[derive(Debug, Clone, PartialEq)]
pub struct CancelResponse {
    pub response_header: ResponseHeader,
    pub cancel_count: UInt32,
}

impl MessageInfo for CancelResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::CancelResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CancelResponse> for CancelResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.cancel_count.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.cancel_count.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let cancel_count = UInt32::decode(stream)?;
        Ok(CancelResponse {
            response_header: response_header,
            cancel_count: cancel_count,
        })
    }
}
