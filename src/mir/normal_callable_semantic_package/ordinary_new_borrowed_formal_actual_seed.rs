//! Sole original actual classifier and its existing checked projections.
use super::*;

/// Original source classification retains a forward's declaration constraint
/// without deciding transport membership. The same row can be projected later.
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub(super) enum OriginalFormalActualSeedV1 {
    ReturnedField(Box<super::super::super::result_class_claim::ReturnedFieldSourceLoanV1>),
    Mixed {
        source: Box<super::super::super::result_class_claim::MixedLocalCallSourceLoanV1>,
        anchor: Box<OriginalFormalActualSeedV1>,
    },
    Integer,
    ProviderStatic(std::rc::Rc<crate::mir::normal_callable_semantic_package::instance_constructor_semantic::provider_source::ProviderStaticArgumentSourceV1>),
    /// Exact usize declaration: Integer lane, without signed result permission.
    SourceInteger,
    Class(Box<str>),
    Null,
    Forward {
        origin: BindingRefV1,
        declared_constraint: Option<Box<str>>,
    },
    Conflict,
}
impl OriginalFormalActualSeedV1 {
    fn returned_field_domain_v1(source: &super::super::super::result_class_claim::ReturnedFieldSourceLoanV1) -> FormalActualSeedV1 {
        match (source.class(), source.nullable()) {
            (Some(class), true) => FormalActualSeedV1::All(Box::new([
                FormalActualSeedV1::Class(class.into()), FormalActualSeedV1::Null,
            ])),
            (Some(class), false) => FormalActualSeedV1::Class(class.into()),
            (None, _) => FormalActualSeedV1::Null,
        }
    }
    /// Source class agreement retains every Forward edge. DeclaredObject is a
    /// separate conjunct in the same domain fold, never an actual class seed.
    pub(super) fn project_source_object_v1(&self) -> FormalActualSeedV1 {
        match self {
            Self::Forward { origin, .. } => FormalActualSeedV1::Forward(*origin),
            Self::Mixed { source, anchor } => FormalActualSeedV1::All(Box::new([
                FormalActualSeedV1::Class(source.fresh_class().into()), anchor.project_source_object_v1(),
            ])),
            _ => self.project_source_integer_v1(false),
        }
    }
    pub(super) fn project_source_integer_v1(&self, current_i64: bool) -> FormalActualSeedV1 {
        match self {
            Self::ReturnedField(source) => Self::returned_field_domain_v1(source),
            Self::Mixed { source, anchor } => FormalActualSeedV1::All(Box::new([
                FormalActualSeedV1::Class(source.fresh_class().into()),
                anchor.project_source_integer_v1(current_i64),
            ])),
            Self::Forward { origin, declared_constraint: Some(_) } => FormalActualSeedV1::Forward(*origin),
            Self::Forward { origin, declared_constraint: None } => FormalActualSeedV1::ForwardIntegerOnly(*origin),
            Self::Integer => FormalActualSeedV1::Integer,
            Self::SourceInteger => if current_i64 { FormalActualSeedV1::Conflict } else { FormalActualSeedV1::Integer },
            Self::Class(class) => FormalActualSeedV1::Class(class.clone()),
            Self::Null => FormalActualSeedV1::Null,
            Self::Conflict | Self::ProviderStatic(_) => FormalActualSeedV1::Conflict,
        }
    }

