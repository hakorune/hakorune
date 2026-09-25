use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::function::ClosureBodyId;
use crate::mir::ssot::closure_call::{classify_closure_call_shape, ClosureCallShape};
use crate::mir::{Callee, MirInstruction, MirModule};

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

    for func in module.functions.values_mut() {
        for block in func.blocks.values_mut() {
            for inst in &mut block.instructions {
                rewritten += canonicalize_callsite_instruction(
                    inst,
                    &mut closure_bodies,
                    &mut next_closure_body_id,
                );
            }
            if let Some(term) = block.terminator.as_mut() {
                rewritten += canonicalize_callsite_instruction(
                    term,
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
        MirInstruction::LegacyCallV0 {
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
        // A typed Global is already an admitted target.  This post-pass is
        // deliberately not a second resolver: it must not parse a display
        // name, append an arity, or turn a Global into a Method.
        MirInstruction::LegacyCallV0 {
            callee: Some(Callee::Global(_)),
            ..
        } => 0,
        MirInstruction::LegacyCallV0 { .. } => 0,
        _ => 0,
    }
}
