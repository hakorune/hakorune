/*!
 * MIR-owned same-module fusion plan rows.
 *
 * Backends may emit selected same-module helpers, but they must not discover
 * instruction windows by scanning neighboring MIR JSON. This module owns the
 * first typed-field RMW window plan.
 */

use crate::mir::function::{TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, BinaryOp, MirFunction, MirInstruction, MirModule, ValueId};
use std::collections::BTreeMap;

pub const SAME_MODULE_TYPED_FIELD_RMW_KIND: &str = "same_module_typed_field_rmw_add_u64";
pub const SAME_MODULE_TYPED_FIELD_RMW_HELPER: &str = "nyash.object.exact_slot_rmw_add_u64_hiii";
pub const SAME_MODULE_RESULT_CAPSULE_RESET_BATCH_KIND: &str =
    "same_module_result_capsule_reset_batch_i64";
pub const SAME_MODULE_RESULT_CAPSULE_RESET_BATCH_HELPER: &str =
    "nyash.object.exact_slot_set4_i64_hiiiii";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SameModuleFusionPlan {
    TypedFieldRmw(TypedFieldRmwFusionPlan),
    ResultCapsuleResetBatch(ResultCapsuleResetBatchPlan),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedFieldRmwFusionPlan {
    pub kind: &'static str,
    pub function: String,
    pub block: BasicBlockId,
    pub get_instruction_index: usize,
    pub binop_instruction_index: usize,
    pub set_instruction_index: usize,
    pub skip_instruction_indices: Vec<usize>,
    pub get_dst: ValueId,
    pub binop_dst: ValueId,
    pub box_reg: ValueId,
    pub field: String,
    pub slot: u32,
    pub delta_reg: ValueId,
    pub helper_symbol: &'static str,
    pub storage: &'static str,
    pub direct_use_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultCapsuleResetBatchPlan {
    pub kind: &'static str,
    pub function: String,
    pub block: BasicBlockId,
    pub first_set_instruction_index: usize,
    pub set_instruction_indices: [usize; 4],
    pub skip_instruction_indices: Vec<usize>,
    pub box_reg: ValueId,
    pub fields: [&'static str; 4],
    pub slots: [u32; 4],
    pub values: [i64; 4],
    pub helper_symbol: &'static str,
    pub storage: &'static str,
}

pub fn refresh_module_same_module_fusion_plans(module: &mut MirModule) {
    let typed_fields = typed_object_field_map(&module.metadata.typed_object_plans);
    for function in module.functions.values_mut() {
        function.metadata.same_module_fusion_plans =
            build_function_same_module_fusion_plans(function, &typed_fields);
    }
}

pub fn build_function_same_module_fusion_plans(
    function: &MirFunction,
    typed_fields: &TypedFieldMap,
) -> Vec<SameModuleFusionPlan> {
    let def_map = build_value_def_map(function);
    let mut plans = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (get_index, get_inst) in block.instructions.iter().enumerate() {
            if let Some(plan) = match_typed_field_rmw_at(
                function,
                &def_map,
                typed_fields,
                block_id,
                get_index,
                get_inst,
            ) {
                plans.push(SameModuleFusionPlan::TypedFieldRmw(plan));
            }
        }
        if let Some(plan) =
            match_result_capsule_reset_batch(function, &def_map, typed_fields, block_id)
        {
            plans.push(SameModuleFusionPlan::ResultCapsuleResetBatch(plan));
        }
    }
    plans
}

type TypedFieldMap = BTreeMap<(String, String), (u32, TypedObjectFieldStorage)>;

fn typed_object_field_map(plans: &[TypedObjectPlan]) -> TypedFieldMap {
    let mut fields = BTreeMap::new();
    for plan in plans {
        for field in &plan.fields {
            fields.insert(
                (plan.box_name.clone(), field.name.clone()),
                (field.slot, field.storage),
            );
        }
    }
    fields
}

fn match_typed_field_rmw_at(
    function: &MirFunction,
    def_map: &ValueDefMap,
    typed_fields: &TypedFieldMap,
    block_id: BasicBlockId,
    get_index: usize,
    get_inst: &MirInstruction,
) -> Option<TypedFieldRmwFusionPlan> {
    let MirInstruction::FieldGet {
        dst: get_dst,
        base: get_box,
        field: get_field,
        ..
    } = get_inst
    else {
        return None;
    };
    let (slot, storage) = typed_field_slot(function, def_map, typed_fields, *get_box, get_field)?;
    if storage != TypedObjectFieldStorage::U64 {
        return None;
    }
    let block = function.blocks.get(&block_id)?;
    if direct_use_count(block, *get_dst) != 1 {
        return None;
    }
    for (binop_index, binop) in block.instructions.iter().enumerate().skip(get_index + 1) {
        let MirInstruction::BinOp {
            dst: binop_dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } = binop
        else {
            continue;
        };
        let lhs_base = resolve_value_origin(function, def_map, *lhs);
        let rhs_base = resolve_value_origin(function, def_map, *rhs);
        let delta_reg = if lhs_base == *get_dst {
            *rhs
        } else if rhs_base == *get_dst {
            *lhs
        } else {
            continue;
        };
        for (set_index, set_inst) in block.instructions.iter().enumerate().skip(binop_index + 1) {
            let MirInstruction::FieldSet {
                base: set_box,
                field: set_field,
                value: set_value,
                ..
            } = set_inst
            else {
                continue;
            };
            if resolve_value_origin(function, def_map, *set_value) != *binop_dst {
                continue;
            }
            if resolve_value_origin(function, def_map, *set_box)
                != resolve_value_origin(function, def_map, *get_box)
            {
                continue;
            }
            if set_field != get_field {
                continue;
            }
            let get_json_index = emitted_instruction_index(block, get_index)?;
            let binop_json_index = emitted_instruction_index(block, binop_index)?;
            let set_json_index = emitted_instruction_index(block, set_index)?;
            return Some(TypedFieldRmwFusionPlan {
                kind: SAME_MODULE_TYPED_FIELD_RMW_KIND,
                function: function.signature.name.clone(),
                block: block_id,
                get_instruction_index: get_json_index,
                binop_instruction_index: binop_json_index,
                set_instruction_index: set_json_index,
                skip_instruction_indices: vec![get_json_index, binop_json_index],
                get_dst: *get_dst,
                binop_dst: *binop_dst,
                box_reg: *get_box,
                field: get_field.clone(),
                slot,
                delta_reg,
                helper_symbol: SAME_MODULE_TYPED_FIELD_RMW_HELPER,
                storage: storage.as_str(),
                direct_use_count: 1,
            });
        }
    }
    None
}

fn match_result_capsule_reset_batch(
    function: &MirFunction,
    def_map: &ValueDefMap,
    typed_fields: &TypedFieldMap,
    block_id: BasicBlockId,
) -> Option<ResultCapsuleResetBatchPlan> {
    const FIELDS: [&str; 4] = ["last_page_id", "last_block_id", "last_reason", "last_ok"];
    const VALUES: [i64; 4] = [-1, -1, 0, 0];
    let block = function.blocks.get(&block_id)?;
    let mut search_from = 0usize;
    let mut set_indices = [0usize; 4];
    let mut set_json_indices = [0usize; 4];
    let mut slots = [0u32; 4];
    let mut box_reg = None;
    for field_index in 0..FIELDS.len() {
        let mut found = None;
        for (inst_index, inst) in block.instructions.iter().enumerate().skip(search_from) {
            let MirInstruction::FieldSet {
                base, field, value, ..
            } = inst
            else {
                continue;
            };
            if field != FIELDS[field_index] {
                continue;
            }
            let (slot, storage) = typed_field_slot(function, def_map, typed_fields, *base, field)?;
            if storage != TypedObjectFieldStorage::I64 || slot != field_index as u32 {
                return None;
            }
            if const_i64_value(function, def_map, *value)? != VALUES[field_index] {
                return None;
            }
            if let Some(first_box) = box_reg {
                if resolve_value_origin(function, def_map, first_box)
                    != resolve_value_origin(function, def_map, *base)
                {
                    return None;
                }
            } else {
                box_reg = Some(*base);
            }
            found = Some((inst_index, slot));
            break;
        }
        let (inst_index, slot) = found?;
        set_indices[field_index] = inst_index;
        set_json_indices[field_index] = emitted_instruction_index(block, inst_index)?;
        slots[field_index] = slot;
        search_from = inst_index + 1;
    }
    Some(ResultCapsuleResetBatchPlan {
        kind: SAME_MODULE_RESULT_CAPSULE_RESET_BATCH_KIND,
        function: function.signature.name.clone(),
        block: block_id,
        first_set_instruction_index: set_json_indices[0],
        set_instruction_indices: set_json_indices,
        skip_instruction_indices: set_json_indices[1..].to_vec(),
        box_reg: box_reg?,
        fields: FIELDS,
        slots,
        values: VALUES,
        helper_symbol: SAME_MODULE_RESULT_CAPSULE_RESET_BATCH_HELPER,
        storage: "exact_slot_i64",
    })
}

fn typed_field_slot(
    function: &MirFunction,
    def_map: &ValueDefMap,
    typed_fields: &TypedFieldMap,
    box_value: ValueId,
    field: &str,
) -> Option<(u32, TypedObjectFieldStorage)> {
    let box_name = typed_object_value_box_name(function, def_map, box_value)?;
    typed_fields.get(&(box_name, field.to_string())).copied()
}

fn typed_object_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    function
        .metadata
        .value_types
        .get(&origin)
        .and_then(box_name_from_type)
        .map(str::to_string)
        .or_else(|| {
            let (block_id, instruction_index) = def_map.get(&origin).copied()?;
            let block = function.blocks.get(&block_id)?;
            match block.instructions.get(instruction_index)? {
                MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
                MirInstruction::Phi { type_hint, .. } => type_hint
                    .as_ref()
                    .and_then(box_name_from_type)
                    .map(str::to_string),
                _ => None,
            }
        })
}

fn box_name_from_type(ty: &crate::mir::MirType) -> Option<&str> {
    match ty {
        crate::mir::MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

fn const_i64_value(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<i64> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::Const {
            value: crate::mir::ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        MirInstruction::Const {
            value: crate::mir::ConstValue::Bool(actual),
            ..
        } => Some(i64::from(*actual)),
        _ => None,
    }
}

fn direct_use_count(block: &crate::mir::BasicBlock, value: ValueId) -> u32 {
    block
        .all_spanned_instructions()
        .map(|spanned| {
            spanned
                .inst
                .used_values()
                .into_iter()
                .filter(|used| *used == value)
                .count() as u32
        })
        .sum()
}

fn emitted_instruction_index(
    block: &crate::mir::BasicBlock,
    instruction_index: usize,
) -> Option<usize> {
    if instruction_index >= block.instructions.len() {
        return None;
    }
    let phi_count = block
        .instructions
        .iter()
        .filter(|inst| matches!(inst, MirInstruction::Phi { .. }))
        .count();
    let non_phi_before = block
        .instructions
        .iter()
        .take(instruction_index)
        .filter(|inst| !matches!(inst, MirInstruction::Phi { .. }))
        .count();
    Some(phi_count + non_phi_before)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};

    #[test]
    fn emits_typed_field_rmw_plan_row_without_backend_scan() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Counter.add/1".to_string(),
                params: vec![MirType::Box("Counter".to_string()), MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Counter".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(1),
            field: "value".to_string(),
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(4),
        });
        block.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(4),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(3),
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(1),
            field: "value".to_string(),
            value: ValueId::new(4),
            declared_type: Some(MirType::Integer),
        });
        function.add_block(block);

        let typed_fields = BTreeMap::from([(
            ("Counter".to_string(), "value".to_string()),
            (0, TypedObjectFieldStorage::U64),
        )]);
        let plans = build_function_same_module_fusion_plans(&function, &typed_fields);

        assert_eq!(plans.len(), 1);
        let SameModuleFusionPlan::TypedFieldRmw(plan) = &plans[0] else {
            panic!("expected typed-field RMW plan");
        };
        assert_eq!(plan.kind, SAME_MODULE_TYPED_FIELD_RMW_KIND);
        assert_eq!(plan.get_instruction_index, 1);
        assert_eq!(plan.binop_instruction_index, 3);
        assert_eq!(plan.set_instruction_index, 4);
        assert_eq!(plan.skip_instruction_indices, vec![1, 3]);
        assert_eq!(plan.slot, 0);
        assert_eq!(plan.storage, "u64");
        assert_eq!(plan.delta_reg, ValueId::new(3));
        assert_eq!(plan.helper_symbol, SAME_MODULE_TYPED_FIELD_RMW_HELPER);
    }

    #[test]
    fn emits_result_capsule_reset_batch_plan_row_without_backend_scan() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Result.reset/0".to_string(),
                params: vec![MirType::Box("ResultCapsule".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ResultCapsule".to_string(),
            args: vec![],
        });
        for (index, value) in [-1, -1, 0, 0].into_iter().enumerate() {
            let value_id = ValueId::new((index as u32) + 2);
            block.add_instruction(MirInstruction::Const {
                dst: value_id,
                value: ConstValue::Integer(value),
            });
            block.add_instruction(MirInstruction::FieldSet {
                base: ValueId::new(1),
                field: ["last_page_id", "last_block_id", "last_reason", "last_ok"][index]
                    .to_string(),
                value: value_id,
                declared_type: Some(MirType::Integer),
            });
        }
        function.add_block(block);

        let typed_fields = BTreeMap::from([
            (
                ("ResultCapsule".to_string(), "last_page_id".to_string()),
                (0, TypedObjectFieldStorage::I64),
            ),
            (
                ("ResultCapsule".to_string(), "last_block_id".to_string()),
                (1, TypedObjectFieldStorage::I64),
            ),
            (
                ("ResultCapsule".to_string(), "last_reason".to_string()),
                (2, TypedObjectFieldStorage::I64),
            ),
            (
                ("ResultCapsule".to_string(), "last_ok".to_string()),
                (3, TypedObjectFieldStorage::I64),
            ),
        ]);
        let plans = build_function_same_module_fusion_plans(&function, &typed_fields);

        assert_eq!(plans.len(), 1);
        let SameModuleFusionPlan::ResultCapsuleResetBatch(plan) = &plans[0] else {
            panic!("expected reset batch plan");
        };
        assert_eq!(plan.kind, SAME_MODULE_RESULT_CAPSULE_RESET_BATCH_KIND);
        assert_eq!(plan.first_set_instruction_index, 2);
        assert_eq!(plan.set_instruction_indices, [2, 4, 6, 8]);
        assert_eq!(plan.skip_instruction_indices, vec![4, 6, 8]);
        assert_eq!(plan.slots, [0, 1, 2, 3]);
        assert_eq!(plan.values, [-1, -1, 0, 0]);
        assert_eq!(
            plan.helper_symbol,
            SAME_MODULE_RESULT_CAPSULE_RESET_BATCH_HELPER
        );
    }
}
