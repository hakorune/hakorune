/*!
 * Shared string corridor shape recognizers.
 *
 * This module is the shape SSOT for the current string corridor lane.
 * It contains pure helper logic only:
 * - substring/concat/helper shape recognition
 * - source identity and const-length observation
 *
 * It consumes generic value-origin queries rather than owning alias-root
 * normalization itself.
 *
 * It does not emit plans and it does not mutate MIR.
 */

use super::value_origin::{resolve_value_origin, ValueDefMap};
use super::ValueId;
use super::{
    ArrayElementWriteKind, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirFunction,
    MirInstruction,
};
use crate::mir::ssot::method_call::method_call_operand_view;
use crate::mir::string_corridor_names::{
    is_len_method_name, is_lowered_len_global, is_runtime_concat3_export,
    is_runtime_len_handle_export, is_runtime_substring_concat3_export, is_runtime_substring_export,
    is_runtime_substring_len_export, is_slice_method_name,
};

/// Read-only call projection shared by canonical and compatibility carriers.
///
/// The corridor recognizers do not own call semantics; they only inspect the
/// already-issued callee/operands.  Keeping this projection here prevents a
/// canonical `MirCall` from silently disappearing when an optimization pass
/// still has a legacy-only pattern match.
pub(crate) struct CallShape<'a> {
    pub(crate) dst: Option<ValueId>,
    pub(crate) callee: &'a Callee,
    pub(crate) args: &'a [ValueId],
    pub(crate) effects: EffectMask,
}

