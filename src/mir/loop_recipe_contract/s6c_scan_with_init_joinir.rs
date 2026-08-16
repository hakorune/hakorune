//! Product-first logical JOINIR input for the S6C `ScanWithInit` cohort.
//!
//! This is deliberately an input façade, not a JOINIR module.  It borrows the
//! already sealed Facts/Recipe/Join product, checks the two source-bound call
//! rows against their typed Recipe rows, and lends only logical rows to the
//! future consumer.  MIR, JoinValueSpace, physical IDs, names, and fallback
//! selection are outside this owner.

use crate::mir::callable_semantic_batch::S6CTypedInputRoleV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::CoreMethodEffectV1;
use crate::mir::resolved_semantics::{
    CoreMethodHomeAbiProfileV1, CoreMethodHomeExecutionPolicyV1, CoreMethodHomeParameterRelationV1,
    CoreMethodHomeReceiverRelationV1, CoreMethodHomeResultRelationV1, CoreMethodHomeSchemaV1,
    ResolvedLexicalRefV1, ResolvedLoopPlacementV1, ResolvedMethodCallReceiverSourceV1,
    VerifiedResolverCoreMethodCallableContractV1,
};

use super::ids::LoopValueKeyV1;
use super::join_sig::{
    LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitTargetV2, LoopJoinEdgeRoleV1,
    LoopJoinLogicalTransferViewV2,
};
use super::s6c_scan_with_init::{
    DefinedRoleV2, S6CScanWithInitRecipeProductRefV2, S6CScanWithInitRecipeRolesRefV2,
};
use super::s6c_scan_with_init_rows::{
    S6CRecipeOperationRowRefV2, S6CRecipeValueRowRefV2, S6CScanWithInitRecipeRowsRefV2,
};
use super::schema_v2::{LoopExitKindV2, LoopValueClassV2};
use super::VerifiedS6CScanWithInitRecipeProductV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalJoinInputRejectV1 {
    Domain(&'static str),
    Row(&'static str),
    Call(&'static str),
    Transfer(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalCallRoleV1 {
    Length,
    Substring,
}

/// A source-bound call paired with its fixed Recipe CallSlot row.
///
/// The resolver contract stays behind this typed view; consumers cannot
/// recover a selector, target object, or raw Recipe from it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CLogicalCallInputRefV1<'a> {
    role: S6CLogicalCallRoleV1,
    contract: &'a VerifiedResolverCoreMethodCallableContractV1,
    row: S6CRecipeOperationRowRefV2<'a>,
}

impl<'a> S6CLogicalCallInputRefV1<'a> {
    pub(crate) const fn role(self) -> S6CLogicalCallRoleV1 {
        self.role
    }

    pub(crate) const fn owner(self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.contract.owner()
    }

    pub(crate) const fn placement(self) -> ResolvedLoopPlacementV1 {
        self.contract.placement()
    }

    pub(crate) fn operation(self) -> CoreMethodOp {
        self.contract.target().row().row().op
    }

    pub(crate) fn target(
        self,
    ) -> &'a crate::mir::resolved_semantics::VerifiedCoreMethodInstanceTargetV1 {
        self.contract.target()
    }

    pub(crate) fn arity(self) -> u32 {
        self.contract.target().row().arity()
    }

    pub(crate) fn call_site(self) -> &'a crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.contract.call_site()
    }

    pub(crate) fn arguments(
        self,
    ) -> &'a [crate::mir::resolved_semantics::ResolvedMethodCallArgumentSourceV1] {
        self.contract.arguments()
    }

    pub(crate) const fn recipe_row(self) -> S6CRecipeOperationRowRefV2<'a> {
        self.row
    }
}

/// Borrow-only logical input handed to a future JOINIR consumer.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CScanWithInitLogicalJoinInputRefV1<'a> {
    rows: S6CScanWithInitRecipeRowsRefV2<'a>,
    roles: S6CScanWithInitRecipeRolesRefV2<'a>,
    facts: crate::mir::loop_structural_facts::S6CScanWithInitFactsRefV1<'a>,
    length: S6CLogicalCallInputRefV1<'a>,
    substring: S6CLogicalCallInputRefV1<'a>,
    transfer: &'a LoopJoinLogicalTransferViewV2<'a>,
}

