use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::extract_single_file_source;
use crate::project::cargo::CargoDeclaredUnitProcessEvidenceV1;
use crate::project::{
    collect_workspace_input_fingerprints_v1, CfgDecisionStateV1, CfgEvaluationEnvironmentV1,
};

use super::cfg_gate::{
    decide_module_cfg_v1, sealed_cfg_environment_v1, select_active_path_v1,
    validate_active_cfg_attributes_v1,
};
use super::declarations::{
    include_literal, outer_cfg_syntax, parse_included_module_source_v1, parse_module_source_v1,
    validate_include_attributes, validate_module_attributes, IncludeDeclarationV1,
    ModuleDeclarationV1, ModulePositionItemV1,
};
use super::error::ModuleTopologyErrorV1;
use super::model::{
    DeclaredIncludeEdgeV1, DeclaredModuleEdgeV1, DeclaredModuleInstanceV1,
    DeclaredModuleTopologyV1, ModuleEdgeKindV1, ModuleInstanceKindV1, ModuleSourceObservationV1,
    DECLARED_MODULE_TOPOLOGY_SCHEMA_V1,
};
use super::path_resolution::{
    canonical_regular_file, normalize_inside_workspace, resolve_external_module_v1,
    resolve_include_source_v1, workspace_relative, ModuleDirectoryOwnershipV1,
};

pub fn collect_declared_module_topology_v1(
    workspace_root: &Path,
    evidence: &CargoDeclaredUnitProcessEvidenceV1,
) -> Result<DeclaredModuleTopologyV1, ModuleTopologyErrorV1> {
    let workspace_root = workspace_root
        .canonicalize()
        .map_err(|_| ModuleTopologyErrorV1::WorkspaceRootInvalid)?;
    if !workspace_root.is_dir() {
        return Err(ModuleTopologyErrorV1::WorkspaceRootInvalid);
    }
    let manifest = workspace_root.join(
        evidence
            .declared_unit()
            .package()
            .manifest_path_workspace_relative(),
    );
    let before = collect_workspace_input_fingerprints_v1(&workspace_root, &manifest)?;
    if &before != evidence.workspace_inputs() {
        return Err(ModuleTopologyErrorV1::WorkspaceEvidenceDrift);
    }
    let root_lexical = normalize_inside_workspace(
        &workspace_root,
        &workspace_root.join(
            evidence
                .declared_unit()
                .target()
                .src_path_workspace_relative(),
        ),
    )?;
    let root_canonical = canonical_regular_file(&workspace_root, &root_lexical)?;
    let environment = sealed_cfg_environment_v1(evidence);
    let mut traversal = ModuleTraversalV1::new(
        workspace_root.clone(),
        evidence.declared_unit().profile_id(),
        evidence.declared_unit().package().package_key(),
        evidence.declared_unit().target().target_key(),
        environment,
    );
    traversal.collect_root(root_lexical, root_canonical)?;
    traversal.verify_sources_unchanged()?;
    let after = collect_workspace_input_fingerprints_v1(&workspace_root, &manifest)?;
    if before != after || &after != evidence.workspace_inputs() {
        return Err(ModuleTopologyErrorV1::WorkspaceEvidenceDrift);
    }
    traversal.finish()
}

struct ModuleTraversalV1 {
    workspace_root: PathBuf,
    profile_id: String,
    package_key: String,
    target_key: String,
    environment: CfgEvaluationEnvironmentV1,
    instances: Vec<DeclaredModuleInstanceV1>,
    edges: Vec<DeclaredModuleEdgeV1>,
    include_edges: Vec<DeclaredIncludeEdgeV1>,
    observations: Vec<ModuleSourceObservationV1>,
    source_snapshots: BTreeMap<PathBuf, String>,
    canonical_ancestry: Vec<PathBuf>,
}

