use std::fs;
use std::path::Path;
use std::process::Command;

use cargo_metadata::{CargoOpt, Metadata, MetadataCommand};

use crate::project::{
    AmbientRustflagsPolicyV1, CargoProcessEvidenceErrorV1, ValidatedBuildProfileInputV1,
};

use super::model::{
    CargoMetadataSnapshotV1, CargoPackageSnapshotV1, CargoResolveNodeSnapshotV1,
    CargoTargetSnapshotV1,
};
use super::process_model::{invocation_evidence, process_evidence, CargoMetadataProcessEvidenceV1};
use crate::project::fingerprint::sha256_bytes;

pub fn collect_cargo_metadata_process_evidence_v1(
    manifest_path: &Path,
    profile: &ValidatedBuildProfileInputV1,
) -> Result<CargoMetadataProcessEvidenceV1, CargoProcessEvidenceErrorV1> {
    require_sanitized_ambient_policy(profile)?;
    let manifest = fs::canonicalize(manifest_path).map_err(|error| {
        CargoProcessEvidenceErrorV1::ManifestCanonicalizationFailed {
            detail: error.kind().to_string(),
        }
    })?;
    let current_dir = manifest
        .parent()
        .ok_or(CargoProcessEvidenceErrorV1::ManifestHasNoParent)?;
    let cargo_version = run_cargo_version()?;

    let mut command = MetadataCommand::new();
    command
        .cargo_path("cargo")
        .manifest_path(&manifest)
        .current_dir(current_dir)
        .other_options(vec![
            "--locked".to_string(),
            "--offline".to_string(),
            "--filter-platform".to_string(),
            profile.target_triple.clone(),
        ]);
    if !profile.default_features_enabled {
        command.features(CargoOpt::NoDefaultFeatures);
    }
    if !profile.requested_features.is_empty() {
        command.features(CargoOpt::SomeFeatures(
            profile
                .requested_features
                .iter()
                .map(|feature| format!("{}/{}", profile.package_name, feature))
                .collect(),
        ));
    }
    sanitize_metadata_environment(&mut command);
    let metadata =
        command
            .exec()
            .map_err(|error| CargoProcessEvidenceErrorV1::CargoMetadataFailed {
                detail: stable_process_detail(&error.to_string()),
            })?;
    let cargo_version_after = run_cargo_version()?;
    if cargo_version != cargo_version_after {
        return Err(CargoProcessEvidenceErrorV1::CargoExecutableChangedDuringObservation);
    }
    let snapshot = snapshot_metadata(metadata);
    let invocation = invocation_evidence(
        sha256_bytes(&cargo_version),
        profile.target_triple.clone(),
        profile.requested_features.clone(),
        profile.default_features_enabled,
    );
    Ok(process_evidence(
        snapshot,
        invocation,
        manifest.to_string_lossy().to_string(),
    ))
}

fn require_sanitized_ambient_policy(
    profile: &ValidatedBuildProfileInputV1,
) -> Result<(), CargoProcessEvidenceErrorV1> {
    if !matches!(
        profile.ambient_rustflags,
        AmbientRustflagsPolicyV1::SanitizedEmpty
    ) {
        return Err(
            CargoProcessEvidenceErrorV1::UnsupportedAmbientRustflagsPolicy {
                profile_id: profile.profile_id.clone(),
            },
        );
    }
    Ok(())
}

fn sanitize_metadata_environment(command: &mut MetadataCommand) {
    const EXACT_KEYS: &[&str] = &[
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_BUILD_TARGET",
        "CARGO_BUILD_RUSTC",
        "RUSTC",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
    ];
    for key in EXACT_KEYS {
        command.env_remove(key);
    }
    for (key, _) in std::env::vars_os() {
        let text = key.to_string_lossy();
        if text.starts_with("CARGO_PROFILE_")
            || (text.starts_with("CARGO_TARGET_") && text.ends_with("_RUSTFLAGS"))
        {
            command.env_remove(key);
        }
    }
}

