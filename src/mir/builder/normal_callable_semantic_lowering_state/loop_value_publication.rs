//! Source value publication and branch/Loop projection transactions.
//!
//! Carrier header/final phis emit physical `ValueId`s that must reach the
//! binding-store `values` projection — the sole authority `read_variable`
//! resolves through.  These methods own that publication edge plus the
//! snapshot/restore pair speculative branch lanes use to keep the
//! projection transactional with name-map state.

use super::*;

impl CallableSemanticLoweringState {
    /// Publish the source point only after its actual indexed Phi definition.
    /// This adds provenance to the same value slot; ordinary publication still
    /// invalidates it and there is no second value/loop registry.
    pub(in crate::mir::builder) fn publish_source_loop_point_v2(
        &mut self, builder: &mut crate::mir::builder::MirBuilder,
        issued: &crate::mir::builder::normal_callable_loop_source_facts::SourceLoopPointCapabilityV2<'_>,
    ) -> Result<(), String> {
        issued.require_live_v2()?;
        let source = issued.source().clone();
        let relation = Rc::clone(issued.relation());
        let value = relation.values().0;
        let claims = self.ordinary_new_claim_ledger().ok_or_else(|| freeze("loop-point-claims-missing"))?;
        let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(claims);
        match &source {
            crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Scalar(source) => {
                context.source_loop_point_product_v2(source)?;
            }
            crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Text(source) => {
                context.source_loop_text_point_product_v2(source)?;
            }
            crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Dynamic(source) => {
                context.source_loop_dynamic_point_product_v2(source)?;
            }
        }
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("loop-point-function-missing"))?;
        match source {
            crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Scalar(source) => {
                let point = super::materialized_values::ScalarJoinPointValueV1::new(source, relation, function)?;
                self.publish_source_final_value(point.binding(), value)?;
                self.values.attach_scalar_join_v1(point)
            }
            crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Text(source) => {
                let point = super::materialized_values::TextJoinPointValueV1::new(source, relation, function)?;
                self.publish_source_final_value(point.binding(), value)?;
                self.values.attach_text_join_v1(point)
            }
            crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Dynamic(source) => {
                let point = super::materialized_values::DynamicJoinPointValueV1::new(source, relation, function)?;
                self.publish_source_final_value(point.binding(), value)?;
                self.values.attach_dynamic_join_v1(point)
            }
        }
    }
    /// Names locate an already selected source binding in the emission cache.
    /// They never classify a binding as a carrier or recover its identity.
    pub(in crate::mir::builder) fn source_loop_binding_name<'a>(
        &'a self,
        binding: BindingRefV1,
        physical: &BTreeMap<String, ValueId>,
    ) -> Result<&'a str, String> {
        let value = self.value_for_exact_binding(self.owner, binding)
            .map_err(|error| error.to_string())?;
        let name = self.binding_names.get(&binding)
            .ok_or_else(|| freeze("loop-carrier-binding-name-missing"))?;
        if physical.get(name.as_ref()) != Some(&value) {
            return Err(freeze("loop-carrier-binding-value-drift"));
        }
        Ok(name)
    }

    pub(in crate::mir::builder) fn publish_source_final_value(
        &mut self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<(), String> {
        if binding.owner() != self.owner {
            return Err(freeze("loop-final-binding-owner-mismatch"));
        }
        let previous = self
            .values
            .get(&binding)
            .copied()
            .ok_or_else(|| freeze("loop-final-binding-before-materialization"))?;
        self.dynamic_origins
            .invalidate_rebind(binding, previous)
            .map_err(|error| error.to_string())?;
        self.values.insert(binding, value);
        Ok(())
    }

    /// Publishes a loop-carrier physical value for the resolver-owned binding
    /// that carries `name`.  Loop producers collect carrier names from the
    /// co-sealed AST; this name index resolves the name to the single
    /// materialized binding so carrier publication and `read_variable` share
    /// `values` as the sole physical authority.  Unknown or ambiguous names
    /// freeze rather than guess a binding.
    pub(in crate::mir::builder) fn publish_source_loop_final_value_named(
        &mut self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        let mut candidates = self
            .binding_names
            .iter()
            .filter(|(binding, recorded)| {
                recorded.as_ref() == name && self.values.contains_key(*binding)
            })
            .map(|(binding, _)| *binding);
        let Some(binding) = candidates.next() else {
            return Err(freeze("loop-final-name-unknown"));
        };
        if candidates.next().is_some() {
            return Err(freeze("loop-final-name-ambiguous"));
        }
        self.publish_source_final_value(binding, value)
    }

    /// Captures the `values` projection for a speculative branch lane.
    ///
    /// Source-bound `if` lowering produces branch bodies before the
    /// condition and rolls name-map state back between them; the ledger's
    /// `values` projection must follow the same transaction or the
    /// condition would read branch-produced rebinds.  The snapshot covers
    /// `values` only: consumption receipts and `active_origins` are facts
    /// about already-emitted values and stay monotone.
    pub(in crate::mir::builder) fn source_values_snapshot(
        &self,
    ) -> crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1 {
        self.values.clone()
    }

    /// Restores the `values` projection captured by `source_values_snapshot`.
    pub(in crate::mir::builder) fn restore_source_values(
        &mut self,
        snapshot: crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1,
    ) {
        self.values = snapshot;
    }
    pub(in crate::mir::builder) fn attach_branch_scalar_phi_v1(&mut self,
        observed: Rc<crate::mir::normal_callable_semantic_package::BranchScalarPhiObservationV1>,
    ) -> Result<(), String> {
        self.values.attach_scalar_join_v1(super::materialized_values::ScalarJoinPointValueV1::from_branch_v1(observed))
    }
    pub(in crate::mir::builder) fn attach_branch_text_phi_v1(&mut self,
        observed: Rc<crate::mir::normal_callable_semantic_package::BranchTextPhiObservationV1>,
    ) -> Result<(), String> {
        self.values.attach_branch_text_v1(observed)
    }

    /// Names locate only pre-If source slots whose exact cached value agrees.
    /// Historical unmatched bindings cannot be rediscovered from the merge map.
    pub(in crate::mir::builder) fn source_if_value_slots_v1(&self,
        physical: &BTreeMap<String, ValueId>,
    ) -> Result<Vec<(BindingRefV1, String)>, String> {
        let mut names = BTreeSet::new();
        let mut rows = Vec::new();
        for (binding, name) in &self.binding_names {
            let Some(value) = self.values.get(binding) else { continue };
            if physical.get(name.as_ref()) != Some(value) { continue; }
            if !names.insert(name.as_ref()) { return Err("If source value slot is ambiguous".into()); }
            rows.push((*binding, name.to_string()));
        }
        Ok(rows)
    }

    /// Restore the source slot projection at the same physical If merge.
    /// A physical join does not itself grant scalar/source provenance.
    pub(in crate::mir::builder) fn restore_if_normal_values_v1(
        &mut self, baseline: MaterializedValuesV1,
        arms: [MaterializedValuesV1; 2], normal: [bool; 2],
        slots: &[(BindingRefV1, String)], physical: &BTreeMap<String, ValueId>,
    ) -> Result<(), String> {
        let [then_values, else_values] = arms;
        let surviving = match normal {
            [true, false] => Some(then_values.clone()),
            [false, true] => Some(else_values.clone()),
            [false, false] => { self.values = baseline; return Ok(()); }
            [true, true] => None,
        };
        let mut rows = Vec::new();
        for (binding, name) in slots {
            let value = *physical.get(name).ok_or("If merged physical slot missing")?;
            if let Some(surviving) = &surviving {
                if surviving.get(binding) != Some(&value) { return Err("If surviving source value differs".into()); }
            }
            rows.push((*binding, value));
        }
        if let Some(surviving) = surviving { self.values = surviving; return Ok(()); }
        self.values = baseline;
        for (binding, value) in rows {
            let unchanged = then_values.get(&binding) == Some(&value)
                && else_values.get(&binding) == Some(&value)
                && then_values.provenance(&binding) == else_values.provenance(&binding);
            let common = unchanged.then(|| then_values.provenance(&binding)).flatten();
            // Unrelated branches must not invalidate a still-live Dynamic origin.
            if self.values.get(&binding) != Some(&value) {
                self.publish_source_final_value(binding, value)?;
            }
            self.values.attach_provenance(binding, common);
        }
        Ok(())
    }
}
