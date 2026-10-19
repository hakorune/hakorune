//! One physical input owns early Header/After definitions through final patch.
//! The source session lends the SAME PhiTxn; it issues no semantic permission.
use crate::mir::builder::control_flow::plan::CoreLoopPlan;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::cell::RefCell;
use std::collections::{BTreeSet, BTreeMap};
use std::rc::Rc;
use crate::mir::builder::{BindingValueObservationV2, LocalSsaPhysicalRelationV1};
use crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2 as Product;
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1 as Producer;

/// Issued only from this session's canonical source-to-destination association.
/// Publication cannot substitute another binding, role or indexed Phi.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SourceLoopPointCapabilityV2<'session> {
    source: Producer,
    relation: Rc<LocalSsaPhysicalRelationV1>,
    owner: PointOwnerV2<'session>,
}
#[derive(Debug, Clone, Copy)]
enum PointOwnerV2<'a> { Session(&'a SourceLoopPhiSessionV1), Completed(&'a CompletedSourceLoopPhiPointsV2) }
impl SourceLoopPointCapabilityV2<'_> {
    pub(in crate::mir::builder) fn source(&self) -> &Producer { &self.source }
    pub(in crate::mir::builder) fn relation(&self) -> &Rc<LocalSsaPhysicalRelationV1> { &self.relation }
    pub(in crate::mir::builder) fn require_live_v2(&self) -> Result<(), String> {
        let session = match self.owner {
            PointOwnerV2::Session(session) => session,
            PointOwnerV2::Completed(completed) => return completed.require_after_association_v2(&self.source, &self.relation),
        };
        let state = session.state.borrow();
        if state.finished || state.transaction.is_none() { return Err(freeze("checked-session-inactive")); }
        let Producer::LoopHeader(basis) = &self.source else { return Err(freeze("checked-point-role")); };
        let row = state.checked.as_ref().and_then(|checked| checked.carriers.get(&basis.binding()))
            .ok_or_else(|| freeze("checked-point-binding"))?;
        if row.source != self.source || !Rc::ptr_eq(&row.header, &self.relation) {
            return Err(freeze("checked-point-association"));
        }
        Ok(())
    }
}
#[derive(Debug)]
struct CheckedCarrierV2 {
    entry: BindingValueObservationV2,
    entry_dependencies: Vec<(BasicBlockId, crate::mir::MirInstruction)>,
    source: Producer,
    header: Rc<LocalSsaPhysicalRelationV1>,
    after: Rc<LocalSsaPhysicalRelationV1>,
}
#[derive(Debug)]
struct CheckedPointsV2 {
    product: Rc<Product>,
    carriers: BTreeMap<BindingRefV1, CheckedCarrierV2>,
}

/// Stack-local patch evidence borrows the SAME session. It owns no copy of the
/// source associations and must wait for whole-graph verification before commit.
#[derive(Debug)]
pub(in crate::mir) struct PreparedSourceLoopPhiCompletionV2<'session> {
    session: &'session SourceLoopPhiSessionV1,
    scope: (BasicBlockId, BasicBlockId),
    rows: crate::mir::builder::control_flow::plan::lowerer::LoopPhiInputRowsV2,
    phis: BTreeMap<BasicBlockId, Box<[crate::mir::MirInstruction]>>,
}
impl PreparedSourceLoopPhiCompletionV2<'_> {
    pub(in crate::mir) fn require_source_v2(&self, product: &Rc<Product>, header: BasicBlockId, after: BasicBlockId) -> Result<(), String> {
        if self.scope != (header, after) || self.session.state.borrow().checked.as_ref().is_none_or(|row| !Rc::ptr_eq(&row.product, product)) {
            return Err(freeze("prepared-source-association"));
        }
        Ok(())
    }
    pub(in crate::mir) fn phis(&self) -> &BTreeMap<BasicBlockId, Box<[crate::mir::MirInstruction]>> { &self.phis }
    pub(in crate::mir) fn commit(self, builder: &mut MirBuilder) -> Result<CompletedSourceLoopPhiPointsV2, String> {
        let result = (|| {
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("function-missing"))?;
            if crate::mir::builder::control_flow::plan::lowerer::capture_exact_loop_phis_v2(function, &self.rows)? != self.phis {
                return Err(freeze("checked-finish-phi-drift"));
            }
            self.session.commit(builder)?;
            let points = self.session.state.borrow_mut().checked.take().ok_or_else(|| freeze("checked-points-missing"))?;
            Ok(CompletedSourceLoopPhiPointsV2 { points, scope: self.scope, phis: self.phis })
        })();
        result.map_err(|error| self.session.abort_error(builder, error))
    }
}

