//! Builder-free prephysical ingress for the fixed S6C `ScanWithInit` product.
//!
//! This module co-seals existing source/Recipe/Join/Completion evidence.  It
//! does not issue a new Recipe, effect meaning, ABI, physical ID, or session.

use super::ids::{LoopBindingKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::s6c_scan_with_init_joinir_output::S6CPrephysicalSourceInputRefV2;
use super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1;
use super::schema_v2::{LoopOperationExecutionClassV2, LoopValueClassV2};
use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1, SourceStmtSiteV1};

const OPERATION_COUNT: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CPrephysicalOperationRoleV2 {
    ConditionIndexRead,
    LengthCall,
    LessCondition,
    BodyIndexRead,
    SliceOne,
    SliceEndAdd,
    SubstringCall,
    TextEqual,
    ReturnIndexRead,
    StepIndexRead,
    StepOne,
    StepAdd,
    StepWrite,
}

impl S6CPrephysicalOperationRoleV2 {
    const ALL: [Self; OPERATION_COUNT] = [
        Self::ConditionIndexRead,
        Self::LengthCall,
        Self::LessCondition,
        Self::BodyIndexRead,
        Self::SliceOne,
        Self::SliceEndAdd,
        Self::SubstringCall,
        Self::TextEqual,
        Self::ReturnIndexRead,
        Self::StepIndexRead,
        Self::StepOne,
        Self::StepAdd,
        Self::StepWrite,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct S6CPrephysicalOperationSealV2 {
    role: S6CPrephysicalOperationRoleV2,
    execution: LoopOperationExecutionClassV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct S6CPrephysicalCompletionSealV2 {
    target_function: crate::mir::resolved_semantics::RegionId,
    explicit_exit_count: usize,
    cleanup_empty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct S6CPrephysicalIngressSealV2 {
    operations: [S6CPrephysicalOperationSealV2; OPERATION_COUNT],
    completion: S6CPrephysicalCompletionSealV2,
    inputs: [BindingRefV1; 3],
    index_binding: BindingRefV1,
    index_carrier_entry: LoopValueKeyV1,
    after: (LoopNodeKeyV1, LoopBindingKeyV1, LoopValueClassV2),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CPrephysicalIngressRejectV2 {
    Output(&'static str),
    Context(&'static str),
    Domain(&'static str),
    Operation(&'static str),
    Anchor(&'static str),
    Calls(&'static str),
    Transfer(&'static str),
    Completion(&'static str),
}

/// One caller-zero Builder-free semantic ingress.  It retains the source
/// output and lends only narrow views through `with_ingress`.
#[derive(Debug)]
pub(crate) struct VerifiedS6CPrephysicalIngressV2 {
    output: super::s6c_scan_with_init_joinir_output::VerifiedS6CScanWithInitLogicalOutputV1,
    seal: S6CPrephysicalIngressSealV2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CPrephysicalIngressRefV2<'a, 'rows, 'facts> {
    source: S6CPrephysicalSourceInputRefV2<'rows, 'facts>,
    seal: &'a S6CPrephysicalIngressSealV2,
}

impl VerifiedS6CPrephysicalIngressV2 {
    pub(crate) fn with_ingress<R>(
        &self,
        callback: impl for<'a, 'rows, 'facts> FnOnce(
            S6CPrephysicalIngressRefV2<'a, 'rows, 'facts>,
        )
            -> Result<R, S6CPrephysicalIngressRejectV2>,
    ) -> Result<R, S6CPrephysicalIngressRejectV2> {
        self.output.with_retained_prephysical_source(|source| {
            callback(S6CPrephysicalIngressRefV2 {
                source,
                seal: &self.seal,
            })
        })
    }

    pub(crate) fn with_text_eq_leaf<R>(
        &self,
        callback: impl for<'facts> FnOnce(S6CPrephysicalTextEqRefV2<'facts>) -> R,
    ) -> R {
        self.output.with_retained_prephysical_source(|source| {
            let logical = source.logical();
            let roles = logical.roles();
            let facts = source.facts().source();
            let binary = facts
                .calls()
                .typed()
                .binaries()
                .iter()
                .find(|binary| {
                    binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::TextEqual
                })
                .expect("verified S6C TextEq source relation");
            let row = logical
                .rows()
                .items()
                .iter()
                .find(|item| item_key(**item) == roles.text_equal().item())
                .copied()
                .expect("verified S6C TextEq Recipe row");
            let if_row = logical
                .rows()
                .items()
                .iter()
                .find(|item| item_key(**item) == roles.text_equal_if())
                .copied()
                .expect("verified S6C TextEq If row");
            callback(S6CPrephysicalTextEqRefV2 {
                operation: S6CPrephysicalOperationRefV2 {
                    role: S6CPrephysicalOperationRoleV2::TextEqual,
                    item: roles.text_equal().item(),
                    execution: self
                        .seal
                        .operations
                        .iter()
                        .find(|operation| {
                            operation.role == S6CPrephysicalOperationRoleV2::TextEqual
                        })
                        .expect("verified S6C TextEq operation")
                        .execution,
                },
                row,
                if_row,
                binary,
            })
        })
    }

    pub(crate) fn with_completion<R>(
        &self,
        callback: impl for<'facts> FnOnce(S6CPrephysicalCompletionRefV2<'facts>) -> R,
    ) -> R {
        self.output.with_retained_prephysical_source(|source| {
            let facts = source.facts().source();
            callback(S6CPrephysicalCompletionRefV2 {
                completion: facts.completion(),
                loop_return_site: facts.loop_return_site(),
                loop_return_value: facts.loop_return_value(),
                tail_site: facts.tail_site(),
                tail_value: facts.tail_value(),
                tail_operand: facts.tail_operand(),
            })
        })
    }
}

impl<'a, 'rows, 'facts> S6CPrephysicalIngressRefV2<'a, 'rows, 'facts> {
    pub(crate) const fn operation_count(self) -> usize {
        OPERATION_COUNT
    }

    pub(crate) fn operation_roles(self) -> impl Iterator<Item = S6CPrephysicalOperationRoleV2> {
        S6CPrephysicalOperationRoleV2::ALL.into_iter()
    }

    pub(crate) const fn input_bindings(self) -> [BindingRefV1; 3] {
        self.seal.inputs
    }

    pub(crate) const fn index_carrier_entry(self) -> LoopValueKeyV1 {
        self.seal.index_carrier_entry
    }

    pub(crate) const fn after(self) -> (LoopNodeKeyV1, LoopBindingKeyV1, LoopValueClassV2) {
        self.seal.after
    }

    pub(crate) fn operation_execution(
        self,
        role: S6CPrephysicalOperationRoleV2,
    ) -> LoopOperationExecutionClassV2 {
        self.operation(role).execution()
    }

    pub(crate) fn operation(
        self,
        role: S6CPrephysicalOperationRoleV2,
    ) -> S6CPrephysicalOperationRefV2 {
        let row = self
            .seal
            .operations
            .iter()
            .find(|row| row.role == role)
            .expect("verified S6C operation role");
        S6CPrephysicalOperationRefV2 {
            role,
            item: item_for_role(self.source.logical().roles(), role),
            execution: row.execution,
        }
    }

    pub(crate) fn completion(self) -> S6CPrephysicalCompletionParityRefV2 {
        S6CPrephysicalCompletionParityRefV2 {
            target_function: self.seal.completion.target_function,
            explicit_exit_count: self.seal.completion.explicit_exit_count,
            cleanup_empty: self.seal.completion.cleanup_empty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CPrephysicalOperationRefV2 {
    role: S6CPrephysicalOperationRoleV2,
    item: LoopItemKeyV1,
    execution: LoopOperationExecutionClassV2,
}

impl S6CPrephysicalOperationRefV2 {
    pub(crate) const fn role(self) -> S6CPrephysicalOperationRoleV2 {
        self.role
    }

    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn execution(self) -> LoopOperationExecutionClassV2 {
        self.execution
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CPrephysicalCompletionParityRefV2 {
    target_function: crate::mir::resolved_semantics::RegionId,
    explicit_exit_count: usize,
    cleanup_empty: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CPrephysicalCompletionRefV2<'facts> {
    completion: &'facts crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    loop_return_site: &'facts SourceStmtSiteV1,
    loop_return_value: &'facts SourceExprSiteV1,
    tail_site: &'facts SourceStmtSiteV1,
    tail_value: &'facts SourceExprSiteV1,
    tail_operand: &'facts SourceExprSiteV1,
}

impl<'facts> S6CPrephysicalCompletionRefV2<'facts> {
    pub(crate) const fn completion(
        self,
    ) -> &'facts crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1 {
        self.completion
    }

    pub(crate) const fn loop_return_site(&self) -> &SourceStmtSiteV1 {
        self.loop_return_site
    }

    pub(crate) const fn loop_return_value(&self) -> &SourceExprSiteV1 {
        self.loop_return_value
    }

    pub(crate) const fn tail_site(&self) -> &SourceStmtSiteV1 {
        self.tail_site
    }

    pub(crate) const fn tail_value(&self) -> &SourceExprSiteV1 {
        self.tail_value
    }

    pub(crate) const fn tail_operand(&self) -> &SourceExprSiteV1 {
        self.tail_operand
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CPrephysicalTextEqRefV2<'facts> {
    operation: S6CPrephysicalOperationRefV2,
    row: S6CLogicalItemV1,
    if_row: S6CLogicalItemV1,
    binary: &'facts crate::mir::callable_semantic_batch::S6CBinaryRelationV1,
}

impl<'facts> S6CPrephysicalTextEqRefV2<'facts> {
    pub(crate) const fn operation(self) -> S6CPrephysicalOperationRefV2 {
        self.operation
    }

    pub(crate) const fn row(self) -> S6CLogicalItemV1 {
        self.row
    }

    pub(crate) const fn if_row(self) -> S6CLogicalItemV1 {
        self.if_row
    }

    pub(crate) const fn binary(
        self,
    ) -> &'facts crate::mir::callable_semantic_batch::S6CBinaryRelationV1 {
        self.binary
    }
}

impl S6CPrephysicalCompletionParityRefV2 {
    pub(crate) const fn target_function(self) -> crate::mir::resolved_semantics::RegionId {
        self.target_function
    }

    pub(crate) const fn explicit_exit_count(self) -> usize {
        self.explicit_exit_count
    }

    pub(crate) const fn cleanup_empty(self) -> bool {
        self.cleanup_empty
    }
}

pub(crate) fn issue_s6c_prephysical_ingress_v2(
    output: super::s6c_scan_with_init_joinir_output::VerifiedS6CScanWithInitLogicalOutputV1,
) -> Result<VerifiedS6CPrephysicalIngressV2, S6CPrephysicalIngressRejectV2> {
    let seal = output
        .with_prephysical_source(validate_prephysical_source)
        .map_err(|_| S6CPrephysicalIngressRejectV2::Output("logical source"))??;
    Ok(VerifiedS6CPrephysicalIngressV2 { output, seal })
}

fn validate_prephysical_source(
    source: S6CPrephysicalSourceInputRefV2<'_, '_>,
) -> Result<S6CPrephysicalIngressSealV2, S6CPrephysicalIngressRejectV2> {
    let logical = source.logical();
    if !logical.domains().is_exact_s6c() {
        return Err(S6CPrephysicalIngressRejectV2::Domain("S6C domains"));
    }
    let rows = logical.rows();
    let items = rows.items();
    if items.len() != 15 {
        return Err(S6CPrephysicalIngressRejectV2::Domain("item placements"));
    }
    let facts = source.facts().source();
    let calls = facts.calls();
    let typed = calls.typed();
    let owner = calls.length().owner();
    if calls.substring().owner() != owner || facts.completion().owner() != owner {
        return Err(S6CPrephysicalIngressRejectV2::Context("owner"));
    }

    let inputs = typed.inputs();
    let mut input_bindings = [None; 3];
    for (slot, role) in [
        (
            0,
            super::super::callable_semantic_batch::S6CTypedInputRoleV1::Subject,
        ),
        (
            1,
            super::super::callable_semantic_batch::S6CTypedInputRoleV1::Needle,
        ),
        (
            2,
            super::super::callable_semantic_batch::S6CTypedInputRoleV1::Index,
        ),
    ] {
        let Some(input) = inputs.iter().find(|input| input.role() == role) else {
            return Err(S6CPrephysicalIngressRejectV2::Context("input role"));
        };
        input_bindings[slot] = Some(input.binding());
    }
    let [Some(subject), Some(needle), Some(index)] = input_bindings else {
        return Err(S6CPrephysicalIngressRejectV2::Context("input coverage"));
    };
    if index != typed.initializer().binding() {
        return Err(S6CPrephysicalIngressRejectV2::Context("index binding"));
    }

    let transfer = logical.logical_transfer();
    if transfer.branches().len() != 1
        || transfer.summary_transfers().len() != 1
        || transfer
            .boundaries()
            .iter()
            .filter(|row| row.role == super::join_sig::LoopJoinEdgeRoleV1::Backedge)
            .count()
            != 1
        || transfer.after().class() != LoopValueClassV2::I64
    {
        return Err(S6CPrephysicalIngressRejectV2::Transfer("logical transfer"));
    }

    let operations = operation_seal(items, logical.roles(), logical.calls(), typed, facts)?;
    let completion = facts.completion();
    if completion.explicit_sites().len() != 2 || !completion.cleanup().crossed_scopes().is_empty() {
        return Err(S6CPrephysicalIngressRejectV2::Completion(
            "exact-two cleanup",
        ));
    }
    let completion_sites = completion.explicit_sites();
    if !completion_sites.contains(facts.loop_return_site())
        || !completion_sites.contains(facts.tail_site())
    {
        return Err(S6CPrephysicalIngressRejectV2::Completion("exit coverage"));
    }

    let after = transfer.after();
    let header = rows.header;
    if after.loop_key() != header.root_loop
        || after.binding() != header.index_binding
        || after.class() != LoopValueClassV2::I64
    {
        return Err(S6CPrephysicalIngressRejectV2::Transfer("After parity"));
    }

    Ok(S6CPrephysicalIngressSealV2 {
        operations,
        completion: S6CPrephysicalCompletionSealV2 {
            target_function: completion.target_function(),
            explicit_exit_count: completion_sites.len(),
            cleanup_empty: completion.cleanup().crossed_scopes().is_empty(),
        },
        inputs: [subject, needle, index],
        index_binding: index,
        index_carrier_entry: header.index_input,
        after: (header.root_loop, header.index_binding, after.class()),
    })
}

fn operation_seal(
    items: &[S6CLogicalItemV1],
    roles: super::s6c_scan_with_init::S6CScanWithInitRecipeRolesRefV2<'_>,
    calls: super::s6c_scan_with_init_joinir_output::S6CLogicalCallPairsRefV1<'_>,
    typed: &crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    source: crate::mir::loop_structural_facts::S6CExitTailSourceCoSealRefV1<'_>,
) -> Result<[S6CPrephysicalOperationSealV2; OPERATION_COUNT], S6CPrephysicalIngressRejectV2> {
    require_control_census(items)?;
    let mut out = [S6CPrephysicalOperationSealV2 {
        role: S6CPrephysicalOperationRoleV2::ALL[0],
        execution: LoopOperationExecutionClassV2::NonFaulting,
    }; OPERATION_COUNT];
    for (slot, role) in S6CPrephysicalOperationRoleV2::ALL.into_iter().enumerate() {
        let item = item_for_role(roles, role);
        let Some(logical) = items.iter().find(|row| item_key(**row) == item) else {
            return Err(S6CPrephysicalIngressRejectV2::Operation("missing role"));
        };
        if matches!(
            logical,
            S6CLogicalItemV1::If { .. } | S6CLogicalItemV1::Exit { .. }
        ) {
            return Err(S6CPrephysicalIngressRejectV2::Operation(
                "control as operation",
            ));
        }
        let execution = match logical {
            S6CLogicalItemV1::CallSlot(call) => {
                let normal_result = (call.role
                    == super::s6c_scan_with_init_joinir::S6CLogicalCallRoleV1::Length)
                    .then_some(calls.length().row().result)
                    .or_else(|| {
                        (call.role
                            == super::s6c_scan_with_init_joinir::S6CLogicalCallRoleV1::Substring)
                            .then_some(calls.substring().row().result)
                    });
                LoopOperationExecutionClassV2::ExternallyBoundOutcome { normal_result }
            }
            _ => LoopOperationExecutionClassV2::NonFaulting,
        };
        verify_anchor_for_role(role, typed, calls, source)?;
        out[slot] = S6CPrephysicalOperationSealV2 { role, execution };
    }
    Ok(out)
}

fn require_control_census(items: &[S6CLogicalItemV1]) -> Result<(), S6CPrephysicalIngressRejectV2> {
    (items
        .iter()
        .filter(|item| {
            matches!(
                item,
                S6CLogicalItemV1::If { .. } | S6CLogicalItemV1::Exit { .. }
            )
        })
        .count()
        == 2)
        .then_some(())
        .ok_or(S6CPrephysicalIngressRejectV2::Operation("If/Exit census"))
}

fn verify_anchor_for_role(
    role: S6CPrephysicalOperationRoleV2,
    typed: &crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    calls: super::s6c_scan_with_init_joinir_output::S6CLogicalCallPairsRefV1<'_>,
    source: crate::mir::loop_structural_facts::S6CExitTailSourceCoSealRefV1<'_>,
) -> Result<(), S6CPrephysicalIngressRejectV2> {
    let _anchor = match role {
        S6CPrephysicalOperationRoleV2::ConditionIndexRead => binary_source(
            typed,
            crate::mir::callable_semantic_batch::S6CBinaryRoleV1::LoopConditionLess,
            "less",
        )?
        .lhs(),
        S6CPrephysicalOperationRoleV2::LengthCall => calls.length().source().call_site(),
        S6CPrephysicalOperationRoleV2::LessCondition => binary_source(
            typed,
            crate::mir::callable_semantic_batch::S6CBinaryRoleV1::LoopConditionLess,
            "less",
        )?
        .site(),
        S6CPrephysicalOperationRoleV2::BodyIndexRead => {
            let Some(argument) = calls.substring().source().arguments().first() else {
                return Err(S6CPrephysicalIngressRejectV2::Anchor("substring argument"));
            };
            let slice = typed
                .binaries()
                .iter()
                .find(|binary| {
                    binary.role()
                        == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::SliceEndAdd
                })
                .ok_or(S6CPrephysicalIngressRejectV2::Anchor("slice add"))?;
            if argument.site() == slice.source().lhs() {
                return Err(S6CPrephysicalIngressRejectV2::Anchor("body index source"));
            }
            return Ok(());
        }
        S6CPrephysicalOperationRoleV2::SliceOne => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::SliceEndAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("slice add"))?
            .source()
            .rhs(),
        S6CPrephysicalOperationRoleV2::SliceEndAdd => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::SliceEndAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("slice add"))?
            .source()
            .site(),
        S6CPrephysicalOperationRoleV2::SubstringCall => calls.substring().source().call_site(),
        S6CPrephysicalOperationRoleV2::TextEqual => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::TextEqual
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("TextEq"))?
            .source()
            .site(),
        S6CPrephysicalOperationRoleV2::ReturnIndexRead => source.loop_return_value(),
        S6CPrephysicalOperationRoleV2::StepIndexRead => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::StepAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("step add"))?
            .source()
            .lhs(),
        S6CPrephysicalOperationRoleV2::StepOne => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::StepAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("step add"))?
            .source()
            .rhs(),
        S6CPrephysicalOperationRoleV2::StepAdd => typed
            .binaries()
            .iter()
            .find(|binary| {
                binary.role() == crate::mir::callable_semantic_batch::S6CBinaryRoleV1::StepAdd
            })
            .ok_or(S6CPrephysicalIngressRejectV2::Anchor("step add"))?
            .source()
            .site(),
        S6CPrephysicalOperationRoleV2::StepWrite => {
            let _ = typed.index_update().statement_site();
            return Ok(());
        }
    };
    Ok(())
}

fn item_for_role(
    roles: super::s6c_scan_with_init::S6CScanWithInitRecipeRolesRefV2<'_>,
    role: S6CPrephysicalOperationRoleV2,
) -> LoopItemKeyV1 {
    match role {
        S6CPrephysicalOperationRoleV2::ConditionIndexRead => roles.condition_index_read().item(),
        S6CPrephysicalOperationRoleV2::LengthCall => roles.length_call().item(),
        S6CPrephysicalOperationRoleV2::LessCondition => roles.less_condition().item(),
        S6CPrephysicalOperationRoleV2::BodyIndexRead => roles.body_index_read().item(),
        S6CPrephysicalOperationRoleV2::SliceOne => roles.slice_one().item(),
        S6CPrephysicalOperationRoleV2::SliceEndAdd => roles.slice_end_add().item(),
        S6CPrephysicalOperationRoleV2::SubstringCall => roles.substring_call().item(),
        S6CPrephysicalOperationRoleV2::TextEqual => roles.text_equal().item(),
        S6CPrephysicalOperationRoleV2::ReturnIndexRead => roles.return_index_read().item(),
        S6CPrephysicalOperationRoleV2::StepIndexRead => roles.step_index_read().item(),
        S6CPrephysicalOperationRoleV2::StepOne => roles.step_one().item(),
        S6CPrephysicalOperationRoleV2::StepAdd => roles.step_add().item(),
        S6CPrephysicalOperationRoleV2::StepWrite => roles.step_write().item(),
    }
}

fn binary_source<'a>(
    typed: &'a crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    role: crate::mir::callable_semantic_batch::S6CBinaryRoleV1,
    label: &'static str,
) -> Result<
    &'a crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
    S6CPrephysicalIngressRejectV2,
> {
    typed
        .binaries()
        .iter()
        .find(|binary| binary.role() == role)
        .map(|binary| binary.source())
        .ok_or(S6CPrephysicalIngressRejectV2::Anchor(label))
}

fn item_key(item: S6CLogicalItemV1) -> LoopItemKeyV1 {
    match item {
        S6CLogicalItemV1::ReadBinding { item, .. }
        | S6CLogicalItemV1::ConstI64 { item, .. }
        | S6CLogicalItemV1::BinaryI64 { item, .. }
        | S6CLogicalItemV1::CompareI64 { item, .. }
        | S6CLogicalItemV1::TextEq { item, .. }
        | S6CLogicalItemV1::If { item, .. }
        | S6CLogicalItemV1::WriteBinding { item, .. }
        | S6CLogicalItemV1::Exit { item, .. } => item,
        S6CLogicalItemV1::CallSlot(call) => call.item,
    }
}

#[cfg(test)]
mod tests {
    use super::{require_control_census, S6CLogicalItemV1, S6CPrephysicalIngressRejectV2};
    use crate::mir::loop_recipe_contract::ids::{
        LoopBindingKeyV1, LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopValueKeyV1,
    };

    fn read(item: u32) -> S6CLogicalItemV1 {
        S6CLogicalItemV1::ReadBinding {
            item: LoopItemKeyV1::new(item),
            block: LoopBlockKeyV1::new(1),
            binding: LoopBindingKeyV1::new(0),
            result: LoopValueKeyV1::new(item),
        }
    }

    #[test]
    fn control_census_rejects_missing_exit() {
        let mut items = (0..15).map(read).collect::<Vec<_>>();
        items[8] = S6CLogicalItemV1::If {
            item: LoopItemKeyV1::new(8),
            block: LoopBlockKeyV1::new(0),
            condition: LoopValueKeyV1::new(7),
            then_block: LoopBlockKeyV1::new(2),
            else_block: None,
        };
        items[10] = S6CLogicalItemV1::Exit {
            item: LoopItemKeyV1::new(10),
            block: LoopBlockKeyV1::new(2),
            exit: LoopExitKeyV1::new(0),
            value: LoopValueKeyV1::new(9),
        };
        assert_eq!(require_control_census(&items), Ok(()));
        items[10] = read(10);
        assert_eq!(
            require_control_census(&items),
            Err(S6CPrephysicalIngressRejectV2::Operation("If/Exit census"))
        );
    }
}