impl<'a> S6CScanWithInitLogicalJoinInputRefV1<'a> {
    pub(crate) const fn rows(self) -> S6CScanWithInitRecipeRowsRefV2<'a> {
        self.rows
    }

    pub(crate) const fn roles(self) -> S6CScanWithInitRecipeRolesRefV2<'a> {
        self.roles
    }

    pub(crate) const fn length(self) -> S6CLogicalCallInputRefV1<'a> {
        self.length
    }

    pub(crate) const fn substring(self) -> S6CLogicalCallInputRefV1<'a> {
        self.substring
    }

    pub(crate) const fn logical_transfer(self) -> &'a LoopJoinLogicalTransferViewV2<'a> {
        self.transfer
    }

    pub(super) const fn facts(
        self,
    ) -> crate::mir::loop_structural_facts::S6CScanWithInitFactsRefV1<'a> {
        self.facts
    }
}

pub(crate) fn with_s6c_scan_with_init_logical_join_input<R>(
    product: &VerifiedS6CScanWithInitRecipeProductV2,
    callback: impl for<'input> FnOnce(S6CScanWithInitLogicalJoinInputRefV1<'input>) -> R,
) -> Result<R, S6CLogicalJoinInputRejectV1> {
    product.with_product(|product| issue_input_view(&product, callback))
}

fn issue_input_view<'a, R>(
    product: &'a S6CScanWithInitRecipeProductRefV2<'a>,
    callback: impl FnOnce(S6CScanWithInitLogicalJoinInputRefV1<'a>) -> R,
) -> Result<R, S6CLogicalJoinInputRejectV1> {
    let rows = product.recipe_rows();
    let roles = product.roles();
    verify_domains(rows)?;
    verify_recipe_rows(rows, roles)?;

    let source = product.facts().source();
    let calls = source.calls();
    let typed = calls.typed();
    let subject = typed
        .inputs()
        .iter()
        .find(|input| input.role() == S6CTypedInputRoleV1::Subject)
        .ok_or(S6CLogicalJoinInputRejectV1::Call("subject input"))?;

    let length_row = rows
        .operation(roles.length_call())
        .ok_or(S6CLogicalJoinInputRejectV1::Call("length Recipe row"))?;
    let substring_row = rows
        .operation(roles.substring_call())
        .ok_or(S6CLogicalJoinInputRejectV1::Call("substring Recipe row"))?;
    verify_call(
        S6CLogicalCallRoleV1::Length,
        calls.length(),
        length_row,
        subject.binding(),
        roles.length_call().result(),
        ResolvedLoopPlacementV1::Condition,
        CoreMethodOp::StringLen,
        0,
        LoopValueClassV2::I64,
        roles.subject_input(),
        roles.body_index_read().result(),
        roles.slice_end_add().result(),
    )?;
    verify_call(
        S6CLogicalCallRoleV1::Substring,
        calls.substring(),
        substring_row,
        subject.binding(),
        roles.substring_call().result(),
        ResolvedLoopPlacementV1::Body,
        CoreMethodOp::StringSubstring,
        2,
        LoopValueClassV2::Text,
        roles.subject_input(),
        roles.body_index_read().result(),
        roles.slice_end_add().result(),
    )?;
    verify_transfer(&rows, roles, product.logical_transfer())?;

    Ok(callback(S6CScanWithInitLogicalJoinInputRefV1 {
        rows,
        roles,
        facts: product.facts(),
        length: S6CLogicalCallInputRefV1 {
            role: S6CLogicalCallRoleV1::Length,
            contract: calls.length(),
            row: length_row,
        },
        substring: S6CLogicalCallInputRefV1 {
            role: S6CLogicalCallRoleV1::Substring,
            contract: calls.substring(),
            row: substring_row,
        },
        transfer: product.logical_transfer(),
    }))
}