fn run_cargo_version() -> Result<Vec<u8>, CargoProcessEvidenceErrorV1> {
    let output = Command::new("cargo").arg("-vV").output().map_err(|error| {
        CargoProcessEvidenceErrorV1::CargoVersionFailed {
            detail: error.kind().to_string(),
        }
    })?;
    if !output.status.success() {
        return Err(CargoProcessEvidenceErrorV1::CargoVersionFailed {
            detail: format!("status={}", output.status),
        });
    }
    Ok(output.stdout)
}

fn snapshot_metadata(metadata: Metadata) -> CargoMetadataSnapshotV1 {
    let mut workspace_member_package_ids = metadata
        .workspace_members
        .iter()
        .map(|id| id.repr.clone())
        .collect::<Vec<_>>();
    workspace_member_package_ids.sort();

    let mut packages = metadata
        .packages
        .into_iter()
        .map(|package| {
            let mut declared_features = package.features.into_keys().collect::<Vec<_>>();
            declared_features.sort();
            let mut targets = package
                .targets
                .into_iter()
                .map(|target| {
                    let mut cargo_kinds = target
                        .kind
                        .into_iter()
                        .map(|kind| kind.to_string())
                        .collect::<Vec<_>>();
                    cargo_kinds.sort();
                    let mut crate_types = target
                        .crate_types
                        .into_iter()
                        .map(|kind| kind.to_string())
                        .collect::<Vec<_>>();
                    crate_types.sort();
                    let mut required_features = target.required_features;
                    required_features.sort();
                    CargoTargetSnapshotV1 {
                        name: target.name,
                        cargo_kinds: cargo_kinds.into_boxed_slice(),
                        crate_types: crate_types.into_boxed_slice(),
                        src_path: target.src_path.to_string(),
                        edition: target.edition.to_string(),
                        required_features: required_features.into_boxed_slice(),
                        test: target.test,
                        doctest: target.doctest,
                    }
                })
                .collect::<Vec<_>>();
            targets.sort_by(|left, right| {
                (&left.name, &left.cargo_kinds, &left.src_path).cmp(&(
                    &right.name,
                    &right.cargo_kinds,
                    &right.src_path,
                ))
            });
            CargoPackageSnapshotV1 {
                cargo_package_id_observation: package.id.repr,
                name: package.name.to_string(),
                version: package.version.to_string(),
                manifest_path: package.manifest_path.to_string(),
                source_observation: package.source.map(|source| source.to_string()),
                declared_features: declared_features.into_boxed_slice(),
                targets: targets.into_boxed_slice(),
            }
        })
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| {
        (&left.manifest_path, &left.name, &left.version).cmp(&(
            &right.manifest_path,
            &right.name,
            &right.version,
        ))
    });

    let resolve_nodes = metadata.resolve.map(|resolve| {
        let mut nodes = resolve
            .nodes
            .into_iter()
            .map(|node| {
                let mut activated_features = node
                    .features
                    .into_iter()
                    .map(|feature| feature.to_string())
                    .collect::<Vec<_>>();
                activated_features.sort();
                CargoResolveNodeSnapshotV1 {
                    cargo_package_id_observation: node.id.repr,
                    activated_features: activated_features.into_boxed_slice(),
                }
            })
            .collect::<Vec<_>>();
        nodes.sort_by(|left, right| {
            left.cargo_package_id_observation
                .cmp(&right.cargo_package_id_observation)
        });
        nodes.into_boxed_slice()
    });

    CargoMetadataSnapshotV1 {
        workspace_root: metadata.workspace_root.to_string(),
        workspace_member_package_ids: workspace_member_package_ids.into_boxed_slice(),
        packages: packages.into_boxed_slice(),
        resolve_nodes,
    }
}

fn stable_process_detail(detail: &str) -> String {
    detail.lines().next().unwrap_or("unknown").to_string()
}
