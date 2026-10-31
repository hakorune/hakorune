//! Caller-prefix Home availability, issued from one resolved source loan.
//! A successful prefix describes Normal-path installations, not runtime
//! cleanup readiness. Construction/argument unwind remains a required
//! dependency; unknown prefix meaning never becomes an empty Home list.

#[path = "home_field_source.rs"]
mod field_source;
pub(crate) use field_source::{
    HomeFieldSourceConsultV1, HomeTerminalFieldSourceConsultV1, NoScalarFieldSourceV1,
};

use super::{
    BindingRefV1, ExprChildRoleV1, FunctionOwnerIdV1, HomeDemandV1, OwnedExprSiteV1,
    ResolvedLexicalRefV1, ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedInstanceEntryHomeLoanV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_recipe_contract::VerifiedLoopHomeAfterLoanV1;

/// Source walk permission from one selected verifier. The search role is a
/// source-only view of the same Static V2 product consumed after Home.
pub(crate) enum SelectedLoopHomeLoanV1 {
    VariableAccumAfter(VerifiedLoopHomeAfterLoanV1),
    NoExitArrayAfter(std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>),
    StaticI64Search(crate::mir::builder::VerifiedStaticI64LoopHomeSourceV2),
    SourceObservation(
        std::rc::Rc<crate::mir::normal_callable_semantic_package::RetainedSourceLoopProductV1>,
    ),
}
use crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1;
use crate::mir::resolved_control_flow::{
    issue_new_fault_continuation_v1, issue_result_new_fault_continuation_v1, issue_shared_new_fault_continuation_v1, NewFaultContinuationV1,
};
use std::collections::{BTreeMap, BTreeSet};

/// Exact source-use request; this port consults original evidence only.
#[derive(Clone, Copy)]
pub(crate) enum BorrowedViewUseRequestV1<'a> {
    FormalFieldWriteReceiver {
        target: &'a SourceExprSiteV1,
        binding: BindingRefV1,
        root: BindingRefV1,
    },
    FormalFieldWriteValue {
        target: &'a SourceExprSiteV1,
    },
    ArrayElementFieldIndexValue {
        call: &'a SourceExprSiteV1,
        index: &'a SourceExprSiteV1,
        binding: BindingRefV1,
        root: BindingRefV1,
    },
    Operand,
    CheckedCompareOperand {
        binary: &'a OwnedExprSiteV1,
    },
    CheckedAddOperand {
        binary: &'a OwnedExprSiteV1,
    },
    IntegerReturn {
        exit: &'a SourceStmtSiteV1,
        binding: BindingRefV1,
    },
    IntegerFieldArrayReturn {
        exit: &'a SourceStmtSiteV1,
    },
    IntegerNestedFieldReturn {
        exit: &'a SourceStmtSiteV1,
        receiver: BindingRefV1,
    },
    IntegerMulReturn {
        exit: &'a SourceStmtSiteV1,
    },
}

#[path = "selected_new_arguments.rs"]
mod selected_new_arguments;
pub(crate) use selected_new_arguments::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1, SelectedNewArgumentUnavailableV1,
    SelectedNewArgumentV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HomePrefixUnavailableV1 {
    EntryDemandMissing,
    SourceMismatch,
    MoveUnsupported(SourceExprSiteV1),
    UseAfterMove(SourceExprSiteV1),
    PrefixNotCovered(SourceStmtSiteV1),
    ArgumentNotCovered(SourceExprSiteV1),
    OverridesNotCovered(SourceExprSiteV1),
    TerminalNotCovered,
    MapCandidateNotCovered(SourceExprSiteV1),
    ReturnValueNotCovered(SourceStmtSiteV1),
    /// The `If` branches fell through with disagreeing surviving Homes or
    /// local classes. Both branch states are retained for diagnostics; the
    /// joined path never substitutes a union or an empty list.
    HomeFlowBranchDivergent {
        site: SourceStmtSiteV1,
        then_homes: Box<[LiveHomeV1]>,
        else_homes: Box<[LiveHomeV1]>,
    },
}

