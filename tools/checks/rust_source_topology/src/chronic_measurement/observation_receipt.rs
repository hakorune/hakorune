use std::path::{Component, Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::error::ChronicScanErrorV1;
use super::model::{
    ChronicAllowanceKindV1, ChronicMeasurementReportV1, ChronicObservationV1,
    CHRONIC_MEASUREMENT_SCHEMA_V1,
};
use super::scan::scan_scope_manifest;

pub const CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1: &str = "chronic-measurement-observations-v1";
const EXPECTED_ALLOWANCE_ROWS: usize = 185;
const SCANNER_VERSION: &str = "chronic-rust-token-scanner-v1";
const FROZEN_SCOPE_ID: &str = "mirbuilder-chronic-rust-v1";
const FROZEN_SCOPE_MANIFEST_HASH: &str =
    "sha256:be3a81afd0be8a2934b21fabcb0c43cb6d530462da7fdbe87ec1afdae126425f";
const FROZEN_SOURCE_SCOPE_HASH: &str =
    "sha256:8d7db05c42c007e556b15bc98fa3fbefbb69c5556b4733d838ccb5fd7cfeca39";
const FROZEN_SCANNER_EVIDENCE_HASH: &str =
    "sha256:cbbf224b2a635851d9502e789597abb7aa50421457ddce720a308d13d902957a";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChronicObservationReceiptV1 {
    pub schema: &'static str,
    pub schema_version: u32,
    pub scanner_version: String,
    pub scope_id: String,
    pub scope_manifest_hash: String,
    pub source_scope_hash: String,
    pub source_commit: String,
    pub scanner_evidence_hash: String,
    pub rows: Vec<ChronicObservationReceiptRowV1>,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChronicObservationReceiptRowV1 {
    pub row_kind: &'static str,
    pub path: String,
    pub item_key: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub attribute_kind: ChronicAllowanceKindV1,
    pub raw_condition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CanonicalKey {
    row_kind: &'static str,
    path: String,
    byte_start: usize,
    byte_end: usize,
    attribute_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct ReceiptHashInput {
    schema: &'static str,
    schema_version: u32,
    scanner_version: String,
    scope_id: String,
    scope_manifest_hash: String,
    source_scope_hash: String,
    source_commit: String,
    scanner_evidence_hash: String,
    rows: Vec<ChronicObservationReceiptRowV1>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ParsedObservationReceiptV1 {
    schema: String,
    schema_version: u32,
    scanner_version: String,
    scope_id: String,
    scope_manifest_hash: String,
    source_scope_hash: String,
    source_commit: String,
    scanner_evidence_hash: String,
    rows: Vec<ParsedObservationReceiptRowV1>,
    receipt_hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ParsedObservationReceiptRowV1 {
    row_kind: String,
    path: String,
    item_key: String,
    byte_start: usize,
    byte_end: usize,
    line_start: usize,
    line_end: usize,
    attribute_kind: ChronicAllowanceKindV1,
    raw_condition: Option<String>,
}

pub fn observation_receipt_json(
    manifest_path: &Path,
    workspace_root: &Path,
    source_commit: &str,
) -> Result<String, ChronicScanErrorV1> {
    validate_source_commit(source_commit)?;
    let report = scan_scope_manifest(manifest_path, workspace_root)?;
    let receipt = project_observation_receipt(&report, source_commit)?;
    let mut output = serde_json::to_string_pretty(&receipt).map_err(|error| {
        ChronicScanErrorV1::ReportSerialize {
            detail: error.to_string(),
        }
    })?;
    output.push('\n');
    Ok(output)
}

pub fn project_observation_receipt(
    report: &ChronicMeasurementReportV1,
    source_commit: &str,
) -> Result<ChronicObservationReceiptV1, ChronicScanErrorV1> {
    validate_source_commit(source_commit)?;
    validate_report_provenance(report)?;

    let mut rows = Vec::new();
    for file in &report.files {
        validate_relative_path(&file.path)?;
        for observation in &file.observations {
            let ChronicObservationV1::DeadCodeAllowance {
                source_range,
                item_key,
                attribute_kind,
                raw_condition,
                ..
            } = observation
            else {
                continue;
            };
            validate_observation_row(&file.path, item_key, source_range, *attribute_kind)?;
            rows.push(ChronicObservationReceiptRowV1 {
                row_kind: "dead_code_allowance",
                path: file.path.clone(),
                item_key: item_key.clone(),
                byte_start: source_range.byte_start,
                byte_end: source_range.byte_end,
                line_start: source_range.start.line,
                line_end: source_range.end.line,
                attribute_kind: *attribute_kind,
                raw_condition: raw_condition.clone(),
            });
        }
    }

    if rows.len() != EXPECTED_ALLOWANCE_ROWS {
        return Err(ChronicScanErrorV1::ObservationReceiptCountDrift {
            expected: EXPECTED_ALLOWANCE_ROWS,
            actual: rows.len(),
        });
    }
    canonicalize_rows(&mut rows)?;
    if report.summary.dead_code_allowance_count != rows.len() {
        return Err(ChronicScanErrorV1::ObservationReceiptCountDrift {
            expected: report.summary.dead_code_allowance_count,
            actual: rows.len(),
        });
    }

    let mut receipt = ChronicObservationReceiptV1 {
        schema: CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1,
        schema_version: 1,
        scanner_version: report.scanner_version.clone(),
        scope_id: report.scope_id.clone(),
        scope_manifest_hash: report.scope_manifest_hash.clone(),
        source_scope_hash: report.source_scope_hash.clone(),
        source_commit: source_commit.to_string(),
        scanner_evidence_hash: report.evidence_hash.clone(),
        rows,
        receipt_hash: String::new(),
    };
    receipt.receipt_hash = receipt_hash(&receipt)?;
    let sealed_hash = receipt_hash(&receipt)?;
    if receipt.receipt_hash != sealed_hash {
        return Err(ChronicScanErrorV1::ObservationReceiptHashDrift {
            expected: sealed_hash,
            actual: receipt.receipt_hash,
        });
    }
    Ok(receipt)
}

pub fn validate_observation_receipt_json(
    input: &str,
    expected_source_commit: &str,
) -> Result<ChronicObservationReceiptV1, ChronicScanErrorV1> {
    validate_source_commit(expected_source_commit)?;
    let parsed: ParsedObservationReceiptV1 = serde_json::from_str(input).map_err(|error| {
        ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: format!("receipt JSON parse failed: {error}"),
        }
    })?;
    validate_frozen_contract(&parsed, expected_source_commit)?;
    if parsed.rows.len() != EXPECTED_ALLOWANCE_ROWS {
        return Err(ChronicScanErrorV1::ObservationReceiptCountDrift {
            expected: EXPECTED_ALLOWANCE_ROWS,
            actual: parsed.rows.len(),
        });
    }
    let mut rows = Vec::with_capacity(parsed.rows.len());
    for row in parsed.rows {
        if row.row_kind != "dead_code_allowance" {
            return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
                detail: format!("unsupported row_kind: {}", row.row_kind),
            });
        }
        let source_range = crate::model::SourceRangeV1 {
            start: crate::model::PositionV1 {
                line: row.line_start,
                column: 0,
            },
            end: crate::model::PositionV1 {
                line: row.line_end,
                column: 0,
            },
            byte_start: row.byte_start,
            byte_end: row.byte_end,
        };
        validate_relative_path(&row.path)?;
        validate_observation_row(&row.path, &row.item_key, &source_range, row.attribute_kind)?;
        rows.push(ChronicObservationReceiptRowV1 {
            row_kind: "dead_code_allowance",
            path: row.path,
            item_key: row.item_key,
            byte_start: row.byte_start,
            byte_end: row.byte_end,
            line_start: row.line_start,
            line_end: row.line_end,
            attribute_kind: row.attribute_kind,
            raw_condition: row.raw_condition,
        });
    }
    canonicalize_rows(&mut rows)?;
    let receipt = ChronicObservationReceiptV1 {
        schema: CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1,
        schema_version: 1,
        scanner_version: parsed.scanner_version,
        scope_id: parsed.scope_id,
        scope_manifest_hash: parsed.scope_manifest_hash,
        source_scope_hash: parsed.source_scope_hash,
        source_commit: parsed.source_commit,
        scanner_evidence_hash: parsed.scanner_evidence_hash,
        rows,
        receipt_hash: parsed.receipt_hash,
    };
    let expected_hash = receipt_hash(&receipt)?;
    if receipt.receipt_hash != expected_hash {
        return Err(ChronicScanErrorV1::ObservationReceiptHashDrift {
            expected: expected_hash,
            actual: receipt.receipt_hash,
        });
    }
    Ok(receipt)
}

fn validate_frozen_contract(
    receipt: &ParsedObservationReceiptV1,
    expected_source_commit: &str,
) -> Result<(), ChronicScanErrorV1> {
    let checks = [
        (
            "schema",
            receipt.schema.as_str(),
            CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1,
        ),
        (
            "scanner_version",
            receipt.scanner_version.as_str(),
            SCANNER_VERSION,
        ),
        ("scope_id", receipt.scope_id.as_str(), FROZEN_SCOPE_ID),
        (
            "scope_manifest_hash",
            receipt.scope_manifest_hash.as_str(),
            FROZEN_SCOPE_MANIFEST_HASH,
        ),
        (
            "source_scope_hash",
            receipt.source_scope_hash.as_str(),
            FROZEN_SOURCE_SCOPE_HASH,
        ),
        (
            "scanner_evidence_hash",
            receipt.scanner_evidence_hash.as_str(),
            FROZEN_SCANNER_EVIDENCE_HASH,
        ),
        (
            "source_commit",
            receipt.source_commit.as_str(),
            expected_source_commit,
        ),
    ];
    if receipt.schema_version != 1 {
        return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: format!("schema_version must be 1, got {}", receipt.schema_version),
        });
    }
    for (label, actual, expected) in checks {
        if actual != expected {
            return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
                detail: format!("{label} drift: expected={expected} actual={actual}"),
            });
        }
    }
    Ok(())
}

