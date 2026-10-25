//! Qualified source needs of one original expression. Classes are scheduling
//! evidence only; full selected input/Home/ABI and final MIR remain separate.
use super::super::borrowed_formal_source::{
    OriginalBorrowedFormalSourceV1, OriginalIntegerBasisV1,
};
use super::super::borrowed_formal_uses::BorrowedFormalUsesDraftV1;
use super::super::source::CallTargetReferenceV1;
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::{
    DeclaredFieldValueSourceV1, ObjectFieldClassSourceV1, ObjectFieldClassSourcesV1,
    OrdinaryNewFieldWriteClaimsV1,
};
use crate::mir::resolved_semantics::{BodyMeReceiverV1, ResolvedLexicalRefV1};
use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1;
use crate::mir::resolved_value_profile::expression_plan::{
    ExpressionNodeSourceV1, ExpressionSourcePlanV1,
};
use crate::mir::resolved_value_profile::expression_projection::{
    project_expression_class_v1, ExpressionLeafClassV1,
};

#[path = "ordinary_new_borrowed_result_field_path.rs"]
mod field_path;
pub(crate) use field_path::ResultPathFieldSourceLoanV1;

pub(super) type FieldKeyV1 = (Box<str>, Box<str>);
pub(super) type FieldSourcesV1 = BTreeMap<FieldKeyV1, Rc<DeclaredFieldValueSourceV1>>;

