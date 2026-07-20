//! CONTENTCFG0's gated child-instance issuance.
//!
//! This module owns the one transition from a classified module-content
//! candidate to direct topology declarations.  It never evaluates outer cfg:
//! `traversal` has already established that the parent declaration is active.

use std::path::Path;

use super::content_draft::{
    classify_module_content_draft_v1, parse_inline_module_content_draft_v1,
    parse_module_content_draft_v1, ClassifiedModuleContentDraftV1, ModuleContentDraftErrorV1,
};
use super::content_gate::{ModuleContentCandidateIdV1, ModuleContentDefiningSurfaceV1};
use super::declarations::{collect_direct_module_position_items_v1, ModuleDeclarationV1};
use super::model::{
    DeclaredModuleEdgeV1, DeclaredModuleInstanceV1, ModuleEdgeKindV1, ModuleInstanceKindV1,
};
use super::path_resolution::{resolve_external_module_v1, ModuleDirectoryOwnershipV1};
use super::traversal::ModuleTraversalV1;
use super::ModuleTopologyErrorV1;

impl ModuleTraversalV1 {
    pub(super) fn classify_file_content(
        &self,
        candidate_id: ModuleContentCandidateIdV1,
        source_path_workspace_relative: &str,
        source: &str,
    ) -> Result<ClassifiedModuleContentDraftV1, ModuleTopologyErrorV1> {
        let surface = ModuleContentDefiningSurfaceV1::SourceFile {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            content_digest: crate::project::fingerprint::sha256_bytes(source.as_bytes()),
        };
        let draft = parse_module_content_draft_v1(
            candidate_id,
            surface,
            source_path_workspace_relative,
            source,
        )
        .map_err(ModuleContentDraftErrorV1::into_topology_error)?;
        classify_module_content_draft_v1(draft, &self.environment)
            .map_err(ModuleContentDraftErrorV1::into_topology_error)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn add_inline_module(
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
        cfg: crate::project::CfgAttributeStreamDecisionV1,
        parent_source: &str,
    ) -> Result<(), ModuleTopologyErrorV1> {
        let body_range = declaration
            .inline_body_range
            .ok_or(ModuleTopologyErrorV1::WorkspaceEvidenceDrift)?;
        let parent_relative = self.relative(parent_lexical_path)?;
        let draft = parse_inline_module_content_draft_v1(
            ModuleContentCandidateIdV1::ModuleEdge {
                edge_id: edge_id.clone(),
            },
            parent_observation_id.to_string(),
            &parent_relative,
            parent_source,
            body_range,
        )
        .map_err(ModuleContentDraftErrorV1::into_topology_error)?;
        let classified = classify_module_content_draft_v1(draft, &self.environment)
            .map_err(ModuleContentDraftErrorV1::into_topology_error)?;
        let (content_gate, parsed) = match classified {
            ClassifiedModuleContentDraftV1::Excluded { gate } => (gate, None),
            ClassifiedModuleContentDraftV1::Included { gate, .. } => {
                let raw_items = declaration
                    .inline_body_items
                    .as_deref()
                    .ok_or(ModuleTopologyErrorV1::WorkspaceEvidenceDrift)?;
                let parsed = collect_direct_module_position_items_v1(
                    &parent_relative,
                    parent_source,
                    raw_items,
                    declaration.include_macro_ambiguity,
                )?;
                (gate, Some(parsed))
            }
        };
        let Some(parsed) = parsed else {
            self.edges.push(DeclaredModuleEdgeV1 {
                edge_id,
                parent_instance_id: parent_instance_id.to_string(),
                declaration_source_observation_id: parent_observation_id.to_string(),
                declaration_range: declaration.range,
                declared_ident_syntax: declaration.ident_syntax.clone(),
                semantic_segment: declaration.semantic_segment.clone(),
                kind: ModuleEdgeKindV1::Inline,
                active_literal_path: literal_path,
                cfg_decision: cfg,
                content_gate: Some(content_gate),
                child_instance_id: None,
                selected_source_path_workspace_relative: None,
            });
            return Ok(());
        };
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
            content_gate: Some(content_gate),
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
            source_observation_id: Some(parent_observation_id.to_string()),
            inline_body_range: declaration.inline_body_range,
        });
        self.walk_items(
            &child_id,
            module_path,
            parent_lexical_path,
            parent_canonical_relative,
            parent_observation_id,
            &directory,
            parent_source,
            &parsed.items,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn add_external_module(
        &mut self,
        edge_id: String,
        parent_instance_id: &str,
        module_path: &str,
        parent_observation_id: &str,
        parent_directory: &ModuleDirectoryOwnershipV1,
        declaration: &ModuleDeclarationV1,
        literal_path: Option<String>,
        cfg: crate::project::CfgAttributeStreamDecisionV1,
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
        let selected_relative = self.relative(&resolved.lexical_path)?;
        let source = self.read_source(&resolved.lexical_path, &resolved.canonical_path)?;
        let classified = self.classify_file_content(
            ModuleContentCandidateIdV1::ModuleEdge {
                edge_id: edge_id.clone(),
            },
            &selected_relative,
            &source,
        )?;
        let (content_gate, parsed) = match classified {
            ClassifiedModuleContentDraftV1::Excluded { gate } => (gate, None),
            ClassifiedModuleContentDraftV1::Included { gate, direct_items } => {
                let parsed = collect_direct_module_position_items_v1(
                    &selected_relative,
                    &source,
                    &direct_items,
                    declaration.include_macro_ambiguity,
                )?;
                (gate, Some(parsed))
            }
        };
        let kind = if literal_path.is_some() {
            ModuleEdgeKindV1::LiteralPath
        } else {
            ModuleEdgeKindV1::Ordinary
        };
        let Some(parsed) = parsed else {
            self.edges.push(DeclaredModuleEdgeV1 {
                edge_id,
                parent_instance_id: parent_instance_id.to_string(),
                declaration_source_observation_id: parent_observation_id.to_string(),
                declaration_range: declaration.range,
                declared_ident_syntax: declaration.ident_syntax.clone(),
                semantic_segment: declaration.semantic_segment.clone(),
                kind,
                active_literal_path: literal_path,
                cfg_decision: cfg,
                content_gate: Some(content_gate),
                child_instance_id: None,
                selected_source_path_workspace_relative: Some(selected_relative),
            });
            return Ok(());
        };
        let child_id = self.next_instance_id();
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
            kind,
            active_literal_path: literal_path,
            cfg_decision: cfg,
            content_gate: Some(content_gate),
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
            source_observation_id: Some(observation_id.clone()),
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
            &source,
            &parsed.items,
        );
        self.canonical_ancestry.pop();
        result
    }
}
