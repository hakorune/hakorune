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

    pub(super) fn commit_into(self, module: &mut MirModule) {
        module.metadata.user_box_decls = self.user_box_decls;
        module.metadata.user_box_field_decls = self.user_box_field_decls;
        module.metadata.record_decls = self.record_decls;
        module.metadata.enum_decls = self.enum_decls;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::FieldDecl;
    use crate::mir::builder::compilation_context::CompilationContext;
    use crate::mir::function::RecordDecl;

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

        prepared.commit_into(&mut module);

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