#[derive(Debug)]
enum LeafV1 {
    Structural,
    Core(crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1),
    /// This exact field projection belongs to the retained Core source node.
    CoreReceiver(usize),
    Integer(OriginalIntegerBasisV1),
    Receiver(BindingRefV1),
    MapLocal {
        binding: BindingRefV1,
        key: FieldKeyV1,
        source: Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::container_field_uses::MapFieldPayloadSourceV1>,
        get: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    },
    ObjectField {
        key: FieldKeyV1,
        source: Rc<ObjectFieldClassSourceV1>,
    },
    /// An untyped child field is an identity only when the exact original
    /// field node is the receiver of its source-issued stored-child call.
    StoredObjectField {
        key: FieldKeyV1,
        reference: CallTargetReferenceV1,
        receiver_site: SourceExprSiteV1,
        source: super::super::StoredReceiverSourceV1,
    },
    Field {
        key: FieldKeyV1,
        source: Rc<DeclaredFieldValueSourceV1>,
    },
    Call {
        reference: CallTargetReferenceV1,
        receiver: crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1,
        stored_receiver: Option<super::super::StoredReceiverSourceV1>,
        receiver_site: SourceExprSiteV1,
    },
    Unresolved,
}
#[derive(Debug)]
pub(super) struct ResultExpressionSourceV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    leaves: Box<[LeafV1]>,
}
impl ResultExpressionSourceV1 {
    pub(super) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(super) fn prepare(
        input: ResolvedFunctionLoweringInputV1<'_>,
        plan: Rc<ExpressionSourcePlanV1>,
        class: &str,
        receiver: Option<BindingRefV1>,
        fields: &FieldSourcesV1,
        objects: &ObjectFieldClassSourcesV1,
        original: Option<&OriginalBorrowedFormalSourceV1>,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        needs: &[Result<Option<super::super::source::PreparedSourceCallNeedV1>, String>],
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        claims: &OrdinaryNewFieldWriteClaimsV1,
    ) -> Self {
        // Resolver references take precedence over shape classification. Demand
        // both rows at the SAME object site instead of inventing a Me category.
        let exact_receiver = |site: &SourceExprSiteV1| {
            receiver.is_some_and(|binding| original::exact_me_receiver_v1(input, site, binding))
        };
        let mut leaves: Vec<_> = plan.nodes().iter().map(|_| LeafV1::Unresolved).collect();
        for index in plan.ordered_postorder_v1().unwrap_or_default() {
            let node = &plan.nodes()[index];
            leaves[index] = match node.source() {
                ExpressionNodeSourceV1::Binary(_) | ExpressionNodeSourceV1::Literal(_) => {
                    LeafV1::Structural
                }
                ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding))
                    if exact_receiver(node.site()) =>
                {
                    LeafV1::Receiver(*binding)
                }
                ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding)) => original
                    .and_then(|original| {
                        original.integer_origin_at_v1(
                            input,
                            node.site(),
                            constructors,
                            definitions,
                            contracts,
                        )
                    })
                    .filter(|basis| basis.binding() == *binding)
                    .map(LeafV1::Integer)
                    .or_else(|| map_local_source_v1(input, *binding, class, receiver,
                        batch, selected, constructors, claims))
                    .unwrap_or(LeafV1::Unresolved),
                ExpressionNodeSourceV1::ReceiverQualification {
                    receiver:
                        crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                            ResolvedLexicalRefV1::Local(binding),
                        ),
                    ..
                } if Some(*binding) == receiver => LeafV1::Receiver(*binding),
                ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::Me {
                    receiver: BodyMeReceiverV1::Lexical(binding),
                    ..
                }) if Some(*binding) == receiver => LeafV1::Receiver(*binding),
                ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess {
                    object,
                    field,
                    ..
                }) => {
                    let parent_class = if exact_receiver(object) {
                        Some(class)
                    } else {
                        plan.nodes()
                            .iter()
                            .position(|node| node.site() == object)
                            .and_then(|index| match &leaves[index] {
                                LeafV1::ObjectField { source, .. } => Some(source.class()),
                                LeafV1::MapLocal { source, .. } => Some(source.class.as_ref()),
                                _ => None,
                            })
                    };
                    let map_root = plan.nodes().iter().position(|node| node.site() == object)
                        .and_then(|index| match &leaves[index] {
                            LeafV1::MapLocal { binding, source, .. } => Some((*binding, source.object)),
                            _ => None,
                        });
                    let path = plan.field_path_v1(node.site());
                    if path
                        .as_ref()
                        .is_none_or(|(root, fields)| Some(*root) != receiver
                            && !map_root.is_some_and(|(binding, _)| binding == *root && fields.len() == 1))
                    {
                        LeafV1::Unresolved
                    } else if let Some(parent_class) = parent_class {
                        let key = (parent_class.into(), field.clone());
                        if let Some(source) = objects.get(&key) {
                            if source.field().is_some() {
                                LeafV1::ObjectField {
                                    key,
                                    source: Rc::clone(source),
                                }
                            } else if let Some((reference, receiver_site, stored)) =
                                stored_receiver_field_v1(input, &plan, node.site(), needs)
                            {
                                if stored.parent_class.as_ref() == key.0.as_ref()
                                    && stored.field_name.as_ref() == key.1.as_ref()
                                    && Some(stored.parent_binding) == receiver
                                    && stored.parent_site == *object
                                {
                                    LeafV1::StoredObjectField {
                                        key,
                                        reference,
                                        receiver_site,
                                        source: stored,
                                    }
                                } else {
                                    LeafV1::Unresolved
                                }
                            } else {
                                LeafV1::Unresolved
                            }
                        } else {
                            if let Some((reference, receiver_site, stored)) =
                                stored_receiver_field_v1(input, &plan, node.site(), needs)
                            {
                                if stored.parent_class.as_ref() == key.0.as_ref()
                                    && stored.field_name.as_ref() == key.1.as_ref()
                                    && Some(stored.parent_binding) == receiver
                                    && stored.parent_site == *object
                                {
                                    LeafV1::StoredObjectField {
                                        key,
                                        reference,
                                        receiver_site,
                                        source: stored,
                                    }
                                } else {
                                    LeafV1::Unresolved
                                }
                            } else {
                                fields
                                    .get(&key)
                                    .filter(|source| map_root.is_none_or(|(_, object)| source.field().object() == object))
                                    .map(|source| LeafV1::Field {
                                        key,
                                        source: Rc::clone(source),
                                    })
                                    .unwrap_or(LeafV1::Unresolved)
                            }
                        }
                    } else {
                        LeafV1::Unresolved
                    }
                }

                ExpressionNodeSourceV1::MethodCall => {
                    if let Some(core) = original.and_then(|source|
                        source.ordinary_core_source_v1(&OwnedExprSiteV1::new(input.owner(), node.site().clone()))) {
                        leaves[index] = crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1::lend(
                            input, &plan, index, core).map(LeafV1::Core).unwrap_or(LeafV1::Unresolved);
                        continue;
                    }
                    let source_call = pending::exact_call_source_v1(input, node.site(), needs);
                    match source_call {
                        Ok(Some((reference, receiver, need))) => input
                            .function()
                            .method_call(node.site())
                            .map(|call| LeafV1::Call {
                                reference,
                                receiver,
                                stored_receiver: need.stored().cloned(),
                                receiver_site: call.receiver_site().clone(),
                            })
                            .unwrap_or(LeafV1::Unresolved),
                        _ => LeafV1::Unresolved,
                    }
                }
                _ => LeafV1::Unresolved,
            };
        }
        // Only the source-issued field receiver is structural. This does not
        // erase a key/argument subtree or reclassify a Map as a scalar field.
        for index in 0..leaves.len() {
            let field = match &leaves[index] {
                LeafV1::Core(core) => core.field_map().map(|field| field.field_site().clone())
                    .or_else(|| core.field_array_count_v1().map(|count| count.receiver().site().clone())),
                _ => None,
            };
            if let Some(field) = field {
                if let Some(receiver) = plan.nodes().iter().position(|node| node.site() == &field) {
                    leaves[receiver] = LeafV1::CoreReceiver(index);
                }
            }
        }
        Self {
            plan,
            leaves: leaves.into_boxed_slice(),
        }
    }
    pub(super) fn class(
        &self,
        fields: &BTreeSet<FieldKeyV1>,
        results: &BTreeMap<FunctionOwnerIdV1, BorrowedResultClassV1>,
    ) -> Option<BorrowedResultClassV1> {
        if !self.core_sources_match() { return None; }
        // Source Integer is broader than the current signed result producer.
        // Keep usize declaration / typed-literal obligations out of I64 loans.
        if self
            .leaves
            .iter()
            .any(|leaf| matches!(leaf, LeafV1::Integer(basis) if !basis.i64_source()))
            || self.plan.nodes().iter().any(|node| {
                matches!(
                    node.source(),
                    ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::TypedInteger { .. })
                )
            })
        {
            return None;
        }
        let class = project_expression_class_v1(&self.plan, |index| match &self.leaves[index] {
            LeafV1::Core(core) if core.source().target().checked_text_keys_law_v1() => ExpressionLeafClassV1::Identity,
            LeafV1::Core(core) => core.result_class().map(ExpressionLeafClassV1::Value)
                .unwrap_or(ExpressionLeafClassV1::Unresolved),
            LeafV1::CoreReceiver(_) => ExpressionLeafClassV1::Identity,
            LeafV1::Integer(_) => ExpressionLeafClassV1::Value(ExpressionValueClassV1::I64),
            LeafV1::Receiver(_) | LeafV1::MapLocal { .. } | LeafV1::ObjectField { .. } | LeafV1::StoredObjectField { .. } => {
                ExpressionLeafClassV1::Identity
            }
            LeafV1::Field { key, .. } if fields.contains(key) => {
                ExpressionLeafClassV1::Value(ExpressionValueClassV1::I64)
            }
            LeafV1::Call { reference, .. } => match results.get(&reference.callee_owner) {
                Some(BorrowedResultClassV1::I64) => {
                    ExpressionLeafClassV1::Value(ExpressionValueClassV1::I64)
                }
                Some(BorrowedResultClassV1::Bool) => {
                    ExpressionLeafClassV1::Value(ExpressionValueClassV1::Bool)
                }
                _ => ExpressionLeafClassV1::Unresolved,
            },
            _ => ExpressionLeafClassV1::Unresolved,
        })?;
        match class {
            ExpressionValueClassV1::I64 => Some(BorrowedResultClassV1::I64),
            ExpressionValueClassV1::Bool => Some(BorrowedResultClassV1::Bool),
            _ => None,
        }
    }
    pub(super) fn field_source(
        &self,
        site: &SourceExprSiteV1,
        receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1,
        class: &str,
        name: &str,
    ) -> Option<(usize, Rc<DeclaredFieldValueSourceV1>)> {
        let index = self
            .plan
            .field_read_node_v1(site, receiver_site, receiver, name)?;
        let receiver_index = self
            .plan
            .nodes()
            .iter()
            .position(|node| node.site() == receiver_site)?;
        if !matches!(self.leaves.get(receiver_index), Some(LeafV1::Receiver(binding)) if *binding == receiver)
        {
            return None;
        }
        match self.leaves.get(index)? {
            LeafV1::Field { key, source } if key.0.as_ref() == class && key.1.as_ref() == name => {
                Some((index, Rc::clone(source)))
            }
            _ => None,
        }
    }
    pub(super) fn uses_fields(&self) -> bool {
        self.leaves
            .iter()
            .any(|leaf| matches!(leaf, LeafV1::Field { .. }))
    }
    pub(super) fn fields(&self) -> impl Iterator<Item = &FieldKeyV1> {
        self.leaves.iter().filter_map(|leaf| match leaf {
            LeafV1::Field { key, .. } => Some(key),
            _ => None,
        })
    }
    pub(super) fn calls(&self) -> impl Iterator<Item = &CallTargetReferenceV1> {
        self.leaves.iter().filter_map(|leaf| match leaf {
            LeafV1::Call { reference, .. } => Some(reference),
            _ => None,
        })
    }
    pub(super) fn calls_ready(&self, source: &PreparedBorrowedFormalIngressV1) -> bool {
        self.core_sources_match() && self.leaves.iter().all(|leaf| {
            if let LeafV1::Core(core) = leaf {
                return source.ordinary_core_source_v1(&OwnedExprSiteV1::new(self.plan.owner(), core.source().source().site().clone()))
                    .is_some_and(|original| core.borrows_original_v1(original));
            }
            if let LeafV1::Integer(basis) = leaf { return basis.ready(source); }
            let LeafV1::Call { reference, receiver, stored_receiver, .. } = leaf else { return true; };
            let mut rows = source.source_incoming.exact_rows().filter(|row| row.call == reference.call_site);
            let row = rows.next();
            rows.next().is_none() && row.is_some_and(|row| row.callee == reference.callee_owner
                && row.source.require_instance().is_ok_and(|target| {
                    if CallTargetReferenceV1::from_target(target) != *reference { return false; }
                    match (&target.receiver, receiver, stored_receiver) {
                        (LexicalInstanceCallReceiverV1::Lexical(binding) | LexicalInstanceCallReceiverV1::SelfReceiver(binding),
                            crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(original)), None) => binding == original,
                        (LexicalInstanceCallReceiverV1::StoredOwnedChild { parent_binding, parent_site, .. }, _, Some(stored)) => {
                            *parent_binding == stored.parent_binding
                                && *parent_site == stored.parent_site
                                && reference.target.owner() == stored.child_class.as_ref()
                        }
                        _ => false,
                    }
                }))
        })
    }
    pub(super) fn fields_closed(&self, fields: &BTreeSet<FieldKeyV1>) -> bool {
        self.core_sources_match() && self.leaves.iter().all(|leaf| match leaf {
            LeafV1::Field { key, .. } => fields.contains(key),
            _ => true,
        })
    }
    fn core_sources_match(&self) -> bool {
        self.leaves.iter().all(|leaf| match leaf {
            LeafV1::Core(core) => core.matches_plan(&self.plan, core.node())
                && (!core.source().target().checked_array_count_law_v1() || core.field_array_count_v1().is_some() || self.leaves.iter().any(|leaf|
                    matches!(leaf, LeafV1::Core(inner) if core.matches_receiver_result_v1(inner)))),
            LeafV1::CoreReceiver(index) => matches!(self.leaves.get(*index), Some(LeafV1::Core(core))
                if (core.field_map().is_some() || core.field_array_count_v1().is_some()) && core.matches_plan(&self.plan, *index)),
            _ => true,
        })
    }

    pub(super) fn fields_match(
        &self,
        sources: &FieldSourcesV1,
        objects: &ObjectFieldClassSourcesV1,
    ) -> bool {
        self.core_sources_match() && self.leaves.iter().all(|leaf| match leaf {
            LeafV1::Core(core) => core.field_map().is_none_or(|field|
                sources.values().any(|original| field.matches_declared_source_v1(original))),
            LeafV1::MapLocal { binding, key, source, get } => {
                binding.owner() == self.plan.owner() && get.owner() == self.plan.owner()
                    && source.original_get_v1(get)
                    && sources.get(key).is_some_and(|original| Rc::ptr_eq(original, &source.source))
            }
            LeafV1::ObjectField { key, source } => objects
                .get(key)
                .is_some_and(|other| Rc::ptr_eq(source, other) && source.field().is_some()),
            LeafV1::Field { key, source } => sources
                .get(key)
                .is_some_and(|other| Rc::ptr_eq(source, other)),
            LeafV1::StoredObjectField {
                key,
                reference,
                receiver_site,
                source,
            } => {
                key.0.as_ref() == source.parent_class.as_ref()
                    && key.1.as_ref() == source.field_name.as_ref()
                    && self.leaves.iter().any(|candidate| matches!(candidate,
                        LeafV1::Call {
                            reference: actual,
                            stored_receiver: Some(stored),
                            receiver_site: actual_receiver,
                            ..
                        } if actual == reference && actual_receiver == receiver_site && stored == source))
            }
            _ => true,
        })
    }
}

