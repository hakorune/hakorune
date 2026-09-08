//! Physical identity checks for the v2 projection planner's borrowed index.
use super::*;
use crate::mir::{BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirModule, MirType};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

fn function(name: &str, params: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.into(),
            params: vec![MirType::Integer; params],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

#[test]
fn map_literal_index_rejects_duplicate_ssa_and_formal_definitions() {
    for formal_collision in [false, true] {
        let mut module = MirModule::new("duplicate".into());
        let mut body = function("main", usize::from(formal_collision));
        let dst = if formal_collision {
            body.params[0]
        } else {
            ValueId::new(7)
        };
        let block = body.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.add_instruction(MirInstruction::Const {
            dst,
            value: ConstValue::Integer(1),
        });
        if !formal_collision {
            block.add_instruction(MirInstruction::Const {
                dst,
                value: ConstValue::Bool(true),
            });
        }
        module.add_function(body);
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        let error = MapBodyIndex::from_view(&view).err().unwrap();
        assert!(error.contains("duplicate-value-definition"), "{error}");
    }
}

#[test]
fn map_literal_index_borrows_the_exact_instruction_and_operation_site() {
    let mut module = MirModule::new("map-index".into());
    let mut body = function("main", 0);
    let block = body.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(7),
        target: ConstructionTarget::IntrinsicMap,
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(8),
        value: ConstValue::Float(-0.0),
    });
    module.add_function(body);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    assert_eq!(
        index.map_operations[&("main", 0, 0)],
        MapOperationKind::Allocate
    );
    let Producer::Instruction(inst) = index.producer(("main", ValueId::new(8))).unwrap() else {
        panic!("constant is not a formal")
    };
    assert!(std::ptr::eq(
        inst,
        &module.functions["main"].blocks[&BasicBlockId::new(0)].instructions[1]
    ));
    assert!(index.producer(("main", ValueId::new(99))).is_err());
    assert!(index.incoming_actuals("main", 0).is_err());
}

#[test]
fn map_literal_index_uses_canonical_definition_and_all_incoming_actuals() {
    let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "stash", 1);
    let mut module = MirModule::new("formal-index".into());
    let mut callee = function(&key.mir_symbol_projection(), 1);
    let formal = callee.params[0];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(20),
        target: ConstructionTarget::IntrinsicMap,
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(21),
        value: ConstValue::String("v".into()),
    });
    entry.add_instruction(MirInstruction::MapLiteralEntryWrite {
        receiver: ValueId::new(20),
        key: ValueId::new(21),
        value: formal,
    });
    module
        .add_cataloged_box_method(key.clone(), callee)
        .unwrap();
    for (name, literal) in [
        ("integer_caller", ConstValue::Integer(1)),
        ("bool_caller", ConstValue::Bool(true)),
    ] {
        let mut body = function(name, 0);
        let block = body.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(7),
            value: literal,
        });
        block.add_instruction(MirInstruction::call(
            Some(ValueId::new(8)),
            crate::mir::Callee::Global(key.canonical_global_target_v1().unwrap()),
            vec![ValueId::new(7)],
            EffectMask::PURE,
        ));
        module.add_function(body);
    }
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let index = MapBodyIndex::from_view(&view).unwrap();
    let target = module.canonical_callable_definition_symbol(&key).unwrap();
    assert!(matches!(
        index.producer((target, formal)).unwrap(),
        Producer::Formal(0)
    ));
    let demand = index.map_value_demands().unwrap();
    assert_eq!(demand.len(), 3);
    assert!(demand.contains(&(target, formal)));
    assert!(demand.contains(&("bool_caller", ValueId::new(7))));
    assert!(demand.contains(&("integer_caller", ValueId::new(7))));
    assert_eq!(
        index.incoming_actuals(target, 0).unwrap(),
        vec![
            ("bool_caller", ValueId::new(7)),
            ("integer_caller", ValueId::new(7))
        ]
    );
}
