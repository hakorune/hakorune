//! Caller-prefix Home availability, issued from one resolved source loan.
//!
//! A successful prefix describes Normal-path installations, not runtime
//! cleanup readiness. Construction/argument unwind remains a required
//! dependency; unknown prefix meaning never becomes an empty Home list.

use super::{
    BindingRefV1, ExprChildRoleV1, OwnedExprSiteV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::{issue_new_fault_continuation_v1, NewFaultContinuationV1};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HomePrefixUnavailableV1 {
    EntryDemandMissing,
    SourceMismatch,
    PrefixNotCovered(SourceStmtSiteV1),
    ArgumentNotCovered(SourceExprSiteV1),
    OverridesNotCovered(SourceExprSiteV1),
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
        Some(ResolvedLiteralSourceV1::Integer(_) | ResolvedLiteralSourceV1::Bool)
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

pub(crate) fn issue_new_home_prefixes_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
) -> BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>> {
    let mut results = BTreeMap::new();
    if selected.is_empty() {
        return results;
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
        return selected
            .keys()
            .map(|site| (site.clone(), Err(HomePrefixUnavailableV1::SourceMismatch)))
            .collect();
    };
    let mut locals = BTreeMap::new();
    let mut homes = Vec::new();
    let mut covered_statements = Vec::new();
    for index in 0..body.statements().len() {
        if results.len() == selected.len() {
            break;
        }
        let Ok(statement) = input.source().body_stmt(&body, index) else {
            unavailable = Some(HomePrefixUnavailableV1::SourceMismatch);
            break;
        };
        covered_statements.push(statement.site().clone());
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
                                        if matches!(
                                            value_class(input, arg.site(), &locals),
                                            Some(LocalValue::Trivial)
                                        ) => {}
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
    results
}
