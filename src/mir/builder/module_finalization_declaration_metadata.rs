//! Prepared declaration metadata for shared module finalization.
//!
//! This owner snapshots the four declaration lanes from the compilation context
//! before the candidate module changes. It deliberately does not refresh plans,
//! lower source, or decide publication.

use super::compilation_context::CompilationContext;
use super::MirModule;
use crate::mir::function::{MirEnumDecl, RecordDecl};
use crate::mir::UserBoxFieldDecl;
use std::collections::{BTreeMap, HashMap};

pub(super) struct PreparedModuleFinalizationDeclarationMetadataV1 {
    user_box_decls: HashMap<String, Vec<String>>,
    user_box_field_decls: HashMap<String, Vec<UserBoxFieldDecl>>,
    record_decls: BTreeMap<String, RecordDecl>,
    enum_decls: BTreeMap<String, MirEnumDecl>,
}

impl PreparedModuleFinalizationDeclarationMetadataV1 {
    pub(super) fn prepare(context: &CompilationContext) -> Self {
        let user_box_field_decls = context
            .user_box_field_decls
            .iter()
            .map(|(name, decls)| {
                (
                    name.clone(),
                    decls
                        .iter()
                        .map(|decl| UserBoxFieldDecl {
                            name: decl.name.clone(),
                            declared_type_name: decl.declared_type_name.clone(),
                            is_weak: decl.is_weak,
                        })
                        .collect(),
                )
            })
            .collect();

        Self {
            user_box_decls: context.user_defined_boxes.clone(),
            user_box_field_decls,
            record_decls: context.record_decls.clone().into_iter().collect(),
            enum_decls: context.enum_decls_for_module_metadata(),
        }
    }

    pub(super) fn commit_into(self, module: &mut MirModule) -> Result<(), String> {
        if module.metadata.canonical_object_membership.is_some() {
            return Err("[freeze:contract][mir/object-definition/membership-already-installed]".into());
        }
        let membership = module.prepare_object_definition_membership(
            &self.user_box_decls, &self.user_box_field_decls,
        )?;
        module.metadata.user_box_decls = self.user_box_decls;
        module.metadata.user_box_field_decls = self.user_box_field_decls;
        module.metadata.record_decls = self.record_decls;
        module.metadata.enum_decls = self.enum_decls;
        module.metadata.canonical_object_membership = membership;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::FieldDecl;
    use crate::mir::builder::compilation_context::CompilationContext;
    use crate::mir::function::RecordDecl;

    #[test]
    fn canonical_membership_is_id_projected_and_drift_never_repairs() {
        use crate::mir::function::CanonicalObjectDefinitionV1;
        let mut context = CompilationContext::default();
        context.user_defined_boxes.insert("Plain".into(), vec!["value".into()]);
        context.user_box_field_decls.insert("Plain".into(), vec![field("value", Some("i64"), false)]);
        let mut module = MirModule::new("membership".into());
        module.install_object_definitions_preflighted(vec![
            CanonicalObjectDefinitionV1::from_source_declaration(
                "Plain".into(), vec![UserBoxFieldDecl {
                    name: "value".into(), declared_type_name: Some("i64".into()), is_weak: false,
                }].into_boxed_slice(), Ok(()),
                crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook,
            ),
        ].into_boxed_slice());
        assert!(module.validate_object_definition_membership().is_err());
        let mut wrong_context = CompilationContext::default();
        wrong_context.user_defined_boxes.insert("Unrelated".into(), Vec::new());
        assert!(PreparedModuleFinalizationDeclarationMetadataV1::prepare(&wrong_context)
            .commit_into(&mut module).is_err());
        assert!(module.metadata.user_box_decls.is_empty(), "failed commit is atomic");
        PreparedModuleFinalizationDeclarationMetadataV1::prepare(&context)
            .commit_into(&mut module).unwrap();
        module.validate_object_definition_membership().unwrap();
        let expected = module.metadata.canonical_object_membership.clone();
        assert_eq!(expected.as_ref().unwrap()["Plain"].declaration_index(), 0);
        assert!(PreparedModuleFinalizationDeclarationMetadataV1::prepare(&context)
            .commit_into(&mut module).is_err());

        module.metadata.user_box_field_decls.get_mut("Plain").unwrap()[0]
            .declared_type_name = Some("StringBox".into());
        assert!(crate::mir::semantic_refresh::refresh_module_semantic_metadata(&mut module).is_err());
        assert_eq!(module.metadata.canonical_object_membership, expected);
        assert_eq!(module.metadata.user_box_field_decls["Plain"][0]
            .declared_type_name.as_deref(), Some("StringBox"), "no repair on error");
        module.metadata.user_box_field_decls.get_mut("Plain").unwrap()[0]
            .declared_type_name = Some("i64".into());
        module.metadata.canonical_object_membership.as_mut().unwrap().insert(
            "Plain".into(), hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(9).unwrap(),
        );
        assert!(module.validate_object_definition_membership().is_err());
        module.metadata.canonical_object_membership = expected;
        module.validate_object_definition_membership().unwrap();
    }

    fn field(name: &str, declared_type_name: Option<&str>, is_weak: bool) -> FieldDecl {
        FieldDecl {
            name: name.to_owned(),
            declared_type_name: declared_type_name.map(str::to_owned),
            is_weak,
            default_value: None,
        }
    }

    #[test]
    fn prepares_every_declaration_lane_before_one_metadata_commit() {
        let mut context = CompilationContext::default();
        context.user_defined_boxes.insert(
            "Page".to_owned(),
            vec!["title".to_owned(), "next".to_owned()],
        );
        context.user_box_field_decls.insert(
            "Page".to_owned(),
            vec![
                field("title", Some("String"), false),
                field("next", None, true),
            ],
        );
        context.record_decls.insert(
            "Point".to_owned(),
            RecordDecl {
                name: "Point".to_owned(),
                type_parameters: Vec::new(),
                fields: vec![UserBoxFieldDecl {
                    name: "x".to_owned(),
                    declared_type_name: Some("i64".to_owned()),
                    is_weak: false,
                }],
                default_field_names: Vec::new(),
            },
        );

        let prepared = PreparedModuleFinalizationDeclarationMetadataV1::prepare(&context);
        let mut module = MirModule::new("declaration_metadata/0".to_owned());

        assert!(module.metadata.user_box_decls.is_empty());
        assert!(module.metadata.user_box_field_decls.is_empty());
        assert!(module.metadata.record_decls.is_empty());
        assert!(module.metadata.enum_decls.is_empty());

        prepared.commit_into(&mut module).unwrap();

        assert_eq!(
            module.metadata.user_box_decls.get("Page"),
            Some(&vec!["title".to_owned(), "next".to_owned()])
        );
        assert_eq!(module.metadata.user_box_field_decls["Page"][1].name, "next");
        assert!(module.metadata.user_box_field_decls["Page"][1].is_weak);
        assert_eq!(module.metadata.record_decls["Point"].fields[0].name, "x");
        assert!(!module.metadata.enum_decls.is_empty());
    }
}
