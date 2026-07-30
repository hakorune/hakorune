//! Uniform incoming-PHI type fallback for Builder return-type finalization.

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

pub(super) fn resolve_from_phi(
    function: &MirFunction,
    return_value: ValueId,
    types: &BTreeMap<ValueId, MirType>,
) -> Option<MirType> {
    for (_bid, block) in function.blocks.iter() {
        for instruction in &block.instructions {
            if let MirInstruction::Phi { dst, inputs, .. } = instruction {
                if *dst == return_value {
                    let mut incoming_types =
                        inputs.iter().filter_map(|(_, value)| types.get(value));
                    if let Some(first) = incoming_types.next() {
                        if incoming_types.all(|ty| ty == first) {
                            return Some(first.clone());
                        }
                    }
                }
            }
        }
    }
    None
}
