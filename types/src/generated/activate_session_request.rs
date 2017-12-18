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
use generated::SignatureData;
use generated::SignedSoftwareCertificate;

/// Activates a session with the server.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivateSessionRequest {
    pub request_header: RequestHeader,
    pub client_signature: SignatureData,
    pub client_software_certificates: Option<Vec<SignedSoftwareCertificate>>,
    pub locale_ids: Option<Vec<UAString>>,
    pub user_identity_token: ExtensionObject,
    pub user_token_signature: SignatureData,
}

impl MessageInfo for ActivateSessionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::ActivateSessionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ActivateSessionRequest> for ActivateSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.client_signature.byte_len();
        size += byte_len_array(&self.client_software_certificates);
        size += byte_len_array(&self.locale_ids);
        size += self.user_identity_token.byte_len();
        size += self.user_token_signature.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.client_signature.encode(stream)?;
        size += write_array(stream, &self.client_software_certificates)?;
        size += write_array(stream, &self.locale_ids)?;
        size += self.user_identity_token.encode(stream)?;
        size += self.user_token_signature.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let client_signature = SignatureData::decode(stream)?;
        let client_software_certificates: Option<Vec<SignedSoftwareCertificate>> = read_array(stream)?;
        let locale_ids: Option<Vec<UAString>> = read_array(stream)?;
        let user_identity_token = ExtensionObject::decode(stream)?;
        let user_token_signature = SignatureData::decode(stream)?;
        Ok(ActivateSessionRequest {
            request_header,
            client_signature,
            client_software_certificates,
            locale_ids,
            user_identity_token,
            user_token_signature,
        })
    }
}
