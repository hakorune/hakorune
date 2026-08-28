use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::function::ClosureBodyId;
use crate::mir::ssot::closure_call::{classify_closure_call_shape, ClosureCallShape};
use crate::mir::ssot::method_call::method_call;
use crate::mir::{Callee, MirInstruction, MirModule, MirType, ValueId};

use super::helpers::{collect_known_user_boxes, known_user_box_name_from_value};
use super::receiver_operand::rewrite_cfg_stable_receiver_operands;

/// Canonicalize call-site instructions.
///
/// Returns number of rewritten instructions.
pub fn canonicalize_callsites(module: &mut MirModule) -> usize {
    canonicalize_callsites_for_site(module)
}

pub(super) fn canonicalize_callsites_for_site(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    let mut closure_bodies = std::mem::take(&mut module.metadata.closure_bodies);
    let mut next_closure_body_id = module.metadata.next_closure_body_id;
    let known_user_boxes = collect_known_user_boxes(module);

    for func in module.functions.values_mut() {
        let value_types = func.metadata.value_types.clone();

        for block in func.blocks.values_mut() {
            for inst in &mut block.instructions {
                rewritten += canonicalize_callsite_instruction(
                    inst,
                    &value_types,
                    &known_user_boxes,
                    &mut closure_bodies,
                    &mut next_closure_body_id,
                );
            }
            if let Some(term) = block.terminator.as_mut() {
                rewritten += canonicalize_callsite_instruction(
                    term,
                    &value_types,
                    &known_user_boxes,
                    &mut closure_bodies,
                    &mut next_closure_body_id,
                );
            }
        }
        rewritten += rewrite_cfg_stable_receiver_operands(func);
    }

    module.metadata.closure_bodies = closure_bodies;
    module.metadata.next_closure_body_id = next_closure_body_id;

    rewritten
}

fn canonicalize_callsite_instruction(
    inst: &mut MirInstruction,
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
        // A typed Global is already an admitted target.  This post-pass is
        // deliberately not a second resolver: it must not parse a display
        // name, append an arity, or turn a Global into a Method.
        MirInstruction::Call {
            callee: Some(Callee::Global(_)),
            ..
        } => 0,
        MirInstruction::Call { .. } => 0,
        _ => 0,
    }
}
