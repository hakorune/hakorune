//! Passive source reaching definitions from the sole resolver traversal.
//! These are source identities and predecessor relations, never types or MIR.
use super::shadow::{ShadowBindingOrdinalV0, ShadowRegionIdV0};
use super::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedLexicalRefV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SourceValueRefV1 {
    owner: FunctionOwnerIdV1,
    slot: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct SourceValueSlotV0(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceValueJoinKindV1 {
    BranchNormal,
    LoopHeader,
    LoopAfter,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceValueEdgeV1 {
    ThenNormal,
    ElseNormal,
    LoopEntry,
    Backedge,
    ConditionFalse,
    Break(SourceStmtSiteV1),
    Continue(SourceStmtSiteV1),
}
#[derive(Debug, Clone)]
pub(crate) enum SourceValueOriginV1<B = BindingRefV1, V = SourceValueRefV1> {
    Entry(B),
    Expression(SourceExprSiteV1),
    Join {
        binding: B,
        site: SourceExprSiteV1,
        kind: SourceValueJoinKindV1,
        incoming: Box<[(SourceValueEdgeV1, V)]>,
    },
    Unknown,
}
#[derive(Debug)]
pub(crate) struct VerifiedSourceValueFlowV1 {
    owner: FunctionOwnerIdV1,
    complete: bool,
    nodes: Box<[SourceValueOriginV1]>,
    reads: BTreeMap<SourceExprSiteV1, (BindingRefV1, SourceValueRefV1)>,
}
impl VerifiedSourceValueFlowV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn is_complete(&self) -> bool {
        self.complete
    }
    pub(crate) fn read(
        &self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Option<SourceValueRefV1> {
        if !self.complete || binding.owner() != self.owner {
            return None;
        }
        self.reads
            .get(site)
            .filter(|(original, _)| *original == binding)
            .map(|(_, value)| *value)
    }
    /// Exact alias provenance only. Computed definitions, joins, unknowns and
    /// cycles cannot claim an entry origin from a convenient predecessor.
    pub(crate) fn alias_entry_origin(
        &self, value: SourceValueRefV1,
        reference: impl FnMut(&SourceExprSiteV1) -> Option<ResolvedLexicalRefV1>,
    ) -> Option<BindingRefV1> {
        match self.origin(self.alias_origin(value, reference)?)? {
            SourceValueOriginV1::Entry(binding) => Some(*binding),
            _ => None,
        }
    }
    /// Follow only exact lexical aliases. Terminal expression identity is
    /// retained; branches, unknowns and cycles never select a predecessor.
    pub(crate) fn alias_origin(
        &self, mut value: SourceValueRefV1,
        mut reference: impl FnMut(&SourceExprSiteV1) -> Option<ResolvedLexicalRefV1>,
    ) -> Option<SourceValueRefV1> {
        let mut seen = BTreeSet::new();
        while seen.insert(value) {
            match self.origin(value)? {
                SourceValueOriginV1::Entry(_) => return Some(value),
                SourceValueOriginV1::Expression(site) => {
                    match reference(site) {
                        Some(ResolvedLexicalRefV1::Local(binding)) => value = self.read(site, binding)?,
                        None => return Some(value),
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }
        None
    }
    pub(crate) fn origin(&self, value: SourceValueRefV1) -> Option<&SourceValueOriginV1> {
        if !self.complete || value.owner != self.owner {
            return None;
        }
        self.nodes.get(value.slot)
    }
}

type Binding = ShadowBindingOrdinalV0;
pub(super) type SourceFlowStateV0 = Option<BTreeMap<Binding, SourceValueSlotV0>>;
#[derive(Debug, Clone)]
struct LoopFrame {
    region: Option<ShadowRegionIdV0>,
    site: SourceExprSiteV1,
    head: BTreeMap<Binding, SourceValueSlotV0>,
    condition: SourceFlowStateV0,
    exits: Vec<(SourceValueEdgeV1, SourceFlowStateV0)>,
    continues: Vec<(SourceValueEdgeV1, SourceFlowStateV0)>,
}
#[derive(Debug, Clone)]
pub(super) struct SourceValueFlowDraftV1 {
    nodes: Vec<SourceValueOriginV1<Binding, SourceValueSlotV0>>,
    reads: BTreeMap<SourceExprSiteV1, (Binding, SourceValueSlotV0)>,
    state: SourceFlowStateV0,
    loops: Vec<LoopFrame>,
    // Source-owner-local trivial Phi reductions, proved from closed ORIGINAL
    // predecessor snapshots. No type/Home/physical permission is stored here.
    invariant_values: BTreeMap<SourceValueSlotV0, SourceValueSlotV0>,
    complete: bool,
}
impl Default for SourceValueFlowDraftV1 {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            reads: BTreeMap::new(),
            state: Some(BTreeMap::new()),
            loops: Vec::new(),
            invariant_values: BTreeMap::new(),
            complete: true,
        }
    }
}
impl SourceValueFlowDraftV1 {
    fn node(
        &mut self,
        origin: SourceValueOriginV1<Binding, SourceValueSlotV0>,
    ) -> SourceValueSlotV0 {
        let value = SourceValueSlotV0(self.nodes.len());
        self.nodes.push(origin);
        value
    }
    pub(super) fn unavailable(&mut self) {
        self.complete = false;
    }
    pub(super) fn declare(&mut self, binding: Binding, entry: bool) {
        let value = self.node(if entry {
            SourceValueOriginV1::Entry(binding)
        } else {
            SourceValueOriginV1::Unknown
        });
        if let Some(state) = &mut self.state {
            state.insert(binding, value);
        }
    }
    pub(super) fn initialize(&mut self, binding: Binding, expression: Option<SourceExprSiteV1>) {
        let value = self.node(
            expression
                .map(SourceValueOriginV1::Expression)
                .unwrap_or(SourceValueOriginV1::Unknown),
        );
        if let Some(state) = &mut self.state {
            state.insert(binding, value);
        }
    }
    pub(super) fn read(&mut self, site: SourceExprSiteV1, binding: Binding) {
        let value = self
            .state
            .as_ref()
            .and_then(|state| state.get(&binding))
            .copied()
            .unwrap_or_else(|| self.node(SourceValueOriginV1::Unknown));
        if self.reads.insert(site, (binding, value)).is_some() {
            self.unavailable();
        }
    }
    pub(super) fn snapshot(&self) -> SourceFlowStateV0 {
        self.state.clone()
    }
    pub(super) fn restore(&mut self, state: SourceFlowStateV0) {
        self.state = state;
    }
    pub(super) fn drop_bindings(&mut self, bindings: impl IntoIterator<Item = Binding>) {
        if let Some(state) = &mut self.state {
            for binding in bindings {
                state.remove(&binding);
            }
        }
    }
    fn join(
        &mut self,
        site: SourceExprSiteV1,
        kind: SourceValueJoinKindV1,
        bindings: impl IntoIterator<Item = Binding>,
        predecessors: &[(SourceValueEdgeV1, SourceFlowStateV0)],
    ) -> SourceFlowStateV0 {
        if !predecessors.iter().any(|(_, state)| state.is_some()) {
            return None;
        }
        let mut joined = BTreeMap::new();
        for binding in bindings {
            let mut incoming = Vec::new();
            for (edge, state) in predecessors {
                let Some(state) = state else {
                    continue;
                };
                let value = state
                    .get(&binding)
                    .copied()
                    .unwrap_or_else(|| self.node(SourceValueOriginV1::Unknown));
                incoming.push((edge.clone(), value));
            }
            let value = if kind == SourceValueJoinKindV1::BranchNormal
                && incoming.iter().all(|(_, value)| *value == incoming[0].1)
            {
                incoming[0].1
            } else {
                self.node(SourceValueOriginV1::Join {
                    binding,
                    site: site.clone(),
                    kind,
                    incoming: incoming.into_boxed_slice(),
                })
            };
            joined.insert(binding, value);
        }
        Some(joined)
    }
    pub(super) fn branch(
        &mut self,
        site: SourceExprSiteV1,
        entry: &SourceFlowStateV0,
        then_state: SourceFlowStateV0,
        else_state: SourceFlowStateV0,
    ) {
        self.state = self.join(
            site,
            SourceValueJoinKindV1::BranchNormal,
            entry.iter().flat_map(|state| state.keys().copied()),
            &[
                (SourceValueEdgeV1::ThenNormal, then_state),
                (SourceValueEdgeV1::ElseNormal, else_state),
            ],
        );
    }
    pub(super) fn begin_loop(&mut self, site: SourceExprSiteV1) {
        let entry = self.state.clone();
        let mut head = BTreeMap::new();
        for (binding, value) in entry.iter().flat_map(|state| state.iter()) {
            let phi = self.node(SourceValueOriginV1::Join {
                binding: *binding,
                site: site.clone(),
                kind: SourceValueJoinKindV1::LoopHeader,
                incoming: Box::new([(SourceValueEdgeV1::LoopEntry, *value)]),
            });
            head.insert(*binding, phi);
        }
        self.state = entry.map(|_| head.clone());
        self.loops.push(LoopFrame {
            region: None,
            site,
            head,
            condition: None,
            exits: Vec::new(),
            continues: Vec::new(),
        });
    }
    pub(super) fn enter_loop_body(&mut self, region: ShadowRegionIdV0) {
        let frame = self.loops.last_mut().expect("source Loop header exists");
        frame.region = Some(region);
        frame.condition = self.state.clone();
    }
    pub(super) fn exit_loop(
        &mut self,
        region: ShadowRegionIdV0,
        site: SourceStmtSiteV1,
        is_continue: bool,
    ) {
        let state = self.state.take();
        let Some(frame) = self
            .loops
            .iter_mut()
            .rev()
            .find(|frame| frame.region == Some(region))
        else {
            self.unavailable();
            return;
        };
        if is_continue {
            frame
                .continues
                .push((SourceValueEdgeV1::Continue(site), state));
        } else {
            frame.exits.push((SourceValueEdgeV1::Break(site), state));
        }
    }
    pub(super) fn terminate(&mut self) {
        self.state = None;
    }
    /// Compare all original reachable Loop snapshots, not a filtered join
    /// array. Nested readonly loops may lend only their already-proved reduction.
    fn invariant_loop_bindings(&self, frame: &LoopFrame) -> Vec<(Binding, SourceValueSlotV0)> {
        if !self.complete { return Vec::new(); }
        frame.head.iter().filter_map(|(binding, header)| {
            let SourceValueOriginV1::Join { binding: original, site, kind: SourceValueJoinKindV1::LoopHeader, incoming } = self.nodes.get(header.0)? else { return None; };
            if original != binding || site != &frame.site { return None; }
            let mut entries = incoming.iter().filter(|(edge, _)| *edge == SourceValueEdgeV1::LoopEntry);
            let (_, entry) = entries.next()?;
            if entries.next().is_some() || entry.0 >= header.0 { return None; }
            let anchor = canonical_slot(&self.invariant_values, *entry)?;
            let mut edges = Vec::new();
            for (edge, state) in frame.continues.iter().chain(frame.exits.iter()) {
                let Some(state) = state else { continue; };
                if edges.contains(edge) || canonical_slot(&self.invariant_values, *state.get(binding)?)? != *header { return None; }
                edges.push(edge.clone());
            }
            if edges.iter().filter(|edge| **edge == SourceValueEdgeV1::ConditionFalse).count() != 1 { return None; }
            Some((*binding, anchor))
        }).collect()
    }
    pub(super) fn finish_loop(&mut self, region: ShadowRegionIdV0) {
        let mut frame = self.loops.pop().expect("source Loop header exists");
        if frame.region != Some(region) {
            self.unavailable();
        }
        frame
            .continues
            .push((SourceValueEdgeV1::Backedge, self.state.take()));
        for (binding, value) in &frame.head {
            let mut extra = Vec::new();
            for (edge, state) in &frame.continues {
                if let Some(state) = state {
                    let incoming = state
                        .get(binding)
                        .copied()
                        .unwrap_or_else(|| self.node(SourceValueOriginV1::Unknown));
                    extra.push((edge.clone(), incoming));
                }
            }
            if let SourceValueOriginV1::Join { incoming, .. } = &mut self.nodes[value.0] {
                *incoming = incoming.iter().cloned().chain(extra).collect();
            } else {
                self.unavailable();
            }
        }
        frame
            .exits
            .push((SourceValueEdgeV1::ConditionFalse, frame.condition.take()));
        let invariant = self.invariant_loop_bindings(&frame);
        let after = self.join(
            frame.site,
            SourceValueJoinKindV1::LoopAfter,
            frame.head.keys().copied(),
            &frame.exits,
        );
        if let Some(after) = &after {
            for (binding, anchor) in invariant {
                // Both original port values share the same pre-Loop anchor.
                // All active snapshots remain untouched until final sealing.
                self.invariant_values.insert(frame.head[&binding], anchor);
                self.invariant_values.insert(after[&binding], anchor);
            }
        }
        self.state = after;
    }

    pub(super) fn seal(
        self,
        owner: FunctionOwnerIdV1,
        binding: impl Fn(Binding) -> BindingRefV1,
        expressions: &BTreeSet<SourceExprSiteV1>,
        statements: &BTreeSet<SourceStmtSiteV1>,
        uses: &BTreeMap<SourceExprSiteV1, ResolvedLexicalRefV1>,
    ) -> Result<VerifiedSourceValueFlowV1, &'static str> {
        if !self.loops.is_empty() {
            return Err("source value flow has unfinished Loop");
        }
        let invariant_values = if self.complete { self.invariant_values } else { BTreeMap::new() };
        let canonical = |slot| canonical_slot(&invariant_values, slot).ok_or("cyclic source value reduction");
        let count = self.nodes.len();
        let value_ref = |slot: SourceValueSlotV0| SourceValueRefV1 {
            owner,
            slot: slot.0,
        };
        let nodes = self
            .nodes
            .into_iter()
            .map(|node| {
                Ok(match node {
                    SourceValueOriginV1::Entry(original) => {
                        let original = binding(original);
                        if original.owner() != owner {
                            return Err("foreign source value entry");
                        }
                        SourceValueOriginV1::Entry(original)
                    }
                    SourceValueOriginV1::Expression(site) => {
                        if !expressions.contains(&site) {
                            return Err("source value expression not observed");
                        }
                        SourceValueOriginV1::Expression(site)
                    }
                    SourceValueOriginV1::Join {
                        binding: original,
                        site,
                        kind,
                        incoming,
                    } => {
                        let original = binding(original);
                        if original.owner() != owner
                            || incoming.is_empty()
                            || (!expressions.contains(&site)
                                && !statements.iter().any(|stmt| stmt.node() == site.node()))
                            || incoming.iter().any(|(edge, value)| {
                                value.0 >= count
                                    || match edge {
                                        SourceValueEdgeV1::Break(site)
                                        | SourceValueEdgeV1::Continue(site) => {
                                            !statements.contains(site)
                                        }
                                        _ => false,
                                    }
                            })
                        {
                            return Err("invalid source value predecessor");
                        }
                        SourceValueOriginV1::Join {
                            binding: original,
                            site,
                            kind,
                            incoming: incoming
                                .into_vec()
                                .into_iter()
                                .map(|(edge, value)| Ok((edge, value_ref(canonical(value)?))))
                                .collect::<Result<_, &'static str>>()?,
                        }
                    }
                    SourceValueOriginV1::Unknown => SourceValueOriginV1::Unknown,
                })
            })
            .collect::<Result<Box<[_]>, _>>()?;
        let reads = self.reads.into_iter().map(|(site, (original, value))| {
            let original = binding(original);
            if original.owner() != owner || value.0 >= count || !expressions.contains(&site)
                || !matches!(uses.get(&site), Some(ResolvedLexicalRefV1::Local(binding)) if *binding == original) {
                return Err("invalid source value read");
            }
            Ok((site, (original, value_ref(canonical(value)?))))
        }).collect::<Result<_, _>>()?;
        let reads: BTreeMap<_, _> = reads;
        if uses.iter().any(|(site, reference)| {
            matches!(reference, ResolvedLexicalRefV1::Local(_)) && !reads.contains_key(site)
        }) {
            return Err("source value local read missing");
        }
        Ok(VerifiedSourceValueFlowV1 {
            owner,
            complete: self.complete,
            nodes,
            reads,
        })
    }
}

/// Every recorded anchor predates its Header/After, so substitution chains
/// strictly decrease. Corrupted cycles never silently select a predecessor.
fn canonical_slot(values: &BTreeMap<SourceValueSlotV0, SourceValueSlotV0>,
    mut value: SourceValueSlotV0,
) -> Option<SourceValueSlotV0> {
    while let Some(next) = values.get(&value) {
        if next.0 >= value.0 { return None; }
        value = *next;
    }
    Some(value)
}