/// Immutable source facts. Cloning preserves the same owner/site identities;
/// no physical value, storage policy, or cleanup implementation is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallerNewHomePrefixV1 {
    destination: BindingRefV1,
    prior_homes: Box<[LiveHomeV1]>,
    normal_homes: Box<[LiveHomeV1]>,
    outward_fault: NewFaultContinuationV1,
    covered_statements: Box<[SourceStmtSiteV1]>,
}

impl CallerNewHomePrefixV1 {
    pub(crate) fn normal_homes(&self) -> &[LiveHomeV1] {
        &self.normal_homes
    }
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior_homes
    }
    pub(crate) fn required_unwind(&self) -> &OwnedExprSiteV1 {
        self.outward_fault.site()
    }
    pub(crate) fn outward_fault(&self) -> &NewFaultContinuationV1 {
        &self.outward_fault
    }
    pub(crate) fn covered_statements(&self) -> &[SourceStmtSiteV1] {
        &self.covered_statements
    }
}

/// Destination-less prefix facts for a return-position `new`. The fresh
/// object's ownership transfers to the caller at the Return edge, so this
/// prefix carries no destination binding and no local installation — only
/// the prior Homes used on Fault and the remaining Homes after Normal argument
/// moves, the outward fault continuation, and the covered statement inventory.
/// Both outcome projections come from the same original terminal walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResultNewHomePrefixV1 {
    purpose: ResultNewPurposeV1,
    prior_homes: Box<[LiveHomeV1]>,
    normal_homes: Box<[LiveHomeV1]>,
    outward_fault: NewFaultContinuationV1,
    covered_statements: Box<[SourceStmtSiteV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResultNewPurposeV1 {
    Return,
    SharedPayload { shared_site: OwnedExprSiteV1, binding: BindingRefV1 },
}

/// Original source of one Shared handle; fresh payloads have no fake local.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SharedPayloadSourceV1 {
    Existing { from: BindingRefV1, acquisition: OwnedExprSiteV1, is_clone: bool },
    Fresh { site: OwnedExprSiteV1 },
}

impl ResultNewHomePrefixV1 {
    pub(crate) fn purpose(&self) -> &ResultNewPurposeV1 { &self.purpose }
    pub(crate) fn normal_homes(&self) -> &[LiveHomeV1] {
        &self.normal_homes
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior_homes
    }
    pub(crate) fn required_unwind(&self) -> &OwnedExprSiteV1 {
        self.outward_fault.site()
    }
    #[cfg(test)]
    pub(crate) fn covered_statements(&self) -> &[SourceStmtSiteV1] {
        &self.covered_statements
    }
}

#[path = "home_new_prefix_binding_rebind.rs"]
mod binding_rebind;
#[path = "home_expression_plan.rs"]
mod expression_plan;
pub(crate) use expression_plan::{
    qualify_argument_expression_v1, qualify_binding_rebind_expression_v1,
    qualify_binding_rebind_expression_with_loop_dynamic_v1,
    qualify_bool_condition_v1, qualify_condition_expression_v1, qualify_initializer_expression_v1,
    qualify_print_expression_v1, BindingRebindDestinationV1, CheckedArgumentExpressionFrameV1,
    CheckedBindingRebindFrameV1, CheckedBoolConditionFrameV1, CheckedExpressionEvaluationV1,
    CheckedExpressionReferenceV1, CheckedExpressionOperationLoanV1, CheckedFieldWriteFrameV1, CheckedInitializerExpressionFrameV1,
    CheckedPrintExpressionFrameV1, CheckedScalarReferenceV1, ExpressionCallSourceV1,
    ExpressionFieldRequirementV1, ExpressionScalarOperationLoanV1, FieldWritePlanV1,
    HomeExpressionPlanV1, QualifiedFieldWriteRhsV1,
};

