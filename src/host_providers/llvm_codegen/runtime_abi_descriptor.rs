//! Target-compiled runtime ABI descriptor reader.
//!
//! This module owns only extraction and structural validation.  Target/session
//! equality is deliberately deferred to the lifecycle invocation owner.

use crate::mir::PublishedLifecycleRuntimeRequirementsV1;
use std::path::{Path, PathBuf};
use std::io::{Read, Seek, SeekFrom};
use std::process::Command;

const MAGIC: &[u8; 8] = b"NYRTABI2";
const RECORD_SIZE: usize = 236;
const TARGET_CAPACITY: usize = 128;
const SECTION_NAME: &str = ".nyash.runtime_abi.v2";

/// Target-compiled field values and opaque detached-End outcome storage.
/// These physical sizes grant no source/storage capability.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ObjectFieldAbiDescriptorV1 {
    pub(crate) revision: u32,
    pub(crate) value_size: u32,
    pub(crate) value_align: u32,
    pub(crate) outcome_size: u32,
    pub(crate) outcome_align: u32,
    pub(crate) end_descriptor_size: u32,
    pub(crate) end_descriptor_align: u32,
}

impl ObjectFieldAbiDescriptorV1 {
    fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != 32 { return Err("invalid object field ABI record size".into()); }
        let words: Vec<_> = bytes.chunks_exact(4).map(|word|
            u32::from_le_bytes(word.try_into().expect("four-byte word"))).collect();
        if words[0] != 32 || words[1] != 1 || [(words[2], words[3]),
            (words[4], words[5]), (words[6], words[7])].iter().any(|&(size, align)|
                size == 0 || !align.is_power_of_two() || size % align != 0) {
            return Err("unsupported object field ABI layout".into());
        }
        Ok(Self { revision: words[1], value_size: words[2], value_align: words[3],
            outcome_size: words[4], outcome_align: words[5],
            end_descriptor_size: words[6], end_descriptor_align: words[7] })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeAbiDescriptorV2 {
    pub(crate) target_triple: String,
    pub(crate) endian: u32,
    pub(crate) pointer_width: u32,
    pub(crate) fault_abi_version: u32,
    pub(crate) status_abi_version: u32,
    pub(crate) diagnostic_size: u32,
    pub(crate) diagnostic_align: u32,
    pub(crate) diagnostic_site_offset: u32,
    pub(crate) diagnostic_details_offset: u32,
    pub(crate) diagnostic_message_offset: u32,
    pub(crate) frame_size: u32,
    pub(crate) frame_align: u32,
    pub(crate) frame_primary_offset: u32,
    pub(crate) frame_suppressed_offset: u32,
    pub(crate) map_size: u32,
    pub(crate) map_align: u32,
    pub(crate) map_revision: u32,
    pub(crate) key_size: u32,
    pub(crate) key_align: u32,
    pub(crate) key_revision: u32,
    pub(crate) outcome_size: u32,
    pub(crate) outcome_align: u32,
    pub(crate) outcome_revision: u32,
}

/// One selected runtime archive plus its target-compiled ABI facts.  This is
/// invocation-owned physical input, not a semantic or backend receipt.
#[derive(Clone, Debug)]
pub(crate) struct LifecycleRuntimeSessionV1 {
    runtime_archive: PathBuf,
    descriptor: RuntimeAbiDescriptorV2,
    field_descriptor: Option<ObjectFieldAbiDescriptorV1>,
}

impl LifecycleRuntimeSessionV1 {
    pub(crate) fn select(runtime_archive: PathBuf) -> Result<Self, String> {
        let (descriptor, field_descriptor) = read_runtime_abi_descriptors(&runtime_archive)?;
        require_lifecycle_entry_abi(&runtime_archive)?;
        require_checked_map_symbols(&runtime_archive)?;
        if descriptor.target_triple != "x86_64-unknown-linux-gnu" {
            return Err(format!(
                "lifecycle runtime archive {} targets unsupported {}",
                runtime_archive.display(),
                descriptor.target_triple
            ));
        }
        Ok(Self {
            runtime_archive,
            descriptor,
            field_descriptor,
        })
    }

