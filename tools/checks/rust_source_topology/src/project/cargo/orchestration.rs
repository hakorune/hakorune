use std::path::Path;

use serde::Serialize;

use crate::project::{
    collect_rustc_cfg_probe_v1, collect_workspace_input_fingerprints_v1,
    CargoProcessEvidenceErrorV1, RustcCfgProbeEvidenceV1, ValidatedBuildProfileInputV1,
    WorkspaceInputFingerprintsV1,
};

use super::adapter::seal_declared_cargo_unit_v1;
use super::command::collect_cargo_metadata_process_evidence_v1;
use super::model::CargoDeclaredCompileUnitEvidenceV1;
use super::process_model::CargoMetadataInvocationEvidenceV1;
use super::CargoEvidenceErrorV1;

#[derive(Debug)]
pub enum CargoDeclaredUnitCollectionErrorV1 {
    Process(CargoProcessEvidenceErrorV1),
    Seal(CargoEvidenceErrorV1),
}

impl std::fmt::Display for CargoDeclaredUnitCollectionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Process(error) => error.fmt(formatter),
            Self::Seal(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CargoDeclaredUnitCollectionErrorV1 {}

impl From<CargoProcessEvidenceErrorV1> for CargoDeclaredUnitCollectionErrorV1 {
    fn from(error: CargoProcessEvidenceErrorV1) -> Self {
        Self::Process(error)
    }
}

impl From<CargoEvidenceErrorV1> for CargoDeclaredUnitCollectionErrorV1 {
    fn from(error: CargoEvidenceErrorV1) -> Self {
        Self::Seal(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CargoDeclaredUnitProcessEvidenceV1 {
    declared_unit: CargoDeclaredCompileUnitEvidenceV1,
    metadata_invocation: CargoMetadataInvocationEvidenceV1,
    rustc_cfg_probe: RustcCfgProbeEvidenceV1,
    workspace_inputs: WorkspaceInputFingerprintsV1,
}

impl CargoDeclaredUnitProcessEvidenceV1 {
    pub const fn declared_unit(&self) -> &CargoDeclaredCompileUnitEvidenceV1 {
        &self.declared_unit
    }

    pub const fn metadata_invocation(&self) -> &CargoMetadataInvocationEvidenceV1 {
        &self.metadata_invocation
    }

    pub const fn rustc_cfg_probe(&self) -> &RustcCfgProbeEvidenceV1 {
        &self.rustc_cfg_probe
    }

    pub const fn workspace_inputs(&self) -> &WorkspaceInputFingerprintsV1 {
        &self.workspace_inputs
    }
}

pub fn collect_declared_cargo_unit_process_evidence_v1(
    manifest_path: &Path,
    profile: &ValidatedBuildProfileInputV1,
) -> Result<CargoDeclaredUnitProcessEvidenceV1, CargoDeclaredUnitCollectionErrorV1> {
    let discovery = collect_cargo_metadata_process_evidence_v1(manifest_path, profile)?;
    let selected_manifest = discovery.selected_manifest_path_observation().to_string();
    let workspace_root = discovery.snapshot().workspace_root.clone();
    let workspace_inputs_before = collect_workspace_input_fingerprints_v1(
        Path::new(&workspace_root),
        Path::new(&selected_manifest),
    )?;
    let metadata = collect_cargo_metadata_process_evidence_v1(manifest_path, profile)?;
    if metadata.snapshot().workspace_root != workspace_root
        || metadata.selected_manifest_path_observation() != selected_manifest
    {
        return Err(CargoProcessEvidenceErrorV1::WorkspaceInputsChangedDuringObservation.into());
    }
    let declared_unit =
        seal_declared_cargo_unit_v1(metadata.snapshot(), &selected_manifest, profile)?;
    let rustc_cfg_probe =
        collect_rustc_cfg_probe_v1(profile, declared_unit.cargo_resolved_root_features())?;
    let workspace_inputs_after = collect_workspace_input_fingerprints_v1(
        Path::new(&workspace_root),
        Path::new(&selected_manifest),
    )?;
    if workspace_inputs_before != workspace_inputs_after {
        return Err(CargoProcessEvidenceErrorV1::WorkspaceInputsChangedDuringObservation.into());
    }
    let (_, metadata_invocation, _) = metadata.into_parts();
    Ok(CargoDeclaredUnitProcessEvidenceV1 {
        declared_unit,
        metadata_invocation,
        rustc_cfg_probe,
        workspace_inputs: workspace_inputs_after,
    })
}
