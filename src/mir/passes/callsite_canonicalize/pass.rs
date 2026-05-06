use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::extern_call_route_plan::ExternCallRouteKind;
use crate::mir::function::ClosureBodyId;
use crate::mir::ssot::closure_call::{classify_closure_call_shape, ClosureCallShape};
use crate::mir::ssot::method_call::method_call;
use crate::mir::{Callee, MirInstruction, MirModule, MirType, ValueId};

use super::helpers::{
    canonicalize_legacy_global_name, collect_const_null_sentinels, collect_const_string_literals,
    collect_known_user_boxes, known_user_box_name_from_value, parse_user_box_method_global_name,
};

/// Canonicalize call-site instructions.
///
/// Returns number of rewritten instructions.
pub fn canonicalize_callsites(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    let mut closure_bodies = std::mem::take(&mut module.metadata.closure_bodies);
    let mut next_closure_body_id = module.metadata.next_closure_body_id;
    let function_names = module.functions.keys().cloned().collect::<BTreeSet<_>>();
    let known_user_boxes = collect_known_user_boxes(module);

    for func in module.functions.values_mut() {
        let const_strings = collect_const_string_literals(func);
        let const_null_sentinels = collect_const_null_sentinels(func);
        let value_types = func.metadata.value_types.clone();

        for block in func.blocks.values_mut() {
            for inst in &mut block.instructions {
                rewritten += canonicalize_callsite_instruction(
                    inst,
                    &const_strings,
                    &const_null_sentinels,
                    &function_names,
                    &value_types,
                    &known_user_boxes,
                    &mut closure_bodies,
                    &mut next_closure_body_id,
                );
            }
            if let Some(term) = block.terminator.as_mut() {
                rewritten += canonicalize_callsite_instruction(
                    term,
                    &const_strings,
                    &const_null_sentinels,
                    &function_names,
                    &value_types,
                    &known_user_boxes,
                    &mut closure_bodies,
                    &mut next_closure_body_id,
                );
            }
        }
    }

    module.metadata.closure_bodies = closure_bodies;
    module.metadata.next_closure_body_id = next_closure_body_id;

    rewritten
}

fn canonicalize_callsite_instruction(
    inst: &mut MirInstruction,
    const_strings: &BTreeMap<ValueId, String>,
    const_null_sentinels: &BTreeSet<ValueId>,
    function_names: &BTreeSet<String>,
    value_types: &BTreeMap<ValueId, MirType>,
    known_user_boxes: &BTreeSet<String>,
    closure_bodies: &mut BTreeMap<ClosureBodyId, Vec<ASTNode>>,
    next_closure_body_id: &mut ClosureBodyId,
) -> usize {
    match inst {
        MirInstruction::NewClosure { body_id, body, .. }
            if body_id.is_none() && !body.is_empty() =>
        {
            let id = *next_closure_body_id;
            *next_closure_body_id = next_closure_body_id.saturating_add(1);
            closure_bodies.insert(id, body.clone());
            *body_id = Some(id);
            body.clear();
            1
        }
        MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Closure {
                    params,
                    captures,
                    me_capture,
                }),
            args,
            ..
        } => match classify_closure_call_shape(*dst, args) {
            ClosureCallShape::CanonicalCtor => {
                let rewritten = MirInstruction::NewClosure {
                    dst: (*dst).expect("canonical closure ctor must have dst"),
                    params: params.clone(),
                    body_id: None,
                    body: vec![],
                    captures: captures.clone(),
                    me: *me_capture,
                };
                *inst = rewritten;
                1
            }
            ClosureCallShape::MissingDst | ClosureCallShape::RuntimeArgs => 0,
        },
        MirInstruction::Call {
            dst,
            func,
            callee: None,
            args,
            effects,
        } => {
            if let Some(name) = const_strings.get(func) {
                let canonical_name =
                    canonicalize_legacy_global_name(name, args.len(), function_names);
                let rewritten = MirInstruction::Call {
                    dst: *dst,
                    func: ValueId::INVALID,
                    callee: Some(Callee::Global(canonical_name)),
                    args: args.clone(),
                    effects: *effects,
                };
                *inst = rewritten;
                1
            } else {
                0
            }
        }
        MirInstruction::Call {
            dst,
            func: _,
            callee: Some(Callee::Global(name)),
            args,
            effects,
        } if name == "BuildBox.emit_program_json_v0/2"
            && args.len() == 2
            && const_null_sentinels.contains(&args[1]) =>
        {
            let rewritten = MirInstruction::Call {
                dst: *dst,
                func: ValueId::INVALID,
                callee: Some(Callee::Extern(
                    ExternCallRouteKind::Stage1EmitProgramJson
                        .symbol()
                        .to_string(),
                )),
                args: vec![args[0]],
                effects: *effects,
            };
            *inst = rewritten;
            1
        }
        MirInstruction::Call {
            dst,
            func: _,
            callee: Some(Callee::Global(name)),
            args,
            effects,
        } if name == "BuildBox._emit_program_json_from_scan_src/1" && args.len() == 1 => {
            let rewritten = MirInstruction::Call {
                dst: *dst,
                func: ValueId::INVALID,
                callee: Some(Callee::Extern(
                    ExternCallRouteKind::Stage1EmitProgramJson
                        .symbol()
                        .to_string(),
                )),
                args: vec![args[0]],
                effects: *effects,
            };
            *inst = rewritten;
            1
        }
        MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    certainty,
                    box_kind,
                }),
            args,
            effects,
            ..
        } => {
            let Some(known_box_name) =
                known_user_box_name_from_value(value_types, known_user_boxes, *receiver)
            else {
                return 0;
            };
            if box_name != "RuntimeDataBox" && box_name != known_box_name {
                return 0;
            }
            if box_name == known_box_name
                && *certainty == TypeCertainty::Known
                && *box_kind == CalleeBoxKind::UserDefined
            {
                return 0;
            }
            *inst = method_call(
                *dst,
                *receiver,
                known_box_name.to_string(),
                method.clone(),
                args.clone(),
                *effects,
                TypeCertainty::Known,
                CalleeBoxKind::UserDefined,
            );
            1
        }
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            dst,
            args,
            effects,
            ..
        } => {
            let canonical_name = canonicalize_legacy_global_name(name, args.len(), function_names);
            let rewritten_name = canonical_name != *name;
            if rewritten_name {
                *name = canonical_name;
            }
            let Some((box_name, method_name, explicit_arity)) =
                parse_user_box_method_global_name(name)
            else {
                return usize::from(rewritten_name);
            };
            let Some(receiver) = args.first().copied() else {
                return usize::from(rewritten_name);
            };
            let Some(known_box_name) =
                known_user_box_name_from_value(value_types, known_user_boxes, receiver)
            else {
                return usize::from(rewritten_name);
            };
            if box_name != known_box_name || explicit_arity != args.len().saturating_sub(1) {
                return usize::from(rewritten_name);
            }
            *inst = method_call(
                *dst,
                receiver,
                known_box_name.to_string(),
                method_name.to_string(),
                args[1..].to_vec(),
                *effects,
                TypeCertainty::Known,
                CalleeBoxKind::UserDefined,
            );
            1
        }
        MirInstruction::Call { .. } => 0,
        _ => 0,
    }
}
