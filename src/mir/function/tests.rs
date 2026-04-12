use super::*;
use crate::mir::{BasicBlockId, EffectMask, MirType, ValueId};

#[test]
fn test_function_creation() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer, MirType::Float],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };

    let entry_block = BasicBlockId::new(0);
    let function = MirFunction::new(signature.clone(), entry_block);

    assert_eq!(function.signature.name, "test_func");
    assert_eq!(function.entry_block, entry_block);
    assert!(function.blocks.contains_key(&entry_block));
}

#[test]
fn test_module_creation() {
    let mut module = MirModule::new("test_module".to_string());

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };

    let function = MirFunction::new(signature, BasicBlockId::new(0));
    module.add_function(function);

    assert_eq!(module.name, "test_module");
    assert!(module.get_function("main").is_some());
    assert_eq!(module.function_names().len(), 1);
}

// Legacy ValueId 割り当て仕様（LoopForm v2 導入前の想定）.
#[test]
#[ignore]
fn test_value_id_generation() {
    let signature = FunctionSignature {
        name: "test".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };

    let mut function = MirFunction::new(signature, BasicBlockId::new(0));

    let val1 = function.next_value_id();
    let val2 = function.next_value_id();
    let val3 = function.next_value_id();

    assert_eq!(val1, ValueId::new(0));
    assert_eq!(val2, ValueId::new(1));
    assert_eq!(val3, ValueId::new(2));
}

// Legacy stats API の想定（現行の拡張とはズレるためアーカイブ扱い）.
#[test]
#[ignore]
fn test_function_stats() {
    let signature = FunctionSignature {
        name: "test".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };

    let function = MirFunction::new(signature, BasicBlockId::new(0));
    let stats = function.stats();

    assert_eq!(stats.block_count, 1);
    assert_eq!(stats.instruction_count, 0);
    assert_eq!(stats.value_count, 0);
    assert!(stats.is_pure);
}
