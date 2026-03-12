use super::BridgeEnv;
use crate::mir::{BasicBlockId, ConstValue, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Lower a throw expression/value according to BridgeEnv and throw_ctx.
///
/// - Result-mode try contextが有効なら、catch ブロックへ jump しつつ incoming に記録する。
/// - それ以外では:
///   - throw_enabled=1 のとき MIR Throw を発行
///   - そうでなければ `const 0` にフォールバック
pub(super) fn lower_throw(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    exception_value: ValueId,
    vars: Option<&BTreeMap<String, ValueId>>,
) -> (ValueId, BasicBlockId) {
    // Result-mode try context active: route to current catch via Jump and record incoming
    if env.try_result_mode && super::throw_ctx::is_active() {
        if crate::config::env::cli_verbose() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[Bridge] lower_throw: routing to catch (Result-mode)"
            ));
        }
        let _ = super::throw_ctx::record_throw(f, cur_bb, exception_value, vars);
        return (exception_value, cur_bb);
    }

    // Legacy path: emit MIR Throw (if enabled) or degrade to const 0
    if env.throw_enabled {
        if let Some(bb) = f.get_block_mut(cur_bb) {
            bb.set_terminator(MirInstruction::Throw {
                exception: exception_value,
                effects: EffectMask::PANIC,
            });
        }
        (exception_value, cur_bb)
    } else {
        let dst = f.next_value_id();
        if let Some(bb) = f.get_block_mut(cur_bb) {
            bb.add_instruction(MirInstruction::Const {
                dst,
                value: ConstValue::Integer(0),
            });
        }
        (dst, cur_bb)
    }
}
