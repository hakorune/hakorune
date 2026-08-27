use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::error::ChronicScanErrorV1;
use super::model::ChronicAllowanceKindV1;
use super::observation_receipt::{
    allowance_kind_name, validate_observation_receipt_json, ChronicObservationReceiptRowV1,
    ChronicObservationReceiptV1, CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1,
};

pub const CHRONIC_SITE_OWNER_MAP_SCHEMA_V1: &str = "chronic-measurement-site-owners-v1";
const MAP_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChronicSiteOwnerMapV1 {
    pub schema: String,
    pub schema_version: u32,
    pub map_id: String,
    pub map_hash: String,
    pub scanner_version: String,
    pub scope_id: String,
    pub scope_manifest_hash: String,
    pub source_scope_hash: String,
    pub source_commit: String,
    pub scanner_evidence_hash: String,
    pub observation_receipt_id: String,
    pub observation_receipt_hash: String,
    pub sites: Vec<ChronicSiteOwnerMapRowV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChronicSiteOwnerMapRowV1 {
    pub row_kind: String,
    pub path: String,
    pub item_key: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub attribute_kind: ChronicAllowanceKindV1,
    pub compile_domain: String,
    pub role: String,
    pub owner_ref: String,
    pub successor_status: String,
    pub successor_ref: String,
    pub retirement_status: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ParsedSiteOwnerMapV1 {
    schema: String,
    schema_version: u32,
    map_id: String,
    map_hash: String,
    scanner_version: String,
    scope_id: String,
    scope_manifest_hash: String,
    source_scope_hash: String,
    source_commit: String,
    scanner_evidence_hash: String,
    observation_receipt_id: String,
    observation_receipt_hash: String,
    sites: Vec<ParsedSiteOwnerMapRowV1>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ParsedSiteOwnerMapRowV1 {
    row_kind: String,
    path: String,
    item_key: String,
    byte_start: usize,
    byte_end: usize,
    line_start: usize,
    line_end: usize,
    attribute_kind: ChronicAllowanceKindV1,
    compile_domain: String,
    role: String,
    owner_ref: String,
    successor_status: String,
    successor_ref: String,
    retirement_status: String,
    evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SiteKey {
    row_kind: String,
    path: String,
    byte_start: usize,
    byte_end: usize,
    attribute_kind: String,
}

#[derive(Debug, Serialize)]
struct SiteOwnerMapHashInput {
    schema: String,
    schema_version: u32,
    map_id: String,
    scanner_version: String,
    scope_id: String,
    scope_manifest_hash: String,
    source_scope_hash: String,
    source_commit: String,
    scanner_evidence_hash: String,
    observation_receipt_id: String,
    observation_receipt_hash: String,
    sites: Vec<ChronicSiteOwnerMapRowV1>,
}

pub fn validate_site_owner_map_toml(
    map_input: &str,
    observation_receipt_input: &str,
    expected_source_commit: &str,
) -> Result<ChronicSiteOwnerMapV1, ChronicScanErrorV1> {
    let receipt =
        validate_observation_receipt_json(observation_receipt_input, expected_source_commit)?;
    let parsed: ParsedSiteOwnerMapV1 =
        toml::from_str(map_input).map_err(|error| ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: format!("site-owner map TOML parse failed: {error}"),
        })?;
    validate_map_header(&parsed, &receipt, expected_source_commit)?;
    let rows = validate_map_rows(&parsed.sites, &receipt)?;
    let map = ChronicSiteOwnerMapV1 {
        schema: parsed.schema,
        schema_version: parsed.schema_version,
        map_id: parsed.map_id,
        map_hash: parsed.map_hash,
        scanner_version: parsed.scanner_version,
        scope_id: parsed.scope_id,
        scope_manifest_hash: parsed.scope_manifest_hash,
        source_scope_hash: parsed.source_scope_hash,
        source_commit: parsed.source_commit,
        scanner_evidence_hash: parsed.scanner_evidence_hash,
        observation_receipt_id: parsed.observation_receipt_id,
        observation_receipt_hash: parsed.observation_receipt_hash,
        sites: rows,
    };
    let expected_hash = site_owner_map_hash(&map)?;
    if map.map_hash != expected_hash {
        return Err(ChronicScanErrorV1::SiteOwnerMapHashDrift {
            expected: expected_hash,
            actual: map.map_hash,
        });
    }
    Ok(map)
}

fn validate_map_header(
    map: &ParsedSiteOwnerMapV1,
    receipt: &ChronicObservationReceiptV1,
    expected_source_commit: &str,
) -> Result<(), ChronicScanErrorV1> {
    if map.schema != CHRONIC_SITE_OWNER_MAP_SCHEMA_V1 || map.schema_version != MAP_SCHEMA_VERSION {
        return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: format!(
                "schema/version must be {CHRONIC_SITE_OWNER_MAP_SCHEMA_V1}/1, got {}/{}",
                map.schema, map.schema_version
            ),
        });
    }
    let expected_map_id = format!("{CHRONIC_SITE_OWNER_MAP_SCHEMA_V1}@{expected_source_commit}");
    if map.map_id != expected_map_id {
        return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: format!(
                "map_id drift: expected={expected_map_id} actual={}",
                map.map_id
            ),
        });
    }
    if !is_sha256_digest(&map.map_hash) {
        return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: "map_hash must be a sha256:<64-lowercase-hex> digest".into(),
        });
    }
    let expected_receipt_id =
        format!("{CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1}@{expected_source_commit}");
    if map.observation_receipt_id != expected_receipt_id
        || map.observation_receipt_hash != receipt.receipt_hash
        || map.source_commit != expected_source_commit
    {
        return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: "observation receipt identity or source_commit drift".into(),
        });
    }
    let provenance = [
        (
            "scanner_version",
            map.scanner_version.as_str(),
            receipt.scanner_version.as_str(),
        ),
        ("scope_id", map.scope_id.as_str(), receipt.scope_id.as_str()),
        (
            "scope_manifest_hash",
            map.scope_manifest_hash.as_str(),
            receipt.scope_manifest_hash.as_str(),
        ),
        (
            "source_scope_hash",
            map.source_scope_hash.as_str(),
            receipt.source_scope_hash.as_str(),
        ),
        (
            "scanner_evidence_hash",
            map.scanner_evidence_hash.as_str(),
            receipt.scanner_evidence_hash.as_str(),
        ),
    ];
    for (label, actual, expected) in provenance {
        if actual != expected {
            return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
                detail: format!("{label} drift: expected={expected} actual={actual}"),
            });
        }
    }
    Ok(())
}

