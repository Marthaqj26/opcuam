// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct InstanceNode {
    pub node_id: NodeId,
    pub node_class: NodeClass,
    pub browse_name: QualifiedName,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: UInt32,
    pub user_write_mask: UInt32,
    pub references: Option<Vec<ReferenceNode>>,
}

impl BinaryEncoder<InstanceNode> for InstanceNode {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.node_class.byte_len();
        size += self.browse_name.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += byte_len_array(&self.references);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.node_class.encode(stream)?;
        size += self.browse_name.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += write_array(stream, &self.references)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let node_class = NodeClass::decode(stream)?;
        let browse_name = QualifiedName::decode(stream)?;
        let display_name = LocalizedText::decode(stream)?;
        let description = LocalizedText::decode(stream)?;
        let write_mask = UInt32::decode(stream)?;
        let user_write_mask = UInt32::decode(stream)?;
        let references: Option<Vec<ReferenceNode>> = read_array(stream)?;
        Ok(InstanceNode {
            node_id: node_id,
            node_class: node_class,
            browse_name: browse_name,
            display_name: display_name,
            description: description,
            write_mask: write_mask,
            user_write_mask: user_write_mask,
            references: references,
        })
    }
}
