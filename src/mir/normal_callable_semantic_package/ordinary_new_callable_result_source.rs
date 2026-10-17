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

/// Same immutable formal Return requirements, all source roots and Completion.
/// No new result law, selected input, executable row or ownership is issued.
#[derive(Debug, Clone)]
pub(crate) struct CallableBorrowedNullableResultSourceLoanV1 {
    owner: FunctionOwnerIdV1,
    key: CanonicalSameModuleCallableKeyV1,
    formal: BindingRefV1,
    ordinal: u32,
    completion: Rc<VerifiedFunctionCompletionV1>,
    roots: Box<[Rc<ResultOriginWitnessV1>]>,
    formals: Box<[BorrowedFormalReturnSourceLoanV1]>,
}
impl CallableBorrowedNullableResultSourceLoanV1 {
    fn validate_source(&self) -> Result<(), String> {
        if self.completion.owner() != self.owner || !self.completion.returns_value()
            || self.completion.implicit_body_end().is_some()
            || !matches!(self.completion.function_exit_contract().declared_result(), DeclaredFunctionResultContractV1::Unannotated)
            || self.key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || self.ordinal >= self.key.arity() || self.formal.owner() != self.owner
            || self.formals.is_empty() || self.roots.is_empty()
        { return Err(freeze("callable-result/nullable-source-contract")); }
        let expected: BTreeSet<_> = self.completion.explicit_sites().iter().map(|site|
            SourcePathV1::from_node(site.node()).child(SourcePathSegmentV1::Value).expr()).collect();
        let actual: BTreeSet<_> = self.roots.iter().map(|root| root.site().site().clone()).collect();
        if expected.is_empty() || actual != expected
            || self.roots.iter().any(|root| root.site().owner() != self.owner)
        { return Err(freeze("callable-result/nullable-return-coverage")); }
        let mut formal_sites = BTreeSet::new();
        for source in &self.formals {
            if source.key() != &self.key || source.formal() != self.formal
                || source.ordinal() != self.ordinal || source.value().owner() != self.owner
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
                    ResultWitnessStepV1::Formal { binding, ordinal })
                    if *binding == self.formal && *ordinal == self.ordinal
                        && (root.origin() == &ResultValueOriginV1::Null
                            || root.origin() == &ResultValueOriginV1::ForwardFormal { ordinal: self.ordinal }) => {
                    actual_formals.insert(root.site().site().clone());
                },
                _ => return Err(freeze("callable-result/nullable-root-role")),
            }
        }
        if actual_formals != formal_sites { return Err(freeze("callable-result/nullable-formal-coverage")); }
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
            owner, key: first.key().clone(), formal: first.formal(), ordinal: first.ordinal(),
            completion: Rc::clone(completion), roots, formals: formals.into_boxed_slice(),
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
            match drift {
                "ordinal" => changed.ordinal += 1,
                "roots" => { changed.roots = changed.roots[1..].into(); },
                "formal" => changed.formals = Box::new([]),
                _ => changed.formals = changed.formals.iter().cloned()
                    .chain(changed.formals.iter().cloned()).collect(),
            }
            assert!(changed.validate_source().is_err(), "nullable source {drift}");
        }
    }
}
