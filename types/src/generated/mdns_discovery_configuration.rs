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

/// The discovery information needed for mDNS registration.
#[derive(Debug, Clone, PartialEq)]
pub struct MdnsDiscoveryConfiguration {
    pub mdns_server_name: UAString,
    pub server_capabilities: Option<Vec<UAString>>,
}

impl BinaryEncoder<MdnsDiscoveryConfiguration> for MdnsDiscoveryConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.mdns_server_name.byte_len();
        size += byte_len_array(&self.server_capabilities);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.mdns_server_name.encode(stream)?;
        size += write_array(stream, &self.server_capabilities)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let mdns_server_name = UAString::decode(stream)?;
        let server_capabilities: Option<Vec<UAString>> = read_array(stream)?;
        Ok(MdnsDiscoveryConfiguration {
            mdns_server_name,
            server_capabilities,
        })
    }
}
