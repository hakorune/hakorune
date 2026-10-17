//! One source issuer for exact Integer reads of field-resident Arrays.
//! Return and local/Birth use relations retain this SAME immutable read loan.
use super::*;
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingKindV1, BodyExpressionShapeV1, ResolvedLexicalRefV1, ResolvedAssignmentTargetV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedFieldArrayGetSourceV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    call: OwnedExprSiteV1,
    receiver: OwnedExprSiteV1,
    index: OwnedExprSiteV1,
    index_field: Option<(BindingRefV1, SourceExprSiteV1, Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>)>,
    owner_box: Box<str>,
    field_name: Box<str>,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    provider: CanonicalSameModuleCallableKeyV1,
    provider_site: SourceExprSiteV1,
    constructor: crate::parser::ConstructorSourceIdV1,
}

impl CheckedFieldArrayGetSourceV1 {
    pub(crate) fn call(&self) -> &OwnedExprSiteV1 { &self.call }
    pub(crate) fn receiver(&self) -> &OwnedExprSiteV1 { &self.receiver }
    pub(crate) fn index(&self) -> &OwnedExprSiteV1 { &self.index }
    pub(crate) fn field_name(&self) -> &str { &self.field_name }
    pub(crate) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.field }
    pub(crate) fn index_field(&self) -> Option<&(BindingRefV1, SourceExprSiteV1, Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>)> { self.index_field.as_ref() }
    pub(crate) fn provider(&self) -> &CanonicalSameModuleCallableKeyV1 { &self.provider }
    pub(crate) fn provider_site(&self) -> &SourceExprSiteV1 { &self.provider_site }
    pub(crate) fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 { &self.caller }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_census(
        &self, owner: FunctionOwnerIdV1,
        census: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    ) -> Result<(), String> {
        if self.call.owner() != owner || self.receiver.owner() != owner || self.index.owner() != owner
            || !census.get(&self.owner_box).is_some_and(|fields| fields.contains(&self.field)) {
            return Err(freeze("array-read/source-census-drift"));
        }
        Ok(())
    }

}

/// The original single, never-rebound local initializer. No AST recovery.
pub(in crate::mir::normal_callable_semantic_package) fn local_initializer(
    input: ResolvedFunctionLoweringInputV1<'_>, binding: BindingRefV1,
) -> Option<(SourceBindingSiteV1, SourceExprSiteV1)> {
    let function = input.function();
    if binding.owner() != input.owner()
        || function.binding(binding).is_none_or(|row| !matches!(row.kind(), BindingKindV1::Local { .. }))
        || function.assignment_targets().any(|(_, target)|
            matches!(target, ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding))
        || input.forest().semantic_owners().any(|(_, product)| product.assignment_targets().any(|(_, target)|
            matches!(target, ResolvedAssignmentTargetV1::UpvarRebind(upvar) if upvar.source() == binding))) { return None; }
    let mut rows = function.expression_source().initializers().filter(|row| row.binding() == binding);
    let row = rows.next()?;
    if rows.next().is_some() || function.declaration_binding(row.declaration_site()) != Some(binding) { return None; }
    Some((row.declaration_site().clone(), row.initializer_site()?.clone()))
}

/// Sole New-local adapter. The early original New census retains the result;
/// later argument and Return owners borrow it instead of issuing another read.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn capture_local_get(
    input: ResolvedFunctionLoweringInputV1<'_>,
    caller: &CanonicalSameModuleCallableKeyV1,
    binding: BindingRefV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    census: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
) -> Result<Option<Rc<ArgumentFieldArrayReadSourceV1>>, String> {
    let Some((declaration, call)) = local_initializer(input, binding) else { return Ok(None) };
    Ok(capture_exact_get(input, caller, &call, selected, batch, constructors, census)?
        .map(|read| Rc::new(ArgumentFieldArrayReadSourceV1::new(read, binding, declaration))))
}

