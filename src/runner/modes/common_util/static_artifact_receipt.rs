//! Root-side validator for the dedicated `ny-llvmc` artifact receipt path.
//!
//! This module is deliberately a transport consumer, not a second artifact or
//! MIR authority. It validates one child-published JSON receipt against the
//! already-issued candidate MIR JSON and returns no reconstructed receipt.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::mir::StaticArtifactReceiptConsumedFenceV1;

const SCHEMA_VERSION: u64 = 1;

pub(crate) fn consume_static_artifact_receipt(
    receipt_path: &Path,
    input_json: &Path,
    expected_published_path: Option<&Path>,
) -> Result<StaticArtifactReceiptConsumedFenceV1, String> {
    let receipt_bytes = fs::read(receipt_path)
        .map_err(|error| format!("static artifact receipt read failed: {error}"))?;
    let receipt: Value = serde_json::from_slice(&receipt_bytes)
        .map_err(|error| format!("static artifact receipt JSON invalid: {error}"))?;
    require_u64(&receipt, "schema_version", SCHEMA_VERSION)?;
    require_text(&receipt, "status", "published")?;

    let input_digest = sha256_file(input_json)?;
    require_text(&receipt, "input_sha256", &input_digest)?;
    if let Some(expected) = expected_published_path {
        require_text(&receipt, "published_path", &expected.display().to_string())?;
    }
    validate_digest_fields(&receipt)?;
    validate_descriptor_against_input(
        receipt
            .get("descriptor")
            .ok_or_else(|| "static artifact receipt missing descriptor".to_owned())?,
        input_json,
    )?;
    validate_symbol_census(
        receipt
            .get("symbol_census")
            .ok_or_else(|| "static artifact receipt missing symbol_census".to_owned())?,
    )?;
    Ok(StaticArtifactReceiptConsumedFenceV1::issue_from_root_validator())
}

fn validate_descriptor_against_input(descriptor: &Value, input_json: &Path) -> Result<(), String> {
    let input: Value = serde_json::from_slice(
        &fs::read(input_json).map_err(|error| format!("candidate MIR read failed: {error}"))?,
    )
    .map_err(|error| format!("candidate MIR JSON invalid: {error}"))?;
    let functions = input
        .get("functions")
        .and_then(Value::as_array)
        .ok_or_else(|| "candidate MIR functions missing".to_owned())?;
    let selected = functions
        .iter()
        .filter_map(|function| {
            function
                .get("metadata")
                .and_then(|metadata| metadata.get("dynamic_v2_aot_call_admission_v2"))
        })
        .collect::<Vec<_>>();
    if selected.len() != 1 {
        return Err(format!(
            "candidate MIR selected metadata count={} expected=1",
            selected.len()
        ));
    }
    let metadata = selected[0];
    for field in [
        "profile",
        "abi_revision",
        "wire_revision",
        "registry_generation",
        "contract_id",
    ] {
        if descriptor.get(field) != metadata.get(field) {
            return Err(format!("static artifact descriptor field drift: {field}"));
        }
    }
    if descriptor.get("compiler_domain")
        != metadata
            .get("plan_stamp")
            .and_then(|stamp| stamp.get("compiler_domain"))
        || descriptor.get("invocation_ordinal")
            != metadata
                .get("plan_stamp")
                .and_then(|stamp| stamp.get("invocation_ordinal"))
    {
        return Err("static artifact descriptor PlanStamp drift".to_owned());
    }
    let calls = metadata
        .get("calls")
        .and_then(Value::as_array)
        .ok_or_else(|| "candidate MIR calls missing".to_owned())?;
    let entries = descriptor
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| "static artifact descriptor entries missing".to_owned())?;
    if entries.len() != calls.len() {
        return Err("static artifact descriptor entry count drift".to_owned());
    }
    let mut expected = BTreeMap::new();
    for call in calls {
        let site = require_u64_value(call, "site_id")?;
        let arity = call
            .get("argument_lanes")
            .and_then(Value::as_array)
            .ok_or_else(|| "candidate MIR call argument_lanes missing".to_owned())?
            .len() as u64;
        if expected
            .insert(
                site,
                (
                    require_u64_value(call, "entry_id")?,
                    require_text_value(call, "symbol")?.to_owned(),
                    arity,
                ),
            )
            .is_some()
        {
            return Err("duplicate candidate MIR call site".to_owned());
        }
    }
    for entry in entries {
        let site = require_u64_value(entry, "site_id")?;
        let Some((entry_id, symbol, arity)) = expected.get(&site) else {
            return Err("foreign artifact descriptor call site".to_owned());
        };
        if require_u64_value(entry, "entry_id")? != *entry_id
            || require_text_value(entry, "symbol")? != symbol
            || require_u64_value(entry, "logical_arity")? != *arity
        {
            return Err("artifact descriptor call entry drift".to_owned());
        }
    }
    Ok(())
}

