//! HEADERPORT0-I0-SHELLFACT0-P0: declaration-lane/failure matrix.

use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use crate::mir::function::{MirEnumDecl, RecordDecl};
use crate::mir::UserBoxFieldDecl;
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

#[test]
fn all_declaration_lanes_move_together_at_the_shell_boundary() {
    let (boxes, fields, records, enums) = facts().into_parts();
    assert_eq!(boxes["Page"], ["used"]);
    assert_eq!(fields["Page"][0].name, "used");
    assert_eq!(records["Pair"].name, "Pair");
    assert!(enums.contains_key("Option"));
}

#[test]
fn empty_and_nonempty_lane_shapes_remain_explicit() {
    let empty = SealedModuleDeclarationFactsV1::new(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    );
    let populated = facts();
    assert!(empty.user_box_decls().is_empty());
    assert!(!populated.user_box_decls().is_empty());
    assert!(populated.record_decls().contains_key("Pair"));
    assert!(populated.enum_decls().contains_key("Option"));
}
