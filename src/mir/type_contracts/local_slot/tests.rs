use super::*;
use crate::mir::function::LocalContractWriteKind;
use crate::mir::MirCompiler;
use crate::mir::{BasicBlockId, BindingId, EffectMask, FunctionSignature, MirType, ValueId};
use crate::parser::NyashParser;

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    let _ = std::panic::catch_unwind(|| init_global_ring0(default_ring0()));
}

fn function_with_local_contract() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.local/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let slot = LocalSlotId::from(BindingId::new(0));
    register_local_slot_contract(&mut function, slot, "x", "u8").unwrap();
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::LocalContractWrite {
            dst: ValueId::new(1),
            src: ValueId::new(0),
            local_slot_id: slot,
            write_kind: LocalContractWriteKind::Init,
        });
    function
}

#[test]
fn validates_typed_carrier_against_write_sites() {
    let function = function_with_local_contract();
    assert!(validate_local_slot_contracts(&function).is_ok());

    let mut missing_write = function.clone();
    missing_write
        .get_block_mut(missing_write.entry_block)
        .unwrap()
        .instructions
        .clear();
    assert!(validate_local_slot_contracts(&missing_write)
        .unwrap_err()
        .contains(LOCAL_CONTRACT_WRITE_SITE_MISSING_TAG));

    let mut duplicate = function.clone();
    duplicate
        .metadata
        .local_slot_contracts
        .push(duplicate.metadata.local_slot_contracts[0].clone());
    assert!(validate_local_slot_contracts(&duplicate)
        .unwrap_err()
        .contains(LOCAL_CONTRACT_DUPLICATE_SLOT_TAG));
}

#[test]
fn source_if_and_loop_writes_preserve_one_local_slot() {
    ensure_ring0_initialized();
    let source = r#"
static box Main {
  main() {
    local x: i64 = 0
    if x == 0 {
      x = 1
    } else {
      x = 2
    }
    loop (x < 3) {
      x = x + 1
    }
    return x
  }
}
"#;
    let ast = NyashParser::parse_from_string(source).unwrap();
    let result = MirCompiler::with_options(false)
        .compile_with_source(ast, Some("local-contract-if-loop.hako"))
        .unwrap();
    let function = result
        .module
        .functions
        .values()
        .find(|function| !function.metadata.local_slot_contracts.is_empty())
        .expect("function with local contract");
    assert_eq!(function.metadata.local_slot_contracts.len(), 1);
    let slot = function.metadata.local_slot_contracts[0].local_slot_id;
    let writes = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::LocalContractWrite { local_slot_id, .. } => Some(*local_slot_id),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(writes.len() >= 3, "writes={writes:?}");
    assert!(writes.iter().all(|observed| *observed == slot));
    assert!(
        !function.metadata.local_identity_evidence.is_empty(),
        "if/loop local publication must carry rebuilt identity evidence"
    );
    assert!(function
        .metadata
        .local_identity_evidence
        .iter()
        .all(|evidence| evidence.local_slot_id == slot));
    assert!(validate_local_slot_contracts(function).is_ok());
}

#[test]
fn optimizer_preserves_runtime_contract_write() {
    ensure_ring0_initialized();
    let source = r#"
static box Main {
  main() {
    local x: u8 = 255
    return x
  }
}
"#;
    let ast = NyashParser::parse_from_string(source).unwrap();
    let result = MirCompiler::with_options(true)
        .compile_with_source(ast, Some("local-contract-optimizer.hako"))
        .unwrap();
    let function = result
        .module
        .functions
        .values()
        .find(|function| !function.metadata.local_slot_contracts.is_empty())
        .expect("function with local contract");
    assert!(function.blocks.values().any(|block| block
        .instructions
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::LocalContractWrite { .. }))));
    assert!(validate_local_slot_contracts(function).is_ok());
}

#[test]
fn shadowed_exact_locals_keep_distinct_contract_slots() {
    ensure_ring0_initialized();
    let source = r#"
static box Main {
  main() {
    local x: i64 = 1
    if true {
      local x: i64 = 2
      x = 3
    }
    x = 4
    return x
  }
}
"#;
    let ast = NyashParser::parse_from_string(source).unwrap();
    let result = MirCompiler::with_options(false)
        .compile_with_source(ast, Some("local-contract-shadow.hako"))
        .unwrap();
    let function = result
        .module
        .functions
        .values()
        .find(|function| function.metadata.local_slot_contracts.len() == 2)
        .expect("outer and shadow local contracts");
    let slots = function
        .metadata
        .local_slot_contracts
        .iter()
        .map(|contract| contract.local_slot_id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(slots.len(), 2);
    let write_slots = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::LocalContractWrite { local_slot_id, .. } => Some(*local_slot_id),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(slots.iter().all(|slot| write_slots.contains(slot)));
    assert!(validate_local_slot_contracts(function).is_ok());
}

#[test]
fn contract_initializer_evaluates_call_once() {
    ensure_ring0_initialized();
    let source = r#"
static box Helper {
  value() { return 7 }
}
static box Main {
  main() {
    local x: i64 = Helper.value()
    return x
  }
}
"#;
    let ast = NyashParser::parse_from_string(source).unwrap();
    let result = MirCompiler::with_options(false)
        .compile_with_source(ast, Some("local-contract-rhs-once.hako"))
        .unwrap();
    let function = result
        .module
        .functions
        .values()
        .find(|function| !function.metadata.local_slot_contracts.is_empty())
        .expect("function with local contract");
    let calls = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count();
    let writes = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter(|instruction| matches!(instruction, MirInstruction::LocalContractWrite { .. }))
        .count();
    assert_eq!(calls, 1);
    assert_eq!(writes, 1);
}
