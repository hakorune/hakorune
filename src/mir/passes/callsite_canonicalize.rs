//! MIR callsite canonicalization pass.
//!
//! Post-RCL-3:
//! - `MirInstruction::BoxCall` / `MirInstruction::ExternCall` are retired.
//! - pass keeps MCL-5 compatibility rewrite only:
//!   `Call(callee=None, func=<const-string>) -> Call(callee=Global, func=INVALID)`.
//! - NCL-0 keeps closure creation canonical as `NewClosure`:
//!   `Call(callee=Closure, dst=Some(_)) -> NewClosure`.
//! - NCL-1 keeps `NewClosure` thin by externalizing inline bodies:
//!   `NewClosure{body=[...], body_id=None} -> NewClosure{body=[], body_id=Some(id)}`.
//! - NCL-2 fixes closure-call shape boundary:
//!   only `dst=Some(_) + args=[]` is canonicalized to `NewClosure`.
//! - UCM-1 canonicalizes known user-box receiver methods onto
//!   `Call(callee=Method{certainty=Known, box_kind=UserDefined})` so later
//!   thin-entry consumers can bind physical entries without backend-local
//!   receiver guessing.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::ssot::closure_call::{classify_closure_call_shape, ClosureCallShape};
use crate::mir::ssot::method_call::method_call;
use crate::mir::{Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId};

