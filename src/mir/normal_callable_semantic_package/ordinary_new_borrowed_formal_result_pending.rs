//! Original return facts owned by the result stage until strict ingress seals them.
use super::super::borrowed_formal_uses::{BorrowedFormalUseDraftKindV1, BorrowedFormalUsesDraftV1};
use super::super::source::{
    CallTargetReferenceV1, PreparedSourceCallNeedV1, PreparedSourceNeedsV1, StoredReceiverSourceV1,
};
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, ResolvedAssignmentTargetV1, ResolvedInitializerRelationV1,
    ResolvedMethodCallReceiverSourceV1,
};

#[path = "ordinary_new_borrowed_formal_result_class_loans.rs"]
mod class_loans;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use class_loans::collect_observed_forward_identities_v1;

#[path = "ordinary_new_borrowed_mul_result_source.rs"]
pub(super) mod mul_result;

#[derive(Debug)]
pub(super) enum BorrowedResultSourcePhaseV1 {
    Pending {
        fields: Box<[PendingFieldResultV1]>,
        calls: Box<[PendingCallResultV1]>,
    },
    SourceSealed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct ForwardIdentityV1 {
    site: OwnedExprSiteV1,
    binding: BindingRefV1,
    source_formal: BindingRefV1,
    call: OwnedExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    ordinal: u32,
    callee_formal: BindingRefV1,
}

impl ForwardIdentityV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(in crate::mir::normal_callable_semantic_package) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(in crate::mir::normal_callable_semantic_package) fn source_formal(&self) -> BindingRefV1 {
        self.source_formal
    }
    pub(in crate::mir::normal_callable_semantic_package) fn call(&self) -> &OwnedExprSiteV1 {
        &self.call
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target(
        &self,
    ) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
    pub(in crate::mir::normal_callable_semantic_package) fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub(in crate::mir::normal_callable_semantic_package) fn callee_formal(&self) -> BindingRefV1 {
        self.callee_formal
    }
}

#[derive(Debug, Clone)]
pub(super) enum ClassLenderV1 {
    Declared(BindingRefV1),
    Forward(ForwardIdentityV1),
}

#[derive(Debug, Clone)]
pub(super) struct ConditionalClassLoanV1 {
    class: Box<str>,
    lender: ClassLenderV1,
}

#[derive(Debug)]
pub(super) struct ConditionalFieldRequirementV1 {
    class: Box<str>,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    lender: ClassLenderV1,
}

#[derive(Debug)]
pub(super) struct PendingFieldResultV1 {
    formal: BindingRefV1,
    use_site: OwnedExprSiteV1,
    read_site: OwnedExprSiteV1,
    field_name: Box<str>,
    requirement: Option<ConditionalFieldRequirementV1>,
}

#[derive(Debug)]
enum BorrowedCallResultOriginV1 {
    Direct,
    Local(ResolvedInitializerRelationV1),
}

#[derive(Debug)]
pub(super) struct PendingCallResultV1 {
    return_site: OwnedExprSiteV1,
    origin: BorrowedCallResultOriginV1,
    reference: CallTargetReferenceV1,
    receiver_source: ResolvedMethodCallReceiverSourceV1,
    stored_receiver: Option<StoredReceiverSourceV1>,
}

fn capture_field(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &OwnedCallableParameterContractDeclarationV1,
    draft: &BorrowedFormalUsesDraftV1,
    site: &SourceExprSiteV1,
) -> Option<PendingFieldResultV1> {
    let BodyExpressionShapeV1::FieldAccess { object, field, .. } =
        input.body_shape()?.expression_shape(site)?
    else {
        return None;
    };
    let ResolvedLexicalRefV1::Local(formal) = input.function().variable_ref(object)? else {
        return None;
    };
    if !contract.parameters.iter().any(|row| row.binding == formal) {
        return None;
    }
    let read_site = OwnedExprSiteV1::new(input.owner(), site.clone());
    let mut rows = draft.uses.iter().filter(|row| row.formal == formal
        && matches!(&row.kind, BorrowedFormalUseDraftKindV1::FieldReadOperand { site } if *site == read_site));
    let row = rows.next()?;
    if rows.next().is_some() {
        return None;
    }
    Some(PendingFieldResultV1 {
        formal,
        use_site: row.site.clone(),
        read_site,
        field_name: field.clone(),
        requirement: None,
    })
}

fn capture_call(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    needs: &[Result<Option<PreparedSourceCallNeedV1>, String>],
) -> Result<Option<PendingCallResultV1>, String> {
    let function = input.function();
    let (call_site, origin) = if function.method_calls().any(|(actual, _)| actual == site) {
        (site.clone(), BorrowedCallResultOriginV1::Direct)
    } else {
        let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(site) else {
            return Ok(None);
        };
        if binding.owner() != input.owner()
            || function
                .binding(binding)
                .is_none_or(|row| !matches!(row.kind(), BindingKindV1::Local { .. }))
            || function.assignment_targets().any(|(_, target)| {
                matches!(target,
                ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding)
            })
        {
            return Ok(None);
        }
        let mut rows = function
            .expression_source()
            .initializers()
            .filter(|row| row.binding() == binding);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some()
            || function.declaration_binding(row.declaration_site()) != Some(binding)
        {
            return Ok(None);
        }
        let Some(call) = row.initializer_site() else {
            return Ok(None);
        };
        (call.clone(), BorrowedCallResultOriginV1::Local(row.clone()))
    };
    let Some((_, call)) = function
        .method_calls()
        .find(|(actual, _)| *actual == &call_site)
    else {
        return Ok(None);
    };
    let owned = OwnedExprSiteV1::new(input.owner(), call_site);
    let mut matching = needs
        .iter()
        .filter_map(|row| row.as_ref().ok()?.as_ref())
        .filter(|row| row.reference().call_site == owned);
    let Some(need) = matching.next() else {
        return Ok(None);
    };
    let reference = need.reference();
    if matching.next().is_some()
        || reference.receiver_site != *call.receiver_site()
        || reference.target.arity() != call.arity()
        || reference.target.name() != call.selector()
        || reference.argument_sites.len() != call.arguments().len()
        || !reference
            .argument_sites
            .iter()
            .zip(call.arguments())
            .all(|(site, argument)| site == argument.site())
    {
        return Err(freeze("borrowed-result/call-source-identity"));
    }
    // A stored local/nested call never enters this direct-return slice.
    if need.stored().is_some() && !matches!(origin, BorrowedCallResultOriginV1::Direct) {
        return Ok(None);
    }
    Ok(Some(PendingCallResultV1 {
        return_site: OwnedExprSiteV1::new(input.owner(), site.clone()),
        origin,
        reference,
        receiver_source: call.receiver(),
        stored_receiver: need.stored().cloned(),
    }))
}

/// A `me.<ArrayBox field>.get(<integer index>)` return source: the
/// whole-Box element-integer census already sealed the field's stores,
/// and the index must satisfy the same `integer_source` leaves. The
/// proof completes at capture — a field get carries no callee dependency
/// and no incoming actual, so it adds no pending row and only credits
/// the i64 class. Anything unproven stays `false`, never an error.
#[allow(clippy::too_many_arguments)]
fn capture_field_array_get(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &OwnedCallableParameterContractDeclarationV1,
    site: &SourceExprSiteV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    proven_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
) -> Result<bool, String> {
    let function = input.function();
    // The return source is either the `get` itself or a local bound by a
    // single, never-rebound initializer — the same shape `capture_call`
    // demands of a local source.
    let call_site = if function.method_calls().any(|(actual, _)| actual == site) {
        site.clone()
    } else {
        let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(site) else {
            return Ok(false);
        };
        if binding.owner() != input.owner()
            || function
                .binding(binding)
                .is_none_or(|row| !matches!(row.kind(), BindingKindV1::Local { .. }))
            || function.assignment_targets().any(|(_, target)| {
                matches!(target,
                ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding)
            })
        {
            return Ok(false);
        }
        let mut rows = function
            .expression_source()
            .initializers()
            .filter(|row| row.binding() == binding);
        let Some(row) = rows.next() else {
            return Ok(false);
        };
        if rows.next().is_some()
            || function.declaration_binding(row.declaration_site()) != Some(binding)
        {
            return Ok(false);
        }
        let Some(call) = row.initializer_site() else {
            return Ok(false);
        };
        call.clone()
    };
    let Some((_, call)) = function
        .method_calls()
        .find(|(actual, _)| *actual == &call_site)
    else {
        return Ok(false);
    };
    if call.selector() != "get" || call.arity() != 1 {
        return Ok(false);
    }
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| freeze("borrowed-result/source-loan"))?;
    let Some(shape) = input.body_shape() else {
        return Ok(false);
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    else {
        return Ok(false);
    };
    if !super::super::super::array_i64_fields::is_self_receiver(&ledger, shape, object) {
        return Ok(false);
    }
    // The owner's own box source decides the field declaration — the
    // same authority the receiver-side census predicate consults.
    let Some(owner_box) = selected
        .keys()
        .filter_map(|selected_key| {
            let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) = selected_key
            else {
                return None;
            };
            (selected.batch_slot(selected_key) == Some(contract.batch_slot)
                && key.namespace()
                    == crate::mir::builder::SameModuleCallableNamespaceV1::InstanceBoxMethod)
                .then(|| key.owner())
        })
        .next()
    else {
        return Ok(false);
    };
    let Some(box_source) = batch
        .ordinary_box_coverage()
        .row_for(owner_box)
        .ok()
        .flatten()
    else {
        return Ok(false);
    };
    let Some(proven) = proven_fields.get(box_source.name()) else {
        return Ok(false);
    };
    let Some((field_ref, declared)) = super::super::super::terminal_home::source_declared_field(
        constructors,
        box_source,
        &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
        field,
    )
    .map_err(|error| format!("{}: {error:?}", freeze("borrowed-result/field-authority")))?
    else {
        return Ok(false);
    };
    if declared.as_deref() != Some("ArrayBox") || !proven.contains(&field_ref) {
        return Ok(false);
    }
    // The index must satisfy the census `integer_source` leaves: numeric
    // `me.<field>` reads plus `get` results on proven fields chain.
    let mut numeric_fields = BTreeSet::new();
    let mut proven_names = BTreeSet::new();
    constructors
        .with_source_object_definition(box_source, |object, definition| {
            for (ordinal, row) in definition.fields().iter().enumerate() {
                if row
                    .declared_type_name
                    .as_deref()
                    .is_some_and(crate::mir::numeric_substrate::is_numeric_integer_type_name)
                {
                    numeric_fields.insert(row.name.clone().into_boxed_str());
                }
                let Some(canonical) =
                    hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                        object, ordinal,
                    )
                else {
                    continue;
                };
                if proven.contains(&canonical) {
                    proven_names.insert(row.name.clone().into_boxed_str());
                }
            }
        })
        .map_err(|_| freeze("borrowed-result/source-loan"))?;
    let mut get_calls = BTreeMap::new();
    for (_, method_call) in ledger.method_calls() {
        if method_call.selector() != "get" || method_call.arity() != 1 {
            continue;
        }
        let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
            shape.expression_shape(method_call.receiver_site())
        else {
            continue;
        };
        if super::super::super::array_i64_fields::is_self_receiver(&ledger, shape, object)
            && proven_names.contains(field.as_ref())
        {
            get_calls.insert(method_call.site().clone(), field.clone());
        }
    }
    // `me.<name>` receiver coverage and unobserved-position actual
    // staging landed: a grounded callee's named incoming edges —
    // including calls inside prefix-failed branch subtrees and
    // condition positions — now carry staged actuals, so the
    // `formal.<field>` index leaf arms this capture end-to-end.
    let coverage = batch.ordinary_box_coverage();
    let formal_i64_field = |field: &str| {
        super::super::super::array_i64_fields::coverage_unique_i64_field(
            constructors,
            coverage,
            field,
        )
    };
    Ok(super::super::super::array_i64_fields::integer_source_at(
        &ledger,
        shape,
        &numeric_fields,
        &proven_names,
        &get_calls,
        &formal_i64_field,
        call.arguments()[0].site(),
        &mut BTreeSet::new(),
        0,
    ))
}

