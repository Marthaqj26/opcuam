// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub message: UAString,
    pub user_name: UAString,
    pub annotation_time: DateTime,
}

impl MessageInfo for Annotation {
    fn object_id(&self) -> ObjectId {
        ObjectId::Annotation_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<Annotation> for Annotation {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.message.byte_len();
        size += self.user_name.byte_len();
        size += self.annotation_time.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.message.encode(stream)?;
        size += self.user_name.encode(stream)?;
        size += self.annotation_time.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let message = UAString::decode(stream)?;
        let user_name = UAString::decode(stream)?;
        let annotation_time = DateTime::decode(stream)?;
        Ok(Annotation {
            message: message,
            user_name: user_name,
            annotation_time: annotation_time,
        })
    }
}
