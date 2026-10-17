//! `.set(index, value)` element-value admission for the borrowed-view draft.
//!
//! This classifier is not ABI authority. It consumes the proven `me.<ArrayBox>`
//! receiver field and the operation owner's checked-compare dominance; the
//! physical closure still proves the routed `ArrayElementWrite` coordinate.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, OwnedExprSiteV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceExprSiteV1, SourceNodeSiteV1,
};

use super::{
    normal_integer_operand, use_dominated_by_if, BorrowedFormalUseDraftErrorV1,
    BorrowedFormalUseDraftKindV1,
};

/// `site` is the value argument of a `.set(index, value)` element write only
/// when the receiver is a proven `me.<ArrayBox>` field, the index argument
/// proves the Normal-Integer class, and an admitted checked compare of the
/// same formal owns a region dominating this use. The index operand is the
/// envelope's integer sibling; a `.set` on any other receiver surface or a
/// different argument position stays `UnresolvedArgument`.
pub(super) fn array_element_value_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    formal: BindingRefV1,
    guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    site: &SourceExprSiteV1,
    all_origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    null_guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    integer_guards: &BTreeMap<BindingRefV1, Vec<std::rc::Rc<super::guarded_actual::CheckedIntegerGuardV1>>>,
    binding: BindingRefV1,
    context: Option<&super::BorrowedFormalSourceUseContextV1<'_>>,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    if call.owner() != input.owner() || call.selector() != "set" || call.arity() != 2 {
        return Ok(None);
    }
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    else {
        return Ok(None);
    };
    let me = match function.variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => match shape.expression_shape(object) {
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return Ok(None),
        },
    };
    let Some(receiver_field) = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::receiver_array_field(
        constructors, receiver, &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()), me, field,
    ).map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)? else { return Ok(None); };
    let index = call
        .arguments()
        .iter()
        .find(|argument| argument.ordinal() == 0);
    let Some(index) = index else {
        return Ok(None);
    };
    let integer = normal_integer_operand(input, origins, constructors, receiver, index.site())?;
    let get_result = if integer {
        false
    } else {
        get_result_index(input, constructors, receiver, index.site())?
    };
    if !integer && !get_result {
        return guarded_field_index_value(input, all_origins, null_guards, integer_guards,
            formal, binding, call, index.site(), site, context,
            std::rc::Rc::new(DeclaredArrayFieldReadSourceV1 {
                call: OwnedExprSiteV1::new(input.owner(), call.site().clone()),
                site: OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
                receiver: OwnedExprSiteV1::new(input.owner(), object.clone()),
                binding: me, field: receiver_field, name: field.clone(),
            }));
    }
    let dominated = guards
        .get(&formal)
        .into_iter()
        .flatten()
        .any(|guard| use_dominated_by_if(site.node(), guard));
    if !dominated {
        return Ok(None);
    }
    Ok(Some(BorrowedFormalUseDraftKindV1::ArrayElementValue {
        call: OwnedExprSiteV1::new(input.owner(), call.site().clone()),
    }))
}

/// `site` is a `.get`-result local for the `.set` index arm only when the
/// binding's sole initializer is a `me.<ArrayBox field>.get`/1 — the same
/// `receiver_array_field` + `CORE_METHOD_CONTRACT_ROWS_V2` authorities the
/// statement coverage consumes — and the binding is never rebound. Any
/// other initializer shape, a rebound local, a receiver outside the entry
/// proof, or a non-`get` call keeps the index outside the draft. This arm
/// is not a `normal_integer_operand` widening: it names only the exact
/// proven `.get` initializer, never a `Dynamic` element type.
fn get_result_index(
    input: ResolvedFunctionLoweringInputV1<'_>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    site: &SourceExprSiteV1,
) -> Result<bool, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(site) else {
        return Ok(false);
    };
    for (_, target) in function.assignment_targets() {
        if matches!(
            target,
            ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding
        ) {
            return Ok(false);
        }
    }
    let mut initializers = function
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == binding);
    let Some(initializer) = initializers.next() else {
        return Ok(false);
    };
    if initializers.next().is_some() {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    let Some(call_site) = initializer.initializer_site() else {
        return Ok(false);
    };
    let Some(call) = function.method_call(call_site) else {
        return Ok(false);
    };
    if call.owner() != input.owner() || call.selector() != "get" || call.arity() != 1 {
        return Ok(false);
    }
    if crate::mir::core_method_result_kind::lookup_core_method_result_row_v2(
        "ArrayBox",
        call.selector(),
        call.arity(),
    )
    .is_none()
    {
        return Ok(false);
    }
    let Some(shape) = input.body_shape() else {
        return Ok(false);
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    else {
        return Ok(false);
    };
    let me = match function.variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => match shape.expression_shape(object) {
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return Ok(false),
        },
    };
    let proven = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::receiver_array_field(
        constructors,
        receiver,
        &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
        me,
        field,
    )
    .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    Ok(proven.is_some())
}

