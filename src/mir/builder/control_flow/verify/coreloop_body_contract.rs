//! Core loop body effect contract (SSOT)
//!
//! - Leaf effects: simple, side-effecting instructions
//! - Control-flow effects: ExitIf / IfEffect (loop-only)
//! - AST helpers for generic loop normalization

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::lower::CoreEffectPlan;

pub(in crate::mir::builder) fn is_leaf_effect_plan(effect: &CoreEffectPlan) -> bool {
    matches!(
        effect,
        CoreEffectPlan::MethodCall { .. }
            | CoreEffectPlan::ExternCall { .. }
            | CoreEffectPlan::GlobalCall { .. }
            | CoreEffectPlan::ValueCall { .. }
            | CoreEffectPlan::NewBox { .. }
            | CoreEffectPlan::FieldGet { .. }
            | CoreEffectPlan::FieldSet { .. }
            | CoreEffectPlan::BinOp { .. }
            | CoreEffectPlan::Compare { .. }
            | CoreEffectPlan::Select { .. }
            | CoreEffectPlan::Const { .. }
    )
}

pub(in crate::mir::builder) fn has_control_flow_effect(effects: &[CoreEffectPlan]) -> bool {
    effects.iter().any(|effect| {
        matches!(
            effect,
            CoreEffectPlan::ExitIf { .. } | CoreEffectPlan::IfEffect { .. }
        )
    })
}

pub(in crate::mir::builder) fn is_effect_only_stmt(stmt: &ASTNode) -> bool {
    matches!(
        stmt,
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. }
    )
}
