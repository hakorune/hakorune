//! INCLUDE-SCOPE0's one production scope-stream connection.
//!
//! This module owns direct-item source order, the two-lane include scope, and
//! same-module `include!` continuation. Content issuance only threads a child
//! scope; declarations only retain syntax and outer cfg rows.

use std::path::Path;

use crate::project::CfgDecisionStateV1;

use super::cfg_gate::{
    decide_module_cfg_stream_v1, select_active_path_v1, validate_selected_cfg_attributes_v1,
};
use super::declarations::{
    include_literal, parse_included_module_source_v1, validate_include_attributes,
    validate_module_attributes, IncludeDeclarationV1, IncludeScopeDeclarationV1,
    ModulePositionItemV1,
};
use super::include_scope::{
    IncludeScopeLanesV1, IncludeScopeSyntaxEvidenceV1, ModuleLocalIncludeNameLaneV1,
    TextualIncludeMacroLaneV1,
};
use super::model::{DeclaredIncludeEdgeV1, DeclaredModuleEdgeV1, ModuleEdgeKindV1};
use super::path_resolution::{resolve_include_source_v1, ModuleDirectoryOwnershipV1};
use super::traversal::ModuleTraversalV1;
use super::ModuleTopologyErrorV1;

#[derive(Clone, Copy)]
enum IncludeScopeActionV1 {
    ModuleLocalName,
    TextualMacro,
}

impl ModuleTraversalV1 {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn walk_items(
        &mut self,
        parent_instance_id: &str,
        parent_syntax_path: &str,
        parent_lexical_path: &Path,
        parent_canonical_relative: &str,
        parent_observation_id: &str,
        parent_directory: &ModuleDirectoryOwnershipV1,
        parent_source: &str,
        items: &[ModulePositionItemV1],
        initial_scope: IncludeScopeLanesV1,
    ) -> Result<IncludeScopeLanesV1, ModuleTopologyErrorV1> {
        let source_relative = self.relative(parent_lexical_path)?;
        let mut scope = self.prepare_module_local_scope_v1(
            items,
            initial_scope,
            &source_relative,
        )?;

        for item in items {
            match item {
                ModulePositionItemV1::ModuleLocalIncludeNameScope(_) => {}
                ModulePositionItemV1::TextualIncludeMacroScope(declaration) => {
                    scope = self.apply_scope_declaration_v1(
                        scope,
                        declaration,
                        IncludeScopeActionV1::TextualMacro,
                        &source_relative,
                    )?;
                }
                ModulePositionItemV1::Include(include) => {
                    scope = self.add_include_source(
                        parent_instance_id,
                        parent_syntax_path,
                        parent_lexical_path,
                        parent_observation_id,
                        include,
                        scope,
                    )?;
                }
                ModulePositionItemV1::Module(declaration) => {
                    self.add_module_declaration_v1(
                        parent_instance_id,
                        parent_syntax_path,
                        parent_lexical_path,
                        parent_canonical_relative,
                        parent_observation_id,
                        parent_directory,
                        declaration,
                        parent_source,
                        scope.child_module_entry(),
                    )?;
                }
            }
        }
        Ok(scope)
    }

    fn prepare_module_local_scope_v1(
        &self,
        items: &[ModulePositionItemV1],
        mut scope: IncludeScopeLanesV1,
        source_relative: &str,
    ) -> Result<IncludeScopeLanesV1, ModuleTopologyErrorV1> {
        for item in items {
            let ModulePositionItemV1::ModuleLocalIncludeNameScope(declaration) = item else {
                continue;
            };
            scope = self.apply_scope_declaration_v1(
                scope,
                declaration,
                IncludeScopeActionV1::ModuleLocalName,
                source_relative,
            )?;
        }
        Ok(scope)
    }

