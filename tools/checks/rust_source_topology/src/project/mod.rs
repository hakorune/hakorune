mod cfg_eval;
mod error;
mod model;
mod profiles;

pub use cfg_eval::decide_cfg_rows_v1;
pub use error::{CfgDecisionErrorV1, ProfileValidationErrorV1};
pub use model::{
    AmbientRustflagsPolicyV1, BuildProfileRequestV1, CargoCompileModeV1, CargoProfileNameV1,
    CargoTargetKindV1, CfgDecisionStateV1, CfgDecisionV1, CfgEvaluationEnvironmentV1,
    CfgRowDecisionV1, RustCargoTopologyProfileSchemaV1, ValidatedBuildProfileInputV1,
    RUST_CARGO_TOPOLOGY_PROFILE_SCHEMA_V1,
};
pub use profiles::{parse_and_verify_profile_schema_v1, verify_profile_requests_v1};
