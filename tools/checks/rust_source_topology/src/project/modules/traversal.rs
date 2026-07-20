use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::extract_single_file_source;
use crate::project::cargo::CargoDeclaredUnitProcessEvidenceV1;
use crate::project::{
    cfg_environment_from_declared_unit_evidence_v1, collect_workspace_input_fingerprints_v1,
    CfgDecisionStateV1, CfgEvaluationEnvironmentV1,
};

use super::content_draft::ClassifiedModuleContentDraftV1;
use super::content_gate::{DeclaredModuleContentGateV1, ModuleContentCandidateIdV1};
use super::declarations::collect_direct_module_position_items_v1;
use super::error::ModuleTopologyErrorV1;
use super::include_scope::IncludeScopeLanesV1;
use super::model::{
    DeclaredIncludeEdgeV1, DeclaredModuleEdgeV1, DeclaredModuleInstanceV1,
    DeclaredModuleTopologyV1, ModuleEdgeKindV1, ModuleInstanceKindV1, ModuleSourceObservationV1,
    DECLARED_MODULE_TOPOLOGY_SCHEMA_V3,
};
use super::path_resolution::{
    canonical_regular_file, normalize_inside_workspace, workspace_relative, ModuleDirectoryOwnershipV1,
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
    let environment = cfg_environment_from_declared_unit_evidence_v1(evidence);
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

pub(super) struct ModuleTraversalV1 {
    pub(super) workspace_root: PathBuf,
    pub(super) profile_id: String,
    pub(super) package_key: String,
    pub(super) target_key: String,
    pub(super) environment: CfgEvaluationEnvironmentV1,
    pub(super) instances: Vec<DeclaredModuleInstanceV1>,
    pub(super) edges: Vec<DeclaredModuleEdgeV1>,
    pub(super) include_edges: Vec<DeclaredIncludeEdgeV1>,
    pub(super) observations: Vec<ModuleSourceObservationV1>,
    pub(super) source_snapshots: BTreeMap<PathBuf, String>,
    pub(super) canonical_ancestry: Vec<PathBuf>,
    pub(super) root_content_gate: Option<DeclaredModuleContentGateV1>,
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
            root_content_gate: None,
        }
    }

    fn collect_root(
        &mut self,
        lexical_path: PathBuf,
        canonical_path: PathBuf,
    ) -> Result<(), ModuleTopologyErrorV1> {
        let directory = ModuleDirectoryOwnershipV1::root(&lexical_path)?;
        let source = self.read_source(&lexical_path, &canonical_path)?;
        let relative = self.relative(&lexical_path)?;
        let root_id = self.next_instance_id();
        let classified =
            self.classify_file_content(ModuleContentCandidateIdV1::Root, &relative, &source)?;
        let (gate, parsed, observation_id) = match classified {
            ClassifiedModuleContentDraftV1::Excluded { gate } => (gate, None, None),
            ClassifiedModuleContentDraftV1::Included { gate, direct_items } => {
                let parsed = collect_direct_module_position_items_v1(
                    &relative,
                    &source,
                    &direct_items,
                )?;
                let observation_id = self.add_source_observation(
                    &root_id,
                    &lexical_path,
                    &canonical_path,
                    "crate",
                    &source,
                    None,
                )?;
                (gate, Some(parsed), Some(observation_id))
            }
        };
        self.root_content_gate = Some(gate);
        self.instances.push(DeclaredModuleInstanceV1 {
            instance_id: root_id.clone(),
            parent_edge_id: None,
            module_syntax_path: "crate".to_string(),
            kind: ModuleInstanceKindV1::Root,
            source_path_workspace_relative: relative,
            canonical_source_path_workspace_relative: self.relative(&canonical_path)?,
            source_observation_id: observation_id,
            inline_body_range: None,
        });
        let Some(parsed) = parsed else {
            return Ok(());
        };
        let root_observation_id = self.instances[0]
            .source_observation_id
            .as_deref()
            .ok_or(ModuleTopologyErrorV1::WorkspaceEvidenceDrift)?
            .to_string();
        self.canonical_ancestry.push(canonical_path);
        let result = self.walk_items(
            &root_id,
            "crate",
            &lexical_path,
            &self.relative(&self.canonical_ancestry[0])?,
            &root_observation_id,
            &directory,
            &source,
            &parsed.items,
            IncludeScopeLanesV1::root(),
        );
        self.canonical_ancestry.pop();
        result.map(|_| ())
    }

    pub(super) fn add_source_observation(
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

    pub(super) fn read_source(
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
        let root_content_gate = self
            .root_content_gate
            .ok_or(ModuleTopologyErrorV1::WorkspaceEvidenceDrift)?;
        let root_included =
            root_content_gate.cfg_decision.final_state == CfgDecisionStateV1::Included;
        let issued_modules = self
            .edges
            .iter()
            .filter(|edge| edge.child_instance_id.is_some())
            .count();
        let external_module_observations = self
            .edges
            .iter()
            .filter(|edge| {
                edge.child_instance_id.is_some() && edge.kind != ModuleEdgeKindV1::Inline
            })
            .count();
        let included_source_observations = self
            .include_edges
            .iter()
            .filter(|edge| edge.cfg_decision.final_state == CfgDecisionStateV1::Included)
            .count();
        if !matches!(
            root_content_gate.candidate_id,
            ModuleContentCandidateIdV1::Root
        ) || self.instances.len() != 1 + issued_modules
            || self.observations.len()
                != usize::from(root_included)
                    + external_module_observations
                    + included_source_observations
            || self.instances.first().is_none_or(|instance| {
                instance.kind != ModuleInstanceKindV1::Root
                    || instance.source_observation_id.is_some() != root_included
            })
            || self.edges.iter().any(|edge| {
                let outer_included = edge.cfg_decision.final_state == CfgDecisionStateV1::Included;
                let Some(gate) = &edge.content_gate else {
                    return outer_included;
                };
                let inner_included = gate.cfg_decision.final_state == CfgDecisionStateV1::Included;
                gate.candidate_id
                    != (ModuleContentCandidateIdV1::ModuleEdge {
                        edge_id: edge.edge_id.clone(),
                    })
                    || edge.child_instance_id.is_some() != (outer_included && inner_included)
                    || (!outer_included && edge.content_gate.is_some())
            })
            || self.instances.iter().any(|instance| {
                let root = instance.kind == ModuleInstanceKindV1::Root;
                (!root && instance.source_observation_id.is_none())
                    || instance
                        .source_observation_id
                        .as_ref()
                        .is_some_and(|observation_id| {
                            !self.observations.iter().any(|observation| {
                                &observation.source_observation_id == observation_id
                                    && (instance.kind == ModuleInstanceKindV1::Inline
                                        || observation.module_instance_id == instance.instance_id)
                            })
                        })
            })
            || self.edges.iter().any(|edge| {
                !self.observations.iter().any(|observation| {
                    observation.source_observation_id == edge.declaration_source_observation_id
                })
            })
            || self.include_edges.iter().any(|edge| {
                let included = edge.cfg_decision.final_state == CfgDecisionStateV1::Included;
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
            schema: DECLARED_MODULE_TOPOLOGY_SCHEMA_V3,
            schema_version: 3,
            profile_id: self.profile_id,
            package_key: self.package_key,
            target_key: self.target_key,
            root_instance_id: "module:0".to_string(),
            root_content_gate,
            module_instances: self.instances.into_boxed_slice(),
            module_edges: self.edges.into_boxed_slice(),
            include_edges: self.include_edges.into_boxed_slice(),
            source_observations: self.observations.into_boxed_slice(),
        })
    }

    pub(super) fn next_instance_id(&self) -> String {
        format!("module:{}", self.instances.len())
    }

    pub(super) fn next_edge_id(&self) -> String {
        format!("edge:{}", self.edges.len())
    }

    pub(super) fn next_include_edge_id(&self) -> String {
        format!("include:{}", self.include_edges.len())
    }

    pub(super) fn relative(&self, path: &Path) -> Result<String, ModuleTopologyErrorV1> {
        workspace_relative(&self.workspace_root, path)
    }
}
