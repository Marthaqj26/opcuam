// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryData {
    pub data_values: Option<Vec<DataValue>>,
}

impl MessageInfo for HistoryData {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryData_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryData> for HistoryData {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.data_values);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.data_values)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let data_values: Option<Vec<DataValue>> = read_array(stream)?;
        Ok(HistoryData {
            data_values: data_values,
        })
    }
}
