//! The one evaluation body and its typed source-ordered ownership data.
use super::*;

/// No independent binding group is published here. Every retained original
/// receipt and live temporary moves to the checked terminal consumer together.
pub(in crate::mir::builder::ordinary_new_admission::selected) struct EmittedExpressionValueV1 {
    pub(in crate::mir::builder::ordinary_new_admission::selected) value: EvaluatedExpressionValueV1,
    pub(in crate::mir::builder::ordinary_new_admission::selected) purpose_acquisition:
        Option<[Binding; 2]>,
    pub(in crate::mir::builder::ordinary_new_admission::selected) body: EmittedExpressionBodyV1,
}

/// One evaluated source scope. A comparison borrows its inputs without
/// inventing a root value or re-rooting the original source expression.
#[derive(Debug)]
pub(in crate::mir) struct EmittedExpressionBodyV1 {
    pub(in crate::mir) values: Vec<Option<EvaluatedExpressionValueV1>>,
    pub(in crate::mir) calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
    pub(in crate::mir) dynamic_calls: Vec<Rc<EmittedDynamicInvocationCallProjectionV1>>,
    pub(in crate::mir) path_reads: Vec<Rc<crate::mir::builder::ExactLexicalReadV1>>,
    pub(in crate::mir) reference_reads:
        std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
    pub(in crate::mir) temporaries: Vec<(usize, ExpressionOwnedTemporaryV1)>,
    pub(in crate::mir) homes: Vec<InvokeOperation>,
    pub(in crate::mir) fault_frame: Option<ValueId>,
    pub(in crate::mir) entry: Option<Binding>,
    pub(in crate::mir) inline_range: Option<(BasicBlockId, usize, usize, bool)>,
    pub(in crate::mir) bindings: Vec<Binding>,
    pub(in crate::mir) root_cleanup: Option<(Binding, Option<BasicBlockId>)>,
}

impl EmittedExpressionBodyV1 {
    pub(in crate::mir) fn string_acquisitions_v1(&self) -> Vec<(usize, ValueId)> {
        string_acquisitions_v1(&self.temporaries)
    }
    /// Borrow the same source-ordered acquisitions for an owned Text destination.
    /// Callers first corroborate this body against its retained checked frame;
    /// this view is not a second source or acquisition inventory.
    pub(in crate::mir) fn ordered_owned_temporaries_v1(
        &self,
        frame: &CheckedExpressionEvaluationV1,
        root: ValueId,
        materialized: bool,
    ) -> Result<Vec<ExpressionOwnedTemporaryV1>, String> {
        let mut values = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        if self.temporaries.iter().map(|(node, _)| *node).ne(frame
            .ordered_nodes()
            .iter()
            .copied()
            .filter(|node| frame.owned_result_nodes().contains(node)))
        {
            return Err(freeze("argument-expression/deferred-value"));
        }
        for &(node, temporary) in &self.temporaries {
            if !temporary.matches_value_v1(self.values.get(node).and_then(Option::as_ref)) {
                return Err(freeze("argument-expression/deferred-value"));
            }
            if !seen.insert(temporary.value()) {
                return Err(freeze("argument-expression/deferred-duplicate"));
            }
            values.push(temporary);
        }
        if materialized {
            if !seen.insert(root) {
                return Err(freeze("argument-expression/deferred-duplicate"));
            }
            values.push(ExpressionOwnedTemporaryV1::String(root));
        }
        let expected = ExpressionOwnedTemporaryV1::String(root);
        if values.last() != Some(&expected) {
            return Err(freeze("argument-expression/deferred-root-order"));
        }
        Ok(values)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ExpressionOwnedTemporaryV1 {
    String(ValueId),
    DynamicCarrier(ValueId),
}
impl ExpressionOwnedTemporaryV1 {
    pub(in crate::mir) fn end_operation(self) -> Result<InvokeOperation, String> {
        match self {
            Self::String(value) => Ok(InvokeOperation::String(
                crate::mir::instruction::StringInvokeOperation::End { value },
            )),
            Self::DynamicCarrier(carrier) => Ok(InvokeOperation::DynamicCarrierEnd { carrier }),
        }
    }
    pub(in crate::mir) fn value(self) -> ValueId {
        match self {
            Self::String(value) | Self::DynamicCarrier(value) => value,
        }
    }
    pub(in crate::mir) fn matches_value_v1(
        self,
        value: Option<&EvaluatedExpressionValueV1>,
    ) -> bool {
        match (self, value) {
            (
                Self::String(expected),
                Some(EvaluatedExpressionValueV1::Operand(Operand::OwnedString(actual))),
            ) => expected == *actual,
            (
                Self::DynamicCarrier(expected),
                Some(EvaluatedExpressionValueV1::Operand(Operand::DynamicCarrier(actual))),
            ) => expected == *actual,
            _ => false,
        }
    }
    pub(in crate::mir) fn is_string(self, value: ValueId) -> bool {
        self == Self::String(value)
    }
}

pub(super) fn string_acquisitions_v1(
    temporaries: &[(usize, ExpressionOwnedTemporaryV1)],
) -> Vec<(usize, ValueId)> {
    temporaries
        .iter()
        .filter_map(|(node, temporary)| match temporary {
            ExpressionOwnedTemporaryV1::String(value) => Some((*node, *value)),
            _ => None,
        })
        .collect()
}
