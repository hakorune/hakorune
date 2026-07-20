use std::collections::BTreeSet;

use super::error::ProfileValidationErrorV1;
use super::model::{
    AmbientRustflagsPolicyV1, BuildProfileRequestDocumentV1, BuildProfileRequestV1,
    CargoCompileModeV1, RustCargoTopologyProfileSchemaV1, ValidatedBuildProfileInputV1,
    RUST_CARGO_TOPOLOGY_PROFILE_SCHEMA_V1,
};

pub fn parse_and_verify_profile_schema_v1(
    input: &str,
) -> Result<RustCargoTopologyProfileSchemaV1, ProfileValidationErrorV1> {
    let document =
        serde_json::from_str::<BuildProfileRequestDocumentV1>(input).map_err(|error| {
            ProfileValidationErrorV1::Json {
                detail: error.to_string(),
            }
        })?;
    if document.schema != RUST_CARGO_TOPOLOGY_PROFILE_SCHEMA_V1 {
        return Err(ProfileValidationErrorV1::WrongSchema {
            actual: document.schema,
        });
    }
    verify_profile_requests_v1(document.profiles)
}

pub fn verify_profile_requests_v1(
    requests: Vec<BuildProfileRequestV1>,
) -> Result<RustCargoTopologyProfileSchemaV1, ProfileValidationErrorV1> {
    if requests.is_empty() {
        return Err(ProfileValidationErrorV1::EmptyProfiles);
    }
    let mut profile_ids = BTreeSet::new();
    let mut profiles = Vec::with_capacity(requests.len());
    for request in requests {
        require_nonempty(&request.profile_id, &request.profile_id, "profile_id")?;
        if !profile_ids.insert(request.profile_id.clone()) {
            return Err(ProfileValidationErrorV1::DuplicateProfileId {
                profile_id: request.profile_id,
            });
        }
        require_nonempty(&request.profile_id, &request.package_name, "package_name")?;
        require_nonempty(&request.profile_id, &request.target_name, "target_name")?;
        require_nonempty(&request.profile_id, &request.target_triple, "target_triple")?;
        require_nonempty(
            &request.profile_id,
            &request.panic_strategy,
            "panic_strategy",
        )?;
        require_test_mode(&request)?;
        require_ambient_rustflags(&request)?;
        let requested_features =
            normalize_features(&request.profile_id, request.requested_features)?;
        let expected_activated_root_features = normalize_features(
            &request.profile_id,
            request.expected_activated_root_features,
        )?;
        profiles.push(ValidatedBuildProfileInputV1 {
            profile_id: request.profile_id,
            package_name: request.package_name,
            target_name: request.target_name,
            target_kind: request.target_kind,
            target_triple: request.target_triple,
            cargo_profile: request.cargo_profile,
            compile_mode: request.compile_mode,
            requested_features: requested_features.into_boxed_slice(),
            expected_activated_root_features: expected_activated_root_features.into_boxed_slice(),
            default_features_enabled: request.default_features_enabled,
            test_cfg: request.test_cfg,
            debug_assertions: request.debug_assertions,
            panic_strategy: request.panic_strategy,
            ambient_rustflags: request.ambient_rustflags,
        });
    }
    profiles.sort_by(|left, right| left.profile_id.cmp(&right.profile_id));
    Ok(RustCargoTopologyProfileSchemaV1 {
        schema: RUST_CARGO_TOPOLOGY_PROFILE_SCHEMA_V1,
        schema_version: 1,
        profiles: profiles.into_boxed_slice(),
    })
}

fn require_nonempty(
    profile_id: &str,
    value: &str,
    field: &'static str,
) -> Result<(), ProfileValidationErrorV1> {
    if value.trim().is_empty() {
        return Err(ProfileValidationErrorV1::EmptyField {
            profile_id: profile_id.to_string(),
            field,
        });
    }
    Ok(())
}

fn require_test_mode(request: &BuildProfileRequestV1) -> Result<(), ProfileValidationErrorV1> {
    let expected_test_cfg = match request.compile_mode {
        CargoCompileModeV1::Normal => false,
        CargoCompileModeV1::UnitTestHarness | CargoCompileModeV1::IntegrationTestTarget => true,
    };
    if request.test_cfg != expected_test_cfg {
        return Err(ProfileValidationErrorV1::TestCompileModeMismatch {
            profile_id: request.profile_id.clone(),
        });
    }
    Ok(())
}

fn require_ambient_rustflags(
    request: &BuildProfileRequestV1,
) -> Result<(), ProfileValidationErrorV1> {
    match &request.ambient_rustflags {
        AmbientRustflagsPolicyV1::SanitizedEmpty => Ok(()),
        AmbientRustflagsPolicyV1::ExactNoCfg {
            rustflags_digest,
            cargo_encoded_rustflags_digest,
        } if !rustflags_digest.is_empty() && !cargo_encoded_rustflags_digest.is_empty() => Ok(()),
        AmbientRustflagsPolicyV1::ExactNoCfg { .. }
        | AmbientRustflagsPolicyV1::FingerprintOnlyUnknown { .. } => {
            Err(ProfileValidationErrorV1::UnsealedAmbientRustflags {
                profile_id: request.profile_id.clone(),
            })
        }
    }
}

fn normalize_features(
    profile_id: &str,
    features: Vec<String>,
) -> Result<Vec<String>, ProfileValidationErrorV1> {
    let mut seen = BTreeSet::new();
    for feature in &features {
        if feature.trim().is_empty() {
            return Err(ProfileValidationErrorV1::EmptyField {
                profile_id: profile_id.to_string(),
                field: "feature",
            });
        }
        if !seen.insert(feature.clone()) {
            return Err(ProfileValidationErrorV1::DuplicateFeature {
                profile_id: profile_id.to_string(),
                feature: feature.clone(),
            });
        }
    }
    Ok(seen.into_iter().collect())
}
