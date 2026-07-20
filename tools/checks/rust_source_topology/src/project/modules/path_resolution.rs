use std::fs;
use std::path::{Component, Path, PathBuf};

use super::error::ModuleTopologyErrorV1;
use super::model::ModuleInstanceKindV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ModuleDirectoryOwnershipV1 {
    source_directory: PathBuf,
    relative_segment: Option<String>,
}

impl ModuleDirectoryOwnershipV1 {
    pub(super) fn root(source_file: &Path) -> Result<Self, ModuleTopologyErrorV1> {
        Ok(Self {
            source_directory: source_file
                .parent()
                .ok_or(ModuleTopologyErrorV1::WorkspaceRootInvalid)?
                .to_path_buf(),
            relative_segment: None,
        })
    }

    pub(super) fn lookup_directory(&self) -> PathBuf {
        match &self.relative_segment {
            Some(segment) => self.source_directory.join(segment),
            None => self.source_directory.clone(),
        }
    }

    pub(super) fn inline_child(&self, segment: &str, literal_path: Option<&str>) -> Self {
        match literal_path {
            Some(path) => Self {
                source_directory: self.source_directory.join(path),
                relative_segment: None,
            },
            None => Self {
                source_directory: self.lookup_directory().join(segment),
                relative_segment: None,
            },
        }
    }

    fn ordinary_file(source_file: &Path, segment: &str) -> Result<Self, ModuleTopologyErrorV1> {
        Ok(Self {
            source_directory: source_file
                .parent()
                .ok_or(ModuleTopologyErrorV1::WorkspaceRootInvalid)?
                .to_path_buf(),
            relative_segment: Some(segment.to_string()),
        })
    }

    fn mod_file(source_file: &Path) -> Result<Self, ModuleTopologyErrorV1> {
        Ok(Self {
            source_directory: source_file
                .parent()
                .ok_or(ModuleTopologyErrorV1::WorkspaceRootInvalid)?
                .to_path_buf(),
            relative_segment: None,
        })
    }
}

pub(super) struct ResolvedExternalModuleV1 {
    pub lexical_path: PathBuf,
    pub canonical_path: PathBuf,
    pub kind: ModuleInstanceKindV1,
    pub directory: ModuleDirectoryOwnershipV1,
}

pub(super) fn resolve_external_module_v1(
    workspace_root: &Path,
    parent_directory: &ModuleDirectoryOwnershipV1,
    segment: &str,
    literal_path: Option<&str>,
) -> Result<ResolvedExternalModuleV1, ModuleTopologyErrorV1> {
    if let Some(path) = literal_path {
        let lexical = normalize_inside_workspace(
            workspace_root,
            &parent_directory.source_directory.join(path),
        )?;
        let canonical = canonical_regular_file(workspace_root, &lexical)?;
        return Ok(ResolvedExternalModuleV1 {
            lexical_path: lexical,
            canonical_path: canonical,
            kind: ModuleInstanceKindV1::LiteralPath,
            directory: ModuleDirectoryOwnershipV1::mod_file(
                &parent_directory.source_directory.join(path),
            )?,
        });
    }

    let lookup = parent_directory.lookup_directory();
    let flat = normalize_inside_workspace(workspace_root, &lookup.join(format!("{segment}.rs")))?;
    let nested = normalize_inside_workspace(workspace_root, &lookup.join(segment).join("mod.rs"))?;
    let flat_exists = candidate_is_file(workspace_root, &flat)?;
    let nested_exists = candidate_is_file(workspace_root, &nested)?;
    let (lexical, kind, directory) = match (flat_exists, nested_exists) {
        (false, false) => {
            return Err(ModuleTopologyErrorV1::OrdinaryModuleMissing {
                module: segment.to_string(),
            })
        }
        (true, true) => {
            return Err(ModuleTopologyErrorV1::OrdinaryModuleAmbiguous {
                module: segment.to_string(),
            })
        }
        (true, false) => (
            flat,
            ModuleInstanceKindV1::OrdinaryFile,
            ModuleDirectoryOwnershipV1::ordinary_file(
                &lookup.join(format!("{segment}.rs")),
                segment,
            )?,
        ),
        (false, true) => (
            nested,
            ModuleInstanceKindV1::OrdinaryModFile,
            ModuleDirectoryOwnershipV1::mod_file(&lookup.join(segment).join("mod.rs"))?,
        ),
    };
    let canonical = canonical_regular_file(workspace_root, &lexical)?;
    Ok(ResolvedExternalModuleV1 {
        lexical_path: lexical,
        canonical_path: canonical,
        kind,
        directory,
    })
}

pub(super) fn normalize_inside_workspace(
    workspace_root: &Path,
    path: &Path,
) -> Result<PathBuf, ModuleTopologyErrorV1> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir => {
                if !normalized.pop() {
                    return Err(ModuleTopologyErrorV1::SourceOutsideWorkspace {
                        path: path.to_string_lossy().into_owned(),
                    });
                }
            }
        }
    }
    if !normalized.starts_with(workspace_root) {
        return Err(ModuleTopologyErrorV1::SourceOutsideWorkspace {
            path: path.to_string_lossy().into_owned(),
        });
    }
    Ok(normalized)
}

pub(super) fn canonical_regular_file(
    workspace_root: &Path,
    path: &Path,
) -> Result<PathBuf, ModuleTopologyErrorV1> {
    let metadata = fs::symlink_metadata(path).map_err(|error| match error.kind() {
        std::io::ErrorKind::NotFound => ModuleTopologyErrorV1::SourceMissing {
            path: display_relative(workspace_root, path),
        },
        _ => ModuleTopologyErrorV1::SourceRead {
            path: display_relative(workspace_root, path),
            detail: error.kind().to_string(),
        },
    })?;
    if !metadata.file_type().is_file() && !metadata.file_type().is_symlink() {
        return Err(ModuleTopologyErrorV1::SourceNotFile {
            path: display_relative(workspace_root, path),
        });
    }
    let canonical = fs::canonicalize(path).map_err(|error| ModuleTopologyErrorV1::SourceRead {
        path: display_relative(workspace_root, path),
        detail: error.kind().to_string(),
    })?;
    if !canonical.starts_with(workspace_root) {
        return Err(ModuleTopologyErrorV1::SourceOutsideWorkspace {
            path: display_relative(workspace_root, path),
        });
    }
    if !canonical.is_file() {
        return Err(ModuleTopologyErrorV1::SourceNotFile {
            path: display_relative(workspace_root, path),
        });
    }
    Ok(canonical)
}

fn candidate_is_file(workspace_root: &Path, path: &Path) -> Result<bool, ModuleTopologyErrorV1> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() || metadata.file_type().is_symlink() => {
            canonical_regular_file(workspace_root, path)?;
            Ok(true)
        }
        Ok(_) => Err(ModuleTopologyErrorV1::SourceNotFile {
            path: display_relative(workspace_root, path),
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(ModuleTopologyErrorV1::SourceRead {
            path: display_relative(workspace_root, path),
            detail: error.kind().to_string(),
        }),
    }
}

pub(super) fn workspace_relative(
    workspace_root: &Path,
    path: &Path,
) -> Result<String, ModuleTopologyErrorV1> {
    path.strip_prefix(workspace_root)
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
        .map_err(|_| ModuleTopologyErrorV1::SourceOutsideWorkspace {
            path: path.to_string_lossy().into_owned(),
        })
}

fn display_relative(workspace_root: &Path, path: &Path) -> String {
    workspace_relative(workspace_root, path).unwrap_or_else(|_| path.to_string_lossy().into_owned())
}
