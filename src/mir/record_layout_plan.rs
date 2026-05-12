/*!
 * Record layout plans for identity-free aggregate metadata.
 *
 * This owner derives layout facts from `record_decls`. It does not lower
 * constructors, rewrite locals, or reuse typed-object user-box layout plans.
 */

use crate::mir::declared_type_storage::storage_for_declared_type;
use crate::mir::function::{RecordLayoutFieldPlan, RecordLayoutPlan};
use crate::mir::{MirModule, UserBoxFieldDecl};

pub const RECORD_LAYOUT_KIND_VALUE_AGGREGATE_V0: &str = "record_value_aggregate_v0";

pub fn refresh_module_record_layout_plans(module: &mut MirModule) {
    module.metadata.record_layout_plans = build_record_layout_plans(module);
}

pub fn build_record_layout_plans(module: &MirModule) -> Vec<RecordLayoutPlan> {
    let mut plans = Vec::new();
    for decl in module.metadata.record_decls.values() {
        let Some(fields) = build_record_field_plans(module, &decl.fields) else {
            continue;
        };
        let layout_id = plans.len() as u32 + 1;
        plans.push(RecordLayoutPlan {
            record_name: decl.name.clone(),
            layout_id,
            layout_kind: RECORD_LAYOUT_KIND_VALUE_AGGREGATE_V0.to_string(),
            field_count: fields.len() as u32,
            fields,
        });
    }
    plans
}

fn build_record_field_plans(
    module: &MirModule,
    field_decls: &[UserBoxFieldDecl],
) -> Option<Vec<RecordLayoutFieldPlan>> {
    let mut fields = Vec::new();
    for (slot, decl) in field_decls.iter().enumerate() {
        if decl.is_weak {
            return None;
        }
        let storage =
            storage_for_declared_type(&module.metadata, decl.declared_type_name.as_deref())?;
        fields.push(RecordLayoutFieldPlan {
            name: decl.name.clone(),
            slot: slot as u32,
            declared_type_name: decl.declared_type_name.clone(),
            storage,
        });
    }
    Some(fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{MirModule, RecordDecl, UserBoxFieldDecl};

    #[test]
    fn build_record_layout_plans_accepts_concrete_typed_fields() {
        let mut module = MirModule::new("record-layout-test".to_string());
        module.metadata.record_decls.insert(
            "Meta".to_string(),
            RecordDecl {
                name: "Meta".to_string(),
                type_parameters: Vec::new(),
                fields: vec![
                    UserBoxFieldDecl {
                        name: "ptr".to_string(),
                        declared_type_name: Some("i64".to_string()),
                        is_weak: false,
                    },
                    UserBoxFieldDecl {
                        name: "size".to_string(),
                        declared_type_name: Some("usize".to_string()),
                        is_weak: false,
                    },
                    UserBoxFieldDecl {
                        name: "label".to_string(),
                        declared_type_name: Some("String".to_string()),
                        is_weak: false,
                    },
                ],
            },
        );

        let plans = build_record_layout_plans(&module);
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].record_name, "Meta");
        assert_eq!(plans[0].layout_id, 1);
        assert_eq!(plans[0].layout_kind, RECORD_LAYOUT_KIND_VALUE_AGGREGATE_V0);
        assert_eq!(plans[0].field_count, 3);
        assert_eq!(plans[0].fields[0].name, "ptr");
        assert_eq!(plans[0].fields[0].storage.as_str(), "i64");
        assert_eq!(plans[0].fields[1].storage.as_str(), "usize");
        assert_eq!(plans[0].fields[2].storage.as_str(), "handle");
    }

    #[test]
    fn build_record_layout_plans_skips_generic_and_weak_records() {
        let mut module = MirModule::new("record-layout-test".to_string());
        module.metadata.record_decls.insert(
            "GenericMeta".to_string(),
            RecordDecl {
                name: "GenericMeta".to_string(),
                type_parameters: vec!["T".to_string()],
                fields: vec![UserBoxFieldDecl {
                    name: "payload".to_string(),
                    declared_type_name: Some("T".to_string()),
                    is_weak: false,
                }],
            },
        );
        module.metadata.record_decls.insert(
            "WeakMeta".to_string(),
            RecordDecl {
                name: "WeakMeta".to_string(),
                type_parameters: Vec::new(),
                fields: vec![UserBoxFieldDecl {
                    name: "ptr".to_string(),
                    declared_type_name: Some("i64".to_string()),
                    is_weak: true,
                }],
            },
        );

        let plans = build_record_layout_plans(&module);
        assert!(plans.is_empty());
    }
}
