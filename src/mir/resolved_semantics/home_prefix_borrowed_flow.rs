//! Readonly result joins over the existing source value graph, without a new CFG.
use super::*;
use crate::mir::normal_callable_semantic_package::{BorrowedNullableLocalSourceLoanV1 as Loan, BorrowedLifetimeAnchorV1};
use crate::mir::resolved_semantics::source_value_flow::{SourceValueEdgeV1 as Edge, SourceValueOriginV1 as Origin, SourceValueRefV1, VerifiedSourceValueFlowV1};
use std::sync::Arc;

/// Each original terminal has exactly its own loan, or an original literal
/// Null. The current value, not just its terminal set, identifies the join.
#[derive(Debug, Clone)]
pub(crate) struct BorrowedReadFlowV1 {
    flow: Arc<VerifiedSourceValueFlowV1>,
    value: SourceValueRefV1,
    binding: BindingRefV1,
    statement: SourceStmtSiteV1,
    terminals: BTreeMap<SourceValueRefV1, Option<Loan>>,
    class: Box<str>,
    support: BorrowedLifetimeAnchorV1,
}
impl PartialEq for BorrowedReadFlowV1 {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.flow, &other.flow) && self.value == other.value
            && self.binding == other.binding && self.statement == other.statement
            && self.terminals == other.terminals && self.class == other.class && self.support == other.support
    }
}
impl Eq for BorrowedReadFlowV1 {}
impl BorrowedReadFlowV1 {
    pub(crate) fn loans(&self) -> impl Iterator<Item = &Loan> { self.terminals.values().filter_map(Option::as_ref) }

    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 { &self.statement }
    pub(crate) fn declared_type(&self) -> &str { &self.class }
    pub(crate) fn has_literal_null(&self) -> bool { self.terminals.values().any(Option::is_none) }
    pub(crate) fn matches_read(&self, input: ResolvedFunctionLoweringInputV1<'_>, site: &SourceExprSiteV1) -> bool {
        let Some(flow) = input.function().expression_source().value_flow() else { return false; };
        let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(site) else { return false; };
        Arc::ptr_eq(flow, &self.flow) && flow.read(site, binding).and_then(|value|
            flow.alias_terminal_with_expression_boundary(value, |at| input.function().variable_ref(at), |_| false)) == Some(self.value)
    }
    pub(crate) fn incoming_loans(&self, index: usize) -> Option<Vec<&Loan>> {
        let (_, incoming) = self.flow.branch_join(&self.statement, self.binding)?;
        let value = incoming.get(index)?.1;
        let origins = self.flow.alias_origins_v1(value, |at| self.flow.read_at(at).map(|(binding, _)| ResolvedLexicalRefV1::Local(binding)))?;
        let mut loans = Vec::new();
        for origin in origins { loans.push(self.terminals.get(&origin)?.as_ref()?); }
        Some(loans)
    }
    pub(crate) fn incoming_expression(&self, index: usize) -> Option<&SourceExprSiteV1> {
        let (_, incoming) = self.flow.branch_join(&self.statement, self.binding)?;
        match self.flow.origin(incoming.get(index)?.1)? { Origin::Expression(site) => Some(site), _ => None }
    }
    fn branch_terminals(input: ResolvedFunctionLoweringInputV1<'_>, flow: &VerifiedSourceValueFlowV1,
        value: SourceValueRefV1, state: &StoredLocal,
    ) -> Option<BTreeMap<SourceValueRefV1, Option<Loan>>> {
        let candidates = match state {
            StoredLocal::BorrowedNullable(state) => {
                if let Some(source) = state.flow_source() {
                    let original = input.function().expression_source().value_flow()?;
                    if !Arc::ptr_eq(&source.flow, original)
                        || flow.alias_terminal_with_expression_boundary(value,
                            |at| input.function().variable_ref(at), |_| false) != Some(source.value) { return None; }
                }
                state.loans().collect::<Vec<_>>()
            },
            StoredLocal::Null => Vec::new(),
            _ => return None,
        };
        let mut terminals = BTreeMap::new();
        let mut used = BTreeSet::new();
        for origin in flow.alias_origins_v1(value, |at| input.function().variable_ref(at))? {
            let Origin::Expression(site) = flow.origin(origin)? else { return None; };
            let matches = candidates.iter().filter(|loan| loan.call().owner() == input.owner() && loan.call().site() == site).collect::<Vec<_>>();
            let loan = match matches.as_slice() {
                [loan] if loan.is_declared_read_v1() => {
                    let mut original = input.function().expression_source().initializers().filter(|row| row.initializer_site() == Some(site));
                    let original_row = original.next()?;
                    if original_row.binding() != loan.destination() || original_row.declaration_site() != loan.declaration()
                        || original.next().is_some() { return None; }
                    used.insert(loan.call().clone());
                    Some((***loan).clone())
                }
                [] if input.function().expression_source().literal(site) == Some(&ResolvedLiteralSourceV1::Null) => None,
                _ => return None,
            };
            terminals.insert(origin, loan);
        }
        // No unrelated loan may hide behind a branch-local state.
        (used.len() == candidates.len()).then_some(terminals)
    }

