// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ElementOperand {
    pub index: UInt32,
}

impl BinaryEncoder<ElementOperand> for ElementOperand {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.index.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.index.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let index = UInt32::decode(stream)?;
        Ok(ElementOperand {
            index: index,
        })
    }
}
