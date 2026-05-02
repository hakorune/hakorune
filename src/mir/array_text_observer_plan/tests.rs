use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

#[test]
fn detects_array_get_indexof_found_predicate_route() {
    let mut function = test_function(MirType::Bool);
    let block = entry_block(&mut function);
    block.add_instruction(array_get(10, 1, 2));
    block.add_instruction(const_s(11, "line"));
    block.add_instruction(indexof_call(12, 10, 11));
    block.add_instruction(const_i(13, 0));
    block.add_instruction(compare(14, CompareOp::Ge, 12, 13));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(14)),
    });

    refresh_function_array_text_observer_routes(&mut function);

    assert_eq!(function.metadata.array_text_observer_routes.len(), 1);
    let route = &function.metadata.array_text_observer_routes[0];
    assert_eq!(route.array_value(), ValueId::new(1));
    assert_eq!(route.index_value(), ValueId::new(2));
    assert_eq!(route.source_value(), ValueId::new(10));
    assert_eq!(route.observer_arg0(), ValueId::new(11));
    assert_eq!(route.observer_arg0_repr_kind(), "const_utf8");
    assert_eq!(route.observer_arg0_text(), Some("line"));
    assert_eq!(route.observer_arg0_byte_len(), Some(4));
    assert!(!route.observer_arg0_keep_live());
    assert_eq!(route.result_value(), ValueId::new(12));
    assert_eq!(route.consumer_shape(), "found_predicate");
    assert_eq!(route.publication_boundary(), "none");
    assert_eq!(route.result_repr(), "scalar_i64");
    assert!(!route.keep_get_live());
}

#[test]
fn detects_array_get_indexof_direct_scalar_route() {
    let mut function = test_function(MirType::Integer);
    let block = entry_block(&mut function);
    block.add_instruction(array_get(10, 1, 2));
    block.add_instruction(const_s(11, "needle"));
    block.add_instruction(indexof_call(12, 10, 11));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    refresh_function_array_text_observer_routes(&mut function);

    assert_eq!(function.metadata.array_text_observer_routes.len(), 1);
    let route = &function.metadata.array_text_observer_routes[0];
    assert_eq!(route.consumer_shape(), "direct_scalar");
    assert_eq!(route.observer_kind(), "indexof");
    assert_eq!(route.proof_region(), "array_get_receiver_indexof");
}

#[test]
fn marks_get_live_when_source_has_non_observer_use() {
    let mut function = test_function(MirType::Integer);
    let block = entry_block(&mut function);
    block.add_instruction(array_get(10, 1, 2));
    block.add_instruction(const_s(11, "needle"));
    block.add_instruction(indexof_call(12, 10, 11));
    block.add_instruction(len_call(13, 10));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    refresh_function_array_text_observer_routes(&mut function);

    let route = &function.metadata.array_text_observer_routes[0];
    assert!(route.keep_get_live());
}

#[test]
fn keeps_get_dead_when_source_only_feeds_same_slot_const_suffix_store() {
    let mut function = test_function(MirType::Integer);
    let block = entry_block(&mut function);
    block.add_instruction(array_get(10, 1, 2));
    block.add_instruction(const_s(11, "line"));
    block.add_instruction(indexof_call(12, 10, 11));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(13),
        src: ValueId::new(10),
    });
    block.add_instruction(const_s(14, "ln"));
    block.add_instruction(add(15, 13, 14));
    block.add_instruction(array_set(16, 1, 2, 15));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    refresh_function_array_text_observer_routes(&mut function);

    let route = &function.metadata.array_text_observer_routes[0];
    assert!(!route.keep_get_live());
}