impl ModuleTraversalV1 {
    fn new(
        workspace_root: PathBuf,
        profile_id: &str,
        package_key: &str,
        target_key: &str,
        environment: CfgEvaluationEnvironmentV1,
    ) -> Self {
        Self {
            workspace_root,
            profile_id: profile_id.to_string(),
            package_key: package_key.to_string(),
            target_key: target_key.to_string(),
            environment,
            instances: Vec::new(),
            edges: Vec::new(),
            include_edges: Vec::new(),
            observations: Vec::new(),
            source_snapshots: BTreeMap::new(),
            canonical_ancestry: Vec::new(),
        }
    }

    fn collect_root(
        &mut self,
        lexical_path: PathBuf,
        canonical_path: PathBuf,
    ) -> Result<(), ModuleTopologyErrorV1> {
        let directory = ModuleDirectoryOwnershipV1::root(&lexical_path)?;
        let source = self.read_source(&lexical_path, &canonical_path)?;
        let parsed = parse_module_source_v1(&self.relative(&lexical_path)?, &source, false)?;
        let root_id = self.next_instance_id();
        let observation_id = self.add_source_observation(
            &root_id,
            &lexical_path,
            &canonical_path,
            "crate",
            &source,
            None,
        )?;
        self.instances.push(DeclaredModuleInstanceV1 {
            instance_id: root_id.clone(),
            parent_edge_id: None,
            module_syntax_path: "crate".to_string(),
            kind: ModuleInstanceKindV1::Root,
            source_path_workspace_relative: self.relative(&lexical_path)?,
            canonical_source_path_workspace_relative: self.relative(&canonical_path)?,
            source_observation_id: observation_id,
            inline_body_range: None,
        });
        self.canonical_ancestry.push(canonical_path);
        let result = self.walk_items(
            &root_id,
            "crate",
            &lexical_path,
            &self.relative(&self.canonical_ancestry[0])?,
            &self.instances[0].source_observation_id.clone(),
            &directory,
            &parsed.items,
        );
        self.canonical_ancestry.pop();
        result
    }

    #[allow(clippy::too_many_arguments)]
    fn walk_items(
        &mut self,
        parent_instance_id: &str,
        parent_syntax_path: &str,
        parent_lexical_path: &Path,
        parent_canonical_relative: &str,
        parent_observation_id: &str,
        parent_directory: &ModuleDirectoryOwnershipV1,
        items: &[ModulePositionItemV1],
    ) -> Result<(), ModuleTopologyErrorV1> {
        for item in items {
            let ModulePositionItemV1::Module(declaration) = item else {
                let ModulePositionItemV1::Include(include) = item else {
                    unreachable!();
                };
                self.add_include_source(
                    parent_instance_id,
                    parent_syntax_path,
                    parent_lexical_path,
                    parent_observation_id,
                    include,
                )?;
                continue;
            };
            let module_path = format!("{parent_syntax_path}::{}", declaration.semantic_segment);
            let cfg = decide_module_cfg_v1(
                &outer_cfg_syntax(&declaration.outer_attributes),
                &self.environment,
            )?;
            if cfg.state == CfgDecisionStateV1::Unknown {
                return Err(ModuleTopologyErrorV1::UnknownCfg {
                    module: module_path,
                });
            }
            let edge_id = self.next_edge_id();
            if cfg.state == CfgDecisionStateV1::Excluded {
                self.edges.push(DeclaredModuleEdgeV1 {
                    edge_id,
                    parent_instance_id: parent_instance_id.to_string(),
                    declaration_source_observation_id: parent_observation_id.to_string(),
                    declaration_range: declaration.range,
                    declared_ident_syntax: declaration.ident_syntax.clone(),
                    semantic_segment: declaration.semantic_segment.clone(),
                    kind: if declaration.inline_items.is_some() {
                        ModuleEdgeKindV1::Inline
                    } else {
                        ModuleEdgeKindV1::Ordinary
                    },
                    active_literal_path: None,
                    cfg_decision: cfg,
                    child_instance_id: None,
                    selected_source_path_workspace_relative: None,
                });
                continue;
            }
            validate_module_attributes(&module_path, &declaration.outer_attributes)?;
            validate_active_cfg_attributes_v1(
                &module_path,
                &declaration.outer_attributes,
                &self.environment,
            )?;
            let literal_path = select_active_path_v1(
                &module_path,
                &declaration.outer_attributes,
                &self.environment,
            )?;
            if let Some(children) = &declaration.inline_items {
                self.add_inline_module(
                    edge_id,
                    parent_instance_id,
                    &module_path,
                    parent_lexical_path,
                    parent_canonical_relative,
                    parent_observation_id,
                    parent_directory,
                    declaration,
                    literal_path,
                    cfg,
                    children,
                )?;
            } else {
                self.add_external_module(
                    edge_id,
                    parent_instance_id,
                    &module_path,
                    parent_observation_id,
                    parent_directory,
                    declaration,
                    literal_path,
                    cfg,
                )?;
            }
        }
        Ok(())
    }

