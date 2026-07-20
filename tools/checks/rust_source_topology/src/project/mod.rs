pub mod cargo;
mod cfg_eval;
mod cfg_stream;
mod error;
mod fingerprint;
mod inner_cfg_surface;
mod model;
mod modules;
mod process_error;
mod profiles;
mod rustc_cfg;

pub use cfg_eval::{cfg_environment_from_declared_unit_evidence_v1, decide_cfg_rows_v1};
pub use cfg_stream::{decide_cfg_attribute_stream_v1, CfgAttributeStreamErrorV1};
pub use error::{CfgDecisionErrorV1, ProfileValidationErrorV1};
pub use fingerprint::{
    collect_workspace_input_fingerprints_v1, CargoConfigFingerprintV1, FileFingerprintV1,
    WorkspaceInputFingerprintsV1,
};
pub use inner_cfg_surface::{
    collect_file_inner_topology_attribute_surface_v1, InnerTopologyAttributeSurfaceErrorV1,
};
pub use model::{
    AmbientRustflagsPolicyV1, BuildProfileRequestV1, CargoCompileModeV1, CargoProfileNameV1,
    CargoTargetKindV1, CfgAttributeConditionDecisionV1, CfgAttributeNestedDecisionV1,
    CfgAttributeNestedDispositionV1, CfgAttributeStreamDecisionV1, CfgAttributeStreamInputRowV1,
    CfgAttributeStreamRowDecisionV1, CfgAttributeStreamRowDispositionV1, CfgDecisionStateV1,
    CfgDecisionV1, CfgEvaluationEnvironmentV1, CfgRowDecisionV1,
    FileInnerTopologyAttributeSurfaceV1, RustCargoTopologyProfileSchemaV1,
    ValidatedBuildProfileInputV1, RUST_CARGO_TOPOLOGY_PROFILE_SCHEMA_V1,
};
pub use modules::{
    collect_declared_module_topology_v1, DeclaredIncludeEdgeV1, DeclaredModuleEdgeV1,
    DeclaredModuleInstanceV1, DeclaredModuleTopologyV1, ModuleEdgeKindV1, ModuleInstanceKindV1,
    ModuleSourceObservationV1, ModuleTopologyErrorV1,
};
pub use process_error::CargoProcessEvidenceErrorV1;
pub use profiles::{parse_and_verify_profile_schema_v1, verify_profile_requests_v1};
pub use rustc_cfg::{collect_rustc_cfg_probe_v1, RustcCfgProbeEvidenceV1};