/// Moved out of the SAME session after exact incoming patch and Phi closure.
/// This is Phi completion only; parent Loop/cleanup acceptance remains required.
#[derive(Debug)]
pub(in crate::mir) struct CompletedSourceLoopPhiPointsV2 {
    points: CheckedPointsV2,
    scope: (BasicBlockId, BasicBlockId),
    phis: BTreeMap<BasicBlockId, Box<[crate::mir::MirInstruction]>>,
}
impl CompletedSourceLoopPhiPointsV2 {
    pub(in crate::mir) fn product(&self) -> &Rc<Product> { &self.points.product }
    pub(in crate::mir) fn phis(&self) -> &BTreeMap<BasicBlockId, Box<[crate::mir::MirInstruction]>> { &self.phis }
    pub(in crate::mir::builder) fn after_point_v2(&self, binding: BindingRefV1)
        -> Result<SourceLoopPointCapabilityV2<'_>, String> {
        let row = self.points.carriers.get(&binding).ok_or_else(|| freeze("completed-binding"))?;
        let issued = SourceLoopPointCapabilityV2 { source: Producer::LoopAfter { product: Rc::clone(&self.points.product), binding },
            relation: Rc::clone(&row.after), owner: PointOwnerV2::Completed(self) };
        issued.require_live_v2()?;
        Ok(issued)
    }
    fn require_after_association_v2(&self, source: &Producer, relation: &Rc<LocalSsaPhysicalRelationV1>) -> Result<(), String> {
        let Producer::LoopAfter { product, binding } = source else { return Err(freeze("completed-role")); };
        let row = self.points.carriers.get(binding).ok_or_else(|| freeze("completed-binding"))?;
        relation.require_completed_v1()?;
        if !Rc::ptr_eq(product, &self.points.product) || !Rc::ptr_eq(relation, &row.after)
            || relation.identity_phi_binding_v1()?.0 != self.scope.1 { return Err(freeze("completed-association")); }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct State {
    scope: Option<(BasicBlockId, BasicBlockId)>,
    adopted: bool,
    finished: bool,
    transaction: Option<PhiTxn>,
    checked: Option<CheckedPointsV2>,
}
#[derive(Debug, Default)]
pub(in crate::mir::builder) struct SourceLoopPhiSessionV1 {
    state: RefCell<State>,
}
impl SourceLoopPhiSessionV1 {
    /// Verify every original Entry before allocation; this same physical owner
    /// then allocates and defines all Header/After points. No caller-supplied
    /// source/value pairing or destination list can enter the checked path.
    pub(in crate::mir::builder) fn prepare_checked(&self, builder: &mut MirBuilder, product: Rc<Product>,
        ledger: &crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
        header: BasicBlockId, after: BasicBlockId,
    ) -> Result<(), String> {
        if header == after || self.state.borrow().scope.is_some() { return Err(freeze("checked-scope-or-repeat")); }
        let claims = ledger.ordinary_new_claim_ledger().ok_or_else(|| freeze("checked-claims-missing"))?;
        let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(claims);
        let expected: BTreeSet<_> = product.source.source.bindings().bindings().filter_map(|(binding, role)|
            (role == crate::mir::builder::CallableLoopSourceBindingClassV1::Carrier).then_some(binding)).collect();
        if ledger.owner() != product.source.source.owner() || expected.is_empty()
            || expected != product.source.headers.keys().copied().collect() {
            return Err(freeze("checked-source-carrier-census"));
        }
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("function-missing"))?;
        let mut entries = Vec::new();
        for (&binding, basis) in &product.source.headers {
            let source = Producer::LoopHeader(Rc::clone(basis));
            if !Rc::ptr_eq(&context.source_loop_point_product_v2(&source)?, &product) {
                return Err(freeze("checked-source-product"));
            }
            let entry = ledger.observe_binding_value_v2(binding).map_err(|error| error.to_string())?;
            ledger.source_loop_binding_name(binding, &builder.function_state.variable_ctx.variable_map)?;
            let dependencies = crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::validate_scalar_binding_v2(
                binding, basis.entry_producer(), &entry, context, function)?;
            entries.push((binding, source, entry, dependencies));
        }
        let destinations: Vec<_> = entries.iter().map(|_| (builder.alloc_typed(crate::mir::MirType::Integer),
            builder.alloc_typed(crate::mir::MirType::Integer))).collect();
        self.prepare(builder, header, after, destinations.iter().map(|row| row.0), destinations.iter().map(|row| row.1))?;
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("function-missing"))?;
        let mut carriers = BTreeMap::new();
        for ((binding, source, entry, entry_dependencies), (head, tail)) in entries.into_iter().zip(destinations) {
            let mut loan = |block, value| -> Result<_, String> {
                let relation = builder.function_state.local_ssa_correspondence.loan(function, value, value)
                    .ok_or_else(|| freeze("checked-phi-index"))?;
                let row = relation.identity_phi_binding_v1()?;
                if row.0 != block || row.1.dst_value() != Some(value) { return Err(freeze("checked-phi-destination")); }
                Ok(relation)
            };
            carriers.insert(binding, CheckedCarrierV2 { entry, entry_dependencies,
                source, header: loan(header, head)?, after: loan(after, tail)? });
        }
        let mut state = self.state.borrow_mut();
        state.checked = Some(CheckedPointsV2 { product, carriers });
        // All checked destinations were defined above. This path has no legacy
        // CoreLoopPlan adoption or Step destinations to reconstruct.
        state.adopted = true;
        Ok(())
    }
    pub(super) fn header_point(&self, product: &Rc<Product>, binding: BindingRefV1)
        -> Result<SourceLoopPointCapabilityV2<'_>, String> {
        let state = self.state.borrow();
        if state.finished || state.transaction.is_none() { return Err(freeze("checked-session-inactive")); }
        let checked = state.checked.as_ref().ok_or_else(|| freeze("checked-points-missing"))?;
        if !Rc::ptr_eq(&checked.product, product) { return Err(freeze("checked-point-product")); }
        let row = checked.carriers.get(&binding).ok_or_else(|| freeze("checked-point-binding"))?;
        Ok(SourceLoopPointCapabilityV2 { source: row.source.clone(), relation: Rc::clone(&row.header), owner: PointOwnerV2::Session(self) })
    }
    /// Definitions precede both source-carrier publication points. A failed
    /// partial definition remains in this same txn for the enclosing abort.
    pub(in crate::mir::builder) fn prepare(
        &self, builder: &mut MirBuilder, header: BasicBlockId, after: BasicBlockId,
        headers: impl Iterator<Item = ValueId>, afters: impl Iterator<Item = ValueId>,
    ) -> Result<(), String> {
        let headers: Vec<_> = headers.collect();
        let afters: Vec<_> = afters.collect();
        let rows: Vec<_> = headers.iter().map(|value| (header, *value))
            .chain(afters.iter().map(|value| (after, *value))).collect();
        if header == after || headers.is_empty() || headers.len() != afters.len()
            || rows.iter().map(|(_, value)| *value).collect::<BTreeSet<_>>().len() != rows.len() {
            return Err(freeze("preparation-destinations"));
        }
        let mut state = self.state.borrow_mut();
        if state.scope.is_some() { return Err(freeze("preparation-repeat")); }
        state.scope = Some((header, after));
        state.transaction = Some(PhiTxn::begin("source-loop-header-after"));
        let txn = state.transaction.as_mut().expect("installed above");
        for (block, value) in rows {
            builder.ensure_block_exists(block)?;
            txn.define_provisional_phi(builder, block, value, "source-loop:before-expose")?;
        }
        Ok(())
    }

    /// Patch the SAME txn from exact source-edge verification, retaining its
    /// ownership until the whole-graph caller completes all fallible checks.
    pub(in crate::mir::builder) fn prepare_checked_completion_v2(&self, builder: &mut MirBuilder,
        inputs: &crate::mir::builder::control_flow::plan::lowerer::VerifiedSourceLoopPhiInputsV2,
    ) -> Result<PreparedSourceLoopPhiCompletionV2<'_>, String> {
        let result = (|| {
            let mut rows: crate::mir::builder::control_flow::plan::lowerer::LoopPhiInputRowsV2 = Vec::new();
            {
                let state = self.state.borrow();
                if state.finished || !state.adopted || state.scope != Some(inputs.scope()) { return Err(freeze("checked-finish-scope")); }
                let points = state.checked.as_ref().ok_or_else(|| freeze("checked-points-missing"))?;
                if !Rc::ptr_eq(&points.product, inputs.product()) { return Err(freeze("checked-finish-product")); }
                let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("function-missing"))?;
                for (&binding, row) in &points.carriers {
                    if inputs.entry(binding) != Some(&row.entry) { return Err(freeze("checked-finish-entry")); }
                    for relation in [&row.header, &row.after] {
                        relation.verify(function)?;
                        let (block, instruction) = relation.identity_phi_binding_v1()?;
                        let crate::mir::MirInstruction::Phi { dst, inputs: pending, .. } = instruction else { return Err(freeze("checked-finish-definition")); };
                        if !pending.is_empty() { return Err(freeze("checked-finish-already-patched")); }
                        rows.push((*block, *dst, inputs.incoming(*block, binding)?.to_vec()));
                    }
                }
            }
            self.with_transaction(|txn| {
                let tokens: Vec<_> = txn.pending_tokens_v1().collect();
                if tokens.len() != rows.len() || rows.iter().any(|(block, value, _)|
                    tokens.iter().filter(|(id, token)| id == block && token.dst() == *value).count() != 1) {
                    return Err(freeze("checked-finish-token-census"));
                }
                for (block, value, incoming) in &rows {
                    let token = tokens.iter().find(|(id, token)| id == block && token.dst() == *value).expect("exact census above").1;
                    txn.patch_phi_inputs(builder, token, incoming.clone(), "source-loop:checked-completion")?;
                }
                Ok(())
            })?;
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("function-missing"))?;
            let phis = crate::mir::builder::control_flow::plan::lowerer::capture_exact_loop_phis_v2(function, &rows)?;
            Ok(PreparedSourceLoopPhiCompletionV2 { session: self, scope: inputs.scope(), rows, phis })
        })();
        result.map_err(|error| self.abort_error(builder, error))
    }

    /// Adopt every original Header/After token exactly once. Only remaining
    /// Step destinations are defined here, through the same transaction.
    pub(in crate::mir::builder) fn adopt_and_define(
        &self, builder: &mut MirBuilder, plan: &CoreLoopPlan,
    ) -> Result<Vec<(ValueId, BasicBlockId, String)>, String> {
        let mut state = self.state.borrow_mut();
        if state.scope != Some((plan.header_bb, plan.after_bb)) || state.adopted || state.finished {
            return Err(freeze("adoption-scope-or-repeat"));
        }
        let txn = state.transaction.as_mut().ok_or_else(|| freeze("transaction-missing"))?;
        let pending: BTreeSet<_> = txn.pending_tokens_v1().map(|(block, token)| (block, token.dst())).collect();
        let all: BTreeSet<_> = plan.phis.iter().map(|phi| (phi.block, phi.dst)).collect();
        let expected: BTreeSet<_> = all.iter().copied()
            .filter(|(block, _)| *block == plan.header_bb || *block == plan.after_bb).collect();
        if pending != expected || all.len() != plan.phis.len()
            || plan.phis.iter().map(|phi| phi.dst).collect::<BTreeSet<_>>().len() != plan.phis.len()
            || all.iter().any(|(block, _)| *block != plan.header_bb && *block != plan.after_bb && *block != plan.step_bb) {
            return Err(freeze("adoption-destination-census"));
        }
        // The source phase cannot patch or replace these provisional definitions.
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("function-missing"))?;
        for &(block, value) in &pending {
            let definitions: Vec<_> = function.blocks.iter().flat_map(|(id, body)| body.all_instructions()
                .filter(move |row| row.dst_value() == Some(value)).map(move |row| (*id, row))).collect();
            if !matches!(definitions.as_slice(), [(id, crate::mir::MirInstruction::Phi { dst, inputs, .. })]
                if *id == block && *dst == value && inputs.is_empty()) || function.params.contains(&value) {
                return Err(freeze("adoption-definition-drift"));
            }
        }
        for phi in &plan.phis {
            if !pending.contains(&(phi.block, phi.dst)) {
                builder.ensure_block_exists(phi.block)?;
                txn.define_provisional_phi(builder, phi.block, phi.dst, "source-loop:step-adoption")?;
            }
        }
        state.adopted = true;
        Ok(plan.phis.iter().map(|phi| (phi.dst, phi.block, phi.tag.clone())).collect())
    }

    /// Only linear patch operations borrow the txn; recursive source lowering
    /// never carries this RefMut across an expression or child callback.
    pub(in crate::mir::builder) fn with_transaction<R>(
        &self, action: impl FnOnce(&mut PhiTxn) -> Result<R, String>,
    ) -> Result<R, String> {
        let mut state = self.state.borrow_mut();
        if !state.adopted || state.finished { return Err(freeze("transaction-not-adopted")); }
        action(state.transaction.as_mut().ok_or_else(|| freeze("transaction-missing"))?)
    }
    pub(in crate::mir::builder) fn require_patched(&self) -> Result<(), String> {
        self.with_transaction(|txn| {
            if txn.pending_tokens_v1().next().is_some() { return Err(freeze("pending-after-patch")); }
            Ok(())
        })
    }
    pub(in crate::mir::builder) fn commit(&self, builder: &mut MirBuilder) -> Result<(), String> {
        self.require_patched()?;
        let mut state = self.state.borrow_mut();
        let txn = state.transaction.take().ok_or_else(|| freeze("transaction-missing"))?;
        txn.commit(builder).map_err(|error| error.to_string())?;
        state.finished = true;
        Ok(())
    }
    /// All enclosing error boundaries call the same owner. Once a txn has
    /// aborted, preserve the returned primary+cleanup error without wrapping it.
    pub(in crate::mir::builder) fn abort_error(&self, builder: &mut MirBuilder, error: String) -> String {
        let transaction = self.state.borrow_mut().transaction.take();
        match transaction { Some(txn) => txn.abort_on_err(builder, error).to_string(), None => error }
    }
}
fn freeze(reason: &str) -> String { format!("[freeze:contract][source-loop-phi/{reason}]") }

