//! Callable result handoff retains the original result owner through finalization.
//! This classifies a carrier only; input, Call, Return and End admission stay separate.
use super::*;
use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
use crate::mir::resolved_control_flow::{DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1};
use crate::mir::resolved_semantics::{SourcePathV1, SourcePathSegmentV1};
use super::super::super::result_class_claim::{
    BorrowedFormalReturnSourceLoanV1, ResultOriginWitnessV1, ResultValueOriginV1,
    ResultWitnessStepV1, OrdinaryNewResultClassV1,
};

/// One prepared body result. Scalar and nullable identity cannot both claim it.
#[derive(Debug)]
pub(crate) enum CallableResultSourceV1 {
    Scalar(CallableScalarResultSourceLoanV1),
    BorrowedNullable(CallableBorrowedNullableResultSourceLoanV1),
}
impl CallableResultSourceV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        match self { Self::Scalar(source) => source.owner(), Self::BorrowedNullable(source) => source.owner }
    }
    pub(crate) fn scalar(&self) -> Option<&CallableScalarResultSourceLoanV1> {
        match self { Self::Scalar(source) => Some(source), _ => None }
    }
    pub(crate) fn borrowed_nullable(&self) -> Option<&CallableBorrowedNullableResultSourceLoanV1> {
        match self { Self::BorrowedNullable(source) => Some(source), _ => None }
    }
}

#[derive(Debug, Clone)]
enum CallableBorrowedResultOriginV1 {
    Formal { binding: BindingRefV1, ordinal: u32, returns: Box<[BorrowedFormalReturnSourceLoanV1]> },
    MapSlot {
        source: Rc<crate::mir::normal_callable_semantic_package::MapStoredResultSourceV1>,
        returns: Box<[crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1]>,
    },
}