#[path = "home_prefix_local_flow.rs"]
mod local_flow;
#[cfg(test)]
pub(crate) use local_flow::assert_text_residence_identity_for_test;
pub(crate) use local_flow::source_loop::{
    BranchNormalTextSourceV1, CheckedSourceLoopBodyV1, CheckedSourceLoopStatementV1, LoopDynamicReadV1, SourceLoopPortKindV1,
    SourceLoopDynamicProducerV1, SourceLoopReturnV1, SourceLoopScanSummaryV1,
    SourceLoopTextProducerV1, SourceLoopValueProducerV1,
};
pub(crate) use local_flow::{
    BranchNormalScalarSourceV1, CurrentScalarProducerV1, CurrentScalarReadV1, FieldAliasPathV1,
    FieldAliasRootSourceV1,
    LiveHomeV1, LoopHeaderValueBasisV1, LoopI64PreStateRequestV1, LoopTextReadV1, SourceScalarKind,
    StaticTextReadV1, TextResidenceAcquisitionV1,
};
pub(crate) use local_flow::{TaggedDynamicReadV1, TaggedDynamicValueV1};
use local_flow::{OrdinaryObservation, PrefixLocalFlow};
#[path = "home_local_call_flow.rs"]
mod local_call_flow;
pub(crate) use local_call_flow::{
    borrowed_actual_source_atom_v1, issue_source_scalar_expression_call, ArgumentCallObservationV1,
    BorrowedCallActualCandidateV1, BorrowedCallActualValueV1, LocalCallArgumentV1,
    LocalCallObservationV1, LocalCallResultClassV1, LocalCallResultSourceV1,
    QualifiedStaticCallClaimV1,
};
#[path = "home_static_i64_call_claim.rs"]
mod static_i64_call_claim;
pub(crate) use static_i64_call_claim::StaticI64CallClaimV1;
#[path = "home_map_descendant_flow.rs"]
mod map_descendant_flow;
#[path = "home_map_flow.rs"]
mod map_flow;
#[path = "home_terminal_relation.rs"]
mod terminal_relation;
pub(crate) use map_flow::{
    ArrayElementSource, LifecyclePrefixSourceV1, MapDestinationV1, MapEntryBorrowKindV1,
    MapEntryStoreClassV1, MapFieldGetLocalV1, MapFieldInstallV1, MapHomeEntry, MapHomeFlow,
    MapHomeObservation, MapValueSource, RootHomeExitKeyV1, RootHomeExitV1, RootHomeFlow,
};
pub(crate) use terminal_relation::issue_terminal_integer_literal_return_from_completion_v1;
use terminal_relation::{
    finite_entry_return, map_literal_keys, observe_text_return_expression_v1, return_scalar,
    terminal_map_get, terminal_returned_source, ReturnScalar,
};
pub(crate) use terminal_relation::{
    TerminalBoolLiteralReturnV1, TerminalCallArgumentV1, TerminalI64AddReturnV1,
    TerminalI64CallReturnV1, TerminalI64FieldReturnV1, TerminalI64ScalarReturnV1,
    TerminalIntegerLiteralReturnV1, TerminalMapGetReceiverClassV1, TerminalMapGetReturnV1,
    TerminalOpaqueCallReturnV1, TerminalRelationV1, TerminalReturnedSourceV1, TerminalUnitReturnV1,
    TerminalValueReturnV1,
};

pub(crate) fn issue_new_home_prefixes_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    entry_home: Option<&VerifiedInstanceEntryHomeLoanV1>,
) -> BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>> {
    scan_new_home_flow(
        input,
        selected,
        std::iter::empty(),
        entry_home,
        None,
        &[],
        false,
        &BTreeSet::new(),
        &mut |_: &crate::mir::resolved_semantics::OwnedExprSiteV1,
              _: &crate::mir::resolved_semantics::SourceExprSiteV1,
              _,
              _,
              _: &str| Ok::<_, std::convert::Infallible>(false),
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(None),
        &mut |_: &crate::mir::resolved_semantics::OwnedExprSiteV1,
              _: &crate::mir::resolved_semantics::SourceExprSiteV1,
              _,
              _,
              _: &str| Ok(false),
        &mut crate::mir::resolved_semantics::home_new_prefix::NoScalarFieldSourceV1,
        &mut |_: &crate::mir::resolved_semantics::OwnedExprSiteV1,
              _: &crate::mir::resolved_semantics::SourceExprSiteV1,
              _,
              _,
              _: &str| Ok(false),
        // The `ArrayBox` field census stays unavailable on this lane.
        &mut |_: &crate::mir::resolved_semantics::OwnedExprSiteV1,
              _: &crate::mir::resolved_semantics::SourceExprSiteV1,
              _,
              _,
              _: &str| Ok(false),
        // The `formal.<field>` index proof stays unavailable on this lane.
        &mut |_: &crate::mir::resolved_semantics::OwnedExprSiteV1,
              _: &crate::mir::resolved_semantics::SourceExprSiteV1,
              _,
              _,
              _: &str| Ok(false),
        &mut |_, _| Ok(None),
        &mut |_, _| Ok(None),
        // The bounded sibling consults no dominated-view draft: a `Handle`
        // leaf stays truthfully uncovered on this lane — the verified
        // completion lane owns the issuer predicate.
        &mut |_, _| Ok(false),
        &mut |_| Ok(None),
        &mut |_| Ok(None),
    )
    .unwrap_or_else(|never| match never {})
    .0
}

