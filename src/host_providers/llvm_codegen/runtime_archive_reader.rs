//! Structural reader for the selected runtime `ar` archive.
//!
//! Two consumers share it: the member walk that collects ELF descriptor
//! sections, and the symbol census that reads the archive's own GNU symbol
//! index instead of running `nm` over every member.  Nothing is cached: every
//! call re-reads the archive.
use super::{c_string_at, elf_section, u16_at, u32_at, u64_at};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

struct MemberHeader {
    name: String,
    /// Payload start, payload end and the next header (after padding).
    start: u64,
    size: u64,
    next: u64,
}

fn decimal(bytes: &[u8]) -> Result<u64, String> {
    std::str::from_utf8(bytes).ok().and_then(|text| {
        let text = text.trim();
        (!text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
            .then(|| text.parse::<u64>().ok()).flatten()
    }).ok_or_else(|| "runtime archive member has invalid size".into())
}

fn open(archive: &Path) -> Result<(File, u64), String> {
    let mut file = File::open(archive)
        .map_err(|error| format!("cannot open runtime archive {}: {error}", archive.display()))?;
    let length = file.metadata().map_err(|error| format!("cannot inspect runtime archive: {error}"))?.len();
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic).map_err(|error| format!("runtime archive header is truncated: {error}"))?;
    if &magic != b"!<arch>\n" {
        return Err("runtime descriptor requires a regular ar archive (thin archives are unsupported)".into());
    }
    Ok((file, length))
}

fn read_header(file: &mut File, position: u64, length: u64) -> Result<MemberHeader, String> {
    file.seek(SeekFrom::Start(position)).map_err(|error| format!("cannot seek runtime archive member: {error}"))?;
    let mut header = [0u8; 60];
    file.read_exact(&mut header).map_err(|error| format!("runtime archive member header is truncated: {error}"))?;
    if &header[58..] != b"`\n" { return Err("runtime archive member header is invalid".into()); }
    let size = decimal(&header[48..58])?;
    let start = position.checked_add(60).ok_or("runtime archive member offset overflows")?;
    let end = start.checked_add(size).ok_or("runtime archive member size overflows")?;
    let next = end.checked_add(size % 2).ok_or("runtime archive member padding overflows")?;
    if next > length { return Err("runtime archive member is truncated".into()); }
    let name = std::str::from_utf8(&header[..16]).map_err(|_| "runtime archive member name is not UTF-8")?.trim_end().to_owned();
    if size % 2 != 0 {
        file.seek(SeekFrom::Start(end)).map_err(|error| format!("cannot seek runtime archive member: {error}"))?;
        let mut padding = [0u8; 1];
        file.read_exact(&mut padding).map_err(|error| format!("runtime archive padding is truncated: {error}"))?;
        if padding[0] != b'\n' { return Err("runtime archive padding is invalid".into()); }
    }
    Ok(MemberHeader { name, start, size, next })
}

fn is_special(name: &str) -> bool { matches!(name, "/" | "//" | "/SYM64/") }

/// Read one member payload into `bytes`; `None` for a BSD symbol table member.
fn read_object<'a>(file: &mut File, header: &MemberHeader, bytes: &'a mut Vec<u8>)
    -> Result<Option<&'a [u8]>, String> {
    let size = usize::try_from(header.size).map_err(|_| "runtime archive member is too large")?;
    if size > bytes.len() {
        bytes.try_reserve_exact(size - bytes.len())
            .map_err(|error| format!("cannot allocate runtime archive member buffer: {error}"))?;
    }
    bytes.resize(size, 0);
    file.seek(SeekFrom::Start(header.start)).map_err(|error| format!("cannot seek runtime archive member: {error}"))?;
    file.read_exact(bytes).map_err(|error| format!("cannot read runtime archive member: {error}"))?;
    // BSD extended names occupy a prefix of the member payload. GNU long names
    // use the separate // table and do not alter the payload.
    let name_size = match header.name.strip_prefix("#1/") {
        Some(size) => usize::try_from(decimal(size.as_bytes())?).map_err(|_| "runtime archive member name is too large")?,
        None => 0,
    };
    let object = bytes.get(name_size..).ok_or("runtime archive member name is truncated")?;
    let member_name = if name_size == 0 { header.name.as_str() } else {
        std::str::from_utf8(&bytes[..name_size])
            .map_err(|_| "runtime archive member name is not UTF-8")?.trim_end_matches('\0')
    };
    Ok((!matches!(member_name.trim_end_matches('/'),
        "__.SYMDEF" | "__.SYMDEF SORTED" | "__.SYMDEF_64" | "__.SYMDEF_64 SORTED")).then_some(object))
}

