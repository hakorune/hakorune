//! Error helpers for using resolver (placeholder)

#[derive(thiserror::Error, Debug)]
pub enum UsingError {
    #[error("failed to read nyash.toml: {0}")]
    ReadToml(String),
    #[error("invalid nyash.toml format: {0}")]
    ParseToml(String),
    #[error("failed to read workspace module '{0}': {1}")]
    ReadWorkspaceModule(String, String),
    #[error("invalid workspace module '{0}': {1}")]
    ParseWorkspaceModule(String, String),
    #[error("workspace module '{0}' is missing module.name")]
    WorkspaceModuleMissingName(String),
}
