//! Original nullable-field initialization choices, shared by diagnostics and construction.
//! Receiver aliases come from the same source analysis as the nonescape proof.
use std::rc::Rc;
use crate::ast::{ASTNode, TypeReferenceV1};
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, FunctionOwnerIdV1,
    ResolvedAssignmentFormV1, ResolvedAssignmentSourceV1, ResolvedAssignmentTargetV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1 as Segment, VerifiedResolvedBodyShapeInventoryV1,
    VerifiedResolvedFunctionV1};
use crate::parser::ParserOrdinaryBoxSourceRowV1;
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use super::instance_constructor_non_escape::BirthReceiverUsesV1;

/// Statement evaluation and declaration initialization are distinct origins.
/// A declaration key never fabricates a resolver statement or expression site.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ConstructionInitKeyV1 {
    Statement(SourceNodeSiteV1),
    Declaration(CanonicalFieldRefV1),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConstructionInitOriginV1 {
    Assignment { assignment: ResolvedAssignmentSourceV1, receiver_site: SourceExprSiteV1 },
    Declaration { source: Rc<NullableConstructionInitializationV1>, field: CanonicalFieldRefV1 },
}
impl ConstructionInitOriginV1 {
    pub(crate) fn key(&self) -> ConstructionInitKeyV1 {
        match self {
            Self::Assignment { assignment, .. } => ConstructionInitKeyV1::Statement(assignment.statement_site().node().clone()),
            Self::Declaration { field, .. } => ConstructionInitKeyV1::Declaration(*field),
        }
    }
    pub(crate) fn assignment(&self) -> Option<&ResolvedAssignmentSourceV1> {
        match self { Self::Assignment { assignment, .. } => Some(assignment), Self::Declaration { .. } => None }
    }
    pub(crate) fn receiver_site(&self) -> Option<&SourceExprSiteV1> {
        match self { Self::Assignment { receiver_site, .. } => Some(receiver_site), Self::Declaration { .. } => None }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum NullableInitializationReasonV1 {
    DuplicateWrite, ConditionalWrite, LoopWrite, ReadBeforeInitialize,
    ReceiverCallBeforeInitialize, ReceiverEscapeBeforeInitialize,
}
impl NullableInitializationReasonV1 {
    fn code(self) -> &'static str {
        match self {
            Self::DuplicateWrite => "duplicate-write", Self::ConditionalWrite => "conditional-write",
            Self::LoopWrite => "loop-write", Self::ReadBeforeInitialize => "read-before-initialize",
            Self::ReceiverCallBeforeInitialize => "receiver-call-before-initialize",
            Self::ReceiverEscapeBeforeInitialize => "receiver-escape-before-initialize",
        }
    }
}
pub(in crate::mir) struct NullableInitializationIssueV1 {
    field: Box<str>, reason: NullableInitializationReasonV1, site: SourceNodeSiteV1,
}
impl std::fmt::Debug for NullableInitializationIssueV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[nullable-field-initialization/{}] field={} site={:?}", self.reason.code(), self.field, self.site)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NullableFieldInitializationV1 {
    pub(super) name: Box<str>,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) child: CanonicalObjectIdV1,
    pub(super) declared: TypeReferenceV1,
    pub(super) assignment: Option<ResolvedAssignmentSourceV1>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NullableConstructionInitializationV1 {
    source: ParserOrdinaryBoxSourceRowV1,
    owner: FunctionOwnerIdV1,
    fields: Box<[NullableFieldInitializationV1]>,
}
impl NullableConstructionInitializationV1 {
    pub(super) fn parent_class(&self) -> &str { self.source.name() }
    pub(super) fn declaration_default(&self, owner: FunctionOwnerIdV1, field: CanonicalFieldRefV1,
        child: CanonicalObjectIdV1, name: &str) -> bool {
        self.owner == owner && self.field(field).is_some_and(|row|
            row.child == child && row.name.as_ref() == name && row.assignment.is_none())
    }

    pub(super) fn matches(&self, source: &ParserOrdinaryBoxSourceRowV1, owner: FunctionOwnerIdV1) -> bool {
        self.source.same_source_as(source) && self.owner == owner
    }
    pub(super) fn field(&self, field: CanonicalFieldRefV1) -> Option<&NullableFieldInitializationV1> {
        self.fields().iter().find(|row| row.field == field)
    }
    pub(super) fn fields(&self) -> &[NullableFieldInitializationV1] { &self.fields }
}