    fn apply_scope_declaration_v1(
        &self,
        scope: IncludeScopeLanesV1,
        declaration: &IncludeScopeDeclarationV1,
        action: IncludeScopeActionV1,
        source_relative: &str,
    ) -> Result<IncludeScopeLanesV1, ModuleTopologyErrorV1> {
        let cfg = decide_module_cfg_stream_v1(&declaration.outer_topology_rows, &self.environment)?;
        if cfg.final_state == CfgDecisionStateV1::Unknown {
            return Err(ModuleTopologyErrorV1::UnknownCfg {
                module: format!("include-scope@{source_relative}"),
            });
        }
        if cfg.final_state == CfgDecisionStateV1::Excluded {
            return Ok(scope);
        }
        let subject = format!("include-scope@{source_relative}");
        validate_selected_cfg_attributes_v1(&subject, &cfg)?;
        if select_active_path_v1(&subject, &cfg)?.is_some() {
            return Err(ModuleTopologyErrorV1::UnsupportedIncludeAttribute {
                path: source_relative.to_string(),
                attribute: "path".to_string(),
            });
        }
        let evidence = IncludeScopeSyntaxEvidenceV1 {
            source_range: declaration.range,
        };
        Ok(match action {
            IncludeScopeActionV1::ModuleLocalName => scope.with_module_local_shadow(evidence),
            IncludeScopeActionV1::TextualMacro => scope.with_textual_macro(evidence),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn add_module_declaration_v1(
        &mut self,
        parent_instance_id: &str,
        parent_syntax_path: &str,
        parent_lexical_path: &Path,
        parent_canonical_relative: &str,
        parent_observation_id: &str,
        parent_directory: &ModuleDirectoryOwnershipV1,
        declaration: &super::declarations::ModuleDeclarationV1,
        parent_source: &str,
        child_scope: IncludeScopeLanesV1,
    ) -> Result<(), ModuleTopologyErrorV1> {
        let module_path = format!("{parent_syntax_path}::{}", declaration.semantic_segment);
        let cfg =
            decide_module_cfg_stream_v1(&declaration.outer_topology_rows, &self.environment)?;
        if cfg.final_state == CfgDecisionStateV1::Unknown {
            return Err(ModuleTopologyErrorV1::UnknownCfg {
                module: module_path,
            });
        }
        let edge_id = self.next_edge_id();
        if cfg.final_state == CfgDecisionStateV1::Excluded {
            self.edges.push(DeclaredModuleEdgeV1 {
                edge_id,
                parent_instance_id: parent_instance_id.to_string(),
                declaration_source_observation_id: parent_observation_id.to_string(),
                declaration_range: declaration.range,
                declared_ident_syntax: declaration.ident_syntax.clone(),
                semantic_segment: declaration.semantic_segment.clone(),
                kind: if declaration.inline_body_items.is_some() {
                    ModuleEdgeKindV1::Inline
                } else {
                    ModuleEdgeKindV1::Ordinary
                },
                active_literal_path: None,
                cfg_decision: cfg,
                content_gate: None,
                child_instance_id: None,
                selected_source_path_workspace_relative: None,
            });
            return Ok(());
        }
        validate_module_attributes(&module_path, &declaration.outer_attributes)?;
        validate_selected_cfg_attributes_v1(&module_path, &cfg)?;
        let literal_path = select_active_path_v1(&module_path, &cfg)?;
        if declaration.inline_body_items.is_some() {
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
                parent_source,
                child_scope,
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
                child_scope,
            )?;
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
        incoming_scope: IncludeScopeLanesV1,
    ) -> Result<IncludeScopeLanesV1, ModuleTopologyErrorV1> {
        let source_relative = self.relative(including_lexical_path)?;
        let cfg = decide_module_cfg_stream_v1(&declaration.outer_topology_rows, &self.environment)?;
        if cfg.final_state == CfgDecisionStateV1::Unknown {
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
        if cfg.final_state == CfgDecisionStateV1::Excluded {
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
            return Ok(incoming_scope);
        }

        validate_include_attributes(&source_relative, &declaration.outer_attributes)?;
        validate_selected_cfg_attributes_v1(&format!("include@{source_relative}"), &cfg)?;
        if select_active_path_v1(&format!("include@{source_relative}"), &cfg)?.is_some() {
            return Err(ModuleTopologyErrorV1::UnsupportedIncludeAttribute {
                path: source_relative,
                attribute: "path".to_string(),
            });
        }
        require_builtin_include_identity_v1(&incoming_scope, &source_relative)?;
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
            &source,
            &parsed.items,
            incoming_scope,
        );
        self.canonical_ancestry.pop();
        result
    }
}

fn require_builtin_include_identity_v1(
    scope: &IncludeScopeLanesV1,
    source_relative: &str,
) -> Result<(), ModuleTopologyErrorV1> {
    if matches!(
        scope.module_local(),
        ModuleLocalIncludeNameLaneV1::BuiltinUnambiguous
    ) && matches!(scope.textual(), TextualIncludeMacroLaneV1::BuiltinVisible)
    {
        return Ok(());
    }
    Err(ModuleTopologyErrorV1::IncludeMacroIdentityUnresolved {
        path: source_relative.to_string(),
    })
}
