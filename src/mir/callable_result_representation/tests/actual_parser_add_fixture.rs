//! Shared actual ParserBox callable-result fixture.
//!
//! This is the sole test owner for extracting `static_const_parse_add` and
//! sealing its exact 15-row activation plan. GenericLoop located-plan tests
//! borrow this fixture rather than copying source or target-site ordinals.

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourcePathSegmentV1};

use super::super::{
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
};
use super::support::{
    declarations, instance_key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
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

pub(crate) fn selected_static_sites() -> [SourceExprSiteV1; 2] {
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
    let [before_loop, loop_step] = selected_static_sites();
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
