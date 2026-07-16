//! HMI-S0 exact-none control-edge witness.
//!
//! MIR JSON V1 does not transport edge arguments. This verifier therefore
//! runs over the final `MirFunction` before JSON publication and admits only
//! functions whose Jump/Branch edge-argument options are exactly `None`.

use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use serde_json::{json, Value};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlEdgeSiteV1 {
    Instruction(usize),
    Terminator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlEdgeKindV1 {
    Jump,
    BranchThen,
    BranchElse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactNoneControlEdgeArgsErrorV1 {
    function: String,
    block: BasicBlockId,
    site: ControlEdgeSiteV1,
    edge: ControlEdgeKindV1,
}

impl fmt::Display for ExactNoneControlEdgeArgsErrorV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let site = match self.site {
            ControlEdgeSiteV1::Instruction(index) => format!("instruction:{index}"),
            ControlEdgeSiteV1::Terminator => "terminator".to_string(),
        };
        let edge = match self.edge {
            ControlEdgeKindV1::Jump => "jump",
            ControlEdgeKindV1::BranchThen => "branch_then",
            ControlEdgeKindV1::BranchElse => "branch_else",
        };
        write!(
            f,
            "[freeze:contract][hmi/control-edge-args-v1/not-exact-none] function={} block={} site={} edge={}",
            self.function, self.block, site, edge
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VerifiedExactNoneControlEdgeArgsV1 {
    _seal: (),
}

impl VerifiedExactNoneControlEdgeArgsV1 {
    pub(crate) fn verify(function: &MirFunction) -> Result<Self, ExactNoneControlEdgeArgsErrorV1> {
        let mut blocks: Vec<_> = function.blocks.keys().copied().collect();
        blocks.sort();
        for block_id in blocks {
            let block = &function.blocks[&block_id];
            for (index, instruction) in block.instructions.iter().enumerate() {
                verify_instruction(
                    function,
                    block_id,
                    ControlEdgeSiteV1::Instruction(index),
                    instruction,
                )?;
            }
            if let Some(terminator) = &block.terminator {
                verify_instruction(
                    function,
                    block_id,
                    ControlEdgeSiteV1::Terminator,
                    terminator,
                )?;
            }
        }
        Ok(Self { _seal: () })
    }

    pub(super) fn to_json(self) -> Value {
        json!({
            "schema_version": 1,
            "mode": "exact_none",
        })
    }
}

fn verify_instruction(
    function: &MirFunction,
    block: BasicBlockId,
    site: ControlEdgeSiteV1,
    instruction: &MirInstruction,
) -> Result<(), ExactNoneControlEdgeArgsErrorV1> {
    let reject = |edge| ExactNoneControlEdgeArgsErrorV1 {
        function: function.signature.name.clone(),
        block,
        site,
        edge,
    };
    match instruction {
        MirInstruction::Jump {
            edge_args: Some(_), ..
        } => Err(reject(ControlEdgeKindV1::Jump)),
        MirInstruction::Branch {
            then_edge_args,
            else_edge_args,
            ..
        } => {
            if then_edge_args.is_some() {
                return Err(reject(ControlEdgeKindV1::BranchThen));
            }
            if else_edge_args.is_some() {
                return Err(reject(ControlEdgeKindV1::BranchElse));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
