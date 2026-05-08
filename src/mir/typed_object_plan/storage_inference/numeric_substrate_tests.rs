use super::*;
use crate::mir::{MirModule, UserBoxFieldDecl};

fn module_with_metadata(metadata: ModuleMetadata) -> MirModule {
    let mut module = MirModule::new("typed_object_numeric_substrate_test".to_string());
    module.metadata = metadata;
    module
}

#[test]
fn build_typed_object_plans_accepts_numeric_substrate_type_names_as_i64_storage() {
    let mut metadata = ModuleMetadata::default();
    metadata.user_box_field_decls.insert(
        "Counters".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "len".to_string(),
                declared_type_name: Some("usize".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "mask".to_string(),
                declared_type_name: Some("u64".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "delta".to_string(),
                declared_type_name: Some("i32".to_string()),
                is_weak: false,
            },
        ],
    );
    let module = module_with_metadata(metadata);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Counters");
    assert_eq!(plans[0].fields.len(), 3);
    for field in &plans[0].fields {
        assert_eq!(field.storage, TypedObjectFieldStorage::I64);
    }
    assert_eq!(
        plans[0].fields[0].declared_type_name.as_deref(),
        Some("usize")
    );
    assert_eq!(
        plans[0].fields[1].declared_type_name.as_deref(),
        Some("u64")
    );
    assert_eq!(
        plans[0].fields[2].declared_type_name.as_deref(),
        Some("i32")
    );
}
