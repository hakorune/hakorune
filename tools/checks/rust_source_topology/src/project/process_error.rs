use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CargoProcessEvidenceErrorV1 {
    UnsupportedAmbientRustflagsPolicy {
        profile_id: String,
    },
    ManifestHasNoParent,
    ManifestCanonicalizationFailed {
        detail: String,
    },
    CargoMetadataFailed {
        detail: String,
    },
    CargoVersionFailed {
        detail: String,
    },
    CargoExecutableChangedDuringObservation,
    RustcVersionFailed {
        detail: String,
    },
    RustcExecutableChangedDuringObservation,
    RustcCfgProbeFailed {
        profile_id: String,
        detail: String,
    },
    RustcCfgMalformed {
        line: String,
    },
    RustcCfgContractMismatch {
        profile_id: String,
        fact: &'static str,
    },
    WorkspaceRootNotDirectory,
    InputReadFailed {
        role: &'static str,
        detail: String,
    },
    InputNotFile {
        role: &'static str,
    },
    InputOutsideWorkspace,
    AmbiguousRepositoryCargoConfig,
    AmbiguousExternalCargoConfig,
    RepositoryCargoConfigNotUtf8,
    RepositoryCargoConfigInvalid {
        detail: String,
    },
    CfgAffectingRepositoryConfig {
        key: String,
    },
    UnsupportedRepositoryRustflags,
    CfgAffectingRepositoryRustflags,
    WorkspaceInputsChangedDuringObservation,
}

impl fmt::Display for CargoProcessEvidenceErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CargoProcessEvidenceErrorV1 as Error;
        match self {
            Error::UnsupportedAmbientRustflagsPolicy { profile_id } => write!(
                formatter,
                "[rust-source-topology/process/ambient-rustflags-unsupported] profile={profile_id}"
            ),
            Error::ManifestHasNoParent => write!(
                formatter,
                "[rust-source-topology/process/manifest-parent-missing]"
            ),
            Error::ManifestCanonicalizationFailed { detail } => write!(
                formatter,
                "[rust-source-topology/process/manifest-canonicalization-failed] detail={detail}"
            ),
            Error::CargoMetadataFailed { detail } => write!(
                formatter,
                "[rust-source-topology/process/cargo-metadata-failed] detail={detail}"
            ),
            Error::CargoVersionFailed { detail } => write!(
                formatter,
                "[rust-source-topology/process/cargo-version-failed] detail={detail}"
            ),
            Error::CargoExecutableChangedDuringObservation => write!(
                formatter,
                "[rust-source-topology/process/cargo-executable-changed]"
            ),
            Error::RustcVersionFailed { detail } => write!(
                formatter,
                "[rust-source-topology/process/rustc-version-failed] detail={detail}"
            ),
            Error::RustcExecutableChangedDuringObservation => write!(
                formatter,
                "[rust-source-topology/process/rustc-executable-changed]"
            ),
            Error::RustcCfgProbeFailed { profile_id, detail } => write!(
                formatter,
                "[rust-source-topology/process/rustc-cfg-failed] profile={profile_id} detail={detail}"
            ),
            Error::RustcCfgMalformed { line } => write!(
                formatter,
                "[rust-source-topology/process/rustc-cfg-malformed] line={line:?}"
            ),
            Error::RustcCfgContractMismatch { profile_id, fact } => write!(
                formatter,
                "[rust-source-topology/process/rustc-cfg-contract-mismatch] profile={profile_id} fact={fact}"
            ),
            Error::WorkspaceRootNotDirectory => write!(
                formatter,
                "[rust-source-topology/process/workspace-root-not-directory]"
            ),
            Error::InputReadFailed { role, detail } => write!(
                formatter,
                "[rust-source-topology/process/input-read-failed] role={role} detail={detail}"
            ),
            Error::InputNotFile { role } => write!(
                formatter,
                "[rust-source-topology/process/input-not-file] role={role}"
            ),
            Error::InputOutsideWorkspace => write!(
                formatter,
                "[rust-source-topology/process/input-outside-workspace]"
            ),
            Error::AmbiguousRepositoryCargoConfig => write!(
                formatter,
                "[rust-source-topology/process/repository-config-ambiguous]"
            ),
            Error::AmbiguousExternalCargoConfig => write!(
                formatter,
                "[rust-source-topology/process/external-config-ambiguous]"
            ),
            Error::RepositoryCargoConfigNotUtf8 => write!(
                formatter,
                "[rust-source-topology/process/repository-config-not-utf8]"
            ),
            Error::RepositoryCargoConfigInvalid { detail } => write!(
                formatter,
                "[rust-source-topology/process/repository-config-invalid] detail={detail}"
            ),
            Error::CfgAffectingRepositoryConfig { key } => write!(
                formatter,
                "[rust-source-topology/process/repository-config-affects-cfg] key={key}"
            ),
            Error::UnsupportedRepositoryRustflags => write!(
                formatter,
                "[rust-source-topology/process/repository-rustflags-unsupported]"
            ),
            Error::CfgAffectingRepositoryRustflags => write!(
                formatter,
                "[rust-source-topology/process/repository-rustflags-affect-cfg]"
            ),
            Error::WorkspaceInputsChangedDuringObservation => write!(
                formatter,
                "[rust-source-topology/process/workspace-inputs-changed]"
            ),
        }
    }
}

impl std::error::Error for CargoProcessEvidenceErrorV1 {}
