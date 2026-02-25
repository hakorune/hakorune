use crate::mir::function::MirFunction;
use crate::mir::verification_types::VerificationError;
use crate::mir::ValueId;

/// Verify SSA form: single assignment and all uses defined
pub fn check_ssa_form(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    use std::collections::HashMap;
    let mut errors = Vec::new();
    let mut definitions: HashMap<ValueId, (crate::mir::BasicBlockId, usize)> = HashMap::new();

    // Treat parameters as defined at the entry block.
    for pid in &function.params {
        definitions.insert(*pid, (function.entry_block, 0));
    }

    // Deterministic iteration: function.blocks is a HashMap, and nondeterminism makes
    // SSA/verify diagnosis harder (e.g. "first_block" can drift run-to-run).
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort_by_key(|b| b.0);

    for block_id in &block_ids {
        let block = &function.blocks[block_id];
        for (inst_idx, sp) in block.all_spanned_instructions_enumerated() {
            if let Some(dst) = sp.inst.dst_value() {
                if let Some((first_block, _)) = definitions.insert(dst, (*block_id, inst_idx)) {
                    errors.push(VerificationError::MultipleDefinition {
                        value: dst,
                        first_block,
                        second_block: *block_id,
                    });
                }
            }
        }
    }

    for block_id in &block_ids {
        let block = &function.blocks[block_id];
        for (inst_idx, sp) in block.all_spanned_instructions_enumerated() {
            for used_value in sp.inst.used_values() {
                if !definitions.contains_key(&used_value) {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[ssa-undef-debug] fn={} bb={:?} inst_idx={} used={:?} inst={:?}",
                            function.signature.name, block_id, inst_idx, used_value, sp.inst
                        ));
                    }
                    errors.push(VerificationError::UndefinedValue {
                        value: used_value,
                        block: *block_id,
                        instruction_index: inst_idx,
                    });
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