/// Projection-only companion for an already-issued logical product.
///
/// The validating issuer above is the only path that rechecks semantic
/// domains, call contracts, and Join transfer parity.  Later products borrow
/// the retained cohort through this private seam instead of issuing another
/// validation receipt.
pub(super) fn with_s6c_scan_with_init_retained_logical_join_input<R>(
    product: &VerifiedS6CScanWithInitRecipeProductV2,
    callback: impl for<'input> FnOnce(S6CScanWithInitLogicalJoinInputRefV1<'input>) -> R,
) -> R {
    product.with_product(|product| {
        let rows = product.recipe_rows();
        let roles = product.roles();
        let facts = product.facts();
        let calls = facts.source().calls();
        let length_row = rows
            .operation(roles.length_call())
            .expect("verified S6C Length row");
        let substring_row = rows
            .operation(roles.substring_call())
            .expect("verified S6C Substring row");
        callback(S6CScanWithInitLogicalJoinInputRefV1 {
            rows,
            roles,
            facts,
            length: S6CLogicalCallInputRefV1 {
                role: S6CLogicalCallRoleV1::Length,
                contract: calls.length(),
                row: length_row,
            },
            substring: S6CLogicalCallInputRefV1 {
                role: S6CLogicalCallRoleV1::Substring,
                contract: calls.substring(),
                row: substring_row,
            },
            transfer: product.logical_transfer(),
        })
    })
}

fn verify_domains(
    rows: S6CScanWithInitRecipeRowsRefV2<'_>,
) -> Result<(), S6CLogicalJoinInputRejectV1> {
    let expected = [
        (rows.loop_count(), 1, "loops"),
        (rows.block_count(), 3, "blocks"),
        (rows.binding_count(), 1, "bindings"),
        (rows.input_count(), 3, "inputs"),
        (rows.value_count(), 15, "values"),
        (rows.item_count(), 15, "items"),
        (rows.carrier_count(), 1, "carriers"),
        (rows.exit_count(), 1, "exits"),
    ];
    expected
        .into_iter()
        .find_map(|(actual, wanted, name)| (actual != wanted).then_some((actual, wanted, name)))
        .map_or(Ok(()), |(_, _, name)| {
            Err(S6CLogicalJoinInputRejectV1::Domain(name))
        })
}