fn stored_receiver_field_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    plan: &ExpressionSourcePlanV1,
    field_site: &SourceExprSiteV1,
    needs: &[Result<Option<super::super::source::PreparedSourceCallNeedV1>, String>],
) -> Option<(
    CallTargetReferenceV1,
    SourceExprSiteV1,
    super::super::StoredReceiverSourceV1,
)> {
    let mut found = None;
    for node in plan.nodes() {
        if !matches!(node.source(), ExpressionNodeSourceV1::MethodCall) {
            continue;
        }
        let Some(call) = input.function().method_call(node.site()) else {
            continue;
        };
        if call.receiver_site() != field_site {
            continue;
        }
        let Ok(Some((reference, _, need))) =
            pending::exact_call_source_v1(input, node.site(), needs)
        else {
            continue;
        };
        let Some(stored) = need.stored() else {
            continue;
        };
        if found.is_some() {
            return None;
        }
        found = Some((reference, call.receiver_site().clone(), stored.clone()));
    }
    found
}

pub(super) fn field_sources_v1(claims: &OrdinaryNewFieldWriteClaimsV1) -> FieldSourcesV1 {
    claims
        .value_sources()
        .filter_map(|(class, name, source)| {
            let source = source.as_ref().ok()?;
            source
                .same_inventory(claims)
                .then(|| ((class.into(), name.into()), Rc::clone(source)))
        })
        .collect()
}

