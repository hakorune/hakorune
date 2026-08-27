use super::*;
use crate::chronic_measurement::error::SiteOwnerMapReferenceFailureV1;
use crate::chronic_measurement::site_owner_map_reference::{
    validate_site_owner_map_toml_with_references_v1, SiteOwnerMapReferenceContextV1,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::Path;

const SOURCE_COMMIT: &str = "d9cff5b744edee3b6450db5d0ffc74478f32b49a";
const REVIEW_HEAD: &str = "f0e93a60e38061c4527fda806804ab8959d43b75";

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
        observation_receipt_id: format!("{CHRONIC_OBSERVATION_RECEIPT_SCHEMA_V1}@{SOURCE_COMMIT}"),
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

fn map_with_reference(reference: &str) -> String {
    let mut map = valid_map();
    for site in &mut map.sites {
        site.owner_ref = reference.to_string();
        site.evidence_refs = vec![reference.to_string()];
    }
    let public = parsed_to_public(&map).unwrap();
    map.map_hash = site_owner_map_hash(&public).unwrap();
    map_text(&map)
}

#[test]
fn strict_map_validator_accepts_test_only_exact_185_rows() {
    let map = valid_map();
    let validated =
        validate_site_owner_map_toml_syntax_v1(&map_text(&map), &receipt_text(), SOURCE_COMMIT)
            .unwrap();
    assert_eq!(validated.sites.len(), 185);
    assert_eq!(validated.source_commit, SOURCE_COMMIT);
    assert!(is_sha256_digest(&validated.map_hash));
}

#[test]
fn strict_map_validator_rejects_partial_hash_and_reference_drift() {
    let mut partial = valid_map();
    partial.sites.pop();
    let error =
        validate_site_owner_map_toml_syntax_v1(&map_text(&partial), &receipt_text(), SOURCE_COMMIT)
            .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapCoverageDrift { .. }
    ));

    let mut malformed_ref = valid_map();
    malformed_ref.sites[0].owner_ref = "src/lib.rs".into();
    let error = validate_site_owner_map_toml_syntax_v1(
        &map_text(&malformed_ref),
        &receipt_text(),
        SOURCE_COMMIT,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapInvalid { .. }
    ));

    let mut hash_drift = valid_map();
    hash_drift.map_hash = "sha256:".to_string() + &"0".repeat(64);
    let error = validate_site_owner_map_toml_syntax_v1(
        &map_text(&hash_drift),
        &receipt_text(),
        SOURCE_COMMIT,
    )
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
        validate_site_owner_map_toml_syntax_v1(&text, &receipt_text(), SOURCE_COMMIT).unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapInvalid { .. }
    ));

    let mut foreign = valid_map();
    foreign.sites[0].path = "src/foreign.rs".into();
    let error =
        validate_site_owner_map_toml_syntax_v1(&map_text(&foreign), &receipt_text(), SOURCE_COMMIT)
            .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapCoverageDrift { .. }
    ));
}

#[test]
fn pinned_reference_validator_resolves_full_revision_blob_and_range() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let reference = format!("tools/checks/rust_source_topology/src/lib.rs#range:1-1@{REVIEW_HEAD}");
    let validated = validate_site_owner_map_toml_with_references_v1(
        &map_with_reference(&reference),
        &receipt_text(),
        SOURCE_COMMIT,
        SiteOwnerMapReferenceContextV1 {
            repository_root: &workspace,
            review_head: REVIEW_HEAD,
        },
    )
    .unwrap();
    assert_eq!(validated.sites.len(), 185);
}

#[test]
fn pinned_reference_validator_resolves_qualified_impl_symbol() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let reference = format!(
        "tools/checks/rust_source_topology/src/chronic_measurement/error.rs#symbol:ChronicScanErrorV1::fmt@{REVIEW_HEAD}"
    );
    let validated = validate_site_owner_map_toml_with_references_v1(
        &map_with_reference(&reference),
        &receipt_text(),
        SOURCE_COMMIT,
        SiteOwnerMapReferenceContextV1 {
            repository_root: &workspace,
            review_head: REVIEW_HEAD,
        },
    )
    .unwrap();
    assert_eq!(validated.sites.len(), 185);
}