fn source_result_pending(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    needs: &[Result<Option<PreparedSourceCallNeedV1>, String>],
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    proven_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    owner: FunctionOwnerIdV1,
) -> Result<BorrowedI64ResultSourceV1, String> {
    let mut matches = contracts.iter().filter(|row| row.owner == owner);
    let contract = matches
        .next()
        .ok_or_else(|| freeze("borrowed-result/source-contract-missing"))?;
    if matches.next().is_some() {
        return Err(freeze("borrowed-result/source-contract-duplicate"));
    }
    batch.with_lowering_input(contract.batch_slot, |input| {
        if input.owner() != owner || !contract.parameters.iter().enumerate().all(|(index, row)|
            row.ordinal as usize == index && row.binding.owner() == owner
            && input.function().declaration_binding(&SourceBindingSiteV1::Parameter { index: row.ordinal }) == Some(row.binding)) {
            return Err(freeze("borrowed-result/source-contract-identity"));
        }
        let sites = input.body_shape().and_then(|shape| super::super::super::verified_value_return_sites(input, shape))
            .filter(|sites| !sites.is_empty()).ok_or_else(|| freeze("borrowed-result/explicit-value-return-missing"))?;
        let draft = drafts.get(&owner).ok_or_else(|| freeze("borrowed-result/source-draft-missing"))?;
        let mut class = None;
        let mut has_construction = false;
        let mut fields = Vec::new();
        let mut calls = Vec::new();
        let mut multiplications = Vec::new();
        for site in &sites {
            let function = input.function();
            let integer = matches!(function.expression_source().literal(site), Some(ResolvedLiteralSourceV1::Integer(_)));
            let exact = matches!(function.variable_ref(site), Some(ResolvedLexicalRefV1::Local(binding))
                if contract.parameters.iter().any(|row| row.binding == binding
                    && row.kind == CallableParameterContractKindV1::ExactTrivial(ExactTrivialParameterAbiV1::I64)));
            let checked_return = draft.integer_return_at(input, &OwnedExprSiteV1::new(owner, site.clone()))
                .map_err(|_| freeze("borrowed-result/checked-return-source-identity"))?.is_some();
            let site_class = if integer || exact || checked_return { BorrowedResultClassV1::I64 }
            else if let Some(product) = mul_result::capture_mul_return(input, draft, site)? {
                multiplications.push(product); BorrowedResultClassV1::I64
            } else if let Some(field) = capture_field(input, contract, draft, site) {
                fields.push(field); BorrowedResultClassV1::I64
            } else if let Some(call) = capture_call(input, site, needs)? {
                calls.push(call); BorrowedResultClassV1::I64
            } else if capture_field_array_get(
                input, contract, site, selected, batch, constructors, proven_fields,
            )? { BorrowedResultClassV1::I64 }
            else if matches!(function.expression_source().literal(site), Some(ResolvedLiteralSourceV1::Null))
                || function.expression_source().construction(site).is_some() { BorrowedResultClassV1::Nullable }
            else { return Err(freeze("borrowed-result/source-not-i64")); };
            has_construction |= function.expression_source().construction(site).is_some();
            if class.is_some_and(|previous| previous != site_class) { return Err(freeze("borrowed-result/source-class-mixed")); }
            class = Some(site_class);
        }
        let class = class.expect("nonempty explicit return sites");
        if class == BorrowedResultClassV1::Nullable && !has_construction { return Err(freeze("borrowed-result/source-not-i64")); }
        Ok(BorrowedI64ResultSourceV1 { returns: sites.into_iter().map(|site| OwnedExprSiteV1::new(owner, site)).collect(),
            class, contract_corroborated: false, dependencies: Box::new([]), multiplications: multiplications.into_boxed_slice(),
            phase: BorrowedResultSourcePhaseV1::Pending { fields: fields.into_boxed_slice(), calls: calls.into_boxed_slice() } })
    }).map_err(|_| freeze("borrowed-result/source-loan"))?
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn prepare_pending_results_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    needs: &PreparedSourceNeedsV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    callable_result_classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    proven_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
) -> BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>> {
    let Ok(needs) = needs else {
        return BTreeMap::new();
    };
    let loans = class_loans::conditional_class_loans_v1(
        batch,
        selected,
        contracts,
        drafts,
        needs,
        callable_result_classes,
    );
    drafts
        .keys()
        .filter(|owner| {
            contracts.iter().any(|row| {
                row.owner == **owner
                    && row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
            })
        })
        .map(|owner| {
            let proof = source_result_pending(
                batch,
                selected,
                contracts,
                drafts,
                needs,
                constructors,
                proven_fields,
                *owner,
            )
            .and_then(|mut proof| {
                if let BorrowedResultSourcePhaseV1::Pending { fields, .. } = &mut proof.phase {
                    for row in fields.iter_mut() {
                        if let Some(loan) = loans.get(&row.formal) {
                            if let Some(field) = super::super::super::nullable_result_integer_field(
                                constructors,
                                batch.ordinary_box_coverage(),
                                &loan.class,
                                &row.read_site,
                                &row.field_name,
                            )
                            .map_err(|error| {
                                format!("{}: {error:?}", freeze("borrowed-result/field-authority"))
                            })? {
                                row.requirement = Some(ConditionalFieldRequirementV1 {
                                    class: loan.class.clone(),
                                    field,
                                    lender: loan.lender.clone(),
                                });
                            }
                        }
                    }
                }
                Ok(proof)
            });
            (*owner, proof)
        })
        .collect()
}

