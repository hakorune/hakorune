//! Caller-prefix Home availability, issued from one resolved source loan.
//!
//! A successful prefix describes Normal-path installations, not runtime
//! cleanup readiness. Construction/argument unwind remains a required
//! dependency; unknown prefix meaning never becomes an empty Home list.

use super::{
    BindingRefV1, ExprChildRoleV1, FunctionOwnerIdV1, HomeDemandV1, OwnedExprSiteV1,
    ResolvedLexicalRefV1, ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedInstanceEntryHomeLoanV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::{
    issue_new_fault_continuation_v1, issue_result_new_fault_continuation_v1, NewFaultContinuationV1,
};
use std::collections::{BTreeMap, BTreeSet};

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
        then_homes: Box<[BindingRefV1]>,
        else_homes: Box<[BindingRefV1]>,
    },
}

/// Immutable source facts. Cloning preserves the same owner/site identities;
/// no physical value, storage policy, or cleanup implementation is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallerNewHomePrefixV1 {
    destination: BindingRefV1,
    prior_homes: Box<[BindingRefV1]>,
    outward_fault: NewFaultContinuationV1,
    covered_statements: Box<[SourceStmtSiteV1]>,
}

impl CallerNewHomePrefixV1 {
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] {
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
/// the live prior Homes (still this frame's exit obligation), the outward
/// fault continuation the construction unwinds through, and the covered
/// statement inventory at the terminal point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResultNewHomePrefixV1 {
    prior_homes: Box<[BindingRefV1]>,
    outward_fault: NewFaultContinuationV1,
    covered_statements: Box<[SourceStmtSiteV1]>,
}

impl ResultNewHomePrefixV1 {
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] {
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

#[path = "home_prefix_local_flow.rs"]
mod local_flow;
pub(crate) use local_flow::SourceScalarKind;
use local_flow::{OrdinaryObservation, PrefixLocalFlow};
#[path = "home_local_call_flow.rs"]
mod local_call_flow;
pub(crate) use local_call_flow::{LocalCallObservationV1, LocalCallResultClassV1};
#[path = "home_map_descendant_flow.rs"]
mod map_descendant_flow;
#[path = "home_map_flow.rs"]
mod map_flow;
#[path = "home_terminal_relation.rs"]
mod terminal_relation;
pub(crate) use map_flow::{
    ArrayElementSource, MapDestinationV1, MapEntryBorrowKindV1, MapEntryStoreClassV1, MapHomeEntry,
    MapHomeFlow, MapHomeObservation, MapValueSource, RootHomeExitV1, RootHomeFlow,
};
pub(crate) use terminal_relation::issue_terminal_integer_literal_return_from_completion_v1;
use terminal_relation::{
    map_literal_keys, return_scalar, terminal_map_get, terminal_returned_source, ReturnScalar,
};
pub(crate) use terminal_relation::{
    TerminalCallArgumentV1, TerminalI64AddReturnV1, TerminalI64CallReturnV1,
    TerminalI64FieldReturnV1, TerminalIntegerLiteralReturnV1, TerminalMapGetReceiverClassV1,
    TerminalMapGetReturnV1, TerminalOpaqueCallReturnV1, TerminalRelationV1,
    TerminalReturnedSourceV1, TerminalUnitReturnV1, TerminalValueReturnV1,
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
        &[],
        false,
        &BTreeSet::new(),
        &mut |_, _, _, _, _| Ok::<_, std::convert::Infallible>(false),
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
        &mut |_, _, _, _, _| Ok(false),
        &mut |_, _, _, _, _| Ok(false),
        &mut |_, _, _, _, _| Ok(false),
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
#[path = "home_new_prefix_field_call.rs"]
mod field_call;
#[path = "home_new_prefix_field_write.rs"]
mod field_write;
#[path = "home_new_prefix_scan.rs"]
mod scan;
#[path = "home_new_prefix_terminal.rs"]
mod terminal;
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
    exit_sites: &[SourceStmtSiteV1],
    uncovered_implicit_exit: bool,
    result_sites: &BTreeSet<OwnedExprSiteV1>,
    field_is_integer: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    map_compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
    terminal_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_map_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_handle_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    local_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
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
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
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
                exits: exit_homes
                    .into_iter()
                    .map(|(site, _)| (site, Err(HomePrefixUnavailableV1::SourceMismatch)))
                    .collect(),
                uncovered_implicit_exit,
                maps,
                local_calls,
            },
            terminal_relations,
            argument_observations,
            result_sites
                .iter()
                .map(|site| (site.clone(), Err(HomePrefixUnavailableV1::SourceMismatch)))
                .collect(),
        ));
    };
    let mut locals = PrefixLocalFlow::new(input);
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
    scan_statement_flow(
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
        local_nullable_call,
        argument_i64_field,
        scalar_field,
        container_field,
    )?;
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
            exits: exit_homes,
            uncovered_implicit_exit,
            maps,
            local_calls,
        },
        terminal_relations,
        argument_observations,
        result_prefixes,
    ))
}
