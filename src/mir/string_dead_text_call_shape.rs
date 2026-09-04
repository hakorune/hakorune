// Pure call-shape matching used by the passive dead-text region plan.
// This module owns no plan state and issues no semantic target.

use crate::mir::string_corridor_names::{
    is_len_method_name, is_lowered_len_global, is_runtime_len_handle_export,
};
use crate::mir::{Callee, MirInstruction, ValueId};

pub(crate) fn match_dead_text_len_call(inst: &MirInstruction) -> Option<(ValueId, Vec<ValueId>)> {
    match inst {
        MirInstruction::Call(call) => match &call.callee {
            Callee::Method {
                method,
                receiver: Some(receiver),
                ..
            } if is_len_method_name(method) => {
                let mut values = vec![*receiver];
                values.extend(call.args.iter().copied());
                Some((call.dst?, values))
            }
            Callee::Extern(name) if call.args.len() == 1 && is_runtime_len_handle_export(name) => {
                Some((call.dst?, call.args.clone()))
            }
            Callee::Global(name)
                if call.args.len() == 1 && is_lowered_len_global(&name.display_name()) =>
            {
                Some((call.dst?, call.args.clone()))
            }
            _ => None,
        },
        MirInstruction::LegacyCallV0 {
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
        MirInstruction::LegacyCallV0 {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 1 && is_runtime_len_handle_export(name) => {
            Some((*dst, args.iter().copied().collect::<Vec<_>>()))
        }
        MirInstruction::LegacyCallV0 {
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
