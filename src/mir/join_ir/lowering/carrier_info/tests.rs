use super::*;
use crate::mir::ValueId;

// Helper: Create a CarrierVar for testing
fn test_carrier(name: &str, id: u32) -> CarrierVar {
    CarrierVar {
        name: name.to_string(),
        host_id: ValueId(id),
        join_id: None,                // Phase 177-STRUCT-1
        role: CarrierRole::LoopState, // Phase 227: Default to LoopState
        init: CarrierInit::FromHost,  // Phase 228: Default to FromHost
    }
}

// Helper: Create a CarrierInfo for testing
fn test_carrier_info(loop_var: &str, loop_id: u32, carriers: Vec<CarrierVar>) -> CarrierInfo {
    CarrierInfo::with_carriers(loop_var.to_string(), ValueId(loop_id), carriers)
}

#[test]
fn test_merge_from_empty() {
    // Merge empty CarrierInfo should not change anything
    let mut carrier_info = test_carrier_info("i", 5, vec![test_carrier("sum", 10)]);

    let other = test_carrier_info("j", 20, vec![]);

    carrier_info.merge_from(&other);

    assert_eq!(carrier_info.carrier_count(), 1);
    assert_eq!(carrier_info.carriers[0].name, "sum");
}

#[test]
fn test_merge_from_new_carrier() {
    // Merge a new carrier that doesn't exist yet
    let mut carrier_info = test_carrier_info("i", 5, vec![test_carrier("sum", 10)]);

    let other = test_carrier_info("j", 20, vec![test_carrier("count", 15)]);

    carrier_info.merge_from(&other);

    assert_eq!(carrier_info.carrier_count(), 2);
    // Should be sorted by name
    assert_eq!(carrier_info.carriers[0].name, "count"); // 'c' < 's'
    assert_eq!(carrier_info.carriers[1].name, "sum");
}

#[test]
fn test_merge_from_duplicate_carrier() {
    // Merge a carrier with the same name should NOT duplicate
    let mut carrier_info = test_carrier_info("i", 5, vec![test_carrier("sum", 10)]);

    let other = test_carrier_info(
        "j",
        20,
        vec![test_carrier("sum", 999)], // Same name, different ID
    );

    carrier_info.merge_from(&other);

    // Should still have only 1 carrier (no duplication)
    assert_eq!(carrier_info.carrier_count(), 1);
    assert_eq!(carrier_info.carriers[0].name, "sum");
    // Original ID should be preserved
    assert_eq!(carrier_info.carriers[0].host_id, ValueId(10));
}
