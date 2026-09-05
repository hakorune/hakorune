//! Focused recursive-child capability declarations.
//!
//! The large recursive child owner keeps the lowering implementations.  This
//! child owns only the stable port contract and its behavior-neutral ingress
//! hooks, so new capabilities do not grow the legacy owner.

use crate::ast::ASTNode;
use crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::{MirBuilder, ValueId};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

use super::control_flow::cleanup::CleanupExitPolicyV1;
use super::normal_script_semantic_lowering_state::{
    ScriptDirectStaticClaimLedgerIssueV1, ScriptDirectStaticClaimTakeV1,
    ScriptDirectStaticClaimedRowV1,
};
use super::raw_structured_child_scope::PreparedRawChildSourceV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ScriptDirectStaticClaimIngressV1 {
    Unavailable,
    Available,
}

/// Completion-only transport for the existing claim-ledger issue.
///
/// `Unavailable` belongs to the compatibility port, while `Ledger` preserves
/// the detecting ledger variant until the physical bridge's existing String
/// diagnostic boundary. This is not a new semantic receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ScriptDirectStaticClaimCompletionErrorV1 {
    Unavailable,
    Ledger(ScriptDirectStaticClaimLedgerIssueV1),
}

impl std::fmt::Display for ScriptDirectStaticClaimCompletionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => write!(
                formatter,
                "[freeze:contract][script-direct-static/claim-consumer-unavailable]"
            ),
            Self::Ledger(issue) => write!(
                formatter,
                "[freeze:contract][script-direct-static/claim-complete] {issue:?}"
            ),
        }
    }
}

/// Result of asking the active raw invocation for the exact DeclaredInstance
/// receiver.  `Unarmed` is the explicit compatibility state; `Ready` is only
/// returned after the package-owned locator and callable state have agreed on
/// the current source site.  The hook never infers a receiver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum DeclaredInstanceReceiverIngressV1 {
    Unarmed,
    Ready {
        key: CanonicalSameModuleCallableKeyV1,
        receiver: ValueId,
    },
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
    ) -> Result<(), ScriptDirectStaticClaimCompletionErrorV1> {
        Err(ScriptDirectStaticClaimCompletionErrorV1::Unavailable)
    }

    /// Borrow the exact receiver value for a source-backed DeclaredInstance
    /// call before argument descent. Compatibility/test ports remain
    /// explicitly unarmed and therefore keep their existing route.
    fn take_declared_instance_receiver_value_v1(
        &mut self,
        _builder: &MirBuilder,
    ) -> Result<DeclaredInstanceReceiverIngressV1, String> {
        Ok(DeclaredInstanceReceiverIngressV1::Unarmed)
    }

    /// Lower the lexical `me` expression at the current exact source site.
    ///
    /// Compatibility/test ports keep the historical builder lookup.  The
    /// source-backed callable port overrides this to consume the callable
    /// ledger's exact receiver-site row, so field reads/writes inside an
    /// instance body cannot leave an unconsumed semantic product behind.
    fn lower_me_expression_v1(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        super::stmts::variable_stmt::build_me_expression(builder)
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

/// Narrow capability for the one source-backed App Main direct-call consumer.
///
/// The default is deliberately unavailable so compatibility/test ports cannot
/// accidentally publish a target.  The invocation port overrides it with the
/// package-owned affine loan; structured scopes and the semantic adapter only
/// forward the already-borrowed capability.
pub(in crate::mir::builder) trait AppMainDirectCallDispositionPortV1 {
    fn take_app_main_direct_call_disposition_v1(
        &mut self,
    ) -> Result<AppMainDirectCallDispositionRowV1, String> {
        Err("[freeze:contract][app-main-direct-call/loan-unavailable]".to_owned())
    }

    fn validate_current_call_argument_site_v1(
        &self,
        _expected: &SourceExprSiteV1,
    ) -> Result<(), String> {
        Ok(())
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
