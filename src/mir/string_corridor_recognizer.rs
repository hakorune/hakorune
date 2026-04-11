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

use super::{
    resolve_value_origin, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirFunction,
    MirInstruction, ValueDefMap, ValueId,
};

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
            effects,
            ..
        } if args.is_empty() && matches!(method.as_str(), "length" | "len") => {
            Some((*dst, *receiver, *effects))
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            effects,
            ..
        } if args.len() == 1 && name == "nyash.string.len_h" => Some((*dst, args[0], *effects)),
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            args,
            effects,
            ..
        } if args.len() == 1 && matches!(name.as_str(), "str.len" | "__str.len") => {
            Some((*dst, args[0], *effects))
        }
        _ => None,
    }
}

pub(crate) fn match_substring_len_call(
    inst: &MirInstruction,
) -> Option<(ValueId, ValueId, ValueId, ValueId)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 3 && name == "nyash.string.substring_len_hii" => {
            Some((*dst, args[0], args[1], args[2]))
        }
        _ => None,
    }
}

pub(crate) fn match_substring_call(
    inst: &MirInstruction,
) -> Option<(ValueId, ValueId, ValueId, ValueId, EffectMask)> {
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
            effects,
            ..
        } if args.len() == 2 && matches!(method.as_str(), "substring" | "slice") => {
            Some((*dst, *receiver, args[0], args[1], *effects))
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            effects,
            ..
        } if args.len() == 3 && name == "nyash.string.substring_hii" => {
            Some((*dst, args[0], args[1], args[2], *effects))
        }
        _ => None,
    }
}

pub(crate) fn match_substring_concat3_helper_call(
    inst: &MirInstruction,
) -> Option<SubstringConcat3HelperShape> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            effects,
            ..
        } if args.len() == 5 && name == "nyash.string.substring_concat3_hhhii" => {
            Some(SubstringConcat3HelperShape {
                dst: *dst,
                left: args[0],
                middle: args[1],
                right: args[2],
                start: args[3],
                end: args[4],
                effects: *effects,
            })
        }
        _ => None,
    }
}

pub(crate) fn match_method_set_call(inst: &MirInstruction) -> Option<MethodSetCallShape> {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if args.len() == 2 && method == "set" => Some(MethodSetCallShape {
            box_name: box_name.clone(),
            receiver: *receiver,
            key: args[0],
            value: args[1],
        }),
        _ => None,
    }
}

pub(crate) fn extract_substring_args(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(source),
                    ..
                }),
            args,
            ..
        } if args.len() == 2 && matches!(method.as_str(), "substring" | "slice") => {
            Some((*source, args[0], args[1]))
        }
        MirInstruction::Call {
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 3 && name == "nyash.string.substring_hii" => {
            Some((args[0], args[1], args[2]))
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
    match block.instructions.get(idx)? {
        MirInstruction::Call {
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 3 && name == "nyash.string.concat3_hhh" => Some(ConcatTripletShape {
            left: resolve_value_origin(function, def_map, args[0]),
            middle: resolve_value_origin(function, def_map, args[1]),
            right: resolve_value_origin(function, def_map, args[2]),
        }),
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
