use serde::{Deserialize, Serialize};

use crate::project::{CargoCompileModeV1, CargoProfileNameV1, CargoTargetKindV1};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CargoMetadataSnapshotV1 {
    pub workspace_root: String,
    pub workspace_member_package_ids: Box<[String]>,
    pub packages: Box<[CargoPackageSnapshotV1]>,
    pub resolve_nodes: Option<Box<[CargoResolveNodeSnapshotV1]>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CargoPackageSnapshotV1 {
    pub cargo_package_id_observation: String,
    pub name: String,
    pub version: String,
    pub manifest_path: String,
    pub source_observation: Option<String>,
    pub declared_features: Box<[String]>,
    pub targets: Box<[CargoTargetSnapshotV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CargoTargetSnapshotV1 {
    pub name: String,
    pub cargo_kinds: Box<[String]>,
    pub crate_types: Box<[String]>,
    pub src_path: String,
    pub edition: String,
    pub required_features: Box<[String]>,
    pub test: bool,
    pub doctest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CargoResolveNodeSnapshotV1 {
    pub cargo_package_id_observation: String,
    pub activated_features: Box<[String]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CargoPackageDeclarationEvidenceV1 {
    package_key: String,
    manifest_path_workspace_relative: String,
    name: String,
    version: String,
}

impl CargoPackageDeclarationEvidenceV1 {
    pub fn package_key(&self) -> &str {
        &self.package_key
    }

    pub fn manifest_path_workspace_relative(&self) -> &str {
        &self.manifest_path_workspace_relative
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CargoTargetDeclarationEvidenceV1 {
    target_key: String,
    name: String,
    semantic_kind: CargoTargetKindV1,
    cargo_kinds: Box<[String]>,
    crate_types: Box<[String]>,
    src_path_workspace_relative: String,
    edition: String,
    required_features: Box<[String]>,
    test: bool,
    doctest: bool,
}

impl CargoTargetDeclarationEvidenceV1 {
    pub fn target_key(&self) -> &str {
        &self.target_key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn semantic_kind(&self) -> CargoTargetKindV1 {
        self.semantic_kind
    }

    pub fn cargo_kinds(&self) -> &[String] {
        &self.cargo_kinds
    }

    pub fn crate_types(&self) -> &[String] {
        &self.crate_types
    }

    pub fn src_path_workspace_relative(&self) -> &str {
        &self.src_path_workspace_relative
    }

    pub fn required_features(&self) -> &[String] {
        &self.required_features
    }

    pub const fn test_enabled(&self) -> bool {
        self.test
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CargoDeclaredCompileUnitEvidenceV1 {
    profile_id: String,
    package: CargoPackageDeclarationEvidenceV1,
    target: CargoTargetDeclarationEvidenceV1,
    requested_target_triple: String,
    requested_cargo_profile: CargoProfileNameV1,
    requested_compile_mode: CargoCompileModeV1,
    requested_features: Box<[String]>,
    default_features_enabled: bool,
    profile_expected_root_features: Box<[String]>,
    cargo_resolved_root_features: Box<[String]>,
    requested_test_cfg: bool,
    requested_debug_assertions: bool,
    requested_panic_strategy: String,
}

impl CargoDeclaredCompileUnitEvidenceV1 {
    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub const fn package(&self) -> &CargoPackageDeclarationEvidenceV1 {
        &self.package
    }

    pub const fn target(&self) -> &CargoTargetDeclarationEvidenceV1 {
        &self.target
    }

    pub fn cargo_resolved_root_features(&self) -> &[String] {
        &self.cargo_resolved_root_features
    }

    pub fn profile_expected_root_features(&self) -> &[String] {
        &self.profile_expected_root_features
    }

    pub const fn requested_compile_mode(&self) -> CargoCompileModeV1 {
        self.requested_compile_mode
    }

    pub const fn requested_test_cfg(&self) -> bool {
        self.requested_test_cfg
    }
}

pub(super) struct CargoEvidencePartsV1 {
    pub profile_id: String,
    pub package: CargoPackageDeclarationEvidenceV1,
    pub target: CargoTargetDeclarationEvidenceV1,
    pub requested_target_triple: String,
    pub requested_cargo_profile: CargoProfileNameV1,
    pub requested_compile_mode: CargoCompileModeV1,
    pub requested_features: Box<[String]>,
    pub default_features_enabled: bool,
    pub profile_expected_root_features: Box<[String]>,
    pub cargo_resolved_root_features: Box<[String]>,
    pub requested_test_cfg: bool,
    pub requested_debug_assertions: bool,
    pub requested_panic_strategy: String,
}

impl From<CargoEvidencePartsV1> for CargoDeclaredCompileUnitEvidenceV1 {
    fn from(parts: CargoEvidencePartsV1) -> Self {
        Self {
            profile_id: parts.profile_id,
            package: parts.package,
            target: parts.target,
            requested_target_triple: parts.requested_target_triple,
            requested_cargo_profile: parts.requested_cargo_profile,
            requested_compile_mode: parts.requested_compile_mode,
            requested_features: parts.requested_features,
            default_features_enabled: parts.default_features_enabled,
            profile_expected_root_features: parts.profile_expected_root_features,
            cargo_resolved_root_features: parts.cargo_resolved_root_features,
            requested_test_cfg: parts.requested_test_cfg,
            requested_debug_assertions: parts.requested_debug_assertions,
            requested_panic_strategy: parts.requested_panic_strategy,
        }
    }
}

pub(super) fn package_evidence(
    package_key: String,
    manifest_path_workspace_relative: String,
    name: String,
    version: String,
) -> CargoPackageDeclarationEvidenceV1 {
    CargoPackageDeclarationEvidenceV1 {
        package_key,
        manifest_path_workspace_relative,
        name,
        version,
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn target_evidence(
    target_key: String,
    name: String,
    semantic_kind: CargoTargetKindV1,
    cargo_kinds: Box<[String]>,
    crate_types: Box<[String]>,
    src_path_workspace_relative: String,
    edition: String,
    required_features: Box<[String]>,
    test: bool,
    doctest: bool,
) -> CargoTargetDeclarationEvidenceV1 {
    CargoTargetDeclarationEvidenceV1 {
        target_key,
        name,
        semantic_kind,
        cargo_kinds,
        crate_types,
        src_path_workspace_relative,
        edition,
        required_features,
        test,
        doctest,
    }
}
