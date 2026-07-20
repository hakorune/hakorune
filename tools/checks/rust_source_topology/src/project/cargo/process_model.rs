use serde::Serialize;

use super::model::CargoMetadataSnapshotV1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CargoMetadataInvocationEvidenceV1 {
    cargo_version_sha256: String,
    target_triple: String,
    requested_features: Box<[String]>,
    default_features_enabled: bool,
    locked: bool,
    offline: bool,
    ambient_cfg_environment_sanitized: bool,
}

impl CargoMetadataInvocationEvidenceV1 {
    pub fn cargo_version_sha256(&self) -> &str {
        &self.cargo_version_sha256
    }

    pub fn target_triple(&self) -> &str {
        &self.target_triple
    }

    pub fn requested_features(&self) -> &[String] {
        &self.requested_features
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoMetadataProcessEvidenceV1 {
    snapshot: CargoMetadataSnapshotV1,
    invocation: CargoMetadataInvocationEvidenceV1,
    selected_manifest_path_observation: String,
}

impl CargoMetadataProcessEvidenceV1 {
    pub const fn snapshot(&self) -> &CargoMetadataSnapshotV1 {
        &self.snapshot
    }

    pub const fn invocation(&self) -> &CargoMetadataInvocationEvidenceV1 {
        &self.invocation
    }

    pub fn selected_manifest_path_observation(&self) -> &str {
        &self.selected_manifest_path_observation
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        CargoMetadataSnapshotV1,
        CargoMetadataInvocationEvidenceV1,
        String,
    ) {
        (
            self.snapshot,
            self.invocation,
            self.selected_manifest_path_observation,
        )
    }
}

pub(super) fn process_evidence(
    snapshot: CargoMetadataSnapshotV1,
    invocation: CargoMetadataInvocationEvidenceV1,
    selected_manifest_path_observation: String,
) -> CargoMetadataProcessEvidenceV1 {
    CargoMetadataProcessEvidenceV1 {
        snapshot,
        invocation,
        selected_manifest_path_observation,
    }
}

pub(super) fn invocation_evidence(
    cargo_version_sha256: String,
    target_triple: String,
    requested_features: Box<[String]>,
    default_features_enabled: bool,
) -> CargoMetadataInvocationEvidenceV1 {
    CargoMetadataInvocationEvidenceV1 {
        cargo_version_sha256,
        target_triple,
        requested_features,
        default_features_enabled,
        locked: true,
        offline: true,
        ambient_cfg_environment_sanitized: true,
    }
}