    fn add_include_source(
        &mut self,
        owning_module_instance_id: &str,
        module_syntax_path: &str,
        including_lexical_path: &Path,
        parent_observation_id: &str,
        declaration: &IncludeDeclarationV1,
    ) -> Result<(), ModuleTopologyErrorV1> {
        let source_relative = self.relative(including_lexical_path)?;
        let cfg = decide_module_cfg_v1(
            &outer_cfg_syntax(&declaration.outer_attributes),
            &self.environment,
        )?;
        if cfg.state == CfgDecisionStateV1::Unknown {
            return Err(ModuleTopologyErrorV1::UnknownCfg {
                module: format!("include@{source_relative}"),
            });
        }
        let include_edge_id = self.next_include_edge_id();
        let parent_observation = self
            .observations
            .iter()
            .find(|row| row.source_observation_id == parent_observation_id)
            .ok_or(ModuleTopologyErrorV1::WorkspaceEvidenceDrift)?;
        let parent_include_edge_id = parent_observation.parent_include_edge_id.clone();
        if cfg.state == CfgDecisionStateV1::Excluded {
            self.include_edges.push(DeclaredIncludeEdgeV1 {
                include_edge_id,
                owning_module_instance_id: owning_module_instance_id.to_string(),
                parent_source_observation_id: parent_observation_id.to_string(),
                parent_include_edge_id,
                invocation_range: declaration.range,
                cfg_decision: cfg,
                literal_path: None,
                selected_source_path_workspace_relative: None,
                child_source_observation_id: None,
            });
            return Ok(());
        }

        validate_include_attributes(&source_relative, &declaration.outer_attributes)?;
        validate_active_cfg_attributes_v1(
            &format!("include@{source_relative}"),
            &declaration.outer_attributes,
            &self.environment,
        )?;
        if select_active_path_v1(
            &format!("include@{source_relative}"),
            &declaration.outer_attributes,
            &self.environment,
        )?
        .is_some()
        {
            return Err(ModuleTopologyErrorV1::UnsupportedIncludeAttribute {
                path: source_relative,
                attribute: "path".to_string(),
            });
        }
        if declaration.include_macro_ambiguity {
            return Err(ModuleTopologyErrorV1::IncludeMacroIdentityUnresolved {
                path: source_relative,
            });
        }
        let literal_path = include_literal(&source_relative, declaration)?;
        let resolved =
            resolve_include_source_v1(&self.workspace_root, including_lexical_path, &literal_path)?;
        if self.canonical_ancestry.contains(&resolved.canonical_path) {
            return Err(ModuleTopologyErrorV1::CanonicalCycle {
                path: self.relative(&resolved.canonical_path)?,
            });
        }
        let selected_relative = self.relative(&resolved.lexical_path)?;
        let source = self.read_source(&resolved.lexical_path, &resolved.canonical_path)?;
        let parsed = parse_included_module_source_v1(&selected_relative, &source)?;
        let observation_id = self.add_source_observation(
            owning_module_instance_id,
            &resolved.lexical_path,
            &resolved.canonical_path,
            module_syntax_path,
            &source,
            Some(include_edge_id.clone()),
        )?;
        self.include_edges.push(DeclaredIncludeEdgeV1 {
            include_edge_id,
            owning_module_instance_id: owning_module_instance_id.to_string(),
            parent_source_observation_id: parent_observation_id.to_string(),
            parent_include_edge_id,
            invocation_range: declaration.range,
            cfg_decision: cfg,
            literal_path: Some(literal_path),
            selected_source_path_workspace_relative: Some(selected_relative),
            child_source_observation_id: Some(observation_id.clone()),
        });
        self.canonical_ancestry.push(resolved.canonical_path);
        let canonical_relative = self.relative(self.canonical_ancestry.last().unwrap())?;
        let result = self.walk_items(
            owning_module_instance_id,
            module_syntax_path,
            &resolved.lexical_path,
            &canonical_relative,
            &observation_id,
            &resolved.directory,
            &parsed.items,
        );
        self.canonical_ancestry.pop();
        result
    }