/// Original self-rooted ArrayBox declaration proof. This is a container read,
/// not an owned field residence or permission to store a borrowed element.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DeclaredArrayFieldReadSourceV1 {
    call: OwnedExprSiteV1,
    site: OwnedExprSiteV1,
    receiver: OwnedExprSiteV1,
    binding: BindingRefV1,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    name: Box<str>,
}
impl DeclaredArrayFieldReadSourceV1 {
    pub(in crate::mir) fn call(&self) -> &OwnedExprSiteV1 { &self.call }
    pub(in crate::mir) fn site(&self) -> &OwnedExprSiteV1 { &self.site }
    pub(in crate::mir) fn receiver(&self) -> &OwnedExprSiteV1 { &self.receiver }
    pub(in crate::mir) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(in crate::mir) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.field }
    pub(in crate::mir) fn name(&self) -> &str { &self.name }
}

/// Original source requirement only: no field class, physical value or entry grant.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct GuardedFormalFieldIndexV1 {
    access: OwnedExprSiteV1,
    receiver: OwnedExprSiteV1,
    binding: BindingRefV1,
    formal: BindingRefV1,
    guard: SourceNodeSiteV1,
}
impl GuardedFormalFieldIndexV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn parts_v1(&self)
        -> (&OwnedExprSiteV1, &OwnedExprSiteV1, BindingRefV1, BindingRefV1, &SourceNodeSiteV1) {
        (&self.access, &self.receiver, self.binding, self.formal, &self.guard)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn matches_source(
        &self, input: ResolvedFunctionLoweringInputV1<'_>, index: &SourceExprSiteV1,
        binding: BindingRefV1, root: BindingRefV1,
    ) -> bool {
        self.access.owner() == input.owner() && self.access.site() == index
            && self.binding == binding && self.formal == root
            && input.function().variable_ref(self.receiver.site()) == Some(ResolvedLexicalRefV1::Local(binding))
            && input.body_shape().is_some_and(|shape| matches!(shape.expression_shape(index),
                Some(BodyExpressionShapeV1::FieldAccess { object, .. }) if object == self.receiver.site()))
            && use_dominated_by_if(index.node(), &self.guard)
    }
}

fn guarded_field_index_value(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    null_guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    integer_guards: &BTreeMap<BindingRefV1, Vec<std::rc::Rc<super::guarded_actual::CheckedIntegerGuardV1>>>,
    formal: BindingRefV1, binding: BindingRefV1,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    index: &SourceExprSiteV1, value: &SourceExprSiteV1,
    context: Option<&super::BorrowedFormalSourceUseContextV1<'_>>,
    receiver_source: std::rc::Rc<DeclaredArrayFieldReadSourceV1>,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let Some(context) = context else { return Ok(None); };
    if !matches!(context.results.get(context.key),
        Some(super::result_class_claim::OrdinaryNewResultClassV1::NullableForwarded { .. }))
    { return Ok(None); }
    let Some(shape) = input.body_shape() else { return Ok(None); };
    let Some(BodyExpressionShapeV1::FieldAccess { object, .. }) = shape.expression_shape(index)
        else { return Ok(None); };
    let Some(ResolvedLexicalRefV1::Local(receiver)) = input.function().variable_ref(object)
        else { return Ok(None); };
    let Some(root) = origins.get(&receiver).copied() else { return Ok(None); };
    let Some(BorrowedFormalUseDraftKindV1::FieldReadOperand { site: access }) =
        super::field_read::field_read_operand_kind(input, root, null_guards, object)?
        else { return Ok(None); };
    if access.site() != index { return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity); }
    let Some(guard) = null_guards.get(&root).into_iter().flatten()
        .find(|guard| use_dominated_by_if(index.node(), guard)) else { return Ok(None); };
    let Some(reach) = integer_guards.get(&formal).into_iter().flatten()
        .find_map(|guard| guard.normal_path_for_operand(input, formal, binding, value))
        else { return Ok(None); };
    if call.arguments().get(1).is_none_or(|arg| arg.ordinal() != 1 || arg.site() != value)
        || call.arguments().first().is_none_or(|arg| arg.ordinal() != 0 || arg.site() != index)
    { return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity); }
    Ok(Some(BorrowedFormalUseDraftKindV1::ArrayElementFieldIndexValue {
        call: OwnedExprSiteV1::new(input.owner(), call.site().clone()),
        index: std::rc::Rc::new(GuardedFormalFieldIndexV1 { access,
            receiver: OwnedExprSiteV1::new(input.owner(), object.clone()), binding: receiver,
            formal: root, guard: guard.clone() }), reach, receiver: receiver_source,
    }))
}
