//! Canonical partition of the existing typed-object layout allocator.
//! Only source-issued declaration contracts select storage; MIR observations
//! and legacy metadata never refine an unavailable canonical field.

use super::TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0;
use crate::mir::declared_type_storage::exact_numeric_storage_for_declared_type;
use crate::mir::function::{
    CanonicalObjectDefinitionV1, CanonicalObjectLayoutUnavailableV1 as Unavailable,
    CanonicalObjectLayoutV1, TypedObjectFieldPlan, TypedObjectPlan,
};
use crate::mir::MirModule;

pub(super) fn type_id_at(position: usize) -> Result<u32, String> {
    u32::try_from(position)
        .ok()
        .and_then(|id| id.checked_add(1))
        .ok_or_else(|| fault("type-id-overflow"))
}

fn layout(
    definition: &CanonicalObjectDefinitionV1,
    type_id: u32,
) -> Result<CanonicalObjectLayoutV1, String> {
    let fields = match definition.local_fields_for_layout() {
        Ok(fields) => fields,
        Err(reason) => return Ok(Err(Unavailable::Declaration(reason))),
    };
    let mut planned = Vec::with_capacity(fields.len());
    for (ordinal, field) in fields.iter().enumerate() {
        let slot = u32::try_from(ordinal).map_err(|_| fault("field-overflow"))?;
        if field.is_weak {
            return Ok(Err(Unavailable::WeakField(slot)));
        }
        let Some(storage) = field
            .declared_type_name
            .as_deref()
            .and_then(exact_numeric_storage_for_declared_type)
        else {
            return Ok(Err(Unavailable::FieldType(slot)));
        };
        planned.push(TypedObjectFieldPlan {
            name: field.name.clone(),
            slot,
            declared_type_name: field.declared_type_name.clone(),
            storage,
            is_weak: false,
        });
    }
    Ok(Ok(TypedObjectPlan {
        box_name: definition.diagnostic_name().to_owned(),
        type_id,
        layout_kind: TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0.into(),
        field_count: u32::try_from(planned.len()).map_err(|_| fault("field-overflow"))?,
        fields: planned,
    }))
}

pub(super) fn prepare(module: &MirModule) -> Result<Vec<CanonicalObjectLayoutV1>, String> {
    module.validate_object_definition_membership()?;
    if let Some(definitions) = module.canonical_object_definitions() {
        let mut ids = std::collections::BTreeSet::new();
        let mut names = std::collections::BTreeSet::new();
        for plan in &module.metadata.typed_object_plans {
            if plan.type_id == 0 || !ids.insert(plan.type_id) || !names.insert(&plan.box_name) {
                return Err(fault("projection-identity-collision"));
            }
            let member = module
                .metadata
                .canonical_object_membership
                .as_ref()
                .and_then(|members| members.get(&plan.box_name));
            match member {
                Some(id) if plan.type_id == type_id_at(id.declaration_index() as usize)? => {}
                None if u64::from(plan.type_id) > definitions.len() as u64 => {}
                _ => return Err(fault("projection-reserved-id")),
            }
        }
    }
    let mut layouts = Vec::new();
    for (position, definition) in module
        .canonical_object_definitions()
        .unwrap_or(&[])
        .iter()
        .enumerate()
    {
        // Every declaration reserves a position, even when its layout is unavailable.
        let expected = layout(definition, type_id_at(position)?)?;
        if let Some(existing) = definition.runtime_layout() {
            if existing != &expected {
                return Err(fault("allocation-drift"));
            }
            let projection: Vec<_> = module
                .metadata
                .typed_object_plans
                .iter()
                .filter(|plan| plan.box_name == definition.diagnostic_name())
                .collect();
            match existing {
                Ok(plan) if projection == vec![plan] => {}
                Err(_) if projection.is_empty() => {}
                _ => return Err(fault("projection-drift")),
            }
        }
        layouts.push(expected);
    }
    Ok(layouts)
}

