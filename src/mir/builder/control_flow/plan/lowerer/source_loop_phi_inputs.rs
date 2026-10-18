//! Original source edges to checked Phi payloads, without Loop completion.
//! Only this graph verifier constructs the session's patch input capability.
use super::source_loop_driver::EmittedSourceLoopGraphV2;
use crate::mir::builder::{BindingValueObservationV2, MirBuilder};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2 as Product;
use crate::mir::resolved_semantics::{BindingRefV1, home_new_prefix::SourceLoopPortKindV1 as Port};
use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
use crate::mir::{BasicBlockId, ValueId, MirInstruction};
use std::{rc::Rc, collections::BTreeMap};

#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedSourceLoopPhiInputsV2 {
    product: Rc<Product>,
    header: BasicBlockId,
    after: BasicBlockId,
    entries: BTreeMap<BindingRefV1, BindingValueObservationV2>,
    incoming: BTreeMap<(BasicBlockId, BindingRefV1), Vec<(BasicBlockId, ValueId)>>,
}
impl VerifiedSourceLoopPhiInputsV2 {
    pub(in crate::mir::builder) fn product(&self) -> &Rc<Product> { &self.product }
    pub(in crate::mir::builder) fn scope(&self) -> (BasicBlockId, BasicBlockId) { (self.header, self.after) }
    pub(in crate::mir::builder) fn entry(&self, binding: BindingRefV1) -> Option<&BindingValueObservationV2> { self.entries.get(&binding) }
    pub(in crate::mir::builder) fn incoming(&self, block: BasicBlockId, binding: BindingRefV1)
        -> Result<&[(BasicBlockId, ValueId)], String> {
        self.incoming.get(&(block, binding)).map(Vec::as_slice).ok_or_else(|| reject("destination"))
    }
}
impl EmittedSourceLoopGraphV2 {
    pub(super) fn verify_phi_inputs_v2(&self, builder: &MirBuilder, state: &CallableSemanticLoweringState)
        -> Result<VerifiedSourceLoopPhiInputsV2, String> {
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function"))?;
        let claims = state.ordinary_new_claim_ledger().ok_or_else(|| reject("claims"))?;
        let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(claims);
        let basis = self.product.source.headers.values().next().ok_or_else(|| reject("empty-carriers"))?;
        if !Rc::ptr_eq(&context.source_loop_point_product_v2(
            &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::LoopHeader(Rc::clone(basis)))?, &self.product) {
            return Err(reject("original-product"));
        }
        if state.owner() != self.product.source.source.owner() || self.header == self.after {
            return Err(reject("source-scope"));
        }
        let original = self.product.source.incoming.as_ref().map_err(|reason| reject(reason))?;
        let first = original.first().ok_or_else(|| reject("empty-carriers"))?;
        let census: Vec<_> = first.header_incoming.iter().chain(first.after_incoming.iter()).map(|row| &row.edge).collect();
        if self.edges.len() != census.len() || census.iter().enumerate().any(|(index, edge)|
            census[..index].contains(edge) || self.edges.iter().filter(|row| &row.source == *edge).count() != 1) {
            return Err(reject("source-edge-census"));
        }
        for edge in &self.edges {
            let header = matches!(edge.source, Edge::LoopEntry | Edge::Backedge | Edge::Continue(_));
            if edge.target != if header { self.header } else { self.after }
                || function.blocks.get(&edge.predecessor).and_then(|block| block.terminator.as_ref())
                    != Some(&MirInstruction::Jump { target: edge.target, edge_args: None })
                || edge.values.len() != original.len() {
                return Err(reject("actual-edge"));
            }
            match (&edge.source, &edge.cleanup) {
                (Edge::LoopEntry | Edge::ConditionFalse, None) => {},
                (Edge::Break(site) | Edge::Continue(site), Some(cleanup)) => {
                    let source = self.product.source.ports.get(cleanup.plan.port()).ok_or_else(|| reject("cleanup-port"))?;
                    let same = match (&edge.source, &source.kind) {
                        (Edge::Break(_), Port::Break(original)) | (Edge::Continue(_), Port::Continue(original)) => original == site,
                        _ => false,
                    };
                    if !same || !Rc::ptr_eq(cleanup.plan.product(), &self.product) || cleanup.continuation != edge.predecessor {
                        return Err(reject("cleanup-source"));
                    }
                }
                (Edge::Backedge, Some(cleanup)) if Rc::ptr_eq(cleanup.plan.product(), &self.product)
                    && cleanup.continuation == edge.predecessor
                    && self.product.source.ports.get(cleanup.plan.port()).is_some_and(|port| matches!(port.kind, Port::Backedge)) => {},
                _ => return Err(reject("cleanup-role")),
            }
        }
        let mut entries = BTreeMap::new();
        let mut incoming = BTreeMap::new();
        for source in original.iter() {
            let binding = source.basis.binding();
            for (block, source_rows) in [(self.header, source.header_incoming.as_ref()), (self.after, source.after_incoming.as_ref())] {
                let mut payloads = Vec::new();
                for expected in source_rows {
                    let mut edges = self.edges.iter().filter(|edge| edge.source == expected.edge && edge.target == block);
                    let edge = edges.next().ok_or_else(|| reject("incoming-edge"))?;
                    if edges.next().is_some() { return Err(reject("incoming-edge-duplicate")); }
                    let mut values = edge.values.iter().filter(|row| row.binding == binding);
                    let value = values.next().ok_or_else(|| reject("incoming-binding"))?;
                    if values.next().is_some() { return Err(reject("incoming-binding-duplicate")); }
                    self.product.validate_incoming_value_v2(&edge.source, binding, value.logical)?;
                    crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::validate_scalar_binding_v2(
                        binding, &expected.producer, &value.observation, context, function)?;
                    value.localized.verify(function)?;
                    if value.localized.values().0 != value.observation.binding_value().1 { return Err(reject("localized-origin")); }
                    match (&edge.source, &value.canonical) {
                        (Edge::LoopEntry, None) => {
                            if entries.insert(binding, value.observation.clone()).is_some() { return Err(reject("entry-duplicate")); }
                        }
                        (Edge::LoopEntry, Some(_)) | (_, None) => return Err(reject("entry-header-phase")),
                        (_, Some(canonical)) => {
                            canonical.verify(function)?;
                            if canonical.values().1 != value.localized.values().1 { return Err(reject("canonical-localized")); }
                        }
                    }
                    payloads.push((edge.predecessor, value.localized.values().1));
                }
                payloads.sort_by_key(|row| row.0);
                if payloads.is_empty() || payloads.windows(2).any(|pair| pair[0].0 == pair[1].0)
                    || incoming.insert((block, binding), payloads).is_some() { return Err(reject("predecessor-census")); }
            }
        }
        Ok(VerifiedSourceLoopPhiInputsV2 { product: Rc::clone(&self.product), header: self.header, after: self.after, entries, incoming })
    }
}
fn reject(reason: &str) -> String { format!("[freeze:contract][source-loop/phi-inputs/{reason}]") }
