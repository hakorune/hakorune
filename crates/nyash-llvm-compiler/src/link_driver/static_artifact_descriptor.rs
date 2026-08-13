//! Artifact-bound descriptor parser for the selected Dynamic V2 static lane.
//!
//! The neutral header owns the byte layout.  This module is its checked Rust
//! projection and observes actual object/executable bytes; it never selects a
//! provider or reconstructs MIR meaning.

use std::fs;
use std::path::Path;

use object::{Object, ObjectSection, ObjectSymbol};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub(super) const DESCRIPTOR_SYMBOL: &str = "hako_dynamic_v2_static_artifact_descriptor_v1";
pub(super) const DESCRIPTOR_SECTION: &str = ".hako_dynamic_v2_descriptor";
pub(super) const LEASE_SYMBOL: &str = "nyrt_dynamic_v2_lease_consume_end_authorized_v1";
const MAGIC: &[u8; 8] = b"HAKODV2\0";
const SCHEMA: u32 = 1;
const SIZE: usize = 192;
const CONTRACT_CAPACITY: usize = 32;
const ENTRY_SYMBOL_CAPACITY: usize = 40;
const ENTRY_SIZE: usize = 52;
const ENTRY_COUNT: usize = 2;
const CONTRACT_OFFSET: usize = 56;
const ENTRIES_OFFSET: usize = 88;
const CONTRACT_ID: &str = "hako.text.scan@1";
const PROFILE: u32 = 1;
const CALL_ABI_REVISION: u32 = 1;
const WIRE_REVISION: u32 = 2;
const ENTRY_ROLES: [&str; ENTRY_COUNT] = ["substring", "index_of"];
const ENTRY_IDS: [u32; ENTRY_COUNT] = [1, 2];
const ENTRY_SYMBOLS: [&str; ENTRY_COUNT] =
    ["hako.text.scan.substring.v1", "hako.text.scan.index_of.v1"];
const ENTRY_ARITIES: [usize; ENTRY_COUNT] = [2, 1];