fn root_position(site: &SourceNodeSiteV1) -> Option<u32> {
    match site.segments().first()? { Segment::Body(index) => Some(*index), _ => None }
}

pub(super) fn issue_nullable_initialization_v1(
    source: &ParserOrdinaryBoxSourceRowV1, object: CanonicalObjectIdV1,
    declaration: &ASTNode, function: &VerifiedResolvedFunctionV1,
    shape: &VerifiedResolvedBodyShapeInventoryV1, receiver: &BirthReceiverUsesV1,
    objects: &[(ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)],
) -> Result<Option<Rc<NullableConstructionInitializationV1>>, NullableInitializationIssueV1> {
    let ASTNode::BoxDeclaration { field_decls, .. } = declaration else { return Ok(None); };
    let mut fields = Vec::new();
    for (ordinal, declaration) in field_decls.iter().enumerate() {
        let Some(nominal) = declaration.declared_type_name.as_deref()
            .and_then(crate::mir::declared_type_storage::declared_object_field_type_v1)
            .filter(|ty| ty.explicitly_nullable()) else { continue; };
        let Some(child) = objects.iter().find_map(|(row, id)|
            (row.name() == nominal.class_name()).then_some(*id)) else { continue; };
        let Some(field) = CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal) else { continue; };
        let writes: Vec<_> = shape.assignment_sources().iter().filter(|row| {
            let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver: base }) = function.assignment_target(row.target_site())
                else { return false; };
            receiver.is_receiver(base) && matches!(shape.expression_shape(row.target_site()),
                Some(BodyExpressionShapeV1::FieldAccess { field, .. }) if field.as_ref() == declaration.name)
        }).collect();
        let issue = |reason, site: &SourceNodeSiteV1| NullableInitializationIssueV1 {
            field: declaration.name.as_str().into(), reason, site: site.clone(),
        };
        if writes.len() > 1 { return Err(issue(NullableInitializationReasonV1::DuplicateWrite, writes[1].statement_site().node())); }
        let assignment = if let Some(write) = writes.first() {
            let statement = write.statement_site().node();
            if !matches!(statement.segments(), [Segment::Body(_)]) {
                let reason = if statement.segments().iter().any(|part| matches!(part,
                    Segment::LoopBodyRoot | Segment::LoopBody(_) | Segment::LoopCondition)) {
                    NullableInitializationReasonV1::LoopWrite
                } else { NullableInitializationReasonV1::ConditionalWrite };
                return Err(issue(reason, statement));
            }
            if write.form() != ResolvedAssignmentFormV1::Plain {
                return Err(issue(NullableInitializationReasonV1::ReadBeforeInitialize, statement));
            }
            let position = root_position(statement).expect("straightline root statement");
            let earlier = |site: &SourceNodeSiteV1| root_position(site).is_some_and(|at| at <= position);
            for expression in shape.expressions() {
                if let BodyExpressionShapeV1::FieldAccess { site, object, field } = expression {
                    if field.as_ref() == declaration.name && receiver.is_receiver(object)
                        && site != write.target_site() && earlier(site.node()) {
                        return Err(issue(NullableInitializationReasonV1::ReadBeforeInitialize, site.node()));
                    }
                }
            }
            let calls: Vec<_> = function.method_calls().filter(|(_, call)| receiver.is_receiver(call.receiver_site())).collect();
            if let Some((site, _)) = calls.iter().find(|(site, _)| earlier(site.node())) {
                return Err(issue(NullableInitializationReasonV1::ReceiverCallBeforeInitialize, site.node()));
            }
            if let Some(site) = receiver.unproven_uses().find(|site| earlier(site.node())) {
                return Err(issue(NullableInitializationReasonV1::ReceiverEscapeBeforeInitialize, site.node()));
            }
            Some((*write).clone())
        } else { None };
        fields.push(NullableFieldInitializationV1 { name: declaration.name.clone().into(), field, child,
            declared: TypeReferenceV1::parse(declaration.declared_type_name.as_deref().expect("declared nullable type"))
                .expect("same structural query already parsed this declaration"), assignment });
    }
    Ok((!fields.is_empty()).then(|| Rc::new(NullableConstructionInitializationV1 {
        source: source.clone(), owner: receiver.receiver().owner(), fields: fields.into_boxed_slice(),
    })))
}