fn verify_recipe_rows(
    rows: S6CScanWithInitRecipeRowsRefV2<'_>,
    roles: S6CScanWithInitRecipeRolesRefV2<'_>,
) -> Result<(), S6CLogicalJoinInputRejectV1> {
    let root = rows
        .root_loop()
        .ok_or(S6CLogicalJoinInputRejectV1::Row("root loop"))?;
    if root.key != roles.root_loop() || root.parent.is_some() || root.body != roles.body_block() {
        return Err(S6CLogicalJoinInputRejectV1::Row("root loop identity"));
    }
    let carrier = rows
        .index_carrier()
        .ok_or(S6CLogicalJoinInputRejectV1::Row("index carrier"))?;
    if carrier.key != roles.index_carrier()
        || carrier.owner_loop != roles.root_loop()
        || carrier.binding != roles.index_binding()
        || carrier.class != LoopValueClassV2::I64
        || carrier.entry_value != roles.index_input()
    {
        return Err(S6CLogicalJoinInputRejectV1::Row("carrier parity"));
    }
    require_value(
        rows.subject_input(),
        LoopValueClassV2::Text,
        "subject class",
    )?;
    require_value(rows.needle_input(), LoopValueClassV2::Text, "needle class")?;
    require_value(rows.index_input(), LoopValueClassV2::I64, "index class")?;
    require_result_class(rows, roles.condition_index_read(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.length_call(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.less_condition(), LoopValueClassV2::Bool)?;
    require_result_class(rows, roles.body_index_read(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.slice_one(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.slice_end_add(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.substring_call(), LoopValueClassV2::Text)?;
    require_result_class(rows, roles.text_equal(), LoopValueClassV2::Bool)?;
    require_result_class(rows, roles.return_index_read(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.step_index_read(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.step_one(), LoopValueClassV2::I64)?;
    require_result_class(rows, roles.step_add(), LoopValueClassV2::I64)?;

    let condition = rows
        .text_equal_if()
        .ok_or(S6CLogicalJoinInputRejectV1::Row("TextEq If"))?;
    if condition.condition != roles.text_equal().result()
        || condition.then_block != roles.text_eq_then_block()
        || condition.else_block.is_some()
    {
        return Err(S6CLogicalJoinInputRejectV1::Row("TextEq If parity"));
    }
    let exit = rows
        .loop_exit(roles.loop_return())
        .ok_or(S6CLogicalJoinInputRejectV1::Row("Loop Return"))?;
    if exit.owner_loop != roles.root_loop()
        || exit.kind
            != (LoopExitKindV2::Return {
                value: Some(roles.return_index_read().result()),
            })
    {
        return Err(S6CLogicalJoinInputRejectV1::Row("Loop Return parity"));
    }
    Ok(())
}

fn require_value(
    value: Option<S6CRecipeValueRowRefV2>,
    class: LoopValueClassV2,
    label: &'static str,
) -> Result<(), S6CLogicalJoinInputRejectV1> {
    match value {
        Some(value) if value.class == class => Ok(()),
        _ => Err(S6CLogicalJoinInputRejectV1::Row(label)),
    }
}

fn require_result_class(
    rows: S6CScanWithInitRecipeRowsRefV2<'_>,
    role: DefinedRoleV2,
    class: LoopValueClassV2,
) -> Result<(), S6CLogicalJoinInputRejectV1> {
    match rows.operation_result_class(role) {
        Some(actual) if actual == class => Ok(()),
        _ => Err(S6CLogicalJoinInputRejectV1::Row("result class")),
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_call(
    role: S6CLogicalCallRoleV1,
    contract: &VerifiedResolverCoreMethodCallableContractV1,
    row: S6CRecipeOperationRowRefV2<'_>,
    subject_binding: crate::mir::resolved_semantics::BindingRefV1,
    expected_result: LoopValueKeyV1,
    expected_placement: ResolvedLoopPlacementV1,
    expected_op: CoreMethodOp,
    expected_arity: u32,
    expected_class: LoopValueClassV2,
    subject_input: LoopValueKeyV1,
    body_index_read: LoopValueKeyV1,
    slice_end_add: LoopValueKeyV1,
) -> Result<(), S6CLogicalJoinInputRejectV1> {
    if contract.placement() != expected_placement
        || contract.target().row().row().op != expected_op
        || contract.target().row().arity() != expected_arity
        || contract.target().schema() != CoreMethodHomeSchemaV1::StringBoxText
        || contract.target().receiver() != CoreMethodHomeReceiverRelationV1::StringBoxReceiver
        || contract.target().abi_profile() != CoreMethodHomeAbiProfileV1::StringBoxTextV1
        || contract.target().execution_policy()
            != CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl
        || contract.target().row().row().effect != CoreMethodEffectV1::PureRead
        || !matches!(
            contract.receiver(),
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
                if binding == subject_binding
        )
    {
        return Err(S6CLogicalJoinInputRejectV1::Call("source target/receiver"));
    }
    let expected_result_relation = match expected_class {
        LoopValueClassV2::I64 => CoreMethodHomeResultRelationV1::I64ToCaller,
        LoopValueClassV2::Text => CoreMethodHomeResultRelationV1::TextToCaller,
        _ => return Err(S6CLogicalJoinInputRejectV1::Call("call result class")),
    };
    if contract.target().result() != expected_result_relation
        || contract.target().parameters().len() != expected_arity as usize
        || contract
            .target()
            .parameters()
            .iter()
            .any(|parameter| *parameter != CoreMethodHomeParameterRelationV1::I64Parameter)
    {
        return Err(S6CLogicalJoinInputRejectV1::Call(
            "source target result/args",
        ));
    }
    verify_recipe_call_slot(
        role,
        row,
        subject_input,
        expected_result,
        body_index_read,
        slice_end_add,
    )
}

fn verify_recipe_call_slot(
    role: S6CLogicalCallRoleV1,
    row: S6CRecipeOperationRowRefV2<'_>,
    subject_input: LoopValueKeyV1,
    expected_result: LoopValueKeyV1,
    body_index_read: LoopValueKeyV1,
    slice_end_add: LoopValueKeyV1,
) -> Result<(), S6CLogicalJoinInputRejectV1> {
    let S6CRecipeOperationRowRefV2::CallSlot {
        receiver,
        args,
        result,
    } = row
    else {
        return Err(S6CLogicalJoinInputRejectV1::Call("Recipe CallSlot"));
    };
    if receiver != Some(subject_input) || result != Some(expected_result) {
        return Err(S6CLogicalJoinInputRejectV1::Call(
            "CallSlot receiver/result",
        ));
    }
    match role {
        S6CLogicalCallRoleV1::Length if args.is_empty() => Ok(()),
        S6CLogicalCallRoleV1::Substring
            if args.len() == 2 && args[0] == body_index_read && args[1] == slice_end_add =>
        {
            Ok(())
        }
        _ => Err(S6CLogicalJoinInputRejectV1::Call("CallSlot arguments")),
    }
}

fn verify_transfer(
    rows: &S6CScanWithInitRecipeRowsRefV2<'_>,
    roles: S6CScanWithInitRecipeRolesRefV2<'_>,
    transfer: &LoopJoinLogicalTransferViewV2<'_>,
) -> Result<(), S6CLogicalJoinInputRejectV1> {
    let after = transfer.after();
    if after.loop_key() != roles.root_loop()
        || after.binding() != roles.index_binding()
        || after.class() != LoopValueClassV2::I64
    {
        return Err(S6CLogicalJoinInputRejectV1::Transfer("After"));
    }
    let [branch] = transfer.branches() else {
        return Err(S6CLogicalJoinInputRejectV1::Transfer("branch count"));
    };
    if branch.owner_loop != roles.root_loop()
        || branch.if_item != roles.text_equal_if()
        || branch.condition != roles.text_equal().result()
    {
        return Err(S6CLogicalJoinInputRejectV1::Transfer("branch identity"));
    }
    match branch.then_arm {
        LoopJoinBranchArmTransferRefV2::Exit(exit)
            if exit.exit_item == roles.loop_return().item()
                && exit.role == LoopJoinEdgeRoleV1::Return
                && exit.target == LoopJoinBranchExitTargetV2::FunctionExit => {}
        _ => return Err(S6CLogicalJoinInputRejectV1::Transfer("then arm")),
    }
    if !matches!(
        branch.else_arm,
        LoopJoinBranchArmTransferRefV2::Fallthrough { .. }
    ) {
        return Err(S6CLogicalJoinInputRejectV1::Transfer("else arm"));
    }
    if transfer.summary_transfers().len() != 1
        || transfer.summary_transfers()[0].role != LoopJoinEdgeRoleV1::Return
        || transfer
            .boundaries()
            .iter()
            .filter(|row| row.role == LoopJoinEdgeRoleV1::Backedge)
            .count()
            != 1
        || rows.loop_exit(roles.loop_return()).is_none()
    {
        return Err(S6CLogicalJoinInputRejectV1::Transfer("summary/backedge"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{verify_recipe_call_slot, S6CLogicalCallRoleV1, S6CLogicalJoinInputRejectV1};
    use crate::mir::loop_recipe_contract::ids::LoopValueKeyV1;
    use crate::mir::loop_recipe_contract::s6c_scan_with_init_rows::S6CRecipeOperationRowRefV2;

    #[test]
    fn logical_join_input_rejects_swapped_substring_arguments() {
        let args = [LoopValueKeyV1::new(8), LoopValueKeyV1::new(6)];
        let row = S6CRecipeOperationRowRefV2::CallSlot {
            receiver: Some(LoopValueKeyV1::new(0)),
            args: &args,
            result: Some(LoopValueKeyV1::new(9)),
        };
        assert_eq!(
            verify_recipe_call_slot(
                S6CLogicalCallRoleV1::Substring,
                row,
                LoopValueKeyV1::new(0),
                LoopValueKeyV1::new(9),
                LoopValueKeyV1::new(6),
                LoopValueKeyV1::new(8),
            ),
            Err(S6CLogicalJoinInputRejectV1::Call("CallSlot arguments"))
        );
    }

    #[test]
    fn logical_join_input_rejects_swapped_call_receiver() {
        let args = [LoopValueKeyV1::new(6), LoopValueKeyV1::new(8)];
        let row = S6CRecipeOperationRowRefV2::CallSlot {
            receiver: Some(LoopValueKeyV1::new(1)),
            args: &args,
            result: Some(LoopValueKeyV1::new(9)),
        };
        assert_eq!(
            verify_recipe_call_slot(
                S6CLogicalCallRoleV1::Substring,
                row,
                LoopValueKeyV1::new(0),
                LoopValueKeyV1::new(9),
                LoopValueKeyV1::new(6),
                LoopValueKeyV1::new(8),
            ),
            Err(S6CLogicalJoinInputRejectV1::Call(
                "CallSlot receiver/result"
            ))
        );
    }
}
