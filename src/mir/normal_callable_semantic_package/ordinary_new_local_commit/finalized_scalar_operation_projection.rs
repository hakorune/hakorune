//! Lend original and derived scalar writers through the same finishing projection.
use super::*;
use crate::mir::normal_callable_semantic_package::OriginalScalarAppendV1;

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
                        |binding| projection.relation_binding(function, binding),
                    )?;
                    let mut destinations = std::collections::BTreeSet::new();
                    let mut lend = |original: &OriginalScalarAppendV1, binding: &Binding| {
                        let finished = projection
                            .relation_binding(function, binding)?
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
                                lend(original, original.original())?;
                            }
                            Ok(())
                        })?;
                    self.ledger
                        .visit_scalar_operation_consumers_v1(owner, |original, relation| {
                            let target = relation
                                .target_binding()
                                .ok_or_else(|| freeze("finished-scalar/derived-target-missing"))?;
                            match &target.1 {
                                MirInstruction::Copy { src, .. } if *src == relation.values().0 => {
                                    Ok(())
                                }
                                MirInstruction::BinOp {
                                    op: crate::mir::BinaryOp::Sub | crate::mir::BinaryOp::Div | crate::mir::BinaryOp::Mod,
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
