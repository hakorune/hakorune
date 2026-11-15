//! Typed eager-expression values and source-qualified non-owning read transport.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedExpressionReferenceV1 as Reference, NullableResultReferenceV1,
};

/// Internal evaluation values keep Bool separate from String/I64 operands.
/// Conversion borrows an existing domain; it never casts or stringifies Bool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EvaluatedExpressionValueV1 {
    Operand(Operand),
    Bool(ValueId),
    /// A source-qualified non-owning reference or sentinel, never a numeric/String operand.
    BoxReference(EvaluatedBoxReferenceV1),
    Null(ValueId),
}
impl From<Operand> for EvaluatedExpressionValueV1 {
    fn from(value: Operand) -> Self {
        Self::Operand(value)
    }
}
impl EvaluatedExpressionValueV1 {
    pub(in crate::mir) fn as_operand(&self) -> Result<&Operand, String> {
        match self {
            Self::Operand(value) => Ok(value),
            Self::Bool(_) | Self::BoxReference(_) | Self::Null(_) => {
                Err(freeze("expression/scalar-continuation-result"))
            }
        }
    }
    pub(in crate::mir) fn value(&self) -> Option<ValueId> {
        match self {
            Self::Operand(value) => value.value(),
            Self::Bool(value) | Self::Null(value) => Some(*value),
            Self::BoxReference(value) => Some(value.value()),
        }
    }
}

/// Ownership is value data, separate from its source origin. C0 constructs
/// Borrowed only; Owned reserves the existing SSA-slot token for later stages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EvaluatedBoxReferenceV1 {
    value: ValueId,
    ownership: BoxReferenceOwnershipV1,
    origin: BoxReferenceOriginV1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BoxReferenceOwnershipV1 {
    Borrowed(BoxReferenceSupportV1),
    // Filled only after the approved Retain/owned-result connection in C onward.
    #[allow(dead_code)]
    Owned {
        token: ValueId,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        nullable: bool,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoxReferenceOriginV1 {
    Local,
    Field,
    Call,
    // Source provenance of later owned construction, not another ownership mode.
    #[allow(dead_code)]
    Fresh,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BoxReferenceSupportV1 {
    Alias(Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionOperationLoanV1>),
    DeclaredFrom(crate::mir::resolved_semantics::home_new_prefix::DeclaredReadValueV1),
    Field(Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>),
}
impl EvaluatedBoxReferenceV1 {
    pub(in crate::mir) fn value(&self) -> ValueId {
        self.value
    }
    /// Only an existing canonical payload proof may supply layout membership.
    /// Alias/from declarations alone have no canonical layout at this phase.
    pub(in crate::mir) fn object(&self) -> Option<hakorune_mir_defs::CanonicalObjectIdV1> {
        match &self.ownership {
            BoxReferenceOwnershipV1::Borrowed(BoxReferenceSupportV1::Field(source)) => match source.result() {
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedObject(child) => Some(child.child()), _ => None,
            },
            BoxReferenceOwnershipV1::Borrowed(BoxReferenceSupportV1::DeclaredFrom(read)) => {
                let mut loans = read.loans();
                let first = loans.next()?.map_payload_v1()?.1;
                loans.all(|loan| loan.map_payload_v1().is_some_and(|(_, object)| object == first)).then_some(first)
            }
            BoxReferenceOwnershipV1::Borrowed(BoxReferenceSupportV1::Alias(_)) => None,
            BoxReferenceOwnershipV1::Owned { object, .. } => Some(*object),
        }
    }
    pub(in crate::mir) fn field_source(
        &self,
    ) -> Option<&Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>> {
        match &self.ownership {
            BoxReferenceOwnershipV1::Borrowed(BoxReferenceSupportV1::Field(source)) => Some(source),
            _ => None,
        }
    }
}
impl EvaluatedExpressionValueV1 {
    pub(in crate::mir) fn borrowed_reference_v1(
        value: ValueId,
        reference: &Reference,
    ) -> Result<Self, String> {
        let (support, origin) = match reference {
            Reference::Alias(loan) => (
                BoxReferenceSupportV1::Alias(Rc::clone(loan)),
                BoxReferenceOriginV1::Local,
            ),
            Reference::NullableResult(NullableResultReferenceV1::DeclaredFrom(read)) => (
                BoxReferenceSupportV1::DeclaredFrom(read.clone()),
                BoxReferenceOriginV1::Call,
            ),
            _ => return Err(freeze("expression/reference-source-node")),
        };
        Ok(Self::BoxReference(EvaluatedBoxReferenceV1 {
            value,
            ownership: BoxReferenceOwnershipV1::Borrowed(support),
            origin,
        }))
    }
    pub(in crate::mir) fn field_read_v1(
        value: ValueId,
        source: &Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
        frame: &CheckedExpressionEvaluationV1,
        node: usize,
    ) -> Self {
        use crate::mir::resolved_value_profile::{
            expression_class::ExpressionValueClassV1 as Class,
            expression_projection::ExpressionLeafClassV1 as Leaf,
        };
        if frame.class(node) == Some(Leaf::Value(Class::BoxReference)) {
            Self::BoxReference(EvaluatedBoxReferenceV1 {
                value,
                ownership: BoxReferenceOwnershipV1::Borrowed(BoxReferenceSupportV1::Field(
                    Rc::clone(source),
                )),
                origin: BoxReferenceOriginV1::Field,
            })
        } else if matches!(
            source.result(),
            crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Text(_)
        ) {
            Operand::OwnedString(value).into()
        } else {
            Operand::NormalInteger(value).into()
        }
    }
}

impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn declared_read_value_v1(
        &self,
        frame: &CheckedExpressionEvaluationV1,
        node: usize,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        let reference = frame
            .references()
            .get(&node)
            .ok_or_else(|| freeze("expression/reference-source-node"))?;
        if !reference.matches_node(frame.source(), node) {
            return Err(freeze("expression/reference-source-node"));
        }
        if let Reference::Alias(_) = reference {
            let (initialized, _) = self
                .exact_read_v1()
                .loan_local_initializer_v1(reference.binding())?;
            if self.definition().and_then(|(_, row)| row.dst_value()) != Some(initialized) {
                return Err(freeze("expression/reference-source-node"));
            }
            return self
                .exact_read_v1()
                .value_for(
                    frame.source().owner(),
                    frame.source().nodes()[node].site().node(),
                    reference.binding(),
                )
                .map_err(|error| error.to_string());
        }
        let Reference::NullableResult(NullableResultReferenceV1::DeclaredFrom(view)) = reference
        else {
            return Err(freeze("expression/reference-source-node"));
        };
        let (value, normal) = context.declared_value_read_v1(view, self.exact_read_v1())?;
        if self.definition() != Some(&normal) {
            return Err(freeze("expression/reference-source-node"));
        }
        Ok(value)
    }
}
