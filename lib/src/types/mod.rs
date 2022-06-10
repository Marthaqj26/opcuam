// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

//! The OPC UA Types module contains data types and enumerations for OPC UA.
//!
//! This includes:
//!
//! 1. All of the built-in data types described in OPC Part 6 Chapter 5 that are encodable.
//! 2. All of the standard data types described in OPC Part 3 Chapter 8 (if not covered by 1.).
//! 3. Autogenerated data types and request / responses as described in OPC Part 4.
//!
//! For the built-in data types, the module provides functions

///Contains constants recognized by OPC UA clients and servers to describe various protocols and
/// profiles used during communication and encryption.
pub mod profiles {
    pub const TRANSPORT_PROFILE_URI_BINARY: &str =
        "http://opcfoundation.org/UA-Profile/Transport/uatcp-uasc-uabinary";
    pub const SECURITY_USER_TOKEN_POLICY_ANONYMOUS: &str =
        "http://opcfoundation.org/UA-Profile/Security/UserToken/Anonymous";
    pub const SECURITY_USER_TOKEN_POLICY_USERPASS: &str =
        "http://opcfoundation.org/UA-Profile/ Security/UserToken-Server/UserNamePassword";
}

pub mod constants {
    /// Default maximum number of elements in an array
    pub const MAX_ARRAY_LENGTH: usize = 1000;
    /// Default maximum size of a string in chars
    pub const MAX_STRING_LENGTH: usize = 65535;
    /// Default maximum size of a byte string in bytes
    pub const MAX_BYTE_STRING_LENGTH: usize = 65535;
    /// Default maximum size of a certificate to send
    pub const MAX_CERTIFICATE_LENGTH: usize = 32767;
    /// Default maximum size of a message in bytes. 0 is any length, i.e. the other end can send a message of any size which is
    /// not recommended in a server configuration. Override in the client / server config.
    /// In clients, max message size is only preferred size since it can be adjusted by the server during the handshake.
    pub const MAX_MESSAGE_SIZE: usize = 65535 * MAX_CHUNK_COUNT;
    /// Default maximum number of chunks in a single message. 0 is any number but this is not recommended
    /// as the default since server memory could be exhausted. Default number can be overridden
    /// by client / server config which is where it should happen if you want a different figure. In clients
    /// chunk size is a preferred value since the server can modify it during the handshake.
    pub const MAX_CHUNK_COUNT: usize = 5;
    /// Default maximum decoding depth for recursive data structures, i.e. if data is nested deeper than this it is
    /// an error during decoding. This is a security measure to stop deeply nested junk being sent to
    /// a server / client.
    pub const MAX_DECODING_DEPTH: usize = 10;
    /// URI supplied for the None security policy
    pub const SECURITY_POLICY_NONE_URI: &str = "http://opcfoundation.org/UA/SecurityPolicy#None";
    /// String used as shorthand in config files, debug etc.for `None` security policy
    pub const SECURITY_POLICY_NONE: &str = "None";
}

// Attributes mask bits
bitflags! {
    pub struct AttributesMask: u32 {
        /// Indicates if the AccessLevel Attribute is set.
        const ACCESS_LEVEL = 1;
        /// Indicates if the ArrayDimensions Attribute is set.
        const ARRAY_DIMENSIONS = 1 << 1;
        /// Indicates if the ContainsNoLoops Attribute is set.
        const CONTAINS_NO_LOOPS = 1 << 3;
        /// Indicates if the DataType Attribute is set.
        const DATA_TYPE = 1 << 4;
        /// Indicates if the Description Attribute is set.
        const DESCRIPTION = 1 << 5;
        /// Indicates if the DisplayName Attribute is set.
        const DISPLAY_NAME = 1 << 6;
        /// Indicates if the EventNotifier Attribute is set.
        const EVENT_NOTIFIER = 1 << 7;
        /// Indicates if the Executable Attribute is set.
        const EXECUTABLE = 1 << 8;
        /// Indicates if the Historizing Attribute is set.
        const HISTORIZING = 1 << 9;
        /// Indicates if the InverseName Attribute is set.
        const INVERSE_NAME = 1 << 10;
        /// Indicates if the IsAbstract Attribute is set.
        const IS_ABSTRACT = 1 << 11;
        /// Indicates if the MinimumSamplingInterval Attribute is set.
        const MINIMUM_SAMPLING_INTERVAL = 1 << 12;
        /// Indicates if the Symmetric Attribute is set.
        const SYMMETRIC = 1 << 15;
        /// Indicates if the UserAccessLevel Attribute is set.
        const USER_ACCESS_LEVEL = 1 << 16;
        /// Indicates if the UserExecutable Attribute is set.
        const USER_EXECUTABLE = 1 << 17;
        /// Indicates if the UserWriteMask Attribute is set.
        const USER_WRITE_MASK = 1 << 18;
        /// Indicates if the ValueRank Attribute is set.
        const VALUE_RANK = 1 << 19;
        /// Indicates if the WriteMask Attribute is set.
        const WRITE_MASK = 1 << 20;
        /// Indicates if the Value Attribute is set
        const VALUE = 1 << 21;
    }
}