fn validate_map_rows(
    rows: &[ParsedSiteOwnerMapRowV1],
    receipt: &ChronicObservationReceiptV1,
) -> Result<Vec<ChronicSiteOwnerMapRowV1>, ChronicScanErrorV1> {
    if rows.len() != receipt.rows.len() {
        return Err(ChronicScanErrorV1::SiteOwnerMapCoverageDrift {
            detail: format!(
                "row count drift: expected={} actual={}",
                receipt.rows.len(),
                rows.len()
            ),
        });
    }
    let mut previous = None;
    let mut normalized = Vec::with_capacity(rows.len());
    for (index, row) in rows.iter().enumerate() {
        validate_map_row(row)?;
        let key = site_key(row);
        if let Some(previous_key) = &previous {
            if previous_key >= &key {
                let kind = if previous_key == &key {
                    "duplicate"
                } else {
                    "out-of-order"
                };
                return Err(ChronicScanErrorV1::SiteOwnerMapCoverageDrift {
                    detail: format!("{kind} canonical key at row {index}"),
                });
            }
        }
        let receipt_row = receipt.rows.get(index).ok_or_else(|| {
            ChronicScanErrorV1::SiteOwnerMapCoverageDrift {
                detail: format!("missing receipt row at index {index}"),
            }
        })?;
        if key != receipt_key(receipt_row) {
            return Err(ChronicScanErrorV1::SiteOwnerMapCoverageDrift {
                detail: format!(
                    "foreign/missing canonical key at row {index}: expected={} actual={}",
                    key_display(&receipt_key(receipt_row)),
                    key_display(&key)
                ),
            });
        }
        if row.item_key != receipt_row.item_key
            || row.line_start != receipt_row.line_start
            || row.line_end != receipt_row.line_end
        {
            return Err(ChronicScanErrorV1::SiteOwnerMapCoverageDrift {
                detail: format!("consistency fields drift at row {index}"),
            });
        }
        previous = Some(key);
        let mut evidence_refs = row.evidence_refs.clone();
        evidence_refs.sort();
        normalized.push(ChronicSiteOwnerMapRowV1 {
            row_kind: row.row_kind.clone(),
            path: row.path.clone(),
            item_key: row.item_key.clone(),
            byte_start: row.byte_start,
            byte_end: row.byte_end,
            line_start: row.line_start,
            line_end: row.line_end,
            attribute_kind: row.attribute_kind,
            compile_domain: row.compile_domain.clone(),
            role: row.role.clone(),
            owner_ref: row.owner_ref.clone(),
            successor_status: row.successor_status.clone(),
            successor_ref: row.successor_ref.clone(),
            retirement_status: row.retirement_status.clone(),
            evidence_refs,
        });
    }
    Ok(normalized)
}