fn validate_source_commit(source_commit: &str) -> Result<(), ChronicScanErrorV1> {
    if source_commit.len() != 40 || !source_commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ChronicScanErrorV1::InvalidSourceCommit {
            detail: "source_commit must be exactly 40 ASCII hexadecimal characters".into(),
        });
    }
    Ok(())
}

fn validate_report_provenance(
    report: &ChronicMeasurementReportV1,
) -> Result<(), ChronicScanErrorV1> {
    if report.schema != CHRONIC_MEASUREMENT_SCHEMA_V1 || report.schema_version != 1 {
        return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: "scanner report schema/version is not chronic-measurement-v1/1".into(),
        });
    }
    if report.scanner_version != SCANNER_VERSION {
        return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: format!("scanner_version must be {SCANNER_VERSION}"),
        });
    }
    if report.scope_id.is_empty() {
        return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: "scope_id is empty".into(),
        });
    }
    for (label, digest) in [
        ("scope_manifest_hash", &report.scope_manifest_hash),
        ("source_scope_hash", &report.source_scope_hash),
        ("evidence_hash", &report.evidence_hash),
    ] {
        validate_digest(label, digest)?;
    }
    Ok(())
}

fn validate_digest(label: &str, digest: &str) -> Result<(), ChronicScanErrorV1> {
    let valid = digest.len() == 71
        && digest.starts_with("sha256:")
        && digest[7..].bytes().all(|byte| byte.is_ascii_hexdigit());
    if !valid {
        return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: format!("{label} must be a sha256:<64-hex> digest"),
        });
    }
    Ok(())
}