/// Bind a local's original get to the complete Map payload-class census.
/// This is not the Entry receiver and lends no nonnull or lifetime permission.
fn map_local_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, binding: BindingRefV1,
    class: &str, receiver: Option<BindingRefV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    claims: &OrdinaryNewFieldWriteClaimsV1,
) -> Option<LeafV1> {
    use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::{result_class_claim, container_field_uses::MapFieldPayloadSourceV1};
    let function = input.function();
    let initializer = result_class_claim::sole_initializer_site(function, binding)?;
    let get = function.method_call_source_v1(&initializer)?;
    if get.owner() != input.owner() || get.selector() != "get" || get.arity() != 1
        || !input.forest().callable_source_ledger(input.owner()).ok()?.uncaptured_binding(binding) { return None; }
    let BodyExpressionShapeV1::FieldAccess { object, field, .. } = input.body_shape()?.expression_shape(get.receiver_site())? else { return None; };
    if !receiver.is_some_and(|receiver| original::exact_me_receiver_v1(input, object, receiver)) { return None; }
    let source = MapFieldPayloadSourceV1::issue(batch, selected, constructors, claims, class, field)?;
    if !source.original_get_v1(get) { return None; }
    Some(LeafV1::MapLocal { binding, key: (class.into(), field.clone()), source: Rc::new(source), get: std::sync::Arc::clone(get) })
}
