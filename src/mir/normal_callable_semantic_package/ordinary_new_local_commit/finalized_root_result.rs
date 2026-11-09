//! Final result projection from the original retained terminal owner.
use super::*;

impl FinalizedRootSourceHandoffV1 {
    /// Derived at this result boundary; never retained as a second source
    /// tag. Every exit must project to the same physical result class —
    /// divergent exit kinds name no single ABI and yield `None`.
    pub(crate) fn result_abi(
        &self,
        module: &crate::mir::MirModule,
    ) -> Result<Option<FinalizedRootResultAbiV1>, String> {
        if let Some(descriptor) = self.ledger.checked_root_object_result_v1()? {
            if descriptor.owner != self.owner
                || descriptor.terminals.len() != self.terminals.len()
                || descriptor.terminals.iter().any(|terminal| {
                    match (self.terminals.get(terminal.return_site()), *terminal) {
                        (
                            Some(TerminalRelationV1::Value(saved)),
                            TerminalRelationV1::Value(original),
                        ) => saved != original,
                        _ => true,
                    }
                })
            {
                return Err(freeze("root-result/source-identity"));
            }
            self.ledger
                .with_finished_projection(self.owner, |symbol, projection| {
                    let function = module
                        .functions
                        .get(symbol)
                        .filter(|function| function.signature.name == symbol)
                        .ok_or_else(|| freeze("root-result/function-missing"))?;
                    let exits = self.ledger.root_exits.borrow();
                    for terminal in &descriptor.terminals {
                        let exit = terminal.return_site();
                        let (order, entry, bindings, finishing) =
                            match exits.get(&(self.owner, exit.clone().into())) {
                                Some(RootHomeExitProgress::Finalized(order)) => {
                                    let (entry, bindings) = self
                                        .call_entries
                                        .get(exit)
                                        .ok_or_else(|| freeze("root-result/entry-missing"))?;
                                    (order, entry, bindings.as_slice(), None)
                                }
                                Some(RootHomeExitProgress::Emitted {
                                    order,
                                    entry,
                                    bindings,
                                    ..
                                }) => (order, entry, bindings.as_slice(), Some(projection)),
                                _ => return Err(freeze("root-result/exit-unavailable")),
                            };
                        let TerminalRelationV1::Value(value) = terminal else {
                            return Err(freeze("root-result/value-missing"));
                        };
                        match value.returned() {
                            TerminalReturnedSourceV1::Construction(_) => {
                                if self
                                    .ledger
                                    .validate_fresh_return_producer_v1(
                                        self.owner,
                                        exit,
                                        entry,
                                        bindings,
                                        Some(projection),
                                        finishing,
                                        Some(function),
                                    )?
                                    .is_none()
                                {
                                    return Err(freeze("root-result/fresh-producer-unavailable"));
                                }
                            }
                            TerminalReturnedSourceV1::NullLiteral => {
                                if order.null_return_binding().is_none() {
                                    return Err(freeze("root-result/null-producer-missing"));
                                }
                            }
                            TerminalReturnedSourceV1::OwnedCall(_) => {}
                            _ => return Err(freeze("root-result/source-unsupported")),
                        }
                        super::finalized_root_cleanup::validate_finished_cleanup_entry(
                            CallValidationContextV1::Finalized(self),
                            self.owner,
                            exit,
                            function,
                            projection,
                            order,
                            entry,
                            bindings,
                            finishing,
                        )?;
                    }
                    Ok(())
                })?;
            return Ok(Some(FinalizedRootResultAbiV1::ObjectReturn {
                owner: descriptor.owner,
                kind: descriptor.kind,
            }));
        }
        let mut abi = None;
        for terminal in self.terminals.values() {
            let owner = terminal.owner();
            if owner != self.owner { return Err(freeze("root-result/source-identity")); }
            // Source evidence retains every operation/producer separately.
            // This boundary projects only the single physical integer ABI.
            let projected = match terminal {
                TerminalRelationV1::Call(_)
                | TerminalRelationV1::IntegerLiteral(_)
                | TerminalRelationV1::Scalar(_) | TerminalRelationV1::MapGet(_) =>
                    FinalizedRootResultAbiV1::I64Return { owner },
                TerminalRelationV1::Unit(_) => FinalizedRootResultAbiV1::UnitReturn { owner },
                TerminalRelationV1::BoolLiteral(_) | TerminalRelationV1::Value(_)
                | TerminalRelationV1::OpaqueCall(_) => return Ok(None),
            };
            match abi {
                None => abi = Some(projected),
                Some(existing) if existing == projected => {}
                Some(_) => return Ok(None),
            }
        }
        Ok(abi)
    }
}

/// Final-handoff projection of the already-issued terminal source relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalizedRootResultAbiV1 {
    /// Same original whole-Root descriptor corroborated against final MIR.
    ObjectReturn {
        owner: FunctionOwnerIdV1,
        kind: crate::mir::instruction::InvokeCallResultKind,
    },
    /// Known integer source exits share one ABI; original source relations
    /// retain their distinct producer and cleanup evidence.
    I64Return { owner: FunctionOwnerIdV1 },
    UnitReturn { owner: FunctionOwnerIdV1 },
}
