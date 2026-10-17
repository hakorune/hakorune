//! Original non-owning field assignment and the Home walk's exact live set.
//! The selected entry must separately lend receiver/value guards and field.
use super::*;
use crate::mir::resolved_semantics::{BindingRefV1, ResolvedAssignmentFormV1};
use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedBorrowedFieldWriteFrameV1 {
    source: Rc<ExpressionSourcePlanV1>,
    // Same allocation order as Print/Bool. The sole unwind owner reverses it.
    prior_homes: Box<[BindingRefV1]>,
}
impl CheckedBorrowedFieldWriteFrameV1 {
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
        let frame = Rc::new(CheckedBorrowedFieldWriteFrameV1 {
            source: Rc::clone(&source), prior_homes: homes.into(),
        });
        Some(Self { source, scalar_operations: Vec::new(), source_fields: Vec::new(),
            print_frame: None, condition_frame: None, borrowed_field_write: Some(frame) })
    }
}
