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
                            match exits.get(&(self.owner, exit.clone())) {
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
                            &self.ledger,
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
            let projected = match terminal {
                TerminalRelationV1::Call(row) => {
                    FinalizedRootResultAbiV1::CallReturn { owner: row.owner() }
                }
                TerminalRelationV1::I64Add(row) => {
                    FinalizedRootResultAbiV1::I64AddReturn { owner: row.owner() }
                }
                TerminalRelationV1::Unit(row) => {
                    FinalizedRootResultAbiV1::UnitReturn { owner: row.owner() }
                }
                TerminalRelationV1::IntegerLiteral(row) => {
                    FinalizedRootResultAbiV1::IntegerLiteralReturn { owner: row.owner() }
                }
                // Bool source evidence has no selected physical result ABI yet.
                TerminalRelationV1::BoolLiteral(_) => return Ok(None),
                TerminalRelationV1::I64Field(row) => {
                    FinalizedRootResultAbiV1::I64FieldReturn { owner: row.owner() }
                }
                TerminalRelationV1::I64Scalar(row) => {
                    FinalizedRootResultAbiV1::I64ScalarReturn { owner: row.owner() }
                }
                // A non-i64 value return derives no physical result ABI at
                // this boundary; the lifecycle capability lane supplies it.
                TerminalRelationV1::Value(_) => return Ok(None),
                // An opaque call return proves no result class at all.
                TerminalRelationV1::OpaqueCall(_) => return Ok(None),
                TerminalRelationV1::MapGet(row) => {
                    FinalizedRootResultAbiV1::MapGetReturn { owner: row.owner() }
                }
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
    CallReturn {
        owner: FunctionOwnerIdV1,
    },
    I64AddReturn {
        owner: FunctionOwnerIdV1,
    },
    UnitReturn {
        owner: FunctionOwnerIdV1,
    },
    IntegerLiteralReturn {
        owner: FunctionOwnerIdV1,
    },
    I64FieldReturn {
        owner: FunctionOwnerIdV1,
    },
    /// `return <bound-i64-local>`/trivial integer expression — the scalar
    /// classifier's proven-i64 exit. The i64 proof stays in the sealed
    /// expression-source inventory; this row only carries the owner.
    I64ScalarReturn {
        owner: FunctionOwnerIdV1,
    },
    /// `return <map>.get("<literal>")` — the readable-Map terminal. The
    /// checked read produces the exact i64 payload; the map itself is
    /// never the returned value.
    MapGetReturn {
        owner: FunctionOwnerIdV1,
    },
}
