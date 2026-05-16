use std::collections::{BTreeMap, BTreeSet};

mod collection_storage;
mod field_origin_inference;
mod merge;
mod param_inference;
mod state;
mod type_facts;
mod value_analysis;

use super::TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0;
use crate::mir::declared_type_storage::storage_for_declared_type;
use crate::mir::function::{
    ModuleMetadata, TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan,
};
use crate::mir::value_origin::build_value_def_map;
use crate::mir::{MirInstruction, MirModule, UserBoxFieldDecl};
use collection_storage::infer_collection_element_storages;
use field_origin_inference::infer_untyped_field_box_origins;
use merge::merge_storage_observation;
use param_inference::infer_param_storages;
use state::{FieldKey, FieldStorageInference};
use type_facts::storage_for_mir_type;
use value_analysis::{box_name_for_value, storage_for_value};

#[cfg(test)]
mod numeric_substrate_tests;
#[cfg(test)]
mod tests;

pub(super) fn build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan> {
    let inferred = infer_untyped_field_storages(module);
    let observed_newboxes = observed_user_newbox_names(module);
    build_typed_object_plans_from_metadata(&module.metadata, &inferred, &observed_newboxes)
}

fn build_typed_object_plans_from_metadata(
    metadata: &ModuleMetadata,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    observed_newboxes: &BTreeSet<String>,
) -> Vec<TypedObjectPlan> {
    let mut names = BTreeSet::new();
    names.extend(metadata.user_box_decls.keys().cloned());
    names.extend(metadata.user_box_field_decls.keys().cloned());
    names.extend(observed_newboxes.iter().cloned());

    let mut plans = Vec::new();
    for name in names {
        if !metadata.user_box_decls.contains_key(&name)
            && !metadata.user_box_field_decls.contains_key(&name)
        {
            continue;
        }
        let empty_field_decls = Vec::new();
        let field_decls = metadata
            .user_box_field_decls
            .get(&name)
            .unwrap_or(&empty_field_decls);
        if field_decls.is_empty() {
            if observed_newboxes.contains(&name) {
                let type_id = plans.len() as u32 + 1;
                plans.push(TypedObjectPlan {
                    box_name: name,
                    type_id,
                    layout_kind: TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0.to_string(),
                    field_count: 0,
                    fields: Vec::new(),
                });
            }
            continue;
        }
        let Some(fields) = build_field_plans(metadata, &name, field_decls, inferred) else {
            continue;
        };
        let type_id = plans.len() as u32 + 1;
        plans.push(TypedObjectPlan {
            box_name: name,
            type_id,
            layout_kind: TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0.to_string(),
            field_count: fields.len() as u32,
            fields,
        });
    }
    plans
}

fn observed_user_newbox_names(module: &MirModule) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for inst in &block.instructions {
                if let MirInstruction::NewBox { box_type, .. } = inst {
                    if module.metadata.user_box_decls.contains_key(box_type)
                        || module.metadata.user_box_field_decls.contains_key(box_type)
                    {
                        names.insert(box_type.clone());
                    }
                }
            }
        }
    }
    names
}

fn build_field_plans(
    metadata: &ModuleMetadata,
    box_name: &str,
    field_decls: &[UserBoxFieldDecl],
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
) -> Option<Vec<TypedObjectFieldPlan>> {
    let mut fields = Vec::new();
    for (slot, decl) in field_decls.iter().enumerate() {
        if decl.is_weak {
            return None;
        }
        let declared = storage_for_declared_type(metadata, decl.declared_type_name.as_deref());
        let observed = match inferred.get(&(box_name.to_string(), decl.name.clone())) {
            Some(FieldStorageInference::Known(storage)) => Some(*storage),
            Some(FieldStorageInference::Conflict) => return None,
            None => None,
        };
        let storage = match (declared, observed) {
            (Some(left), Some(right)) if !typed_object_storage_compatible(left, right) => {
                return None;
            }
            (Some(storage), _) | (None, Some(storage)) => storage,
            (None, None) => return None,
        };
        fields.push(TypedObjectFieldPlan {
            name: decl.name.clone(),
            slot: slot as u32,
            declared_type_name: decl.declared_type_name.clone(),
            storage,
            is_weak: decl.is_weak,
        });
    }
    Some(fields)
}

fn typed_object_storage_compatible(
    declared: TypedObjectFieldStorage,
    observed: TypedObjectFieldStorage,
) -> bool {
    declared == observed || (declared.uses_integer_lane() && observed.uses_integer_lane())
}

fn infer_untyped_field_storages(module: &MirModule) -> BTreeMap<FieldKey, FieldStorageInference> {
    let declared_fields = declared_field_sets(&module.metadata);
    let field_box_origins = infer_untyped_field_box_origins(module, &declared_fields);
    let mut inferred = BTreeMap::new();

    for _ in 0..4 {
        let mut changed = false;
        let param_storages = infer_param_storages(module, &inferred, &field_box_origins);
        let collection_element_storages = infer_collection_element_storages(
            module,
            &inferred,
            &field_box_origins,
            &param_storages,
        );
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
            for block in function.blocks.values() {
                for inst in &block.instructions {
                    let MirInstruction::FieldSet {
                        base,
                        field,
                        value,
                        declared_type,
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
                    let storage = declared_type
                        .as_ref()
                        .and_then(storage_for_mir_type)
                        .or_else(|| {
                            storage_for_value(
                                module,
                                function,
                                &def_map,
                                *value,
                                &inferred,
                                &field_box_origins,
                                &param_storages,
                                &collection_element_storages,
                            )
                        });
                    if let Some(storage) = storage {
                        changed |= merge_storage_observation(
                            &mut inferred,
                            (box_name, field.clone()),
                            storage,
                        );
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    inferred
}

fn declared_field_sets(metadata: &ModuleMetadata) -> BTreeMap<String, BTreeSet<String>> {
    metadata
        .user_box_field_decls
        .iter()
        .map(|(box_name, fields)| {
            (
                box_name.clone(),
                fields.iter().map(|field| field.name.clone()).collect(),
            )
        })
        .collect()
}