fn validate_relative_path(path: &str) -> Result<(), ChronicScanErrorV1> {
    let value = Path::new(path);
    if path.is_empty()
        || value.is_absolute()
        || path.contains('\\')
        || value.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(ChronicScanErrorV1::PathEscape {
            path: path.to_string(),
        });
    }
    Ok(())
}

fn validate_observation_row(
    path: &str,
    item_key: &str,
    source_range: &crate::model::SourceRangeV1,
    _attribute_kind: ChronicAllowanceKindV1,
) -> Result<(), ChronicScanErrorV1> {
    if item_key.is_empty() {
        return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: format!("{path}: empty item_key"),
        });
    }
    if source_range.byte_start >= source_range.byte_end
        || source_range.start.line == 0
        || source_range.end.line < source_range.start.line
        || (source_range.start.line == source_range.end.line
            && source_range.start.column > source_range.end.column)
    {
        return Err(ChronicScanErrorV1::ObservationReceiptInvalid {
            detail: format!("{path}:{item_key}: invalid source range"),
        });
    }
    Ok(())
}

fn canonical_key(row: &ChronicObservationReceiptRowV1) -> CanonicalKey {
    CanonicalKey {
        row_kind: row.row_kind,
        path: row.path.clone(),
        byte_start: row.byte_start,
        byte_end: row.byte_end,
        attribute_kind: allowance_kind_name(row.attribute_kind),
    }
}

fn canonicalize_rows(
    rows: &mut Vec<ChronicObservationReceiptRowV1>,
) -> Result<(), ChronicScanErrorV1> {
    let input_keys: Vec<CanonicalKey> = rows.iter().map(canonical_key).collect();
    for pair in input_keys.windows(2) {
        if pair[0] > pair[1] {
            return Err(ChronicScanErrorV1::ObservationReceiptOutOfOrder {
                previous: key_display(&pair[0]),
                current: key_display(&pair[1]),
            });
        }
    }
    rows.sort_by_key(canonical_key);
    for pair in rows.windows(2) {
        let previous = canonical_key(&pair[0]);
        let current = canonical_key(&pair[1]);
        if previous == current {
            return Err(ChronicScanErrorV1::ObservationReceiptDuplicateKey {
                key: key_display(&current),
            });
        }
    }
    Ok(())
}

