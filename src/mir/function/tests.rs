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

#[test]
fn selected_dynamic_metadata_pair_observation_is_linear_and_fail_closed() {
    let mut metadata = FunctionMetadata::default();
    assert!(matches!(
        metadata.selected_dynamic_metadata_observation(),
        DynamicV2MetadataPairObservation::Ordinary
    ));
    let ordinary_clone = metadata.clone();
    assert!(matches!(
        ordinary_clone.selected_dynamic_metadata_observation(),
        DynamicV2MetadataPairObservation::Ordinary
    ));

    metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    assert!(matches!(
        metadata.selected_dynamic_metadata_observation(),
        DynamicV2MetadataPairObservation::Partial
    ));

    metadata
        .install_dynamic_v2_aot_metadata_for_test(
            crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(
            ),
        )
        .expect("admission install");
    assert!(matches!(
        metadata.selected_dynamic_metadata_observation(),
        DynamicV2MetadataPairObservation::Selected { .. }
    ));

    let scrubbed = metadata.clone();
    assert!(matches!(
        scrubbed.selected_dynamic_metadata_observation(),
        DynamicV2MetadataPairObservation::Scrubbed
    ));

    let mut admission_only = FunctionMetadata::default();
    admission_only
        .install_dynamic_v2_aot_metadata_for_test(
            crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(
            ),
        )
        .expect("admission-only install");
    assert!(matches!(
        admission_only.selected_dynamic_metadata_observation(),
        DynamicV2MetadataPairObservation::Partial
    ));
}

#[test]
fn checked_function_publication_rejects_duplicate_without_replacement() {
    let mut module = MirModule::new("checked-publication".to_string());
    let first = MirFunction::new(
        FunctionSignature {
            name: "same/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(1),
    );
    let duplicate = MirFunction::new(first.signature.clone(), BasicBlockId::new(2));

    module.try_add_function(first).unwrap();
    let error = module.try_add_function(duplicate).unwrap_err();

    assert_eq!(error.function_name, "same/0");
    assert_eq!(
        module.get_function("same/0").unwrap().entry_block,
        BasicBlockId::new(1)
    );
}

#[test]
fn atomic_function_batch_rejects_before_any_function_is_inserted() {
    let mut module = MirModule::new("atomic-publication".to_string());
    let signature = FunctionSignature {
        name: "same/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let first = MirFunction::new(signature.clone(), BasicBlockId::new(1));
    let duplicate = MirFunction::new(signature, BasicBlockId::new(2));

    let error = module
        .try_add_functions_atomic(vec![first, duplicate])
        .unwrap_err();

    assert_eq!(error.function_name, "same/0");
    assert!(module.functions.is_empty());
}

#[test]
fn atomic_function_batch_preserves_existing_module_on_late_collision() {
    let mut module = MirModule::new("atomic-existing".to_string());
    let signature = |name: &str| FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    module.add_function(MirFunction::new(
        signature("existing/0"),
        BasicBlockId::new(1),
    ));
    let fresh = MirFunction::new(signature("fresh/0"), BasicBlockId::new(2));
    let collision = MirFunction::new(signature("existing/0"), BasicBlockId::new(3));

    let error = module
        .try_add_functions_atomic(vec![fresh, collision])
        .unwrap_err();

    assert_eq!(error.function_name, "existing/0");
    assert_eq!(module.functions.len(), 1);
    assert!(module.get_function("fresh/0").is_none());
    assert_eq!(
        module.get_function("existing/0").unwrap().entry_block,
        BasicBlockId::new(1)
    );
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
