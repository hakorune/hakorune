//! Sole original actual classifier and its existing checked projections.
use super::*;
#[path = "ordinary_new_borrowed_actual_method_result.rs"]
mod method_result;
pub(super) use method_result::OriginalActualSourceContextV1;
use method_result::{OriginalMethodResultSeedV1, OriginalMethodResultResolutionV1};
pub(super) use method_result::OriginalCoreOperandRequestV1;

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
    PointValue {
        owner: FunctionOwnerIdV1,
        value: Option<crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1>,
        legacy: Box<OriginalFormalActualSeedV1>,
    },
    Binary {
        owner: FunctionOwnerIdV1,
        source: crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
        lhs: Box<OriginalFormalActualSeedV1>,
        rhs: Box<OriginalFormalActualSeedV1>,
    },
    Integer,
    Text,
    Other,
    /// New source evidence cannot activate any old agreement or transport.
    SourceOnly(Box<OriginalFormalActualSeedV1>),
    /// Preserve the old projection while retaining unproved SUM stability.
    SumUnknown(Box<OriginalFormalActualSeedV1>),
    MethodResult(Box<OriginalMethodResultSeedV1>),
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
    /// Nominal, formal and provider anchors already own their source SUM.
    /// Scalar reaching-value analysis must not replace their original proof.
    fn retains_original_binding_anchor_v1(&self) -> bool {
        match self {
            Self::ReturnedField(_) | Self::Mixed { .. } | Self::Class(_)
                | Self::Forward { .. } | Self::ProviderStatic(_) => true,
            Self::SourceOnly(source) | Self::SumUnknown(source)
                | Self::PointValue { legacy: source, .. } => source.retains_original_binding_anchor_v1(),
            Self::MethodResult(source) => matches!(source.normal_representation(),
                Some(crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactNominalBox { .. })),
            _ => false,
        }
    }
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
            Self::SumUnknown(legacy) | Self::PointValue { legacy, .. } => legacy.project_source_object_v1(),
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
            Self::SumUnknown(legacy) | Self::PointValue { legacy, .. } => legacy.project_source_integer_v1(current_i64),
            Self::Binary { .. } | Self::Conflict | Self::ProviderStatic(_) | Self::Text | Self::Other | Self::SourceOnly(_) | Self::MethodResult(_) => FormalActualSeedV1::Conflict,
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
            Self::SumUnknown(legacy) | Self::PointValue { legacy, .. } => legacy.project_selected_v1(caller, transport_owners),
            Self::Binary { .. } | Self::Conflict | Self::ProviderStatic(_) | Self::Text | Self::Other | Self::SourceOnly(_) | Self::MethodResult(_) => FormalActualSeedV1::Conflict,
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
        receiver, definitions, contracts, site, guarded, None, None,
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
    source_context: Option<&OriginalActualSourceContextV1<'_>>,
) -> OriginalFormalActualSeedV1 {
    classify_original_actual_seed_with_seen_v1(input, batch, selected, callable_result_classes,
        caller, candidates, receiver, definitions, contracts, site, guarded, inventory, source_context, &BTreeSet::new())
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
    source_context: Option<&OriginalActualSourceContextV1<'_>>,
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
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(_)) => return OriginalFormalActualSeedV1::Text,
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::TypedInteger { .. }) => return OriginalFormalActualSeedV1::Conflict,
        Some(_) => return OriginalFormalActualSeedV1::Other,
        None => {}
    }
    if function
        .expression_source()
        .negative_integer_immediate(site)
        .is_some()
    {
        return OriginalFormalActualSeedV1::Integer;
    }
    if source_context.is_some_and(OriginalActualSourceContextV1::is_point_premise) {
        // The existing child inventory proves a transparent empty-prelude
        // BlockExpr. Keep the outer Core receiver site; solve only its value
        // through the same original tail, without rewriting source or effects.
        if let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) {
            use crate::mir::resolved_value_profile::{expression_source::{expression_source_v1, ResolvedExpressionSourceV1},
                expression_children::{expression_children_v1, ExpressionChildrenV1, ExpressionChildReachV1}};
            if matches!(expression_source_v1(&ledger, input.body_shape(), site), Some(ResolvedExpressionSourceV1::Shape(
                crate::mir::resolved_semantics::BodyExpressionShapeV1::BlockExpr { .. }))) {
                if let Ok(ExpressionChildrenV1::Known(children)) = expression_children_v1(&ledger, input.body_shape(), site) {
                    if let [child] = children.as_slice() {
                        if child.role == crate::mir::resolved_semantics::SourcePathSegmentV1::BlockExprTail
                            && child.reach == ExpressionChildReachV1::Always {
                            return classify_original_actual_seed_with_seen_v1(input, batch, selected, callable_result_classes,
                                caller, candidates, receiver, definitions, contracts, &child.site, None, inventory, source_context, inherited);
                        }
                    }
                }
            }
        }
        if let Some(source) = function.expression_source().binary(site) {
            let classify = |site: &SourceExprSiteV1| classify_original_actual_seed_with_seen_v1(input, batch, selected, callable_result_classes,
                caller, candidates, receiver, definitions, contracts, site, None, inventory, source_context, inherited);
            return OriginalFormalActualSeedV1::Binary { owner: input.owner(), source: source.clone(),
                lhs: Box::new(classify(source.lhs())), rhs: Box::new(classify(source.rhs())) };
        }
    }
    if let Some(method) = function.method_call_source_v1(site) {
        let (proof, core_operand_request) = match source_context.map(|context| context.resolve(input, method, inventory)) {
            Some(OriginalMethodResultResolutionV1::Existing { proof, operands }) => (proof, operands),
            Some(OriginalMethodResultResolutionV1::ConditionalCore(request)) => (None, Some(request)),
            _ => (None, None),
        };
        let point_context = core_operand_request.and_then(|_| source_context.map(OriginalActualSourceContextV1::for_point_premise));
        let classify = |site: &SourceExprSiteV1| classify_original_actual_seed_with_seen_v1(input, batch, selected, callable_result_classes,
            caller, candidates, receiver, definitions, contracts, site, None, inventory, point_context.as_ref().or(source_context), inherited);
        let requirements = if let Some(request) = core_operand_request { request.i64_ordinals() }
            else { proof.as_ref().map(|proof| proof.callee_i64_ordinals()).unwrap_or(&[]) };
        let required_i64 = requirements.iter().map(|ordinal| {
            match method.arguments().get(*ordinal as usize).filter(|argument| argument.ordinal() == *ordinal) {
                Some(argument) => classify(argument.site()),
                None => OriginalFormalActualSeedV1::Conflict,
            }
        }).collect();
        let mut required_text = Vec::new();
        if let Some(request) = core_operand_request {
            required_text.push(classify(method.receiver_site()));
            for ordinal in request.text_ordinals() {
                required_text.push(match method.arguments().get(*ordinal as usize).filter(|arg| arg.ordinal() == *ordinal) {
                    Some(argument) => classify(argument.site()),
                    None => OriginalFormalActualSeedV1::Conflict,
                });
            }
        }
        return OriginalFormalActualSeedV1::MethodResult(Box::new(OriginalMethodResultSeedV1 {
            method: std::sync::Arc::clone(method), proof, core_operand_request, required_i64,
            required_text: required_text.into_boxed_slice(),
        }));
    }
    let Some(mut binding) = arg_site_binding(input, site) else {
        return OriginalFormalActualSeedV1::Conflict;
    };
    if let Some(context) = source_context.filter(|context| context.is_point_premise()) {
        let legacy_context = context.for_legacy_actual();
        let legacy = classify_original_actual_seed_with_seen_v1(input, batch, selected, callable_result_classes,
            caller, candidates, receiver, definitions, contracts, site, None, inventory, Some(&legacy_context), inherited);
        if legacy.retains_original_binding_anchor_v1() { return legacy; }
        let value = uncaptured_source_binding_v1(input, binding).then(||
            function.expression_source().value_flow()
                .filter(|flow| flow.owner() == input.owner())
                .and_then(|flow| flow.read(site, binding))).flatten();
        return OriginalFormalActualSeedV1::PointValue { owner: input.owner(), value, legacy: Box::new(legacy) };
    }
    if value_domain::declared_integer_seed_v1(input, contracts, binding) {
        return if contracts.iter().filter(|row| row.owner == input.owner()).flat_map(|row| &row.parameters)
            .any(|row| row.binding == binding && matches!(row.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
        { OriginalFormalActualSeedV1::Integer } else { OriginalFormalActualSeedV1::SourceInteger };
    }
    let mut visited = inherited.clone();
    let mut sum_stable = true;
    loop {
        if binding.owner() != caller || !visited.insert(binding) {
            return OriginalFormalActualSeedV1::Conflict;
        }
        sum_stable &= stable_source_binding_v1(input, binding);
        if let Some(candidate) = candidates
            .iter()
            .find(|candidate| candidate.destination == binding)
        {
            return sum_checked(if candidate.construction.is_ok() {
                OriginalFormalActualSeedV1::Class(candidate.class.clone())
            } else { OriginalFormalActualSeedV1::Conflict }, sum_stable);
        }
        if let Some((root, row)) = receiver {
            if root == binding {
                return sum_checked(OriginalFormalActualSeedV1::Class(row.name().into()), sum_stable);
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
            return sum_checked(OriginalFormalActualSeedV1::Class(class), sum_stable);
        }
        if let Some(inventory) = inventory {
            if let Some(seed) = source_seeds::classify_mixed_actual_seed_v1(input, batch, selected,
                callable_result_classes, caller, candidates, receiver, definitions, contracts,
                binding, inventory, &visited, source_context) {
                return sum_checked(seed, sum_stable);
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
            return sum_checked(OriginalFormalActualSeedV1::Forward {
                origin: *formal, declared_constraint,
            }, sum_stable);
        }
        match alias_source_binding(input, binding) {
            Some(next) => binding = next,
            None => {
                if !sum_stable { return OriginalFormalActualSeedV1::Conflict; }
                let mut owners = contracts.iter().filter(|row| row.owner == input.owner());
                if let Some(contract) = owners.next().filter(|_| owners.next().is_none()) {
                    if let Some(formal) = contract.parameters.iter().enumerate().find_map(|(ordinal, formal)|
                        (formal.binding == binding && formal.ordinal as usize == ordinal
                            && function.declaration_binding(&crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter { index: formal.ordinal }) == Some(binding)).then_some(formal)) {
                        return OriginalFormalActualSeedV1::SourceOnly(Box::new(OriginalFormalActualSeedV1::Forward {
                            origin: binding, declared_constraint: match &formal.kind {
                                CallableParameterContractKindV1::DeclaredObject(class) => Some(class.clone()), _ => None,
                            },
                        }));
                    }
                }
                let mut initializers = function.expression_source().initializers().filter(|row| row.binding() == binding);
                if let Some(initializer) = initializers.next().filter(|_| initializers.next().is_none()) {
                    if function.declaration_binding(initializer.declaration_site()) == Some(binding) {
                        if let Some(site) = initializer.initializer_site() {
                            let source = classify_original_actual_seed_with_seen_v1(input, batch, selected, callable_result_classes,
                                caller, candidates, receiver, definitions, contracts, site, None, inventory, source_context, &visited);
                            return OriginalFormalActualSeedV1::SourceOnly(Box::new(source));
                        }
                    }
                }
                return OriginalFormalActualSeedV1::Conflict;
            }
        }
    }
}

fn sum_checked(seed: OriginalFormalActualSeedV1, stable: bool) -> OriginalFormalActualSeedV1 {
    if stable { seed } else { OriginalFormalActualSeedV1::SumUnknown(Box::new(seed)) }
}

pub(super) fn stable_source_binding_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>, binding: BindingRefV1,
) -> bool {
    use crate::mir::resolved_semantics::ResolvedAssignmentTargetV1 as Target;
    uncaptured_source_binding_v1(input, binding)
        && !input.function().assignment_targets().any(|(_, target)| matches!(target, Target::BindingRebind(actual) if *actual == binding))
}

pub(super) fn uncaptured_source_binding_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>, binding: BindingRefV1,
) -> bool {
    input.forest().callable_source_ledger(input.owner())
        .is_ok_and(|ledger| ledger.uncaptured_binding(binding))
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

