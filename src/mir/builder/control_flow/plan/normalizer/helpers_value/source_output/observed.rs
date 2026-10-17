//! Observe the existing sole writers after normalization. Actual definitions
//! corroborate the immutable source output; they never issue source meaning.
use super::*;
use crate::mir::builder::ssa::local::correspondence::LocalSsaPhysicalRelationV1;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(in crate::mir) struct ObservedSourceValueV1 {
    projection: SourceValueProjectionV1,
    relations: Vec<Rc<LocalSsaPhysicalRelationV1>>,
    bindings: Vec<Binding>,
}
impl SourceValueProjectionV1 {
    pub(in crate::mir::builder) fn observe(
        &self, builder: &mut MirBuilder, owner: FunctionOwnerIdV1,
        source: &crate::mir::builder::VerifiedNoExitIntegerExpressionV1,
    ) -> Result<ObservedSourceValueV1, String> {
        if !self.matches_source(owner, source) { return Err(reject("source-tree")); }
        let mut observed = ObservedSourceValueV1 {
            projection: self.clone(), relations: Vec::new(), bindings: Vec::new(),
        };
        observed.observe_node(builder, &self.original)?;
        Ok(observed)
    }
}
impl ObservedSourceValueV1 {
    pub(in crate::mir) fn require_completed_v1(&self) -> Result<(), String> {
        for relation in &self.relations { relation.require_completed_v1()?; }
        Ok(())
    }
    pub(in crate::mir) fn resolve_binding_v1(&self, original: &Binding, require_completed: bool)
        -> Result<Option<Binding>, String>
    {
        for relation in &self.relations {
            if let Some(binding) = relation.resolve_observed_binding_v1(original, require_completed)? {
                return Ok(Some(binding));
            }
        }
        Ok(None)
    }
    pub(in crate::mir) fn root_value(&self) -> ValueId { self.projection.root_value() }
    pub(in crate::mir) fn bindings(&self) -> &[Binding] { &self.bindings }
    pub(in crate::mir) fn validate_source(&self, owner: FunctionOwnerIdV1,
        source: &crate::mir::builder::VerifiedNoExitIntegerExpressionV1,
    ) -> Result<(), String> {
        if !self.projection.matches_source(owner, source) { return Err(reject("source-tree")); }
        Ok(())
    }
    /// Final consumers supply the SAME finishing projection used by the
    /// lifecycle census. Every captured physical relation remains immutable.
    pub(in crate::mir) fn validate(
        &self, function: &MirFunction,
        project: &mut impl FnMut(&Binding) -> Result<Option<Binding>, String>,
    ) -> Result<(), String> {
        for relation in &self.relations { relation.verify_projected(function, project)?; }
        for original in &self.bindings {
            let resolved = self.resolve_binding_v1(original, false)?.unwrap_or_else(|| original.clone());
            let Some((block, instruction)) = project(&resolved)? else {
                if matches!(original.1, MirInstruction::Const { .. } | MirInstruction::Copy { .. }) { continue; }
                return Err(reject("producer-removed"));
            };
            let body = function.blocks.get(&block).ok_or_else(|| reject("producer-block"))?;
            if body.all_instructions().filter(|row| **row == instruction).count() != 1 {
                return Err(reject("producer-drift"));
            }
        }
        Ok(())
    }
    fn loan(&mut self, builder: &mut MirBuilder, from: ValueId, to: ValueId) -> Result<Rc<LocalSsaPhysicalRelationV1>, String> {
        let state = &mut builder.function_state;
        let function = state.current_function.as_ref().ok_or_else(|| reject("no-function"))?;
        let relation = state.local_ssa_correspondence.loan(function, from, to)
            .ok_or_else(|| reject("ssa-relation-missing"))?;
        state.local_ssa_correspondence.require_member(&relation)?;
        relation.verify(function)?;
        relation.append_observed_bindings_v1(&mut self.bindings);
        if !self.relations.iter().any(|row| Rc::ptr_eq(row, &relation)) { self.relations.push(Rc::clone(&relation)); }
        Ok(relation)
    }
    fn observe_node(&mut self, builder: &mut MirBuilder, source: &NormalizedSourceValueV1) -> Result<(), String> {
        let value = self.projection.values[&source.value];
        let relation = self.loan(builder, value, value)?;
        let mut bindings = Vec::new(); relation.append_observed_bindings_v1(&mut bindings);
        match &source.kind {
            NormalizedSourceValueKindV1::Variable(_) => {}
            NormalizedSourceValueKindV1::Literal(literal) => {
                if !matches!(bindings.as_slice(), [(_, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(actual) })]
                    if *dst == value && actual == literal) { return Err(reject("literal-producer")); }
            }
            NormalizedSourceValueKindV1::Add(lhs, rhs) => {
                let [(_, MirInstruction::BinOp { dst, lhs: actual_lhs, op: crate::mir::BinaryOp::Add, rhs: actual_rhs })] = bindings.as_slice()
                    else { return Err(reject("add-producer")); };
                if *dst != value { return Err(reject("add-destination")); }
                self.observe_node(builder, lhs)?;
                self.observe_node(builder, rhs)?;
                self.loan(builder, self.projection.values[&lhs.value], *actual_lhs)?;
                self.loan(builder, self.projection.values[&rhs.value], *actual_rhs)?;
            }
        }
        Ok(())
    }
}
fn reject(reason: &str) -> String { format!("[freeze:contract][source-value-observation/{reason}]") }
