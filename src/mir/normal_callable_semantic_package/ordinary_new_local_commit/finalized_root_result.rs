//! Final result projection from the original retained terminal owner.
use super::*;

impl FinalizedRootSourceHandoffV1 {
    /// Derived at this result boundary; never retained as a second source
    /// tag. Every exit must project to the same physical result class —
    /// divergent exit kinds name no single ABI and yield `None`.
    pub(crate) fn result_abi(&self) -> Option<FinalizedRootResultAbiV1> {
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
                TerminalRelationV1::I64Field(row) => {
                    FinalizedRootResultAbiV1::I64FieldReturn { owner: row.owner() }
                }
                TerminalRelationV1::I64Scalar(row) => {
                    FinalizedRootResultAbiV1::I64ScalarReturn { owner: row.owner() }
                }
                // A non-i64 value return derives no physical result ABI at
                // this boundary; the lifecycle capability lane supplies it.
                TerminalRelationV1::Value(_) => return None,
                // An opaque call return proves no result class at all.
                TerminalRelationV1::OpaqueCall(_) => return None,
                TerminalRelationV1::MapGet(row) => {
                    FinalizedRootResultAbiV1::MapGetReturn { owner: row.owner() }
                }
            };
            match abi {
                None => abi = Some(projected),
                Some(existing) if existing == projected => {}
                Some(_) => return None,
            }
        }
        abi
    }
}

/// Final-handoff projection of the already-issued terminal source relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalizedRootResultAbiV1 {
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
