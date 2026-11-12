//! Completed expression observations, separate from the emitter's working state.
use super::*;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedExpressionEvaluationV1, CheckedPrintExpressionFrameV1,
};
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct ExpressionEmissionV1<C> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) contract:
        C,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) values:
        Box<[Option<EvaluatedExpressionValueV1>]>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) calls:
        Box<[Rc<EmittedLexicalCallProjectionV1>]>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) path_reads:
        Box<[Rc<crate::mir::builder::ExactLexicalReadV1>]>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) reference_reads:
        std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) original:
        Vec<Binding>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) entry:
        Option<Binding>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) inline_range:
        Option<(BasicBlockId, usize, usize, bool)>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) terminal:
        Option<Binding>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) continuation:
        Option<Binding>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) root_fault:
        Option<BasicBlockId>,
}

pub(crate) trait ExpressionEmissionSourceV1 {
    fn evaluation(&self) -> &CheckedExpressionEvaluationV1;
}
impl ExpressionEmissionSourceV1 for Rc<CheckedPrintExpressionFrameV1> {
    fn evaluation(&self) -> &CheckedExpressionEvaluationV1 {
        self.as_ref().evaluation()
    }
}
impl<C: ExpressionEmissionSourceV1> ExpressionEmissionV1<C> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn from_body(
        contract: C,
        body: crate::mir::builder::EmittedExpressionBodyV1,
        terminal: Option<Binding>,
        continuation: Option<Binding>,
    ) -> Self {
        Self {
            contract,
            values: body.values.into_boxed_slice(),
            calls: body.calls.into_boxed_slice(),
            path_reads: body.path_reads.into_boxed_slice(),
            reference_reads: body.reference_reads,
            original: body.bindings,
            entry: body.entry,
            inline_range: body.inline_range,
            terminal,
            continuation,
            root_fault: body.root_cleanup.and_then(|(_, fault)| fault),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn expression(
        &self,
    ) -> super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::expression_projection::ExpressionPhysicalProjectionRefV1::new(
            self.contract.evaluation(),
            &self.values,
            &self.calls,
            &self.path_reads,
            &self.reference_reads,
            &self.original,
        )
        .with_inline_range_v1(self.inline_range)
        .with_root_fault_v1(self.root_fault)
    }
}
