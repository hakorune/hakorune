//! Builder-free prephysical ingress for the fixed S6C `ScanWithInit` product.
//!
//! This module co-seals existing source/Recipe/Join/Completion evidence.  It
//! does not issue a new Recipe, effect meaning, ABI, physical ID, or session.

use super::ids::{LoopBindingKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::s6c_scan_with_init_joinir_output::S6CScanWithInitLogicalOutputRefV1;
use super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1;
use super::schema_v2::{LoopOperationExecutionClassV2, LoopValueClassV2};
use crate::mir::loop_structural_facts::S6CScanWithInitFactsRefV1;
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CPrephysicalSourceInputRefV2<'rows, 'facts> {
    logical: S6CScanWithInitLogicalOutputRefV1<'rows, 'facts>,
    facts: S6CScanWithInitFactsRefV1<'facts>,
}

impl<'rows, 'facts> S6CPrephysicalSourceInputRefV2<'rows, 'facts> {
    pub(crate) const fn from_parts(
        logical: S6CScanWithInitLogicalOutputRefV1<'rows, 'facts>,
        facts: S6CScanWithInitFactsRefV1<'facts>,
    ) -> Self {
        Self { logical, facts }
    }

    fn logical(self) -> S6CScanWithInitLogicalOutputRefV1<'rows, 'facts> {
        self.logical
    }

    fn facts(self) -> S6CScanWithInitFactsRefV1<'facts> {
        self.facts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct S6CPrephysicalOperationSealV2 {
    role: S6CPrephysicalOperationRoleV2,
    item: LoopItemKeyV1,
    anchor_count: u8,
    execution: LoopOperationExecutionClassV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct S6CPrephysicalCompletionSealV2 {
    target_function: crate::mir::resolved_semantics::RegionId,
    explicit_exit_count: usize,
    cleanup_empty: bool,
    loop_return_site: SourceStmtSiteV1,
    tail_site: SourceStmtSiteV1,
    tail_value: SourceExprSiteV1,
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
    context: super::semantic_context::VerifiedLoopSemanticContextV1,
    seal: S6CPrephysicalIngressSealV2,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CPrephysicalIngressRefV2<'a, 'rows, 'facts> {
    source: S6CPrephysicalSourceInputRefV2<'rows, 'facts>,
    context: &'a super::semantic_context::VerifiedLoopSemanticContextV1,
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
        self.output
            .with_prephysical_source(|source| {
                callback(S6CPrephysicalIngressRefV2 {
                    source,
                    context: &self.context,
                    seal: &self.seal,
                })
            })
            .map_err(|_| S6CPrephysicalIngressRejectV2::Output("logical source"))?
    }
}

impl<'a, 'rows, 'facts> S6CPrephysicalIngressRefV2<'a, 'rows, 'facts> {
    pub(crate) fn logical(self) -> S6CScanWithInitLogicalOutputRefV1<'rows, 'facts> {
        self.source.logical()
    }

    pub(crate) const fn context(
        self,
    ) -> &'a super::semantic_context::VerifiedLoopSemanticContextV1 {
        self.context
    }

    pub(crate) const fn operation_count(self) -> usize {
        OPERATION_COUNT
    }

    pub(crate) fn operation_roles(self) -> impl Iterator<Item = S6CPrephysicalOperationRoleV2> {
        S6CPrephysicalOperationRoleV2::ALL.into_iter()
    }

    pub(crate) fn anchor_count(self, role: S6CPrephysicalOperationRoleV2) -> usize {
        self.seal
            .operations
            .iter()
            .find(|row| row.role == role)
            .map_or(0, |row| row.anchor_count as usize)
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
    ) -> Option<LoopOperationExecutionClassV2> {
        self.seal
            .operations
            .iter()
            .find(|row| row.role == role)
            .map(|row| row.execution)
    }

    pub(crate) fn completion(self) -> (&'a SourceStmtSiteV1, &'a SourceStmtSiteV1, bool) {
        (
            &self.seal.completion.loop_return_site,
            &self.seal.completion.tail_site,
            self.seal.completion.cleanup_empty,
        )
    }
}

pub(crate) fn issue_s6c_prephysical_ingress_v2(
    output: super::s6c_scan_with_init_joinir_output::VerifiedS6CScanWithInitLogicalOutputV1,
) -> Result<VerifiedS6CPrephysicalIngressV2, S6CPrephysicalIngressRejectV2> {
    let (context, seal) = output
        .with_prephysical_source(validate_prephysical_source)
        .map_err(|_| S6CPrephysicalIngressRejectV2::Output("logical source"))??;
    Ok(VerifiedS6CPrephysicalIngressV2 {
        output,
        context,
        seal,
    })
}

fn validate_prephysical_source(
    source: S6CPrephysicalSourceInputRefV2<'_, '_>,
) -> Result<
    (
        super::semantic_context::VerifiedLoopSemanticContextV1,
        S6CPrephysicalIngressSealV2,
    ),
    S6CPrephysicalIngressRejectV2,
> {
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
    let membership = typed.membership();
    let source_identity = membership.source();
    let owner = calls.length().owner();
    if calls.substring().owner() != owner || facts.completion().owner() != owner {
        return Err(S6CPrephysicalIngressRejectV2::Context("owner"));
    }

    let context = super::semantic_context::VerifiedLoopSemanticContextV1::from_parts(
        owner,
        source_identity.function_origin(),
        source_identity.source_kind(),
        source_identity.site().clone(),
        membership.frame().clone(),
        membership.scope_region(),
    );

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

    let operations = operation_seal(items, logical.calls(), typed, facts)?;
    let completion = facts.completion();
    if completion.explicit_sites().len() != 2 || !completion.cleanup().crossed_scopes().is_empty() {
        return Err(S6CPrephysicalIngressRejectV2::Completion(
            "exact-two cleanup",
        ));
    }
    let source = facts;
    let completion_sites = completion.explicit_sites();
    let loop_return_site = source.loop_return_site().clone();
    let tail_site = source.tail_site().clone();
    if !completion_sites.contains(&loop_return_site) || !completion_sites.contains(&tail_site) {
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

    Ok((
        context,
        S6CPrephysicalIngressSealV2 {
            operations,
            completion: S6CPrephysicalCompletionSealV2 {
                target_function: completion.target_function(),
                explicit_exit_count: completion_sites.len(),
                cleanup_empty: completion.cleanup().crossed_scopes().is_empty(),
                loop_return_site,
                tail_site,
                tail_value: source.tail_value().clone(),
            },
            inputs: [subject, needle, index],
            index_binding: index,
            index_carrier_entry: header.index_input,
            after: (header.root_loop, header.index_binding, after.class()),
        },
    ))
}

fn operation_seal(
    items: &[S6CLogicalItemV1],
    calls: super::s6c_scan_with_init_joinir_output::S6CLogicalCallPairsRefV1<'_>,
    typed: &crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    source: crate::mir::loop_structural_facts::S6CExitTailSourceCoSealRefV1<'_>,
) -> Result<[S6CPrephysicalOperationSealV2; OPERATION_COUNT], S6CPrephysicalIngressRejectV2> {
    require_control_census(items)?;
    let expected = [
        (S6CPrephysicalOperationRoleV2::ConditionIndexRead, 0, 1),
        (S6CPrephysicalOperationRoleV2::LengthCall, 1, 1),
        (S6CPrephysicalOperationRoleV2::LessCondition, 2, 1),
        (S6CPrephysicalOperationRoleV2::BodyIndexRead, 3, 2),
        (S6CPrephysicalOperationRoleV2::SliceOne, 4, 1),
        (S6CPrephysicalOperationRoleV2::SliceEndAdd, 5, 1),
        (S6CPrephysicalOperationRoleV2::SubstringCall, 6, 1),
        (S6CPrephysicalOperationRoleV2::TextEqual, 7, 1),
        (S6CPrephysicalOperationRoleV2::ReturnIndexRead, 9, 1),
        (S6CPrephysicalOperationRoleV2::StepIndexRead, 11, 1),
        (S6CPrephysicalOperationRoleV2::StepOne, 12, 1),
        (S6CPrephysicalOperationRoleV2::StepAdd, 13, 1),
        (S6CPrephysicalOperationRoleV2::StepWrite, 14, 2),
    ];
    let mut out = [S6CPrephysicalOperationSealV2 {
        role: expected[0].0,
        item: LoopItemKeyV1::new(expected[0].1),
        anchor_count: expected[0].2,
        execution: LoopOperationExecutionClassV2::NonFaulting,
    }; OPERATION_COUNT];
    for (slot, (role, item, anchors)) in expected.into_iter().enumerate() {
        let Some(logical) = items
            .iter()
            .find(|row| item_key(**row) == LoopItemKeyV1::new(item))
        else {
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
        let actual_anchors = anchor_count_for_role(role, typed, calls, source)?;
        if actual_anchors != anchors {
            return Err(S6CPrephysicalIngressRejectV2::Anchor("role multiplicity"));
        }
        out[slot] = S6CPrephysicalOperationSealV2 {
            role,
            item: LoopItemKeyV1::new(item),
            anchor_count: anchors,
            execution,
        };
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

fn anchor_count_for_role(
    role: S6CPrephysicalOperationRoleV2,
    typed: &crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    calls: super::s6c_scan_with_init_joinir_output::S6CLogicalCallPairsRefV1<'_>,
    source: crate::mir::loop_structural_facts::S6CExitTailSourceCoSealRefV1<'_>,
) -> Result<u8, S6CPrephysicalIngressRejectV2> {
    let count = match role {
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
            return Ok(2);
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
            return Ok(2);
        }
    };
    let _ = count;
    Ok(1)
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
