// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ModificationInfo {
    pub modification_time: DateTime,
    pub update_type: HistoryUpdateType,
    pub user_name: UAString,
}

impl MessageInfo for ModificationInfo {
    fn object_id(&self) -> ObjectId {
        ObjectId::ModificationInfo_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ModificationInfo> for ModificationInfo {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.modification_time.byte_len();
        size += self.update_type.byte_len();
        size += self.user_name.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.modification_time.encode(stream)?;
        size += self.update_type.encode(stream)?;
        size += self.user_name.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let modification_time = DateTime::decode(stream)?;
        let update_type = HistoryUpdateType::decode(stream)?;
        let user_name = UAString::decode(stream)?;
        Ok(ModificationInfo {
            modification_time: modification_time,
            update_type: update_type,
            user_name: user_name,
        })
    }
}