#[derive(Debug, PartialEq, Eq)]
pub(super) struct StaticAotArtifactEntryV1 {
    pub(super) site_id: u32,
    pub(super) entry_id: u32,
    pub(super) logical_arity: u32,
    pub(super) symbol: Box<str>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct StaticAotArtifactDescriptorV1 {
    pub(super) profile: u32,
    pub(super) abi_revision: u32,
    pub(super) wire_revision: u32,
    pub(super) compiler_domain: u64,
    pub(super) invocation_ordinal: u64,
    pub(super) registry_generation: u64,
    pub(super) contract_id: Box<str>,
    pub(super) entries: [StaticAotArtifactEntryV1; ENTRY_COUNT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StaticArtifactRejectV1 {
    ArtifactMissing,
    InvalidJson,
    MissingDescriptor,
    DuplicateDescriptor,
    ForeignDescriptor,
    InvalidDescriptor,
    DescriptorMismatch,
    MissingSymbol,
    DuplicateSymbol,
    UnexpectedSymbolState,
    LinkFailed,
    PublishFailed,
}

impl std::fmt::Display for StaticArtifactRejectV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[dynamic-v2/static-artifact] {:?}", self)
    }
}

impl std::error::Error for StaticArtifactRejectV1 {}

pub(super) fn expected_descriptor_from_json(
    path: &Path,
) -> Result<Option<StaticAotArtifactDescriptorV1>, StaticArtifactRejectV1> {
    let bytes = fs::read(path).map_err(|_| StaticArtifactRejectV1::ArtifactMissing)?;
    let root: Value =
        serde_json::from_slice(&bytes).map_err(|_| StaticArtifactRejectV1::InvalidJson)?;
    let functions = root
        .get("functions")
        .and_then(Value::as_array)
        .filter(|functions| !functions.is_empty())
        .ok_or(StaticArtifactRejectV1::InvalidJson)?;
    let entry_index = functions
        .iter()
        .position(|function| function.get("name").and_then(Value::as_str) == Some("main"))
        .or_else(|| {
            functions.iter().position(|function| {
                function.get("name").and_then(Value::as_str) == Some("ny_main")
            })
        })
        .unwrap_or(0);
    let selected = functions
        .iter()
        .enumerate()
        .filter_map(|(index, function)| {
            function
                .get("metadata")
                .and_then(|metadata| metadata.get("dynamic_v2_aot_call_admission_v2"))
                .map(|metadata| (index, metadata))
        });
    let rows = selected.collect::<Vec<_>>();
    let Some((index, metadata)) = rows.first().copied() else {
        return Ok(None);
    };
    if rows.len() != 1 {
        return Err(StaticArtifactRejectV1::DuplicateDescriptor);
    }
    if index != entry_index {
        return Err(StaticArtifactRejectV1::ForeignDescriptor);
    }
    Ok(Some(descriptor_from_json(metadata)?))
}

fn descriptor_from_json(
    metadata: &Value,
) -> Result<StaticAotArtifactDescriptorV1, StaticArtifactRejectV1> {
    if u32_field(metadata, "schema_version")? != 2
        || text_field(metadata, "contract_id")? != CONTRACT_ID
        || u32_field(metadata, "profile")? != PROFILE
        || u32_field(metadata, "abi_revision")? != CALL_ABI_REVISION
        || u32_field(metadata, "wire_revision")? != WIRE_REVISION
    {
        return Err(StaticArtifactRejectV1::InvalidDescriptor);
    }
    let stamp = metadata
        .get("plan_stamp")
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)?;
    let calls = metadata
        .get("calls")
        .and_then(Value::as_array)
        .filter(|calls| calls.len() == ENTRY_COUNT)
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)?;
    let mut entries: [Option<StaticAotArtifactEntryV1>; ENTRY_COUNT] = [None, None];
    for call in calls {
        let site = usize::try_from(u64_field(call, "site_id")?)
            .map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)?;
        if site >= ENTRY_COUNT || entries[site].is_some() {
            return Err(StaticArtifactRejectV1::InvalidDescriptor);
        }
        let arguments = call
            .get("argument_lanes")
            .and_then(Value::as_array)
            .ok_or(StaticArtifactRejectV1::InvalidDescriptor)?;
        if text_field(call, "role")? != ENTRY_ROLES[site]
            || u32_field(call, "entry_id")? != ENTRY_IDS[site]
            || text_field(call, "symbol")? != ENTRY_SYMBOLS[site]
            || arguments.len() != ENTRY_ARITIES[site]
        {
            return Err(StaticArtifactRejectV1::InvalidDescriptor);
        }
        entries[site] = Some(StaticAotArtifactEntryV1 {
            site_id: u32::try_from(site).map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)?,
            entry_id: u32_field(call, "entry_id")?,
            logical_arity: u32::try_from(arguments.len())
                .map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)?,
            symbol: text_field(call, "symbol")?.into(),
        });
        if u32_field(call, "abi_revision")? != u32_field(metadata, "abi_revision")?
            || u32_field(call, "wire_revision")? != u32_field(metadata, "wire_revision")?
        {
            return Err(StaticArtifactRejectV1::InvalidDescriptor);
        }
    }
    let [Some(first), Some(second)] = entries else {
        return Err(StaticArtifactRejectV1::InvalidDescriptor);
    };
    Ok(StaticAotArtifactDescriptorV1 {
        profile: u32_field(metadata, "profile")?,
        abi_revision: u32_field(metadata, "abi_revision")?,
        wire_revision: u32_field(metadata, "wire_revision")?,
        compiler_domain: nonzero_u64_field(stamp, "compiler_domain")?,
        invocation_ordinal: nonzero_u64_field(stamp, "invocation_ordinal")?,
        registry_generation: nonzero_u64_field(metadata, "registry_generation")?,
        contract_id: text_field(metadata, "contract_id")?.into(),
        entries: [first, second],
    })
}

pub(super) fn observe_descriptor(
    path: &Path,
) -> Result<(StaticAotArtifactDescriptorV1, [u8; 32]), StaticArtifactRejectV1> {
    let bytes = fs::read(path).map_err(|_| StaticArtifactRejectV1::ArtifactMissing)?;
    let file = object::File::parse(bytes.as_slice())
        .map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)?;
    let mut sections = file
        .sections()
        .filter(|section| section.name().ok() == Some(DESCRIPTOR_SECTION));
    let section = sections
        .next()
        .ok_or(StaticArtifactRejectV1::MissingDescriptor)?;
    if sections.next().is_some() {
        return Err(StaticArtifactRejectV1::DuplicateDescriptor);
    }
    let descriptor_bytes = section
        .data()
        .map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)?;
    if descriptor_bytes.len() != SIZE {
        return Err(StaticArtifactRejectV1::InvalidDescriptor);
    }
    let mut descriptor_symbols = file
        .symbols()
        .filter(|symbol| symbol.name().ok() == Some(DESCRIPTOR_SYMBOL));
    let descriptor_symbol = descriptor_symbols
        .next()
        .ok_or(StaticArtifactRejectV1::MissingSymbol)?;
    if descriptor_symbols.next().is_some() {
        return Err(StaticArtifactRejectV1::DuplicateSymbol);
    }
    if descriptor_symbol.is_undefined()
        || descriptor_symbol.section_index() != Some(section.index())
    {
        return Err(StaticArtifactRejectV1::UnexpectedSymbolState);
    }
    let descriptor = parse_descriptor_bytes(descriptor_bytes)?;
    let digest: [u8; 32] = Sha256::digest(descriptor_bytes).into();
    Ok((descriptor, digest))
}

