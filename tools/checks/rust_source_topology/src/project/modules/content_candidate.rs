//! CONTENTCFG0-P0's test-only content-candidate observer.
//!
//! This is intentionally not a module traversal.  It creates neither module
//! instances nor topology edges.  It only follows outer-Included declarations
//! far enough to classify the next source or inline content surface.  An
//! Excluded content draft stops before its descendants are parsed.

use std::fs;
use std::path::{Path, PathBuf};

use crate::project::cargo::CargoDeclaredUnitProcessEvidenceV1;
use crate::project::{
    cfg_environment_from_declared_unit_evidence_v1, CfgDecisionStateV1, CfgEvaluationEnvironmentV1,
};

use super::cfg_gate::{
    decide_module_cfg_stream_v1, select_active_path_v1, validate_selected_cfg_attributes_v1,
};
use super::content_draft::{
    classify_module_content_draft_v1, parse_module_content_draft_v1,
    ClassifiedModuleContentDraftV1, ModuleContentDraftErrorV1,
};
use super::declarations::{
    parse_module_source_v1, validate_module_attributes, ModuleDeclarationV1, ModulePositionItemV1,
};
use super::path_resolution::{
    canonical_regular_file, normalize_inside_workspace, resolve_external_module_v1,
    workspace_relative, ModuleDirectoryOwnershipV1,
};
use super::{ModuleContentCandidateIdV1, ModuleContentDefiningSurfaceV1, ModuleTopologyErrorV1};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContentCandidateObservationV1 {
    source_path_workspace_relative: String,
    inner_cfg_state: CfgDecisionStateV1,
}

#[derive(Debug)]
enum ContentCandidateObservationErrorV1 {
    Module(ModuleTopologyErrorV1),
    Draft(ModuleContentDraftErrorV1),
}