fn validate_map_row(row: &ParsedSiteOwnerMapRowV1) -> Result<(), ChronicScanErrorV1> {
    if row.row_kind != "dead_code_allowance"
        || row.path.is_empty()
        || row.item_key.is_empty()
        || row.byte_start >= row.byte_end
        || row.line_start == 0
        || row.line_end < row.line_start
    {
        return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: format!("invalid site row identity/range: {}", row.path),
        });
    }
    if !matches!(
        row.compile_domain.as_str(),
        "production_default" | "cfg_test" | "feature_nonselected" | "generated_included"
    ) || !matches!(
        row.role.as_str(),
        "runtime"
            | "test_support"
            | "fixture"
            | "compatibility"
            | "guard_evidence"
            | "generated_registry"
            | "mixed"
    ) || !matches!(
        row.retirement_status.as_str(),
        "Unknown" | "Retain" | "Candidate"
    ) {
        return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: format!("invalid classification fields for {}", row.path),
        });
    }
    if row.owner_ref.is_empty() || row.evidence_refs.is_empty() {
        return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
            detail: format!("owner/evidence is empty for {}", row.path),
        });
    }
    validate_reference(&row.owner_ref)?;
    let mut evidence_seen = BTreeSet::new();
    for reference in &row.evidence_refs {
        validate_reference(reference)?;
        if !evidence_seen.insert(reference) {
            return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
                detail: format!("duplicate evidence reference for {}", row.path),
            });
        }
    }
    match row.successor_status.as_str() {
        "required" => validate_reference(&row.successor_ref)?,
        "not_required" if row.successor_ref == "none_required" => {}
        _ => {
            return Err(ChronicScanErrorV1::SiteOwnerMapInvalid {
                detail: format!("invalid successor contract for {}", row.path),
            })
        }
    }
    Ok(())
}

fn validate_reference(reference: &str) -> Result<(), ChronicScanErrorV1> {
    let Some((body, revision)) = reference.rsplit_once('@') else {
        return Err(reference_error("missing revision"));
    };
    if revision.len() != 40
        || !revision
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(reference_error(
            "revision must be 40 lowercase hex characters",
        ));
    }
    let Some((path, anchor)) = body.split_once('#') else {
        return Err(reference_error("missing anchor"));
    };
    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path.split('/').any(|part| part == "..")
        || path.chars().any(char::is_whitespace)
        || anchor.is_empty()
        || anchor.contains('#')
        || anchor.contains('@')
        || anchor.chars().any(char::is_whitespace)
    {
        return Err(reference_error("invalid tracked path or anchor"));
    }
    let valid_anchor = ["symbol:", "range:", "edge:"].iter().any(|prefix| {
        anchor
            .strip_prefix(prefix)
            .is_some_and(|value| !value.is_empty())
    });
    if !valid_anchor {
        return Err(reference_error("anchor must use symbol:, range:, or edge:"));
    }
    Ok(())
}

fn reference_error(detail: &str) -> ChronicScanErrorV1 {
    ChronicScanErrorV1::SiteOwnerMapInvalid {
        detail: format!("invalid tracked reference: {detail}"),
    }
}

