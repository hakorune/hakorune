//! Shared actual ParserBox callable-result fixture.
//!
//! This is the sole test owner for extracting `static_const_parse_add` and
//! sealing its exact 15-row activation plan. GenericLoop located-plan tests
//! borrow this fixture rather than copying source or target-site ordinals.

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{
    observe_method_calls_shadow_view_v0, FunctionSyntaxViewV1, ReceiverPolicyV1, SourceExprSiteV1,
    SourcePathSegmentV1,
};
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;
use crate::parser::NyashParser;

use super::super::{
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
};
use super::support::{
    declarations, extend_current_owner_targets, instance_key, qualified_targets, seal_with_targets,
    site, CallSiteSpecV1,
};

pub(crate) fn source() -> String {
    let parser = include_str!(concat!(
        "../../../../lang/src/compiler/parser/",
        "parser_box.hako"
    ));
    let start = parser
        .find("\n  static_const_parse_add(text, pos) {")
        .expect("actual ParserBox method start");
    let end = parser
        .find("\n  static_const_parse_mul(text, pos) {")
        .expect("actual ParserBox method end");
    format!(
        "static box ParserStringUtilsBox {{ skip_ws(text, pos) {{ return pos }} }}\nbox ParserBox {{ {}\n}}",
        &parser[start..end],
    )
}

fn instance_result_contract_source() -> String {
    let parser = include_str!(concat!(
        "../../../../lang/src/compiler/parser/",
        "parser_box.hako"
    ));
    let eval_pos = method_slice(
        parser,
        "\n  static_const_eval_pos(ret) {",
        "\n  static_const_bitand(lhs, rhs) {",
    );
    let parse_add = method_slice(
        parser,
        "\n  static_const_parse_add(text, pos) {",
        "\n  static_const_parse_mul(text, pos) {",
    );
    let string_helpers = include_str!(concat!(
        "../../../../lang/src/shared/common/",
        "string_helpers.hako"
    ));
    format!(
        "{string_helpers}\nstatic box ParserStringUtilsBox {{ skip_ws(text, pos) {{ return pos }} }}\nbox ParserBox {{{eval_pos}{parse_add}\n}}"
    )
}

fn method_slice<'source>(source: &'source str, start: &str, end: &str) -> &'source str {
    let start = source.find(start).expect("actual ParserBox method start");
    let end = source[start..]
        .find(end)
        .map(|offset| start + offset)
        .expect("actual ParserBox method end");
    &source[start..end]
}

/// Returns the exact parsed instance-method declaration used by this fixture.
///
/// The raw-prefix harness borrows this sole fixture instead of restating the
/// ParserBox source or reconstructing a method declaration from catalog data.
pub(crate) fn method_declaration_for_lowering() -> crate::ast::ASTNode {
    let root = NyashParser::parse_from_string(&source())
        .expect("actual ParserBox lowering fixture must parse");
    let crate::ast::ASTNode::Program { statements, .. } = root else {
        panic!("actual ParserBox lowering fixture root must be Program")
    };
    let methods = statements
        .into_iter()
        .find_map(|statement| match statement {
            crate::ast::ASTNode::BoxDeclaration { name, methods, .. } if name == "ParserBox" => {
                Some(methods)
            }
            _ => None,
        })
        .expect("actual ParserBox declaration");
    methods
        .into_iter()
        .map(|(_, declaration)| declaration)
        .find(|declaration| {
            matches!(
                declaration,
                crate::ast::ASTNode::FunctionDeclaration { name, params, .. }
                    if name == "static_const_parse_add" && params.len() == 2
            )
        })
        .expect("actual ParserBox.static_const_parse_add/2 declaration")
}

/// Supplies a fresh builder-owned declaration catalog for raw lowering tests.
///
/// This stays in the sole actual-source fixture so the harness never derives
/// callable declarations from a hand-written source fragment.
pub(crate) fn declaration_catalog_for_lowering() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    declarations(&source())
}

/// Exact static target candidates. Source-gate selection is decided separately.
pub(crate) fn static_target_candidate_sites() -> [SourceExprSiteV1; 2] {
    [
        site(vec![
            SourcePathSegmentV1::Body(3),
            SourcePathSegmentV1::Value,
        ]),
        site(vec![
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(5),
            SourcePathSegmentV1::Value,
        ]),
    ]
}

pub(crate) fn plan() -> VerifiedCallableResultActivationPlanV1 {
    let source = source();
    let declarations = Box::new(declarations(&source));
    let [before_loop, loop_step] = static_target_candidate_sites();
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "static_const_parse_add",
                caller_arity: 2,
                site: before_loop,
            },
            CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "static_const_parse_add",
                caller_arity: 2,
                site: loop_step,
            },
        ],
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("actual ParserBox activation rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows)
        .expect("actual ParserBox activation plan")
}

