use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::SourceRangeV1;

pub const RUST_CARGO_TOPOLOGY_PROFILE_SCHEMA_V1: &str = "rust-cargo-topology-profile-schema-v1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BuildProfileRequestDocumentV1 {
    pub schema: String,
    pub profiles: Vec<BuildProfileRequestV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BuildProfileRequestV1 {
    pub profile_id: String,
    pub package_name: String,
    pub target_name: String,
    pub target_kind: CargoTargetKindV1,
    pub target_triple: String,
    pub cargo_profile: CargoProfileNameV1,
    pub compile_mode: CargoCompileModeV1,
    pub requested_features: Vec<String>,
    pub expected_activated_root_features: Vec<String>,
    pub default_features_enabled: bool,
    pub test_cfg: bool,
    pub debug_assertions: bool,
    pub panic_strategy: String,
    pub ambient_rustflags: AmbientRustflagsPolicyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CargoTargetKindV1 {
    Library,
    Binary,
    IntegrationTest,
    Example,
    BuildScript,
    ProcMacro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CargoProfileNameV1 {
    Dev,
    Test,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CargoCompileModeV1 {
    Normal,
    UnitTestHarness,
    IntegrationTestTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AmbientRustflagsPolicyV1 {
    SanitizedEmpty,
    ExactNoCfg {
        rustflags_digest: String,
        cargo_encoded_rustflags_digest: String,
    },
    FingerprintOnlyUnknown {
        digest: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedBuildProfileInputV1 {
    pub profile_id: String,
    pub package_name: String,
    pub target_name: String,
    pub target_kind: CargoTargetKindV1,
    pub target_triple: String,
    pub cargo_profile: CargoProfileNameV1,
    pub compile_mode: CargoCompileModeV1,
    pub requested_features: Box<[String]>,
    pub expected_activated_root_features: Box<[String]>,
    pub default_features_enabled: bool,
    pub test_cfg: bool,
    pub debug_assertions: bool,
    pub panic_strategy: String,
    pub ambient_rustflags: AmbientRustflagsPolicyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RustCargoTopologyProfileSchemaV1 {
    pub schema: &'static str,
    pub schema_version: u32,
    pub profiles: Box<[ValidatedBuildProfileInputV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgEvaluationEnvironmentV1 {
    pub profile_id: String,
    pub target_kind: CargoTargetKindV1,
    pub target_triple: String,
    pub activated_features: BTreeSet<String>,
    pub test_cfg: bool,
    pub debug_assertions: bool,
    pub target_features: BTreeSet<String>,
    pub target_features_sealed: bool,
    pub target_predicates_sealed: bool,
    pub known_flags: BTreeMap<String, bool>,
    pub known_key_values: BTreeMap<String, BTreeSet<String>>,
}

impl CfgEvaluationEnvironmentV1 {
    pub fn from_profile_input(profile: &ValidatedBuildProfileInputV1) -> Self {
        Self {
            profile_id: profile.profile_id.clone(),
            target_kind: profile.target_kind,
            target_triple: profile.target_triple.clone(),
            activated_features: profile
                .expected_activated_root_features
                .iter()
                .cloned()
                .collect(),
            test_cfg: profile.test_cfg,
            debug_assertions: profile.debug_assertions,
            target_features: BTreeSet::new(),
            target_features_sealed: false,
            target_predicates_sealed: false,
            known_flags: BTreeMap::new(),
            known_key_values: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CfgDecisionStateV1 {
    Included,
    Excluded,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CfgRowDecisionV1 {
    pub syntax: String,
    pub state: CfgDecisionStateV1,
    pub unknown_predicates: Box<[String]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CfgDecisionV1 {
    pub profile_id: String,
    pub state: CfgDecisionStateV1,
    pub rows: Box<[CfgRowDecisionV1]>,
}

/// One exact source-order attribute input to the bounded CFG stream.
///
/// This is intentionally richer than the legacy `Vec<String>` cfg facade:
/// CFGSTREAM0 needs the original ordinal and half-open source range so a later
/// topology consumer cannot pair a decision with a different attribute row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CfgAttributeStreamInputRowV1 {
    pub source_ordinal: u32,
    pub source_range: SourceRangeV1,
    pub syntax: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CfgAttributeStreamRowDispositionV1 {
    Evaluated,
    TopologyNeutral,
    NotReachedAfterExclusion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CfgAttributeNestedDispositionV1 {
    Evaluated,
    TopologyNeutral,
    NotEvaluatedInactiveCfgAttr,
    NotReachedAfterExclusion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CfgAttributeConditionDecisionV1 {
    pub syntax: String,
    pub state: CfgDecisionStateV1,
    pub unknown_predicates: Box<[String]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CfgAttributeNestedDecisionV1 {
    pub syntax: String,
    pub disposition: CfgAttributeNestedDispositionV1,
    pub state: Option<CfgDecisionStateV1>,
    pub unknown_predicates: Box<[String]>,
    pub nested: Box<[CfgAttributeNestedDecisionV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CfgAttributeStreamRowDecisionV1 {
    pub input: CfgAttributeStreamInputRowV1,
    pub disposition: CfgAttributeStreamRowDispositionV1,
    pub state: Option<CfgDecisionStateV1>,
    pub unknown_predicates: Box<[String]>,
    pub cfg_attr_condition: Option<CfgAttributeConditionDecisionV1>,
    pub nested: Box<[CfgAttributeNestedDecisionV1]>,
}

/// One source-order, short-circuiting CFG / cfg_attr decision.
///
/// `decisive_row_ordinal` identifies the first terminal Excluded or Unknown
/// row. Exclusion preserves later input rows as `NotReachedAfterExclusion`;
/// Unknown returns immediately because topology consumers must reject it rather
/// than let a later false predicate erase the unknown fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CfgAttributeStreamDecisionV1 {
    pub profile_id: String,
    pub final_state: CfgDecisionStateV1,
    pub decisive_row_ordinal: Option<u32>,
    pub rows: Box<[CfgAttributeStreamRowDecisionV1]>,
}
