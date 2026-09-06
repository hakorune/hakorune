//! Caller-prefix Home availability, issued from one resolved source loan.
//!
//! A successful prefix describes Normal-path installations, not runtime
//! cleanup readiness. Construction/argument unwind remains a required
//! dependency; unknown prefix meaning never becomes an empty Home list.

use super::{
    BindingRefV1, ExprChildRoleV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::{issue_new_fault_continuation_v1, NewFaultContinuationV1};
use std::collections::BTreeMap;

#[path = "selected_new_arguments.rs"]
mod selected_new_arguments;
pub(crate) use selected_new_arguments::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1,
    SelectedNewArgumentUnavailableV1, SelectedNewArgumentV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HomePrefixUnavailableV1 {
    EntryDemandMissing,
    SourceMismatch,
    PrefixNotCovered(SourceStmtSiteV1),
    ArgumentNotCovered(SourceExprSiteV1),
    OverridesNotCovered(SourceExprSiteV1),
    TerminalNotCovered,
    ReturnValueNotCovered(SourceStmtSiteV1),
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

#[derive(Clone, Copy)]
enum LocalValue {
    Home(BindingRefV1),
    Handle(BindingRefV1),
    Trivial,
    Uninitialized,
}

fn value_class(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &BTreeMap<BindingRefV1, LocalValue>,
) -> Option<LocalValue> {
    if matches!(
        input.function().expression_source().literal(site),
        Some(ResolvedLiteralSourceV1::Integer(_) | ResolvedLiteralSourceV1::Bool(_))
    ) {
        return Some(LocalValue::Trivial);
    }
    let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(site)? else {
        return None;
    };
    match locals.get(&binding)? {
        LocalValue::Home(root) | LocalValue::Handle(root) => Some(LocalValue::Handle(*root)),
        LocalValue::Trivial => Some(LocalValue::Trivial),
        LocalValue::Uninitialized => None,
    }
}

/// Exact source classification for the selected direct-New argument profile.
/// The scanner owns this decision; package code must consume its observation
/// instead of replaying source syntax.
fn selected_new_argument_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &BTreeMap<BindingRefV1, LocalValue>,
) -> Option<SelectedNewArgumentKindV1> {
    match input.function().expression_source().literal(site) {
        Some(ResolvedLiteralSourceV1::Integer(value)) => {
            return Some(SelectedNewArgumentKindV1::Integer(*value));
        }
        Some(ResolvedLiteralSourceV1::Bool(value)) => {
            return Some(SelectedNewArgumentKindV1::Bool(*value));
        }
        _ => {}
    }
    let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(site)? else {
        return None;
    };
    matches!(locals.get(&binding), Some(LocalValue::Trivial))
        .then_some(SelectedNewArgumentKindV1::Local { binding })
}

pub(crate) fn issue_new_home_prefixes_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
) -> BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>> {
    scan_new_home_flow(input, selected, None, &mut |_, _, _, _, _|
        Ok::<_, std::convert::Infallible>(false))
        .unwrap_or_else(|never| match never {}).0
}

/// Source-only terminal shape for the selected ordinary-`New` root.
///
/// This records the decision made by the Completion ownership walk.  It has no
/// physical value, ABI, recipe, JSON, or backend authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalI64AddReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    add_site: OwnedExprSiteV1,
    field_reads: [OwnedExprSiteV1; 2],
}

impl TerminalI64AddReturnV1 {
    fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        add_site: OwnedExprSiteV1,
        field_reads: [OwnedExprSiteV1; 2],
    ) -> Self {
        Self { owner, return_site, add_site, field_reads }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }

    pub(crate) fn add_site(&self) -> &OwnedExprSiteV1 {
        &self.add_site
    }

    pub(crate) fn field_reads(&self) -> &[OwnedExprSiteV1; 2] {
        &self.field_reads
    }
}

