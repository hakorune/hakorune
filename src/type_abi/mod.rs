//! Type ABI read-only view surface.
//!
//! This module intentionally starts as a descriptor/view skeleton. Existing
//! domain data remains the truth; adapters implement `TypeAbiView` when a
//! domain needs cold tooling or report output.

pub mod method_entry;
pub mod pack;
pub mod report;

/// Stable tag for a Type ABI entry payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum TypeAbiTag {
    Method = 1,
    Field = 2,
    Memory = 3,
    String = 4,
    Gui = 5,
    Index = 0xffff,
}

/// Append-only payload sink used by read-only Type ABI adapters.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TypeAbiPayloadSink {
    bytes: Vec<u8>,
}

impl TypeAbiPayloadSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn write_u16_le(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_u32_le(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeAbiError {
    EncodeFailed(&'static str),
    UnsupportedSchema(u16),
}

impl std::fmt::Display for TypeAbiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeAbiError::EncodeFailed(reason) => write!(f, "type ABI encode failed: {reason}"),
            TypeAbiError::UnsupportedSchema(schema) => {
                write!(f, "unsupported type ABI payload schema: {schema}")
            }
        }
    }
}

impl std::error::Error for TypeAbiError {}

/// Read-only adapter over existing domain truth.
pub trait TypeAbiView {
    fn abi_tag(&self) -> TypeAbiTag;
    fn abi_id(&self) -> u32;
    fn abi_name(&self) -> Option<&str>;
    fn payload_schema(&self) -> u16;
    fn encode_payload(&self, out: &mut TypeAbiPayloadSink) -> Result<(), TypeAbiError>;
}

/// Cold header derived from a `TypeAbiView`.
///
/// This is a transient query result, not a domain truth or persisted pack
/// entry. Hot paths must keep using domain plans directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAbiEntryHeader {
    pub tag: TypeAbiTag,
    pub id: u32,
    pub name: Option<String>,
    pub payload_schema: u16,
}

impl TypeAbiEntryHeader {
    pub fn from_view<V: TypeAbiView + ?Sized>(view: &V) -> Self {
        Self {
            tag: view.abi_tag(),
            id: view.abi_id(),
            name: view.abi_name().map(str::to_owned),
            payload_schema: view.payload_schema(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyMethodView;

    impl TypeAbiView for DummyMethodView {
        fn abi_tag(&self) -> TypeAbiTag {
            TypeAbiTag::Method
        }

        fn abi_id(&self) -> u32 {
            7
        }

        fn abi_name(&self) -> Option<&str> {
            Some("len")
        }

        fn payload_schema(&self) -> u16 {
            1
        }

        fn encode_payload(&self, out: &mut TypeAbiPayloadSink) -> Result<(), TypeAbiError> {
            out.write_u8(0);
            out.write_u16_le(300);
            Ok(())
        }
    }

    #[test]
    fn type_abi_view_encodes_payload_without_pack() {
        let view = DummyMethodView;
        let mut sink = TypeAbiPayloadSink::new();

        view.encode_payload(&mut sink).unwrap();

        assert_eq!(view.abi_tag(), TypeAbiTag::Method);
        assert_eq!(view.abi_id(), 7);
        assert_eq!(view.abi_name(), Some("len"));
        assert_eq!(view.payload_schema(), 1);
        assert_eq!(sink.as_slice(), &[0, 44, 1]);
    }

    #[test]
    fn payload_sink_keeps_little_endian_order() {
        let mut sink = TypeAbiPayloadSink::new();
        sink.write_u16_le(0x1234);
        sink.write_u32_le(0x12345678);

        assert_eq!(sink.into_vec(), vec![0x34, 0x12, 0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn entry_header_is_a_cold_view_query_result() {
        let view = DummyMethodView;

        let header = TypeAbiEntryHeader::from_view(&view);

        assert_eq!(
            header,
            TypeAbiEntryHeader {
                tag: TypeAbiTag::Method,
                id: 7,
                name: Some("len".to_string()),
                payload_schema: 1
            }
        );
    }
}
