//! Internal Type ABI pack builder.
//!
//! A pack is a generated cold snapshot over `TypeAbiView` entries. It is
//! discardable and must not become planner/lowering truth.

use super::{TypeAbiEntryHeader, TypeAbiError, TypeAbiPayloadSink, TypeAbiView};

pub const TYPE_ABI_PACK_SCHEMA_V0: u16 = 1;
const TYPE_ABI_PACK_MAGIC: &[u8; 4] = b"TYAB";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAbiPack {
    schema_version: u16,
    entry_count: u32,
    bytes: Vec<u8>,
}

impl TypeAbiPack {
    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn entry_count(&self) -> u32 {
        self.entry_count
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

pub fn build_type_abi_pack(views: &[&dyn TypeAbiView]) -> Result<TypeAbiPack, TypeAbiError> {
    let entry_count = u32::try_from(views.len())
        .map_err(|_| TypeAbiError::EncodeFailed("too many type ABI entries"))?;
    let mut bytes = Vec::new();

    bytes.extend_from_slice(TYPE_ABI_PACK_MAGIC);
    bytes.extend_from_slice(&TYPE_ABI_PACK_SCHEMA_V0.to_le_bytes());
    bytes.extend_from_slice(&entry_count.to_le_bytes());

    for view in views {
        encode_pack_entry(*view, &mut bytes)?;
    }

    Ok(TypeAbiPack {
        schema_version: TYPE_ABI_PACK_SCHEMA_V0,
        entry_count,
        bytes,
    })
}

fn encode_pack_entry(view: &dyn TypeAbiView, bytes: &mut Vec<u8>) -> Result<(), TypeAbiError> {
    let header = TypeAbiEntryHeader::from_view(view);
    let name = header.name.unwrap_or_default();
    let name_bytes = name.as_bytes();
    let name_len = u16::try_from(name_bytes.len())
        .map_err(|_| TypeAbiError::EncodeFailed("type ABI entry name too long"))?;

    let mut payload = TypeAbiPayloadSink::new();
    view.encode_payload(&mut payload)?;
    let payload = payload.into_vec();
    let payload_len = u32::try_from(payload.len())
        .map_err(|_| TypeAbiError::EncodeFailed("type ABI payload too large"))?;

    bytes.extend_from_slice(&(header.tag as u16).to_le_bytes());
    bytes.extend_from_slice(&header.id.to_le_bytes());
    bytes.extend_from_slice(&header.payload_schema.to_le_bytes());
    bytes.extend_from_slice(&name_len.to_le_bytes());
    bytes.extend_from_slice(&payload_len.to_le_bytes());
    bytes.extend_from_slice(name_bytes);
    bytes.extend_from_slice(&payload);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::runtime::type_box_abi::MethodEntry;

    use super::*;
    use crate::type_abi::{method_entry::TYPE_ABI_METHOD_ENTRY_SCHEMA_V0, TypeAbiTag, TypeAbiView};

    #[test]
    fn pack_builder_encodes_method_entry_snapshot() {
        let entry = MethodEntry {
            name: "len",
            arity: 0,
            slot: 200,
        };

        let pack = build_type_abi_pack(&[&entry as &dyn TypeAbiView]).unwrap();

        assert_eq!(pack.schema_version(), TYPE_ABI_PACK_SCHEMA_V0);
        assert_eq!(pack.entry_count(), 1);

        let mut expected = Vec::new();
        expected.extend_from_slice(b"TYAB");
        expected.extend_from_slice(&TYPE_ABI_PACK_SCHEMA_V0.to_le_bytes());
        expected.extend_from_slice(&1_u32.to_le_bytes());
        expected.extend_from_slice(&(TypeAbiTag::Method as u16).to_le_bytes());
        expected.extend_from_slice(&200_u32.to_le_bytes());
        expected.extend_from_slice(&TYPE_ABI_METHOD_ENTRY_SCHEMA_V0.to_le_bytes());
        expected.extend_from_slice(&3_u16.to_le_bytes());
        expected.extend_from_slice(&8_u32.to_le_bytes());
        expected.extend_from_slice(b"len");
        expected.extend_from_slice(&[0, 200, 0, 3, 0, b'l', b'e', b'n']);

        assert_eq!(pack.as_slice(), expected.as_slice());
    }

    #[test]
    fn empty_pack_is_a_valid_snapshot_header() {
        let pack = build_type_abi_pack(&[]).unwrap();

        assert_eq!(pack.schema_version(), TYPE_ABI_PACK_SCHEMA_V0);
        assert_eq!(pack.entry_count(), 0);
        assert_eq!(pack.as_slice(), &[b'T', b'Y', b'A', b'B', 1, 0, 0, 0, 0, 0]);
    }
}
