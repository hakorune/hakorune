//! BORROW-P0-ROOT-P0c focused shell declaration-fact transaction proof.

use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_lowering_shell::{
    ModuleDeclarationFactShellPrepareErrorV1, ModuleLoweringShellV1,
};
use crate::mir::function::{MirEnumDecl, RecordDecl};
use crate::mir::{MirModule, UserBoxFieldDecl};
use std::collections::BTreeMap;

fn facts() -> SealedModuleDeclarationFactsV1 {
    let mut boxes = BTreeMap::new();
    boxes.insert("Page".to_owned(), vec!["used".to_owned()]);
    let mut fields = BTreeMap::new();
    fields.insert(
        "Page".to_owned(),
        vec![UserBoxFieldDecl {
            name: "used".to_owned(),
            declared_type_name: Some("i64".to_owned()),
            is_weak: false,
        }],
    );
    let mut records = BTreeMap::new();
    records.insert(
        "Pair".to_owned(),
        RecordDecl {
            name: "Pair".to_owned(),
            type_parameters: Vec::new(),
            fields: Vec::new(),
            default_field_names: Vec::new(),
        },
    );
    let mut enums = BTreeMap::new();
    enums.insert(
        "Option".to_owned(),
        MirEnumDecl {
            type_parameters: Vec::new(),
            variants: Vec::new(),
        },
    );
    SealedModuleDeclarationFactsV1::new(boxes, fields, records, enums)
}

fn empty_shell() -> ModuleLoweringShellV1 {
    ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())).unwrap()
}

#[test]
fn prepared_shell_commit_moves_all_four_declaration_lanes_once() {
    let mut shell = empty_shell()
        .prepare_declaration_fact_commit(facts())
        .unwrap()
        .commit();
    shell.with_port(|port| {
        let metadata = port.metadata();
        assert_eq!(metadata.user_box_decls["Page"], ["used"]);
        assert_eq!(metadata.user_box_field_decls["Page"][0].name, "used");
        assert_eq!(metadata.record_decls["Pair"].name, "Pair");
        assert!(metadata.enum_decls.contains_key("Option"));
    });
    assert!(!shell.has_published_functions());
}

#[test]
fn failed_preparation_returns_the_exact_unmodified_shell_and_sealed_facts() {
    let mut module = MirModule::new("main".to_owned());
    module
        .metadata
        .user_box_decls
        .insert("Existing".to_owned(), Vec::new());
    let shell = ModuleLoweringShellV1::from_empty_module(module).unwrap();
    let rejected = shell.prepare_declaration_fact_commit(facts()).unwrap_err();
    assert_eq!(
        rejected.error(),
        &ModuleDeclarationFactShellPrepareErrorV1::DestinationNotEmpty {
            user_box_decls: 1,
            user_box_field_decls: 0,
            record_decls: 0,
            enum_decls: 0,
        }
    );
    let (mut shell, facts, _) = rejected.into_parts();
    shell.with_port(|port| {
        assert!(port.metadata().user_box_decls.contains_key("Existing"));
        assert!(!port.metadata().user_box_decls.contains_key("Page"));
    });
    assert!(facts.user_box_decls().contains_key("Page"));
}