/// Source-family selection shared by capture and late attachment. This only
/// identifies the original own-Me Array get; it grants no element/index type.
fn exact_get_candidate(
    input: ResolvedFunctionLoweringInputV1<'_>, caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1, selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1, constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<Option<(Box<str>, hakorune_mir_defs::CanonicalFieldRefV1)>, String> {
    if caller.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod { return Ok(None) }
    let Some(call) = input.function().method_call(call_site) else { return Ok(None) };
    if call.selector() != "get" || call.arity() != 1 { return Ok(None) }
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| freeze("array-read/source-loan"))?;
    let Some(shape) = input.body_shape() else { return Ok(None) };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) = shape.expression_shape(call.receiver_site()) else { return Ok(None) };
    if !super::array_i64_fields::is_self_receiver(&ledger, shape, object) { return Ok(None) }
    let declaration = batch.declarations().find(|row| row.owner() == input.owner())
        .ok_or_else(|| freeze("array-read/caller-source-missing"))?;
    if selected.key_for_batch_slot(declaration.batch_slot())
        != Some(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller.clone())) {
        return Err(freeze("array-read/caller-source-drift"));
    }
    let Some(box_source) = batch.ordinary_box_coverage().row_for(caller.owner()).ok().flatten() else { return Ok(None) };
    let Some((canonical, declared)) = super::terminal_home::source_declared_field(constructors, box_source,
        &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()), field)
        .map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-result/field-authority")))? else { return Ok(None) };
    Ok((declared.as_deref() == Some("ArrayBox")).then(|| (field.clone(), canonical)))
}

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn capture_exact_get(
    input: ResolvedFunctionLoweringInputV1<'_>,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    proven_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
) -> Result<Option<Rc<CheckedFieldArrayGetSourceV1>>, String> {
    let Some((field, field_ref)) = exact_get_candidate(input, caller, call_site, selected, batch, constructors)? else { return Ok(None) };
    let function = input.function();
    let call = function.method_call(call_site).ok_or_else(|| freeze("array-read/original-call-missing"))?;
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| freeze("borrowed-result/source-loan"))?;
    let shape = input.body_shape().ok_or_else(|| freeze("array-read/original-shape-missing"))?;
    let owner_box = caller.owner();
    let box_source = batch.ordinary_box_coverage().row_for(owner_box).ok().flatten()
        .ok_or_else(|| freeze("array-read/original-box-missing"))?;
    let Some(proven) = proven_fields.get(box_source.name()) else { return Ok(None) };
    if !proven.contains(&field_ref) { return Ok(None) }
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
        if super::array_i64_fields::is_self_receiver(&ledger, shape, object)
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
        super::array_i64_fields::coverage_unique_i64_field(
            constructors,
            coverage,
            field,
        )
    };
    let integer_index = super::array_i64_fields::integer_source_at(
        &ledger,
        shape,
        &numeric_fields,
        &proven_names,
        &get_calls,
        &formal_i64_field,
        call.arguments()[0].site(),
        &mut BTreeSet::new(),
        0,
    );
    if !integer_index { return Ok(None); }
    let index_field = match shape.expression_shape(call.arguments()[0].site()) {
        Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) => {
            match ledger.variable_ref(object) {
                Some(ResolvedLexicalRefV1::Local(binding)) if ledger.binding(binding)
                    .is_some_and(|row| matches!(row.kind(), BindingKindV1::Parameter { .. })) => {
                    let field = super::array_i64_fields::coverage_unique_i64_field_source(
                        constructors, coverage, field).ok_or_else(|| freeze("borrowed-result/array-index-field"))?;
                    Some((binding, object.clone(), field))
                }
                _ => None,
            }
        }
        _ => None,
    };
    let birth = constructors.birth_row_for(box_source)
        .map_err(|_| freeze("borrowed-result/array-read-provider"))?
        .ok_or_else(|| freeze("borrowed-result/array-read-provider-missing"))?;
    let provider = birth.published_birth_key()
        .ok_or_else(|| freeze("borrowed-result/array-read-provider-key"))?.clone();
    let [provider_owner] = birth.forest().roots() else {
        return Err(freeze("borrowed-result/array-read-provider-owner"));
    };
    let provider_ledger = birth.forest().callable_source_ledger(*provider_owner)
        .map_err(|_| freeze("borrowed-result/array-read-provider-source"))?;
    let provider_shape = birth.body_shape()
        .ok_or_else(|| freeze("borrowed-result/array-read-provider-shape"))?;
    let provider_site = crate::mir::resolved_semantics::resolve_birth_provider(
        &provider_ledger, provider_shape, &field,
    ).map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-result/array-read-provider")))?;
    Ok(Some(Rc::new(CheckedFieldArrayGetSourceV1 {
        caller: caller.clone(),
        call: OwnedExprSiteV1::new(input.owner(), call_site.clone()),
        receiver: OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
        index: OwnedExprSiteV1::new(input.owner(), call.arguments()[0].site().clone()),
        index_field, owner_box: owner_box.into(), field_name: field.clone(), field: field_ref, provider,
        provider_site, constructor: birth.source_id().clone(),
    })))
}

fn freeze(reason: &str) -> String { format!("[freeze:contract][ordinary-new/{reason}]") }

/// Attach only the source loan held by the original New row/ordinal. Source
/// family selection precedes its demand: unrelated scalar locals stay separate.
pub(in crate::mir::normal_callable_semantic_package) fn retain_argument_read(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    original: &lexical_instance_call::BorrowedResultCohortV1,
    census: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    argument: &mut OrdinaryNewTrivialArgumentV1,
) -> Result<(), String> {
    let OrdinaryNewTrivialArgumentKindV1::Local { binding } = argument.kind() else { return Ok(()) };
    let binding = *binding;
    let declaration = batch.declarations().find(|row| row.owner() == argument.owner())
        .ok_or_else(|| freeze("array-read/argument-owner"))?;
    let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller)) =
        selected.key_for_batch_slot(declaration.batch_slot()) else { return Ok(()) };
    let candidate = batch.with_lowering_input(declaration.batch_slot(), |input| -> Result<bool, String> {
        if input.function().variable_ref(argument.site()) != Some(ResolvedLexicalRefV1::Local(binding)) {
            return Err(freeze("array-read/argument-binding"));
        }
        let Some((_, call)) = local_initializer(input, binding) else { return Ok(false) };
        Ok(exact_get_candidate(input, caller, &call, selected, batch, constructors)?.is_some())
    }).map_err(|_| freeze("array-read/argument-source-loan"))??;
    if !candidate { return Ok(()) }
    let source = original.original_new_argument_array_read_v1(argument.new_site(), argument.ordinal(), argument.site(), binding)?;
    if let Some(source) = source {
        source.read().recheck_census(argument.owner(), census)?;
        argument.retain_field_array_read(Rc::clone(source))?;
    }
    Ok(())
}

pub(super) fn retain_argument_reads(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    original: &lexical_instance_call::BorrowedResultCohortV1,
    census: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    claims: &mut [OrdinaryNewAdmissionClaimV1], results: &mut [OrdinaryNewResultClaimV1],
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for core in claims.iter_mut().map(|row| &mut row.core).chain(results.iter_mut().map(|row| &mut row.core)) {
        let Ok(arguments) = &mut core.argument_rows else { continue };
        for argument in arguments.iter_mut() {
            retain_argument_read(batch, selected, constructors, original, census, argument)
                .map_err(|detail| OrdinaryNewCoSealIssueV1::FieldArrayArgumentSource {
                    site: argument.new_site().clone(), detail: detail.into(),
                })?;
        }
    }
    Ok(())
}
