//! Qualify the RHS of the SAME original Plain field assignment.
//! Source effects and target reach are checked by the Home walk first.
use super::super::{CurrentScalarReadV1, SourceScalarKind};
use super::*;
use super::{CheckedExpressionReferenceV1 as Reference, CheckedScalarReferenceV1 as Scalar};
use crate::mir::resolved_semantics::{ResolvedAssignmentFormV1, SourceExprSiteV1};
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1 as Class,
    expression_plan::ExpressionPlanRootV1,
    expression_projection::{project_expression_nodes_v1, ExpressionLeafClassV1 as Leaf},
};
use std::collections::{BTreeMap, BTreeSet};

/// Original whole-RHS scalar/Text evaluation or exact Array transfer, never execution
/// permission. Acquisition, checked commit and End remain physical obligations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QualifiedFieldWriteRhsV1 {
    source: Rc<ExpressionSourcePlanV1>,
    order: Box<[usize]>,
    classes: Box<[Leaf]>,
    references: BTreeMap<
        usize,
        (
            Reference,
            Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>>,
        ),
    >,
    // A conditional transfer of the exact live holder, not a physical End grant.
    owned_array: Option<crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1>,
    evaluation: Option<Rc<CheckedExpressionEvaluationV1>>,
}
impl QualifiedFieldWriteRhsV1 {
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.source
    }
    pub(crate) fn owned_array(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1> {
        self.owned_array.as_ref()
    }
    pub(crate) fn evaluation(&self) -> Option<&CheckedExpressionEvaluationV1> {
        self.evaluation.as_deref()
    }
    pub(crate) fn ordered_nodes(&self) -> &[usize] {
        self.evaluation().map_or(&self.order, |evaluation| evaluation.ordered_nodes())
    }
    pub(crate) fn node_class(&self, node: usize) -> Option<Leaf> {
        match self.evaluation() {
            Some(evaluation) => evaluation.class(node),
            None => self.classes.get(node).copied(),
        }
    }
    pub(crate) fn reference_source(&self, node: usize) -> Option<&Reference> {
        match self.evaluation() {
            Some(evaluation) => evaluation.references().get(&node),
            None => self.references.get(&node).map(|row| &row.0),
        }
    }
    pub(crate) fn reference(
        &self,
        node: usize,
    ) -> Option<(
        crate::mir::resolved_semantics::BindingRefV1,
        SourceScalarKind,
    )> {
        let source = self.reference_source(node)?;
        let kind = match source {
            Reference::Scalar(read) => read.kind(),
            Reference::FiniteEntry {
                read,
                exclusion: None,
            } => match read.leaf_class(None)? {
                Leaf::Value(Class::I64) => SourceScalarKind::Integer,
                Leaf::Value(Class::Bool) => SourceScalarKind::Bool,
                _ => return None,
            },
            _ => return None,
        };
        Some((source.binding(), kind))
    }
    pub(crate) fn signed_reference(
        &self,
        node: usize,
    ) -> Option<(
        &CurrentScalarReadV1,
        &Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>,
    )> {
        let (Reference::Scalar(Scalar::Original(read)), Some(sum)) = self.references.get(&node)?
        else {
            return None;
        };
        Some((read, sum))
    }
    pub(crate) fn is_consistent(&self, source: &Rc<ExpressionSourcePlanV1>) -> bool {
        if let Some(evaluation) = self.evaluation() {
            return Rc::ptr_eq(source, self.source())
                && Rc::ptr_eq(source, evaluation.source())
                && self.owned_array.is_none() && self.order.is_empty()
                && self.classes.is_empty() && self.references.is_empty()
                && !source.has_opaque_obligations()
                && evaluation.root_class() == Leaf::Value(Class::String)
                && evaluation.reference_coverage_is_exact()
                && matches!(source.root(), ExpressionPlanRootV1::Assignment { assignment }
                    if assignment.form() == ResolvedAssignmentFormV1::Plain
                    && source.nodes().first().is_some_and(|node| node.site() == assignment.value_site()));
        }
        if let Some(array) = self.owned_array() {
            return Rc::ptr_eq(source, self.source())
                && source.owner() == array.binding().owner()
                && !source.has_opaque_obligations()
                && self.classes.as_ref() == [Leaf::Identity]
                && self.order.as_ref() == [0]
                && self.references.is_empty()
                && matches!(source.root(), ExpressionPlanRootV1::Assignment { assignment }
                    if assignment.form() == ResolvedAssignmentFormV1::Plain
                    && source.nodes().first().is_some_and(|node| node.site() == assignment.value_site()))
                && matches!(source.nodes(), [node] if matches!(node.source(),
                    ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == array.binding()));
        }
        if !Rc::ptr_eq(source, self.source())
            || self.classes.len() != source.nodes().len()
            || self.node_class(0) != Some(Leaf::Value(Class::I64))
            || source.ordered_postorder_v1().ok().as_deref() != Some(self.ordered_nodes())
        {
            return false;
        }
        let mut references = 0;
        for (index, node) in source.nodes().iter().enumerate() {
            if let ExpressionNodeSourceV1::Reference(binding) = node.source() {
                if self.node_class(index) == Some(Leaf::Identity) {
                    continue;
                }
                let Some((original, kind)) = self.reference(index) else {
                    return false;
                };
                if !self
                    .reference_source(index)
                    .is_some_and(|proof| proof.matches_node(source, index))
                    || *binding
                        != crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(original)
                    || original.owner() != source.owner()
                    || self.node_class(index)
                        != Some(Leaf::Value(match kind {
                            SourceScalarKind::Integer => Class::I64,
                            SourceScalarKind::Bool => Class::Bool,
                        }))
                {
                    return false;
                }
                if self.signed_reference(index).is_some_and(|(read, sum)| {
                    read.read() != node.site()
                        || read.binding() != original
                        || read.kind() != kind
                        || !sum.signed_only()
                }) {
                    return false;
                }
                references += 1;
            }
        }
        references == self.references.len()
            && project_expression_nodes_v1(source, &BTreeSet::new(), |index| self.classes[index])
                .is_some_and(|classes| classes.as_slice() == self.classes.as_ref())
    }
}

