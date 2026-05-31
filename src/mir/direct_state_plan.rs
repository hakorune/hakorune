/*!
 * Metadata-only direct-state candidate plans.
 *
 * This module does not create a runtime layout and does not enable lowering.
 * It records which typed user-box fields are primitive enough to be considered
 * by a later NativeDirect guard.
 */

use crate::mir::declared_type_storage::storage_for_declared_type;
use crate::mir::function::{DirectStateFieldPlan, DirectStatePlan, TypedObjectFieldStorage};
use crate::mir::{MirModule, UserBoxFieldDecl};

pub const DIRECT_STATE_REPR_V0: &str = "direct_v0";

pub fn refresh_module_direct_state_plans(module: &mut MirModule) {
    module.metadata.direct_state_plans = build_direct_state_plans(module);
}

pub fn build_direct_state_plans(module: &MirModule) -> Vec<DirectStatePlan> {
    let mut names: Vec<_> = module
        .metadata
        .user_box_field_decls
        .keys()
        .cloned()
        .collect();
    names.sort();

    names
        .into_iter()
        .filter_map(|box_name| {
            let fields = module.metadata.user_box_field_decls.get(&box_name)?;
            build_direct_state_plan(module, box_name, fields)
        })
        .collect()
}

fn build_direct_state_plan(
    module: &MirModule,
    box_name: String,
    decls: &[UserBoxFieldDecl],
) -> Option<DirectStatePlan> {
    let mut selected_fields = Vec::new();
    let mut unsupported_field_count = 0u32;

    for (slot, decl) in decls.iter().enumerate() {
        let storage =
            storage_for_declared_type(&module.metadata, decl.declared_type_name.as_deref());
        match storage {
            Some(storage) if direct_state_storage_supported(storage) && !decl.is_weak => {
                selected_fields.push(DirectStateFieldPlan {
                    name: decl.name.clone(),
                    slot: slot as u32,
                    declared_type_name: decl.declared_type_name.clone(),
                    storage,
                });
            }
            _ => unsupported_field_count += 1,
        }
    }

    if selected_fields.is_empty() {
        return None;
    }

    let all_fields_supported = unsupported_field_count == 0;
    Some(DirectStatePlan {
        box_name,
        state_repr: DIRECT_STATE_REPR_V0.to_string(),
        field_decl_authority: true,
        selected_field_count: selected_fields.len() as u32,
        unsupported_field_count,
        materialization_boundary_known: all_fields_supported,
        positive_net_expected: all_fields_supported,
        fields: selected_fields,
    })
}

fn direct_state_storage_supported(storage: TypedObjectFieldStorage) -> bool {
    storage.uses_integer_lane()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::TypedObjectFieldStorage;

    fn field(name: &str, ty: &str) -> UserBoxFieldDecl {
        UserBoxFieldDecl {
            name: name.to_string(),
            declared_type_name: Some(ty.to_string()),
            is_weak: false,
        }
    }

    fn weak_field(name: &str, ty: &str) -> UserBoxFieldDecl {
        UserBoxFieldDecl {
            name: name.to_string(),
            declared_type_name: Some(ty.to_string()),
            is_weak: true,
        }
    }

    #[test]
    fn primitive_box_becomes_positive_direct_state_candidate() {
        let mut module = MirModule::new("direct-state-test".to_string());
        module.metadata.user_box_field_decls.insert(
            "Counter".to_string(),
            vec![field("count", "i64"), field("generation", "usize")],
        );

        refresh_module_direct_state_plans(&mut module);

        assert_eq!(module.metadata.direct_state_plans.len(), 1);
        let plan = &module.metadata.direct_state_plans[0];
        assert_eq!(plan.box_name, "Counter");
        assert_eq!(plan.state_repr, DIRECT_STATE_REPR_V0);
        assert!(plan.field_decl_authority);
        assert_eq!(plan.selected_field_count, 2);
        assert_eq!(plan.unsupported_field_count, 0);
        assert!(plan.materialization_boundary_known);
        assert!(plan.positive_net_expected);
        assert_eq!(plan.fields[0].storage, TypedObjectFieldStorage::I64);
        assert_eq!(plan.fields[1].storage, TypedObjectFieldStorage::USize);
    }

    #[test]
    fn mixed_box_reports_unsupported_fields_without_positive_net() {
        let mut module = MirModule::new("direct-state-mixed-test".to_string());
        module.metadata.user_box_field_decls.insert(
            "Facade".to_string(),
            vec![
                field("counter", "usize"),
                field("page", "HakoAllocPageModel"),
                weak_field("weak_counter", "i64"),
            ],
        );

        refresh_module_direct_state_plans(&mut module);

        assert_eq!(module.metadata.direct_state_plans.len(), 1);
        let plan = &module.metadata.direct_state_plans[0];
        assert_eq!(plan.selected_field_count, 1);
        assert_eq!(plan.unsupported_field_count, 2);
        assert!(!plan.materialization_boundary_known);
        assert!(!plan.positive_net_expected);
        assert_eq!(plan.fields[0].name, "counter");
    }

    #[test]
    fn handle_only_box_is_not_a_direct_state_candidate() {
        let mut module = MirModule::new("direct-state-handle-test".to_string());
        module
            .metadata
            .user_box_field_decls
            .insert("Holder".to_string(), vec![field("item", "StringBox")]);

        refresh_module_direct_state_plans(&mut module);

        assert!(module.metadata.direct_state_plans.is_empty());
    }
}