#[cfg(test)]
impl SourceLoopPhiSessionV1 {
    pub(in crate::mir::builder) fn assert_checked_points_for_test(builder: &mut MirBuilder,
        ledger: &mut crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
        product: &Rc<Product>,
    ) {
        let header = builder.next_block_id();
        let after = builder.next_block_id();
        let binding = *product.source.headers.keys().next().unwrap();
        let entry = ledger.value_for_exact_binding(ledger.owner(), binding).unwrap();
        let entry_block = builder.function_state.current_block.unwrap();
        let original = builder.function_state.current_function.as_ref().unwrap().blocks[&entry_block].instructions.clone();
        let rows = &mut builder.function_state.current_function.as_mut().unwrap().blocks.get_mut(&entry_block).unwrap().instructions;
        let row = rows.iter_mut().find(|row| row.dst_value() == Some(entry)).unwrap();
        let crate::mir::MirInstruction::Const { value: crate::mir::ConstValue::Integer(value), .. } = row else { panic!("literal Entry"); };
        *value += 1;
        let rejected = Self::default();
        assert!(rejected.prepare_checked(builder, Rc::clone(product), ledger, header, after).unwrap_err().contains("scalar-literal-producer"));
        assert!(rejected.state.borrow().scope.is_none(), "all Entry checks precede Phi allocation/definition");
        builder.function_state.current_function.as_mut().unwrap().blocks.get_mut(&entry_block).unwrap().instructions = original;
        let session = Self::default();
        session.prepare_checked(builder, Rc::clone(product), ledger, header, after).unwrap();
        assert!(session.prepare_checked(builder, Rc::clone(product), ledger, header, after).is_err());
        let issued = session.header_point(product, binding).unwrap();
        issued.require_live_v2().unwrap();
        assert_eq!(issued.relation().identity_phi_binding_v1().unwrap().0, header);
        let state = session.state.borrow();
        let checked = state.checked.as_ref().unwrap();
        let row = &checked.carriers[&binding];
        assert_eq!(row.entry.binding_value().1, entry);
        assert!(!row.entry_dependencies.is_empty());
        assert_eq!(row.after.identity_phi_binding_v1().unwrap().0, after);
        let mut swapped = issued.clone();
        swapped.relation = Rc::clone(&row.after);
        assert!(swapped.require_live_v2().is_err(), "Header cannot borrow After's real Phi");
        swapped = issued.clone();
        swapped.source = Producer::LoopAfter { product: Rc::clone(product), binding };
        assert!(swapped.require_live_v2().is_err(), "a pending session cannot issue After");
        drop(state);
        for (other, role) in product.source.source.bindings().bindings() {
            if role != crate::mir::builder::CallableLoopSourceBindingClassV1::Carrier {
                assert!(session.header_point(product, other).is_err(), "readonly/iteration locals cannot become carriers");
            }
        }
        ledger.publish_source_loop_point_v2(builder, &issued).unwrap();
        let observed = ledger.observe_binding_value_v2(binding).unwrap();
        assert!(Rc::ptr_eq(observed.loan_scalar_join_v1(issued.source()).unwrap().relation(), issued.relation()));
        let error = session.abort_error(builder, "expected source session abort".into());
        assert!(error.contains("expected source session abort"));
        assert!(issued.require_live_v2().is_err());
        assert!(issued.relation().identity_phi_binding_v1().is_err());
        assert!(ledger.publish_source_loop_point_v2(builder, &issued).is_err(), "aborted capability cannot publish");
    }
}