    pub(super) fn project_selected_v1(
        &self,
        caller: FunctionOwnerIdV1,
        transport_owners: Option<&BTreeSet<FunctionOwnerIdV1>>,
    ) -> FormalActualSeedV1 {
        match self {
            Self::ReturnedField(source) => Self::returned_field_domain_v1(source),
            Self::Mixed { source, anchor } => FormalActualSeedV1::All(Box::new([
                FormalActualSeedV1::Class(source.fresh_class().into()),
                anchor.project_selected_v1(caller, transport_owners),
            ])),
            Self::Integer | Self::SourceInteger => FormalActualSeedV1::Integer,
            Self::Class(class) => FormalActualSeedV1::Class(class.clone()),
            Self::Null => FormalActualSeedV1::Null,
            Self::Conflict | Self::ProviderStatic(_) => FormalActualSeedV1::Conflict,
            Self::Forward { origin, declared_constraint } => {
                let outside = transport_owners.is_some_and(|owners| !owners.contains(&caller));
                match (outside, declared_constraint) {
                    (false, Some(class)) => FormalActualSeedV1::Class(class.clone()),
                    (true, None) => FormalActualSeedV1::ForwardIntegerOnly(*origin),
                    _ => FormalActualSeedV1::Forward(*origin),
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn classify_actual_seed_in_scope(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    callable_result_classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    caller: FunctionOwnerIdV1,
    candidates: &[super::super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &SourceExprSiteV1,
    transport_owners: Option<&BTreeSet<FunctionOwnerIdV1>>,
    guarded: Option<(&BorrowedGuardedActualV1, &OwnedExprSiteV1, u32)>,
) -> FormalActualSeedV1 {
    classify_original_actual_seed_in_scope(
        input, batch, selected, callable_result_classes, caller, candidates,
        receiver, definitions, contracts, site, guarded, None,
    ).project_selected_v1(caller, transport_owners)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn classify_original_actual_seed_in_scope(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    callable_result_classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    caller: FunctionOwnerIdV1,
    candidates: &[super::super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &SourceExprSiteV1,
    guarded: Option<(&BorrowedGuardedActualV1, &OwnedExprSiteV1, u32)>,
    inventory: Option<&BorrowedIncomingInventoryV1>,
) -> OriginalFormalActualSeedV1 {
    classify_original_actual_seed_with_seen_v1(input, batch, selected, callable_result_classes,
        caller, candidates, receiver, definitions, contracts, site, guarded, inventory, &BTreeSet::new())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn classify_original_actual_seed_with_seen_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    callable_result_classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    caller: FunctionOwnerIdV1,
    candidates: &[super::super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &SourceExprSiteV1,
    guarded: Option<(&BorrowedGuardedActualV1, &OwnedExprSiteV1, u32)>,
    inventory: Option<&BorrowedIncomingInventoryV1>,
    inherited: &BTreeSet<BindingRefV1>,
) -> OriginalFormalActualSeedV1 {
    if let Some((fact, call, ordinal)) = guarded {
        return if fact.corroborates(input, call, ordinal, site) {
            OriginalFormalActualSeedV1::Integer
        } else {
            OriginalFormalActualSeedV1::Conflict
        };
    }
    let function = input.function();
    match function.expression_source().literal(site) {
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null) => {
            return OriginalFormalActualSeedV1::Null;
        }
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(_)) => {
            return OriginalFormalActualSeedV1::Integer;
        }
        Some(_) => return OriginalFormalActualSeedV1::Conflict,
        None => {}
    }
    if function
        .expression_source()
        .negative_integer_immediate(site)
        .is_some()
    {
        return OriginalFormalActualSeedV1::Integer;
    }
    let Some(mut binding) = arg_site_binding(input, site) else {
        return OriginalFormalActualSeedV1::Conflict;
    };
    if value_domain::declared_integer_seed_v1(input, contracts, binding) {
        return if contracts.iter().filter(|row| row.owner == input.owner()).flat_map(|row| &row.parameters)
            .any(|row| row.binding == binding && matches!(row.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
        { OriginalFormalActualSeedV1::Integer } else { OriginalFormalActualSeedV1::SourceInteger };
    }
    let mut visited = inherited.clone();
    loop {
        if binding.owner() != caller || !visited.insert(binding) {
            return OriginalFormalActualSeedV1::Conflict;
        }
        if let Some(candidate) = candidates
            .iter()
            .find(|candidate| candidate.destination == binding)
        {
            return if candidate.construction.is_ok() {
                OriginalFormalActualSeedV1::Class(candidate.class.clone())
            } else {
                OriginalFormalActualSeedV1::Conflict
            };
        }
        if let Some((root, row)) = receiver {
            if root == binding {
                return OriginalFormalActualSeedV1::Class(row.name().into());
            }
        }
        if let Some(class) =
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue::nullable_received_result_class(
                selected,
                batch,
                callable_result_classes,
                candidates,
                input, receiver,
                binding,
            )
        {
            return OriginalFormalActualSeedV1::Class(class);
        }
        if let Some(inventory) = inventory {
            if let Some(seed) = source_seeds::classify_mixed_actual_seed_v1(input, batch, selected,
                callable_result_classes, caller, candidates, receiver, definitions, contracts,
                binding, inventory, &visited) {
                return seed;
            }
        }
        if let Some(formal) = definitions
            .get(&caller)
            .and_then(|draft| draft.origins.get(&binding))
        {
            let declared_constraint = contracts
                .iter()
                .find(|row| row.owner == caller)
                .and_then(|row| row.parameters.iter().find(|row| row.binding == *formal))
                .and_then(|row| match &row.kind {
                    CallableParameterContractKindV1::DeclaredObject(class) => Some(class.clone()),
                    _ => None,
                });
            return OriginalFormalActualSeedV1::Forward {
                origin: *formal,
                declared_constraint,
            };
        }
        match alias_source_binding(input, binding) {
            Some(next) => binding = next,
            None => return OriginalFormalActualSeedV1::Conflict,
        }
    }
}

/// One alias hop for `local q = <binding>`: the initializer declaring
/// `binding` names its own source binding. Foreign, double-declared or
/// non-variable initializers answer `None` and the walk conflicts.
pub(super) fn alias_source_binding(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    binding: BindingRefV1,
) -> Option<BindingRefV1> {
    let function = input.function();
    let mut matching = function
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == binding);
    let initializer = matching.next()?;
    if matching.next().is_some() {
        return None;
    }
    arg_site_binding(input, initializer.initializer_site()?)
}