/// Same immutable Return requirements, all source roots and Completion.
/// No new result law, selected input, executable row or ownership is issued.
#[derive(Debug, Clone)]
pub(crate) struct CallableBorrowedNullableResultSourceLoanV1 {
    owner: FunctionOwnerIdV1,
    key: CanonicalSameModuleCallableKeyV1,
    origin: CallableBorrowedResultOriginV1,
    completion: Rc<VerifiedFunctionCompletionV1>,
    roots: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl CallableBorrowedNullableResultSourceLoanV1 {
    fn validate_source(&self) -> Result<(), String> {
        if self.completion.owner() != self.owner || !self.completion.returns_value()
            || self.completion.implicit_body_end().is_some()
            || !matches!(self.completion.function_exit_contract().declared_result(), DeclaredFunctionResultContractV1::Unannotated)
            || self.key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || self.roots.is_empty()
        { return Err(freeze("callable-result/nullable-source-contract")); }
        let expected: BTreeSet<_> = self.completion.explicit_sites().iter().map(|site|
            SourcePathV1::from_node(site.node()).child(SourcePathSegmentV1::Value).expr()).collect();
        let actual: BTreeSet<_> = self.roots.iter().map(|root| root.site().site().clone()).collect();
        if expected.is_empty() || actual != expected
            || self.roots.iter().any(|root| root.site().owner() != self.owner)
        { return Err(freeze("callable-result/nullable-return-coverage")); }
        match &self.origin {
            CallableBorrowedResultOriginV1::Formal { binding, ordinal, returns } => {
                if *ordinal >= self.key.arity() || binding.owner() != self.owner || returns.is_empty() {
                    return Err(freeze("callable-result/nullable-source-contract"));
                }
                let mut formal_sites = BTreeSet::new();
                for source in returns.iter() {
                    if source.key() != &self.key || source.formal() != *binding
                        || source.ordinal() != *ordinal || source.value().owner() != self.owner
                        || !formal_sites.insert(source.value().site().clone())
                        || source.roots().len() != self.roots.len()
                        || source.roots().iter().zip(self.roots.iter()).any(|(a,b)| !Rc::ptr_eq(a,b))
                    { return Err(freeze("callable-result/nullable-formal-source")); }
                }
                let mut actual_formals = BTreeSet::new();
                for root in &self.roots {
                    match (root.origin(), root.step()) {
                        (ResultValueOriginV1::Null, ResultWitnessStepV1::NullLiteral) => {},
                        (ResultValueOriginV1::Null | ResultValueOriginV1::ForwardFormal { .. },
                            ResultWitnessStepV1::Formal { binding: actual_binding, ordinal: actual_ordinal })
                            if *actual_binding == *binding && *actual_ordinal == *ordinal
                                && (root.origin() == &ResultValueOriginV1::Null
                                    || root.origin() == &ResultValueOriginV1::ForwardFormal { ordinal: *ordinal }) => {
                            actual_formals.insert(root.site().site().clone());
                        },
                        _ => return Err(freeze("callable-result/nullable-root-role")),
                    }
                }
                if actual_formals != formal_sites { return Err(freeze("callable-result/nullable-formal-coverage")); }
            }
            CallableBorrowedResultOriginV1::MapSlot { source, returns } => {
                use crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1 as Returned;
                if !source.corroborates_roots_v1(self.owner, &self.roots)
                    || source.key_binding_v1().owner() != self.owner || returns.len() != expected.len() {
                    return Err(freeze("callable-result/nullable-root-role"));
                }
                let flow = self.completion.cleanup().root_flow()
                    .ok_or_else(|| freeze("callable-result/nullable-source-contract"))?;
                let mut sites = BTreeSet::new();
                for returned in returns {
                    let at = OwnedExprSiteV1::new(self.owner, returned.value_site().clone());
                    if returned.owner() != self.owner || !sites.insert(returned.value_site().clone())
                        || !self.completion.explicit_sites().contains(returned.return_site())
                        || SourcePathV1::from_node(returned.return_site().node()).child(SourcePathSegmentV1::Value).expr()
                            != *returned.value_site() {
                        return Err(freeze("callable-result/nullable-return-coverage"));
                    }
                    match returned.returned() {
                        Returned::StoredMapGet { source: same, key, .. }
                            if Rc::ptr_eq(source, same) && key.reference_coverage_is_exact()
                                && key.root_class() == crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                                    crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String)
                                && source.present_get_v1(&at).is_some_and(|get|
                                matches!(key.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                    statement, call, ordinal: 0 } if statement == returned.return_site() && std::sync::Arc::ptr_eq(call, get))) => {},
                        Returned::StoredMapInstalled(install) if Rc::ptr_eq(install.source(), source)
                            && source.installed_local_v1(&at).is_some_and(|(binding, construction, set)|
                                binding == install.binding() && construction.site() == install.acquisition().site()
                                    && set.owner() == install.site().owner() && set.site() == install.site().site())
                            && flow.maps().iter().any(|row| matches!(row,
                                crate::mir::resolved_semantics::home_new_prefix::MapHomeObservation::FieldInstall(same)
                                    if Rc::ptr_eq(same, install))) => {},
                        _ => return Err(freeze("callable-result/nullable-root-role")),
                    }
                }
                if sites != expected { return Err(freeze("callable-result/nullable-return-coverage")); }
            }
        }
        Ok(())
    }

    /// Join the existing whole-result loan to its original LOCAL caller.
    /// Require logical Call completion here; physical Return packets keep their gate.
    pub(in crate::mir::normal_callable_semantic_package) fn require_local_source_v1(
        &self, local: &super::super::super::result_class_claim::BorrowedNullableLocalSourceLoanV1,
        completion: &Rc<VerifiedFunctionCompletionV1>, roots: &[&Rc<ResultOriginWitnessV1>],
    ) -> Result<(), String> {
        self.validate_source()?;
        let support = match &self.origin {
            CallableBorrowedResultOriginV1::Formal { ordinal, .. } =>
                crate::mir::instruction::BorrowedResultSupportV1::Formal(*ordinal),
            CallableBorrowedResultOriginV1::MapSlot { source, .. } => source.result_support_v1(),
        };
        if self.owner != local.callee_owner() || self.key != *local.key()
            || !Rc::ptr_eq(&self.completion, completion) || support != local.result_support_v1()
            || matches!(self.origin, CallableBorrowedResultOriginV1::MapSlot { .. }) != local.is_map_slot()
            || roots.len() != self.roots.len()
            || roots.iter().zip(&self.roots).any(|(a, b)| !Rc::ptr_eq(a, b)) {
            return Err(freeze("callable-result/nullable-source-contract"));
        }
        if let CallableBorrowedResultOriginV1::MapSlot { returns, .. } = &self.origin {
            let flow = self.completion.cleanup().root_flow()
                .ok_or_else(|| freeze("callable-result/nullable-source-contract"))?;
            for returned in returns {
                let exit = flow.exit_row(returned.return_site())
                    .ok_or_else(|| freeze("callable-result/nullable-return-coverage"))?
                    .map_err(|_| freeze("callable-result/nullable-return-coverage"))?;
                if let crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::StoredMapInstalled(install) = returned.returned() {
                    if install.prior_homes().iter().filter(|home| home.binding() == install.binding()).count() != 1
                        || !install.normal_homes().iter().eq(install.prior_homes().iter()
                            .filter(|home| home.binding() != install.binding()))
                        || exit.homes().iter().any(|home| home.binding() == install.binding()) {
                        return Err(freeze("callable-result/nullable-root-role"));
                    }
                }
            }
        }
        Ok(())
    }

    /// Structural correspondence only. Existing source exit/FinishedBindings
    /// validators still prove each Return operand and its exact original origin.
    pub(crate) fn validate_function(&self, function: &crate::mir::MirFunction) -> Result<(), String> {
        self.validate_source()?;
        let returns: Vec<_> = function.blocks.values().flat_map(|block| block.all_instructions())
            .filter_map(|row| match row { crate::mir::MirInstruction::Return { value } => Some(value), _ => None }).collect();
        if function.signature.name != self.key.mir_symbol_projection()
            || function.params.len() != self.key.arity() as usize + 1
            || function.signature.params.len() != function.params.len()
            || returns.len() != self.completion.explicit_sites().len()
            || returns.iter().any(|value| value.is_none())
        { return Err(freeze("callable-result/nullable-function-correspondence")); }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn callable_result_source_v1(&self, owner: FunctionOwnerIdV1)
        -> Result<Option<CallableResultSourceV1>, String> {
        let scalar = self.callable_scalar_result_source_v1(owner)?;
        let borrowed = self.callable_borrowed_nullable_result_source_v1(owner)?;
        match (scalar, borrowed) {
            (Some(_), Some(_)) => Err(freeze("callable-result/source-overlap")),
            (Some(source), None) => Ok(Some(CallableResultSourceV1::Scalar(source))),
            (None, Some(source)) => Ok(Some(CallableResultSourceV1::BorrowedNullable(source))),
            (None, None) => Ok(None),
        }
    }
    fn callable_borrowed_nullable_result_source_v1(&self, owner: FunctionOwnerIdV1)
        -> Result<Option<CallableBorrowedNullableResultSourceLoanV1>, String> {
        let mut map_keys = self.callable_result_classes.source_rows().filter(|(_, exits)|
            exits.iter().flat_map(|exit| exit.witnesses()).any(|root|
                root.site().owner() == owner && matches!(root.step(), ResultWitnessStepV1::MapStored { .. })));
        if let Some((key, _)) = map_keys.next() {
            if map_keys.next().is_some() { return Err(freeze("callable-result/source-overlap")); }
            let roots: Box<[_]> = self.callable_result_classes.checked_original_source_roots_v1(key, owner)
                .map_err(freeze)?.into_iter().cloned().collect();
            let first = roots.first().ok_or_else(|| freeze("callable-result/nullable-return-coverage"))?;
            let source = self.callable_result_classes.map_stored_source_v1(first.site()).map_err(freeze)?
                .ok_or_else(|| freeze("callable-result/nullable-root-role"))?;
            source.receiver_v1().require_entry_source_v1(self)?;
            let crate::mir::instruction::BorrowedResultSupportV1::ReceiverFieldSlot { key_ordinal, .. } = source.result_support_v1()
                else { return Err(freeze("callable-result/nullable-root-role")); };
            let catalog = self.original_parameter_contracts.as_deref()
                .ok_or_else(|| freeze("callable-result/nullable-catalog"))?;
            let mut contracts = catalog.iter().filter(|row| row.owner == owner);
            let contract = contracts.next().ok_or_else(|| freeze("callable-result/nullable-parameter"))?;
            let (selected, _) = self.original_instance_input_issuance.as_ref()
                .ok_or_else(|| freeze("callable-result/nullable-catalog"))?;
            if contracts.next().is_some() || key_ordinal >= key.arity()
                || selected.key_for_batch_slot(contract.batch_slot)
                    != Some(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key.clone()))
                || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                || contract.parameters.len() != key.arity() as usize
                || contract.parameters.iter().enumerate().any(|(ordinal, row)| row.ordinal as usize != ordinal || row.binding.owner() != owner)
                || contract.parameters.get(key_ordinal as usize).is_none_or(|row| row.binding != source.key_binding_v1()) {
                return Err(freeze("callable-result/nullable-ordinal"));
            }
            let completion = self.completion_source_for_owner_v1(owner)
                .ok_or_else(|| freeze("callable-result/nullable-completion"))?;
            let mut returns = Vec::new();
            for relation in self.terminal_relations_for_owner(owner) {
                let crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Value(returned) = relation
                    else { return Err(freeze("callable-result/nullable-root-role")); };
                returns.push(returned.clone());
            }
            let result = CallableBorrowedNullableResultSourceLoanV1 { owner, key: key.clone(),
                origin: CallableBorrowedResultOriginV1::MapSlot { source, returns: returns.into_boxed_slice() },
                completion: Rc::clone(completion), roots };
            result.validate_source()?;
            return Ok(Some(result));
        }
        let Some(ingress) = &self.borrowed_formal_source else { return Ok(None); };
        let ingress = ingress.as_ref().map_err(Clone::clone)?;
        let Some(definition) = ingress.source_definition_for(owner) else { return Ok(None); };
        if !definition.uses.iter().any(|row| matches!(row.kind, Use::BorrowedFormalReturn { .. })) {
            return Ok(None);
        }
        // Carrier classification borrows result source, independently of input
        // participation or LOCAL incoming demand. Entry keeps its own gate.
        let completion = self.completion_source_for_owner_v1(owner)
            .ok_or_else(|| freeze("callable-result/nullable-completion"))?;
        let mut formals = Vec::new();
        for row in &definition.uses {
            let Use::BorrowedFormalReturn { exit, source } = &row.kind else { continue; };
            self.callable_result_classes.recheck_borrowed_formal_return_source_v1(source)
                .map_err(freeze)?;
            if row.site != *source.value() || row.formal != source.formal()
                || row.site.owner() != owner
                || SourcePathV1::from_node(exit.node()).child(SourcePathSegmentV1::Value).expr() != *row.site.site()
                || !completion.explicit_sites().contains(exit)
                || definition.origins.get(&row.binding) != Some(&row.formal)
            { return Err(freeze("callable-result/nullable-use-identity")); }
            formals.push(source.as_ref().clone());
        }
        let first = &formals[0];
        if self.callable_result_classes.get(first.key())
            != Some(&OrdinaryNewResultClassV1::NullableForwarded { ordinal: first.ordinal() })
        { return Err(freeze("callable-result/nullable-law")); }
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("callable-result/nullable-catalog"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts.next().ok_or_else(|| freeze("callable-result/nullable-parameter"))?;
        if contracts.next().is_some()
            || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || contract.parameters.len() != first.key().arity() as usize
            || contract.parameters.iter().enumerate().any(|(ordinal, row)|
                row.ordinal as usize != ordinal || row.binding.owner() != owner)
            || contract.parameters.iter().filter(|row| row.ordinal == first.ordinal()
                && row.binding == first.formal() && row.kind.is_ordinary_borrowed_handle()).count() != 1
        { return Err(freeze("callable-result/nullable-ordinal")); }
        let roots = self.callable_result_classes.checked_original_source_roots_v1(first.key(), owner)
            .map_err(freeze)?.into_iter().cloned().collect();
        let source = CallableBorrowedNullableResultSourceLoanV1 {
            owner, key: first.key().clone(), origin: CallableBorrowedResultOriginV1::Formal {
                binding: first.formal(), ordinal: first.ordinal(), returns: formals.into_boxed_slice(),
            }, completion: Rc::clone(completion), roots,
        };
        source.validate_source()?;
        Ok(Some(source))
    }
}

#[cfg(test)]
impl CallableBorrowedNullableResultSourceLoanV1 {
    pub(crate) fn assert_source_drifts_refuse_for_test(&self) {
        self.validate_source().unwrap();
        for drift in ["ordinal", "roots", "formal", "duplicate-formal"] {
            let mut changed = self.clone();
            if drift == "roots" { changed.roots = changed.roots[1..].into(); }
            else {
                let CallableBorrowedResultOriginV1::Formal { ordinal, returns, .. } = &mut changed.origin else {
                    panic!("formal source corruption requires its original role");
                };
                match drift {
                    "ordinal" => *ordinal += 1,
                    "formal" => *returns = Box::new([]),
                    _ => *returns = returns.iter().cloned().chain(returns.iter().cloned()).collect(),
                }
            }
            assert!(changed.validate_source().is_err(), "nullable source {drift}");
        }
    }
}