#[path = "home_new_prefix_arguments.rs"]
mod arguments;
pub(crate) use arguments::{
    issue_new_home_prefixes_probing_fields_v1, issue_new_home_prefixes_with_arguments_v1,
};
#[path = "home_new_prefix_branch.rs"]
mod branch;
pub(crate) use branch::SourceNullExclusionV1;
#[path = "home_new_prefix_field_call.rs"]
mod field_call;
pub(crate) use field_call::CheckedFieldArrayTextAppendV1;
#[path = "home_new_prefix_field_read.rs"]
mod field_read;
#[path = "home_new_prefix_scalar_expression.rs"]
mod scalar_expression;
pub(crate) use field_read::{
    BorrowedArrayFieldSourceV1, LocalFieldReadDemandV1, LocalFieldReadRequestV1,
    LocalFieldReadResultV1, ReceivedFieldReadRequestV1,
};
pub(in crate::mir) use scalar_expression::contains_static_eq_lhs_source_v1;
#[path = "home_new_prefix_field_write.rs"]
mod field_write;
#[path = "home_terminal_object_return.rs"]
mod object_return;
pub(crate) use object_return::{ObjectReturnAcquisitionV1, TerminalObjectReturnObligationV1};
#[path = "home_new_prefix_named_array.rs"]
mod named_array;
pub(crate) use named_array::NamedArrayHomeObservationV1;

#[path = "home_new_prefix_condition.rs"]
mod condition;
#[path = "home_new_prefix_local_statement.rs"]
mod local_statement;
#[path = "home_new_prefix_mixed_call.rs"]
mod mixed_call;
#[path = "home_new_prefix_new_local.rs"]
mod new_local;
#[path = "home_new_prefix_scan.rs"]
mod scan;
#[path = "home_new_prefix_source_loop.rs"]
mod source_loop;
pub(crate) use condition::{
    requires_condition_expression_walk_v1, requires_nested_call_argument_walk_v1,
};
#[path = "home_new_prefix_argument_expression.rs"]
mod argument_expression;
#[path = "home_new_prefix_initializer_expression.rs"]
mod initializer_expression;
#[cfg(test)]
pub(crate) use argument_expression::observe_for_test as observe_argument_expression_for_test;
#[cfg(test)]
pub(crate) use initializer_expression::observe_for_test as observe_local_initializers_for_test;
#[path = "home_new_prefix_print.rs"]
mod print;
#[path = "home_new_prefix_search_loop.rs"]
mod search_loop;
#[path = "home_new_prefix_terminal.rs"]
mod terminal;
#[path = "home_terminal_literal.rs"]
mod terminal_literal;
use scan::scan_statement_flow;