pub(in crate::mir::resolved_semantics::home_new_prefix) fn qualify_field_write_expression_v1<E>(
    source: Rc<ExpressionSourcePlanV1>,
    require_i64: bool,
    frame: Option<Rc<CheckedFieldWriteFrameV1>>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<SourceScalarKind>,
    signed_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        CurrentScalarReadV1,
        Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>,
    )>,
    finite_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<
        crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
    >,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    let ExpressionPlanRootV1::Assignment { assignment } = source.root() else {
        return Ok(None);
    };
    if assignment.form() != ResolvedAssignmentFormV1::Plain
        || source.has_opaque_obligations()
        || source
            .nodes()
            .first()
            .is_none_or(|root| root.site() != assignment.value_site())
    {
        return Ok(None);
    }
    if frame.as_ref().is_some_and(|frame| {
        !require_i64
            || !Rc::ptr_eq(frame.source(), &source)
            || (frame.owned_local().is_none() && frame.entry().is_none())
    }) {
        return Ok(None);
    }
    let mut leaves = vec![Leaf::Unresolved; source.nodes().len()];
    let mut requests = Vec::new();
    let mut references = BTreeMap::new();
    let mut qualified_rhs = None;
    for (index, node) in source.nodes().iter().enumerate() {
        if matches!(
            node.source(),
            ExpressionNodeSourceV1::Shape(
                crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
            )
        ) {
            let Some(request) = field_request(node.site()) else {
                return Ok(None);
            };
            if source.field_read_node_v1(
                request.site.site(),
                &request.receiver_site,
                request.receiver,
                &request.field,
            ) != Some(index)
            {
                return Ok(None);
            }
            let Some(receiver) = source
                .nodes()
                .iter()
                .position(|node| node.site() == &request.receiver_site)
            else {
                return Ok(None);
            };
            leaves[receiver] = Leaf::Identity;
            leaves[index] = Leaf::Value(Class::I64);
            requests.push(request);
        }
    }
    // Existing non-arithmetic field effects retain their original source without
    // gaining scalar capability. Every Sub/Div node, however, owes a typed proof.
    let scalar_nodes: Vec<_> = source
        .nodes()
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            node.binary()
                .filter(|row| {
                    matches!(
                        row.operator(),
                        ResolvedBinaryOperatorV1::Subtract | ResolvedBinaryOperatorV1::Divide
                    )
                })
                .map(|binary| (index, binary))
        })
        .collect();
    let mut operations = Vec::new();
    if require_i64 || !scalar_nodes.is_empty() {
        for (index, node) in source.nodes().iter().enumerate() {
            if leaves[index] == Leaf::Identity {
                continue;
            }
            if matches!(node.source(), ExpressionNodeSourceV1::Reference(_)) {
                let (proof, sum, kind) = if let Some(kind) = scalar_reference(node.site()) {
                    let ExpressionNodeSourceV1::Reference(
                        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
                    ) = node.source()
                    else {
                        return Ok(None);
                    };
                    let (scalar, sum) = match signed_reference(node.site())
                        .filter(|(_, sum)| sum.signed_only())
                    {
                        Some((read, sum)) if read.binding() == *binding && read.kind() == kind => {
                            (Scalar::Original(read), Some(sum))
                        }
                        Some(_) => return Ok(None),
                        None => (
                            Scalar::Obligation {
                                binding: *binding,
                                kind,
                            },
                            None,
                        ),
                    };
                    (Reference::Scalar(scalar), sum, kind)
                } else if let Some(read) = finite_reference(node.site()) {
                    let kind = match read.leaf_class(None) {
                        Some(Leaf::Value(Class::I64)) => SourceScalarKind::Integer,
                        Some(Leaf::Value(Class::Bool)) => SourceScalarKind::Bool,
                        _ => return Ok(None),
                    };
                    (
                        Reference::FiniteEntry {
                            read,
                            exclusion: None,
                        },
                        None,
                        kind,
                    )
                } else {
                    return Ok(None);
                };
                if !proof.matches_node(&source, index) {
                    return Ok(None);
                }
                references.insert(index, (proof, sum));
                leaves[index] = Leaf::Value(match kind {
                    SourceScalarKind::Integer => Class::I64,
                    SourceScalarKind::Bool => Class::Bool,
                });
            }
        }
        let Some(classes) =
            project_expression_nodes_v1(&source, &BTreeSet::new(), |index| leaves[index])
        else {
            return Ok(None);
        };
        if require_i64 && classes.first() != Some(&Leaf::Value(Class::I64)) {
            return Ok(None);
        }
        let indices: BTreeMap<_, _> = source
            .nodes()
            .iter()
            .enumerate()
            .map(|(index, node)| (node.site(), index))
            .collect();
        for (index, binary) in scalar_nodes {
            if [binary.lhs(), binary.rhs()].iter().any(|site| {
                indices
                    .get(site)
                    .is_none_or(|index| classes[*index] != Leaf::Value(Class::I64))
            }) {
                return Ok(None);
            }
            let positive_divisor = if binary.operator() == ResolvedBinaryOperatorV1::Divide {
                let Some(rhs) = indices
                    .get(binary.rhs())
                    .map(|index| &source.nodes()[*index])
                else {
                    return Ok(None);
                };
                let ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Integer(value)) =
                    rhs.source()
                else {
                    return Ok(None);
                };
                if *value <= 0 {
                    return Ok(None);
                }
                Some(*value)
            } else {
                None
            };
            operations.push(Rc::new(ExpressionScalarOperationLoanV1 {
                plan: Rc::clone(&source),
                node: index,
                positive_divisor,
            }));
        }
        if frame.is_some() {
            let Ok(order) = source.ordered_postorder_v1() else {
                return Ok(None);
            };
            let rhs = Rc::new(QualifiedFieldWriteRhsV1 {
                source: Rc::clone(&source),
                order: order.into_boxed_slice(),
                classes: classes.into_boxed_slice(),
                references,
                owned_array: None,
                evaluation: None,
            });
            if !rhs.is_consistent(&source) {
                return Ok(None);
            }
            qualified_rhs = Some(rhs);
        }
    }
    let candidate = HomeExpressionPlanV1 {
        source: Rc::clone(&source),
        scalar_operations: operations,
        source_fields: Vec::new(),
        initializer_homes: None,
        argument_frame: None,
        initializer_frame: None,
        rebind_frame: None,
        print_frame: None,
        condition_frame: None,
        loop_recipe: None,
        field_write: frame.map(|frame| FieldWritePlanV1 {
            frame,
            rhs: qualified_rhs,
        }),
    };
    let Some(candidate) = candidate.with_source_field_requirements(&requests) else {
        return Ok(None);
    };
    if !can_install(&candidate) {
        return Ok(None);
    }
    if !field_batch(
        &requests,
        super::super::LocalFieldReadDemandV1::SourceExpression { probe: false, plan: source },
    )? {
        return Ok(None);
    }
    Ok(Some(candidate))
}

