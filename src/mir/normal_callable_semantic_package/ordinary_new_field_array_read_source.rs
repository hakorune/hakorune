//! Shared original residence/provider authority for field Array accesses.
//! Element/Count/Dynamic reads and Text append keep distinct operation permissions.
use super::*;
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingKindV1, BodyExpressionShapeV1, ResolvedLexicalRefV1, ResolvedAssignmentTargetV1};
#[path = "ordinary_new_field_array_read_bounds.rs"]
mod bounds;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldArrayResidenceSourceV1 {
    owner_box: Box<str>, field_name: Box<str>,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    provider: CanonicalSameModuleCallableKeyV1,
    provider_site: SourceExprSiteV1,
    constructor: crate::parser::ConstructorSourceIdV1,
}

#[derive(Debug, Clone)]
struct ArrayProviderCallSourceV1 {
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    manifest: crate::mir::core_method_result_kind::CoreMethodManifestRowRefV2,
    writer: field_write_claim::OriginalFieldWriteSourceLoanV1,
    append_target: Option<Rc<crate::mir::resolved_semantics::VerifiedCoreMethodInstanceTargetV1>>,
}
impl PartialEq for ArrayProviderCallSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.method, &other.method) && self.writer == other.writer
            && match (&self.append_target, &other.append_target) {
                (None, None) => true, (Some(a), Some(b)) => Rc::ptr_eq(a,b), _ => false,
            }
            && self.manifest.brand() == other.manifest.brand()
            && self.manifest.arity() == other.manifest.arity()
            && std::ptr::eq(self.manifest.row(), other.manifest.row())
    }
}
impl Eq for ArrayProviderCallSourceV1 {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldArrayReadSourceOperationV1 {
    Element {
        index: OwnedExprSiteV1,
        index_field: Option<(BindingRefV1, SourceExprSiteV1, Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>)>,
    },
    Count(ArrayProviderCallSourceV1),
    DynamicGet(ArrayProviderCallSourceV1),
    TextAppend(ArrayProviderCallSourceV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedFieldArrayAccessSourceV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    call: OwnedExprSiteV1,
    receiver: OwnedExprSiteV1,
    residence: FieldArrayResidenceSourceV1,
    receiver_root: BindingRefV1,
    receiver_alias: Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedArrayFieldSourceV1>,
    operation: FieldArrayReadSourceOperationV1,
    bounds: Option<bounds::CheckedArrayGetBoundsSourceV1>,
}

impl CheckedFieldArrayAccessSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn matches_original_call_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> bool {
        self.matches_original_call_base_v1(input)
            && self.bounds.as_ref().is_none_or(|proof| proof.matches(input, self))
    }
    /// Original call identity without re-entering its optional bounds proof.
    fn matches_original_call_base_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        let Some(call) = input.function().method_call_source_v1(self.call.site()) else { return false; };
        if input.owner() != self.call.owner() || call.receiver_site() != self.receiver.site() { return false; }
        if self.receiver_root.owner() != input.owner() { return false; }
        if self.receiver_alias.is_none() && CanonicalArrayFieldReadSourceV1::original_receiver_v1(input, &self.receiver, self.field_name())
            .ok().flatten().is_none_or(|(_, binding)| binding != self.receiver_root) { return false; }
        if let Some(alias) = &self.receiver_alias {
            let Some((field, initializer)) = alias.fixed_source_v1() else { return false; };
            if input.function().variable_ref(call.receiver_site()) != Some(ResolvedLexicalRefV1::Local(alias.binding()))
                || local_initializer(input, alias.binding()) != Some((initializer.declaration_site().clone(), field.site().site().clone()))
                || input.forest().callable_source_ledger(input.owner()).ok()
                    .is_none_or(|ledger| !ledger.uncaptured_binding(alias.binding()))
                || !field.matches_original_receiver_v1(input, field.site(), field.name()).unwrap_or(false)
                || field.binding() != self.receiver_root || field.field() != self.field() || field.name() != self.field_name() { return false; }
        }
        match &self.operation {
            FieldArrayReadSourceOperationV1::Count(source) | FieldArrayReadSourceOperationV1::DynamicGet(source) | FieldArrayReadSourceOperationV1::TextAppend(source) => std::sync::Arc::ptr_eq(call, &source.method),
            FieldArrayReadSourceOperationV1::Element { index, .. } => call.selector() == "get" && call.arity() == 1
                && call.arguments().first().is_some_and(|argument| argument.ordinal() == 0 && argument.site() == index.site()),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn read_manifest_v1(&self)
        -> Option<crate::mir::core_method_result_kind::CoreMethodManifestRowRefV2> {
        match &self.operation {
            FieldArrayReadSourceOperationV1::Count(source) | FieldArrayReadSourceOperationV1::DynamicGet(source) => Some(source.manifest),
            _ => None,
        }
    }
    pub(crate) fn is_dynamic_get(&self) -> bool { matches!(self.operation, FieldArrayReadSourceOperationV1::DynamicGet(_)) }
    pub(crate) fn get_is_in_bounds_v1(&self) -> bool { self.is_dynamic_get() && self.bounds.is_some() }
    pub(in crate::mir::normal_callable_semantic_package) fn borrows_direct_receiver_v1(&self, field: &CanonicalArrayFieldReadSourceV1) -> bool {
        self.receiver_alias.is_none() && self.receiver == *field.site() && self.receiver_root == field.binding()
            && self.field() == field.field() && self.field_name() == field.name()
            && field.fixed_origin_v1().is_some_and(|(caller, _)| caller == self.caller())
    }
    pub(crate) fn receiver_alias_v1(&self)
        -> Option<&crate::mir::resolved_semantics::home_new_prefix::BorrowedArrayFieldSourceV1> { self.receiver_alias.as_ref() }
    pub(crate) fn receiver_root_binding_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> Option<BindingRefV1> {
        self.matches_original_call_v1(input).then_some(self.receiver_root)
    }
    pub(crate) fn matches_expression_receiver_v1(&self,
        plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        if !(self.is_count() || self.is_dynamic_get()) || plan.owner() != self.call.owner() { return false; }
        let mut nodes = plan.nodes().iter().filter(|node| node.site() == self.receiver.site());
        let Some(receiver) = nodes.next() else { return false; };
        if nodes.next().is_some() { return false; }
        if let Some(alias) = &self.receiver_alias {
            return matches!(receiver.source(), Node::Reference(ResolvedLexicalRefV1::Local(binding)) if *binding == alias.binding())
                && alias.fixed_source_v1().is_some_and(|(source, initializer)| source.field() == self.field()
                    && source.name() == self.field_name() && initializer.initializer_site() == Some(source.site().site()));
        }
        matches!(receiver.source(), Node::Shape(BodyExpressionShapeV1::FieldAccess { object, field, .. })
            if field.as_ref() == self.field_name() && plan.nodes().iter().any(|node| node.site() == object
                && matches!(node.source(), Node::Reference(ResolvedLexicalRefV1::Local(binding))
                    | Node::Shape(BodyExpressionShapeV1::Me { receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding), .. })
                    | Node::ReceiverQualification { receiver: crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)), .. }
                    if *binding == self.receiver_root)))
    }
    pub(crate) fn call(&self) -> &OwnedExprSiteV1 { &self.call }
    pub(crate) fn receiver(&self) -> &OwnedExprSiteV1 { &self.receiver }
    pub(crate) fn index(&self) -> Option<&OwnedExprSiteV1> {
        match &self.operation { FieldArrayReadSourceOperationV1::Element { index, .. } => Some(index), _ => None }
    }
    pub(crate) fn is_element(&self) -> bool { matches!(self.operation, FieldArrayReadSourceOperationV1::Element { .. }) }
    pub(crate) fn append_method(&self) -> Option<&std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>> {
        match &self.operation { FieldArrayReadSourceOperationV1::TextAppend(source) => Some(&source.method), _ => None }
    }
    pub(crate) fn is_count(&self) -> bool { matches!(self.operation, FieldArrayReadSourceOperationV1::Count(_)) }
    pub(crate) fn field_name(&self) -> &str { &self.residence.field_name }
    pub(crate) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.residence.field }
    pub(crate) fn index_field(&self) -> Option<&(BindingRefV1, SourceExprSiteV1, Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>)> {
        match &self.operation { FieldArrayReadSourceOperationV1::Element { index_field, .. } => index_field.as_ref(), _ => None }
    }
    pub(crate) fn provider(&self) -> &CanonicalSameModuleCallableKeyV1 { &self.residence.provider }
    pub(crate) fn provider_site(&self) -> &SourceExprSiteV1 { &self.residence.provider_site }
    pub(crate) fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 { &self.caller }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_census(
        &self, owner: FunctionOwnerIdV1,
        census: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    ) -> Result<(), String> {
        if self.call.owner() != owner || self.receiver.owner() != owner {
            return Err(freeze("array-read/source-census-drift"));
        }
        let valid = match &self.operation {
            FieldArrayReadSourceOperationV1::Element { index, .. } => index.owner() == owner
                && census.get(&self.residence.owner_box).is_some_and(|fields| fields.contains(&self.field())),
            FieldArrayReadSourceOperationV1::Count(source) => {
                use crate::mir::core_method_result_kind::{CoreMethodSemanticLawV2 as Law, CoreMethodEffectV1 as Effect, CoreMethodResultKindV1 as ResultKind};
                let row = source.manifest.row();
                source.append_target.is_none() && source.method.owner() == owner && source.method.site() == self.call.site()
                    && source.method.receiver_site() == self.receiver.site() && source.method.arity() == 0
                    && crate::boxes::array::ArrayMethodId::from_name_and_arity(source.method.selector(), source.method.arity() as usize)
                        == Some(crate::boxes::array::ArrayMethodId::Length)
                    && row.op == crate::mir::core_method_op::CoreMethodOp::ArrayLen
                    && row.receiver_box == "ArrayBox" && row.effect == Effect::PureRead && row.result_kind == ResultKind::I64Value
                    && source.manifest.arity() == 0 && source.manifest.semantic_law_for_arity() == Some(Law::ArrayCount)
                    && source.writer.corroborates_sole_array_provider_v1(&self.residence.owner_box,
                        self.field_name(), &self.residence.constructor, self.provider_site())
            }
            FieldArrayReadSourceOperationV1::DynamicGet(source) => {
                use crate::mir::core_method_result_kind::{CoreMethodEffectV1 as Effect, CoreMethodResultKindV1 as ResultKind};
                source.append_target.is_none() && source.method.owner() == owner && source.method.site() == self.call.site()
                    && source.method.receiver_site() == self.receiver.site() && source.method.arity() == 1
                    && source.method.arguments().len() == 1 && source.method.arguments()[0].ordinal() == 0
                    && crate::boxes::array::ArrayMethodId::from_name_and_arity(source.method.selector(), 1)
                        == Some(crate::boxes::array::ArrayMethodId::Get)
                    && source.manifest.row().op == crate::mir::core_method_op::CoreMethodOp::ArrayGet
                    && source.manifest.row().receiver_box == "ArrayBox" && source.manifest.row().effect == Effect::PureRead
                    && source.manifest.row().result_kind == ResultKind::Dynamic && source.manifest.arity() == 1
                    && source.writer.corroborates_sole_array_provider_v1(&self.residence.owner_box,
                        self.field_name(), &self.residence.constructor, self.provider_site())
            }
            FieldArrayReadSourceOperationV1::TextAppend(source) => {
                use crate::mir::resolved_semantics::{CoreMethodHomeSchemaV1 as Schema, CoreMethodHomeParameterRelationV1 as Parameter, CoreMethodHomeResultRelationV1 as ResultKind};
                source.method.owner() == owner && source.method.site() == self.call.site()
                    && source.method.receiver_site() == self.receiver.site() && source.method.arity() == 1
                    && source.append_target.as_ref().is_some_and(|target|
                        target.schema() == Schema::ArrayTextAppend && target.row().row().op == crate::mir::core_method_op::CoreMethodOp::ArrayPush
                            && target.parameters() == [Parameter::TextRetainedByReceiver] && target.result() == ResultKind::NoValue)
                    && source.writer.corroborates_sole_array_provider_v1(&self.residence.owner_box,
                        self.field_name(), &self.residence.constructor, self.provider_site())
            }
        };
        if !valid { return Err(freeze("array-read/source-census-drift")); }
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
    let Some(call) = input.function().method_call(call_site) else { return Ok(None) };
    if call.selector() != "get" || call.arity() != 1 { return Ok(None) }
    exact_array_field_candidate(input, caller, call_site, selected, batch, constructors)
}

