/*!
 * MIR pass: legacy method_id injection compatibility stub.
 *
 * RCL-3-min3 retires `MirInstruction::BoxCall`/`ExternCall` and keeps only
 * canonical `MirInstruction::Call`.
 * Method metadata now flows through `Callee` directly, so this pass has
 * nothing to inject.
 */

use crate::mir::MirModule;

pub fn inject_method_ids(_module: &mut MirModule) -> usize {
    0
}
