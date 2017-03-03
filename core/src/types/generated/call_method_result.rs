// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CallMethodResult {
    pub status_code: StatusCode,
    pub input_argument_results: Option<Vec<StatusCode>>,
    pub input_argument_diagnostic_infos: Option<Vec<DiagnosticInfo>>,
    pub output_arguments: Option<Vec<Variant>>,
}

impl MessageInfo for CallMethodResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::CallMethodResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CallMethodResult> for CallMethodResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.input_argument_results);
        size += byte_len_array(&self.input_argument_diagnostic_infos);
        size += byte_len_array(&self.output_arguments);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.input_argument_results)?;
        size += write_array(stream, &self.input_argument_diagnostic_infos)?;
        size += write_array(stream, &self.output_arguments)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let input_argument_results: Option<Vec<StatusCode>> = read_array(stream)?;
        let input_argument_diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        let output_arguments: Option<Vec<Variant>> = read_array(stream)?;
        Ok(CallMethodResult {
            status_code: status_code,
            input_argument_results: input_argument_results,
            input_argument_diagnostic_infos: input_argument_diagnostic_infos,
            output_arguments: output_arguments,
        })
    }
}