fn exact_array_field_candidate(
    input: ResolvedFunctionLoweringInputV1<'_>, caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1, selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1, constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<Option<(Box<str>, hakorune_mir_defs::CanonicalFieldRefV1)>, String> {
    if caller.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod { return Ok(None) }
    let Some(call) = input.function().method_call(call_site) else { return Ok(None) };
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
    Ok(constructors.is_array_field(canonical).map_err(|_| freeze("array-read/field-source"))?
        .then(|| (field.clone(), canonical)))
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
) -> Result<Option<Rc<CheckedFieldArrayAccessSourceV1>>, String> {
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
    let receiver_root = CanonicalArrayFieldReadSourceV1::original_receiver_v1(input,
        &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()), &field)?
        .ok_or_else(|| freeze("array-read/receiver-source-binding"))?.1;
    let residence = capture_residence(constructors, box_source, owner_box, field, field_ref)?;
    Ok(Some(Rc::new(CheckedFieldArrayAccessSourceV1 {
        caller: caller.clone(),
        call: OwnedExprSiteV1::new(input.owner(), call_site.clone()),
        receiver: OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
        residence, receiver_root, receiver_alias: None, bounds: None,
        operation: FieldArrayReadSourceOperationV1::Element {
            index: OwnedExprSiteV1::new(input.owner(), call.arguments()[0].site().clone()), index_field,
        },
    })))
}