pub(super) fn require_object_symbol_count(
    file: &object::File<'_>,
    symbol: &str,
    undefined: bool,
    expected: usize,
) -> Result<(), StaticArtifactRejectV1> {
    let count = file
        .symbols()
        .filter(|candidate| candidate.name().ok() == Some(symbol))
        .filter(|candidate| candidate.is_undefined() == undefined)
        .count();
    match count {
        value if value == expected => Ok(()),
        0 => Err(StaticArtifactRejectV1::MissingSymbol),
        _ => Err(StaticArtifactRejectV1::DuplicateSymbol),
    }
}

pub(super) fn require_object_call_symbols(
    path: &Path,
    descriptor: &StaticAotArtifactDescriptorV1,
    undefined: bool,
) -> Result<(), StaticArtifactRejectV1> {
    let bytes = fs::read(path).map_err(|_| StaticArtifactRejectV1::ArtifactMissing)?;
    let file = object::File::parse(bytes.as_slice())
        .map_err(|_| StaticArtifactRejectV1::UnexpectedSymbolState)?;
    for entry in &descriptor.entries {
        require_object_symbol_count(&file, &entry.symbol, undefined, 1)?;
    }
    require_object_symbol_count(&file, LEASE_SYMBOL, undefined, 1)
}

pub(super) fn require_archive_call_symbols(
    path: &Path,
    descriptor: &StaticAotArtifactDescriptorV1,
) -> Result<(), StaticArtifactRejectV1> {
    let bytes = fs::read(path).map_err(|_| StaticArtifactRejectV1::ArtifactMissing)?;
    let archive = object::read::archive::ArchiveFile::parse(bytes.as_slice())
        .map_err(|_| StaticArtifactRejectV1::UnexpectedSymbolState)?;
    let required = descriptor
        .entries
        .iter()
        .map(|entry| entry.symbol.as_ref())
        .chain(std::iter::once(LEASE_SYMBOL))
        .collect::<Vec<_>>();
    let mut counts = vec![0_usize; required.len()];
    for member in archive.members() {
        let member = member.map_err(|_| StaticArtifactRejectV1::UnexpectedSymbolState)?;
        let member_bytes = member
            .data(bytes.as_slice())
            .map_err(|_| StaticArtifactRejectV1::UnexpectedSymbolState)?;
        let file = object::File::parse(member_bytes)
            .map_err(|_| StaticArtifactRejectV1::UnexpectedSymbolState)?;
        for symbol in file
            .symbols()
            .filter(|symbol| symbol.is_global() && !symbol.is_undefined())
        {
            let name = symbol
                .name()
                .map_err(|_| StaticArtifactRejectV1::UnexpectedSymbolState)?;
            for (index, required_name) in required.iter().enumerate() {
                if name == *required_name {
                    counts[index] += 1;
                }
            }
        }
    }
    for count in counts {
        match count {
            1 => {}
            0 => return Err(StaticArtifactRejectV1::MissingSymbol),
            _ => return Err(StaticArtifactRejectV1::DuplicateSymbol),
        }
    }
    Ok(())
}

pub(super) fn sha256_file(path: &Path) -> Result<[u8; 32], StaticArtifactRejectV1> {
    let bytes = fs::read(path).map_err(|_| StaticArtifactRejectV1::ArtifactMissing)?;
    Ok(Sha256::digest(bytes).into())
}

fn parse_descriptor_bytes(
    bytes: &[u8],
) -> Result<StaticAotArtifactDescriptorV1, StaticArtifactRejectV1> {
    if bytes.len() != SIZE
        || &bytes[..MAGIC.len()] != MAGIC
        || read_u32(bytes, 8)? != SCHEMA
        || read_u32(bytes, 12)? as usize != SIZE
        || read_u32(bytes, 28)? as usize != ENTRY_COUNT
    {
        return Err(StaticArtifactRejectV1::InvalidDescriptor);
    }
    let entries = [parse_entry(bytes, 0)?, parse_entry(bytes, 1)?];
    Ok(StaticAotArtifactDescriptorV1 {
        profile: read_u32(bytes, 16)?,
        abi_revision: read_u32(bytes, 20)?,
        wire_revision: read_u32(bytes, 24)?,
        compiler_domain: nonzero(read_u64(bytes, 32)?)?,
        invocation_ordinal: nonzero(read_u64(bytes, 40)?)?,
        registry_generation: nonzero(read_u64(bytes, 48)?)?,
        contract_id: read_fixed_text(bytes, CONTRACT_OFFSET, CONTRACT_CAPACITY)?.into(),
        entries,
    })
}

