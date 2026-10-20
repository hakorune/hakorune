//! One symbol census for checked Map and the advertised optional field ABI.
use std::{path::Path, process::Command};
const SYMBOLS: &[&str] = &[
    "nyash.map.storage_init_v1",
    "nyash.map.checked_new_v1",
    "nyash.map.key_init_v1",
    "nyash.map.key_prepare_utf8_v1",
    "nyash.map.key_dispose_v1",
    "nyash.map.outcome_init_v1",
    "nyash.map.checked_install_indexed_end_v1",
    "nyash.map.checked_install_value_v1",
    "nyash.map.checked_install_text_v1",
    "nyash.map.checked_install_empty_array_v1",
    "nyash.map.checked_install_borrowed_array_v1",
    "nyash.map.outcome_end_v1",
    "nyash.map.outcome_dispose_v1",
    "nyash.map.checked_end_v1",
    "nyash.map.checked_get_i64_v1",
    "nyash.map.checked_array_length_v1",
    "nyash.map.checked_array_index_map_v1",
    "nyash.map.checked_get_text_v1",
    "nyash.map.storage_dispose_v1",
    "nyash.map.storage_move_v1",
];
const FIELD_SYMBOLS: &[&str] = &[
    "nyash.map.checked_has_v1",
    "nyash.map.checked_borrow_indexed_v1",
    "nyash.object.checked_dynamic_replace_v1",
    "nyash.object.checked_dynamic_initialize_v1",
    "nyash.object.checked_dynamic_get_v1",
    "nyash.object.checked_dynamic_take_v1",
    "nyash.object.field_outcome_init_v1",
    "nyash.object.field_outcome_end_v1",
    "nyash.object.field_outcome_dispose_v1",
    "nyash.object.home_release_completed_fields_v1",
    "nyash.object.checked_map_field_install_v1",
    "nyash.object.checked_map_field_borrow_v1",
    "nyash.object.checked_map_field_has_v1",
    "nyash.object.checked_map_field_get_v1",
];
pub(super) fn require_checked_map_symbols(archive: &Path, fields: bool) -> Result<(), String> {
    let output = Command::new("nm")
        .args([
            "--extern-only",
            "--defined-only",
            "--format=posix",
            "--no-demangle",
        ])
        .arg(archive)
        .output()
        .map_err(|e| format!("checked Map symbol inventory: {e}"))?;
    if !output.status.success() {
        return Err("checked Map symbol inventory failed".into());
    }
    let text =
        std::str::from_utf8(&output.stdout).map_err(|_| "checked Map symbols are not UTF-8")?;
    validate(text, fields)
}
fn validate(text: &str, fields: bool) -> Result<(), String> {
    // Read each nm row once. Keep original required-symbol order for diagnostics;
    // splitting/allocating every row once per required entry dominates opt0 builds.
    let active = SYMBOLS.len() + if fields { FIELD_SYMBOLS.len() } else { 0 };
    let symbols = || SYMBOLS.iter().chain(FIELD_SYMBOLS).take(active);
    let mut inventory = [(0usize, true); SYMBOLS.len() + FIELD_SYMBOLS.len()];
    for line in text.lines() {
        let mut fields = line.split_whitespace();
        let Some(name) = fields.next() else {
            continue;
        };
        let Some(index) = symbols().position(|symbol| *symbol == name) else {
            continue;
        };
        let (count, function) = &mut inventory[index];
        *count += 1;
        *function &= matches!(fields.next(), Some("T" | "W")) && fields.next().is_some();
    }
    for (symbol, (count, function)) in symbols().zip(inventory) {
        if !function {
            return Err(format!("checked Map symbol is not a function: {symbol}"));
        }
        if count != 1 {
            return Err(format!("checked Map symbol missing or duplicate: {symbol}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_each_opaque_entry_exactly_once() {
        for fields in [false, true] {
            let symbols: Vec<_> = SYMBOLS.iter().chain(if fields { FIELD_SYMBOLS } else { &[] }).collect();
            let all = symbols.iter().map(|name| format!("{name} T 0 1\n")).collect::<String>();
            assert!(validate(&all, fields).is_ok());
            for name in symbols {
                assert!(validate(&all.replace(&format!("{name} T"), &format!("{name} D")), fields).is_err());
                assert!(validate(&all.replace(&format!("{name} T 0 1\n"), ""), fields).is_err());
                assert!(validate(&format!("{all}{name} T 0 1\n"), fields).is_err());
            }
        }
    }
}
