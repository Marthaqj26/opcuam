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

/// Activates a session with the server.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivateSessionResponse {
    pub response_header: ResponseHeader,
    pub server_nonce: ByteString,
    pub results: Option<Vec<StatusCode>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for ActivateSessionResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::ActivateSessionResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ActivateSessionResponse> for ActivateSessionResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.server_nonce.byte_len();
        size += byte_len_array(&self.results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.server_nonce.encode(stream)?;
        size += write_array(stream, &self.results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let server_nonce = ByteString::decode(stream)?;
        let results: Option<Vec<StatusCode>> = read_array(stream)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        Ok(ActivateSessionResponse {
            response_header,
            server_nonce,
            results,
            diagnostic_infos,
        })
    }
}