    pub(super) fn require_input(
        &self,
        requirements: PublishedLifecycleRuntimeRequirementsV1,
        fault_abi_version: u32,
    ) -> Result<(), String> {
        if self.descriptor.target_triple != "x86_64-unknown-linux-gnu"
            || self.descriptor.pointer_width != 8
            || self.descriptor.endian != 1
            || self.descriptor.fault_abi_version != fault_abi_version
            || self.descriptor.status_abi_version != 1
            || [
                self.descriptor.diagnostic_size,
                self.descriptor.diagnostic_align,
                self.descriptor.diagnostic_site_offset,
                self.descriptor.diagnostic_details_offset,
                self.descriptor.diagnostic_message_offset,
                self.descriptor.frame_size,
                self.descriptor.frame_align,
                self.descriptor.frame_primary_offset,
                self.descriptor.frame_suppressed_offset,
            ] != [48, 8, 8, 16, 32, 448, 8, 16, 64]
        {
            return Err("lifecycle input/runtime session mismatch".into());
        }
        match requirements {
            PublishedLifecycleRuntimeRequirementsV1::TypedObject {
                storage_profile: 1 | 2,
            } => Ok(()),
            PublishedLifecycleRuntimeRequirementsV1::TypedObject { .. } => {
                Err("lifecycle input has unsupported object storage profile".into())
            }
            PublishedLifecycleRuntimeRequirementsV1::NativeArray => {
                require_native_array_symbols(&self.runtime_archive)
            }
        }
    }

