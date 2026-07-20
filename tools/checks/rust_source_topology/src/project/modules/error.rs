use std::fmt;

use crate::project::{CargoProcessEvidenceErrorV1, CfgDecisionErrorV1};

#[derive(Debug)]
pub enum ModuleTopologyErrorV1 {
    CargoEvidence(CargoProcessEvidenceErrorV1),
    WorkspaceEvidenceDrift,
    WorkspaceRootInvalid,
    SourceOutsideWorkspace { path: String },
    SourceMissing { path: String },
    SourceNotFile { path: String },
    SourceRead { path: String, detail: String },
    SourceChanged { path: String },
    Parse { path: String, detail: String },
    Cfg(CfgDecisionErrorV1),
    UnknownCfg { module: String },
    UnsupportedModuleAttribute { module: String, attribute: String },
    UnsupportedInnerTopologyAttribute { path: String },
    ModuleInBlock { path: String },
    NonLiteralPath { module: String },
    MultipleActivePaths { module: String },
    OrdinaryModuleMissing { module: String },
    OrdinaryModuleAmbiguous { module: String },
    CanonicalCycle { path: String },
}

impl fmt::Display for ModuleTopologyErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CargoEvidence(error) => error.fmt(formatter),
            Self::WorkspaceEvidenceDrift => write!(formatter, "[rust-source-topology/module/workspace-evidence-drift]"),
            Self::WorkspaceRootInvalid => write!(formatter, "[rust-source-topology/module/workspace-root-invalid]"),
            Self::SourceOutsideWorkspace { path } => write!(formatter, "[rust-source-topology/module/source-outside-workspace] path={path:?}"),
            Self::SourceMissing { path } => write!(formatter, "[rust-source-topology/module/source-missing] path={path:?}"),
            Self::SourceNotFile { path } => write!(formatter, "[rust-source-topology/module/source-not-file] path={path:?}"),
            Self::SourceRead { path, detail } => write!(formatter, "[rust-source-topology/module/source-read-failed] path={path:?} detail={detail}"),
            Self::SourceChanged { path } => write!(formatter, "[rust-source-topology/module/source-changed] path={path:?}"),
            Self::Parse { path, detail } => write!(formatter, "[rust-source-topology/module/parse-failed] path={path:?} detail={detail}"),
            Self::Cfg(error) => error.fmt(formatter),
            Self::UnknownCfg { module } => write!(formatter, "[rust-source-topology/module/cfg-unknown] module={module}"),
            Self::UnsupportedModuleAttribute { module, attribute } => write!(formatter, "[rust-source-topology/module/unsupported-attribute] module={module} attribute={attribute:?}"),
            Self::UnsupportedInnerTopologyAttribute { path } => write!(formatter, "[rust-source-topology/module/unsupported-inner-topology-attribute] path={path:?}"),
            Self::ModuleInBlock { path } => write!(formatter, "[rust-source-topology/module/block-module-unsupported] path={path:?}"),
            Self::NonLiteralPath { module } => write!(formatter, "[rust-source-topology/module/nonliteral-path] module={module}"),
            Self::MultipleActivePaths { module } => write!(formatter, "[rust-source-topology/module/multiple-active-paths] module={module}"),
            Self::OrdinaryModuleMissing { module } => write!(formatter, "[rust-source-topology/module/ordinary-missing] module={module}"),
            Self::OrdinaryModuleAmbiguous { module } => write!(formatter, "[rust-source-topology/module/ordinary-ambiguous] module={module}"),
            Self::CanonicalCycle { path } => write!(formatter, "[rust-source-topology/module/canonical-cycle] path={path:?}"),
        }
    }
}

impl std::error::Error for ModuleTopologyErrorV1 {}

impl From<CargoProcessEvidenceErrorV1> for ModuleTopologyErrorV1 {
    fn from(error: CargoProcessEvidenceErrorV1) -> Self {
        Self::CargoEvidence(error)
    }
}

impl From<CfgDecisionErrorV1> for ModuleTopologyErrorV1 {
    fn from(error: CfgDecisionErrorV1) -> Self {
        Self::Cfg(error)
    }
}
