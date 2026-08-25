pub(super) use super::add_result_representation::prepare_coreplan_add_result_representation_v1;
pub(super) use super::common::lower_me_this_method_effect;
pub(super) use super::cond_lowering_prelude::lower_blockexpr_value_prelude_stmts;
pub(super) use super::helpers_pure_value::is_pure_value_expr;
pub(super) use super::newbox::record_newbox_metadata;
pub(super) use super::CoreEffectPlan;
pub(super) use crate::mir::builder::calls::extern_calls;
pub(super) use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
pub(super) use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, LoopPlanExpressionPortV1, RawLoopPlanExpressionPortV1,
};
pub(super) use crate::mir::builder::MirBuilder;
pub(super) use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
pub(super) use crate::mir::{BinaryOp, ConstValue, Effect, EffectMask, MirType, ValueId};
pub(super) use std::collections::BTreeMap;

impl super::PlanNormalizer {
    /// Helper: Lower value AST to (ValueId, const_effects)
    /// Returns the ValueId and any Const instructions needed to define literals
    ///
    /// phi_bindings: current logical bindings for plan lowering. `variable_map`
    /// is a fallback/cache and may temporarily contain branch-local values.
    pub(in crate::mir::builder) fn lower_value_ast(
        ast: &crate::ast::ASTNode,
        builder: &mut MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
    ) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
        let port = RawLoopPlanExpressionPortV1::new();
        Self::lower_value_input(&port, port.expr(ast), builder, phi_bindings)
    }
}

mod lower;
mod variant;
