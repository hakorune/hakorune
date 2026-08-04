//! Same-pass facts needed to project the first portable If recipe.
//!
//! This module is an observer owned by the existing trivial analyzer.  It does
//! not select a route, emit MIR, or own portable recipe identities.  The
//! producer/adapter later maps these owner-branded facts to recipe-local keys.

use std::collections::BTreeMap;

use crate::ast::{BinaryOperator, LiteralValue};
use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

use super::product::TrivialRepresentationV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TrivialRecipeBinaryOpV1 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

impl TrivialRecipeBinaryOpV1 {
    fn from_ast(operator: &BinaryOperator) -> Option<Self> {
        Some(match operator {
            BinaryOperator::Add => Self::Add,
            BinaryOperator::Subtract => Self::Subtract,
            BinaryOperator::Multiply => Self::Multiply,
            BinaryOperator::Divide => Self::Divide,
            BinaryOperator::Modulo => Self::Modulo,
            BinaryOperator::BitAnd => Self::BitAnd,
            BinaryOperator::BitOr => Self::BitOr,
            BinaryOperator::BitXor => Self::BitXor,
            BinaryOperator::Shl => Self::Shl,
            BinaryOperator::Shr => Self::Shr,
            BinaryOperator::Equal => Self::Equal,
            BinaryOperator::NotEqual => Self::NotEqual,
            BinaryOperator::Less => Self::Less,
            BinaryOperator::Greater => Self::Greater,
            BinaryOperator::LessEqual => Self::LessEqual,
            BinaryOperator::GreaterEqual => Self::GreaterEqual,
            BinaryOperator::And | BinaryOperator::Or => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TrivialRecipeExprKindV1 {
    Read {
        binding: BindingRefV1,
    },
    ConstI64 {
        value: i64,
    },
    ConstBool {
        value: bool,
    },
    Binary {
        op: TrivialRecipeBinaryOpV1,
        left: SourceExprSiteV1,
        right: SourceExprSiteV1,
    },
    DirectStaticCall,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TrivialRecipeExprFactV1 {
    site: SourceExprSiteV1,
    representation: TrivialRepresentationV1,
    kind: TrivialRecipeExprKindV1,
}

impl TrivialRecipeExprFactV1 {
    fn new(
        site: SourceExprSiteV1,
        representation: TrivialRepresentationV1,
        kind: TrivialRecipeExprKindV1,
    ) -> Self {
        Self {
            site,
            representation,
            kind,
        }
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn representation(&self) -> TrivialRepresentationV1 {
        self.representation
    }

    pub(crate) fn kind(&self) -> &TrivialRecipeExprKindV1 {
        &self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RecipeBranchV1 {
    Then,
    Else,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AssignmentFactV1 {
    statement: SourceStmtSiteV1,
    binding: BindingRefV1,
    value: SourceExprSiteV1,
    representation: TrivialRepresentationV1,
}

impl AssignmentFactV1 {
    pub(crate) const fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn value(&self) -> &SourceExprSiteV1 {
        &self.value
    }

    pub(crate) const fn representation(&self) -> TrivialRepresentationV1 {
        self.representation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IfEntryWitnessV1 {
    binding: BindingRefV1,
    representation: TrivialRepresentationV1,
}

impl IfEntryWitnessV1 {
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn representation(&self) -> TrivialRepresentationV1 {
        self.representation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IfFactDraftV1 {
    statement: SourceStmtSiteV1,
    condition: SourceExprSiteV1,
    explicit_else: bool,
    then_body: Option<SourceBodySiteV1>,
    else_body: Option<SourceBodySiteV1>,
    then_assignments: Vec<AssignmentFactV1>,
    else_assignments: Vec<AssignmentFactV1>,
    continuation_read: Option<SourceExprSiteV1>,
    entry_witness: Option<IfEntryWitnessV1>,
    direct_call_site: Option<SourceExprSiteV1>,
}

impl IfFactDraftV1 {
    fn new(statement: SourceStmtSiteV1, condition: SourceExprSiteV1, explicit_else: bool) -> Self {
        Self {
            statement,
            condition,
            explicit_else,
            then_body: None,
            else_body: None,
            then_assignments: Vec::new(),
            else_assignments: Vec::new(),
            continuation_read: None,
            entry_witness: None,
            direct_call_site: None,
        }
    }

    fn assignments(&self, branch: RecipeBranchV1) -> &[AssignmentFactV1] {
        match branch {
            RecipeBranchV1::Then => &self.then_assignments,
            RecipeBranchV1::Else => &self.else_assignments,
        }
    }
}

/// The owner-branded witness emitted by the same traversal as the trivial
/// representation profile.  It is intentionally not a portable artifact.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialIfRecipeFactsV1 {
    if_fact: IfFactDraftV1,
    expressions: Box<[TrivialRecipeExprFactV1]>,
}

impl VerifiedTrivialIfRecipeFactsV1 {
    pub(crate) fn if_site(&self) -> &SourceStmtSiteV1 {
        &self.if_fact.statement
    }

    pub(crate) fn condition(&self) -> &SourceExprSiteV1 {
        &self.if_fact.condition
    }

    pub(crate) fn then_body(&self) -> Option<&SourceBodySiteV1> {
        self.if_fact.then_body.as_ref()
    }

    pub(crate) fn else_body(&self) -> Option<&SourceBodySiteV1> {
        self.if_fact.else_body.as_ref()
    }

    pub(crate) const fn has_explicit_else(&self) -> bool {
        self.if_fact.explicit_else
    }

    pub(crate) const fn has_implicit_else(&self) -> bool {
        !self.if_fact.explicit_else
    }

    pub(crate) fn then_assignment_count(&self) -> usize {
        self.if_fact.then_assignments.len()
    }

    pub(crate) fn else_assignment_count(&self) -> usize {
        self.if_fact.else_assignments.len()
    }

    pub(super) fn assignments(&self, branch: RecipeBranchV1) -> &[AssignmentFactV1] {
        self.if_fact.assignments(branch)
    }

    pub(crate) fn continuation_read(&self) -> Option<&SourceExprSiteV1> {
        self.if_fact.continuation_read.as_ref()
    }

    pub(crate) fn entry_witness(&self) -> Option<IfEntryWitnessV1> {
        self.if_fact.entry_witness
    }

    pub(crate) fn then_assignment(&self) -> Option<&AssignmentFactV1> {
        self.if_fact.then_assignments.first()
    }

    pub(crate) fn else_assignment(&self) -> Option<&AssignmentFactV1> {
        self.if_fact.else_assignments.first()
    }

    pub(crate) fn expressions(&self) -> &[TrivialRecipeExprFactV1] {
        &self.expressions
    }

    pub(crate) fn expression_count(&self) -> usize {
        self.expressions.len()
    }

    pub(crate) fn direct_call_site(&self) -> Option<&SourceExprSiteV1> {
        self.if_fact.direct_call_site.as_ref()
    }
}

#[derive(Debug, Default)]
pub(super) struct TrivialIfRecipeFactsDraftV1 {
    expressions: BTreeMap<SourceExprSiteV1, TrivialRecipeExprFactV1>,
    ifs: Vec<IfFactDraftV1>,
    if_stack: Vec<usize>,
    branches: Vec<(usize, RecipeBranchV1)>,
    pending_continuation: Option<(usize, BindingRefV1)>,
    unsupported: bool,
    direct_call_site: Option<SourceExprSiteV1>,
}

impl TrivialIfRecipeFactsDraftV1 {
    pub(super) fn record_literal(
        &mut self,
        site: SourceExprSiteV1,
        value: &LiteralValue,
        representation: TrivialRepresentationV1,
    ) {
        let kind = match value {
            LiteralValue::Integer(value) | LiteralValue::TypedInteger { value, .. } => {
                TrivialRecipeExprKindV1::ConstI64 { value: *value }
            }
            LiteralValue::Bool(value) => TrivialRecipeExprKindV1::ConstBool { value: *value },
            _ => {
                self.unsupported = true;
                return;
            }
        };
        self.expressions.insert(
            site.clone(),
            TrivialRecipeExprFactV1::new(site, representation, kind),
        );
    }

    pub(super) fn record_read(
        &mut self,
        site: SourceExprSiteV1,
        binding: BindingRefV1,
        representation: TrivialRepresentationV1,
    ) {
        self.expressions.insert(
            site.clone(),
            TrivialRecipeExprFactV1::new(
                site.clone(),
                representation,
                TrivialRecipeExprKindV1::Read { binding },
            ),
        );
        if self.branches.is_empty() {
            if let Some((if_index, expected)) = self.pending_continuation {
                if expected == binding {
                    self.ifs[if_index].continuation_read = Some(site);
                }
            }
        }
    }

    pub(super) fn record_binary(
        &mut self,
        site: SourceExprSiteV1,
        operator: &BinaryOperator,
        left: SourceExprSiteV1,
        right: SourceExprSiteV1,
        representation: TrivialRepresentationV1,
    ) {
        let Some(op) = TrivialRecipeBinaryOpV1::from_ast(operator) else {
            self.unsupported = true;
            return;
        };
        self.expressions.insert(
            site.clone(),
            TrivialRecipeExprFactV1::new(
                site,
                representation,
                TrivialRecipeExprKindV1::Binary { op, left, right },
            ),
        );
    }

    pub(super) fn begin_if(
        &mut self,
        statement: SourceStmtSiteV1,
        condition: SourceExprSiteV1,
        explicit_else: bool,
    ) {
        let index = self.ifs.len();
        self.ifs
            .push(IfFactDraftV1::new(statement, condition, explicit_else));
        self.if_stack.push(index);
    }

    pub(super) fn set_bodies(
        &mut self,
        then_body: SourceBodySiteV1,
        else_body: Option<SourceBodySiteV1>,
    ) {
        if let Some(index) = self.if_stack.last().copied() {
            self.ifs[index].then_body = Some(then_body);
            self.ifs[index].else_body = else_body;
        }
    }

    pub(super) fn begin_branch(&mut self, branch: RecipeBranchV1) {
        if let Some(index) = self.if_stack.last().copied() {
            self.branches.push((index, branch));
        }
    }

    pub(super) fn end_branch(&mut self) {
        self.branches.pop();
    }

    pub(super) fn record_assignment(
        &mut self,
        statement: SourceStmtSiteV1,
        binding: BindingRefV1,
        value: SourceExprSiteV1,
        representation: TrivialRepresentationV1,
    ) {
        let Some((index, branch)) = self.branches.last().copied() else {
            return;
        };
        let assignment = AssignmentFactV1 {
            statement,
            binding,
            value,
            representation,
        };
        match branch {
            RecipeBranchV1::Then => self.ifs[index].then_assignments.push(assignment),
            RecipeBranchV1::Else => self.ifs[index].else_assignments.push(assignment),
        }
    }

    pub(super) fn mark_unsupported(&mut self) {
        self.unsupported = true;
    }

    pub(super) fn record_direct_call(&mut self, site: SourceExprSiteV1) {
        let is_branch_assignment_value = self.in_branch()
            && matches!(site.node().segments().last(), Some(SourcePathSegmentV1::Value));
        if !is_branch_assignment_value || self.direct_call_site.is_some() {
            self.unsupported = true;
            return;
        }
        self.direct_call_site = Some(site.clone());
        self.expressions.insert(
            site.clone(),
            TrivialRecipeExprFactV1::new(
                site,
                TrivialRepresentationV1::InlineI64,
                TrivialRecipeExprKindV1::DirectStaticCall,
            ),
        );
    }

    pub(super) fn in_branch(&self) -> bool {
        !self.branches.is_empty()
    }

    pub(super) fn finish_if(
        &mut self,
        merge_bindings: &[BindingRefV1],
        baseline: &BTreeMap<BindingRefV1, TrivialRepresentationV1>,
    ) {
        let Some(index) = self.if_stack.pop() else {
            return;
        };
        if let [binding] = merge_bindings {
            let Some(representation) = baseline.get(binding).copied() else {
                self.unsupported = true;
                self.pending_continuation = None;
                return;
            };
            self.ifs[index].entry_witness = Some(IfEntryWitnessV1 {
                binding: *binding,
                representation,
            });
            self.pending_continuation = Some((index, *binding));
        } else {
            self.pending_continuation = None;
        }
    }

    pub(super) fn finish(self) -> Option<VerifiedTrivialIfRecipeFactsV1> {
        if self.unsupported || self.ifs.len() != 1 {
            return None;
        }
        let mut if_fact = self.ifs.into_iter().next()?;
        if_fact.direct_call_site = self.direct_call_site;
        let branch_shape_ok = if if_fact.explicit_else {
            if_fact.then_assignments.len() == 1
                && if_fact.else_assignments.len() == 1
                && if_fact.else_body.is_some()
        } else {
            if_fact.then_assignments.len() == 1
                && if_fact.else_assignments.is_empty()
                && if_fact.else_body.is_none()
        };
        if !branch_shape_ok
            || if_fact.continuation_read.is_none()
            || if_fact.entry_witness.is_none()
        {
            return None;
        }
        let entry = if_fact.entry_witness?;
        let then = &if_fact.then_assignments[0];
        if if_fact.explicit_else {
            if then.binding != if_fact.else_assignments[0].binding
                || then.representation != if_fact.else_assignments[0].representation
            {
                return None;
            }
        } else if then.binding != entry.binding || then.representation != entry.representation {
            return None;
        }
        Some(VerifiedTrivialIfRecipeFactsV1 {
            if_fact,
            expressions: self.expressions.into_values().collect(),
        })
    }
}
