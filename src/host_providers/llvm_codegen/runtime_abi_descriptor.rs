//! Target-compiled runtime ABI descriptor reader.
//!
//! This module owns only extraction and structural validation.  Target/session
//! equality is deliberately deferred to the lifecycle invocation owner.

use std::path::{Path, PathBuf};
use std::process::Command;

const MAGIC: &[u8; 8] = b"NYRTABI1";
const RECORD_SIZE: usize = 200;
const TARGET_CAPACITY: usize = 128;
const SECTION_NAME: &str = ".nyash.runtime_abi.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeAbiDescriptorV1 {
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
}

/// One selected runtime archive plus its target-compiled ABI facts.  This is
/// invocation-owned physical input, not a semantic or backend receipt.
#[derive(Clone, Debug)]
pub(crate) struct LifecycleRuntimeSessionV1 {
    runtime_archive: PathBuf,
    descriptor: RuntimeAbiDescriptorV1,
}

impl LifecycleRuntimeSessionV1 {
    pub(crate) fn select(runtime_archive: PathBuf) -> Result<Self, String> {
        let descriptor = read_runtime_abi_descriptor(&runtime_archive)?;
        require_lifecycle_entry_abi(&runtime_archive)?;
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
        })
    }

    pub(crate) fn runtime_archive(&self) -> &Path {
        &self.runtime_archive
    }
    pub(crate) fn descriptor(&self) -> &RuntimeAbiDescriptorV1 {
        &self.descriptor
    }
}

/// Read the one retained descriptor from a selected runtime archive.  `ar` is
/// used only to unpack named members; the descriptor is selected by ELF section
/// name, never by a byte-pattern scan.
pub(crate) fn read_runtime_abi_descriptor(
    archive: &Path,
) -> Result<RuntimeAbiDescriptorV1, String> {
    let members = archive_members(archive)?;
    let mut found = Vec::new();
    for member in members {
        let bytes = archive_member_bytes(archive, &member)?;
        if let Some(section) = elf_section(&bytes, SECTION_NAME)? {
            found.push(decode_descriptor(section)?);
        }
    }
    match found.len() {
        1 => Ok(found.remove(0)),
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
    for member in archive_members(archive)? {
        let bytes = archive_member_bytes(archive, &member)?;
        if let Some(record) = elf_section(&bytes, SECTION)? {
            if record != EXPECTED {
                return Err("unsupported lifecycle entry ABI record".to_owned());
            }
            count += 1;
        }
    }
    if count != 1 {
        return Err(format!(
            "lifecycle archive requires exactly one entry ABI record; found {count}"
        ));
    }
    Ok(())
}

fn archive_members(archive: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("ar")
        .arg("t")
        .arg(archive)
        .output()
        .map_err(|error| format!("cannot list runtime archive {}: {error}", archive.display()))?;
    if !output.status.success() {
        return Err(format!(
            "cannot list runtime archive {}: {}",
            archive.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let names = String::from_utf8(output.stdout).map_err(|_| {
        format!(
            "runtime archive {} has non-UTF8 member name",
            archive.display()
        )
    })?;
    Ok(names
        .lines()
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect())
}

fn archive_member_bytes(archive: &Path, member: &str) -> Result<Vec<u8>, String> {
    let output = Command::new("ar")
        .arg("p")
        .arg(archive)
        .arg(member)
        .output()
        .map_err(|error| {
            format!(
                "cannot extract {member} from {}: {error}",
                archive.display()
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "cannot extract {member} from {}: {}",
            archive.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output.stdout)
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

fn decode_descriptor(bytes: &[u8]) -> Result<RuntimeAbiDescriptorV1, String> {
    if bytes.len() != RECORD_SIZE {
        return Err("runtime ABI descriptor has unexpected length".to_owned());
    }
    if &bytes[..8] != MAGIC {
        return Err("runtime ABI descriptor has invalid magic".to_owned());
    }
    if u32_at(bytes, 8)? != RECORD_SIZE as u32 || u32_at(bytes, 12)? != 1 {
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
    let descriptor = RuntimeAbiDescriptorV1 {
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
