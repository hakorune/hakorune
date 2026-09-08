//! Symbols required by the runtime descriptor's checked Map contract.
use std::{path::Path, process::Command};
const SYMBOLS: &[&str] = &[
    "nyash.map.storage_init_v1",
    "nyash.map.checked_new_v1",
    "nyash.map.key_init_v1",
    "nyash.map.key_prepare_utf8_v1",
    "nyash.map.key_dispose_v1",
    "nyash.map.outcome_init_v1",
    "nyash.map.checked_install_indexed_v1",
    "nyash.map.outcome_end_v1",
    "nyash.map.outcome_dispose_v1",
    "nyash.map.checked_end_v1",
    "nyash.map.storage_dispose_v1",
];
pub(super) fn require_checked_map_symbols(archive: &Path) -> Result<(), String> {
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
    validate(text)
}
fn validate(text: &str) -> Result<(), String> {
    for symbol in SYMBOLS {
        let mut count = 0;
        for line in text.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.first().copied() != Some(*symbol) { continue; }
            if fields.len() < 3 || !matches!(fields[1], "T" | "W") {
                return Err(format!("checked Map symbol is not a function: {symbol}"));
            }
            count += 1;
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
        let all = SYMBOLS
            .iter()
            .map(|name| format!("{name} T 0 1\n"))
            .collect::<String>();
        assert!(validate(&all).is_ok());
        for name in SYMBOLS {
            assert!(validate(&all.replace(&format!("{name} T"), &format!("{name} D"))).is_err());
            assert!(validate(&all.replace(&format!("{name} T 0 1\n"), "")).is_err());
            assert!(validate(&format!("{all}{name} T 0 1\n")).is_err());
        }
    }
}