/// Exact source relation for a Completion-backed untyped integer literal return.
/// It records source identity and value only; it owns no physical representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalIntegerLiteralReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value: i64,
}
impl TerminalIntegerLiteralReturnV1 {
    fn issue(owner: FunctionOwnerIdV1, return_site: SourceStmtSiteV1, value_site: SourceExprSiteV1, value: i64) -> Self {
        Self { owner, return_site, value_site, value }
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 { &self.return_site }
    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 { &self.value_site }
    pub(crate) const fn value(&self) -> i64 { self.value }
}

/// Exact source relation for a Completion-backed explicit bare return.
/// This contains no physical value or ABI category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalUnitReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
}

impl TerminalUnitReturnV1 {
    fn issue(owner: FunctionOwnerIdV1, return_site: SourceStmtSiteV1) -> Self {
        Self { owner, return_site }
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 { &self.return_site }
}

#[derive(PartialEq, Eq)]
enum ReturnScalar {
    Integer,
    OtherTrivial,
    IntegerField(OwnedExprSiteV1),
    I64Add { site: OwnedExprSiteV1, field_reads: [OwnedExprSiteV1; 2] },
}

// This classifier is terminal-only: argument and prefix-local eligibility
// still belongs to value_class. Field authority is borrowed from the exact
// selected New's source definition, never from runtime layout or MIR types.
fn return_scalar<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &BTreeMap<BindingRefV1, LocalValue>,
    field_is_integer: &mut impl FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str) -> Result<bool, E>,
) -> Result<Option<ReturnScalar>, E> {
    if matches!(input.function().expression_source().literal(site),
        Some(ResolvedLiteralSourceV1::Integer(_))) {
        return Ok(Some(ReturnScalar::Integer));
    }
    if matches!(value_class(input, site, locals), Some(LocalValue::Trivial)) {
        return Ok(Some(ReturnScalar::OtherTrivial));
    }
    let Ok(expr) = input.source().expr_at(&OwnedExprSiteV1::new(input.owner(), site.clone()))
        else { return Ok(None); };
    match expr.node() {
        ASTNode::FieldAccess { field, .. } => {
            let Ok(receiver) = input.source().child_expr_from_expr(&expr, ExprChildRoleV1::Receiver)
                else { return Ok(None); };
            let Some(LocalValue::Handle(home)) = value_class(input, receiver.site(), locals)
                else { return Ok(None); };
            let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(receiver.site())
                else { return Ok(None); };
            let field_site = OwnedExprSiteV1::new(input.owner(), site.clone());
            Ok(field_is_integer(&field_site, receiver.site(), binding, home, field)?
                .then_some(ReturnScalar::IntegerField(field_site)))
        }
        ASTNode::BinaryOp { operator: crate::ast::BinaryOperator::Add, .. } => {
            let Ok(left) = input.source().child_expr_from_expr(&expr, ExprChildRoleV1::BinaryLeft)
                else { return Ok(None); };
            let Ok(right) = input.source().child_expr_from_expr(&expr, ExprChildRoleV1::BinaryRight)
                else { return Ok(None); };
            let left = return_scalar(input, left.site(), locals, field_is_integer)?;
            let right = return_scalar(input, right.site(), locals, field_is_integer)?;
            let add_site = OwnedExprSiteV1::new(input.owner(), site.clone());
            match (left, right) {
                (Some(ReturnScalar::IntegerField(left)), Some(ReturnScalar::IntegerField(right))) => {
                    Ok(Some(ReturnScalar::I64Add { site: add_site, field_reads: [left, right] }))
                }
                (Some(_), Some(_)) => Ok(Some(ReturnScalar::Integer)),
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

/// One source walk supplies both New-failure prefixes and terminal ownership.
/// The caller must take the terminal from the Completion verified on this input.
pub(crate) fn scan_new_home_flow<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    terminal: Option<&SourceStmtSiteV1>,
    field_is_integer: &mut impl FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str) -> Result<bool, E>,
) -> Result<(
    BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    Result<Box<[BindingRefV1]>, HomePrefixUnavailableV1>,
    Option<TerminalI64AddReturnV1>,
    Option<TerminalUnitReturnV1>,
    Option<TerminalIntegerLiteralReturnV1>,
    BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
), E> {
    let mut results = BTreeMap::new();
    let mut terminal_homes = Err(HomePrefixUnavailableV1::TerminalNotCovered);
    let mut terminal_result = None;
    let mut terminal_unit_return = None;
    let mut terminal_integer_literal = None;
    let mut argument_observations = BTreeMap::new();
    if selected.is_empty() && terminal.is_none() {
        return Ok((results, terminal_homes, terminal_result, terminal_unit_return, terminal_integer_literal, argument_observations));
    }
    let function = input.function();
    let mut unavailable = (function.declaration_sites().any(|site| {
        matches!(
            site,
            SourceBindingSiteV1::Receiver | SourceBindingSiteV1::Parameter { .. }
        )
    }) || !input
        .forest()
        .ordered_capture_demands(input.owner())
        .is_empty())
    .then_some(HomePrefixUnavailableV1::EntryDemandMissing);
    let Ok(body) = input.source().root_body() else {
        return Ok((selected
            .keys()
            .map(|site| (site.clone(), Err(HomePrefixUnavailableV1::SourceMismatch)))
            .collect(), Err(HomePrefixUnavailableV1::SourceMismatch), terminal_result, terminal_unit_return,
            terminal_integer_literal, argument_observations));
    };
    let mut locals = BTreeMap::new();
    let mut homes = Vec::new();
    let mut covered_statements = Vec::new();
    for index in 0..body.statements().len() {
        if terminal.is_none() && results.len() == selected.len() {
            break;
        }
        let Ok(statement) = input.source().body_stmt(&body, index) else {
            unavailable = Some(HomePrefixUnavailableV1::SourceMismatch);
            break;
        };
        covered_statements.push(statement.site().clone());
        if terminal == Some(statement.site()) {
            let scalar_return = match statement.node() {
                ASTNode::Return { value: None, .. } => {
                    terminal_unit_return = Some(TerminalUnitReturnV1::issue(
                        input.owner(), statement.site().clone()));
                    true
                }
                ASTNode::Return { value: Some(_), .. } => match input.source()
                    .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue) {
                    Ok(value) => match input.function().expression_source().literal(value.site()) {
                        Some(ResolvedLiteralSourceV1::Integer(number)) => {
                            terminal_integer_literal = Some(TerminalIntegerLiteralReturnV1::issue(
                                input.owner(), statement.site().clone(), value.site().clone(), *number));
                            true
                        }
                        _ => match return_scalar(input, value.site(), &locals, field_is_integer)? {
                            Some(ReturnScalar::I64Add { site, field_reads }) => {
                                terminal_result = Some(TerminalI64AddReturnV1::issue(
                                    input.owner(), statement.site().clone(), site, field_reads));
                                true
                            }
                            Some(_) => true,
                            None => false,
                        },
                    },
                    Err(_) => false,
                },
                _ => false,
            };
            terminal_homes = match &unavailable {
                Some(issue) => Err(issue.clone()),
                None if !scalar_return => Err(HomePrefixUnavailableV1::ReturnValueNotCovered(statement.site().clone())),
                None if results.len() != selected.len() => Err(HomePrefixUnavailableV1::SourceMismatch),
                None => Ok(homes.iter().rev().copied().collect()),
            };
            if terminal_homes.is_err() {
                terminal_result = None;
                terminal_unit_return = None;
                terminal_integer_literal = None;
            }
            break;
        }
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = statement.node()
        else {
            unavailable.get_or_insert_with(|| {
                HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
            });
            continue;
        };
        // Natural syntax permits one initialized local. Do not give synthetic
        // multiple-initializer carriers a premature sequential-install meaning.
        if variables.len() != 1 && initial_values.iter().any(Option::is_some) {
            unavailable.get_or_insert_with(|| {
                HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
            });
        }
        for ordinal in 0..variables.len() {
            let declaration = SourceBindingSiteV1::Local {
                statement: statement.site().clone(),
                ordinal: ordinal as u32,
            };
            let relation = function
                .expression_source()
                .initializers()
                .find(|row| row.declaration_site() == &declaration);
            let Some(relation) = relation else {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                continue;
            };
            let binding = relation.binding();
            if binding.owner() != input.owner()
                || function.declaration_binding(&declaration) != Some(binding)
            {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
            }
            let Some(site) = relation.initializer_site() else {
                locals.insert(binding, LocalValue::Uninitialized);
                continue;
            };
            let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
            if let Some(destination) = selected.get(&owned) {
                if *destination != binding {
                    unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                }
                let located = input.source().expr_at(&owned);
                match located {
                    Ok(new) => match new.node() {
                        ASTNode::New {
                            arguments,
                            field_initializers,
                            ..
                        } => {
                            let observed_arguments = arguments.iter().enumerate().map(|(ordinal, _)| {
                                let ordinal = u32::try_from(ordinal).map_err(|_| {
                                    SelectedNewArgumentUnavailableV1::ArgumentOrdinalOverflow { new_site: owned.clone() }
                                })?;
                                let argument = input.source().child_expr_from_expr(
                                    &new, ExprChildRoleV1::CallArgument(ordinal),
                                ).map_err(|_| SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: owned.clone() })?;
                                let kind = selected_new_argument_kind(input, argument.site(), &locals).ok_or_else(|| {
                                    SelectedNewArgumentUnavailableV1::ArgumentNotTrivial { new_site: owned.clone(), site: argument.site().clone() }
                                })?;
                                Ok(SelectedNewArgumentV1::new(ordinal, argument.site().clone(), kind))
                            }).collect::<Result<Vec<_>, _>>().map(|rows| rows.into_boxed_slice());
                            argument_observations.insert(owned.clone(), SelectedNewArgumentObservationV1::new(owned.clone(), observed_arguments));
                            if !field_initializers.is_empty() {
                                unavailable.get_or_insert_with(|| {
                                    HomePrefixUnavailableV1::OverridesNotCovered(site.clone())
                                });
                            }
                            for argument in 0..arguments.len() {
                                let arg = input.source().child_expr_from_expr(
                                    &new,
                                    ExprChildRoleV1::CallArgument(argument as u32),
                                );
                                match arg {
                                    // Handle arguments require the selected parameter's
                                    // source demand, not merely a physical borrow ABI.
                                    Ok(arg)
                                        if selected_new_argument_kind(
                                            input, arg.site(), &locals,
                                        ).is_some() => {}
                                    Ok(arg) => {
                                        unavailable.get_or_insert_with(|| {
                                            HomePrefixUnavailableV1::ArgumentNotCovered(
                                                arg.site().clone(),
                                            )
                                        });
                                    }
                                    Err(_) => {
                                        unavailable
                                            .get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                                    }
                                }
                            }
                        }
                        _ => {
                            unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                        }
                    },
                    Err(_) => {
                        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                    }
                }
                let result = match &unavailable {
                    Some(issue) => Err(issue.clone()),
                    None => issue_new_fault_continuation_v1(input, &owned)
                        .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)
                        .map(|outward_fault| CallerNewHomePrefixV1 {
                        destination: binding,
                        prior_homes: homes.iter().rev().copied().collect(),
                        outward_fault,
                        covered_statements: covered_statements.clone().into_boxed_slice(),
                    }),
                };
                results.insert(owned, result);
                // This is the Normal successor only, after exact local commit.
                homes.push(binding);
                locals.insert(binding, LocalValue::Home(binding));
            } else if let Some(class) = value_class(input, site, &locals) {
                locals.insert(binding, class);
            } else {
                unavailable.get_or_insert_with(|| {
                    HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone())
                });
            }
        }
    }
    for site in selected.keys() {
        results.entry(site.clone()).or_insert_with(|| {
            Err(unavailable
                .clone()
                .unwrap_or(HomePrefixUnavailableV1::SourceMismatch))
        });
    }
    Ok((results, terminal_homes, terminal_result, terminal_unit_return, terminal_integer_literal, argument_observations))
}