pub(crate) fn call_shape(inst: &MirInstruction) -> Option<CallShape<'_>> {
    match inst {
        MirInstruction::Call(call) => Some(CallShape {
            dst: call.dst,
            callee: &call.callee,
            args: &call.args,
            effects: call.effects,
        }),
        MirInstruction::LegacyCallV0 {
            dst,
            callee: Some(callee),
            args,
            effects,
            ..
        } => Some(CallShape {
            dst: *dst,
            callee,
            args,
            effects: *effects,
        }),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AddShape {
    pub idx: usize,
    pub dst: ValueId,
    pub lhs: ValueId,
    pub rhs: ValueId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SubstringCallProducerShape {
    pub source: ValueId,
    pub start: ValueId,
    pub end: ValueId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ConcatTripletShape {
    pub left: ValueId,
    pub middle: ValueId,
    pub right: ValueId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SubstringConcat3HelperShape {
    pub dst: ValueId,
    pub left: ValueId,
    pub middle: ValueId,
    pub right: ValueId,
    pub start: ValueId,
    pub end: ValueId,
    pub effects: EffectMask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MethodSetCallShape {
    pub box_name: String,
    pub receiver: ValueId,
    pub key: ValueId,
    pub value: ValueId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StringSourceIdentity {
    Value(ValueId),
    ConstString(String),
}

pub(crate) fn match_add_in_block(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<AddShape> {
    let (inst_bbid, idx) = def_map.get(&value).copied()?;
    if inst_bbid != bbid {
        return None;
    }
    let block = function.blocks.get(&inst_bbid)?;
    match block.instructions.get(idx)? {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } => Some(AddShape {
            idx,
            dst: *dst,
            lhs: *lhs,
            rhs: *rhs,
        }),
        _ => None,
    }
}

pub(crate) fn match_len_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, EffectMask)> {
    let call = call_shape(inst)?;
    let dst = call.dst?;
    match call.callee {
        Callee::Method {
            method,
            receiver: Some(receiver),
            ..
        } if is_len_method_name(method) => {
            let view = method_call_operand_view(*receiver, call.args, 0)?;
            Some((dst, view.operand_receiver, call.effects))
        }
        Callee::Extern(name)
            if call.args.len() == 1 && is_runtime_len_handle_export(name) =>
        {
            Some((dst, call.args[0], call.effects))
        }
        Callee::Global(name)
            if call.args.len() == 1 && is_lowered_len_global(&name.display_name()) =>
        {
            Some((dst, call.args[0], call.effects))
        }
        _ => None,
    }
}

pub(crate) fn match_substring_len_call(
    inst: &MirInstruction,
) -> Option<(ValueId, ValueId, ValueId, ValueId)> {
    let call = call_shape(inst)?;
    let dst = call.dst?;
    match call.callee {
        Callee::Extern(name)
            if call.args.len() == 3 && is_runtime_substring_len_export(name) =>
        {
            Some((dst, call.args[0], call.args[1], call.args[2]))
        }
        _ => None,
    }
}

pub(crate) fn match_substring_call(
    inst: &MirInstruction,
) -> Option<(ValueId, ValueId, ValueId, ValueId, EffectMask)> {
    let call = call_shape(inst)?;
    let dst = call.dst?;
    match call.callee {
        Callee::Method {
            method,
            receiver: Some(receiver),
            ..
        } if is_slice_method_name(method) => {
            let view = method_call_operand_view(*receiver, call.args, 2)?;
            let [start, end] = view.explicit_args else {
                return None;
            };
            Some((dst, view.operand_receiver, *start, *end, call.effects))
        }
        Callee::Extern(name)
            if call.args.len() == 3 && is_runtime_substring_export(name) =>
        {
            Some((dst, call.args[0], call.args[1], call.args[2], call.effects))
        }
        _ => None,
    }
}

pub(crate) fn match_substring_concat3_helper_call(
    inst: &MirInstruction,
) -> Option<SubstringConcat3HelperShape> {
    let call = call_shape(inst)?;
    let dst = call.dst?;
    match call.callee {
        Callee::Extern(name)
            if call.args.len() == 5 && is_runtime_substring_concat3_export(name) =>
        {
            Some(SubstringConcat3HelperShape {
                dst,
                left: call.args[0],
                middle: call.args[1],
                right: call.args[2],
                start: call.args[3],
                end: call.args[4],
                effects: call.effects,
            })
        }
        _ => None,
    }
}

pub(crate) fn match_method_set_call(inst: &MirInstruction) -> Option<MethodSetCallShape> {
    if let MirInstruction::ArrayElementWrite {
        kind: ArrayElementWriteKind::Set,
        receiver,
        index: Some(key),
        value,
        ..
    } = inst
    {
        return Some(MethodSetCallShape {
            // ArrayElementWrite is the canonical ArrayBox set carrier.  The
            // method-shaped projection keeps existing corridor consumers on
            // the shared matcher without reintroducing a call authority.
            box_name: "ArrayBox".to_string(),
            receiver: *receiver,
            key: *key,
            value: *value,
        });
    }

    let call = call_shape(inst)?;
    match call.callee {
        Callee::Method {
            box_name,
            method,
            receiver: Some(receiver),
            ..
        } if call.args.len() == 2 && method == "set" => Some(MethodSetCallShape {
            box_name: box_name.clone(),
            receiver: *receiver,
            key: call.args[0],
            value: call.args[1],
        }),
        _ => None,
    }
}

pub(crate) fn extract_substring_args(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
    let call = call_shape(inst)?;
    match call.callee {
        Callee::Method {
            method,
            receiver: Some(source),
            ..
        } if is_slice_method_name(method) => {
            let view = method_call_operand_view(*source, call.args, 2)?;
            let [start, end] = view.explicit_args else {
                return None;
            };
            Some((view.operand_receiver, *start, *end))
        }
        Callee::Extern(name)
            if call.args.len() == 3 && is_runtime_substring_export(name) =>
        {
            Some((call.args[0], call.args[1], call.args[2]))
        }
        _ => None,
    }
}

pub(crate) fn match_substring_call_shape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<SubstringCallProducerShape> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let (_, receiver, start, end, _) = match_substring_call(block.instructions.get(idx)?)?;
    Some(SubstringCallProducerShape {
        source: resolve_value_origin(function, def_map, receiver),
        start: resolve_value_origin(function, def_map, start),
        end: resolve_value_origin(function, def_map, end),
    })
}

pub(crate) fn match_concat_triplet_from_extern(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<ConcatTripletShape> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let call = call_shape(block.instructions.get(idx)?)?;
    match call.callee {
        Callee::Extern(name)
            if call.args.len() == 3 && is_runtime_concat3_export(name) =>
        {
            Some(ConcatTripletShape {
                left: resolve_value_origin(function, def_map, call.args[0]),
                middle: resolve_value_origin(function, def_map, call.args[1]),
                right: resolve_value_origin(function, def_map, call.args[2]),
            })
        }
        _ => None,
    }
}

pub(crate) fn match_concat_triplet_from_add_chain(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<ConcatTripletShape> {
    let root = resolve_value_origin(function, def_map, value);
    let outer = match_add_in_block(function, bbid, def_map, root)?;
    if outer.dst != root {
        return None;
    }

    let lhs_root = resolve_value_origin(function, def_map, outer.lhs);
    let rhs_root = resolve_value_origin(function, def_map, outer.rhs);

    if let Some(inner) = match_add_in_block(function, bbid, def_map, lhs_root) {
        if inner.idx < outer.idx && inner.dst == lhs_root {
            return Some(ConcatTripletShape {
                left: resolve_value_origin(function, def_map, inner.lhs),
                middle: resolve_value_origin(function, def_map, inner.rhs),
                right: rhs_root,
            });
        }
    }

    if let Some(inner) = match_add_in_block(function, bbid, def_map, rhs_root) {
        if inner.idx < outer.idx && inner.dst == rhs_root {
            return Some(ConcatTripletShape {
                left: lhs_root,
                middle: resolve_value_origin(function, def_map, inner.lhs),
                right: resolve_value_origin(function, def_map, inner.rhs),
            });
        }
    }

    None
}

pub(crate) fn match_concat_triplet(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<ConcatTripletShape> {
    match_concat_triplet_from_extern(function, def_map, value)
        .or_else(|| match_concat_triplet_from_add_chain(function, bbid, def_map, value))
}

pub(crate) fn string_source_identity(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<StringSourceIdentity> {
    let root = resolve_value_origin(function, def_map, value);
    let Some((bbid, idx)) = def_map.get(&root).copied() else {
        return Some(StringSourceIdentity::Value(root));
    };
    let Some(block) = function.blocks.get(&bbid) else {
        return Some(StringSourceIdentity::Value(root));
    };
    match block.instructions.get(idx) {
        Some(MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        }) => Some(StringSourceIdentity::ConstString(text.clone())),
        _ => Some(StringSourceIdentity::Value(root)),
    }
}

pub(crate) fn const_string_length(text: &str) -> i64 {
    if crate::config::env::string_codepoint_mode() {
        text.chars().count() as i64
    } else {
        text.len() as i64
    }
}