/// One source walk supplies both New-failure prefixes and terminal ownership.
/// The caller must take the terminal from the Completion verified on this input.
pub(crate) fn scan_new_home_flow<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    parameters: impl IntoIterator<
        Item = (
            u32,
            BindingRefV1,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
        ),
    >,
    entry_home: Option<&VerifiedInstanceEntryHomeLoanV1>,
    source_value_class_facts: Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>>,
    exit_sites: &[SourceStmtSiteV1],
    uncovered_implicit_exit: bool,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    field_is_integer: &mut impl crate::mir::resolved_semantics::home_new_prefix::HomeTerminalFieldSourceConsultV1<E>,
    map_compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
    terminal_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_map_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_handle_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    // The issuer's lexical instance-call membership for the exact-i64
    // result lane — `local x = recv.m(..)` on a claim-local `new` binding.
    local_lexical_i64_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    // The issuer's lexical instance-call membership for the
    // nullable-handle result lane — `local x = recv.m(..)` on a
    // claim-local receiver whose callee's claim is `NullableObject`.
    local_lexical_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    // The issuer's qualified static-box call membership: `local x =
    // Alias.m(..)` sites whose sealed target is a `StaticBoxMethod` with an
    // `ExactI64` result disposition carry the callee-required i64 ordinals.
    local_static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    argument_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's receiver-side scalar-field proof for `me.<field>` reads
    // inside a self-rooted field-write RHS — a separate contract from the
    // argument-position `i64` proof.
    scalar_field: &mut impl HomeFieldSourceConsultV1<E>,
    // The issuer's receiver-side container-field proof for `me.<field>`
    // receivers of builtin container calls — a separate contract from
    // both scalar proofs.
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's receiver-side `ArrayBox` field proof for
    // `me.<field>` receivers — true only when the declared field sits
    // inside the whole-Box element-integer census the issuer sealed.
    array_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's `formal.<field>` index proof — the scanner supplies
    // the exact read site and the receiver's resolved parameter binding;
    // the predicate alone decides the unique-declaration proof.
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's local-initializer `receiver.field` read membership —
    // the scanner supplies the exact read site, receiver site, receiver
    // binding, movable root, and any field-read alias class; the
    // predicate alone decides the field declaration and result class.
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    borrowed_actuals: &mut impl FnMut(
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
    // The issuer's dominated-view use membership — `true` only when the
    // sealed borrowed-formal draft admits an `ArrayElementValue`,
    // `AddOperand`, or `NewArgument` value use at this exact leaf site.
    // Coverage consult only; the draft stays the sole admission authority.
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
    loop_prestate: &mut impl FnMut(
        LoopI64PreStateRequestV1,
    ) -> Result<Option<SelectedLoopHomeLoanV1>, E>,
    object_return: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<ObjectReturnCallQualificationV1>, E>,
) -> Result<
    (
        BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
        RootHomeFlow,
        BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
        BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
        BTreeMap<OwnedExprSiteV1, Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>>,
    ),
    E,
