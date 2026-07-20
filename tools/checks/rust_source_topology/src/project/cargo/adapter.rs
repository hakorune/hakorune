use std::collections::BTreeSet;
use std::path::Path;

use crate::project::{
    CargoCompileModeV1, CargoProfileNameV1, CargoTargetKindV1, ValidatedBuildProfileInputV1,
};

use super::error::CargoEvidenceErrorV1;
use super::model::{
    package_evidence, target_evidence, CargoDeclaredCompileUnitEvidenceV1, CargoEvidencePartsV1,
    CargoMetadataSnapshotV1, CargoPackageSnapshotV1, CargoResolveNodeSnapshotV1,
    CargoTargetSnapshotV1,
};

pub fn seal_declared_cargo_unit_v1(
    snapshot: &CargoMetadataSnapshotV1,
    selected_manifest_path: &str,
    profile: &ValidatedBuildProfileInputV1,
) -> Result<CargoDeclaredCompileUnitEvidenceV1, CargoEvidenceErrorV1> {
    let manifest_relative = workspace_relative(
        &snapshot.workspace_root,
        selected_manifest_path,
        CargoEvidenceErrorV1::SelectedManifestOutsideWorkspace,
    )?;
    let package = select_package(snapshot, selected_manifest_path)?;
    if package.name != profile.package_name {
        return Err(CargoEvidenceErrorV1::PackageNameMismatch {
            expected: profile.package_name.clone(),
            actual: package.name.clone(),
        });
    }
    if package.source_observation.is_some() {
        return Err(CargoEvidenceErrorV1::SelectedPackageHasExternalSource {
            package_name: package.name.clone(),
        });
    }
    let target = select_target(package, profile)?;
    let target_source_relative = workspace_relative(
        &snapshot.workspace_root,
        &target.src_path,
        CargoEvidenceErrorV1::TargetSourceOutsideWorkspace {
            target_name: target.name.clone(),
        },
    )?;
    require_compile_mode(profile, target)?;

    let declared_features = exact_set("declared_features", &package.declared_features)?;
    let requested_features = exact_set("requested_features", &profile.requested_features)?;
    for feature in &requested_features {
        if !declared_features.contains(feature) {
            return Err(CargoEvidenceErrorV1::RequestedFeatureUnknown {
                feature: feature.clone(),
            });
        }
    }

    let resolve = select_resolve_node(snapshot, package)?;
    let resolved_features = exact_set("activated_features", &resolve.activated_features)?;
    for feature in &requested_features {
        if !resolved_features.contains(feature) {
            return Err(CargoEvidenceErrorV1::RequestedFeatureInactive {
                feature: feature.clone(),
            });
        }
    }
    require_default_feature_disposition(profile, &declared_features, &resolved_features)?;

    let expected_features = exact_set(
        "expected_activated_root_features",
        &profile.expected_activated_root_features,
    )?;
    if expected_features != resolved_features {
        return Err(CargoEvidenceErrorV1::ActivatedFeatureMismatch {
            expected: expected_features.into_iter().collect(),
            actual: resolved_features.into_iter().collect(),
        });
    }

    let required_features = exact_set("target_required_features", &target.required_features)?;
    for feature in &required_features {
        if !resolved_features.contains(feature) {
            return Err(CargoEvidenceErrorV1::RequiredFeatureInactive {
                target_name: target.name.clone(),
                feature: feature.clone(),
            });
        }
    }

    let cargo_kinds = exact_set("target_kinds", &target.cargo_kinds)?;
    let crate_types = exact_set("target_crate_types", &target.crate_types)?;
    let package_key = format!(
        "{}::{}@{}",
        manifest_relative, package.name, package.version
    );
    let target_key = format!(
        "{}::{}:{}",
        package_key,
        semantic_kind_name(profile.target_kind),
        target.name
    );

    Ok(CargoEvidencePartsV1 {
        profile_id: profile.profile_id.clone(),
        package: package_evidence(
            package_key,
            manifest_relative,
            package.name.clone(),
            package.version.clone(),
        ),
        target: target_evidence(
            target_key,
            target.name.clone(),
            profile.target_kind,
            cargo_kinds
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            crate_types
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            target_source_relative,
            target.edition.clone(),
            required_features
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            target.test,
            target.doctest,
        ),
        requested_target_triple: profile.target_triple.clone(),
        requested_cargo_profile: profile.cargo_profile,
        requested_compile_mode: profile.compile_mode,
        requested_features: requested_features
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        default_features_enabled: profile.default_features_enabled,
        profile_expected_root_features: expected_features
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        cargo_resolved_root_features: resolved_features
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        requested_test_cfg: profile.test_cfg,
        requested_debug_assertions: profile.debug_assertions,
        requested_panic_strategy: profile.panic_strategy.clone(),
    }
    .into())
}

fn select_package<'a>(
    snapshot: &'a CargoMetadataSnapshotV1,
    selected_manifest_path: &str,
) -> Result<&'a CargoPackageSnapshotV1, CargoEvidenceErrorV1> {
    let workspace_members = exact_set(
        "workspace_member_package_ids",
        &snapshot.workspace_member_package_ids,
    )?;
    let matches = snapshot
        .packages
        .iter()
        .filter(|package| package.manifest_path == selected_manifest_path)
        .collect::<Vec<_>>();
    let package = match matches.as_slice() {
        [] => return Err(CargoEvidenceErrorV1::PackageForManifestMissing),
        [package] => *package,
        _ => return Err(CargoEvidenceErrorV1::PackageForManifestAmbiguous),
    };
    if !workspace_members.contains(&package.cargo_package_id_observation) {
        return Err(CargoEvidenceErrorV1::PackageNotWorkspaceMember {
            package_name: package.name.clone(),
        });
    }
    Ok(package)
}