impl HomeExpressionPlanV1 {
    /// Same original assignment, using the shared checked Text evaluator. This
    /// wrapper grants neither a physical operand nor a FieldSet consume lease.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn text_field_write(
        frame: Rc<CheckedFieldWriteFrameV1>,
        evaluation: Rc<CheckedExpressionEvaluationV1>,
    ) -> Option<Self> {
        frame.entry()?.text_value()?;
        if !Rc::ptr_eq(frame.source(), evaluation.source())
            || frame.prior_homes() != evaluation.prior_homes() {
            return None;
        }
        let source = Rc::clone(frame.source());
        let rhs = Rc::new(QualifiedFieldWriteRhsV1 {
            source: Rc::clone(&source), order: Box::default(), classes: Box::default(),
            references: BTreeMap::new(), owned_array: None, evaluation: Some(evaluation),
        });
        let plan = Self {
            source, scalar_operations: Vec::new(), source_fields: Vec::new(),
            initializer_homes: None, argument_frame: None, initializer_frame: None,
            rebind_frame: None, print_frame: None, condition_frame: None, loop_recipe: None,
            field_write: Some(FieldWritePlanV1 { frame, rhs: Some(rhs) }),
        };
        plan.is_consistent().then_some(plan)
    }
    /// Original Plain LOCAL Array move, staged before the source Home successor.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn owned_array_field_write(
        frame: Rc<CheckedFieldWriteFrameV1>,
        array: crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1,
    ) -> Option<Self> {
        let local = frame.owned_local()?;
        let field = local
            .construction()
            .allocation_only_field(local.original().write().field())?;
        if field.field() != local.field()
            || field.storage() != crate::mir::function::TypedObjectFieldStorage::Dynamic
            || (!array.has_element_contract()
                && !array.proves_empty_transfer(
                    local.original().write().assignment(),
                    local.root(),
                    local.original().write().field(),
                ))
            || local.root() == array.binding()
            || frame
                .prior_homes()
                .iter()
                .filter(|home| home.binding() == array.binding())
                .count()
                != 1
        {
            return None;
        }
        let source = Rc::clone(frame.source());
        let rhs = Rc::new(QualifiedFieldWriteRhsV1 {
            source: Rc::clone(&source),
            order: vec![0].into_boxed_slice(),
            classes: vec![Leaf::Identity].into_boxed_slice(),
            references: BTreeMap::new(),
            owned_array: Some(array),
            evaluation: None,
        });
        if !rhs.is_consistent(&source) {
            return None;
        }
        Some(Self {
            source,
            scalar_operations: Vec::new(),
            source_fields: Vec::new(),
            initializer_homes: None,
            argument_frame: None,
            initializer_frame: None,
            rebind_frame: None,
            print_frame: None,
            condition_frame: None,
            loop_recipe: None,
            field_write: Some(FieldWritePlanV1 {
                frame,
                rhs: Some(rhs),
            }),
        })
    }
}