> {
    let mut results = BTreeMap::new();
    let mut exit_homes: BTreeMap<
        SourceStmtSiteV1,
        Result<RootHomeExitV1, HomePrefixUnavailableV1>,
    > = exit_sites
        .iter()
        .map(|site| {
            (
                site.clone(),
                Err(HomePrefixUnavailableV1::TerminalNotCovered),
            )
        })
        .collect();
    let mut maps = Vec::new();
    let mut local_calls = Vec::new();
    let mut path_calls = BTreeSet::new();
    let mut terminal_relations = BTreeMap::new();
    let mut argument_observations = BTreeMap::new();
    let mut result_prefixes = BTreeMap::new();
    let function = input.function();
    // Capture demands are never covered by an entry loan — they keep the
    // named unavailability either way. A receiver demand is covered only by
    // the sole Home ABI issuer's loan for this exact declaration.
    let mut unavailable = (!input
        .forest()
        .ordered_capture_demands(input.owner())
        .is_empty()
        || (function
            .declaration_sites()
            .any(|site| matches!(site, SourceBindingSiteV1::Receiver))
            && entry_home.is_none()))
    .then_some(HomePrefixUnavailableV1::EntryDemandMissing);
    let Ok(body) = input.source().root_body() else {
        return Ok((
            selected
                .keys()
                .map(|site| (site.clone(), Err(HomePrefixUnavailableV1::SourceMismatch)))
                .collect(),
            RootHomeFlow {
                named_arrays: Vec::new(),
                exits: exit_homes
                    .into_iter()
                    .map(|(site, _)| {
                        (
                            RootHomeExitKeyV1::Explicit(site),
                            Err(HomePrefixUnavailableV1::SourceMismatch),
                        )
                    })
                    .chain(uncovered_implicit_exit.then_some((
                        RootHomeExitKeyV1::ImplicitBodyEnd,
                        Err(HomePrefixUnavailableV1::SourceMismatch),
                    )))
                    .collect(),
                maps,
                local_calls,
                expression_plans: Vec::new(),
            },
            terminal_relations,
            argument_observations,
            result_sites
                .iter()
                .map(|site| (site.clone(), Err(HomePrefixUnavailableV1::SourceMismatch)))
                .collect(),
        ));
    };
    let mut locals = PrefixLocalFlow::with_source_value_class_facts(input, source_value_class_facts);
    // A lent entry installs the loan's own verified receiver/parameter
    // bindings; without one the caller's declared parameter rows install.
    let installed = match entry_home {
        Some(loan) => locals.install_entry_home(loan),
        None => locals.install_parameters(parameters),
    };
    if !installed {
        unavailable = Some(HomePrefixUnavailableV1::EntryDemandMissing);
    }
    let mut homes = Vec::new();
    let mut covered_statements = Vec::new();
    let mut selected_seen = Vec::new();
    let exit_set: BTreeSet<SourceStmtSiteV1> = exit_sites.iter().cloned().collect();
    let terminated = scan_statement_flow(
        input,
        &body,
        &exit_set,
        selected,
        result_sites,
        &mut results,
        &mut exit_homes,
        &mut maps,
        &mut local_calls,
        &mut path_calls,
        &mut terminal_relations,
        &mut argument_observations,
        &mut result_prefixes,
        &mut locals,
        &mut homes,
        &mut covered_statements,
        &mut selected_seen,
        &mut unavailable,
        field_is_integer,
        map_compatible,
        terminal_call,
        local_map_call,
        local_handle_call,
        local_lexical_i64_call,
        local_lexical_nullable_call,
        local_nullable_call,
        local_static_call,
        argument_i64_field,
        scalar_field,
        container_field,
        array_i64_field,
        formal_i64_field,
        local_field_read,
        borrowed_actuals,
        view_use,
        loop_prestate,
        object_return,
    )?;
    // Capture exactly the same path state at the original body boundary.
    // A failed walk retains its failure; an empty Home list is not assumed.
    let implicit_exit = uncovered_implicit_exit.then(|| {
        let end = u32::try_from(body.statements().len())
            .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)?;
        if terminated {
            return Err(HomePrefixUnavailableV1::TerminalNotCovered);
        }
        if let Some(issue) = &unavailable {
            return Err(issue.clone());
        }
        Ok(RootHomeExitV1::implicit_from_live_v1(
            body.site().clone(),
            end,
            &homes,
            path_calls.iter().cloned().collect(),
            covered_statements.clone().into_boxed_slice(),
        ))
    });
    // Statements after the terminal are never walked; their sealed map
    // literals still owe loop1 one row each — issue Unavailable rows.
    map_descendant_flow::issue_unobserved_descendants(input, &mut maps);
    for site in selected.keys() {
        results.entry(site.clone()).or_insert_with(|| {
            Err(unavailable
                .clone()
                .unwrap_or(HomePrefixUnavailableV1::SourceMismatch))
        });
    }
    // A claim-member return-position `new` the terminal point never reached
    // keeps the same unavailability as every other unwalked site — claim
    // membership alone never fabricates coverage.
    for site in result_sites {
        result_prefixes.entry(site.clone()).or_insert_with(|| {
            Err(unavailable
                .clone()
                .unwrap_or(HomePrefixUnavailableV1::TerminalNotCovered))
        });
    }
    Ok((
        results,
        RootHomeFlow {
            named_arrays: locals.named_array_observations(),
            exits: exit_homes
                .into_iter()
                .map(|(site, row)| (RootHomeExitKeyV1::Explicit(site), row))
                .chain(implicit_exit.map(|row| (RootHomeExitKeyV1::ImplicitBodyEnd, row)))
                .collect(),
            maps,
            local_calls,
            expression_plans: locals.expression_plans(),
        },
        terminal_relations,
        argument_observations,
        result_prefixes,
    ))
}

#[path = "home_borrowed_call_port.rs"]
mod borrowed_call_port;
pub(crate) use borrowed_call_port::{
    BorrowedCallActualRequestV1, BorrowedCallArgumentsV1, ObjectCallSourceSupportV1,
};

#[cfg(test)]
pub(crate) use local_call_flow::issue_static_source_local_for_test;

#[cfg(test)]
pub(crate) use local_flow::source_loop::assert_original_loop_source_walk_for_test;