#[test]
fn attaches_executor_contract_for_observer_conditional_suffix_store_region() {
    let mut function = test_function(MirType::Integer);
    for id in 1..=5 {
        function.blocks.insert(
            BasicBlockId::new(id),
            BasicBlock::new(BasicBlockId::new(id)),
        );
    }

    {
        let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.add_instruction(const_i(20, 0));
        block.set_terminator(jump(1));
    }
    {
        let block = function.blocks.get_mut(&BasicBlockId::new(1)).unwrap();
        block.predecessors.insert(BasicBlockId::new(0));
        block.predecessors.insert(BasicBlockId::new(4));
        block.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(21),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(20)),
                (BasicBlockId::new(4), ValueId::new(32)),
            ],
            type_hint: Some(MirType::Integer),
        });
        block.add_instruction(const_i(22, 64));
        block.add_instruction(compare(23, CompareOp::Lt, 21, 22));
        block.set_terminator(branch(23, 2, 5));
    }
    {
        let block = function.blocks.get_mut(&BasicBlockId::new(2)).unwrap();
        block.predecessors.insert(BasicBlockId::new(1));
        block.add_instruction(array_get(24, 1, 21));
        block.add_instruction(const_s(25, "line"));
        block.add_instruction(indexof_call(26, 24, 25));
        block.add_instruction(const_i(27, 0));
        block.add_instruction(compare(28, CompareOp::Ge, 26, 27));
        block.set_terminator(branch(28, 3, 4));
    }
    {
        let block = function.blocks.get_mut(&BasicBlockId::new(3)).unwrap();
        block.predecessors.insert(BasicBlockId::new(2));
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(29),
            src: ValueId::new(24),
        });
        block.add_instruction(const_s(30, "ln"));
        block.add_instruction(add(31, 29, 30));
        block.add_instruction(array_set(33, 1, 21, 31));
        block.set_terminator(jump(4));
    }
    {
        let block = function.blocks.get_mut(&BasicBlockId::new(4)).unwrap();
        block.predecessors.insert(BasicBlockId::new(2));
        block.predecessors.insert(BasicBlockId::new(3));
        block.add_instruction(const_i(34, 1));
        block.add_instruction(add(32, 21, 34));
        block.set_terminator(jump(1));
    }
    {
        let block = function.blocks.get_mut(&BasicBlockId::new(5)).unwrap();
        block.predecessors.insert(BasicBlockId::new(1));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(21)),
        });
    }

    refresh_function_array_text_observer_routes(&mut function);

    let route = &function.metadata.array_text_observer_routes[0];
    let contract = route.executor_contract().expect("executor contract");
    assert_eq!(contract.execution_mode(), "single_region_executor");
    assert!(contract.is_single_region_executor());
    assert_eq!(
        contract.consumer_capabilities(),
        vec!["compare_only", "sink_store"]
    );
    let mapping = contract.region_mapping().expect("region mapping");
    assert_eq!(mapping.array_root_value(), ValueId::new(1));
    assert_eq!(mapping.loop_index_phi_value(), ValueId::new(21));
    assert_eq!(mapping.loop_index_initial_const(), 0);
    assert_eq!(mapping.loop_bound_const(), 64);
    assert_eq!(mapping.begin_block(), BasicBlockId::new(0));
    assert_eq!(mapping.begin_to_header_block(), BasicBlockId::new(1));
    assert_eq!(mapping.observer_block(), BasicBlockId::new(2));
    assert_eq!(mapping.then_store_block(), BasicBlockId::new(3));
    assert_eq!(mapping.latch_block(), BasicBlockId::new(4));
    assert_eq!(mapping.exit_block(), BasicBlockId::new(5));
    assert_eq!(mapping.suffix_text(), "ln");
    assert_eq!(mapping.suffix_byte_len(), 2);
}

#[test]
fn marks_observer_arg_live_when_const_is_used_elsewhere() {
    let mut function = test_function(MirType::Integer);
    let block = entry_block(&mut function);
    block.add_instruction(array_get(10, 1, 2));
    block.add_instruction(const_s(11, "needle"));
    block.add_instruction(indexof_call(12, 10, 11));
    block.add_instruction(len_call(13, 11));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    refresh_function_array_text_observer_routes(&mut function);

    let route = &function.metadata.array_text_observer_routes[0];
    assert!(route.observer_arg0_keep_live());
}

fn test_function(return_type: MirType) -> MirFunction {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("ArrayBox".to_string()), MirType::Integer],
        return_type,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function.params = vec![ValueId::new(1), ValueId::new(2)];
    function
}

fn entry_block(function: &mut MirFunction) -> &mut BasicBlock {
    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
}

fn const_i(dst: u32, value: i64) -> MirInstruction {
    MirInstruction::Const {
        dst: ValueId::new(dst),
        value: ConstValue::Integer(value),
    }
}

fn const_s(dst: u32, value: &str) -> MirInstruction {
    MirInstruction::Const {
        dst: ValueId::new(dst),
        value: ConstValue::String(value.to_string()),
    }
}

fn compare(dst: u32, op: CompareOp, lhs: u32, rhs: u32) -> MirInstruction {
    MirInstruction::Compare {
        dst: ValueId::new(dst),
        op,
        lhs: ValueId::new(lhs),
        rhs: ValueId::new(rhs),
    }
}

fn array_get(dst: u32, array: u32, index: u32) -> MirInstruction {
    method_call(
        dst,
        "RuntimeDataBox",
        "get",
        array,
        vec![ValueId::new(index)],
    )
}

fn indexof_call(dst: u32, receiver: u32, needle: u32) -> MirInstruction {
    method_call(
        dst,
        "RuntimeDataBox",
        "indexOf",
        receiver,
        vec![ValueId::new(needle)],
    )
}

fn len_call(dst: u32, receiver: u32) -> MirInstruction {
    method_call(dst, "RuntimeDataBox", "length", receiver, vec![])
}

fn add(dst: u32, lhs: u32, rhs: u32) -> MirInstruction {
    MirInstruction::BinOp {
        dst: ValueId::new(dst),
        op: BinaryOp::Add,
        lhs: ValueId::new(lhs),
        rhs: ValueId::new(rhs),
    }
}

fn branch(cond: u32, then_bb: u32, else_bb: u32) -> MirInstruction {
    MirInstruction::Branch {
        condition: ValueId::new(cond),
        then_bb: BasicBlockId::new(then_bb),
        else_bb: BasicBlockId::new(else_bb),
        then_edge_args: None,
        else_edge_args: None,
    }
}

fn jump(target: u32) -> MirInstruction {
    MirInstruction::Jump {
        target: BasicBlockId::new(target),
        edge_args: None,
    }
}

fn array_set(dst: u32, array: u32, index: u32, value: u32) -> MirInstruction {
    method_call(
        dst,
        "RuntimeDataBox",
        "set",
        array,
        vec![ValueId::new(index), ValueId::new(value)],
    )
}

fn method_call(
    dst: u32,
    box_name: &str,
    method: &str,
    receiver: u32,
    args: Vec<ValueId>,
) -> MirInstruction {
    MirInstruction::Call {
        dst: Some(ValueId::new(dst)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(ValueId::new(receiver)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}
