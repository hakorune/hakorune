//! Lend original and derived scalar writers through the same finishing projection.
use super::*;
use crate::mir::normal_callable_semantic_package::OriginalScalarAppendV1;

impl OrdinaryNewClaimLedgerV1 {
    /// Select exact optional scalar membership before physical projection.
    pub(in crate::mir) fn project_scalar_operation_binding_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        projection: &super::physical_boundary::FinishedBindings,
        binding: &Binding,
    ) -> Result<Option<Binding>, String> {
        let mut selected = None;
        self.visit_scalar_operation_originals_v1(|original| {
            if original.source().owner() != owner || self.scalar_operation_original_required_v1(original) {
                return Ok(());
            }
            if original.original() == binding {
                selected = Some((Rc::clone(original), None));
            } else if selected.is_none() {
                for relation in original.operand_relations() {
                    let mut observed = Vec::new();
                    relation.append_observed_bindings_v1(&mut observed);
                    if observed.contains(binding) {
                        selected = Some((Rc::clone(original), Some(Rc::clone(relation))));
                        break;
                    }
                }
            }
            Ok(())
        })?;
        if selected.is_none() {
            self.visit_scalar_operation_consumers_v1(owner, |original, relation| {
                if selected.is_none() && !self.scalar_operation_original_required_v1(original) {
                    let mut observed = Vec::new();
                    relation.append_observed_bindings_v1(&mut observed);
                    if observed.contains(binding) {
                        selected = Some((Rc::clone(original), Some(Rc::clone(relation))));
                    }
                }
                Ok(())
            })?;
        }
        match selected {
            Some((original, relation)) => projection.scalar_candidate_binding_v1(
                function, &original, relation.as_deref(), binding),
            None => projection.relation_binding(function, binding),
        }
    }
}

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn visit_finished_scalar_operations_v1(
        &self,
        module: &crate::mir::MirModule,
        mut visit: impl FnMut(&MirFunction, &Binding, &OriginalScalarAppendV1) -> Result<(), String>,
    ) -> Result<(), String> {
        let mut owners = std::collections::BTreeSet::new();
        self.ledger
            .visit_scalar_operation_originals_v1(|original| {
                owners.insert(original.source().owner());
                Ok(())
            })?;
        for owner in owners {
            self.ledger
                .with_finished_projection(owner, |symbol, projection| {
                    let function = module
                        .functions
                        .get(symbol)
                        .ok_or_else(|| freeze("finished-scalar/function-missing"))?;
                    self.ledger.verify_finished_scalar_operations_v1(
                        owner,
                        function,
                        |binding| self.ledger.project_scalar_operation_binding_v1(owner, function, projection, binding),
                    )?;
                    let mut destinations = std::collections::BTreeSet::new();
                    let mut lend = |original: &OriginalScalarAppendV1, binding: &Binding| {
                        let finished = self.ledger
                            .project_scalar_operation_binding_v1(owner, function, projection, binding)?
                            .ok_or_else(|| freeze("finished-scalar/producer-removed"))?;
                        let dst = finished
                            .1
                            .dst_value()
                            .ok_or_else(|| freeze("finished-scalar/destination-missing"))?;
                        if !destinations.insert(dst) {
                            return Err(freeze("finished-scalar/duplicate-producer"));
                        }
                        visit(function, &finished, original)
                    };
                    self.ledger
                        .visit_scalar_operation_originals_v1(|original| {
                            if original.source().owner() == owner {
                                if !self.ledger.scalar_operation_original_required_v1(original)
                                    && self.ledger.project_scalar_operation_binding_v1(owner, function, projection, original.original())?.is_none() {
                                    return Ok(());
                                }
                                lend(original, original.original())?;
                            }
                            Ok(())
                        })?;
                    self.ledger
                        .visit_scalar_operation_consumers_v1(owner, |original, relation| {
                            if !self.ledger.scalar_operation_original_required_v1(original)
                                && self.ledger.project_scalar_operation_binding_v1(owner, function, projection, original.original())?.is_none() {
                                return Ok(());
                            }
                            let target = relation
                                .target_binding()
                                .ok_or_else(|| freeze("finished-scalar/derived-target-missing"))?;
                            match &target.1 {
                                MirInstruction::Copy { src, .. } if *src == relation.values().0 => {
                                    Ok(())
                                }
                                MirInstruction::BinOp {
                                    op: crate::mir::BinaryOp::Add | crate::mir::BinaryOp::Sub | crate::mir::BinaryOp::Div | crate::mir::BinaryOp::Mod,
                                    ..
                                } => lend(original, target),
                                _ => Err(freeze("finished-scalar/derived-operation")),
                            }
                        })
                })?;
        }
        Ok(())
    }
}
