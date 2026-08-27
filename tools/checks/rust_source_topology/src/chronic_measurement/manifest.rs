use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::error::ChronicScanErrorV1;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScopeManifestV1 {
    pub schema_version: u32,
    pub scanner_version: String,
    pub scope_id: String,
    #[serde(default)]
    pub workspace_root: String,
    #[serde(default)]
    pub exclude_prefixes: Vec<String>,
    pub roots: Vec<ScopeRootV1>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScopeRootV1 {
    pub path: String,
    pub kind: ScopeRootKindV1,
    pub compile_domain: String,
    pub role: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ScopeRootKindV1 {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub(crate) struct ScopedRustFileV1 {
    pub relative_path: String,
    pub absolute_path: PathBuf,
    pub compile_domain: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadedScopeV1 {
    pub manifest: ScopeManifestV1,
    pub manifest_hash: String,
    pub files: Vec<ScopedRustFileV1>,
}

pub(crate) fn load_scope_manifest(
    manifest_path: &Path,
    workspace_root: &Path,
) -> Result<LoadedScopeV1, ChronicScanErrorV1> {
    if manifest_path.as_os_str().is_empty() {
        return Err(ChronicScanErrorV1::EmptyManifestPath);
    }
    let bytes = fs::read(manifest_path).map_err(|error| ChronicScanErrorV1::ManifestRead {
        detail: error.to_string(),
    })?;
    let manifest_text =
        String::from_utf8(bytes.clone()).map_err(|_| ChronicScanErrorV1::ManifestParse {
            detail: "manifest is not UTF-8".into(),
        })?;
    let manifest: ScopeManifestV1 =
        toml::from_str(&manifest_text).map_err(|error| ChronicScanErrorV1::ManifestParse {
            detail: error.to_string(),
        })?;
    validate_manifest(&manifest)?;
    let root = workspace_root
        .canonicalize()
        .map_err(|error| ChronicScanErrorV1::ManifestRead {
            detail: format!("workspace root: {error}"),
        })?;
    let mut files = Vec::new();
    let mut seen = BTreeSet::new();
    for entry in &manifest.roots {
        let relative = clean_relative_path(&entry.path)?;
        if excluded(&relative, &manifest.exclude_prefixes) {
            return Err(ChronicScanErrorV1::InvalidManifest {
                detail: format!("root is excluded: {}", entry.path),
            });
        }
        let absolute = root.join(&relative);
        match entry.kind {
            ScopeRootKindV1::File => {
                collect_file(&root, &relative, &absolute, entry, &mut seen, &mut files)?
            }
            ScopeRootKindV1::Directory => collect_directory(
                &root,
                &relative,
                &absolute,
                entry,
                &manifest.exclude_prefixes,
                &mut seen,
                &mut files,
            )?,
        }
    }
    if files.is_empty() {
        return Err(ChronicScanErrorV1::EmptyScope);
    }
    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(LoadedScopeV1 {
        manifest,
        manifest_hash: format!("sha256:{:x}", Sha256::digest(bytes)),
        files,
    })
}

pub(crate) fn scope_paths(scope: &LoadedScopeV1) -> BTreeSet<String> {
    scope
        .files
        .iter()
        .map(|file| file.relative_path.clone())
        .collect()
}

fn validate_manifest(manifest: &ScopeManifestV1) -> Result<(), ChronicScanErrorV1> {
    if manifest.schema_version != 1
        || manifest.scanner_version.is_empty()
        || manifest.scope_id.is_empty()
        || manifest.roots.is_empty()
    {
        return Err(ChronicScanErrorV1::InvalidManifest {
            detail: "schema_version=1, scanner_version, scope_id, and roots are required".into(),
        });
    }
    if !manifest.workspace_root.is_empty() && manifest.workspace_root != "." {
        return Err(ChronicScanErrorV1::InvalidManifest {
            detail: "workspace_root must be '.' for a workspace-relative manifest".into(),
        });
    }
    for entry in &manifest.roots {
        if entry.path.is_empty() || entry.compile_domain.is_empty() || entry.role.is_empty() {
            return Err(ChronicScanErrorV1::InvalidManifest {
                detail: "root path, compile_domain, and role are required".into(),
            });
        }
        if !valid_compile_domain(&entry.compile_domain) || !valid_role(&entry.role) {
            return Err(ChronicScanErrorV1::InvalidManifest {
                detail: format!("unknown classification for {}", entry.path),
            });
        }
    }
    Ok(())
}

fn valid_compile_domain(value: &str) -> bool {
    matches!(
        value,
        "production_default"
            | "cfg_test"
            | "feature_nonselected"
            | "generated_included"
            | "unknown"
    )
}

fn valid_role(value: &str) -> bool {
    matches!(
        value,
        "runtime"
            | "test_support"
            | "fixture"
            | "compatibility"
            | "guard_evidence"
            | "generated_registry"
            | "mixed"
            | "unknown"
    )
}

fn clean_relative_path(value: &str) -> Result<PathBuf, ChronicScanErrorV1> {
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ChronicScanErrorV1::PathEscape {
            path: value.to_string(),
        });
    }
    Ok(path.to_path_buf())
}

fn excluded(relative: &Path, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| {
        let prefix = Path::new(prefix);
        relative == prefix || relative.starts_with(prefix)
    })
}

fn collect_file(
    root: &Path,
    relative: &Path,
    absolute: &Path,
    entry: &ScopeRootV1,
    seen: &mut BTreeSet<String>,
    files: &mut Vec<ScopedRustFileV1>,
) -> Result<(), ChronicScanErrorV1> {
    let metadata =
        fs::symlink_metadata(absolute).map_err(|_| ChronicScanErrorV1::ScopeEntryMissing {
            path: relative.display().to_string(),
        })?;
    if metadata.file_type().is_symlink() {
        return Err(ChronicScanErrorV1::SymlinkInput {
            path: relative.display().to_string(),
        });
    }
    if !metadata.is_file() || relative.extension().and_then(|ext| ext.to_str()) != Some("rs") {
        return Err(ChronicScanErrorV1::ScopeEntryKindMismatch {
            path: relative.display().to_string(),
        });
    }
    push_file(root, relative, absolute, entry, seen, files)
}

fn collect_directory(
    root: &Path,
    relative: &Path,
    absolute: &Path,
    entry: &ScopeRootV1,
    excludes: &[String],
    seen: &mut BTreeSet<String>,
    files: &mut Vec<ScopedRustFileV1>,
) -> Result<(), ChronicScanErrorV1> {
    let metadata =
        fs::symlink_metadata(absolute).map_err(|_| ChronicScanErrorV1::ScopeEntryMissing {
            path: relative.display().to_string(),
        })?;
    if metadata.file_type().is_symlink() {
        return Err(ChronicScanErrorV1::SymlinkInput {
            path: relative.display().to_string(),
        });
    }
    if !metadata.is_dir() {
        return Err(ChronicScanErrorV1::ScopeEntryKindMismatch {
            path: relative.display().to_string(),
        });
    }
    let mut children = fs::read_dir(absolute)
        .map_err(|error| ChronicScanErrorV1::DirectoryRead {
            path: relative.display().to_string(),
            detail: error.to_string(),
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| ChronicScanErrorV1::DirectoryRead {
            path: relative.display().to_string(),
            detail: error.to_string(),
        })?;
    children.sort_by_key(|child| child.file_name());
    for child in children {
        let child_path = child.path();
        let child_relative =
            child_path
                .strip_prefix(root)
                .map_err(|_| ChronicScanErrorV1::PathEscape {
                    path: child_path.display().to_string(),
                })?;
        if excluded(child_relative, excludes) {
            continue;
        }
        let metadata = fs::symlink_metadata(&child_path).map_err(|error| {
            ChronicScanErrorV1::DirectoryRead {
                path: child_relative.display().to_string(),
                detail: error.to_string(),
            }
        })?;
        if metadata.file_type().is_symlink() {
            return Err(ChronicScanErrorV1::SymlinkInput {
                path: child_relative.display().to_string(),
            });
        }
        if metadata.is_dir() {
            collect_directory(
                root,
                child_relative,
                &child_path,
                entry,
                excludes,
                seen,
                files,
            )?;
        } else if metadata.is_file()
            && child_relative.extension().and_then(|ext| ext.to_str()) == Some("rs")
        {
            push_file(root, child_relative, &child_path, entry, seen, files)?;
        }
    }
    Ok(())
}

fn push_file(
    root: &Path,
    relative: &Path,
    absolute: &Path,
    entry: &ScopeRootV1,
    seen: &mut BTreeSet<String>,
    files: &mut Vec<ScopedRustFileV1>,
) -> Result<(), ChronicScanErrorV1> {
    let relative_path = relative.display().to_string();
    let canonical = absolute
        .canonicalize()
        .map_err(|error| ChronicScanErrorV1::SourceRead {
            path: relative_path.clone(),
            detail: error.to_string(),
        })?;
    if !canonical.starts_with(root) {
        return Err(ChronicScanErrorV1::PathEscape {
            path: relative_path,
        });
    }
    if !seen.insert(relative.display().to_string()) {
        return Err(ChronicScanErrorV1::DuplicateScopePath {
            path: relative.display().to_string(),
        });
    }
    files.push(ScopedRustFileV1 {
        relative_path: relative.display().to_string(),
        absolute_path: absolute.to_path_buf(),
        compile_domain: entry.compile_domain.clone(),
        role: entry.role.clone(),
    });
    Ok(())
}