impl std::fmt::Display for ContentCandidateObservationErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Module(error) => error.fmt(formatter),
            Self::Draft(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ContentCandidateObservationErrorV1 {}

impl From<ModuleTopologyErrorV1> for ContentCandidateObservationErrorV1 {
    fn from(error: ModuleTopologyErrorV1) -> Self {
        Self::Module(error)
    }
}

impl From<ModuleContentDraftErrorV1> for ContentCandidateObservationErrorV1 {
    fn from(error: ModuleContentDraftErrorV1) -> Self {
        Self::Draft(error)
    }
}

struct ContentCandidateObserverV1 {
    workspace_root: PathBuf,
    environment: CfgEvaluationEnvironmentV1,
    observations: Vec<ContentCandidateObservationV1>,
    next_edge_ordinal: usize,
    canonical_ancestry: Vec<PathBuf>,
}

impl ContentCandidateObserverV1 {
    fn new(workspace_root: PathBuf, environment: CfgEvaluationEnvironmentV1) -> Self {
        Self {
            workspace_root,
            environment,
            observations: Vec::new(),
            next_edge_ordinal: 0,
            canonical_ancestry: Vec::new(),
        }
    }

    fn observe_root(
        &mut self,
        lexical_path: PathBuf,
        canonical_path: PathBuf,
    ) -> Result<(), ContentCandidateObservationErrorV1> {
        let directory = ModuleDirectoryOwnershipV1::root(&lexical_path)?;
        self.canonical_ancestry.push(canonical_path.clone());
        let result = self.observe_file_candidate(
            ModuleContentCandidateIdV1::Root,
            lexical_path,
            canonical_path,
            directory,
            false,
        );
        self.canonical_ancestry.pop();
        result
    }

    #[allow(clippy::too_many_arguments)]
    fn observe_file_candidate(
        &mut self,
        candidate_id: ModuleContentCandidateIdV1,
        lexical_path: PathBuf,
        canonical_path: PathBuf,
        directory: ModuleDirectoryOwnershipV1,
        inherited_include_macro_ambiguity: bool,
    ) -> Result<(), ContentCandidateObservationErrorV1> {
        let relative = workspace_relative(&self.workspace_root, &lexical_path)?;
        let source = self.read_source(&lexical_path, &canonical_path)?;
        let surface = ModuleContentDefiningSurfaceV1::SourceFile {
            source_path_workspace_relative: relative.clone(),
            content_digest: crate::project::fingerprint::sha256_bytes(source.as_bytes()),
        };
        let draft = parse_module_content_draft_v1(candidate_id, surface, &relative, &source)?;
        match classify_module_content_draft_v1(draft, &self.environment)? {
            ClassifiedModuleContentDraftV1::Excluded { gate } => {
                self.record(
                    &relative,
                    gate.cfg_decision.final_state,
                    gate.inner_cfg_sites.len(),
                );
                Ok(())
            }
            ClassifiedModuleContentDraftV1::Included { gate, .. } => {
                self.record(
                    &relative,
                    gate.cfg_decision.final_state,
                    gate.inner_cfg_sites.len(),
                );
                let parsed =
                    parse_module_source_v1(&relative, &source, inherited_include_macro_ambiguity)?;
                self.walk_items(&lexical_path, &directory, &source, &parsed.items)
            }
        }
    }

    fn walk_items(
        &mut self,
        parent_lexical_path: &Path,
        parent_directory: &ModuleDirectoryOwnershipV1,
        parent_source: &str,
        items: &[ModulePositionItemV1],
    ) -> Result<(), ContentCandidateObservationErrorV1> {
        for item in items {
            let ModulePositionItemV1::Module(declaration) = item else {
                continue;
            };
            let outer =
                decide_module_cfg_stream_v1(&declaration.outer_topology_rows, &self.environment)?;
            match outer.final_state {
                CfgDecisionStateV1::Excluded => continue,
                CfgDecisionStateV1::Unknown => {
                    return Err(ModuleTopologyErrorV1::UnknownCfg {
                        module: declaration.semantic_segment.clone(),
                    }
                    .into())
                }
                CfgDecisionStateV1::Included => {}
            }
            validate_module_attributes(
                &declaration.semantic_segment,
                &declaration.outer_attributes,
            )?;
            validate_selected_cfg_attributes_v1(&declaration.semantic_segment, &outer)?;
            let literal_path = select_active_path_v1(&declaration.semantic_segment, &outer)?;
            let candidate_id = self.next_edge_candidate();
            if let Some(children) = &declaration.inline_items {
                self.observe_inline_candidate(
                    candidate_id,
                    declaration,
                    parent_lexical_path,
                    parent_directory,
                    parent_source,
                    literal_path,
                    children,
                )?;
            } else {
                self.observe_external_candidate(
                    candidate_id,
                    declaration,
                    parent_directory,
                    literal_path,
                )?;
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn observe_inline_candidate(
        &mut self,
        candidate_id: ModuleContentCandidateIdV1,
        declaration: &ModuleDeclarationV1,
        parent_lexical_path: &Path,
        parent_directory: &ModuleDirectoryOwnershipV1,
        parent_source: &str,
        literal_path: Option<String>,
        children: &[ModulePositionItemV1],
    ) -> Result<(), ContentCandidateObservationErrorV1> {
        let body_range = declaration
            .inline_body_range
            .ok_or(ModuleTopologyErrorV1::WorkspaceEvidenceDrift)?;
        let braced_body = parent_source
            .get(body_range.byte_start..body_range.byte_end)
            .ok_or(ModuleTopologyErrorV1::AttributeRangeInvalid {
                path: workspace_relative(&self.workspace_root, parent_lexical_path)?,
                byte_start: body_range.byte_start,
                byte_end: body_range.byte_end,
            })?;
        let body = braced_body
            .strip_prefix('{')
            .and_then(|source| source.strip_suffix('}'))
            .ok_or(ModuleTopologyErrorV1::WorkspaceEvidenceDrift)?;
        let parent_relative = workspace_relative(&self.workspace_root, parent_lexical_path)?;
        let surface = ModuleContentDefiningSurfaceV1::InlineBody {
            parent_source_observation_id: format!(
                "proof-source:{parent_relative}:{}..{}",
                body_range.byte_start, body_range.byte_end
            ),
            body_range,
        };
        let draft = parse_module_content_draft_v1(candidate_id, surface, &parent_relative, body)?;
        let directory =
            parent_directory.inline_child(&declaration.semantic_segment, literal_path.as_deref());
        match classify_module_content_draft_v1(draft, &self.environment)? {
            ClassifiedModuleContentDraftV1::Excluded { gate } => {
                self.record(
                    &parent_relative,
                    gate.cfg_decision.final_state,
                    gate.inner_cfg_sites.len(),
                );
                Ok(())
            }
            ClassifiedModuleContentDraftV1::Included { gate, .. } => {
                self.record(
                    &parent_relative,
                    gate.cfg_decision.final_state,
                    gate.inner_cfg_sites.len(),
                );
                self.walk_items(parent_lexical_path, &directory, body, children)
            }
        }
    }

    fn observe_external_candidate(
        &mut self,
        candidate_id: ModuleContentCandidateIdV1,
        declaration: &ModuleDeclarationV1,
        parent_directory: &ModuleDirectoryOwnershipV1,
        literal_path: Option<String>,
    ) -> Result<(), ContentCandidateObservationErrorV1> {
        let resolved = resolve_external_module_v1(
            &self.workspace_root,
            parent_directory,
            &declaration.semantic_segment,
            literal_path.as_deref(),
        )?;
        if self.canonical_ancestry.contains(&resolved.canonical_path) {
            return Err(ModuleTopologyErrorV1::CanonicalCycle {
                path: workspace_relative(&self.workspace_root, &resolved.canonical_path)?,
            }
            .into());
        }
        self.canonical_ancestry
            .push(resolved.canonical_path.clone());
        let result = self.observe_file_candidate(
            candidate_id,
            resolved.lexical_path,
            resolved.canonical_path,
            resolved.directory,
            declaration.include_macro_ambiguity,
        );
        self.canonical_ancestry.pop();
        result
    }

    fn next_edge_candidate(&mut self) -> ModuleContentCandidateIdV1 {
        let edge_id = format!("content-proof-edge:{}", self.next_edge_ordinal);
        self.next_edge_ordinal += 1;
        ModuleContentCandidateIdV1::ModuleEdge { edge_id }
    }

    fn read_source(
        &self,
        lexical_path: &Path,
        canonical_path: &Path,
    ) -> Result<String, ContentCandidateObservationErrorV1> {
        fs::read_to_string(canonical_path).map_err(|error| {
            ModuleTopologyErrorV1::SourceRead {
                path: workspace_relative(&self.workspace_root, lexical_path)
                    .unwrap_or_else(|_| lexical_path.to_string_lossy().into_owned()),
                detail: error.kind().to_string(),
            }
            .into()
        })
    }

    fn record(&mut self, path: &str, state: CfgDecisionStateV1, inner_site_count: usize) {
        if inner_site_count != 0 {
            self.observations.push(ContentCandidateObservationV1 {
                source_path_workspace_relative: path.to_string(),
                inner_cfg_state: state,
            });
        }
    }
}

fn observe_content_candidates_v1(
    workspace_root: &Path,
    evidence: &CargoDeclaredUnitProcessEvidenceV1,
) -> Result<Vec<ContentCandidateObservationV1>, ContentCandidateObservationErrorV1> {
    let workspace_root = workspace_root
        .canonicalize()
        .map_err(|_| ModuleTopologyErrorV1::WorkspaceRootInvalid)?;
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
    let mut observer = ContentCandidateObserverV1::new(workspace_root, environment);
    observer.observe_root(root_lexical, root_canonical)?;
    Ok(observer.observations)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::project::cargo::collect_declared_cargo_unit_process_evidence_v1;
    use crate::project::{parse_and_verify_profile_schema_v1, ValidatedBuildProfileInputV1};

    use super::*;

    const PROFILES: &str = include_str!("../../../tests/fixtures/profiles_v1.json");

    #[test]
    fn exact_six_profile_content_candidate_matrix_is_pinned() {
        let root = root_dir();
        let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
        let expected = [
            ("host-default-dev", 0),
            ("host-default-release", 0),
            ("host-llvm-harness-dev", 0),
            ("host-test-unit-default", 11),
            ("host-vm-reference-dev", 0),
            ("wasm32-default-dev", 0),
        ];
        for (profile_id, expected_count) in expected {
            let evidence = collect_declared_cargo_unit_process_evidence_v1(
                &root.join("Cargo.toml"),
                profile(&schema.profiles, profile_id),
            )
            .unwrap();
            let observations = observe_content_candidates_v1(&root, &evidence).unwrap();
            assert_eq!(observations.len(), expected_count, "profile={profile_id}");
            assert!(observations
                .iter()
                .all(|row| row.inner_cfg_state == CfgDecisionStateV1::Excluded));
        }
    }

    #[test]
    fn host_test_candidate_paths_are_exact_and_all_excluded() {
        let root = root_dir();
        let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
        let evidence = collect_declared_cargo_unit_process_evidence_v1(
            &root.join("Cargo.toml"),
            profile(&schema.profiles, "host-test-unit-default"),
        )
        .unwrap();
        let paths = observe_content_candidates_v1(&root, &evidence)
            .unwrap()
            .into_iter()
            .map(|row| row.source_path_workspace_relative)
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            vec![
                "src/mir/builder/resolved_lowering/if_tests.rs",
                "src/mir/builder/resolved_lowering/null_tests.rs",
                "src/mir/builder/resolved_lowering/parameter_tests.rs",
                "src/mir/builder/resolved_lowering/return_tests.rs",
                "src/mir/builder/resolved_lowering/void_tests.rs",
                "src/mir/compiler/acyclic_callable_module_activation_tests.rs",
                "src/mir/compiler/recursive_callable_module_activation_tests.rs",
                "src/mir/compiler/sibling_call_tests.rs",
                "src/tests/functionbox_call_tests.rs",
                "src/tests/plugin_hygiene.rs",
                "src/tests/refcell_assignment_test.rs",
            ]
        );
    }

    fn profile<'a>(
        profiles: &'a [ValidatedBuildProfileInputV1],
        profile_id: &str,
    ) -> &'a ValidatedBuildProfileInputV1 {
        profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
            .unwrap()
    }

    fn root_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .unwrap()
    }
}