// Write mask bits (similar but different to AttributesMask)
//
// See Part 3, Table 43
bitflags! {
    pub struct WriteMask: u32 {
        /// Indicates if the AccessLevel Attribute is writable.
        const ACCESS_LEVEL = 1;
        /// Indicates if the ArrayDimensions Attribute is writable.
        const ARRAY_DIMENSIONS = 1 << 1;
        ///Indicates if the BrowseName Attribute is writable.
        const BROWSE_NAME = 1 << 2;
        /// Indicates if the ContainsNoLoops Attribute is writable.
        const CONTAINS_NO_LOOPS = 1 << 3;
        /// Indicates if the DataType Attribute is writable.
        const DATA_TYPE = 1 << 4;
        /// Indicates if the Description Attribute is writable.
        const DESCRIPTION = 1 << 5;
        /// Indicates if the DisplayName Attribute is writable.
        const DISPLAY_NAME = 1 << 6;
        /// Indicates if the EventNotifier Attribute is writable.
        const EVENT_NOTIFIER = 1 << 7;
        /// Indicates if the Executable Attribute is writable.
        const EXECUTABLE = 1 << 8;
        /// Indicates if the Historizing Attribute is writable.
        const HISTORIZING = 1 << 9;
        /// Indicates if the InverseName Attribute is writable.
        const INVERSE_NAME = 1 << 10;
        /// Indicates if the IsAbstract Attribute is writable.
        const IS_ABSTRACT = 1 << 11;
        /// Indicates if the MinimumSamplingInterval Attribute is writable.
        const MINIMUM_SAMPLING_INTERVAL = 1 << 12;
        /// Indicates if the NodeClass Attribute is writable.
        const NODE_CLASS = 1 << 13;
        /// Indicates if the NodeId Attribute is writable.
        const NODE_ID = 1 << 14;
        /// Indicates if the Symmetric Attribute is writable.
        const SYMMETRIC = 1 << 15;
        /// Indicates if the UserAccessLevel Attribute is writable.
        const USER_ACCESS_LEVEL = 1 << 16;
        /// Indicates if the UserExecutable Attribute is writable.
        const USER_EXECUTABLE = 1 << 17;
        /// Indicates if the UserWriteMask Attribute is writable.
        const USER_WRITE_MASK = 1 << 18;
        /// Indicates if the ValueRank Attribute is writable.
        const VALUE_RANK = 1 << 19;
        /// Indicates if the WriteMask Attribute is writable.
        const WRITE_MASK = 1 << 20;
        /// Indicates if the Value Attribute is writable for a VariableType. It does not apply for Variables
        /// since this is handled by the AccessLevel and UserAccessLevel Attributes for the Variable.
        /// For Variables this bit shall be set to 0.
        const VALUE_FOR_VARIABLE_TYPE = 1 << 21;
        /// Indicates if the DataTypeDefinition Attribute is writable.
        const DATA_TYPE_DEFINITION = 1 << 22;
        /// Indicates if the RolePermissions Attribute is writable.
        const ROLE_PERMISSIONS = 1 << 23;
        /// Indicates if the AccessRestrictions Attribute is writable
        const ACCESS_RESTRICTIONS = 1 << 24;
        /// Indicates if the AccessLevelEx Attribute is writable
        const ACCESS_LEVEL_EX = 1 << 25;

        // Bits 26-31. Reserved for future use. Shall always be zero.
    }
}

// Bits that control the reference description coming back from browse()
bitflags! {
    pub struct BrowseDescriptionResultMask: u32 {
        const RESULT_MASK_REFERENCE_TYPE = 1;
        const RESULT_MASK_IS_FORWARD = 1 << 1;
        const RESULT_MASK_NODE_CLASS = 1 << 2;
        const RESULT_MASK_BROWSE_NAME = 1 << 3;
        const RESULT_MASK_DISPLAY_NAME = 1 << 4;
        const RESULT_MASK_TYPE_DEFINITION = 1 << 5;
    }
}

// Bits for a node class mask
bitflags! {
    pub struct NodeClassMask: u32 {
        const OBJECT = 1;
        const VARIABLE = 1 << 1;
        const METHOD = 1 << 2;
        const OBJECT_TYPE = 1 << 3;
        const VARIABLE_TYPE = 1 << 4;
        const REFERENCE_TYPE = 1 << 5;
        const DATA_TYPE = 1 << 6;
        const VIEW = 1 << 7;
    }
}

#[rustfmt::skip]
mod status_codes;
#[rustfmt::skip]
pub mod node_ids;

pub mod argument;
pub mod array;
pub mod attribute;
pub mod basic_types;
pub mod byte_string;
pub mod data_types;
pub mod data_value;
pub mod date_time;
pub mod diagnostic_info;
pub mod encoding;
pub mod extension_object;
pub mod guid;
pub mod localized_text;
pub mod node_id;
pub mod notification_message;
pub mod numeric_range;
pub mod operand;
pub mod qualified_name;
pub mod relative_path;
pub mod request_header;
pub mod response_header;

#[rustfmt::skip]
pub mod service_types;
pub mod status_code;
pub mod string;
pub mod variant;

pub use crate::types::{
    argument::*, array::*, attribute::*, basic_types::*, byte_string::*, data_types::*,
    data_value::*, date_time::*, diagnostic_info::*, encoding::*, extension_object::*, guid::*,
    localized_text::*, node_id::*, node_ids::*, numeric_range::*, operand::*, qualified_name::*,
    request_header::*, response_header::*, service_types::*, status_code::*, string::*, variant::*,
};

#[cfg(test)]
mod tests;