fn select_target<'a>(
    package: &'a CargoPackageSnapshotV1,
    profile: &ValidatedBuildProfileInputV1,
) -> Result<&'a CargoTargetSnapshotV1, CargoEvidenceErrorV1> {
    let named = package
        .targets
        .iter()
        .filter(|target| target.name == profile.target_name)
        .collect::<Vec<_>>();
    if named.is_empty() {
        return Err(CargoEvidenceErrorV1::TargetMissing {
            target_name: profile.target_name.clone(),
        });
    }
    let mut matching = Vec::new();
    for target in named {
        let kind = classify_target_kind(target)?;
        if kind == profile.target_kind {
            matching.push(target);
        }
    }
    match matching.as_slice() {
        [] => Err(CargoEvidenceErrorV1::TargetKindMismatch {
            target_name: profile.target_name.clone(),
        }),
        [target] => Ok(*target),
        _ => Err(CargoEvidenceErrorV1::TargetAmbiguous {
            target_name: profile.target_name.clone(),
        }),
    }
}

fn classify_target_kind(
    target: &CargoTargetSnapshotV1,
) -> Result<CargoTargetKindV1, CargoEvidenceErrorV1> {
    let mut kinds = BTreeSet::new();
    for raw in &target.cargo_kinds {
        let kind = match raw.as_str() {
            "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" => CargoTargetKindV1::Library,
            "bin" => CargoTargetKindV1::Binary,
            "test" => CargoTargetKindV1::IntegrationTest,
            "example" => CargoTargetKindV1::Example,
            "custom-build" => CargoTargetKindV1::BuildScript,
            "proc-macro" => CargoTargetKindV1::ProcMacro,
            kind => {
                return Err(CargoEvidenceErrorV1::UnsupportedCargoTargetKind {
                    target_name: target.name.clone(),
                    kind: kind.to_string(),
                })
            }
        };
        kinds.insert(kind);
    }
    match kinds.into_iter().collect::<Vec<_>>().as_slice() {
        [kind] => Ok(*kind),
        _ => Err(CargoEvidenceErrorV1::TargetKindMismatch {
            target_name: target.name.clone(),
        }),
    }
}

fn select_resolve_node<'a>(
    snapshot: &'a CargoMetadataSnapshotV1,
    package: &CargoPackageSnapshotV1,
) -> Result<&'a CargoResolveNodeSnapshotV1, CargoEvidenceErrorV1> {
    let nodes = snapshot
        .resolve_nodes
        .as_ref()
        .ok_or(CargoEvidenceErrorV1::ResolveMissing)?;
    let matching = nodes
        .iter()
        .filter(|node| node.cargo_package_id_observation == package.cargo_package_id_observation)
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [] => Err(CargoEvidenceErrorV1::ResolveNodeMissing {
            package_name: package.name.clone(),
        }),
        [node] => Ok(*node),
        _ => Err(CargoEvidenceErrorV1::ResolveNodeAmbiguous {
            package_name: package.name.clone(),
        }),
    }
}

fn require_default_feature_disposition(
    profile: &ValidatedBuildProfileInputV1,
    declared: &BTreeSet<String>,
    resolved: &BTreeSet<String>,
) -> Result<(), CargoEvidenceErrorV1> {
    let expected_default = profile.default_features_enabled && declared.contains("default");
    if resolved.contains("default") != expected_default {
        return Err(CargoEvidenceErrorV1::DefaultFeatureDispositionMismatch);
    }
    Ok(())
}

fn require_compile_mode(
    profile: &ValidatedBuildProfileInputV1,
    target: &CargoTargetSnapshotV1,
) -> Result<(), CargoEvidenceErrorV1> {
    let valid = match profile.compile_mode {
        CargoCompileModeV1::Normal => !profile.test_cfg,
        CargoCompileModeV1::UnitTestHarness => {
            profile.cargo_profile == CargoProfileNameV1::Test
                && profile.test_cfg
                && profile.target_kind == CargoTargetKindV1::Library
                && target.test
        }
        CargoCompileModeV1::IntegrationTestTarget => {
            profile.cargo_profile == CargoProfileNameV1::Test
                && profile.test_cfg
                && profile.target_kind == CargoTargetKindV1::IntegrationTest
                && target.test
        }
    };
    if !valid {
        return Err(CargoEvidenceErrorV1::CompileModeTargetMismatch {
            profile_id: profile.profile_id.clone(),
        });
    }
    Ok(())
}

fn exact_set(
    owner: &'static str,
    values: &[String],
) -> Result<BTreeSet<String>, CargoEvidenceErrorV1> {
    let mut result = BTreeSet::new();
    for value in values {
        if !result.insert(value.clone()) {
            return Err(CargoEvidenceErrorV1::DuplicateSnapshotValue {
                owner,
                value: value.clone(),
            });
        }
    }
    Ok(result)
}

fn workspace_relative(
    workspace_root: &str,
    path: &str,
    error: CargoEvidenceErrorV1,
) -> Result<String, CargoEvidenceErrorV1> {
    let relative = Path::new(path)
        .strip_prefix(Path::new(workspace_root))
        .map_err(|_| error.clone())?;
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(error);
    }
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn semantic_kind_name(kind: CargoTargetKindV1) -> &'static str {
    match kind {
        CargoTargetKindV1::Library => "library",
        CargoTargetKindV1::Binary => "binary",
        CargoTargetKindV1::IntegrationTest => "integration-test",
        CargoTargetKindV1::Example => "example",
        CargoTargetKindV1::BuildScript => "build-script",
        CargoTargetKindV1::ProcMacro => "proc-macro",
    }
}