/// Sequential structural ar traversal. Reopening the entire archive with `ar p`
/// for every object made ABI selection quadratic in archive size. Keep one file
/// and one member buffer; every ELF member participates in the one census.
pub(super) fn visit_archive_members(archive: &Path, mut visit: impl FnMut(&[u8]) -> Result<(), String>)
    -> Result<(), String> {
    let (mut file, length) = open(archive)?;
    let (mut position, mut bytes) = (8u64, Vec::new());
    while position < length {
        let header = read_header(&mut file, position, length)?;
        if !is_special(&header.name) {
            if let Some(object) = read_object(&mut file, &header, &mut bytes)? { visit(object)?; }
        }
        position = header.next;
    }
    Ok(())
}

/// One `nm --format=posix` style row (`name T 0 0`) for every required symbol
/// that the archive's GNU symbol index lists, with the type letter taken from
/// the defining member's own ELF symbol table.  An archive without a GNU index
/// is rejected; it is never traversed instead.
pub(super) fn defined_symbol_rows(archive: &Path, wanted: &[&str]) -> Result<String, String> {
    let (mut file, length) = open(archive)?;
    let index = read_header(&mut file, 8, length)?;
    let word: u64 = match index.name.as_str() {
        "/" => 4,
        "/SYM64/" => 8,
        _ => return Err(format!(
            "runtime archive {} has no GNU symbol index; rebuild it with `ranlib`", archive.display())),
    };
    let wanted_index: HashMap<&[u8], usize> = wanted.iter().enumerate().map(|(i, name)| (name.as_bytes(), i)).collect();
    let mut hits: Vec<(u64, usize)> = Vec::new();
    {
        file.seek(SeekFrom::Start(index.start)).map_err(|error| format!("cannot seek runtime archive symbol index: {error}"))?;
        let mut reader = BufReader::with_capacity(1 << 20, (&mut file).take(index.size));
        let big_endian = |reader: &mut dyn Read| -> Result<u64, String> {
            let mut buffer = [0u8; 8];
            let slot = &mut buffer[8 - word as usize..];
            reader.read_exact(slot).map_err(|_| "runtime archive symbol index is truncated".to_owned())?;
            Ok(u64::from_be_bytes(buffer))
        };
        let count = big_endian(&mut reader)?;
        if count.checked_mul(word).and_then(|bytes| bytes.checked_add(word)).map_or(true, |bytes| bytes > index.size) {
            return Err("runtime archive symbol index is truncated".into());
        }
        let offsets = (0..count).map(|_| big_endian(&mut reader)).collect::<Result<Vec<_>, _>>()?;
        let mut name = Vec::new();
        for offset in offsets {
            name.clear();
            reader.read_until(0, &mut name).map_err(|error| format!("cannot read runtime archive symbol index: {error}"))?;
            if name.pop() != Some(0) { return Err("runtime archive symbol index is truncated".into()); }
            if let Some(&wanted) = wanted_index.get(name.as_slice()) { hits.push((offset, wanted)); }
        }
    }
    hits.sort_unstable();
    let (mut rows, mut bytes, mut group) = (String::new(), Vec::new(), 0);
    while group < hits.len() {
        let offset = hits[group].0;
        let header = read_header(&mut file, offset, length)?;
        let object = if is_special(&header.name) { None } else { read_object(&mut file, &header, &mut bytes)? }
            .ok_or("runtime archive symbol index names a non-object member")?;
        while group < hits.len() && hits[group].0 == offset {
            let name = wanted[hits[group].1];
            if !elf_defined_rows(object, name, &mut rows)? {
                return Err(format!("runtime archive symbol index lists {name} but its member does not define it"));
            }
            group += 1;
        }
    }
    Ok(rows)
}

/// Append one row per global or weak definition of `name` in this ELF member.
/// Function symbols are `T` (global) or `W` (weak); any other definition gets
/// `D`, so the callers' "not a function definition" rejection is unchanged.
fn elf_defined_rows(object: &[u8], name: &str, rows: &mut String) -> Result<bool, String> {
    let (Some(symbols), Some(strings)) = (elf_section(object, ".symtab")?, elf_section(object, ".strtab")?) else {
        return Err("runtime archive member has no ELF symbol table".into());
    };
    let mut found = false;
    for entry in symbols.chunks_exact(24) {
        let (info, section) = (entry[4], u16_at(entry, 6)?);
        let (binding, kind) = (info >> 4, info & 15);
        if section == 0 || !matches!(binding, 1 | 2) || c_string_at(strings, u32_at(entry, 0)? as usize)? != name {
            continue;
        }
        let letter = match (kind, binding) { (2, 1) => 'T', (2, _) => 'W', _ => 'D' };
        rows.push_str(&format!("{name} {letter} {:x} {:x}\n", u64_at(entry, 8)?, u64_at(entry, 16)?));
        found = true;
    }
    Ok(found)
}