#[test]
fn pinned_reference_validator_rejects_tree_range_and_missing_symbol() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let tree_reference = format!(
        "tools/checks/rust_source_topology/src/chronic_measurement#range:1-1@{REVIEW_HEAD}"
    );
    let error = validate_site_owner_map_toml_with_references_v1(
        &map_with_reference(&tree_reference),
        &receipt_text(),
        SOURCE_COMMIT,
        SiteOwnerMapReferenceContextV1 {
            repository_root: &workspace,
            review_head: REVIEW_HEAD,
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapReference {
            failure: SiteOwnerMapReferenceFailureV1::PathNotBlob,
            ..
        }
    ));

    let symbol_reference = format!(
        "tools/checks/rust_source_topology/src/lib.rs#symbol:MissingA1Symbol@{REVIEW_HEAD}"
    );
    let error = validate_site_owner_map_toml_with_references_v1(
        &map_with_reference(&symbol_reference),
        &receipt_text(),
        SOURCE_COMMIT,
        SiteOwnerMapReferenceContextV1 {
            repository_root: &workspace,
            review_head: REVIEW_HEAD,
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapReference {
            failure: SiteOwnerMapReferenceFailureV1::SymbolMissing,
            ..
        }
    ));
}

#[test]
fn pinned_reference_validator_rejects_range_edge_and_invalid_review_head() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let range_reference =
        format!("tools/checks/rust_source_topology/src/lib.rs#range:9999-10000@{REVIEW_HEAD}");
    let error = validate_site_owner_map_toml_with_references_v1(
        &map_with_reference(&range_reference),
        &receipt_text(),
        SOURCE_COMMIT,
        SiteOwnerMapReferenceContextV1 {
            repository_root: &workspace,
            review_head: REVIEW_HEAD,
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapReference {
            failure: SiteOwnerMapReferenceFailureV1::RangeOutOfBounds,
            ..
        }
    ));

    let edge_reference =
        format!("tools/checks/rust_source_topology/src/lib.rs#edge:unsupported@{REVIEW_HEAD}");
    let error = validate_site_owner_map_toml_with_references_v1(
        &map_with_reference(&edge_reference),
        &receipt_text(),
        SOURCE_COMMIT,
        SiteOwnerMapReferenceContextV1 {
            repository_root: &workspace,
            review_head: REVIEW_HEAD,
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapReference {
            failure: SiteOwnerMapReferenceFailureV1::EdgeUnsupported,
            ..
        }
    ));

    let invalid_review_head = "A".repeat(40);
    let valid_reference =
        format!("tools/checks/rust_source_topology/src/lib.rs#range:1-1@{REVIEW_HEAD}");
    let error = validate_site_owner_map_toml_with_references_v1(
        &map_with_reference(&valid_reference),
        &receipt_text(),
        SOURCE_COMMIT,
        SiteOwnerMapReferenceContextV1 {
            repository_root: &workspace,
            review_head: &invalid_review_head,
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::SiteOwnerMapReference {
            failure: SiteOwnerMapReferenceFailureV1::InvalidRevision,
            ..
        }
    ));
}

#[test]
fn site_owner_map_hash_matches_independent_golden_vector() {
    #[derive(Deserialize)]
    struct Golden {
        schema: String,
        canonical_json_hex: String,
        sha256: String,
    }

    let golden: Golden = toml::from_str(include_str!(
        "../../../tests/fixtures/chronic_site_owner_map_hash_golden_v1.toml"
    ))
    .unwrap();
    assert_eq!(golden.schema, "chronic-site-owner-map-hash-golden-v1");
    let map = SiteOwnerMapHashInput {
        schema: "s".into(),
        schema_version: 1,
        map_id: "m".into(),
        scanner_version: "v".into(),
        scope_id: "i".into(),
        scope_manifest_hash: "h".into(),
        source_scope_hash: "h".into(),
        source_commit: "0".repeat(40),
        scanner_evidence_hash: "h".into(),
        observation_receipt_id: "r".into(),
        observation_receipt_hash: "h".into(),
        sites: Vec::new(),
    };
    let bytes = serde_json::to_vec(&map).unwrap();
    let expected_bytes = decode_hex(&golden.canonical_json_hex);
    assert_eq!(bytes, expected_bytes);
    assert_eq!(
        format!("sha256:{:x}", Sha256::digest(&bytes)),
        golden.sha256
    );
}

fn decode_hex(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = (pair[0] as char).to_digit(16).unwrap();
            let low = (pair[1] as char).to_digit(16).unwrap();
            ((high << 4) | low) as u8
        })
        .collect()
}
