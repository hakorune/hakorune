/*!
 * MIR CFG Extractor - Extract Control Flow Graph information for analysis
 *
 * Phase 154: Provides CFG data to hako_check for dead block detection
 */

use super::{MirFunction, MirInstruction, MirModule};
use serde_json::{json, Value};

/// Extract CFG information from MIR module as JSON
///
/// Output format:
/// ```json
/// {
///   "functions": [
///     {
///       "name": "Main.main/0",
///       "entry_block": 0,
///       "blocks": [
///         {
///           "id": 0,
///           "reachable": true,
///           "successors": [1, 2],
///           "terminator": "Branch"
///         }
///       ]
///     }
///   ]
/// }
/// ```
pub fn extract_cfg_info(module: &MirModule) -> Value {
    let mut functions = Vec::new();

    for (_func_id, function) in &module.functions {
        functions.push(extract_function_cfg(function));
    }

    json!({
        "functions": functions
    })
}

/// Extract CFG info for a single function
fn extract_function_cfg(function: &MirFunction) -> Value {
    let mut blocks = Vec::new();

    for (block_id, block) in &function.blocks {
        // Extract successor IDs
        let successors: Vec<u32> = block.successors.iter().map(|id| id.0).collect();

        // Determine terminator type
        let terminator_name = match &block.terminator {
            Some(inst) => terminator_to_string(inst),
            None => "None".to_string(),
        };

        blocks.push(json!({
            "id": block_id.0,
            "reachable": block.reachable,
            "successors": successors,
            "terminator": terminator_name
        }));
    }

    // Sort blocks by ID for deterministic output
    blocks.sort_by_key(|b| b["id"].as_u64().unwrap_or(0));

    json!({
        "name": function.signature.name,
        "entry_block": function.entry_block.0,
        "blocks": blocks
    })
}

/// Convert terminator instruction to string name
fn terminator_to_string(inst: &MirInstruction) -> String {
    match inst {
        MirInstruction::Branch { .. } => "Branch".to_string(),
        MirInstruction::Jump { .. } => "Jump".to_string(),
        MirInstruction::Return { .. } => "Return".to_string(),
        _ => "Unknown".to_string(),
    }
}

// TODO: These tests need to be updated to use the new MirModule/MirFunction API
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirModule, FunctionSignature, MirType, EffectMask};
//     use std::collections::BTreeMap;
//
//     #[test]
//     fn test_extract_simple_cfg() {
//         let mut module = MirModule::new("test");
//
//         // Create simple function with 2 blocks
//         let signature = FunctionSignature {
//             name: "test_fn".to_string(),
//             params: vec![],
//             return_type: MirType::Void,
//             effects: EffectMask::empty(),
//         };
//         let mut function = MirFunction::new(signature, BasicBlockId(0));
//
//         let mut block0 = BasicBlock::new(BasicBlockId(0));
//         block0.reachable = true;
//         block0.successors.insert(BasicBlockId(1));
//         block0.terminator = Some(MirInstruction::Jump {
//             target: BasicBlockId(1),
//         });
//
//         let mut block1 = BasicBlock::new(BasicBlockId(1));
//         block1.reachable = true;
//         block1.terminator = Some(MirInstruction::Return { value: None });
//
//         function.blocks.insert(BasicBlockId(0), block0);
//         function.blocks.insert(BasicBlockId(1), block1);
//
//         module.functions.insert("test_fn".to_string(), function);
//
//         // Extract CFG
//         let cfg = extract_cfg_info(&module);
//
//         // Verify structure
//         assert!(cfg["functions"].is_array());
//         let functions = cfg["functions"].as_array().unwrap();
//         assert_eq!(functions.len(), 1);
//
//         let func = &functions[0];
//         assert_eq!(func["name"], "test_fn");
//         assert_eq!(func["entry_block"], 0);
//
//         let blocks = func["blocks"].as_array().unwrap();
//         assert_eq!(blocks.len(), 2);
//
//         // Check block 0
//         assert_eq!(blocks[0]["id"], 0);
//         assert_eq!(blocks[0]["reachable"], true);
//         assert_eq!(blocks[0]["terminator"], "Jump");
//         assert_eq!(blocks[0]["successors"].as_array().unwrap(), &[json!(1)]);
//
//         // Check block 1
//         assert_eq!(blocks[1]["id"], 1);
//         assert_eq!(blocks[1]["reachable"], true);
//         assert_eq!(blocks[1]["terminator"], "Return");
//     }
//
//     #[test]
//     fn test_unreachable_block() {
//         let mut module = MirModule::new("test");
//
//         let mut function = MirFunction::new(MirSignature::new("test_dead".to_string()));
//         function.entry_block = BasicBlockId(0);
//
//         let mut block0 = BasicBlock::new(BasicBlockId(0));
//         block0.reachable = true;
//         block0.terminator = Some(MirInstruction::Return { value: None });
//
//         // Unreachable block
//         let mut block1 = BasicBlock::new(BasicBlockId(1));
//         block1.reachable = false; // Marked as unreachable
//         block1.terminator = Some(MirInstruction::Return { value: None });
//
//         function.blocks.insert(BasicBlockId(0), block0);
//         function.blocks.insert(BasicBlockId(1), block1);
//
//         module.functions.insert("test_dead".to_string(), function);
//
//         let cfg = extract_cfg_info(&module);
//         let blocks = cfg["functions"][0]["blocks"].as_array().unwrap();
//
//         // Find unreachable block
//         let dead_block = blocks.iter().find(|b| b["id"] == 1).unwrap();
//         assert_eq!(dead_block["reachable"], false);
//     }
// }
