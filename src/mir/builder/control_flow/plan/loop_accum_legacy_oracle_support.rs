//! Test-only legacy AccumConstLoop oracle.
//!
//! This module is deliberately a thin observer around the existing legacy
//! path.  It owns no recipe, PHI, or SSA vocabulary: facts, the legacy
//! `RecipeComposer`, `PlanVerifier`, and `PlanLowerer` remain the authorities.
//! A parent test module may include this file with `#[path]` when it needs a
//! semantic oracle for the portable Accum fixture.

#![cfg(test)]

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::{try_build_outcome, PlanLowerer};
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

/// Build the direct source shape represented by the AccumConstLoop fixture.
///
/// The returned AST is an oracle input only.  It intentionally uses ordinary
/// variable reads and assignments, so the legacy facts/composer path performs
/// the same source-side work as production lowering.
pub(super) fn direct_accum_source() -> (ASTNode, Vec<ASTNode>) {
    let variable = |name: &str| ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    };
    let integer = |value: i64| ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    };
    let add = |left: ASTNode, right: ASTNode| ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    };
    let assignment = |name: &str, value: ASTNode| ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    };

    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(variable("i")),
        right: Box::new(integer(3)),
        span: Span::unknown(),
    };
    let body = vec![
        assignment("sum", add(variable("sum"), integer(1))),
        assignment("i", add(variable("i"), integer(1))),
    ];
    (condition, body)
}

/// Lower one already-selected direct Accum source through the legacy oracle.
///
/// The caller owns the candidate `MirBuilder` and must prepare the ordinary
/// function/lexical scope and variable bindings before calling this helper.
/// Keeping that setup outside this function prevents the oracle from creating
/// a second builder transaction or silently changing the production boundary.
pub(super) fn lower_accum_legacy_oracle(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    func_name: &str,
) -> Result<Option<ValueId>, String> {
    let plan = prepare_accum_legacy_plan(builder, condition, body, func_name)?;
    PlanLowerer::lower(
        builder,
        plan,
        &LoopRouteContext::new(condition, body, func_name, false, false),
    )
}

/// Compose and verify the legacy CorePlan without consuming it.
///
/// This is test-only inspection support for the semantic parity digest.  The
/// production composer/lowerer remain the authorities; the helper merely lets
/// tests retain a clone before the consuming lower call.
pub(super) fn prepare_accum_legacy_plan(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    func_name: &str,
) -> Result<CorePlan, String> {
    let ctx = LoopRouteContext::new(condition, body, func_name, false, false);
    let outcome = try_build_outcome(&ctx)?;
    let facts = outcome
        .facts
        .as_ref()
        .ok_or_else(|| "legacy Accum oracle produced no canonical facts".to_string())?;
    if facts.facts.accum_const_loop().is_none() {
        return Err("legacy Accum oracle received non-Accum facts".to_string());
    }

    let plan = RecipeComposer::compose_accum_const_loop_recipe(builder, facts, &ctx)
        .map_err(|freeze| freeze.to_string())?;
    if !matches!(&plan, CorePlan::Loop(_)) {
        return Err("legacy Accum oracle produced a non-Loop CorePlan".to_string());
    }
    PlanVerifier::verify(&plan)?;
    Ok(plan)
}
