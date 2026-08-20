//! Focused recursive-child capability declarations.
//!
//! The large recursive child owner keeps the lowering implementations.  This
//! child owns only the stable port contract and its behavior-neutral ingress
//! hooks, so new capabilities do not grow the legacy owner.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::{MirBuilder, ValueId};

use super::control_flow::cleanup::CleanupExitPolicyV1;
use super::normal_script_semantic_lowering_state::{
    ScriptDirectStaticClaimTakeV1, ScriptDirectStaticClaimedRowV1,
};
use super::raw_structured_child_scope::PreparedRawChildSourceV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ScriptDirectStaticClaimIngressV1 {
    Unavailable,
    Available,
}

pub(in crate::mir::builder) trait RecursiveChildLoweringPortV1 {
    type BodyInput;
    type StatementInput;
    type ExpressionInput;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String>;

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String>;

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String>;

    /// Behavior-neutral pre-descent capability.  The default is explicitly
    /// non-consuming; only a source-backed invocation may report Available.
    fn script_direct_static_claim_ingress_v1(
        &mut self,
        _box_name: &str,
        _method: &str,
        _argument_count: usize,
    ) -> Result<ScriptDirectStaticClaimIngressV1, String> {
        Ok(ScriptDirectStaticClaimIngressV1::Unavailable)
    }

    /// Take the already-issued Script direct-static row before any receiver or
    /// argument effect.  The default is deliberately unavailable; physical
    /// policy lives in the dedicated bridge, not in this recursive port.
    fn take_script_direct_static_claim_v1(
        &mut self,
        _box_name: &str,
        _method: &str,
        _receiver: &ASTNode,
        _arguments: &[ASTNode],
    ) -> Result<ScriptDirectStaticClaimTakeV1, String> {
        Ok(ScriptDirectStaticClaimTakeV1::Unavailable)
    }

    /// Complete one claimed row after the physical Call and its result
    /// publication have succeeded.  Compatibility ports never receive a
    /// claimed row, so their default is a hard boundary rather than a
    /// fallback path.
    fn complete_script_direct_static_claim_v1(
        &mut self,
        _claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), String> {
        Err("[freeze:contract][script-direct-static/claim-consumer-unavailable]".to_owned())
    }

    /// Isolated test-only ports deny cleanup exits unless they explicitly
    /// provide an operation policy. Production ports must override this.
    fn cleanup_exit_policy_v1(&self) -> CleanupExitPolicyV1 {
        CleanupExitPolicyV1::default()
    }

    fn prepare_expression_child_source_v1(
        &self,
        _parent: &ASTNode,
        _role: ExprChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        Ok(PreparedRawChildSourceV1::Preserve)
    }

    fn prepare_body_child_source_v1(
        &self,
        _parent: &ASTNode,
        _role: BodyChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        Ok(PreparedRawChildSourceV1::Preserve)
    }

    fn prepare_body_statement_source_v1(
        &self,
        _statement: &ASTNode,
        _index: usize,
    ) -> Result<PreparedRawChildSourceV1, String> {
        Ok(PreparedRawChildSourceV1::Preserve)
    }

    fn with_prepared_child_source_v1<R>(
        &mut self,
        _source: PreparedRawChildSourceV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        execute(self)
    }

    fn with_call_argument_source_v1<R>(
        &mut self,
        _index: usize,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        execute(self)
    }
}

pub(in crate::mir::builder) trait RawAstChildLoweringPortV1:
    RecursiveChildLoweringPortV1<
    BodyInput = Vec<ASTNode>,
    StatementInput = ASTNode,
    ExpressionInput = ASTNode,
>
{
}
