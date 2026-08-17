//! Caller-zero logical consumer for the sealed S6C output product.
//!
//! This owner only checks and acknowledges an already sealed logical view.  It
//! does not issue Facts, Recipe, Join, MIR, or physical meaning.

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1};
use super::join_sig::{
    LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitTargetV2, LoopJoinEdgeRoleV1,
};
use super::s6c_scan_with_init_joinir::S6CLogicalCallRoleV1;
use super::s6c_scan_with_init_joinir_output::{
    S6CLogicalOutputDomainCountsV1, S6CScanWithInitLogicalOutputRefV1,
    VerifiedS6CScanWithInitLogicalOutputV1,
};
use super::s6c_scan_with_init_joinir_output_rows::S6CLogicalOutputRejectV1;
use super::schema_v2::LoopValueClassV2;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::resolved_semantics::ResolvedLoopPlacementV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalConsumerResultV1 {
    Consumed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalConsumerRejectV1 {
    Output(S6CLogicalOutputRejectV1),
}

pub(crate) fn consume_s6c_scan_with_init_logical_output_v1(
    output: &VerifiedS6CScanWithInitLogicalOutputV1,
) -> Result<S6CLogicalConsumerResultV1, S6CLogicalConsumerRejectV1> {
    output
        .try_with_output(|view| validate_view(view))
        .map_err(S6CLogicalConsumerRejectV1::Output)
}

fn validate_view<'rows, 'product>(
    view: S6CScanWithInitLogicalOutputRefV1<'rows, 'product>,
) -> Result<S6CLogicalConsumerResultV1, S6CLogicalOutputRejectV1> {
    require_domains(view.domains())?;

    let calls = view.calls();
    if calls.len() != 2 {
        return Err(S6CLogicalOutputRejectV1::Call("consumer call count"));
    }
    let length = calls.length();
    let substring = calls.substring();
    if length.role() != S6CLogicalCallRoleV1::Length
        || length.source().role() != S6CLogicalCallRoleV1::Length
        || length.source().operation() != CoreMethodOp::StringLen
        || length.source().arity() != 0
        || length.source().placement() != ResolvedLoopPlacementV1::Condition
    {
        return Err(S6CLogicalOutputRejectV1::Call("Length parity"));
    }
    if substring.role() != S6CLogicalCallRoleV1::Substring
        || substring.source().role() != S6CLogicalCallRoleV1::Substring
        || substring.source().operation() != CoreMethodOp::StringSubstring
        || substring.source().arity() != 2
        || substring.source().placement() != ResolvedLoopPlacementV1::Body
    {
        return Err(S6CLogicalOutputRejectV1::Call("Substring parity"));
    }

    let transfer = view.logical_transfer();
    if transfer.branches().len() != 1 || transfer.summary_transfers().len() != 1 {
        return Err(S6CLogicalOutputRejectV1::Control("transfer cardinality"));
    }
    let backedges = transfer
        .boundaries()
        .iter()
        .filter(|row| row.role == LoopJoinEdgeRoleV1::Backedge)
        .count();
    if backedges != 1 {
        return Err(S6CLogicalOutputRejectV1::Control("backedge cardinality"));
    }
    if transfer.summary_transfers()[0].role != LoopJoinEdgeRoleV1::Return {
        return Err(S6CLogicalOutputRejectV1::Control("return summary"));
    }
    if transfer.after().class() != LoopValueClassV2::I64 {
        return Err(S6CLogicalOutputRejectV1::Control("After class"));
    }

    let branch = transfer.branches()[0];
    if !matches!(
        branch.then_arm,
        LoopJoinBranchArmTransferRefV2::Exit(exit)
            if exit.role == LoopJoinEdgeRoleV1::Return
                && exit.target == LoopJoinBranchExitTargetV2::FunctionExit
    ) {
        return Err(S6CLogicalOutputRejectV1::Control("then Return"));
    }
    if !matches!(
        branch.else_arm,
        LoopJoinBranchArmTransferRefV2::Fallthrough { continuation, .. }
            if continuation.block == LoopBlockKeyV1::new(1)
                && continuation.item == LoopItemKeyV1::new(11)
    ) {
        return Err(S6CLogicalOutputRejectV1::Control("else fallthrough"));
    }

    Ok(S6CLogicalConsumerResultV1::Consumed)
}

fn require_domains(
    domains: S6CLogicalOutputDomainCountsV1,
) -> Result<(), S6CLogicalOutputRejectV1> {
    domains
        .is_exact_s6c()
        .then_some(())
        .ok_or(S6CLogicalOutputRejectV1::Domain("consumer domains"))
}

#[cfg(test)]
mod tests {
    use super::require_domains;
    use crate::mir::loop_recipe_contract::s6c_scan_with_init_joinir_output::S6CLogicalOutputDomainCountsV1;

    #[test]
    fn consumer_rejects_domain_drift_before_consumed_terminal() {
        let domains = S6CLogicalOutputDomainCountsV1::from_test(1, 3, 1, 3, 14, 15, 1, 1);
        assert!(require_domains(domains).is_err());
    }
}
