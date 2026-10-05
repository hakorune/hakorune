//! Source-result projection for borrowed callers of the existing lexical owner.
use super::borrowed_formal_actuals::PendingBorrowedFormalActualsV1;
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, ResolvedLiteralSourceV1, SourceBindingSiteV1, SourceExprSiteV1,
};
use std::collections::{BTreeMap, BTreeSet};

#[path = "ordinary_new_borrowed_formal_result_composition.rs"]
mod composition;

/// The sole result class a borrowed callee's uniform return sites prove.
/// `I64` is the existing literal/exact-formal scalar lane; `Nullable` is
/// the borrowed-result nullable-handle class — every explicit value-return
/// site is `return null` or `return new ..`, the callee carries a
/// `NullableObject` claim, and the caller's invoke mints `NullableHandle`.
/// Mixed or unproven forms stay rejected — never a silent scalar fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BorrowedResultClassV1 {
    I64,
    Nullable,
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct BorrowedI64ResultSourceV1 {
    pub(super) returns: Box<[OwnedExprSiteV1]>,
    pub(super) class: BorrowedResultClassV1,
    pub(super) contract_corroborated: bool,
    /// Original call rows, never result permission inferred from an annotation.
    dependencies: Box<[LexicalInstanceCallSourceTargetV1]>,
}

/// `return <formal>.<field>` — a guarded borrowed-formal field read is an
/// I64 result when the callee's own sealed draft admitted this exact
/// FieldAccess site, the co-sealed object view was minted for the exact
/// formal binding, and the field is declared `i64` on the view's class —
/// the same declaration authority the received-nullable lane enforces.
/// Aliases, undominated reads and object-typed fields never classify.
fn guarded_formal_i64_field(
    input: ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    source: &PreparedBorrowedFormalIngressV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    owner: FunctionOwnerIdV1,
    contract: &OwnedCallableParameterContractDeclarationV1,
    site: &SourceExprSiteV1,
) -> Result<bool, String> {
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) = input
        .body_shape()
        .and_then(|shape| shape.expression_shape(site))
    else {
        return Ok(false);
    };
    // The receiver must be the exact parameter binding — a rebound alias
    // keeps its own binding and stays unclassified here.
    let formal = match input.function().variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding))
            if contract
                .parameters
                .iter()
                .any(|parameter| parameter.binding == binding) =>
        {
            binding
        }
        _ => return Ok(false),
    };
    let owned_site = OwnedExprSiteV1::new(owner, site.clone());
    // The callee's sealed use draft must admit this exact FieldAccess —
    // an undominated, in-arm or ambiguous read never entered the draft.
    let admitted = source.definitions.get(&owner).is_some_and(|draft| {
        draft.uses.iter().any(|row| {
            row.formal == formal
                && matches!(
                    row.kind,
                    BorrowedFormalUseDraftKindV1::FieldReadOperand { site: ref read }
                        if *read == owned_site
                )
        })
    });
    if !admitted {
        return Ok(false);
    }
    let Some(view) = source.formal_object_view(formal) else {
        return Ok(false);
    };
    crate::mir::normal_callable_semantic_package::ordinary_new_coseal::nullable_result_integer_field(
        instance_constructors,
        batch.ordinary_box_coverage(),
        view.class(),
        &owned_site,
        field,
    )
    .map(|field| field.is_some())
    .map_err(|error| format!("{}: {error:?}", freeze("borrowed-result/field-authority")))
}

