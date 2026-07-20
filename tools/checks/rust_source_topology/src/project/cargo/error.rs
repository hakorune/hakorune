use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CargoEvidenceErrorV1 {
    SelectedManifestOutsideWorkspace,
    PackageForManifestMissing,
    PackageForManifestAmbiguous,
    PackageNotWorkspaceMember {
        package_name: String,
    },
    PackageNameMismatch {
        expected: String,
        actual: String,
    },
    SelectedPackageHasExternalSource {
        package_name: String,
    },
    TargetMissing {
        target_name: String,
    },
    TargetAmbiguous {
        target_name: String,
    },
    TargetKindMismatch {
        target_name: String,
    },
    UnsupportedCargoTargetKind {
        target_name: String,
        kind: String,
    },
    TargetSourceOutsideWorkspace {
        target_name: String,
    },
    ResolveMissing,
    ResolveNodeMissing {
        package_name: String,
    },
    ResolveNodeAmbiguous {
        package_name: String,
    },
    DuplicateSnapshotValue {
        owner: &'static str,
        value: String,
    },
    RequestedFeatureUnknown {
        feature: String,
    },
    RequestedFeatureInactive {
        feature: String,
    },
    DefaultFeatureDispositionMismatch,
    ActivatedFeatureMismatch {
        expected: Vec<String>,
        actual: Vec<String>,
    },
    RequiredFeatureInactive {
        target_name: String,
        feature: String,
    },
    CompileModeTargetMismatch {
        profile_id: String,
    },
}

impl fmt::Display for CargoEvidenceErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CargoEvidenceErrorV1 as Error;
        match self {
            Error::SelectedManifestOutsideWorkspace => write!(
                formatter,
                "[rust-source-topology/cargo/manifest-outside-workspace]"
            ),
            Error::PackageForManifestMissing => write!(
                formatter,
                "[rust-source-topology/cargo/package-for-manifest-missing]"
            ),
            Error::PackageForManifestAmbiguous => write!(
                formatter,
                "[rust-source-topology/cargo/package-for-manifest-ambiguous]"
            ),
            Error::PackageNotWorkspaceMember { package_name } => write!(
                formatter,
                "[rust-source-topology/cargo/package-not-workspace-member] package={package_name}"
            ),
            Error::PackageNameMismatch { expected, actual } => write!(
                formatter,
                "[rust-source-topology/cargo/package-name-mismatch] expected={expected} actual={actual}"
            ),
            Error::SelectedPackageHasExternalSource { package_name } => write!(
                formatter,
                "[rust-source-topology/cargo/selected-package-external] package={package_name}"
            ),
            Error::TargetMissing { target_name } => write!(
                formatter,
                "[rust-source-topology/cargo/target-missing] target={target_name}"
            ),
            Error::TargetAmbiguous { target_name } => write!(
                formatter,
                "[rust-source-topology/cargo/target-ambiguous] target={target_name}"
            ),
            Error::TargetKindMismatch { target_name } => write!(
                formatter,
                "[rust-source-topology/cargo/target-kind-mismatch] target={target_name}"
            ),
            Error::UnsupportedCargoTargetKind { target_name, kind } => write!(
                formatter,
                "[rust-source-topology/cargo/target-kind-unsupported] target={target_name} kind={kind}"
            ),
            Error::TargetSourceOutsideWorkspace { target_name } => write!(
                formatter,
                "[rust-source-topology/cargo/target-source-outside-workspace] target={target_name}"
            ),
            Error::ResolveMissing => write!(
                formatter,
                "[rust-source-topology/cargo/resolve-missing]"
            ),
            Error::ResolveNodeMissing { package_name } => write!(
                formatter,
                "[rust-source-topology/cargo/resolve-node-missing] package={package_name}"
            ),
            Error::ResolveNodeAmbiguous { package_name } => write!(
                formatter,
                "[rust-source-topology/cargo/resolve-node-ambiguous] package={package_name}"
            ),
            Error::DuplicateSnapshotValue { owner, value } => write!(
                formatter,
                "[rust-source-topology/cargo/duplicate-snapshot-value] owner={owner} value={value}"
            ),
            Error::RequestedFeatureUnknown { feature } => write!(
                formatter,
                "[rust-source-topology/cargo/requested-feature-unknown] feature={feature}"
            ),
            Error::RequestedFeatureInactive { feature } => write!(
                formatter,
                "[rust-source-topology/cargo/requested-feature-inactive] feature={feature}"
            ),
            Error::DefaultFeatureDispositionMismatch => write!(
                formatter,
                "[rust-source-topology/cargo/default-feature-disposition-mismatch]"
            ),
            Error::ActivatedFeatureMismatch { expected, actual } => write!(
                formatter,
                "[rust-source-topology/cargo/activated-feature-mismatch] expected={expected:?} actual={actual:?}"
            ),
            Error::RequiredFeatureInactive {
                target_name,
                feature,
            } => write!(
                formatter,
                "[rust-source-topology/cargo/required-feature-inactive] target={target_name} feature={feature}"
            ),
            Error::CompileModeTargetMismatch { profile_id } => write!(
                formatter,
                "[rust-source-topology/cargo/compile-mode-target-mismatch] profile={profile_id}"
            ),
        }
    }
}

impl std::error::Error for CargoEvidenceErrorV1 {}