    pub(super) fn issue_join(input: ResolvedFunctionLoweringInputV1<'_>, binding: BindingRefV1,
        statement: &SourceStmtSiteV1, left: &StoredLocal, right: &StoredLocal,
    ) -> Option<Self> {
        let flow = input.function().expression_source().value_flow()?;
        if binding.owner() != input.owner() || flow.owner() != input.owner() { return None; }
        let (value, incoming) = flow.branch_join(statement, binding)?;
        let [(Edge::ThenNormal, left_value), (Edge::ElseNormal, right_value)] = incoming else { return None; };
        let mut terminals = Self::branch_terminals(input, flow, *left_value, left)?;
        for (origin, loan) in Self::branch_terminals(input, flow, *right_value, right)? {
            if terminals.get(&origin).is_some_and(|previous| previous != &loan) { return None; }
            terminals.insert(origin, loan);
        }
        let mut calls = terminals.values().filter_map(Option::as_ref);
        let first = calls.next()?;
        let class = first.declared_read_result_v1()?.declared_type();
        let support = first.lifetime_anchor_v1()?;
        for loan in calls {
            if !loan.is_declared_read_v1() || loan.declared_read_result_v1()?.declared_type() != class
                || loan.lifetime_anchor_v1()? != support { return None; }
        }
        Some(Self { flow: Arc::clone(flow), value, binding, statement: statement.clone(),
            class: class.into(), support, terminals })
    }
}

impl PrefixLocalFlow<'_> {
    pub(super) fn branch_borrowed_read_join_v1(&self, other: &Self, binding: BindingRefV1,
        statement: &SourceStmtSiteV1,
    ) -> Option<borrowed_nullable::BorrowedNullableStateV1> {
        if !std::ptr::eq(self.input.function(), other.input.function())
            || !std::ptr::eq(self.input.forest(), other.input.forest()) { return None; }
        let left = self.locals.get(&binding)?;
        let right = other.locals.get(&binding)?;
        let valid = |state: &StoredLocal| match state {
            StoredLocal::BorrowedNullable(state) => state.read_valid(),
            StoredLocal::Null => true,
            _ => false,
        };
        let source = BorrowedReadFlowV1::issue_join(self.input, binding, statement, left, right)?;
        // Preserve invalid provenance as invalid; a join never revives a read.
        let read_valid = valid(left) && valid(right)
            && source.loans().all(|loan| self.declared_read_support_live_v1(loan)
                && other.declared_read_support_live_v1(loan));
        Some(borrowed_nullable::BorrowedNullableStateV1::from_flow(source, read_valid))
    }
}

impl PrefixLocalFlow<'_> {
    /// Join a sibling fall-through branch state into this one. Both sides
    /// were forked from the same entry snapshot; a binding that exists on
    /// both sides must carry the identical stored class — disagreement is a
    /// divergence the caller names, never a merged guess.
    ///
    /// A binding present on only one side was declared inside that
    /// branch's own scope: `resolve_if` pushes an `IfThen`/`IfElse`
    /// lexical frame and `leave_region_scope` pops it, so no post-join
    /// source site can resolve that `BindingRefV1`. The join still must
    /// not keep the sibling's initialized class — the value was proven on
    /// one path only — so the surviving entry is downgraded to
    /// `Uninitialized`, which keeps `observe` and every `is_*` probe
    /// fail-closed if a future shape ever does resolve it.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn join_branch(&mut self, other: &Self) -> bool {
        self.join_branch_at(other, None)
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn join_branch_at(
        &mut self, other: &Self, site: Option<&SourceStmtSiteV1>,
    ) -> bool {
        if !source_loop::same_context(&self.source_loop, &other.source_loop) {
            return false;
        }
        for (binding, value) in &other.locals {
            if let Some(own) = self.locals.get_mut(binding) {
                if !borrowed_nullable::meet_read_state_v1(own, value) && !stored_local_same(own, value) {
                    if let Some(merged) = site.and_then(|site| self.branch_borrowed_read_join_v1(other, *binding, site)) {
                        self.store(*binding, StoredLocal::BorrowedNullable(merged));
                        continue;
                    }
                    let Some(merged) = site.and_then(|site|
                        self.branch_text_join_v1(other, *binding, site)) else {
                        return false;
                    };
                    self.store(*binding, StoredLocal::TextBranch { source: merged });
                }
            }
        }
        for binding in self.locals.keys().copied().collect::<Vec<_>>() {
            if !other.locals.contains_key(&binding) {
                self.store(binding, StoredLocal::Uninitialized);
            }
        }
        for binding in other.locals.keys() {
            if !self.locals.contains_key(binding) {
                self.store(*binding, StoredLocal::Uninitialized);
            }
        }
        self.null_exclusions
            .retain(|root, proof| other.null_exclusions.get(root) == Some(proof));
        // Narrowing is path-sensitive: a mark survives the join only when
        // both siblings carried it.
        self.nonnull
            .retain(|binding| other.nonnull.contains(binding));
        self.opaque_formals
            .retain(|binding| other.opaque_formals.contains(binding));
        true
    }

}

/// Prefix-authorized use of one exact joined readonly value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedFlowReadV1 {
    source: std::rc::Rc<BorrowedReadFlowV1>,
    read: OwnedExprSiteV1,
    binding: BindingRefV1,
}
impl BorrowedFlowReadV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn issue(input: ResolvedFunctionLoweringInputV1<'_>, source: std::rc::Rc<BorrowedReadFlowV1>, site: &SourceExprSiteV1) -> Option<Self> {
        let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(site)? else { return None; };
        source.matches_read(input, site).then(|| Self { source, binding, read: OwnedExprSiteV1::new(input.owner(), site.clone()) })
    }
    pub(crate) fn source(&self) -> &std::rc::Rc<BorrowedReadFlowV1> { &self.source }
    pub(crate) fn same_flow_v1(&self, flow: &Arc<VerifiedSourceValueFlowV1>) -> bool { Arc::ptr_eq(&self.source.flow, flow) }
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 { &self.read }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn corroborates(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        self.read.owner() == input.owner() && input.function().variable_ref(self.read.site()) == Some(ResolvedLexicalRefV1::Local(self.binding))
            && self.source.matches_read(input, self.read.site())
    }
}