fn allowance_kind_name(kind: ChronicAllowanceKindV1) -> &'static str {
    match kind {
        ChronicAllowanceKindV1::OuterAllow => "outer_allow",
        ChronicAllowanceKindV1::InnerAllow => "inner_allow",
        ChronicAllowanceKindV1::CfgAttrAllow => "cfg_attr_allow",
    }
}

fn key_display(key: &CanonicalKey) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        key.row_kind, key.path, key.byte_start, key.byte_end, key.attribute_kind
    )
}

fn receipt_hash(receipt: &ChronicObservationReceiptV1) -> Result<String, ChronicScanErrorV1> {
    let input = ReceiptHashInput {
        schema: receipt.schema,
        schema_version: receipt.schema_version,
        scanner_version: receipt.scanner_version.clone(),
        scope_id: receipt.scope_id.clone(),
        scope_manifest_hash: receipt.scope_manifest_hash.clone(),
        source_scope_hash: receipt.source_scope_hash.clone(),
        source_commit: receipt.source_commit.clone(),
        scanner_evidence_hash: receipt.scanner_evidence_hash.clone(),
        rows: receipt.rows.clone(),
    };
    let bytes =
        serde_json::to_vec(&input).map_err(|error| ChronicScanErrorV1::ReportSerialize {
            detail: error.to_string(),
        })?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{PositionV1, SourceRangeV1};

    fn row(path: &str, start: usize, end: usize) -> ChronicObservationReceiptRowV1 {
        ChronicObservationReceiptRowV1 {
            row_kind: "dead_code_allowance",
            path: path.into(),
            item_key: "item".into(),
            byte_start: start,
            byte_end: end,
            line_start: 1,
            line_end: 1,
            attribute_kind: ChronicAllowanceKindV1::OuterAllow,
            raw_condition: None,
        }
    }

    #[test]
    fn source_commit_requires_full_hex_revision() {
        assert!(validate_source_commit("abc").is_err());
        assert!(validate_source_commit(&"g".repeat(40)).is_err());
        assert!(validate_source_commit(&"a".repeat(40)).is_ok());
    }

    #[test]
    fn range_validation_rejects_reversed_rows() {
        let range = SourceRangeV1 {
            start: PositionV1 { line: 2, column: 0 },
            end: PositionV1 { line: 1, column: 0 },
            byte_start: 2,
            byte_end: 1,
        };
        assert!(validate_observation_row(
            "src/lib.rs",
            "item",
            &range,
            ChronicAllowanceKindV1::OuterAllow,
        )
        .is_err());
    }

    #[test]
    fn range_validation_rejects_zero_length_rows() {
        let range = SourceRangeV1 {
            start: PositionV1 { line: 1, column: 0 },
            end: PositionV1 { line: 1, column: 0 },
            byte_start: 7,
            byte_end: 7,
        };
        assert!(validate_observation_row(
            "src/lib.rs",
            "item",
            &range,
            ChronicAllowanceKindV1::OuterAllow,
        )
        .is_err());
    }

    #[test]
    fn hash_omits_receipt_field() {
        let mut receipt = ChronicObservationReceiptV1 {
            schema: CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1,
            schema_version: 1,
            scanner_version: SCANNER_VERSION.into(),
            scope_id: "scope".into(),
            scope_manifest_hash: "sha256:".to_string() + &"a".repeat(64),
            source_scope_hash: "sha256:".to_string() + &"b".repeat(64),
            source_commit: "c".repeat(40),
            scanner_evidence_hash: "sha256:".to_string() + &"d".repeat(64),
            rows: vec![row("src/lib.rs", 1, 2)],
            receipt_hash: "first".into(),
        };
        let first = receipt_hash(&receipt).unwrap();
        receipt.receipt_hash = "second".into();
        assert_eq!(first, receipt_hash(&receipt).unwrap());
    }

    #[test]
    fn canonical_rows_reject_out_of_order_and_duplicate_keys() {
        let mut out_of_order = vec![row("src/z.rs", 1, 2), row("src/a.rs", 1, 2)];
        assert!(matches!(
            canonicalize_rows(&mut out_of_order),
            Err(ChronicScanErrorV1::ObservationReceiptOutOfOrder { .. })
        ));

        let mut duplicate = vec![row("src/a.rs", 1, 2), row("src/a.rs", 1, 2)];
        assert!(matches!(
            canonicalize_rows(&mut duplicate),
            Err(ChronicScanErrorV1::ObservationReceiptDuplicateKey { .. })
        ));
    }
}