fn source_result(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    source: &PreparedBorrowedFormalIngressV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    owner: FunctionOwnerIdV1,
) -> Result<BorrowedI64ResultSourceV1, String> {
    let mut matches = contracts.iter().filter(|row| row.owner == owner);
    let contract = matches
        .next()
        .ok_or_else(|| freeze("borrowed-result/source-contract-missing"))?;
    if matches.next().is_some() {
        return Err(freeze("borrowed-result/source-contract-duplicate"));
    }
    batch
        .with_lowering_input(contract.batch_slot, |input| {
            let identity_matches = input.owner() == owner
                && contract
                    .parameters
                    .iter()
                    .enumerate()
                    .all(|(index, parameter)| {
                        parameter.ordinal as usize == index
                            && parameter.binding.owner() == owner
                            && input.function().declaration_binding(
                                &SourceBindingSiteV1::Parameter {
                                    index: parameter.ordinal,
                                },
                            ) == Some(parameter.binding)
                    });
            if !identity_matches {
                return Err(freeze("borrowed-result/source-contract-identity"));
            }
            let sites = input
                .body_shape()
                .and_then(|shape| super::super::verified_value_return_sites(input, shape))
                .filter(|sites| !sites.is_empty())
                .ok_or_else(|| freeze("borrowed-result/explicit-value-return-missing"))?;
            let mut class = None;
            let mut has_construction = false;
            let mut dependencies = Vec::new();
            for site in &sites {
                let function = input.function();
                let integer = matches!(
                    function.expression_source().literal(site),
                    Some(ResolvedLiteralSourceV1::Integer(_))
                );
                let exact_formal = match function.variable_ref(site) {
                    Some(ResolvedLexicalRefV1::Local(binding)) => {
                        contract.parameters.iter().any(|parameter| {
                            parameter.binding == binding
                                && parameter.kind
                                    == CallableParameterContractKindV1::ExactTrivial(
                                        ExactTrivialParameterAbiV1::I64,
                                    )
                        })
                    }
                    _ => false,
                };
                // `return null` is the sealed literal projection and
                // `return new ..` is the sealed construction inventory —
                // together they form the bounded nullable-result class.
                // A `return <formal>.<i64 field>` on the guarded view joins
                // the scalar lane through its own draft/view/declaration
                // proof. Anything else (String/Float/Bool, variable
                // handles, opaque subtrees) is not an admitted result form.
                let site_class = if integer
                    || exact_formal
                    || guarded_formal_i64_field(
                        input,
                        batch,
                        source,
                        instance_constructors,
                        owner,
                        contract,
                        site,
                    )?
                    || composition::retain_call_dependency(
                        input,
                        source,
                        owner,
                        site,
                        &mut dependencies,
                    )? {
                    BorrowedResultClassV1::I64
                } else if matches!(
                    function.expression_source().literal(site),
                    Some(ResolvedLiteralSourceV1::Null)
                ) || function.expression_source().construction(site).is_some()
                {
                    BorrowedResultClassV1::Nullable
                } else {
                    return Err(freeze("borrowed-result/source-not-i64"));
                };
                has_construction |= function.expression_source().construction(site).is_some();
                match class {
                    None => class = Some(site_class),
                    Some(existing) if existing == site_class => {}
                    // A callee whose exits split scalar i64 and nullable
                    // object forms has no single borrowed result contract.
                    Some(_) => return Err(freeze("borrowed-result/source-class-mixed")),
                }
            }
            let class = class.expect("explicit value-return sites are non-empty");
            // A nullable result is an object-or-null contract: without a
            // `return new ..` exit no `NullableObject` claim can name the
            // carried class, so a null-only body stays unclassified rather
            // than drifting onto the old literal-i64 default.
            if class == BorrowedResultClassV1::Nullable && !has_construction {
                return Err(freeze("borrowed-result/source-not-i64"));
            }
            Ok(BorrowedI64ResultSourceV1 {
                returns: sites
                    .into_iter()
                    .map(|site| OwnedExprSiteV1::new(owner, site))
                    .collect(),
                class,
                contract_corroborated: false,
                dependencies: dependencies.into_boxed_slice(),
            })
        })
        .map_err(|_| freeze("borrowed-result/source-loan"))?
}