fn site_owner_map_hash(map: &ChronicSiteOwnerMapV1) -> Result<String, ChronicScanErrorV1> {
    let input = SiteOwnerMapHashInput {
        schema: map.schema.clone(),
        schema_version: map.schema_version,
        map_id: map.map_id.clone(),
        scanner_version: map.scanner_version.clone(),
        scope_id: map.scope_id.clone(),
        scope_manifest_hash: map.scope_manifest_hash.clone(),
        source_scope_hash: map.source_scope_hash.clone(),
        source_commit: map.source_commit.clone(),
        scanner_evidence_hash: map.scanner_evidence_hash.clone(),
        observation_receipt_id: map.observation_receipt_id.clone(),
        observation_receipt_hash: map.observation_receipt_hash.clone(),
        sites: map.sites.clone(),
    };
    let bytes =
        serde_json::to_vec(&input).map_err(|error| ChronicScanErrorV1::ReportSerialize {
            detail: error.to_string(),
        })?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

fn is_sha256_digest(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn site_key(row: &ParsedSiteOwnerMapRowV1) -> SiteKey {
    SiteKey {
        row_kind: row.row_kind.clone(),
        path: row.path.clone(),
        byte_start: row.byte_start,
        byte_end: row.byte_end,
        attribute_kind: allowance_kind_name(row.attribute_kind).into(),
    }
}

fn receipt_key(row: &ChronicObservationReceiptRowV1) -> SiteKey {
    SiteKey {
        row_kind: row.row_kind.into(),
        path: row.path.clone(),
        byte_start: row.byte_start,
        byte_end: row.byte_end,
        attribute_kind: allowance_kind_name(row.attribute_kind).into(),
    }
}

fn key_display(key: &SiteKey) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        key.row_kind, key.path, key.byte_start, key.byte_end, key.attribute_kind
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const SOURCE_COMMIT: &str = "d9cff5b744edee3b6450db5d0ffc74478f32b49a";

    fn receipt_text() -> String {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        std::fs::read_to_string(
            workspace.join("tools/checks/manifests/chronic_measurement_observations_v1.json"),
        )
        .unwrap()
    }

    fn valid_map() -> ParsedSiteOwnerMapV1 {
        let receipt = validate_observation_receipt_json(&receipt_text(), SOURCE_COMMIT).unwrap();
        let evidence = format!(
            "tools/checks/manifests/chronic_measurement_observations_v1.json#edge:receipt@{SOURCE_COMMIT}"
        );
        let sites = receipt
            .rows
            .iter()
            .map(|row| ParsedSiteOwnerMapRowV1 {
                row_kind: row.row_kind.into(),
                path: row.path.clone(),
                item_key: row.item_key.clone(),
                byte_start: row.byte_start,
                byte_end: row.byte_end,
                line_start: row.line_start,
                line_end: row.line_end,
                attribute_kind: row.attribute_kind,
                compile_domain: "production_default".into(),
                role: "runtime".into(),
                owner_ref: evidence.clone(),
                successor_status: "not_required".into(),
                successor_ref: "none_required".into(),
                retirement_status: "Retain".into(),
                evidence_refs: vec![evidence.clone()],
            })
            .collect();
        let mut map = ParsedSiteOwnerMapV1 {
            schema: CHRONIC_SITE_OWNER_MAP_SCHEMA_V1.into(),
            schema_version: MAP_SCHEMA_VERSION,
            map_id: format!("{CHRONIC_SITE_OWNER_MAP_SCHEMA_V1}@{SOURCE_COMMIT}"),
            map_hash: String::new(),
            scanner_version: receipt.scanner_version,
            scope_id: receipt.scope_id,
            scope_manifest_hash: receipt.scope_manifest_hash,
            source_scope_hash: receipt.source_scope_hash,
            source_commit: SOURCE_COMMIT.into(),
            scanner_evidence_hash: receipt.scanner_evidence_hash,
            observation_receipt_id: format!(
                "{CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1}@{SOURCE_COMMIT}"
            ),
            observation_receipt_hash: receipt.receipt_hash,
            sites,
        };
        let public = parsed_to_public(&map).unwrap();
        map.map_hash = site_owner_map_hash(&public).unwrap();
        map
    }

    fn parsed_to_public(
        map: &ParsedSiteOwnerMapV1,
    ) -> Result<ChronicSiteOwnerMapV1, ChronicScanErrorV1> {
        Ok(ChronicSiteOwnerMapV1 {
            schema: map.schema.clone(),
            schema_version: map.schema_version,
            map_id: map.map_id.clone(),
            map_hash: map.map_hash.clone(),
            scanner_version: map.scanner_version.clone(),
            scope_id: map.scope_id.clone(),
            scope_manifest_hash: map.scope_manifest_hash.clone(),
            source_scope_hash: map.source_scope_hash.clone(),
            source_commit: map.source_commit.clone(),
            scanner_evidence_hash: map.scanner_evidence_hash.clone(),
            observation_receipt_id: map.observation_receipt_id.clone(),
            observation_receipt_hash: map.observation_receipt_hash.clone(),
            sites: map
                .sites
                .iter()
                .map(|row| ChronicSiteOwnerMapRowV1 {
                    row_kind: row.row_kind.clone(),
                    path: row.path.clone(),
                    item_key: row.item_key.clone(),
                    byte_start: row.byte_start,
                    byte_end: row.byte_end,
                    line_start: row.line_start,
                    line_end: row.line_end,
                    attribute_kind: row.attribute_kind,
                    compile_domain: row.compile_domain.clone(),
                    role: row.role.clone(),
                    owner_ref: row.owner_ref.clone(),
                    successor_status: row.successor_status.clone(),
                    successor_ref: row.successor_ref.clone(),
                    retirement_status: row.retirement_status.clone(),
                    evidence_refs: row.evidence_refs.clone(),
                })
                .collect(),
        })
    }

    fn map_text(map: &ParsedSiteOwnerMapV1) -> String {
        toml::to_string(map).unwrap()
    }

    #[test]
    fn strict_map_validator_accepts_test_only_exact_185_rows() {
        let map = valid_map();
        let validated =
            validate_site_owner_map_toml(&map_text(&map), &receipt_text(), SOURCE_COMMIT).unwrap();
        assert_eq!(validated.sites.len(), 185);
        assert_eq!(validated.source_commit, SOURCE_COMMIT);
        assert!(is_sha256_digest(&validated.map_hash));
    }

    #[test]
    fn strict_map_validator_rejects_partial_hash_and_reference_drift() {
        let mut partial = valid_map();
        partial.sites.pop();
        let error =
            validate_site_owner_map_toml(&map_text(&partial), &receipt_text(), SOURCE_COMMIT)
                .unwrap_err();
        assert!(matches!(
            error,
            ChronicScanErrorV1::SiteOwnerMapCoverageDrift { .. }
        ));

        let mut malformed_ref = valid_map();
        malformed_ref.sites[0].owner_ref = "src/lib.rs".into();
        let error =
            validate_site_owner_map_toml(&map_text(&malformed_ref), &receipt_text(), SOURCE_COMMIT)
                .unwrap_err();
        assert!(matches!(
            error,
            ChronicScanErrorV1::SiteOwnerMapInvalid { .. }
        ));

        let mut hash_drift = valid_map();
        hash_drift.map_hash = "sha256:".to_string() + &"0".repeat(64);
        let error =
            validate_site_owner_map_toml(&map_text(&hash_drift), &receipt_text(), SOURCE_COMMIT)
                .unwrap_err();
        assert!(matches!(
            error,
            ChronicScanErrorV1::SiteOwnerMapHashDrift { .. }
        ));
    }

    #[test]
    fn strict_map_validator_rejects_unknown_fields_and_foreign_rows() {
        let map = valid_map();
        let mut text = map_text(&map);
        text.push_str("unexpected = true\n");
        let error =
            validate_site_owner_map_toml(&text, &receipt_text(), SOURCE_COMMIT).unwrap_err();
        assert!(matches!(
            error,
            ChronicScanErrorV1::SiteOwnerMapInvalid { .. }
        ));

        let mut foreign = valid_map();
        foreign.sites[0].path = "src/foreign.rs".into();
        let error =
            validate_site_owner_map_toml(&map_text(&foreign), &receipt_text(), SOURCE_COMMIT)
                .unwrap_err();
        assert!(matches!(
            error,
            ChronicScanErrorV1::SiteOwnerMapCoverageDrift { .. }
        ));
    }
}