    #[allow(clippy::too_many_arguments)]
    fn add_inline_module(
        &mut self,
        edge_id: String,
        parent_instance_id: &str,
        module_path: &str,
        parent_lexical_path: &Path,
        parent_canonical_relative: &str,
        parent_observation_id: &str,
        parent_directory: &ModuleDirectoryOwnershipV1,
        declaration: &ModuleDeclarationV1,
        literal_path: Option<String>,
        cfg: crate::project::CfgDecisionV1,
        children: &[ModulePositionItemV1],
    ) -> Result<(), ModuleTopologyErrorV1> {
        let child_id = self.next_instance_id();
        let directory =
            parent_directory.inline_child(&declaration.semantic_segment, literal_path.as_deref());
        self.edges.push(DeclaredModuleEdgeV1 {
            edge_id: edge_id.clone(),
            parent_instance_id: parent_instance_id.to_string(),
            declaration_source_observation_id: parent_observation_id.to_string(),
            declaration_range: declaration.range,
            declared_ident_syntax: declaration.ident_syntax.clone(),
            semantic_segment: declaration.semantic_segment.clone(),
            kind: ModuleEdgeKindV1::Inline,
            active_literal_path: literal_path,
            cfg_decision: cfg,
            child_instance_id: Some(child_id.clone()),
            selected_source_path_workspace_relative: None,
        });
        self.instances.push(DeclaredModuleInstanceV1 {
            instance_id: child_id.clone(),
            parent_edge_id: Some(edge_id),
            module_syntax_path: module_path.to_string(),
            kind: ModuleInstanceKindV1::Inline,
            source_path_workspace_relative: self.relative(parent_lexical_path)?,
            canonical_source_path_workspace_relative: parent_canonical_relative.to_string(),
            source_observation_id: parent_observation_id.to_string(),
            inline_body_range: declaration.inline_body_range,
        });
        self.walk_items(
            &child_id,
            module_path,
            parent_lexical_path,
            parent_canonical_relative,
            parent_observation_id,
            &directory,
            children,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn add_external_module(
        &mut self,
        edge_id: String,
        parent_instance_id: &str,
        module_path: &str,
        parent_observation_id: &str,
        parent_directory: &ModuleDirectoryOwnershipV1,
        declaration: &ModuleDeclarationV1,
        literal_path: Option<String>,
        cfg: crate::project::CfgDecisionV1,
    ) -> Result<(), ModuleTopologyErrorV1> {
        let resolved = resolve_external_module_v1(
            &self.workspace_root,
            parent_directory,
            &declaration.semantic_segment,
            literal_path.as_deref(),
        )?;
        if self.canonical_ancestry.contains(&resolved.canonical_path) {
            return Err(ModuleTopologyErrorV1::CanonicalCycle {
                path: self.relative(&resolved.canonical_path)?,
            });
        }
        let child_id = self.next_instance_id();
        let selected_relative = self.relative(&resolved.lexical_path)?;
        let source = self.read_source(&resolved.lexical_path, &resolved.canonical_path)?;
        let parsed = parse_module_source_v1(
            &selected_relative,
            &source,
            declaration.include_macro_ambiguity,
        )?;
        let observation_id = self.add_source_observation(
            &child_id,
            &resolved.lexical_path,
            &resolved.canonical_path,
            module_path,
            &source,
            None,
        )?;
        self.edges.push(DeclaredModuleEdgeV1 {
            edge_id: edge_id.clone(),
            parent_instance_id: parent_instance_id.to_string(),
            declaration_source_observation_id: parent_observation_id.to_string(),
            declaration_range: declaration.range,
            declared_ident_syntax: declaration.ident_syntax.clone(),
            semantic_segment: declaration.semantic_segment.clone(),
            kind: if literal_path.is_some() {
                ModuleEdgeKindV1::LiteralPath
            } else {
                ModuleEdgeKindV1::Ordinary
            },
            active_literal_path: literal_path,
            cfg_decision: cfg,
            child_instance_id: Some(child_id.clone()),
            selected_source_path_workspace_relative: Some(selected_relative.clone()),
        });
        let canonical_relative = self.relative(&resolved.canonical_path)?;
        self.instances.push(DeclaredModuleInstanceV1 {
            instance_id: child_id.clone(),
            parent_edge_id: Some(edge_id),
            module_syntax_path: module_path.to_string(),
            kind: resolved.kind,
            source_path_workspace_relative: selected_relative,
            canonical_source_path_workspace_relative: canonical_relative.clone(),
            source_observation_id: observation_id.clone(),
            inline_body_range: None,
        });
        self.canonical_ancestry.push(resolved.canonical_path);
        let result = self.walk_items(
            &child_id,
            module_path,
            &resolved.lexical_path,
            &canonical_relative,
            &observation_id,
            &resolved.directory,
            &parsed.items,
        );
        self.canonical_ancestry.pop();
        result
    }

    fn add_source_observation(
        &mut self,
        module_instance_id: &str,
        lexical_path: &Path,
        canonical_path: &Path,
        module_path: &str,
        source: &str,
        parent_include_edge_id: Option<String>,
    ) -> Result<String, ModuleTopologyErrorV1> {
        let id = format!("source:{}", self.observations.len());
        let lexical_relative = self.relative(lexical_path)?;
        let canonical_relative = self.relative(canonical_path)?;
        let topology = extract_single_file_source(&lexical_relative, module_path, source).map_err(
            |error| ModuleTopologyErrorV1::Parse {
                path: lexical_relative.clone(),
                detail: error.to_string(),
            },
        )?;
        self.observations.push(ModuleSourceObservationV1 {
            source_observation_id: id.clone(),
            module_instance_id: module_instance_id.to_string(),
            parent_include_edge_id,
            source_path_workspace_relative: lexical_relative,
            canonical_source_path_workspace_relative: canonical_relative,
            topology,
        });
        Ok(id)
    }

    fn read_source(
        &mut self,
        lexical_path: &Path,
        canonical_path: &Path,
    ) -> Result<String, ModuleTopologyErrorV1> {
        let source = fs::read_to_string(canonical_path).map_err(|error| {
            ModuleTopologyErrorV1::SourceRead {
                path: self
                    .relative(lexical_path)
                    .unwrap_or_else(|_| lexical_path.to_string_lossy().into_owned()),
                detail: error.kind().to_string(),
            }
        })?;
        if let Some(previous) = self.source_snapshots.get(canonical_path) {
            if previous != &source {
                return Err(ModuleTopologyErrorV1::SourceChanged {
                    path: self.relative(canonical_path)?,
                });
            }
        } else {
            self.source_snapshots
                .insert(canonical_path.to_path_buf(), source.clone());
        }
        Ok(source)
    }

    fn verify_sources_unchanged(&self) -> Result<(), ModuleTopologyErrorV1> {
        for (path, expected) in &self.source_snapshots {
            let actual =
                fs::read_to_string(path).map_err(|error| ModuleTopologyErrorV1::SourceRead {
                    path: self
                        .relative(path)
                        .unwrap_or_else(|_| path.to_string_lossy().into_owned()),
                    detail: error.kind().to_string(),
                })?;
            if &actual != expected {
                return Err(ModuleTopologyErrorV1::SourceChanged {
                    path: self.relative(path)?,
                });
            }
        }
        Ok(())
    }

    fn finish(self) -> Result<DeclaredModuleTopologyV1, ModuleTopologyErrorV1> {
        let included_modules = self
            .edges
            .iter()
            .filter(|edge| edge.cfg_decision.state == CfgDecisionStateV1::Included)
            .count();
        let external_module_observations = self
            .edges
            .iter()
            .filter(|edge| {
                edge.cfg_decision.state == CfgDecisionStateV1::Included
                    && edge.kind != ModuleEdgeKindV1::Inline
            })
            .count();
        let included_source_observations = self
            .include_edges
            .iter()
            .filter(|edge| edge.cfg_decision.state == CfgDecisionStateV1::Included)
            .count();
        if self.instances.len() != 1 + included_modules
            || self.observations.len()
                != 1 + external_module_observations + included_source_observations
            || self.edges.iter().any(|edge| {
                (edge.cfg_decision.state == CfgDecisionStateV1::Included)
                    != edge.child_instance_id.is_some()
            })
            || self.edges.iter().any(|edge| {
                !self.observations.iter().any(|observation| {
                    observation.source_observation_id == edge.declaration_source_observation_id
                })
            })
            || self.include_edges.iter().any(|edge| {
                let included = edge.cfg_decision.state == CfgDecisionStateV1::Included;
                let complete = edge.literal_path.is_some()
                    && edge.selected_source_path_workspace_relative.is_some()
                    && edge.child_source_observation_id.is_some();
                let parent = self.observations.iter().find(|observation| {
                    observation.source_observation_id == edge.parent_source_observation_id
                });
                let child = edge
                    .child_source_observation_id
                    .as_ref()
                    .and_then(|child_id| {
                        self.observations
                            .iter()
                            .find(|observation| &observation.source_observation_id == child_id)
                    });
                included != complete
                    || !self
                        .instances
                        .iter()
                        .any(|instance| instance.instance_id == edge.owning_module_instance_id)
                    || parent.is_none()
                    || parent.is_some_and(|observation| {
                        observation.parent_include_edge_id != edge.parent_include_edge_id
                    })
                    || child.is_some_and(|observation| {
                        observation.module_instance_id != edge.owning_module_instance_id
                            || observation.parent_include_edge_id.as_deref()
                                != Some(edge.include_edge_id.as_str())
                    })
                    || (included && child.is_none())
            })
        {
            return Err(ModuleTopologyErrorV1::WorkspaceEvidenceDrift);
        }
        Ok(DeclaredModuleTopologyV1 {
            schema: DECLARED_MODULE_TOPOLOGY_SCHEMA_V1,
            schema_version: 1,
            profile_id: self.profile_id,
            package_key: self.package_key,
            target_key: self.target_key,
            root_instance_id: "module:0".to_string(),
            module_instances: self.instances.into_boxed_slice(),
            module_edges: self.edges.into_boxed_slice(),
            include_edges: self.include_edges.into_boxed_slice(),
            source_observations: self.observations.into_boxed_slice(),
        })
    }

    fn next_instance_id(&self) -> String {
        format!("module:{}", self.instances.len())
    }

    fn next_edge_id(&self) -> String {
        format!("edge:{}", self.edges.len())
    }

    fn next_include_edge_id(&self) -> String {
        format!("include:{}", self.include_edges.len())
    }

    fn relative(&self, path: &Path) -> Result<String, ModuleTopologyErrorV1> {
        workspace_relative(&self.workspace_root, path)
    }
}
