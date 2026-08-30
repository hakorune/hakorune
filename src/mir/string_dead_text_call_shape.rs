// Pure call-shape matching used by the passive dead-text region plan.
// This module owns no plan state and issues no semantic target.

use crate::mir::string_corridor_names::{
    is_len_method_name, is_lowered_len_global, is_runtime_len_handle_export,
};
use crate::mir::{Callee, MirInstruction, ValueId};

pub(crate) fn match_dead_text_len_call(inst: &MirInstruction) -> Option<(ValueId, Vec<ValueId>)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if is_len_method_name(method) => {
            let mut values = vec![*receiver];
            values.extend(args.iter().copied());
            Some((*dst, values))
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 1 && is_runtime_len_handle_export(name) => {
            Some((*dst, args.iter().copied().collect::<Vec<_>>()))
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            args,
            ..
        } if args.len() == 1 && is_lowered_len_global(&name.display_name()) => {
            Some((*dst, args.iter().copied().collect::<Vec<_>>()))
        }
        _ => None,
    }
}
