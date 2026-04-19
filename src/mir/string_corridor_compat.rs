/*!
 * String corridor compatibility semantic recovery.
 *
 * This module isolates legacy/helper/runtime-export name recovery from the
 * canonical string-corridor fact builder. It exists to preserve current
 * behavior during migration without letting domain fact builders own compat
 * semantics directly.
 */

use super::{MirInstruction, StringCorridorCarrier, StringCorridorFact, ValueId};
use crate::mir::definitions::call_unified::Callee;
use crate::mir::string_corridor_names::{
    is_len_method_name, is_runtime_len_export, is_runtime_slice_export, is_slice_method_name,
    is_stringish_box_name,
};

pub(crate) fn infer_compat_fact_from_instruction(
    inst: &MirInstruction,
) -> Option<(ValueId, StringCorridorFact)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            args,
            ..
        } => infer_compat_from_method(box_name, method, args.len()).map(|fact| (*dst, fact)),
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            ..
        } => infer_compat_from_global(name).map(|fact| (*dst, fact)),
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            ..
        } => infer_compat_from_runtime_export(name).map(|fact| (*dst, fact)),
        _ => None,
    }
}

pub(crate) fn infer_compat_from_method(
    box_name: &str,
    method: &str,
    arity: usize,
) -> Option<StringCorridorFact> {
    let is_runtime_data_string_facade = box_name == "RuntimeDataBox";
    let is_stringish = is_stringish_box_name(box_name);

    match arity {
        0 if is_len_method_name(method) && (is_stringish || is_runtime_data_string_facade) => Some(
            StringCorridorFact::str_len(StringCorridorCarrier::MethodCall),
        ),
        2 if is_slice_method_name(method) && (is_stringish || is_runtime_data_string_facade) => {
            Some(StringCorridorFact::str_slice(
                StringCorridorCarrier::MethodCall,
            ))
        }
        _ => None,
    }
}

pub(crate) fn infer_compat_from_global(name: &str) -> Option<StringCorridorFact> {
    if let Some(fact) = infer_compat_from_runtime_export(name) {
        return Some(fact);
    }

    let (box_name, rest) = name.split_once('.')?;
    if !is_stringish_box_name(box_name) {
        return None;
    }
    let (method, arity) = rest.split_once('/').unwrap_or((rest, ""));
    let arity = arity.parse::<usize>().ok()?;

    match arity {
        0 if is_len_method_name(method) => Some(StringCorridorFact::str_len(
            StringCorridorCarrier::GlobalLoweredFunction,
        )),
        2 if is_slice_method_name(method) => Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::GlobalLoweredFunction,
        )),
        _ => None,
    }
}

pub(crate) fn infer_compat_from_runtime_export(name: &str) -> Option<StringCorridorFact> {
    if is_runtime_slice_export(name) {
        Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::RuntimeExport,
        ))
    } else if is_runtime_len_export(name) {
        Some(StringCorridorFact::str_len(
            StringCorridorCarrier::RuntimeExport,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{StringCorridorOp, StringCorridorRole};

    #[test]
    fn compat_method_recovery_accepts_runtime_data_substring() {
        let fact = infer_compat_from_method("RuntimeDataBox", "substring", 2)
            .expect("substring compat fact");
        assert_eq!(fact.op, StringCorridorOp::StrSlice);
        assert_eq!(fact.role, StringCorridorRole::BorrowProducer);
        assert_eq!(fact.carrier, StringCorridorCarrier::MethodCall);
    }

    #[test]
    fn compat_runtime_export_recovery_accepts_substring_helper() {
        let fact = infer_compat_from_runtime_export("nyash.string.substring_hii")
            .expect("substring runtime export fact");
        assert_eq!(fact.op, StringCorridorOp::StrSlice);
        assert_eq!(fact.role, StringCorridorRole::BorrowProducer);
        assert_eq!(fact.carrier, StringCorridorCarrier::RuntimeExport);
    }
}