fn validate_digest_fields(receipt: &Value) -> Result<(), String> {
    for field in [
        "input_sha256",
        "object_digest",
        "runtime_archive_digest",
        "executable_digest",
        "descriptor_digest",
    ] {
        let value = require_text_value(receipt, field)?;
        if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(format!("invalid artifact digest: {field}"));
        }
    }
    Ok(())
}

fn validate_symbol_census(census: &Value) -> Result<(), String> {
    for field in [
        "required",
        "object_undefined",
        "archive_defined",
        "executable_defined",
    ] {
        if require_u64_value(census, field)? == 0 {
            return Err(format!("empty artifact symbol census: {field}"));
        }
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("artifact input digest failed: {error}"))?;
    Ok(Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn require_u64(object: &Value, field: &str, expected: u64) -> Result<(), String> {
    if require_u64_value(object, field)? != expected {
        return Err(format!("artifact receipt field drift: {field}"));
    }
    Ok(())
}

fn require_u64_value(object: &Value, field: &str) -> Result<u64, String> {
    object
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("artifact receipt missing u64 field: {field}"))
}

fn require_text(object: &Value, field: &str, expected: &str) -> Result<(), String> {
    if require_text_value(object, field)? != expected {
        return Err(format!("artifact receipt field drift: {field}"));
    }
    Ok(())
}

fn require_text_value<'a>(object: &'a Value, field: &str) -> Result<&'a str, String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("artifact receipt missing text field: {field}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fixture(root: &Path) -> (std::path::PathBuf, std::path::PathBuf) {
        let input = root.join("candidate.json");
        let receipt = root.join("receipt.json");
        let candidate = json!({
            "functions": [{"metadata": {"dynamic_v2_aot_call_admission_v2": {
                "profile": 1, "abi_revision": 1, "wire_revision": 2,
                "registry_generation": 7, "contract_id": "hako.text.scan@1",
                "plan_stamp": {"compiler_domain": 3, "invocation_ordinal": 9},
                "calls": [{"site_id": 0, "entry_id": 1, "symbol": "scan", "argument_lanes": []}]
            }}}]
        });
        fs::write(&input, serde_json::to_vec(&candidate).unwrap()).unwrap();
        let input_sha256 = sha256_file(&input).unwrap();
        let value = json!({
            "schema_version": 1, "status": "published", "input_sha256": input_sha256,
            "published_path": "/tmp/out", "object_digest": "a".repeat(64),
            "runtime_archive_digest": "b".repeat(64), "executable_digest": "c".repeat(64),
            "descriptor_digest": "d".repeat(64),
            "descriptor": {"profile": 1, "abi_revision": 1, "wire_revision": 2,
                "registry_generation": 7, "contract_id": "hako.text.scan@1",
                "compiler_domain": 3, "invocation_ordinal": 9,
                "entries": [{"site_id": 0, "entry_id": 1, "logical_arity": 0, "symbol": "scan"}]},
            "symbol_census": {"required": 1, "object_undefined": 1,
                "archive_defined": 1, "executable_defined": 1}
        });
        fs::write(&receipt, serde_json::to_vec(&value).unwrap()).unwrap();
        (input, receipt)
    }

    #[test]
    fn consumes_one_matching_receipt_without_reconstructing_a_product() {
        let root = std::env::temp_dir().join(format!(
            "hakorune_static_receipt_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let (input, receipt) = fixture(&root);
        consume_static_artifact_receipt(&receipt, &input, Some(Path::new("/tmp/out"))).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_input_digest_drift_before_any_publication_use() {
        let root = std::env::temp_dir().join(format!(
            "hakorune_static_receipt_drift_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let (input, receipt) = fixture(&root);
        fs::write(&input, b"changed").unwrap();
        assert!(consume_static_artifact_receipt(&receipt, &input, None).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
