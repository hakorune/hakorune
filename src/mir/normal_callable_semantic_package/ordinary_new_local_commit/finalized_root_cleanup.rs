//! Same-owner final cleanup corroboration, retaining original source order.
use super::*;
use crate::mir::MirModule;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn validate_finalized_root_cleanup_v1(
        &self,
        module: &MirModule,
    ) -> Result<(), String> {
        let exits = self.ledger.root_exits.borrow();
        for exit in self.call_entries.keys() {
            if !matches!(
                exits.get(&(self.owner, exit.clone())),
                Some(RootHomeExitProgress::Finalized(_))
            ) {
                return Err(freeze("final-cleanup/order-missing"));
            }
        }
        for ((owner, exit), progress) in exits.iter() {
            match progress {
                RootHomeExitProgress::Finalized(order) => {
                    if *owner != self.owner {
                        return Err(freeze("final-cleanup/foreign-owner"));
                    }
                    let (entry, bindings) = self
                        .call_entries
                        .get(exit)
                        .ok_or_else(|| freeze("final-cleanup/entry-missing"))?;
                    self.ledger
                        .with_finished_projection(*owner, |symbol, projection| {
                            let function = module
                                .functions
                                .get(symbol)
                                .filter(|f| f.signature.name == symbol)
                                .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
                            let RootHomeExitEntry::Call {
                                invoke,
                                projection: result,
                                frame,
                                ..
                            } = entry
                            else {
                                return Err(freeze("final-cleanup/entry-kind"));
                            };
                            for binding in bindings.iter().chain([invoke, result, frame]) {
                                if !projection.recorded().contains(binding)
                                    || !physical_boundary::check_binding(
                                        function, None, binding.0, &binding.1,
                                    )?
                                {
                                    return Err(freeze("final-cleanup/binding-unrecorded"));
                                }
                            }
                            order.validate_direct_entry(entry, bindings, None)?;
                            root_cleanup_graph::ordered_paths::validate(
                                function, bindings, entry, order, None,
                            )?;
                            root_cleanup_graph::ordered_structure::validate_finished_call(
                                function,
                                bindings,
                                entry,
                                order.ingress_result_kind(),
                            )
                        })?;
                }
                RootHomeExitProgress::Emitted {
                    order,
                    bindings,
                    entry,
                    ..
                } => {
                    if *owner != self.owner && !matches!(entry, RootHomeExitEntry::Call { .. }) {
                        continue;
                    }
                    self.ledger
                        .with_finished_projection(*owner, |symbol, projection| {
                            let function = module
                                .functions
                                .get(symbol)
                                .filter(|f| f.signature.name == symbol)
                                .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
                            let mut mandatory = projection.bindings(bindings)?;
                            match entry {
                                RootHomeExitEntry::Call {
                                    invoke,
                                    projection: result,
                                    frame,
                                    ..
                                }
                                | RootHomeExitEntry::MapGet {
                                    invoke,
                                    projection: result,
                                    frame,
                                    ..
                                } => {
                                    for original in [invoke, result, frame] {
                                        mandatory.push(
                                            projection
                                                .binding(original.0, &original.1)?
                                                .ok_or_else(|| {
                                                    freeze("final-cleanup/entry-binding-missing")
                                                })?,
                                        );
                                    }
                                }
                                RootHomeExitEntry::Plain { .. } => {}
                            }
                            for binding in mandatory {
                                if !projection.recorded().contains(&binding)
                                    || !physical_boundary::check_binding(
                                        function, None, binding.0, &binding.1,
                                    )?
                                {
                                    return Err(freeze("final-cleanup/binding-unrecorded"));
                                }
                            }
                            order.validate_direct_entry(entry, bindings, Some(projection))?;
                            root_cleanup_graph::ordered_paths::validate(
                                function,
                                bindings,
                                entry,
                                order,
                                Some(projection),
                            )?;
                            root_cleanup_graph::ordered_structure::validate_projected(
                                function,
                                bindings,
                                entry,
                                projection,
                                order.ingress_result_kind(),
                            )
                        })?;
                }
                RootHomeExitProgress::Unavailable => {}
                _ => {} // Unselected source-only rows retain their existing gate.
            }
        }
        Ok(())
    }
}