/// Canonicalize call-site instructions.
///
/// Returns number of rewritten instructions.
pub fn canonicalize_callsites(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    let mut closure_bodies = std::mem::take(&mut module.metadata.closure_bodies);
    let mut next_closure_body_id = module.metadata.next_closure_body_id;
    let function_names = module.functions.keys().cloned().collect::<BTreeSet<_>>();
    let known_user_boxes = collect_known_user_boxes(module);

    for (_func_name, func) in &mut module.functions {
        let const_strings = collect_const_string_literals(func);
        let value_types = func.metadata.value_types.clone();

        for (_bbid, block) in &mut func.blocks {
            for inst in &mut block.instructions {
                rewritten += canonicalize_callsite_instruction(
                    inst,
                    &const_strings,
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
    function_names: &BTreeSet<String>,
    value_types: &BTreeMap<ValueId, MirType>,
    known_user_boxes: &BTreeSet<String>,
    closure_bodies: &mut BTreeMap<crate::mir::function::ClosureBodyId, Vec<crate::ast::ASTNode>>,
    next_closure_body_id: &mut crate::mir::function::ClosureBodyId,
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

fn canonicalize_legacy_global_name(
    name: &str,
    args_len: usize,
    function_names: &BTreeSet<String>,
) -> String {
    if has_explicit_arity(name) {
        return name.to_string();
    }
    if !name.contains('.') {
        return name.to_string();
    }
    let suffixed = format!("{name}/{args_len}");
    if function_names.contains(&suffixed) {
        return suffixed;
    }
    name.to_string()
}

fn has_explicit_arity(name: &str) -> bool {
    matches!(
        name.rsplit_once('/'),
        Some((_base, arity)) if arity.chars().all(|c| c.is_ascii_digit())
    )
}

fn collect_const_string_literals(func: &MirFunction) -> BTreeMap<ValueId, String> {
    let mut out = BTreeMap::new();
    for block in func.blocks.values() {
        for inst in &block.instructions {
            if let MirInstruction::Const {
                dst,
                value: ConstValue::String(s),
            } = inst
            {
                out.insert(*dst, s.clone());
            }
        }
    }
    out
}

fn collect_known_user_boxes(module: &MirModule) -> BTreeSet<String> {
    module
        .metadata
        .user_box_decls
        .keys()
        .chain(module.metadata.user_box_field_decls.keys())
        .cloned()
        .collect()
}

fn known_user_box_name_from_value<'a>(
    value_types: &'a BTreeMap<ValueId, MirType>,
    known_user_boxes: &BTreeSet<String>,
    value: ValueId,
) -> Option<&'a str> {
    let MirType::Box(box_name) = value_types.get(&value)? else {
        return None;
    };
    if known_user_boxes.contains(box_name) {
        Some(box_name.as_str())
    } else {
        None
    }
}

fn parse_user_box_method_global_name(name: &str) -> Option<(&str, &str, usize)> {
    let (base, arity) = name.rsplit_once('/')?;
    let explicit_arity = arity.parse::<usize>().ok()?;
    let (box_name, method_name) = base.rsplit_once('.')?;
    Some((box_name, method_name, explicit_arity))
}

#[cfg(test)]
mod tests {
    use super::canonicalize_callsites;
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlockId, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType, UserBoxFieldDecl, ValueId,
    };

    #[test]
    fn mcl5_rewrites_legacy_call_with_const_string_func_to_global_callee() {
        let mut module = MirModule::new("mcl5".to_string());
        let signature = FunctionSignature {
            name: "mcl5/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");

        block.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: crate::mir::ConstValue::String("RewriteKnownMini.run/1".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(3)),
            func: ValueId(1),
            callee: None,
            args: vec![ValueId(2)],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(3)),
        });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 1);

        let inst = &module
            .get_function("mcl5/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[2];

        assert!(matches!(
            inst,
            MirInstruction::Call {
                dst,
                func,
                callee: Some(Callee::Global(name)),
                args,
                effects,
            } if *dst == Some(ValueId(3))
                && *func == ValueId::INVALID
                && name == "RewriteKnownMini.run/1"
                && args == &vec![ValueId(2)]
                && *effects == EffectMask::PURE
        ));
    }

    #[test]
    fn mcl5_suffixes_unsuffixed_qualified_global_name_when_matching_arity_exists() {
        let mut module = MirModule::new("mcl5_suffix".to_string());
        let callee_sig = FunctionSignature {
            name: "RewriteKnownMini.run/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        module.add_function(MirFunction::new(callee_sig, BasicBlockId(0)));

        let signature = FunctionSignature {
            name: "mcl5_suffix/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));
        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");

        block.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: crate::mir::ConstValue::String("RewriteKnownMini.run".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(3)),
            func: ValueId(1),
            callee: None,
            args: vec![ValueId(2)],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(3)),
        });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 1);

        let inst = &module
            .get_function("mcl5_suffix/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[2];

        assert!(matches!(
            inst,
            MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                ..
            } if name == "RewriteKnownMini.run/1"
        ));
    }

    #[test]
    fn mcl5_suffixes_existing_global_callee_when_matching_arity_exists() {
        let mut module = MirModule::new("mcl5_global_suffix".to_string());
        let callee_sig = FunctionSignature {
            name: "RewriteKnownMini.run/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        module.add_function(MirFunction::new(callee_sig, BasicBlockId(0)));

        let signature = FunctionSignature {
            name: "mcl5_global_suffix/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));
        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");

        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("RewriteKnownMini.run".to_string())),
            args: vec![ValueId(2)],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(3)),
        });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 1);

        let inst = &module
            .get_function("mcl5_global_suffix/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[1];

        assert!(matches!(
            inst,
            MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                ..
            } if name == "RewriteKnownMini.run/1"
        ));
    }

    #[test]
    fn mcl4_no_legacy_callsite_variants_after_rcl3() {
        let mut module = MirModule::new("mcl4".to_string());
        let signature = FunctionSignature {
            name: "mcl4/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::IO,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "StringBox".to_string(),
                method: "id".to_string(),
                receiver: Some(ValueId(2)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId(3)],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("env.console.log".to_string())),
            args: vec![ValueId(10)],
            effects: EffectMask::IO,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return { value: None });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 0, "canonical calls should remain unchanged");

        let instructions = &module
            .get_function("mcl4/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions;

        assert!(matches!(
            &instructions[0],
            MirInstruction::Call {
                callee: Some(Callee::Method { .. }),
                ..
            }
        ));
        assert!(matches!(
            &instructions[1],
            MirInstruction::Call {
                callee: Some(Callee::Extern(_)),
                ..
            }
        ));
    }

    #[test]
    fn ncl0_rewrites_call_closure_to_newclosure() {
        let mut module = MirModule::new("ncl0".to_string());
        let signature = FunctionSignature {
            name: "ncl0/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(9)),
            func: ValueId::INVALID,
            callee: Some(Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![("outer".to_string(), ValueId(3))],
                me_capture: Some(ValueId(4)),
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return { value: None });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 1);

        let inst = &module
            .get_function("ncl0/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[0];
        assert!(matches!(
            inst,
            MirInstruction::NewClosure {
                dst,
                params,
                body_id,
                body,
                captures,
                me
            } if *dst == ValueId(9)
                && params == &vec!["x".to_string()]
                && *body_id == None
                && body.is_empty()
                && captures == &vec![("outer".to_string(), ValueId(3))]
                && *me == Some(ValueId(4))
        ));
    }

    #[test]
    fn ncl0_does_not_rewrite_closure_call_with_runtime_args() {
        let mut module = MirModule::new("ncl0_args".to_string());
        let signature = FunctionSignature {
            name: "ncl0_args/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(9)),
            func: ValueId::INVALID,
            callee: Some(Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![("outer".to_string(), ValueId(3))],
                me_capture: None,
            }),
            args: vec![ValueId(8)],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return { value: None });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 0);
        let inst = &module
            .get_function("ncl0_args/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[0];
        assert!(matches!(
            inst,
            MirInstruction::Call {
                callee: Some(Callee::Closure { .. }),
                args,
                ..
            } if args == &vec![ValueId(8)]
        ));
    }

    #[test]
    fn ncl2_does_not_rewrite_closure_call_without_dst() {
        let mut module = MirModule::new("ncl2_missing_dst".to_string());
        let signature = FunctionSignature {
            name: "ncl2_missing_dst/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");
        block.instructions.push(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![],
                me_capture: None,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return { value: None });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 0);
        let inst = &module
            .get_function("ncl2_missing_dst/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[0];
        assert!(matches!(
            inst,
            MirInstruction::Call {
                dst: None,
                callee: Some(Callee::Closure { .. }),
                args,
                ..
            } if args.is_empty()
        ));
    }

    #[test]
    fn ncl1_externalizes_inline_newclosure_body() {
        let mut module = MirModule::new("ncl1".to_string());
        let signature = FunctionSignature {
            name: "ncl1/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");
        let inline_body = vec![crate::ast::ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: Span::unknown(),
        }];
        block.instructions.push(MirInstruction::NewClosure {
            dst: ValueId(11),
            params: vec!["x".to_string()],
            body_id: None,
            body: inline_body.clone(),
            captures: vec![],
            me: None,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return { value: None });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 1);

        let inst = &module
            .get_function("ncl1/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[0];

        let body_id = match inst {
            MirInstruction::NewClosure { body_id, body, .. } => {
                assert!(body.is_empty(), "inline body must be externalized");
                body_id.expect("body_id must be assigned")
            }
            _ => panic!("expected NewClosure after canonicalization"),
        };

        assert_eq!(
            module.metadata.closure_bodies.get(&body_id),
            Some(&inline_body)
        );
    }

    #[test]
    fn ucm1_rewrites_runtime_data_union_method_call_to_known_user_box_method() {
        let mut module = MirModule::new("ucm1_method".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Counter".to_string(), vec!["value".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Counter".to_string(),
            vec![UserBoxFieldDecl {
                name: "value".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: false,
            }],
        );

        let signature = FunctionSignature {
            name: "ucm1_method/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));
        func.metadata
            .value_types
            .insert(ValueId(1), MirType::Box("Counter".to_string()));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "step".to_string(),
                receiver: Some(ValueId(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(3)),
        });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 1);

        let inst = &module
            .get_function("ucm1_method/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[0];
        assert!(matches!(
            inst,
            MirInstruction::Call {
                dst: Some(ValueId(3)),
                func,
                callee: Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::UserDefined,
                }),
                args,
                effects,
            } if *func == ValueId::INVALID
                && box_name == "Counter"
                && method == "step"
                && *receiver == ValueId(1)
                && args.is_empty()
                && *effects == EffectMask::PURE
        ));
    }

    #[test]
    fn ucm1_rewrites_user_box_global_method_call_to_canonical_method_shape() {
        let mut module = MirModule::new("ucm1_global".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Counter".to_string(), vec!["value".to_string()]);

        let signature = FunctionSignature {
            name: "ucm1_global/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));
        func.metadata
            .value_types
            .insert(ValueId(1), MirType::Box("Counter".to_string()));

        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Counter.step/0".to_string())),
            args: vec![ValueId(1)],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 1);

        let inst = &module
            .get_function("ucm1_global/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[0];
        assert!(matches!(
            inst,
            MirInstruction::Call {
                dst: Some(ValueId(4)),
                func,
                callee: Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::UserDefined,
                }),
                args,
                effects,
            } if *func == ValueId::INVALID
                && box_name == "Counter"
                && method == "step"
                && *receiver == ValueId(1)
                && args.is_empty()
                && *effects == EffectMask::PURE
        ));
    }
}