fn parse_entry(
    bytes: &[u8],
    index: usize,
) -> Result<StaticAotArtifactEntryV1, StaticArtifactRejectV1> {
    let offset = ENTRIES_OFFSET + index * ENTRY_SIZE;
    let site_id = read_u32(bytes, offset)?;
    if site_id as usize != index {
        return Err(StaticArtifactRejectV1::InvalidDescriptor);
    }
    Ok(StaticAotArtifactEntryV1 {
        site_id,
        entry_id: read_u32(bytes, offset + 4)?,
        logical_arity: read_u32(bytes, offset + 8)?,
        symbol: read_fixed_text(bytes, offset + 12, ENTRY_SYMBOL_CAPACITY)?.into(),
    })
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, StaticArtifactRejectV1> {
    let raw: [u8; 4] = bytes
        .get(offset..offset + 4)
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)?
        .try_into()
        .map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)?;
    Ok(u32::from_le_bytes(raw))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, StaticArtifactRejectV1> {
    let raw: [u8; 8] = bytes
        .get(offset..offset + 8)
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)?
        .try_into()
        .map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)?;
    Ok(u64::from_le_bytes(raw))
}

fn read_fixed_text(
    bytes: &[u8],
    offset: usize,
    capacity: usize,
) -> Result<&str, StaticArtifactRejectV1> {
    let field = bytes
        .get(offset..offset + capacity)
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)?;
    let end = field
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)?;
    if end == 0 || field[end + 1..].iter().any(|byte| *byte != 0) {
        return Err(StaticArtifactRejectV1::InvalidDescriptor);
    }
    std::str::from_utf8(&field[..end]).map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)
}

fn text_field<'a>(value: &'a Value, key: &str) -> Result<&'a str, StaticArtifactRejectV1> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)
}

fn u32_field(value: &Value, key: &str) -> Result<u32, StaticArtifactRejectV1> {
    u32::try_from(u64_field(value, key)?).map_err(|_| StaticArtifactRejectV1::InvalidDescriptor)
}

fn nonzero_u64_field(value: &Value, key: &str) -> Result<u64, StaticArtifactRejectV1> {
    nonzero(u64_field(value, key)?)
}

fn nonzero(value: u64) -> Result<u64, StaticArtifactRejectV1> {
    (value != 0)
        .then_some(value)
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)
}

fn u64_field(value: &Value, key: &str) -> Result<u64, StaticArtifactRejectV1> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or(StaticArtifactRejectV1::InvalidDescriptor)
}

#[cfg(test)]
mod static_artifact_layout_tests {
    use super::*;

    const HEADER: &str =
        include_str!("../../../../include/hako_dynamic_v2_artifact_descriptor_v1.h");

    #[test]
    fn rust_projection_matches_the_neutral_header_layout() {
        for token in [
            "HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SCHEMA UINT32_C(1)",
            "HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE UINT32_C(192)",
            "HAKO_DYNAMIC_V2_ARTIFACT_CONTRACT_CAPACITY UINT32_C(32)",
            "HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_SYMBOL_CAPACITY UINT32_C(40)",
            "HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_SIZE UINT32_C(52)",
            "HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_COUNT UINT32_C(2)",
            "HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_CONTRACT_ID UINT32_C(56)",
            "HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_ENTRIES UINT32_C(88)",
            "HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_OFFSET_SITE_ID UINT32_C(0)",
            "HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_OFFSET_SYMBOL UINT32_C(12)",
        ] {
            assert!(HEADER.contains(token), "neutral header drifted: {token}");
        }
        assert_eq!(SCHEMA, 1);
        assert_eq!(SIZE, 192);
        assert_eq!(CONTRACT_CAPACITY, 32);
        assert_eq!(ENTRY_SYMBOL_CAPACITY, 40);
        assert_eq!(ENTRY_SIZE, 52);
        assert_eq!(ENTRY_COUNT, 2);
        assert_eq!(CONTRACT_OFFSET, 56);
        assert_eq!(ENTRIES_OFFSET, 88);
        assert!(HEADER.contains(DESCRIPTOR_SYMBOL));
        assert!(HEADER.contains(DESCRIPTOR_SECTION));
    }
}
