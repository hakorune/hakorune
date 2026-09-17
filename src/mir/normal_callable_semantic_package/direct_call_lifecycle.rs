//! Co-seal existing source products in the affine direct-call inventory.
//! No target lookup by name, second Completion, or physical continuation is issued.
use super::super::model::OwnedCallableParameterContractDeclarationV1;
use super::super::ordinary_new_coseal::OrdinaryNewClaimLedgerV1;
use super::super::result_contract::VerifiedCallableResultContractCohortV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallResultClassV1, TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, SourceBindingSiteV1};

fn map_owned(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    row: &DirectCallDispositionRowV1,
) -> bool {
    map_owned_owner(batch, row.emission.target().callable().owner())
}

/// The callee's own sealed products already prove it will emit lifecycle
/// sites: a terminal Call relation, or a root flow carrying local calls or
/// Map rows. Header annotations and raw names never decide this; a callee
/// outside every sealed lifecycle product keeps the ordinary Call route.
fn callee_lifecycle_participant(
    root: &OrdinaryNewClaimLedgerV1,
    callee: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> bool {
    if root.call_source_completion_for_owner(callee).is_some() {
        return true;
    }
    root.completion_for_owner(callee)
        .and_then(|completion| completion.cleanup().root_flow())
        .is_some_and(|flow| !flow.local_calls().is_empty() || !flow.maps().is_empty())
}

pub(in crate::mir::normal_callable_semantic_package) fn map_owned_owner(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> bool {
    batch.declarations().any(|declaration| {
        declaration.owner() == owner
            && declaration
                .body_shape()
                .expressions()
                .iter()
                .any(|expression| matches!(expression, BodyExpressionShapeV1::MapLiteral { .. }))
    })
}

/// Whether the callable's sealed body proves a Map result: its own Map
/// coverage exists and its terminal statement returns a Map literal.
/// The header annotation alone never decides this. `return <map-local>`
/// stays outside this source-level proof and fails closed.
pub(in crate::mir::normal_callable_semantic_package) fn map_result_callee(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    callable: crate::mir::resolved_semantics::ResolvedCallableRefV1,
) -> bool {
    let mut declarations = batch
        .declarations()
        .filter(|declaration| declaration.owner() == callable.owner());
    let Some(declaration) = declarations.next() else {
        return false;
    };
    if declarations.next().is_some() {
        return false;
    }
    let body = declaration.body_shape();
    if !body
        .expressions()
        .iter()
        .any(|expression| matches!(expression, BodyExpressionShapeV1::MapLiteral { .. }))
    {
        return false;
    }
    matches!(
        body.statements().last(),
        Some(crate::mir::resolved_semantics::BodyStatementShapeV1::Return {
            value: Some(value_site),
            ..
        }) if matches!(
            body.expression_shape(value_site),
            Some(BodyExpressionShapeV1::MapLiteral { .. })
        )
    )
}

/// The callee's own terminal relation is the sole result-class evidence:
/// a Map-source `return` yields a `Map` call result, every other admitted
/// relation stays `I64`. Declared annotations never decide this.
pub(in crate::mir::normal_callable_semantic_package) fn call_result_kind(
    terminal_relation: Option<&TerminalRelationV1>,
) -> InvokeCallResultKind {
    match terminal_relation {
        Some(TerminalRelationV1::Value(row))
            if matches!(
                row.returned(),
                TerminalReturnedSourceV1::MapLiteral(_) | TerminalReturnedSourceV1::MapLocal(_)
            ) =>
        {
            InvokeCallResultKind::Map
        }
        _ => InvokeCallResultKind::I64,
    }
}

fn exact_formals(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameters: &[OwnedCallableParameterContractDeclarationV1],
    row: &DirectCallDispositionRowV1,
) -> bool {
    let target = row.emission.target();
    if !target.published_key().is_some_and(|key| {
        key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
    }) {
        return false;
    }
    let mut matches = parameters
        .iter()
        .filter(|row| row.owner == target.callable().owner());
    let Some(contract) = matches.next() else {
        return false;
    };
    if matches.next().is_some()
        || contract.parameters.len() != row.argument_sites.len()
        || target.signature().arity() != row.argument_sites.len()
    {
        return false;
    }
    batch
        .with_lowering_input(contract.batch_slot, |input| {
            input.owner() == contract.owner
                && contract
                    .parameters
                    .iter()
                    .enumerate()
                    .all(|(index, parameter)| {
                        parameter.ordinal as usize == index
                            && parameter.kind
                                == CallableParameterContractKindV1::ExactTrivial(
                                    ExactTrivialParameterAbiV1::I64,
                                )
                            && target.signature().params()[index] == ExactTrivialScalarAbiV1::I64
                            && input.function().declaration_binding(
                                &SourceBindingSiteV1::Parameter {
                                    index: parameter.ordinal,
                                },
                            ) == Some(parameter.binding)
                    })
        })
        .unwrap_or(false)
}

impl DirectCallDispositionLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn is_i64_call(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &OwnedExprSiteV1,
    ) -> bool {
        if site.owner() != self.owner || input.owner() != self.owner {
            return false;
        }
        let Some(DirectCallDispositionSlotV1::Ready(row)) = self.rows.get(site) else {
            return false;
        };
        row.emission.target().signature().result() == Some(ExactTrivialScalarAbiV1::I64)
            && input
                .function()
                .direct_call_target(site.site())
                .is_some_and(|target| target.callable() == row.emission.target().callable())
            && input
                .function()
                .direct_call_observations()
                .any(|(observed_site, observation)| {
                    observed_site == site.site()
                        && observation.argument_sites() == row.argument_sites()
                })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn has_map_target(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
    ) -> bool {
        self.rows.values().any(|slot| match slot {
            DirectCallDispositionSlotV1::Ready(row) => map_owned(batch, row),
            DirectCallDispositionSlotV1::Taken => false,
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn map_target_owners(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
    ) -> Option<Box<[FunctionOwnerIdV1]>> {
        let mut owners = std::collections::BTreeSet::new();
        for row in self.rows.values().filter_map(|slot| match slot {
            DirectCallDispositionSlotV1::Ready(row) if map_owned(batch, row) => Some(row),
            DirectCallDispositionSlotV1::Ready(_) | DirectCallDispositionSlotV1::Taken => None,
        }) {
            owners.insert(row.emission.target().callable().owner());
        }
        (!owners.is_empty()).then(|| owners.into_iter().collect())
    }

    /// The affine loan is spent product evidence: any consumed slot at
    /// install means the package crossed a boundary it must not have.
    pub(in crate::mir::normal_callable_semantic_package) fn has_taken_slot(&self) -> bool {
        self.rows
            .values()
            .any(|slot| matches!(slot, DirectCallDispositionSlotV1::Taken))
    }

    /// A local call into an unannotated Map-owned callee: the header's
    /// `None` result is syntax only — the callee's sealed terminal relation
    /// proves the Map result class at co-seal.
    pub(in crate::mir::normal_callable_semantic_package) fn is_map_result_call(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &OwnedExprSiteV1,
    ) -> bool {
        if site.owner() != self.owner || input.owner() != self.owner {
            return false;
        }
        match self.rows.get(site) {
            Some(DirectCallDispositionSlotV1::Ready(row)) => {
                row.emission.target().signature().result().is_none()
                    && map_owned(batch, row)
                    && exact_formals(batch, parameters, row)
                    && input
                        .function()
                        .direct_call_target(site.site())
                        .is_some_and(|target| target.callable() == row.emission.target().callable())
                    && input.function().direct_call_observations().any(
                        |(observed_site, observation)| {
                            observed_site == site.site()
                                && observation.argument_sites() == row.argument_sites()
                        },
                    )
            }
            _ => false,
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn co_seal_lifecycle(
        &mut self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        results: &VerifiedCallableResultContractCohortV1,
        root: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), DirectCallLoanErrorV1> {
        let reject = DirectCallLoanErrorV1::LifecycleSourceMismatch;
        for (site, slot) in &mut self.rows {
            let DirectCallDispositionSlotV1::Ready(row) = slot else {
                return Err(reject);
            };
            // Unavailable Map coverage is still Map-owned, never Scalar evidence.
            if !map_owned(batch, row) {
                // An unannotated target is callable only through the
                // map-result lane; a scalar call into it has no admitted
                // route.
                if row.emission.target().signature().result() != Some(ExactTrivialScalarAbiV1::I64)
                {
                    return Err(reject);
                }
                let Some(local) = root.local_call_for_owner(self.owner, site.site()) else {
                    continue;
                };
                let signature = row.emission.target().signature();
                let (caller, terminal_site) =
                    match root.call_source_completion_for_owner(self.owner) {
                        Some((caller, terminal)) => (caller, Some(terminal.return_site())),
                        None => {
                            // A lifecycle-bearing callee cannot ride the scalar
                            // Call route even under the caller's Plain exit: its
                            // own sealed flow already proves Invoke-bearing
                            // members, so this call must reach it through the
                            // same Invoke edge and the Plain exit entry records
                            // the local binding groups directly. A callee with
                            // no sealed lifecycle product keeps the Call route.
                            let callee = row.emission.target().callable().owner();
                            if !callee_lifecycle_participant(root, callee) {
                                continue;
                            }
                            (root.completion_for_owner(self.owner).ok_or(reject)?, None)
                        }
                    };
                if local.owner() != self.owner
                    || local.site() != site
                    || local.destination().owner() != self.owner
                    || local.arguments().len() != row.argument_sites.len()
                    || local.result() != LocalCallResultClassV1::I64
                    || signature.arity() != row.argument_sites.len()
                    || signature.result() != Some(ExactTrivialScalarAbiV1::I64)
                    || signature
                        .params()
                        .iter()
                        .any(|kind| *kind != ExactTrivialScalarAbiV1::I64)
                    || caller.owner() != self.owner
                    || terminal_site.is_some_and(|site| caller.explicit_site() != Some(site))
                    || !caller.returns_value()
                    || !matches!(caller.cleanup().terminal_homes(), Some(Ok(_)))
                    || !local.prior_homes().is_empty()
                {
                    return Err(reject);
                }
                // The original caller relation owns local placement, not Map
                // membership. Its existing Invoke consumer records the binding.
                row.execution = DirectCallExecutionV1::Lifecycle;
                continue;
            }
            if !exact_formals(batch, parameters, row) {
                return Err(reject);
            }
            // The caller's own exit evidence is required regardless of
            // which statement the call result flows through: a terminal
            // `return <call>` site and a local `local m = <call>` site both
            // consume the same caller terminal Homes accounting.
            let caller = root.completion_for_owner(self.owner).ok_or(reject)?;
            if !caller.returns_value() || !matches!(caller.cleanup().terminal_homes(), Some(Ok(_)))
            {
                return Err(reject);
            }
            // The source scan classifies a local call by the callee's
            // sealed header annotation: `:i64` observations carry the I64
            // class, unannotated map-result callees carry the Map class.
            let expected_class = match row.emission.target().signature().result() {
                Some(ExactTrivialScalarAbiV1::I64) => LocalCallResultClassV1::I64,
                None => LocalCallResultClassV1::Map,
            };
            let completion = root.call_source_completion_for_owner(self.owner);
            let arguments = match completion {
                Some((call_completion, terminal)) if site.site() == terminal.call_site() => {
                    if call_completion.explicit_site() != Some(terminal.return_site())
                        || terminal.arguments().len() != row.argument_sites.len()
                    {
                        return Err(reject);
                    }
                    terminal.arguments()
                }
                _ => {
                    let local = root
                        .local_call_for_owner(self.owner, site.site())
                        .ok_or(reject)?;
                    if local.owner() != self.owner
                        || local.site().site() != site.site()
                        || local.arguments().len() != row.argument_sites.len()
                        || local.result() != expected_class
                    {
                        return Err(reject);
                    }
                    // A scalar call into a map-owned callee stays admitted
                    // only under the caller's terminal-call undertaking.
                    // A Map receive owns its binding and cleanup directly.
                    if local.result() == LocalCallResultClassV1::I64 && completion.is_none() {
                        return Err(reject);
                    }
                    local.arguments()
                }
            };
            let owner = row.emission.target().callable().owner();
            let mut matches = results.rows().filter(|result| result.owner() == owner);
            let callee = matches.next().ok_or(reject)?.borrow();
            if matches.next().is_some()
                || owner == self.owner
                || !callee.completion().returns_value()
            {
                return Err(reject);
            }
            let flow = callee.completion().cleanup().root_flow().ok_or(reject)?;
            if flow.terminal_homes().is_err()
                || flow.maps().iter().any(|map| map.complete().is_none())
                || arguments.len() != row.argument_sites.len()
            {
                return Err(reject);
            }
            // The sealed result contract is the sole result-class
            // authority: the annotation and the callee's terminal relation
            // must agree, and the terminal classifies the result kind.
            match (callee.result(), callee.terminal_relation()) {
                (
                    Some(ExactTrivialScalarAbiV1::I64),
                    Some(TerminalRelationV1::IntegerLiteral(value)),
                ) => {
                    if value.owner() != owner
                        || callee.completion().explicit_site() != Some(value.return_site())
                        || flow.maps().is_empty()
                    {
                        return Err(reject);
                    }
                }
                (None, Some(TerminalRelationV1::Value(value))) => {
                    let covered = match value.returned() {
                        TerminalReturnedSourceV1::MapLiteral(literal) => flow
                            .maps()
                            .iter()
                            .any(|map| map.site() == literal && map.complete().is_some()),
                        TerminalReturnedSourceV1::MapLocal(binding) => {
                            flow.maps().iter().any(|map| {
                                map.complete()
                                    .is_some_and(|row| row.local_binding() == Some(*binding))
                            })
                        }
                        _ => false,
                    };
                    if value.owner() != owner
                        || callee.completion().explicit_site() != Some(value.return_site())
                        || !covered
                    {
                        return Err(reject);
                    }
                }
                _ => return Err(reject),
            }
            row.result = call_result_kind(callee.terminal_relation());
            row.execution = DirectCallExecutionV1::Lifecycle;
        }
        Ok(())
    }
}