/// Builds actual source-gate inputs for disconnected proof tests only.
///
/// The callback cannot retain borrowed catalog evidence. It does not construct
/// activation rows or touch Builder, CorePlan, or caller-ledger state.
pub(crate) fn with_source_gate_inputs<R>(
    f: impl FnOnce(
        &VerifiedSameModuleCallableDeclarationCatalogV1,
        &CanonicalSameModuleCallableKeyV1,
        &[SourceExprSiteV1],
        &VerifiedSourceStaticCallTargetCatalogV1<'_>,
        &super::super::VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
    ) -> R,
) -> R {
    let source = source();
    let declarations = declarations(&source);
    let [before_loop, loop_step] = static_target_candidate_sites();
    let targets = qualified_targets(
        &declarations,
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "static_const_parse_add",
                caller_arity: 2,
                site: before_loop,
            },
            CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "static_const_parse_add",
                caller_arity: 2,
                site: loop_step,
            },
        ],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = instance_key(&declarations, "ParserBox", "static_const_parse_add", 2);
    let declaration = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "ParserBox",
            "static_const_parse_add",
            2,
        )
        .expect("actual ParserBox caller declaration");
    let sites =
        observe_method_calls_shadow_view_v0(FunctionSyntaxViewV1::from_borrowed_function_parts(
            declaration.params(),
            declaration.body(),
            ReceiverPolicyV1::DeclaredInstance,
        ))
        .expect("actual ParserBox method-call inventory")
        .into_iter()
        .map(|(site, _)| site)
        .collect::<Vec<_>>();

    f(&declarations, &caller, &sites, &targets, &results)
}

/// Supplies the exact source-only inputs for the nested instance-result proof.
///
/// This extends the sole actual ParserBox fixture without changing its
/// 15-row activation source or plan. The static result catalog contains only
/// actual StringHelpers dependency rows; the two ParserBox instance call sites
/// are deliberately absent from that static target catalog.
pub(crate) fn with_instance_result_contract_inputs<R>(
    f: impl FnOnce(
        &VerifiedSameModuleCallableDeclarationCatalogV1,
        &CanonicalSameModuleCallableKeyV1,
        &[SourceExprSiteV1; 2],
        &VerifiedSourceStaticCallTargetCatalogV1<'_>,
        &super::super::VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
    ) -> R,
) -> R {
    let source = instance_result_contract_source();
    let declarations = declarations(&source);
    let dependency_targets = qualified_targets(
        &declarations,
        &[("StringHelpers", "StringHelpers")],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "static_const_eval_pos",
            caller_arity: 1,
            site: site(vec![
                SourcePathSegmentV1::Body(3),
                SourcePathSegmentV1::Value,
            ]),
        }],
    );
    let targets = extend_current_owner_targets(
        dependency_targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "StringHelpers",
            caller_name: "to_i64",
            caller_arity: 1,
            site: site(vec![
                SourcePathSegmentV1::Body(12),
                SourcePathSegmentV1::LoopBody(2),
                SourcePathSegmentV1::Initializer(0),
            ]),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = instance_key(&declarations, "ParserBox", "static_const_parse_add", 2);
    let sites = [
        site(vec![
            SourcePathSegmentV1::Body(3),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Argument(1),
        ]),
        site(vec![
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(5),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Argument(1),
        ]),
    ];
    f(&declarations, &caller, &sites, &targets, &results)
}

/// Supplies one same-allocation source/result view for the bounded Stage-B
/// carrier correspondence proof.
///
/// Unlike `with_instance_result_contract_inputs`, this sibling includes the
/// exact outer static `Body(3).Value` target. The general call-result row for
/// that site intentionally remains absent because its nested instance
/// argument is closed by the separate bounded contract.
pub(crate) fn with_stageb_carrier_correspondence_inputs<R>(
    f: impl FnOnce(
        &VerifiedSameModuleCallableDeclarationCatalogV1,
        &CanonicalSameModuleCallableKeyV1,
        &SourceExprSiteV1,
        &[SourceExprSiteV1; 2],
        &VerifiedSourceStaticCallTargetCatalogV1<'_>,
        &super::super::VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
    ) -> R,
) -> R {
    let source = instance_result_contract_source();
    let declarations = declarations(&source);
    let outer_site = site(vec![
        SourcePathSegmentV1::Body(3),
        SourcePathSegmentV1::Value,
    ]);
    let inner_sites = [
        site(vec![
            SourcePathSegmentV1::Body(3),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Argument(1),
        ]),
        site(vec![
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(5),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Argument(1),
        ]),
    ];
    let dependency_targets = qualified_targets(
        &declarations,
        &[("StringHelpers", "StringHelpers")],
        &[
            CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "static_const_eval_pos",
                caller_arity: 1,
                site: site(vec![
                    SourcePathSegmentV1::Body(3),
                    SourcePathSegmentV1::Value,
                ]),
            },
            CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "static_const_parse_add",
                caller_arity: 2,
                site: outer_site.clone(),
            },
            CallSiteSpecV1 {
                caller_owner: "ParserBox",
                caller_name: "static_const_parse_add",
                caller_arity: 2,
                site: site(vec![
                    SourcePathSegmentV1::Body(4),
                    SourcePathSegmentV1::LoopBody(5),
                    SourcePathSegmentV1::Value,
                ]),
            },
        ],
    );
    let targets = extend_current_owner_targets(
        dependency_targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "StringHelpers",
            caller_name: "to_i64",
            caller_arity: 1,
            site: site(vec![
                SourcePathSegmentV1::Body(12),
                SourcePathSegmentV1::LoopBody(2),
                SourcePathSegmentV1::Initializer(0),
            ]),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = instance_key(&declarations, "ParserBox", "static_const_parse_add", 2);

    f(
        &declarations,
        &caller,
        &outer_site,
        &inner_sites,
        &targets,
        &results,
    )
}

pub(crate) fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
) -> CanonicalSameModuleCallableKeyV1 {
    instance_key(
        plan.declaration_catalog(),
        "ParserBox",
        "static_const_parse_add",
        2,
    )
}
