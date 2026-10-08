//! Original checked Normal-Integer evidence for one later source argument.
//! The existing use classifier is the sole producer. No entry, execution,
//! positivity, Object class or physical carrier permission is issued here.

use std::rc::Rc;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, RegionId, ResolvedLexicalRefV1, ScopeId, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1,
};

use super::{BorrowedCompareSourceV1, BorrowedFormalUseDraftErrorV1};

#[derive(Debug)]
pub(super) struct CheckedIntegerGuardV1 {
    binary: OwnedExprSiteV1,
    source: Rc<BorrowedCompareSourceV1>,
    formal: BindingRefV1,
    statement: SourceNodeSiteV1,
    parent: RegionId,
    scope: ScopeId,
}

impl CheckedIntegerGuardV1 {
    pub(super) fn from_compare(
        input: ResolvedFunctionLoweringInputV1<'_>,
        formal: BindingRefV1,
        binary: &OwnedExprSiteV1,
        source: Rc<BorrowedCompareSourceV1>,
    ) -> Result<Self, BorrowedFormalUseDraftErrorV1> {
        let function = input.function();
        let (statement, control) = function
            .with_if_region_for_condition(binary.site(), |row| {
                (row.site().node().clone(), row.bundle().control())
            })
            .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
        let parent = function
            .region(control)
            .and_then(|row| row.parent())
            .ok_or(BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
        let scope = function
            .region(parent)
            .and_then(|row| row.lexical_scope())
            .ok_or(BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
        if formal.owner() != input.owner()
            || binary.owner() != input.owner()
            || function.exact_scope_containing(&statement) != Some(scope)
        {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        Ok(Self {
            binary: binary.clone(),
            source,
            formal,
            statement,
            parent,
            scope,
        })
    }

    fn precedes(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
    ) -> bool {
        let guard = self.statement.segments();
        let path = site.node().segments();
        let Some((last, prefix)) = guard.split_last() else {
            return false;
        };
        if path.len() <= prefix.len()
            || &path[..prefix.len()] != prefix
            || input.function().exact_scope_containing(site.node()) != Some(self.scope)
        {
            return false;
        }
        // Only exact sibling statements in one admitted sequence. The older
        // first-divergence helper admits descendant/bypass paths and is not
        // authority for this source fact.
        match (last, &path[prefix.len()]) {
            (SourcePathSegmentV1::Body(a), SourcePathSegmentV1::Body(b))
            | (SourcePathSegmentV1::IfThen(a), SourcePathSegmentV1::IfThen(b))
            | (SourcePathSegmentV1::IfElse(a), SourcePathSegmentV1::IfElse(b)) => a < b,
            _ => false,
        }
    }

    fn corroborates(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        let function = input.function();
        if self.binary.owner() != input.owner() || self.formal.owner() != input.owner() {
            return false;
        }
        let Some(binary) = function.expression_source().binary(self.binary.site()) else {
            return false;
        };
        let (operator, left, right, _) = self.source.comparison_parts();
        if binary.operator() != operator
            || left.owner() != input.owner()
            || right.owner() != input.owner()
            || binary.lhs() != left.site()
            || binary.rhs() != right.site()
        {
            return false;
        }
        function
            .with_if_region_for_condition(self.binary.site(), |row| {
                row.site().node() == &self.statement
                    && function
                        .region(row.bundle().control())
                        .and_then(|row| row.parent())
                        == Some(self.parent)
                    && function
                        .region(self.parent)
                        .and_then(|row| row.lexical_scope())
                        == Some(self.scope)
                    && function.exact_scope_containing(&self.statement) == Some(self.scope)
            })
            .unwrap_or(false)
    }
}

#[derive(Debug)]
pub(crate) struct BorrowedGuardedActualV1 {
    guard: Rc<CheckedIntegerGuardV1>,
    call: OwnedExprSiteV1,
    ordinal: u32,
    site: OwnedExprSiteV1,
    binding: BindingRefV1,
}

impl BorrowedGuardedActualV1 {
    pub(super) fn from_call(
        input: ResolvedFunctionLoweringInputV1<'_>,
        guard: Rc<CheckedIntegerGuardV1>,
        call: &SourceExprSiteV1,
        ordinal: u32,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) -> Option<Self> {
        if !guard.precedes(input, site) {
            return None;
        }
        let fact = Self {
            guard,
            call: OwnedExprSiteV1::new(input.owner(), call.clone()),
            ordinal,
            site: OwnedExprSiteV1::new(input.owner(), site.clone()),
            binding,
        };
        fact.corroborates(input, &fact.call, ordinal, site)
            .then_some(fact)
    }

    pub(crate) fn call(&self) -> &OwnedExprSiteV1 {
        &self.call
    }

    pub(crate) fn corroborates(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> bool {
        let function = input.function();
        let Some(original) = function.method_call(call.site()) else {
            return false;
        };
        self.call == *call
            && call.owner() == input.owner()
            && original.owner() == input.owner()
            && original.site() == call.site()
            && self.ordinal == ordinal
            && original
                .arguments()
                .get(ordinal as usize)
                .is_some_and(|row| row.ordinal() == ordinal && row.site() == site)
            && self.site.owner() == input.owner()
            && self.site.site() == site
            && self.binding.owner() == input.owner()
            && function.variable_ref(site) == Some(ResolvedLexicalRefV1::Local(self.binding))
            && self.guard.corroborates(input)
            && self.guard.precedes(input, site)
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_guarded_actual_tests.rs"]
mod tests;