fn capture_residence(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    box_source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
    owner_box: &str, field: Box<str>, field_ref: hakorune_mir_defs::CanonicalFieldRefV1,
) -> Result<FieldArrayResidenceSourceV1, String> {
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
    Ok(FieldArrayResidenceSourceV1 { owner_box: owner_box.into(), field_name: field,
        field: field_ref, provider, provider_site, constructor: birth.source_id().clone() })
}

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn capture_exact_read(
    input: ResolvedFunctionLoweringInputV1<'_>, caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1, selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1, constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    writers: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Result<Option<Rc<CheckedFieldArrayAccessSourceV1>>, String> {
    let Some(method) = input.function().method_call_source_v1(call_site) else { return Ok(None) };
    use crate::boxes::array::ArrayMethodId;
    let (op, arity) = match ArrayMethodId::from_name_and_arity(method.selector(), method.arity() as usize) {
        Some(ArrayMethodId::Length) => (crate::mir::core_method_op::CoreMethodOp::ArrayLen, 0),
        Some(ArrayMethodId::Get) if method.arguments().len() == 1 && method.arguments()[0].ordinal() == 0 =>
            (crate::mir::core_method_op::CoreMethodOp::ArrayGet, 1),
        _ => return Ok(None),
    };
    let (field, canonical, receiver_alias) = if let Some((field, canonical)) =
        exact_array_field_candidate(input, caller, call_site, selected, batch, constructors)? {
        (field, canonical, None)
    } else {
        let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(method.receiver_site()) else { return Ok(None); };
        let Some((_, site)) = local_initializer(input, binding) else { return Ok(None); };
        let Some(alias) = capture_fixed_alias_v1(input, caller, &site, selected, batch, constructors, writers)? else { return Ok(None); };
        let Some((source, _)) = alias.fixed_source_v1() else { return Ok(None); };
        (source.name().into(), source.field(), Some(alias))
    };
    let box_source = batch.ordinary_box_coverage().row_for(caller.owner()).ok().flatten()
        .ok_or_else(|| freeze("array-read/original-box-missing"))?;
    let receiver_root = match &receiver_alias {
        Some(alias) => alias.fixed_source_v1().ok_or_else(|| freeze("array-read/receiver-source-binding"))?.0.binding(),
        None => CanonicalArrayFieldReadSourceV1::original_receiver_v1(input,
            &OwnedExprSiteV1::new(input.owner(), method.receiver_site().clone()), &field)?
            .ok_or_else(|| freeze("array-read/receiver-source-binding"))?.1,
    };
    let residence = capture_residence(constructors, box_source, caller.owner(), field, canonical)?;
    let writer = writers.array_provider_source_v1(caller.owner(), &residence.field_name,
        &residence.constructor, &residence.provider_site).ok_or_else(|| freeze("array-read/provider-writers"))?;
    let manifest = crate::mir::core_method_result_kind::issue_core_method_manifest_row_ref_v2(op, arity)
        .ok_or_else(|| freeze("array-read/count-manifest"))?;
    let provider = ArrayProviderCallSourceV1 {
        method: std::sync::Arc::clone(method), manifest, writer, append_target: None,
    };
    let operation = if op == crate::mir::core_method_op::CoreMethodOp::ArrayLen {
        FieldArrayReadSourceOperationV1::Count(provider)
    } else { FieldArrayReadSourceOperationV1::DynamicGet(provider) };
    let mut source = CheckedFieldArrayAccessSourceV1 {
        caller: caller.clone(), call: OwnedExprSiteV1::new(input.owner(), call_site.clone()),
        receiver: OwnedExprSiteV1::new(input.owner(), method.receiver_site().clone()), residence, receiver_root, receiver_alias,
        operation, bounds: None,
    };
    if source.is_dynamic_get() {
        source.bounds = bounds::capture(input, &source, |site|
            capture_exact_count(input, caller, site, selected, batch, constructors, writers))?;
    }
    let source = Rc::new(source);
    source.recheck_census(input.owner(), &BTreeMap::new())?;
    Ok(Some(source))
}