pub(super) fn pending_ready_v1(proof: &BorrowedI64ResultSourceV1) -> bool {
    match &proof.phase {
        BorrowedResultSourcePhaseV1::Pending { fields, .. } => {
            fields.iter().all(|row| row.requirement.is_some())
        }
        BorrowedResultSourcePhaseV1::SourceSealed => true,
    }
}

pub(super) fn dependency_owners_v1(proof: &BorrowedI64ResultSourceV1) -> Vec<FunctionOwnerIdV1> {
    match &proof.phase {
        BorrowedResultSourcePhaseV1::Pending { calls, .. } => {
            calls.iter().map(|row| row.reference.callee_owner).collect()
        }
        BorrowedResultSourcePhaseV1::SourceSealed => proof
            .dependencies
            .iter()
            .map(|row| row.callee_owner())
            .collect(),
    }
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn stored_eligible_v1(
    need: &PreparedSourceCallNeedV1,
    results: &BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
    grounded: &BTreeSet<FunctionOwnerIdV1>,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> bool {
    let reference = need.reference();
    grounded.contains(&reference.call_site.owner()) && grounded.contains(&reference.callee_owner)
        && results.get(&reference.call_site.owner()).and_then(|row| row.as_ref().ok()).is_some_and(|proof|
            proof.class == BorrowedResultClassV1::I64 && matches!(&proof.phase,
                BorrowedResultSourcePhaseV1::Pending { calls, .. } if calls.iter().any(|row| row.reference == reference
                    && matches!(row.origin, BorrowedCallResultOriginV1::Direct))))
        && class_loans::forward_identities_v1(need, contracts, drafts).is_some_and(|rows| !rows.is_empty())
}

fn seal_one(
    owner: FunctionOwnerIdV1,
    proof: &mut BorrowedI64ResultSourceV1,
    source: &PreparedBorrowedFormalIngressV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<(), String> {
    let BorrowedResultSourcePhaseV1::Pending { fields, calls } = &proof.phase else {
        return proof.require_source_sealed_v1();
    };
    mul_result::verify_mul_returns(owner, proof, source)?;
    for row in fields.iter() {
        if !source.definitions.get(&owner).is_some_and(|draft| draft.uses.iter().any(|use_row|
            use_row.formal == row.formal && use_row.site == row.use_site
            && matches!(&use_row.kind, BorrowedFormalUseDraftKindV1::FieldReadOperand { site } if *site == row.read_site))) {
            return Err(freeze("borrowed-result/field-source-identity"));
        }
        let view = source
            .formal_object_view(row.formal)
            .ok_or_else(|| freeze("borrowed-result/source-not-i64"))?;
        let field = super::super::super::nullable_result_integer_field(
            constructors,
            batch.ordinary_box_coverage(),
            view.class(),
            &row.read_site,
            &row.field_name,
        )
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-result/field-authority")))?
        .ok_or_else(|| freeze("borrowed-result/source-not-i64"))?;
        if field.object() != view.object() {
            return Err(freeze("borrowed-result/field-object-identity"));
        }
        if let Some(requirement) = &row.requirement {
            if requirement.class.as_ref() != view.class() || requirement.field != field {
                return Err(freeze("borrowed-result/conditional-field-drift"));
            }
            match &requirement.lender {
                ClassLenderV1::Declared(formal) if *formal == row.formal && view.is_declared() => {}
                ClassLenderV1::Forward(identity)
                    if source.forwards.iter().any(|actual| {
                        actual.site == identity.site
                            && actual.binding == identity.binding
                            && actual.source_formal == identity.source_formal
                            && actual.call == identity.call
                            && actual.target == identity.target
                            && actual.ordinal == identity.ordinal
                            && actual.callee_formal == identity.callee_formal
                    }) => {}
                _ => return Err(freeze("borrowed-result/conditional-lender-drift")),
            }
        }
    }
    let mut dependencies = Vec::new();
    for row in calls.iter() {
        if row.return_site.owner() != owner || !proof.returns.contains(&row.return_site) {
            return Err(freeze("borrowed-result/call-return-identity"));
        }
        match &row.origin {
            BorrowedCallResultOriginV1::Direct if row.return_site == row.reference.call_site => {}
            BorrowedCallResultOriginV1::Local(initializer)
                if initializer.binding().owner() == owner
                    && initializer.initializer_site() == Some(row.reference.call_site.site()) => {}
            _ => return Err(freeze("borrowed-result/call-origin-identity")),
        }
        let mut incoming = source
            .incoming
            .iter()
            .filter(|actual| actual.call == row.reference.call_site);
        let actual = incoming
            .next()
            .ok_or_else(|| freeze("borrowed-result/source-not-i64"))?;
        let target = actual.source.require_instance()?;
        if incoming.next().is_some()
            || CallTargetReferenceV1::from_target(target) != row.reference
            || actual.callee != row.reference.callee_owner
            || !source.definitions.contains_key(&actual.callee)
        {
            return Err(freeze("borrowed-result/call-source-identity"));
        }
        match (&row.stored_receiver, &target.receiver) {
            (None, LexicalInstanceCallReceiverV1::Lexical(binding))
            | (None, LexicalInstanceCallReceiverV1::SelfReceiver(binding))
                if row.receiver_source
                    == ResolvedMethodCallReceiverSourceV1::Lexical(
                        ResolvedLexicalRefV1::Local(*binding),
                    ) => {}
            (
                Some(probe),
                LexicalInstanceCallReceiverV1::StoredOwnedChild {
                    parent_binding,
                    parent_site,
                    ..
                },
            ) if *parent_binding == probe.parent_binding && *parent_site == probe.parent_site => {}
            _ => return Err(freeze("borrowed-result/call-receiver-identity")),
        }
        dependencies.push(target.clone());
    }
    proof.dependencies = dependencies.into_boxed_slice();
    proof.phase = BorrowedResultSourcePhaseV1::SourceSealed;
    Ok(())
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn seal_pending_results_v1(
    mut results: BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>> {
    let source = match source {
        Ok(source) => source,
        Err(issue) => {
            for proof in results.values_mut() {
                *proof = Err(issue.clone());
            }
            return results;
        }
    };
    results.retain(|owner, _| source.definitions.contains_key(owner));
    for (owner, result) in &mut results {
        if let Ok(proof) = result {
            if let Err(issue) = seal_one(*owner, proof, source, batch, constructors) {
                *result = Err(issue);
            }
        }
    }
    composition::ground_source_results(&mut results);
    results
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_result_pending_tests.rs"]
mod tests;
