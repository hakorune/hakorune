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

    match (method, arity) {
        ("length", 0) | ("len", 0) if is_stringish || is_runtime_data_string_facade => Some(
            StringCorridorFact::str_len(StringCorridorCarrier::MethodCall),
        ),
        ("substring", 2) | ("slice", 2) if is_stringish || is_runtime_data_string_facade => Some(
            StringCorridorFact::str_slice(StringCorridorCarrier::MethodCall),
        ),
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

    match (method, arity) {
        ("length", 0) | ("len", 0) => Some(StringCorridorFact::str_len(
            StringCorridorCarrier::GlobalLoweredFunction,
        )),
        ("substring", 2) | ("slice", 2) => Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::GlobalLoweredFunction,
        )),
        _ => None,
    }
}

pub(crate) fn infer_compat_from_runtime_export(name: &str) -> Option<StringCorridorFact> {
    match name {
        "nyash.string.substring_hii" => Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::RuntimeExport,
        )),
        "nyash.string.substring_concat_hhii" | "nyash.string.substring_concat3_hhhii" => Some(
            StringCorridorFact::str_slice(StringCorridorCarrier::RuntimeExport),
        ),
        "nyash.string.substring_len_hii" => Some(StringCorridorFact::str_len(
            StringCorridorCarrier::RuntimeExport,
        )),
        "nyash.string.length_si" | "nyrt_string_length" | "nyrt.string.length" => Some(
            StringCorridorFact::str_len(StringCorridorCarrier::RuntimeExport),
        ),
        _ => None,
    }
}

fn is_stringish_box_name(box_name: &str) -> bool {
    matches!(box_name, "StringBox" | "String" | "__str")
        || box_name.ends_with("StringBox")
        || box_name.ends_with("String")
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