/// Retain actual provider operations on this exact initializer alias. Other
/// receivers and unsupported methods do not become operation permissions.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn capture_alias_reads_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, caller: &CanonicalSameModuleCallableKeyV1,
    alias: &crate::mir::resolved_semantics::home_new_prefix::BorrowedArrayFieldSourceV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1, constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    writers: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Result<Vec<Rc<CheckedFieldArrayAccessSourceV1>>, String> {
    let ledger = input.forest().callable_source_ledger(input.owner())
        .map_err(|_| freeze("array-read/original-call-missing"))?;
    let mut reads = Vec::new();
    for (_, method) in ledger.method_calls() {
        if input.function().variable_ref(method.receiver_site()) != Some(ResolvedLexicalRefV1::Local(alias.binding())) {
            continue;
        }
        let Some(read) = capture_exact_read(input, caller, method.site(), selected, batch, constructors, writers)? else { continue; };
        if read.receiver_alias_v1() != Some(alias) || !read.matches_original_call_v1(input) {
            return Err(freeze("array-read/receiver-source-binding"));
        }
        reads.push(read);
    }
    Ok(reads)
}

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn capture_exact_count(
    input: ResolvedFunctionLoweringInputV1<'_>, caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1, selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1, constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    writers: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Result<Option<Rc<CheckedFieldArrayAccessSourceV1>>, String> {
    let Some(method) = input.function().method_call_source_v1(call_site) else { return Ok(None); };
    if crate::boxes::array::ArrayMethodId::from_name_and_arity(method.selector(), method.arity() as usize)
        != Some(crate::boxes::array::ArrayMethodId::Length) { return Ok(None); }
    capture_exact_read(input, caller, call_site, selected, batch, constructors, writers)
}

