use std::collections::{BTreeMap, BTreeSet};

use crate::mir::value_origin::build_value_def_map;
use crate::mir::{MirInstruction, MirModule};

use super::merge::merge_box_origin_observation;
use super::param_inference::infer_param_box_origins;
use super::state::FieldBoxOriginMap;
use super::value_analysis::{box_name_for_value, box_origin_for_value};

pub(super) fn infer_untyped_field_box_origins(
    module: &MirModule,
    declared_fields: &BTreeMap<String, BTreeSet<String>>,
) -> FieldBoxOriginMap {
    let mut origins = FieldBoxOriginMap::new();
    for _ in 0..module.functions.len().max(1) {
        let current = origins.clone();
        let param_box_origins = infer_param_box_origins(module, &current);
        let mut changed = false;
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
            for block in function.blocks.values() {
                for inst in &block.instructions {
                    let MirInstruction::FieldSet {
                        base, field, value, ..
                    } = inst
                    else {
                        continue;
                    };
                    let Some(box_name) = box_name_for_value(function, &def_map, *base) else {
                        continue;
                    };
                    if !declared_fields
                        .get(&box_name)
                        .is_some_and(|fields| fields.contains(field))
                    {
                        continue;
                    }
                    let Some(origin_box) = box_origin_for_value(
                        module,
                        function,
                        &def_map,
                        *value,
                        &current,
                        &param_box_origins,
                    ) else {
                        continue;
                    };
                    changed |= merge_box_origin_observation(
                        &mut origins,
                        (box_name, field.clone()),
                        origin_box,
                    );
                }
            }
        }
        if !changed {
            break;
        }
    }
    origins
}
