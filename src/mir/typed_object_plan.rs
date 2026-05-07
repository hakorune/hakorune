/*!
 * Typed object layout plans for EXE lowering.
 *
 * MIR owns the object layout truth. Backends consume these plans instead of
 * rediscovering user-box declarations or cloning VM InstanceBox semantics.
 */

use std::collections::BTreeSet;

use crate::mir::{
    function::{ModuleMetadata, TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan},
    MirModule, UserBoxFieldDecl,
};

pub const TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0: &str = "runtime_slot_object_v0";

pub fn refresh_module_typed_object_plans(module: &mut MirModule) {
    module.metadata.typed_object_plans = build_typed_object_plans(&module.metadata);
}

pub fn build_typed_object_plans(metadata: &ModuleMetadata) -> Vec<TypedObjectPlan> {
    let mut names = BTreeSet::new();
    names.extend(metadata.user_box_field_decls.keys().cloned());

    let mut plans = Vec::new();
    for name in names {
        let Some(field_decls) = metadata.user_box_field_decls.get(&name) else {
            continue;
        };
        let Some(fields) = build_i64_field_plans(field_decls) else {
            continue;
        };
        if fields.is_empty() {
            continue;
        }
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

fn build_i64_field_plans(field_decls: &[UserBoxFieldDecl]) -> Option<Vec<TypedObjectFieldPlan>> {
    let mut fields = Vec::new();
    for (slot, decl) in field_decls.iter().enumerate() {
        if decl.is_weak {
            return None;
        }
        let storage = storage_for_declared_type(decl.declared_type_name.as_deref())?;
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

fn storage_for_declared_type(type_name: Option<&str>) -> Option<TypedObjectFieldStorage> {
    match type_name {
        Some("IntegerBox") | Some("Integer") | Some("i64") => Some(TypedObjectFieldStorage::I64),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::UserBoxFieldDecl;

    #[test]
    fn build_typed_object_plans_accepts_nonweak_i64_fields() {
        let mut metadata = ModuleMetadata::default();
        metadata.user_box_field_decls.insert(
            "Pair".to_string(),
            vec![
                UserBoxFieldDecl {
                    name: "left".to_string(),
                    declared_type_name: Some("IntegerBox".to_string()),
                    is_weak: false,
                },
                UserBoxFieldDecl {
                    name: "right".to_string(),
                    declared_type_name: Some("Integer".to_string()),
                    is_weak: false,
                },
            ],
        );

        let plans = build_typed_object_plans(&metadata);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].box_name, "Pair");
        assert_eq!(plans[0].type_id, 1);
        assert_eq!(
            plans[0].layout_kind,
            TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0
        );
        assert_eq!(plans[0].field_count, 2);
        assert_eq!(plans[0].fields[0].slot, 0);
        assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
        assert_eq!(plans[0].fields[1].slot, 1);
    }

    #[test]
    fn build_typed_object_plans_rejects_weak_or_unknown_storage() {
        let mut metadata = ModuleMetadata::default();
        metadata.user_box_field_decls.insert(
            "WeakBox".to_string(),
            vec![UserBoxFieldDecl {
                name: "next".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: true,
            }],
        );
        metadata.user_box_field_decls.insert(
            "AnyBox".to_string(),
            vec![UserBoxFieldDecl {
                name: "value".to_string(),
                declared_type_name: None,
                is_weak: false,
            }],
        );

        let plans = build_typed_object_plans(&metadata);

        assert!(plans.is_empty());
    }
}
