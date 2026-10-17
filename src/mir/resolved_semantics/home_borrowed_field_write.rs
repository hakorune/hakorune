//! One original assignment frame with explicit borrowed-formal or owned-LOCAL origin.
//! The selected entry must separately lend receiver/value guards and field.
use super::*;
use crate::mir::resolved_semantics::{BindingRefV1, ResolvedAssignmentFormV1};
use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedFieldWriteFrameV1 {
    source: Rc<ExpressionSourcePlanV1>,
    // Same allocation order as Print/Bool. The sole unwind owner reverses it.
    prior_homes: Box<[BindingRefV1]>,
    receiver: FieldWriteReceiverV1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldWriteReceiverV1 {
    FormalBorrowed,
    OwnedLocal(crate::mir::normal_callable_semantic_package::OwnedLocalFieldWriteSourceV1),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FieldWritePlanV1 {
    pub(super) frame: Rc<CheckedFieldWriteFrameV1>,
    pub(super) rhs: Option<Rc<QualifiedFieldWriteRhsV1>>,
}
impl CheckedFieldWriteFrameV1 {
    pub(crate) fn owned_local(&self) -> Option<&crate::mir::normal_callable_semantic_package::OwnedLocalFieldWriteSourceV1> {
        match &self.receiver { FieldWriteReceiverV1::OwnedLocal(source) => Some(source), _ => None }
    }
    pub(crate) fn is_borrowed(&self) -> bool { matches!(self.receiver, FieldWriteReceiverV1::FormalBorrowed) }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn for_owned_local(
        local: crate::mir::normal_callable_semantic_package::OwnedLocalFieldWriteSourceV1,
        homes: &[BindingRefV1],
    ) -> Option<Rc<Self>> {
        let mut unique = std::collections::BTreeSet::new();
        if !homes.contains(&local.root()) || homes.iter().any(|home|
            home.owner() != local.source().owner() || !unique.insert(*home)) { return None; }
        Some(Rc::new(Self { source: Rc::clone(local.source()), prior_homes: homes.into(),
            receiver: FieldWriteReceiverV1::OwnedLocal(local) }))
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { &self.source }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] { &self.prior_homes }
}

impl HomeExpressionPlanV1 {
    /// The caller has already proved the original formal write's receiver and
    /// RHS uses in the canonical branch walk. No physical permission is issued.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn borrowed_field_write(
        source: Rc<ExpressionSourcePlanV1>, homes: &[BindingRefV1],
    ) -> Option<Self> {
        let ExpressionPlanRootV1::Assignment { assignment } = source.root() else { return None };
        if assignment.form() != ResolvedAssignmentFormV1::Plain || source.has_opaque_obligations()
            || source.nodes().first().is_none_or(|node| node.site() != assignment.value_site())
        { return None; }
        let mut unique = std::collections::BTreeSet::new();
        if homes.iter().any(|home| home.owner() != source.owner() || !unique.insert(*home)) {
            return None;
        }
        let frame = Rc::new(CheckedFieldWriteFrameV1 {
            source: Rc::clone(&source), prior_homes: homes.into(), receiver: FieldWriteReceiverV1::FormalBorrowed,
        });
        Some(Self { source, scalar_operations: Vec::new(), source_fields: Vec::new(), initializer_homes: None,
            print_frame: None, condition_frame: None, field_write: Some(FieldWritePlanV1 { frame, rhs: None }) })
    }
}