pub(in crate::mir::normal_callable_semantic_package) fn capture_text_append(
    input: ResolvedFunctionLoweringInputV1<'_>, caller: &CanonicalSameModuleCallableKeyV1,
    box_source: &crate::parser::ParserOrdinaryBoxSourceRowV1, call_site: &SourceExprSiteV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    writers: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Result<Option<Rc<CheckedFieldArrayAccessSourceV1>>, String> {
    let Some(method) = input.function().method_call_source_v1(call_site) else { return Ok(None) };
    if caller.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod || caller.owner() != box_source.name()
        || crate::boxes::array::ArrayMethodId::from_name_and_arity(method.selector(), method.arity() as usize)
            != Some(crate::boxes::array::ArrayMethodId::Push) { return Ok(None); }
    let Some(shape) = input.body_shape() else { return Ok(None) };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) = shape.expression_shape(method.receiver_site()) else { return Ok(None) };
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| freeze("array-read/source-loan"))?;
    if !super::array_i64_fields::is_self_receiver(&ledger, shape, object) { return Ok(None) }
    let Some((canonical, _)) = super::terminal_home::source_declared_field(constructors, box_source,
        &OwnedExprSiteV1::new(input.owner(), method.receiver_site().clone()), field)
        .map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-result/field-authority")))? else { return Ok(None) };
    if !constructors.is_array_field(canonical).map_err(|_| freeze("array-read/field-source"))? { return Ok(None); }
    let receiver_root = CanonicalArrayFieldReadSourceV1::original_receiver_v1(input,
        &OwnedExprSiteV1::new(input.owner(), method.receiver_site().clone()), field)?
        .ok_or_else(|| freeze("array-read/receiver-source-binding"))?.1;
    let residence = capture_residence(constructors, box_source, caller.owner(), field.clone(), canonical)?;
    let writer = writers.array_provider_source_v1(caller.owner(), &residence.field_name,
        &residence.constructor, &residence.provider_site).ok_or_else(|| freeze("array-read/provider-writers"))?;
    let manifest = crate::mir::core_method_result_kind::issue_core_method_manifest_row_ref_v2(crate::mir::core_method_op::CoreMethodOp::ArrayPush,1)
        .ok_or_else(|| freeze("array-read/count-manifest"))?;
    let target = crate::mir::resolved_semantics::CoreMethodInstanceTargetIssuerV1::array_text_append(manifest.brand())
        .and_then(|mut issuer| issuer.issue(manifest)).map_err(|_| freeze("array-read/source-census-drift"))?;
    let source = Rc::new(CheckedFieldArrayAccessSourceV1 { caller: caller.clone(),
        call: OwnedExprSiteV1::new(input.owner(), call_site.clone()),
        receiver: OwnedExprSiteV1::new(input.owner(), method.receiver_site().clone()), residence, receiver_root, receiver_alias: None, bounds: None,
        operation: FieldArrayReadSourceOperationV1::TextAppend(ArrayProviderCallSourceV1 {
            method: std::sync::Arc::clone(method), manifest, writer, append_target: Some(Rc::new(target)),
        }),
    });
    source.recheck_census(input.owner(), &BTreeMap::new())?;
    Ok(Some(source))
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

