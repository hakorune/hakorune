//! Shared string indexing helpers (byte vs codepoint).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringIndexMode {
    Byte,
    CodePoint,
}

pub fn index_mode_from_env() -> StringIndexMode {
    if crate::config::env::string_codepoint_mode() {
        StringIndexMode::CodePoint
    } else {
        StringIndexMode::Byte
    }
}

pub fn index_of(haystack: &str, needle: &str, start: Option<i64>, mode: StringIndexMode) -> i64 {
    match mode {
        StringIndexMode::Byte => index_of_bytes(haystack, needle, start),
        StringIndexMode::CodePoint => index_of_codepoints(haystack, needle, start),
    }
}

pub fn last_index_of(haystack: &str, needle: &str, mode: StringIndexMode) -> i64 {
    last_index_of_from(haystack, needle, None, mode)
}

pub fn last_index_of_from(
    haystack: &str,
    needle: &str,
    start: Option<i64>,
    mode: StringIndexMode,
) -> i64 {
    match mode {
        StringIndexMode::Byte => last_index_of_bytes(haystack, needle, start),
        StringIndexMode::CodePoint => last_index_of_codepoints(haystack, needle, start),
    }
}

pub fn substring(haystack: &str, start: i64, end: Option<i64>, mode: StringIndexMode) -> String {
    match mode {
        StringIndexMode::Byte => substring_bytes(haystack, start, end),
        StringIndexMode::CodePoint => substring_codepoints(haystack, start, end),
    }
}

fn index_of_bytes(haystack: &str, needle: &str, start: Option<i64>) -> i64 {
    let start_idx = start.unwrap_or(0).max(0) as usize;
    if start_idx > haystack.len() {
        return -1;
    }
    haystack[start_idx..]
        .find(needle)
        .map(|i| (start_idx + i) as i64)
        .unwrap_or(-1)
}

fn index_of_codepoints(haystack: &str, needle: &str, start: Option<i64>) -> i64 {
    let start_idx = start.unwrap_or(0).max(0) as usize;
    let Some(byte_start) = byte_offset_for_cp(haystack, start_idx) else {
        return -1;
    };
    if byte_start > haystack.len() {
        return -1;
    }
    haystack[byte_start..]
        .find(needle)
        .map(|rel| {
            let abs = byte_start + rel;
            haystack[..abs].chars().count() as i64
        })
        .unwrap_or(-1)
}

fn last_index_of_bytes(haystack: &str, needle: &str, start: Option<i64>) -> i64 {
    let hay = haystack.as_bytes();
    let needle = needle.as_bytes();
    let len = hay.len();
    let pos = start.unwrap_or(len as i64).max(0).min(len as i64) as usize;
    if needle.is_empty() {
        return pos as i64;
    }
    if len < needle.len() {
        return -1;
    }
    let max_start = pos.min(len - needle.len());
    for i in (0..=max_start).rev() {
        if hay[i..].starts_with(needle) {
            return i as i64;
        }
    }
    -1
}

fn last_index_of_codepoints(haystack: &str, needle: &str, start: Option<i64>) -> i64 {
    let len = haystack.chars().count();
    let pos = start.unwrap_or(len as i64).max(0).min(len as i64) as usize;
    if needle.is_empty() {
        return pos as i64;
    }

    let mut last = -1;
    for (cp_idx, (byte_pos, _)) in haystack.char_indices().enumerate() {
        if cp_idx > pos {
            break;
        }
        if haystack[byte_pos..].starts_with(needle) {
            last = cp_idx as i64;
        }
    }
    last
}

fn substring_bytes(haystack: &str, start: i64, end: Option<i64>) -> String {
    let len = haystack.len() as i64;
    let start = start.max(0).min(len);
    let end = end.unwrap_or(len).max(0).min(len);
    if start > end {
        return String::new();
    }
    let bytes = haystack.as_bytes();
    String::from_utf8(bytes[start as usize..end as usize].to_vec()).unwrap_or_default()
}

fn substring_codepoints(haystack: &str, start: i64, end: Option<i64>) -> String {
    let len = haystack.chars().count() as i64;
    let start = start.max(0).min(len) as usize;
    let end = end.unwrap_or(len).max(start as i64).min(len) as usize;
    let chars: Vec<char> = haystack.chars().collect();
    chars[start..end].iter().collect()
}

fn byte_offset_for_cp(haystack: &str, cp_index: usize) -> Option<usize> {
    let mut count = 0usize;
    for (byte_pos, _) in haystack.char_indices() {
        if count == cp_index {
            return Some(byte_pos);
        }
        count += 1;
    }
    if count == cp_index {
        Some(haystack.len())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{last_index_of, last_index_of_from, StringIndexMode};

    #[test]
    fn last_index_of_from_uses_start_position_in_byte_mode() {
        assert_eq!(last_index_of("banana", "na", StringIndexMode::Byte), 4);
        assert_eq!(
            last_index_of_from("banana", "na", Some(3), StringIndexMode::Byte),
            2
        );
        assert_eq!(
            last_index_of_from("banana", "na", Some(1), StringIndexMode::Byte),
            -1
        );
    }

    #[test]
    fn last_index_of_from_clamps_and_handles_empty_needle() {
        assert_eq!(
            last_index_of_from("banana", "na", Some(99), StringIndexMode::Byte),
            4
        );
        assert_eq!(
            last_index_of_from("banana", "", Some(3), StringIndexMode::Byte),
            3
        );
        assert_eq!(
            last_index_of_from("banana", "", Some(-5), StringIndexMode::Byte),
            0
        );
    }

    #[test]
    fn last_index_of_from_uses_codepoint_position_in_codepoint_mode() {
        assert_eq!(
            last_index_of_from("éxéx", "é", Some(2), StringIndexMode::CodePoint),
            2
        );
        assert_eq!(
            last_index_of_from("éxéx", "é", Some(1), StringIndexMode::CodePoint),
            0
        );
    }
}
