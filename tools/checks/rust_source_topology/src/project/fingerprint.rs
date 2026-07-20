use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::process_error::CargoProcessEvidenceErrorV1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileFingerprintV1 {
    role: &'static str,
    workspace_relative_path: String,
    sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceInputFingerprintsV1 {
    manifest: FileFingerprintV1,
    cargo_lock: FileFingerprintV1,
    repository_cargo_config: Option<FileFingerprintV1>,
    external_cargo_configs: Box<[CargoConfigFingerprintV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CargoConfigFingerprintV1 {
    scope: String,
    sha256: String,
}

impl WorkspaceInputFingerprintsV1 {
    pub const fn manifest(&self) -> &FileFingerprintV1 {
        &self.manifest
    }

    pub const fn cargo_lock(&self) -> &FileFingerprintV1 {
        &self.cargo_lock
    }

    pub const fn repository_cargo_config(&self) -> Option<&FileFingerprintV1> {
        self.repository_cargo_config.as_ref()
    }

    pub fn external_cargo_configs(&self) -> &[CargoConfigFingerprintV1] {
        &self.external_cargo_configs
    }
}

impl CargoConfigFingerprintV1 {
    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }
}

impl FileFingerprintV1 {
    pub fn workspace_relative_path(&self) -> &str {
        &self.workspace_relative_path
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }
}

pub fn collect_workspace_input_fingerprints_v1(
    workspace_root: &Path,
    selected_manifest: &Path,
) -> Result<WorkspaceInputFingerprintsV1, CargoProcessEvidenceErrorV1> {
    let workspace_root = canonical_directory(workspace_root)?;
    let selected_manifest = canonical_file(selected_manifest, "manifest")?;
    let manifest_relative = workspace_relative(&workspace_root, &selected_manifest)?;
    let external_cargo_configs = collect_external_cargo_configs(&workspace_root)?;

    let cargo_lock_path = workspace_root.join("Cargo.lock");
    let cargo_lock = fingerprint_file(&workspace_root, &cargo_lock_path, "cargo_lock")?;
    let manifest =
        fingerprint_file_from_relative(&selected_manifest, "manifest", manifest_relative)?;
    let repository_cargo_config = repository_config_path(&workspace_root)?
        .map(|path| -> Result<_, CargoProcessEvidenceErrorV1> {
            let relative = if path.file_name().and_then(|name| name.to_str()) == Some("config") {
                ".cargo/config"
            } else {
                ".cargo/config.toml"
            };
            let bytes =
                fs::read(&path).map_err(|error| CargoProcessEvidenceErrorV1::InputReadFailed {
                    role: "repository_cargo_config",
                    detail: error.kind().to_string(),
                })?;
            validate_repository_cargo_config(&bytes)?;
            Ok(fingerprint_bytes(
                "repository_cargo_config",
                relative,
                &bytes,
            ))
        })
        .transpose()?;

    Ok(WorkspaceInputFingerprintsV1 {
        manifest,
        cargo_lock,
        repository_cargo_config,
        external_cargo_configs,
    })
}

pub(crate) fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn fingerprint_file(
    workspace_root: &Path,
    path: &Path,
    role: &'static str,
) -> Result<FileFingerprintV1, CargoProcessEvidenceErrorV1> {
    let canonical = canonical_file(path, role)?;
    let relative = workspace_relative(workspace_root, &canonical)?;
    fingerprint_file_from_relative(&canonical, role, relative)
}

fn fingerprint_file_from_relative(
    path: &Path,
    role: &'static str,
    relative: String,
) -> Result<FileFingerprintV1, CargoProcessEvidenceErrorV1> {
    let bytes = fs::read(path).map_err(|error| CargoProcessEvidenceErrorV1::InputReadFailed {
        role,
        detail: error.kind().to_string(),
    })?;
    Ok(fingerprint_bytes(role, &relative, &bytes))
}

fn fingerprint_bytes(role: &'static str, relative: &str, bytes: &[u8]) -> FileFingerprintV1 {
    FileFingerprintV1 {
        role,
        workspace_relative_path: relative.replace('\\', "/"),
        sha256: sha256_bytes(bytes),
    }
}

fn canonical_directory(path: &Path) -> Result<PathBuf, CargoProcessEvidenceErrorV1> {
    let canonical =
        fs::canonicalize(path).map_err(|error| CargoProcessEvidenceErrorV1::InputReadFailed {
            role: "workspace_root",
            detail: error.kind().to_string(),
        })?;
    if !canonical.is_dir() {
        return Err(CargoProcessEvidenceErrorV1::WorkspaceRootNotDirectory);
    }
    Ok(canonical)
}

fn canonical_file(path: &Path, role: &'static str) -> Result<PathBuf, CargoProcessEvidenceErrorV1> {
    let canonical =
        fs::canonicalize(path).map_err(|error| CargoProcessEvidenceErrorV1::InputReadFailed {
            role,
            detail: error.kind().to_string(),
        })?;
    if !canonical.is_file() {
        return Err(CargoProcessEvidenceErrorV1::InputNotFile { role });
    }
    Ok(canonical)
}

fn workspace_relative(
    workspace_root: &Path,
    path: &Path,
) -> Result<String, CargoProcessEvidenceErrorV1> {
    let relative = path
        .strip_prefix(workspace_root)
        .map_err(|_| CargoProcessEvidenceErrorV1::InputOutsideWorkspace)?;
    if relative.as_os_str().is_empty() {
        return Err(CargoProcessEvidenceErrorV1::InputOutsideWorkspace);
    }
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn repository_config_path(
    workspace_root: &Path,
) -> Result<Option<PathBuf>, CargoProcessEvidenceErrorV1> {
    let modern = workspace_root.join(".cargo/config.toml");
    let legacy = workspace_root.join(".cargo/config");
    match (modern.is_file(), legacy.is_file()) {
        (true, true) => Err(CargoProcessEvidenceErrorV1::AmbiguousRepositoryCargoConfig),
        (true, false) => Ok(Some(modern)),
        (false, true) => Ok(Some(legacy)),
        (false, false) => Ok(None),
    }
}

fn collect_external_cargo_configs(
    workspace_root: &Path,
) -> Result<Box<[CargoConfigFingerprintV1]>, CargoProcessEvidenceErrorV1> {
    let mut configs = Vec::new();
    let mut seen = BTreeSet::new();
    let mut ancestor = workspace_root.parent();
    let mut ordinal = 1usize;
    while let Some(directory) = ancestor {
        if let Some(path) = config_in_cargo_directory(&directory.join(".cargo"))? {
            let canonical = fs::canonicalize(&path).unwrap_or(path);
            if seen.insert(canonical.clone()) {
                configs.push(read_external_config(
                    format!("ancestor:{ordinal}"),
                    &canonical,
                )?);
            }
        }
        ancestor = directory.parent();
        ordinal += 1;
    }
    if let Some(home) = cargo_home() {
        if !home.starts_with(workspace_root) {
            if let Some(path) = config_in_cargo_directory(&home)? {
                let canonical = fs::canonicalize(&path).unwrap_or(path);
                if seen.insert(canonical.clone()) {
                    configs.push(read_external_config("cargo_home".to_string(), &canonical)?);
                }
            }
        }
    }
    configs.sort_by(|left, right| left.scope.cmp(&right.scope));
    Ok(configs.into_boxed_slice())
}

fn config_in_cargo_directory(
    cargo_directory: &Path,
) -> Result<Option<PathBuf>, CargoProcessEvidenceErrorV1> {
    let modern = cargo_directory.join("config.toml");
    let legacy = cargo_directory.join("config");
    match (modern.is_file(), legacy.is_file()) {
        (true, true) => Err(CargoProcessEvidenceErrorV1::AmbiguousExternalCargoConfig),
        (true, false) => Ok(Some(modern)),
        (false, true) => Ok(Some(legacy)),
        (false, false) => Ok(None),
    }
}

fn read_external_config(
    scope: String,
    path: &Path,
) -> Result<CargoConfigFingerprintV1, CargoProcessEvidenceErrorV1> {
    let bytes = fs::read(path).map_err(|error| CargoProcessEvidenceErrorV1::InputReadFailed {
        role: "external_cargo_config",
        detail: error.kind().to_string(),
    })?;
    validate_repository_cargo_config(&bytes)?;
    Ok(CargoConfigFingerprintV1 {
        scope,
        sha256: sha256_bytes(&bytes),
    })
}

fn cargo_home() -> Option<PathBuf> {
    std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo")))
}

fn validate_repository_cargo_config(bytes: &[u8]) -> Result<(), CargoProcessEvidenceErrorV1> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| CargoProcessEvidenceErrorV1::RepositoryCargoConfigNotUtf8)?;
    let document = text.parse::<toml::Value>().map_err(|error| {
        CargoProcessEvidenceErrorV1::RepositoryCargoConfigInvalid {
            detail: error.message().to_string(),
        }
    })?;
    reject_cfg_affecting_config(&document, &mut Vec::new())
}

fn reject_cfg_affecting_config(
    value: &toml::Value,
    path: &mut Vec<String>,
) -> Result<(), CargoProcessEvidenceErrorV1> {
    let toml::Value::Table(table) = value else {
        return Ok(());
    };
    for (key, child) in table {
        path.push(key.clone());
        let dotted = path.join(".");
        if dotted == "profile"
            || dotted == "env"
            || dotted == "build.target"
            || dotted == "build.rustc"
            || dotted == "build.rustc-wrapper"
            || dotted == "build.rustc-workspace-wrapper"
        {
            return Err(CargoProcessEvidenceErrorV1::CfgAffectingRepositoryConfig { key: dotted });
        }
        if key == "rustflags" {
            validate_rustflags(child)?;
        }
        reject_cfg_affecting_config(child, path)?;
        path.pop();
    }
    Ok(())
}

fn validate_rustflags(value: &toml::Value) -> Result<(), CargoProcessEvidenceErrorV1> {
    let tokens = match value {
        toml::Value::String(flags) => flags
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
        toml::Value::Array(flags) => flags
            .iter()
            .map(|flag| flag.as_str().map(str::to_string))
            .collect::<Option<Vec<_>>>()
            .ok_or(CargoProcessEvidenceErrorV1::UnsupportedRepositoryRustflags)?,
        _ => return Err(CargoProcessEvidenceErrorV1::UnsupportedRepositoryRustflags),
    };
    let mut index = 0;
    while index < tokens.len() {
        let token = &tokens[index];
        if token == "--cfg" || token.starts_with("--cfg=") {
            return Err(CargoProcessEvidenceErrorV1::CfgAffectingRepositoryRustflags);
        }
        if token == "-C" {
            let Some(option) = tokens.get(index + 1) else {
                return Err(CargoProcessEvidenceErrorV1::UnsupportedRepositoryRustflags);
            };
            if !is_safe_codegen_option(option) {
                return Err(CargoProcessEvidenceErrorV1::CfgAffectingRepositoryRustflags);
            }
            index += 2;
            continue;
        }
        if let Some(option) = token.strip_prefix("-C") {
            if !is_safe_codegen_option(option) {
                return Err(CargoProcessEvidenceErrorV1::CfgAffectingRepositoryRustflags);
            }
            index += 1;
            continue;
        }
        return Err(CargoProcessEvidenceErrorV1::UnsupportedRepositoryRustflags);
    }
    Ok(())
}

fn is_safe_codegen_option(option: &str) -> bool {
    option.starts_with("link-arg=") || option.starts_with("linker=")
}