/// Exact source for a self-rooted canonical Array field receiver. Operation
/// permission and owned residence remain with their independent source owners.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CanonicalArrayFieldReadSourceV1 {
    site: OwnedExprSiteV1,
    receiver: OwnedExprSiteV1,
    binding: BindingRefV1,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    name: Box<str>,
    declared_type_name: Option<Box<str>>,
    origin: ArrayFieldReadOriginV1,
}
#[derive(Debug)]
enum ArrayFieldReadOriginV1 {
    BorrowedEntry { call: OwnedExprSiteV1 },
    TextAppend { frame: Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldArrayTextAppendV1> },
    FixedReceiver {
        caller: CanonicalSameModuleCallableKeyV1,
        source: Rc<field_write_claim::ObjectFieldClassSourceV1>,
    },
}
impl PartialEq for ArrayFieldReadOriginV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BorrowedEntry { call: left }, Self::BorrowedEntry { call: right }) => left == right,
            (Self::TextAppend { frame: left }, Self::TextAppend { frame: right }) => Rc::ptr_eq(left, right),
            (Self::FixedReceiver { caller: left, source }, Self::FixedReceiver { caller: right, source: other }) =>
                left == right && Rc::ptr_eq(source, other),
            _ => false,
        }
    }
}
impl Eq for ArrayFieldReadOriginV1 {}
impl CanonicalArrayFieldReadSourceV1 {
    /// The operation lends its original receiver, not permission to read an
    /// element or execute an append. Keep the same frame through affine reads.
    pub(in crate::mir::normal_callable_semantic_package) fn text_append_v1(
        input: ResolvedFunctionLoweringInputV1<'_>,
        frame: Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldArrayTextAppendV1>,
    ) -> Result<Self, String> {
        let source = frame.source();
        let Some((receiver, binding)) = Self::original_receiver_v1(input, source.receiver(), source.field_name())? else {
            return Err(freeze("array-read/receiver-source-binding"));
        };
        if !source.matches_original_call_v1(input) || receiver.site() != frame.receiver_read()
            || binding != frame.receiver() || source.append_method().is_none() {
            return Err(freeze("array-read/receiver-source-binding"));
        }
        Ok(Self { site: source.receiver().clone(), receiver, binding, field: source.field(),
            name: source.field_name().into(), declared_type_name: None,
            origin: ArrayFieldReadOriginV1::TextAppend { frame } })
    }
    pub(in crate::mir) fn text_append_origin_v1(&self)
        -> Option<&Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldArrayTextAppendV1>> {
        match &self.origin { ArrayFieldReadOriginV1::TextAppend { frame } => Some(frame), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn fixed_receiver_v1(
        input: ResolvedFunctionLoweringInputV1<'_>, caller: CanonicalSameModuleCallableKeyV1,
        site: &OwnedExprSiteV1, name: &str, source: Rc<field_write_claim::ObjectFieldClassSourceV1>,
    ) -> Result<Option<Self>, String> {
        let Some((receiver, binding)) = Self::original_receiver_v1(input, site, name)? else { return Ok(None); };
        if caller.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod {
            return Err(freeze("array-read/receiver-source-caller"));
        }
        let Some((field, declared)) = source.array_declaration_v1() else { return Ok(None); };
        Ok(Some(Self { site: site.clone(), receiver, binding, field, name: name.into(),
            declared_type_name: declared.map(Into::into), origin: ArrayFieldReadOriginV1::FixedReceiver { caller, source } }))
    }
    fn original_receiver_v1(input: ResolvedFunctionLoweringInputV1<'_>, site: &OwnedExprSiteV1, name: &str,
    ) -> Result<Option<(OwnedExprSiteV1, BindingRefV1)>, String> {
        use crate::mir::resolved_semantics::BodyMeReceiverV1;
        if input.owner() != site.owner() { return Err(freeze("array-read/receiver-source-owner")); }
        let Some(shape) = input.body_shape() else { return Ok(None); };
        let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) = shape.expression_shape(site.site()) else { return Ok(None); };
        if field.as_ref() != name { return Err(freeze("array-read/receiver-source-field")); }
        let Some(BodyExpressionShapeV1::Me { receiver: BodyMeReceiverV1::Lexical(binding), .. }) = shape.expression_shape(object) else { return Ok(None); };
        let expected = crate::mir::resolved_semantics::SourcePathV1::from_node(site.site().node())
            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver).expr();
        if object != &expected || binding.owner() != input.owner()
            || input.function().binding(*binding).is_none_or(|row| row.kind() != BindingKindV1::Receiver) {
            return Err(freeze("array-read/receiver-source-binding"));
        }
        Ok(Some((OwnedExprSiteV1::new(input.owner(), object.clone()), *binding)))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_original_receiver_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>, site: &OwnedExprSiteV1, name: &str,
    ) -> Result<bool, String> {
        Ok(&self.site == site && self.name() == name
            && Self::original_receiver_v1(input, site, name)? == Some((self.receiver.clone(), self.binding)))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn fixed_origin_v1(&self)
        -> Option<(&CanonicalSameModuleCallableKeyV1, &Rc<field_write_claim::ObjectFieldClassSourceV1>)> {
        match &self.origin { ArrayFieldReadOriginV1::FixedReceiver { caller, source } => Some((caller, source)), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_entry_v1(
        call: OwnedExprSiteV1, site: OwnedExprSiteV1, receiver: OwnedExprSiteV1,
        binding: BindingRefV1, field: hakorune_mir_defs::CanonicalFieldRefV1,
        name: Box<str>, declared_type_name: Option<Box<str>>,
    ) -> Self {
        Self { site, receiver, binding, field, name, declared_type_name,
            origin: ArrayFieldReadOriginV1::BorrowedEntry { call } }
    }
    pub(in crate::mir) fn entry_call_v1(&self) -> Option<&OwnedExprSiteV1> {
        match &self.origin { ArrayFieldReadOriginV1::BorrowedEntry { call } => Some(call), _ => None }
    }
    pub(in crate::mir) fn site(&self) -> &OwnedExprSiteV1 { &self.site }
    pub(in crate::mir) fn receiver(&self) -> &OwnedExprSiteV1 { &self.receiver }
    pub(in crate::mir) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(in crate::mir) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.field }
    pub(in crate::mir) fn name(&self) -> &str { &self.name }
    pub(in crate::mir) fn declared_type_name(&self) -> Option<&str> { self.declared_type_name.as_deref() }
}

/// The sole fixed-provider local alias adapter. Count and the Home field walk
/// retain this same source shape; neither can invent an operation or Home.
pub(in crate::mir::normal_callable_semantic_package) fn capture_fixed_alias_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, caller: &CanonicalSameModuleCallableKeyV1,
    site: &SourceExprSiteV1, selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1, constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    writers: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedArrayFieldSourceV1>, String> {
    if caller.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod { return Ok(None); }
    let mut declarations = batch.declarations().filter(|row| row.owner() == input.owner());
    let Some(declaration) = declarations.next() else { return Ok(None); };
    if declarations.next().is_some() || selected.key_for_batch_slot(declaration.batch_slot())
        != Some(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller.clone())) {
        return Err(freeze("array-read/receiver-source-caller"));
    }
    let Some(BodyExpressionShapeV1::FieldAccess { field: name, .. }) = input.body_shape()
        .and_then(|shape| shape.expression_shape(site)) else { return Ok(None); };
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| freeze("array-read/receiver-source-owner"))?;
    let mut initializers = input.function().expression_source().initializers().filter(|row| row.initializer_site() == Some(site));
    let Some(initializer) = initializers.next() else { return Ok(None); };
    if initializers.next().is_some() || !ledger.uncaptured_binding(initializer.binding())
        || local_initializer(input, initializer.binding()) != Some((initializer.declaration_site().clone(), site.clone())) { return Ok(None); }
    let Some(source) = writers.array_source(caller.owner(), name) else { return Ok(None); };
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some(field) = CanonicalArrayFieldReadSourceV1::fixed_receiver_v1(input, caller.clone(), &owned, name, source)? else { return Ok(None); };
    if !constructors.is_array_field(field.field()).map_err(|_| freeze("array-read/field-source"))? { return Ok(None); }
    Ok(crate::mir::resolved_semantics::home_new_prefix::BorrowedArrayFieldSourceV1::fixed_alias_v1(Rc::new(field), initializer.clone()))
}
