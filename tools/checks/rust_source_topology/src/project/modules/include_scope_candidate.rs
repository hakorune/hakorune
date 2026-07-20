//! INCLUDE-SCOPE0-P0's test-only shared-CFG scope observer.
//!
//! It consumes only existing content-draft, included-fragment parser, and
//! declaration-range products. It is neither a production declaration issuer
//! nor a second syntax-parser, CFG evaluator, or source-range authority.

use syn::{Attribute, Item, ItemMacro, UseTree};

use crate::project::{CfgDecisionStateV1, CfgEvaluationEnvironmentV1};

use super::cfg_gate::decide_module_cfg_stream_v1;
use super::content_draft::{
    classify_module_content_draft_v1, parse_module_content_draft_v1,
    ClassifiedModuleContentDraftV1,
};
use super::content_gate::{ModuleContentCandidateIdV1, ModuleContentDefiningSurfaceV1};
use super::declarations::{
    collect_item_outer_topology_rows_v1, direct_item_source_range_v1,
    parse_included_direct_items_v1,
};
use super::include_scope::{
    IncludeScopeLanesV1, IncludeScopeSyntaxEvidenceV1, ModuleLocalIncludeNameLaneV1,
    TextualIncludeMacroLaneV1,
};
use super::ModuleTopologyErrorV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScopeChildKindV1 {
    External,
    Inline,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScopeChildEntryObservationV1 {
    kind: ScopeChildKindV1,
    scope: IncludeScopeLanesV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IncludeInvocationObservationV1 {
    scope: IncludeScopeLanesV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IncludeScopeTraceV1 {
    final_scope: IncludeScopeLanesV1,
    child_entries: Box<[ScopeChildEntryObservationV1]>,
    include_invocations: Box<[IncludeInvocationObservationV1]>,
}

struct IncludeScopeContentTraceV1 {
    scope_scan_count: usize,
    trace: Option<IncludeScopeTraceV1>,
}

fn observe_file_content_scope_v1(
    source_path: &str,
    source: &str,
    initial_scope: IncludeScopeLanesV1,
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<IncludeScopeContentTraceV1, ModuleTopologyErrorV1> {
    let draft = parse_module_content_draft_v1(
        ModuleContentCandidateIdV1::Root,
        ModuleContentDefiningSurfaceV1::SourceFile {
            source_path_workspace_relative: source_path.to_string(),
            content_digest: crate::project::fingerprint::sha256_bytes(source.as_bytes()),
        },
        source_path,
        source,
    )
    .map_err(super::content_draft::ModuleContentDraftErrorV1::into_topology_error)?;
    match classify_module_content_draft_v1(draft, environment)
        .map_err(super::content_draft::ModuleContentDraftErrorV1::into_topology_error)?
    {
        ClassifiedModuleContentDraftV1::Excluded { .. } => Ok(IncludeScopeContentTraceV1 {
            scope_scan_count: 0,
            trace: None,
        }),
        ClassifiedModuleContentDraftV1::Included { direct_items, .. } => {
            let trace = observe_direct_items_v1(
                source_path,
                source,
                &direct_items,
                initial_scope,
                environment,
            )?;
            Ok(IncludeScopeContentTraceV1 {
                scope_scan_count: 1,
                trace: Some(trace),
            })
        }
    }
}

fn observe_same_module_include_v1(
    source_path: &str,
    included_source: &str,
    incoming_scope: IncludeScopeLanesV1,
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<IncludeScopeTraceV1, ModuleTopologyErrorV1> {
    let items = parse_included_direct_items_v1(source_path, included_source)?;
    observe_direct_items_v1(
        source_path,
        included_source,
        &items,
        incoming_scope,
        environment,
    )
}

fn observe_direct_items_v1(
    source_path: &str,
    source: &str,
    items: &[Item],
    mut scope: IncludeScopeLanesV1,
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<IncludeScopeTraceV1, ModuleTopologyErrorV1> {
    let mut child_entries = Vec::new();
    let mut include_invocations = Vec::new();

    for item in items {
        let rows = collect_item_outer_topology_rows_v1(
            source_path,
            item_attributes_v1(item),
            source,
        )?;
        let decision = decide_module_cfg_stream_v1(&rows, environment)?;
        match decision.final_state {
            CfgDecisionStateV1::Excluded => continue,
            CfgDecisionStateV1::Unknown => {
                return Err(ModuleTopologyErrorV1::UnknownCfg {
                    module: format!("include-scope@{source_path}"),
                })
            }
            CfgDecisionStateV1::Included => {}
        }

        let evidence = IncludeScopeSyntaxEvidenceV1 {
            source_range: direct_item_source_range_v1(item, source),
        };
        match item {
            Item::Use(item_use) if use_tree_may_import_include_v1(&item_use.tree) => {
                scope = scope.with_module_local_shadow(evidence);
            }
            Item::Macro(item_macro) if is_macro_rules_include_v1(item_macro) => {
                scope = scope.with_textual_macro(evidence);
            }
            Item::Macro(item_macro) if item_macro.mac.path.is_ident("include") => {
                include_invocations.push(IncludeInvocationObservationV1 {
                    scope: scope.clone(),
                });
            }
            Item::Mod(item_mod) => {
                child_entries.push(ScopeChildEntryObservationV1 {
                    kind: if item_mod.content.is_some() {
                        ScopeChildKindV1::Inline
                    } else {
                        ScopeChildKindV1::External
                    },
                    scope: scope.child_module_entry(),
                });
            }
            _ => {}
        }
    }

    Ok(IncludeScopeTraceV1 {
        final_scope: scope,
        child_entries: child_entries.into_boxed_slice(),
        include_invocations: include_invocations.into_boxed_slice(),
    })
}

fn item_attributes_v1(item: &Item) -> &[Attribute] {
    match item {
        Item::Use(item_use) => &item_use.attrs,
        Item::Macro(item_macro) => &item_macro.attrs,
        Item::Mod(item_mod) => &item_mod.attrs,
        _ => &[],
    }
}

fn is_macro_rules_include_v1(item: &ItemMacro) -> bool {
    item.mac.path.is_ident("macro_rules")
        && item
            .ident
            .as_ref()
            .is_some_and(|ident| ident == "include")
}

fn use_tree_may_import_include_v1(tree: &UseTree) -> bool {
    match tree {
        UseTree::Path(path) => use_tree_may_import_include_v1(&path.tree),
        UseTree::Name(name) => name.ident == "include",
        UseTree::Rename(rename) => rename.rename == "include",
        UseTree::Group(group) => group.items.iter().any(use_tree_may_import_include_v1),
        UseTree::Glob(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{parse_and_verify_profile_schema_v1, CfgEvaluationEnvironmentV1};

    const PROFILES: &str = include_str!("../../../tests/fixtures/profiles_v1.json");

    #[test]
    fn excluded_glob_has_no_scope_effect_but_active_and_unknown_globs_do() {
        let excluded = observe_file_content_scope_v1(
            "src/lib.rs",
            "#[cfg(any())] use crate::prelude::*;\ninclude!(\"x.inc\");\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        )
        .unwrap()
        .trace
        .unwrap();
        assert_builtin_include(&excluded.include_invocations[0].scope);

        let active = observe_file_content_scope_v1(
            "src/lib.rs",
            "use crate::prelude::*;\ninclude!(\"x.inc\");\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        )
        .unwrap()
        .trace
        .unwrap();
        assert!(matches!(
            active.include_invocations[0].scope.module_local(),
            ModuleLocalIncludeNameLaneV1::PotentiallyShadowed(_)
        ));

        let unknown = observe_file_content_scope_v1(
            "src/lib.rs",
            "#[cfg(scope_unknown)] use crate::prelude::*;\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        );
        assert!(matches!(unknown, Err(ModuleTopologyErrorV1::UnknownCfg { .. })));
    }

    #[test]
    fn parent_import_does_not_poison_inline_or_external_child_entry() {
        let trace = observe_file_content_scope_v1(
            "src/lib.rs",
            "use crate::prelude::*;\nmod external;\nmod inline {}\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        )
        .unwrap()
        .trace
        .unwrap();
        assert_eq!(trace.child_entries.len(), 2);
        assert_eq!(trace.child_entries[0].kind, ScopeChildKindV1::External);
        assert_eq!(trace.child_entries[1].kind, ScopeChildKindV1::Inline);
        for child in &trace.child_entries {
            let child_include = observe_same_module_include_v1(
                "src/child.rs",
                "include!(\"part.inc\");\n",
                child.scope.clone(),
                &environment(),
            )
            .unwrap();
            assert_builtin_include(&child_include.include_invocations[0].scope);
        }
    }

    #[test]
    fn textual_macro_visibility_is_source_ordered_and_inherited_by_children() {
        let before_child = observe_file_content_scope_v1(
            "src/lib.rs",
            "macro_rules! include { ($path:literal) => {}; }\nmod child;\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        )
        .unwrap()
        .trace
        .unwrap();
        assert!(matches!(
            before_child.child_entries[0].scope.textual(),
            TextualIncludeMacroLaneV1::UserMacroVisible(_)
        ));
        let poisoned_child = observe_same_module_include_v1(
            "src/child.rs",
            "include!(\"part.inc\");\n",
            before_child.child_entries[0].scope.clone(),
            &environment(),
        )
        .unwrap();
        assert!(matches!(
            poisoned_child.include_invocations[0].scope.textual(),
            TextualIncludeMacroLaneV1::UserMacroVisible(_)
        ));

        let after_child = observe_file_content_scope_v1(
            "src/lib.rs",
            "mod child;\nmacro_rules! include { ($path:literal) => {}; }\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        )
        .unwrap()
        .trace
        .unwrap();
        let clean_child = observe_same_module_include_v1(
            "src/child.rs",
            "include!(\"part.inc\");\n",
            after_child.child_entries[0].scope.clone(),
            &environment(),
        )
        .unwrap();
        assert_builtin_include(&clean_child.include_invocations[0].scope);
    }

    #[test]
    fn included_source_scope_returns_to_following_sibling() {
        let before_include = observe_file_content_scope_v1(
            "src/lib.rs",
            "include!(\"part.inc\");\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        )
        .unwrap()
        .trace
        .unwrap();
        let included = observe_same_module_include_v1(
            "src/part.inc",
            "macro_rules! include { ($path:literal) => {}; }\n",
            before_include.final_scope,
            &environment(),
        )
        .unwrap();
        let following = observe_file_content_scope_v1(
            "src/lib.rs",
            "include!(\"later.inc\");\n",
            included.final_scope,
            &environment(),
        )
        .unwrap()
        .trace
        .unwrap();
        assert!(matches!(
            following.include_invocations[0].scope.textual(),
            TextualIncludeMacroLaneV1::UserMacroVisible(_)
        ));
    }

    #[test]
    fn excluded_content_performs_zero_scope_scans() {
        let trace = observe_file_content_scope_v1(
            "src/lib.rs",
            "#![cfg(any())]\nuse crate::prelude::*;\nmacro_rules! include { ($path:literal) => {}; }\n",
            IncludeScopeLanesV1::root(),
            &environment(),
        )
        .unwrap();
        assert_eq!(trace.scope_scan_count, 0);
        assert!(trace.trace.is_none());
    }

    fn assert_builtin_include(scope: &IncludeScopeLanesV1) {
        assert!(matches!(
            scope.module_local(),
            ModuleLocalIncludeNameLaneV1::BuiltinUnambiguous
        ));
        assert!(matches!(
            scope.textual(),
            TextualIncludeMacroLaneV1::BuiltinVisible
        ));
    }

    fn environment() -> CfgEvaluationEnvironmentV1 {
        let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
        let profile = schema
            .profiles
            .iter()
            .find(|profile| profile.profile_id == "host-default-dev")
            .unwrap();
        CfgEvaluationEnvironmentV1::from_profile_input(profile)
    }
}