    pub(crate) fn runtime_archive(&self) -> &Path {
        &self.runtime_archive
    }
    pub(crate) fn descriptor(&self) -> &RuntimeAbiDescriptorV2 {
        &self.descriptor
    }
    pub(crate) fn field_descriptor(&self) -> Option<ObjectFieldAbiDescriptorV1> {
        self.field_descriptor
    }
}

#[path = "runtime_map_symbols.rs"]
mod map_symbols;
use map_symbols::require_checked_map_symbols;

const NATIVE_ARRAY_SYMBOLS: [&str; 6] = [
    "nyash.array.checked_new_v1",
    "nyash.array.checked_claim_v1",
    "nyash.array.checked_append_i64_v1",
    "nyash.array.checked_append_bool_v1",
    "nyash.array.checked_append_f64_v1",
    "nyrt_handle_release_h",
];

fn require_native_array_symbols(archive: &Path) -> Result<(), String> {
    let output = Command::new("nm")
        .args([
            "--extern-only",
            "--defined-only",
            "--format=posix",
            "--no-demangle",
        ])
        .arg(archive)
        .output()
        .map_err(|error| format!("cannot inspect native Array runtime symbols: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "native Array runtime symbol inventory failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let text = std::str::from_utf8(&output.stdout)
        .map_err(|_| "native Array runtime symbol inventory is not UTF-8")?;
    require_native_array_symbol_inventory(text)
}

fn require_native_array_symbol_inventory(text: &str) -> Result<(), String> {
    let mut counts = [0usize; NATIVE_ARRAY_SYMBOLS.len()];
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.ends_with("]:") {
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() < 2 {
            return Err("malformed native Array symbol inventory".into());
        }
        let Some(index) = NATIVE_ARRAY_SYMBOLS
            .iter()
            .position(|symbol| *symbol == fields[0])
        else {
            continue;
        };
        // Undefined, local or data symbols cannot provide an exported function.
        if !matches!(fields[1], "T" | "W") || fields.len() < 3 {
            return Err(format!(
                "native Array runtime symbol is not a function definition: {}",
                fields[0]
            ));
        }
        counts[index] += 1;
    }
    for (symbol, count) in NATIVE_ARRAY_SYMBOLS.iter().zip(counts) {
        if count != 1 {
            return Err(format!(
                "native Array runtime requires one definition of {symbol}; found {count}"
            ));
        }
    }
    Ok(())
}

/// Read the one retained descriptor from every selected archive member. The
/// descriptor is selected by ELF section name, never by a byte-pattern scan.
fn read_runtime_abi_descriptors(archive: &Path)
    -> Result<(RuntimeAbiDescriptorV2, Option<ObjectFieldAbiDescriptorV1>), String> {
    let mut found = Vec::new();
    let mut fields = Vec::new();
    visit_archive_members(archive, |bytes| {
        if let Some(section) = elf_section(bytes, SECTION_NAME)? {
            found.push(decode_descriptor(section)?);
        }
        if let Some(section) = elf_section(bytes, ".nyash.object_field_abi.v1")? {
            fields.push(ObjectFieldAbiDescriptorV1::decode(section)?);
        }
        Ok(())
    })?;
    if fields.len() > 1 { return Err("duplicate object field ABI sections".into()); }
    match found.len() {
        1 => Ok((found.remove(0), fields.pop())),
        0 => Err(format!(
            "runtime archive {} has no {SECTION_NAME} section",
            archive.display()
        )),
        _ => Err(format!(
            "runtime archive {} has duplicate {SECTION_NAME} sections",
            archive.display()
        )),
    }
}

/// The entry owner emits this contract independently of Fault layout.
fn require_lifecycle_entry_abi(archive: &Path) -> Result<(), String> {
    const SECTION: &str = ".nyash.entry_abi.v1";
    const EXPECTED: &[u8; 16] = b"NYENTRY1\x01\x00\x00\x00\x01\x00\x00\x00";
    let mut count = 0;
    visit_archive_members(archive, |bytes| {
        if let Some(record) = elf_section(bytes, SECTION)? {
            if record != EXPECTED {
                return Err("unsupported lifecycle entry ABI record".to_owned());
            }
            count += 1;
        }
        Ok(())
    })?;
    if count != 1 {
        return Err(format!(
            "lifecycle archive requires exactly one entry ABI record; found {count}"
        ));
    }
    Ok(())
}

/// Sequential structural ar traversal. Reopening the entire archive with `ar p`
/// for every object made ABI selection quadratic in archive size. Keep one file
/// and one member buffer; every ELF member still participates in both censuses.
fn visit_archive_members(archive: &Path, mut visit: impl FnMut(&[u8]) -> Result<(), String>)
    -> Result<(), String> {
    let mut file = std::fs::File::open(archive)
        .map_err(|error| format!("cannot open runtime archive {}: {error}", archive.display()))?;
    let length = file.metadata().map_err(|error| format!("cannot inspect runtime archive: {error}"))?.len();
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic).map_err(|error| format!("runtime archive header is truncated: {error}"))?;
    if &magic != b"!<arch>\n" {
        return Err("runtime descriptor requires a regular ar archive (thin archives are unsupported)".into());
    }
    let decimal = |bytes: &[u8]| -> Result<u64, String> {
        std::str::from_utf8(bytes).ok().and_then(|text| {
            let text = text.trim();
            (!text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
                .then(|| text.parse::<u64>().ok()).flatten()
        }).ok_or_else(|| "runtime archive member has invalid size".into())
    };
    let mut position = 8u64;
    let mut bytes = Vec::new();
    while position < length {
        let mut header = [0u8; 60];
        file.read_exact(&mut header).map_err(|error| format!("runtime archive member header is truncated: {error}"))?;
        if &header[58..] != b"`\n" { return Err("runtime archive member header is invalid".into()); }
        let size = decimal(&header[48..58])?;
        let start = position.checked_add(60).ok_or("runtime archive member offset overflows")?;
        let end = start.checked_add(size).ok_or("runtime archive member size overflows")?;
        let next = end.checked_add(size % 2).ok_or("runtime archive member padding overflows")?;
        if next > length { return Err("runtime archive member is truncated".into()); }
        let name = std::str::from_utf8(&header[..16]).map_err(|_| "runtime archive member name is not UTF-8")?.trim_end();
        if !matches!(name, "/" | "//" | "/SYM64/") {
            let size = usize::try_from(size).map_err(|_| "runtime archive member is too large")?;
            if size > bytes.len() {
                bytes.try_reserve_exact(size - bytes.len())
                    .map_err(|error| format!("cannot allocate runtime archive member buffer: {error}"))?;
            }
            bytes.resize(size, 0);
            file.read_exact(&mut bytes).map_err(|error| format!("cannot read runtime archive member: {error}"))?;
            // BSD extended names occupy a prefix of the member payload. GNU
            // long names use the separate // table and do not alter the payload.
            let name_size = match name.strip_prefix("#1/") {
                Some(size) => usize::try_from(decimal(size.as_bytes())?).map_err(|_| "runtime archive member name is too large")?,
                None => 0,
            };
            let object = bytes.get(name_size..).ok_or("runtime archive member name is truncated")?;
            let member_name = if name_size == 0 { name } else {
                std::str::from_utf8(&bytes[..name_size])
                    .map_err(|_| "runtime archive member name is not UTF-8")?.trim_end_matches('\0')
            };
            if !matches!(member_name.trim_end_matches('/'),
                "__.SYMDEF" | "__.SYMDEF SORTED" | "__.SYMDEF_64" | "__.SYMDEF_64 SORTED") {
                visit(object)?;
            }
        }
        file.seek(SeekFrom::Start(end)).map_err(|error| format!("cannot seek runtime archive member: {error}"))?;
        if size % 2 != 0 {
            let mut padding = [0u8; 1];
            file.read_exact(&mut padding).map_err(|error| format!("runtime archive padding is truncated: {error}"))?;
            if padding[0] != b'\n' { return Err("runtime archive padding is invalid".into()); }
        }
        position = next;
    }
    Ok(())
}

fn elf_section<'a>(bytes: &'a [u8], wanted: &str) -> Result<Option<&'a [u8]>, String> {
    if bytes.len() < 64 || &bytes[..4] != b"\x7fELF" {
        return Ok(None);
    }
    if bytes[4] != 2 || bytes[5] != 1 {
        return Err("runtime descriptor requires ELF64 little-endian object member".to_owned());
    }
    let section_offset = u64_at(bytes, 40)? as usize;
    let entry_size = u16_at(bytes, 58)? as usize;
    let mut section_count = u16_at(bytes, 60)? as usize;
    let mut names_index = u16_at(bytes, 62)? as usize;
    if entry_size < 64 {
        return Err("runtime descriptor ELF section table is invalid".to_owned());
    }
    // ELF uses section zero for extended counts when a Rust object has more
    // sections than the 16-bit header fields can represent.
    let first_header = bytes
        .get(section_offset..section_offset + 64)
        .ok_or("runtime descriptor ELF section zero is truncated")?;
    if section_count == 0 {
        section_count = u64_at(first_header, 32)? as usize;
    }
    if names_index == u16::MAX as usize {
        names_index = u32_at(first_header, 40)? as usize;
    }
    if section_count == 0 || names_index >= section_count {
        return Err("runtime descriptor ELF section table is invalid".to_owned());
    }
    let section_table_end = section_offset
        .checked_add(
            entry_size
                .checked_mul(section_count)
                .ok_or("runtime descriptor ELF section table overflows")?,
        )
        .ok_or("runtime descriptor ELF section table overflows")?;
    if section_table_end > bytes.len() {
        return Err("runtime descriptor ELF section table is truncated".to_owned());
    }
    let name_header = section_header(bytes, section_offset, entry_size, names_index)?;
    let name_table = slice_at(
        bytes,
        name_header.0,
        name_header.1,
        "runtime descriptor ELF name table",
    )?;
    let mut found = None;
    for index in 0..section_count {
        let header = section_header(bytes, section_offset, entry_size, index)?;
        let name = c_string_at(name_table, header.2)?;
        if name == wanted {
            if found.is_some() {
                return Err(
                    "runtime descriptor object has duplicate descriptor sections".to_owned(),
                );
            }
            found = Some(slice_at(
                bytes,
                header.0,
                header.1,
                "runtime descriptor section",
            )?);
        }
    }
    Ok(found)
}

// Returns (content offset, content size, name offset).
fn section_header(
    bytes: &[u8],
    table_offset: usize,
    entry_size: usize,
    index: usize,
) -> Result<(usize, usize, usize), String> {
    let start = table_offset
        .checked_add(
            entry_size
                .checked_mul(index)
                .ok_or("runtime descriptor ELF section index overflows")?,
        )
        .ok_or("runtime descriptor ELF section index overflows")?;
    let header = bytes
        .get(start..start + 64)
        .ok_or("runtime descriptor ELF section header is truncated")?;
    Ok((
        u64_at(header, 24)? as usize,
        u64_at(header, 32)? as usize,
        u32_at(header, 0)? as usize,
    ))
}

fn decode_descriptor(bytes: &[u8]) -> Result<RuntimeAbiDescriptorV2, String> {
    if bytes.len() != RECORD_SIZE {
        return Err("runtime ABI descriptor has unexpected length".to_owned());
    }
    if &bytes[..8] != MAGIC {
        return Err("runtime ABI descriptor has invalid magic".to_owned());
    }
    if u32_at(bytes, 8)? != RECORD_SIZE as u32 || u32_at(bytes, 12)? != 2 {
        return Err("runtime ABI descriptor has unsupported revision".to_owned());
    }
    let target_len = u32_at(bytes, 16)? as usize;
    if target_len == 0 || target_len >= TARGET_CAPACITY {
        return Err("runtime ABI descriptor has invalid target length".to_owned());
    }
    let target = std::str::from_utf8(&bytes[72..72 + target_len])
        .map_err(|_| "runtime ABI descriptor target is not UTF-8")?
        .to_owned();
    if bytes[72 + target_len..72 + TARGET_CAPACITY]
        .iter()
        .any(|byte| *byte != 0)
    {
        return Err("runtime ABI descriptor target padding is nonzero".to_owned());
    }
    let descriptor = RuntimeAbiDescriptorV2 {
        target_triple: target,
        endian: u32_at(bytes, 20)?,
        pointer_width: u32_at(bytes, 24)?,
        fault_abi_version: u32_at(bytes, 28)?,
        status_abi_version: u32_at(bytes, 32)?,
        diagnostic_size: u32_at(bytes, 36)?,
        diagnostic_align: u32_at(bytes, 40)?,
        diagnostic_site_offset: u32_at(bytes, 44)?,
        diagnostic_details_offset: u32_at(bytes, 48)?,
        diagnostic_message_offset: u32_at(bytes, 52)?,
        frame_size: u32_at(bytes, 56)?,
        frame_align: u32_at(bytes, 60)?,
        frame_primary_offset: u32_at(bytes, 64)?,
        frame_suppressed_offset: u32_at(bytes, 68)?,
        map_size: u32_at(bytes, 200)?,
        map_align: u32_at(bytes, 204)?,
        map_revision: u32_at(bytes, 208)?,
        key_size: u32_at(bytes, 212)?,
        key_align: u32_at(bytes, 216)?,
        key_revision: u32_at(bytes, 220)?,
        outcome_size: u32_at(bytes, 224)?,
        outcome_align: u32_at(bytes, 228)?,
        outcome_revision: u32_at(bytes, 232)?,
    };
    if descriptor.endian != 1
        || !matches!(descriptor.pointer_width, 4 | 8)
        || descriptor.fault_abi_version != 1
        || descriptor.status_abi_version != 1
        || descriptor.diagnostic_size == 0
        || descriptor.frame_size == 0
        || !descriptor.diagnostic_align.is_power_of_two()
        || !descriptor.frame_align.is_power_of_two()
        || descriptor.diagnostic_site_offset >= descriptor.diagnostic_size
        || descriptor.diagnostic_details_offset >= descriptor.diagnostic_size
        || descriptor.diagnostic_message_offset >= descriptor.diagnostic_size
        || descriptor.frame_primary_offset >= descriptor.frame_size
        || descriptor.frame_suppressed_offset >= descriptor.frame_size
    {
        return Err("runtime ABI descriptor has inconsistent layout values".to_owned());
    }
    for (size, align, revision) in [
        (
            descriptor.map_size,
            descriptor.map_align,
            descriptor.map_revision,
        ),
        (
            descriptor.key_size,
            descriptor.key_align,
            descriptor.key_revision,
        ),
        (
            descriptor.outcome_size,
            descriptor.outcome_align,
            descriptor.outcome_revision,
        ),
    ] {
        if size == 0 || !align.is_power_of_two() || size % align != 0 || revision != 1 {
            return Err("runtime ABI descriptor has inconsistent opaque layout".into());
        }
    }
    Ok(descriptor)
}

fn slice_at<'a>(
    bytes: &'a [u8],
    offset: usize,
    size: usize,
    label: &str,
) -> Result<&'a [u8], String> {
    let end = offset
        .checked_add(size)
        .ok_or_else(|| format!("{label} overflows"))?;
    bytes
        .get(offset..end)
        .ok_or_else(|| format!("{label} is truncated"))
}
fn c_string_at(bytes: &[u8], offset: usize) -> Result<&str, String> {
    let rest = bytes
        .get(offset..)
        .ok_or("runtime descriptor ELF name offset is invalid")?;
    let end = rest
        .iter()
        .position(|byte| *byte == 0)
        .ok_or("runtime descriptor ELF name is unterminated")?;
    std::str::from_utf8(&rest[..end])
        .map_err(|_| "runtime descriptor ELF name is not UTF-8".to_owned())
}
fn u16_at(bytes: &[u8], offset: usize) -> Result<u16, String> {
    let value = bytes
        .get(offset..offset + 2)
        .ok_or("runtime descriptor is truncated")?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}
fn u32_at(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let value = bytes
        .get(offset..offset + 4)
        .ok_or("runtime descriptor is truncated")?;
    Ok(u32::from_le_bytes(
        value.try_into().expect("fixed four-byte slice"),
    ))
}
fn u64_at(bytes: &[u8], offset: usize) -> Result<u64, String> {
    let value = bytes
        .get(offset..offset + 8)
        .ok_or("runtime descriptor is truncated")?;
    Ok(u64::from_le_bytes(
        value.try_into().expect("fixed eight-byte slice"),
    ))
}

#[cfg(test)]
#[path = "runtime_abi_descriptor_tests.rs"]
mod tests;
