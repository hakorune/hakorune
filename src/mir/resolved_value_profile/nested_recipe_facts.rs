//! Same-pass facts for the isolated depth-one Nested If profile.

use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1, SourceStmtSiteV1};

use super::recipe_facts::{AssignmentFactV1, IfEntryWitnessV1, TrivialRecipeExprFactV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedIfNodeFactsV1 {
    statement: SourceStmtSiteV1,
    condition: SourceExprSiteV1,
    then_body: SourceBodySiteV1,
    else_body: SourceBodySiteV1,
    then_assignments: Box<[AssignmentFactV1]>,
    else_assignments: Box<[AssignmentFactV1]>,
    entry_witness: IfEntryWitnessV1,
}

impl NestedIfNodeFactsV1 {
    pub(crate) fn new(
        statement: SourceStmtSiteV1,
        condition: SourceExprSiteV1,
        then_body: SourceBodySiteV1,
        else_body: SourceBodySiteV1,
        then_assignments: Vec<AssignmentFactV1>,
        else_assignments: Vec<AssignmentFactV1>,
        entry_witness: IfEntryWitnessV1,
    ) -> Self {
        Self {
            statement,
            condition,
            then_body,
            else_body,
            then_assignments: then_assignments.into_boxed_slice(),
            else_assignments: else_assignments.into_boxed_slice(),
            entry_witness,
        }
    }

    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) fn condition(&self) -> &SourceExprSiteV1 {
        &self.condition
    }

    pub(crate) fn then_body(&self) -> &SourceBodySiteV1 {
        &self.then_body
    }

    pub(crate) fn else_body(&self) -> &SourceBodySiteV1 {
        &self.else_body
    }

    pub(crate) fn then_assignments(&self) -> &[AssignmentFactV1] {
        &self.then_assignments
    }

    pub(crate) fn else_assignments(&self) -> &[AssignmentFactV1] {
        &self.else_assignments
    }

    pub(crate) const fn entry_witness(&self) -> IfEntryWitnessV1 {
        self.entry_witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedNestedTrivialIfRecipeFactsV1 {
    outer: NestedIfNodeFactsV1,
    inner: NestedIfNodeFactsV1,
    continuation_read: SourceExprSiteV1,
    expressions: Box<[TrivialRecipeExprFactV1]>,
}

impl VerifiedNestedTrivialIfRecipeFactsV1 {
    pub(crate) fn new(
        outer: NestedIfNodeFactsV1,
        inner: NestedIfNodeFactsV1,
        continuation_read: SourceExprSiteV1,
        expressions: Vec<TrivialRecipeExprFactV1>,
    ) -> Self {
        Self {
            outer,
            inner,
            continuation_read,
            expressions: expressions.into_boxed_slice(),
        }
    }

    pub(crate) fn outer(&self) -> &NestedIfNodeFactsV1 {
        &self.outer
    }

    pub(crate) fn inner(&self) -> &NestedIfNodeFactsV1 {
        &self.inner
    }

    pub(crate) fn continuation_read(&self) -> &SourceExprSiteV1 {
        &self.continuation_read
    }

    pub(crate) fn expressions(&self) -> &[TrivialRecipeExprFactV1] {
        &self.expressions
    }

    pub(crate) fn shared_binding(&self) -> BindingRefV1 {
        self.outer.entry_witness().binding()
    }
}
