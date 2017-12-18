// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::ObjectId;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;

/// The target of the translated path.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowsePathTarget {
    pub target_id: ExpandedNodeId,
    pub remaining_path_index: UInt32,
}

impl MessageInfo for BrowsePathTarget {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowsePathTarget_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowsePathTarget> for BrowsePathTarget {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.target_id.byte_len();
        size += self.remaining_path_index.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.target_id.encode(stream)?;
        size += self.remaining_path_index.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let target_id = ExpandedNodeId::decode(stream)?;
        let remaining_path_index = UInt32::decode(stream)?;
        Ok(BrowsePathTarget {
            target_id,
            remaining_path_index,
        })
    }
}
