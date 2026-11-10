use crate::mir::function::MirFunction;
use crate::mir::verification_types::VerificationError;

/// One shared definition index for SSA checks and domain verifiers.
pub(super) fn definitions(
    function: &MirFunction,
) -> (
    crate::mir::value_origin::ValueDefMap,
    Vec<VerificationError>,
) {
    let mut errors = Vec::new();
    let mut definitions: crate::mir::value_origin::ValueDefMap = Default::default();

    // Treat parameters as defined at the entry block.
    for pid in &function.params {
        if let Some((first_block, _)) = definitions.insert(*pid, (function.entry_block, usize::MAX))
        {
            errors.push(VerificationError::MultipleDefinition {
                value: *pid,
                first_block,
                second_block: function.entry_block,
            });
        }
    }

    // Deterministic iteration: function.blocks is a HashMap, and nondeterminism makes
    // SSA/verify diagnosis harder (e.g. "first_block" can drift run-to-run).
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort_by_key(|b| b.0);

    for block_id in &block_ids {
        let block = &function.blocks[block_id];
        for (inst_idx, instruction) in block.all_instructions().enumerate() {
            if let Some(dst) = instruction.dst_value() {
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

    (definitions, errors)
}

/// Verify SSA form: single assignment and all uses defined.
pub fn check_ssa_form(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let (definitions, mut errors) = definitions(function);
    check_uses(function, &definitions, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(super) fn check_uses(
    function: &MirFunction,
    definitions: &crate::mir::value_origin::ValueDefMap,
    errors: &mut Vec<VerificationError>,
) {
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in &block_ids {
        let block = &function.blocks[block_id];
        for (inst_idx, instruction) in block.all_instructions().enumerate() {
            for used_value in instruction.used_values() {
                if !definitions.contains_key(&used_value) {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[ssa-undef-debug] fn={} bb={:?} inst_idx={} used={:?} inst={:?}",
                            function.signature.name, block_id, inst_idx, used_value, instruction
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
}
