//! Box: PHI block remapper (Phase 94 tidy-up)
//!
//! Responsibility: remap only the block ids of MIR `Phi` inputs using the
//! caller-provided `local_block_map`, leaving ValueIds untouched (already
//! remapped upstream). Handles both `type_hint = None` and `Some(...)`.

use std::collections::BTreeMap;

use crate::mir::{BasicBlockId, MirInstruction, MirType, ValueId};

/// Remap a single PHI instruction's block ids.
pub(crate) fn remap_phi_instruction(
    dst: ValueId,
    inputs: &[(BasicBlockId, ValueId)],
    type_hint: Option<MirType>,
    local_block_map: &BTreeMap<BasicBlockId, BasicBlockId>,
)-> MirInstruction {
    MirInstruction::Phi {
        dst,
        inputs: remap_phi_inputs(inputs, local_block_map),
        type_hint,
    }
}

fn remap_phi_inputs(
    inputs: &[(BasicBlockId, ValueId)],
    local_block_map: &BTreeMap<BasicBlockId, BasicBlockId>,
) -> Vec<(BasicBlockId, ValueId)> {
    inputs
        .iter()
        .map(|(bb, val)| {
            let remapped_bb = local_block_map.get(bb).copied().unwrap_or(*bb);
            (remapped_bb, *val)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bb(id: u32) -> BasicBlockId {
        BasicBlockId(id)
    }

    #[test]
    fn remaps_blocks_preserves_values_and_type_none() {
        let inputs = vec![(bb(1), ValueId(10)), (bb(2), ValueId(11))];
        let mut map = BTreeMap::new();
        map.insert(bb(1), bb(10));
        map.insert(bb(99), bb(100)); // unused entry should not matter

        let inst = remap_phi_instruction(ValueId(5), &inputs, None, &map);

        match inst {
            MirInstruction::Phi { dst, inputs, type_hint } => {
                assert_eq!(dst, ValueId(5));
                assert_eq!(
                    inputs,
                    vec![(bb(10), ValueId(10)), (bb(2), ValueId(11))]
                );
                assert!(type_hint.is_none());
            }
            other => panic!("expected Phi, got {:?}", other),
        }
    }

    #[test]
    fn remaps_blocks_preserves_type_hint() {
        let inputs = vec![(bb(3), ValueId(20))];
        let mut map = BTreeMap::new();
        map.insert(bb(3), bb(30));

        let inst = remap_phi_instruction(ValueId(7), &inputs, Some(MirType::Integer), &map);

        match inst {
            MirInstruction::Phi { dst, inputs, type_hint } => {
                assert_eq!(dst, ValueId(7));
                assert_eq!(inputs, vec![(bb(30), ValueId(20))]);
                assert_eq!(type_hint, Some(MirType::Integer));
            }
            other => panic!("expected Phi, got {:?}", other),
        }
    }
}

