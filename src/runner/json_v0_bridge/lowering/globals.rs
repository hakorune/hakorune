use super::BridgeEnv;
use crate::mir::{BasicBlockId, ConstValue, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Resolve Bridge-specific globals (imports/hostbridge/env/me dummy).
///
/// - `vars`: current local/global value map (updated when a new value is synthesized)
/// - returns Some(ValueId) when the name is handled here, or Ok(None) to let callers
///   fallback to "undefined variable" handling.
pub(super) fn resolve_bridge_global(
    name: &str,
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    vars: &mut BTreeMap<String, ValueId>,
) -> Result<Option<ValueId>, String> {
    // Phase 21.8: using-imported modules/boxes
    if let Some(box_type) = env.imports.get(name) {
        let dst = f.next_value_id();
        if let Some(bb) = f.get_block_mut(cur_bb) {
            bb.add_instruction(MirInstruction::Const {
                dst,
                value: ConstValue::String(box_type.clone()),
            });
        }
        vars.insert(name.to_string(), dst);
        return Ok(Some(dst));
    }

    // Phase 25.1a: well-known bridge globals
    if name == "hostbridge" {
        let dst = f.next_value_id();
        if let Some(bb) = f.get_block_mut(cur_bb) {
            bb.add_instruction(MirInstruction::Const {
                dst,
                value: ConstValue::String("hostbridge".into()),
            });
        }
        vars.insert(name.to_string(), dst);
        return Ok(Some(dst));
    }

    if name == "env" {
        let dst = f.next_value_id();
        if let Some(bb) = f.get_block_mut(cur_bb) {
            bb.add_instruction(MirInstruction::Const {
                dst,
                value: ConstValue::String("env".into()),
            });
        }
        vars.insert(name.to_string(), dst);
        return Ok(Some(dst));
    }

    if name == "me" {
        if env.allow_me_dummy {
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur_bb) {
                bb.add_instruction(MirInstruction::NewBox {
                    dst,
                    box_type: env.me_class.clone(),
                    args: vec![],
                });
            }
            vars.insert(name.to_string(), dst);
            Ok(Some(dst))
        } else {
            Err(
                "undefined 'me' outside box context (set NYASH_BRIDGE_ME_DUMMY=1 to inject placeholder)"
                    .into(),
            )
        }
    } else {
        Ok(None)
    }
}