pub(super) fn validate(module: &MirModule) -> Result<(), String> {
    if module
        .canonical_object_definitions()
        .unwrap_or(&[])
        .iter()
        .any(|definition| definition.runtime_layout().is_none())
    {
        return Err(fault("allocation-missing"));
    }
    prepare(module).map(|_| ())
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][mir/object-layout/{reason}]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{ObjectLayoutUnavailableV1, UserBoxFieldDecl};

    fn field(name: &str) -> UserBoxFieldDecl {
        UserBoxFieldDecl {
            name: name.into(),
            declared_type_name: Some("i64".into()),
            is_weak: false,
        }
    }

    fn module() -> MirModule {
        let mut module = MirModule::new("canonical_layout".into());
        let definitions = vec![
            CanonicalObjectDefinitionV1::from_source_declaration(
                "Empty".into(),
                Box::new([]),
                Ok(()),
            ),
            CanonicalObjectDefinitionV1::from_source_declaration(
                "Inherited".into(),
                Box::new([]),
                Err(ObjectLayoutUnavailableV1::Inheritance),
            ),
            CanonicalObjectDefinitionV1::from_source_declaration(
                "Pair".into(),
                vec![field("left"), field("right")].into_boxed_slice(),
                Ok(()),
            ),
        ];
        for definition in &definitions {
            module.metadata.user_box_decls.insert(
                definition.diagnostic_name().into(),
                definition
                    .fields()
                    .iter()
                    .map(|field| field.name.clone())
                    .collect(),
            );
            module.metadata.user_box_field_decls.insert(
                definition.diagnostic_name().into(),
                definition.fields().to_vec(),
            );
        }
        module.install_object_definitions_preflighted(definitions.into_boxed_slice());
        module.metadata.canonical_object_membership = module
            .prepare_object_definition_membership(
                &module.metadata.user_box_decls,
                &module.metadata.user_box_field_decls,
            )
            .unwrap();
        module
    }

    fn add_compatibility(module: &mut MirModule, name: &str) {
        module
            .metadata
            .user_box_decls
            .insert(name.into(), vec!["value".into()]);
        module
            .metadata
            .user_box_field_decls
            .insert(name.into(), vec![field("value")]);
    }

    #[test]
    fn reserved_ids_survive_compatibility_changes_and_repeated_refresh() {
        let mut module = module();
        assert!(validate(&module)
            .unwrap_err()
            .contains("allocation-missing"));
        add_compatibility(&mut module, "ZCompat");
        super::super::refresh(&mut module).unwrap();
        let layouts: Vec<_> = module
            .canonical_object_definitions()
            .unwrap()
            .iter()
            .map(|definition| definition.runtime_layout().unwrap().clone())
            .collect();
        assert_eq!(layouts[0].as_ref().unwrap().type_id, 1);
        assert_eq!(layouts[0].as_ref().unwrap().field_count, 0);
        assert!(matches!(
            &layouts[1],
            Err(Unavailable::Declaration(
                ObjectLayoutUnavailableV1::Inheritance
            ))
        ));
        assert_eq!(layouts[2].as_ref().unwrap().type_id, 3);
        assert_eq!(
            module.metadata.typed_object_plans.last().unwrap().type_id,
            4
        );
        validate(&module).unwrap();
        module.metadata.user_box_decls.remove("ZCompat");
        module.metadata.user_box_field_decls.remove("ZCompat");
        add_compatibility(&mut module, "ACompat");
        for _ in 0..2 {
            super::super::refresh(&mut module).unwrap();
        }
        for (definition, original) in module
            .canonical_object_definitions()
            .unwrap()
            .iter()
            .zip(&layouts)
        {
            assert_eq!(definition.runtime_layout(), Some(original));
        }
        assert_eq!(
            module.metadata.typed_object_plans.last().unwrap().type_id,
            4
        );
        validate(&module).unwrap();
    }

    #[test]
    fn projection_tamper_and_missing_rows_reject_without_repair() {
        let mut module = module();
        super::super::refresh(&mut module).unwrap();
        let original = module.metadata.typed_object_plans.clone();
        module.metadata.typed_object_plans[1].type_id = 1;
        let tampered = module.metadata.typed_object_plans.clone();
        assert!(super::super::refresh(&mut module)
            .unwrap_err()
            .contains("projection-identity-collision"));
        assert_eq!(module.metadata.typed_object_plans, tampered);
        let error =
            crate::mir::backend_capability::enforce_mir_backend_supported(&module, "ny-llvmc-obj")
                .unwrap_err();
        assert!(error.contains("projection-identity-collision"));
        assert_eq!(module.metadata.typed_object_plans, tampered);
        module.metadata.typed_object_plans = original.clone();
        module.metadata.typed_object_plans.pop();
        assert!(validate(&module).is_err());
        module.metadata.typed_object_plans = original;
        validate(&module).unwrap();
        for bad_id in [0, 1, 2, 3] {
            let mut foreign = module.metadata.typed_object_plans[0].clone();
            foreign.box_name = "Foreign".into();
            foreign.type_id = bad_id;
            module.metadata.typed_object_plans.push(foreign);
            let bad_projection = module.metadata.typed_object_plans.clone();
            assert!(validate(&module).is_err());
            assert!(super::super::refresh(&mut module).is_err());
            assert_eq!(module.metadata.typed_object_plans, bad_projection);
            module.metadata.typed_object_plans.pop();
        }
    }

    #[test]
    fn unsupported_field_has_no_observation_fallback_and_ids_are_checked() {
        let definition = CanonicalObjectDefinitionV1::from_source_declaration(
            "Unknown".into(),
            vec![UserBoxFieldDecl {
                name: "value".into(),
                declared_type_name: None,
                is_weak: false,
            }]
            .into_boxed_slice(),
            Ok(()),
        );
        assert_eq!(
            layout(&definition, 1).unwrap(),
            Err(Unavailable::FieldType(0))
        );
        let mut weak = field("weak");
        weak.is_weak = true;
        let definition = CanonicalObjectDefinitionV1::from_source_declaration(
            "Weak".into(),
            vec![weak].into_boxed_slice(),
            Ok(()),
        );
        assert_eq!(
            layout(&definition, 1).unwrap(),
            Err(Unavailable::WeakField(0))
        );
        assert_eq!(type_id_at(u32::MAX as usize - 1).unwrap(), u32::MAX);
        assert!(type_id_at(u32::MAX as usize).is_err());
        assert!(type_id_at(usize::MAX).is_err());
    }
}