// Prepare from the same source loan before either prefix walk. Move this
// projection into the existing ledger afterwards; never resolve it again
// from an already completed caller flow.
pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_i64_results_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>> {
    let mut results = match source {
        Ok(rows) => rows
            .definitions
            .keys()
            .map(|owner| {
                (
                    *owner,
                    source_result(batch, contracts, rows, instance_constructors, *owner),
                )
            })
            .collect(),
        // The source error remains mandatory at selected demand.
        Err(_) => BTreeMap::new(),
    };
    composition::ground_source_results(&mut results);
    results
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn install_borrowed_formal_preparation_v1(
        &mut self,
        source: Result<PreparedBorrowedFormalIngressV1, String>,
        mut actuals: PendingBorrowedFormalActualsV1,
        results: BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
    ) {
        super::borrowed_formal_actuals::finish_borrowed_call_actuals_v1(&source, &mut actuals);
        self.borrowed_i64_results = results;
        self.borrowed_formal_source = Some(source);
        self.borrowed_formal_actuals = actuals;
    }

    pub(super) fn corroborate_borrowed_i64_result_v1(
        &mut self,
        source: &LexicalInstanceCallSourceTargetV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) {
        let Some(pending) = self.borrowed_i64_results.get_mut(&source.callee_owner()) else {
            return;
        };
        let Ok(proof) = pending else {
            return;
        };
        let expected: BTreeSet<_> = proof.returns.iter().cloned().collect();
        let class = proof.class;
        let agrees = results.row(source.target_batch_slot()).is_some_and(|row| {
            let borrowed = row.borrow();
            let completion = borrowed.completion();
            let observed: BTreeSet<_> = completion
                .explicit_sites()
                .iter()
                .map(|exit| {
                    let value =
                        crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
                            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Value)
                            .expr();
                    OwnedExprSiteV1::new(completion.owner(), value)
                })
                .collect();
            // The declared result contract must agree with the class the
            // source sites already proved: an exact `i64` annotation for
            // the scalar lane, or an unannotated declaration whose complete
            // source-I64 return set already proved the same class — the
            // declaration stays Unannotated and the executable I64 is this
            // borrowed projection, never a manufactured annotation. `Void`
            // also reports `result() == None` and does not acquire the
            // permission. The nullable lane keeps its unannotated declaration
            // plus the sealed `NullableObject` claim.
            let result_agrees = match class {
                BorrowedResultClassV1::I64 => {
                    row.result()
                        == Some(crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64)
                        || (row.result().is_none()
                            && matches!(
                                completion.function_exit_contract().declared_result(),
                                crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Unannotated
                            ))
                }
                BorrowedResultClassV1::Nullable => {
                    row.result().is_none()
                        && matches!(
                            self.callable_result_classes.get(source.target()),
                            Some(
                                crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(_)
                            )
                        )
                }
            };
            row.owner() == source.callee_owner()
                && completion.owner() == source.callee_owner()
                && completion.returns_value()
                && expected.len() == proof.returns.len()
                && observed == expected
                && result_agrees
        });
        if agrees {
            proof.contract_corroborated = true;
        } else {
            *pending = Err(freeze("borrowed-result/result-contract-mismatch"));
        }
    }

    /// Source-phase corroboration precedes old root selection and final result issuance.
    pub(crate) fn borrowed_terminal_arguments_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Result<
        Option<Box<[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>>,
        String,
    > {
        use crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1;
        let Some(crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Call(
            terminal,
        )) = self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        if !terminal
            .arguments()
            .iter()
            .any(|arg| matches!(arg, TerminalCallArgumentV1::Lexical(_)))
        {
            return Ok(None);
        }
        let completion = self
            .completion_for_owner(owner)
            .ok_or_else(|| freeze("borrowed-terminal/completion-missing"))?;
        let direct_value = crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Value)
            .expr();
        if completion.owner() != owner
            || !completion.returns_value()
            || !completion.explicit_sites().contains(exit)
            || terminal.owner() != owner
            || terminal.return_site() != exit
            || terminal.call_site() != &direct_value
        {
            return Err(freeze("borrowed-terminal/return-site"));
        }
        let site = OwnedExprSiteV1::new(owner, terminal.call_site().clone());
        let arguments = terminal
            .arguments()
            .iter()
            .map(|argument| match argument {
                TerminalCallArgumentV1::Lexical(argument) => Ok(argument.clone()),
                _ => Err(freeze("borrowed-terminal/ordered-arguments")),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        let homes = completion
            .cleanup()
            .root_flow()
            .and_then(|flow| flow.exit_row(exit).and_then(Result::ok))
            .ok_or_else(|| freeze("borrowed-terminal/home-row-missing"))?;
        if !self.validate_borrowed_terminal_arguments_v1(&site, &arguments, homes.homes())? {
            return Err(freeze("borrowed-terminal/incoming-missing"));
        }
        Ok(Some(arguments))
    }

    /// The private shared source sealer owns strict-node membership. Demand
    /// the original borrowed projection at every selected descendant; None
    /// cannot erase a BorrowedActual. A strict sibling may contain no borrow.
    fn validate_borrowed_terminal_arguments_v1(
        &self,
        site: &OwnedExprSiteV1,
        arguments: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
        homes: &[BindingRefV1],
    ) -> Result<bool, String> {
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-terminal/source-missing"))?;
        if let Some(expected) =
            super::borrowed_formal_actuals::project_pending_borrowed_i64_arguments_v1(
                source,
                &self.borrowed_formal_actuals,
                &self.borrowed_i64_results,
                site,
            )?
        {
            if arguments != expected.as_ref() {
                return Err(freeze("borrowed-terminal/ordered-arguments"));
            }
            return Ok(true);
        }
        let mut borrowed = false;
        for (ordinal, argument) in arguments.iter().enumerate() {
            match argument {
                LocalCallArgumentV1::Integer(_) => {}
                LocalCallArgumentV1::Scalar(binding) if binding.owner() == site.owner() => {}
                LocalCallArgumentV1::BorrowedActual { .. } => {
                    return Err(freeze("borrowed-terminal/incoming-missing"));
                }
                LocalCallArgumentV1::CallResult(inner) => {
                    let expected =
                        crate::mir::resolved_semantics::SourcePathV1::from_node(site.site().node())
                            .child(
                                crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(
                                    ordinal as u32,
                                ),
                            )
                            .expr();
                    if inner.site().owner() != site.owner()
                        || inner.site().site() != &expected
                        || inner.prior_homes() != homes
                    {
                        return Err(freeze("borrowed-terminal/nested-site-or-homes"));
                    }
                    borrowed |= self.validate_borrowed_terminal_arguments_v1(
                        inner.site(),
                        inner.arguments(),
                        homes,
                    )?;
                }
                _ => return Err(freeze("borrowed-terminal/ordered-arguments")),
            }
        }
        Ok(borrowed)
    }

    /// Read the original sealed terminal trees, without a second site inventory.
    pub(super) fn terminal_lexical_call_selected_v1(&self, site: &OwnedExprSiteV1) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::{
            LocalCallArgumentV1, TerminalCallArgumentV1,
        };
        fn contains(arguments: &[LocalCallArgumentV1], site: &OwnedExprSiteV1) -> bool {
            arguments.iter().any(|argument| match argument {
                LocalCallArgumentV1::CallResult(inner) => {
                    inner.site() == site || contains(inner.arguments(), site)
                }
                _ => false,
            })
        }
        self.call_relations_for_owner(site.owner())
            .iter()
            .any(|terminal| {
                terminal.arguments().iter().any(|argument| match argument {
                    TerminalCallArgumentV1::Lexical(argument) => {
                        terminal.call_site() == site.site()
                            || contains(std::slice::from_ref(argument), site)
                    }
                    _ => false,
                })
            })
    }

    pub(super) fn corroborate_terminal_lexical_result_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        if results.row(source.target_batch_slot()).is_some_and(|row| {
            let borrowed = row.borrow();
            let completion = borrowed.completion();
            // Direct-return corroboration borrows the same complete
            // source-result evidence as the local-call lane: an annotated
            // `i64` row, or the unannotated callee whose I64 proof already
            // passed `corroborate_borrowed_i64_result_v1`.
            let unannotated_proven = row.result().is_none()
                && self
                    .borrowed_i64_results
                    .get(&source.callee_owner())
                    .and_then(|proof| proof.as_ref().ok())
                    .is_some_and(|proof| {
                        proof.class == BorrowedResultClassV1::I64 && proof.contract_corroborated
                    });
            row.owner() == source.callee_owner()
                && (row.result()
                    == Some(crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64)
                    || unannotated_proven)
                && completion.owner() == source.callee_owner()
                && completion.returns_value()
                && !completion.explicit_sites().is_empty()
        }) {
            Ok(())
        } else {
            Err(freeze("lexical-instance-call/terminal-result-mismatch"))
        }
    }

    /// Issuance must cover every node of the same sealed tree. Missing rows
    /// cannot disappear merely because the prepared-source loop skipped them.
    pub(super) fn validate_terminal_lexical_ready_v1(&self) -> Result<(), String> {
        use crate::mir::resolved_semantics::home_new_prefix::{
            LocalCallArgumentV1, TerminalCallArgumentV1, TerminalRelationV1,
        };
        fn check(
            site: &OwnedExprSiteV1,
            arguments: &[LocalCallArgumentV1],
            rows: &BTreeMap<OwnedExprSiteV1, LexicalInstanceCallDispositionSlotV1>,
        ) -> Result<(), String> {
            let Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) = rows.get(site) else {
                return Err(freeze("lexical-instance-call/terminal-disposition-missing"));
            };
            if row.call_site() != site
                || row.result() != Some(crate::mir::instruction::InvokeCallResultKind::I64)
                || row.argument_sites().len() != arguments.len()
                || row.target().arity() as usize != arguments.len()
            {
                return Err(freeze("lexical-instance-call/terminal-source-mismatch"));
            }
            for (argument, source_site) in arguments.iter().zip(row.argument_sites()) {
                if let LocalCallArgumentV1::CallResult(inner) = argument {
                    if inner.site().owner() != site.owner() || inner.site().site() != source_site {
                        return Err(freeze(
                            "lexical-instance-call/terminal-nested-source-mismatch",
                        ));
                    }
                    check(inner.site(), inner.arguments(), rows)?;
                }
            }
            Ok(())
        }
        let rows = self.lexical_instance_calls.borrow();
        for relation in self
            .terminal_relation
            .values()
            .filter(|relation| !self.terminal_relation_index.contains_key(&relation.owner()))
            .chain(
                self.terminal_relation_index
                    .values()
                    .flat_map(|index| index.values()),
            )
        {
            let TerminalRelationV1::Call(terminal) = relation else {
                continue;
            };
            if !terminal
                .arguments()
                .iter()
                .any(|argument| matches!(argument, TerminalCallArgumentV1::Lexical(_)))
            {
                continue;
            }
            let arguments = terminal
                .arguments()
                .iter()
                .map(|argument| match argument {
                    TerminalCallArgumentV1::Lexical(argument) => Ok(argument.clone()),
                    _ => Err(freeze("borrowed-terminal/ordered-arguments")),
                })
                .collect::<Result<Vec<_>, _>>()?;
            check(
                &OwnedExprSiteV1::new(terminal.owner(), terminal.call_site().clone()),
                &arguments,
                &rows,
            )?;
        }
        Ok(())
    }

    /// Retain the original Taken lexical row; no second root-instance slot exists.
    pub(crate) fn take_borrowed_lexical_call_for_return_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Result<Option<LexicalInstanceCallDispositionRowV1>, String> {
        if self.borrowed_terminal_arguments_v1(owner, exit)?.is_none() {
            return Ok(None);
        }
        let (_, terminal) = self
            .call_source_completion_for_owner_at(owner, exit)
            .ok_or_else(|| freeze("borrowed-terminal/source-missing"))?;
        let row = self
            .take_lexical_instance_call(owner, terminal.call_site())?
            .ok_or_else(|| freeze("borrowed-terminal/disposition-missing"))?;
        // A strict outer node has no borrowed incoming edge of its own. Its
        // selected result was corroborated by the same final lexical issuer.
        self.borrowed_call_actuals_v1(&row)?;
        if row.result() != Some(crate::mir::instruction::InvokeCallResultKind::I64) {
            return Err(freeze("borrowed-terminal/result-not-i64"));
        }
        Ok(Some(row))
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_result_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_terminal_tests.rs"]
mod terminal_tests;
