//! Type ABI adapter for runtime `MethodEntry` truth.
//!
//! The adapter reads the existing TypeBox method slot entry. It does not create
//! or own a separate method descriptor.

use crate::runtime::type_box_abi::MethodEntry;

use super::{TypeAbiError, TypeAbiPayloadSink, TypeAbiTag, TypeAbiView};

/// Payload schema for a TypeBox `MethodEntry` view.
///
/// Layout:
///
/// ```text
/// u8  arity
/// u16 slot_le
/// u16 name_len_le
/// u8[name_len] name_utf8
/// ```
pub const TYPE_ABI_METHOD_ENTRY_SCHEMA_V0: u16 = 1;

impl TypeAbiView for MethodEntry {
    fn abi_tag(&self) -> TypeAbiTag {
        TypeAbiTag::Method
    }

    fn abi_id(&self) -> u32 {
        // v0 MethodEntry is type-local truth, so the slot is the only stable
        // id available at this adapter layer. Type-qualified ids belong to the
        // later TypeBox/pack layer, not this view.
        u32::from(self.slot)
    }

    fn abi_name(&self) -> Option<&str> {
        Some(self.name)
    }

    fn payload_schema(&self) -> u16 {
        TYPE_ABI_METHOD_ENTRY_SCHEMA_V0
    }

    fn encode_payload(&self, out: &mut TypeAbiPayloadSink) -> Result<(), TypeAbiError> {
        let name_bytes = self.name.as_bytes();
        let name_len = u16::try_from(name_bytes.len())
            .map_err(|_| TypeAbiError::EncodeFailed("method name too long"))?;

        out.write_u8(self.arity);
        out.write_u16_le(self.slot);
        out.write_u16_le(name_len);
        out.write_bytes(name_bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::type_box_abi::MethodEntry;

    use super::*;
    use crate::type_abi::TypeAbiEntryHeader;

    #[test]
    fn method_entry_view_reads_existing_slot_truth() {
        let entry = MethodEntry {
            name: "contains",
            arity: 1,
            slot: 309,
        };

        assert_eq!(entry.abi_tag(), TypeAbiTag::Method);
        assert_eq!(entry.abi_id(), 309);
        assert_eq!(entry.abi_name(), Some("contains"));
        assert_eq!(entry.payload_schema(), TYPE_ABI_METHOD_ENTRY_SCHEMA_V0);
    }

    #[test]
    fn method_entry_payload_encodes_arity_slot_and_name() {
        let entry = MethodEntry {
            name: "contains",
            arity: 1,
            slot: 309,
        };
        let mut sink = TypeAbiPayloadSink::new();

        entry.encode_payload(&mut sink).unwrap();

        assert_eq!(
            sink.into_vec(),
            vec![1, 0x35, 0x01, 0x08, 0x00, b'c', b'o', b'n', b't', b'a', b'i', b'n', b's']
        );
    }

    #[test]
    fn method_entry_can_be_read_as_in_memory_header_without_pack() {
        let entry = MethodEntry {
            name: "len",
            arity: 0,
            slot: 200,
        };

        let header = TypeAbiEntryHeader::from_view(&entry);

        assert_eq!(header.tag, TypeAbiTag::Method);
        assert_eq!(header.id, 200);
        assert_eq!(header.name.as_deref(), Some("len"));
        assert_eq!(header.payload_schema, TYPE_ABI_METHOD_ENTRY_SCHEMA_V0);
    }
}
