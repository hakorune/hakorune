//! HEADERPORT0-I0-SHELLFACT0-S0: sealed module declaration facts.
//!
//! This is a source-declaration snapshot for the function-empty module shell.
//! It owns no Builder, AST body, function map, collector, or derived layout
//! plan.  Publication into `ModuleLoweringShellV1` remains disconnected.

use std::collections::BTreeMap;

use crate::mir::function::{MirEnumDecl, RecordDecl};
use crate::mir::UserBoxFieldDecl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct SealedModuleDeclarationFactsV1 {
    user_box_decls: BTreeMap<String, Vec<String>>,
    user_box_field_decls: BTreeMap<String, Vec<UserBoxFieldDecl>>,
    record_decls: BTreeMap<String, RecordDecl>,
    enum_decls: BTreeMap<String, MirEnumDecl>,
    _seal: SealedModuleDeclarationFactsSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SealedModuleDeclarationFactsSealV1;

impl SealedModuleDeclarationFactsV1 {
    /// Construct one deterministic snapshot from already-owned declaration
    /// outputs.  No semantic inference or representation refresh occurs.
    pub(in crate::mir::builder) fn new(
        user_box_decls: BTreeMap<String, Vec<String>>,
        user_box_field_decls: BTreeMap<String, Vec<UserBoxFieldDecl>>,
        record_decls: BTreeMap<String, RecordDecl>,
        enum_decls: BTreeMap<String, MirEnumDecl>,
    ) -> Self {
        Self {
            user_box_decls,
            user_box_field_decls,
            record_decls,
            enum_decls,
            _seal: SealedModuleDeclarationFactsSealV1,
        }
    }

    pub(in crate::mir::builder) fn user_box_decls(&self) -> &BTreeMap<String, Vec<String>> {
        &self.user_box_decls
    }

    pub(in crate::mir::builder) fn user_box_field_decls(
        &self,
    ) -> &BTreeMap<String, Vec<UserBoxFieldDecl>> {
        &self.user_box_field_decls
    }

    pub(in crate::mir::builder) fn record_decls(&self) -> &BTreeMap<String, RecordDecl> {
        &self.record_decls
    }

    pub(in crate::mir::builder) fn enum_decls(&self) -> &BTreeMap<String, MirEnumDecl> {
        &self.enum_decls
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_has_no_function_or_derived_authority() {
        let facts = SealedModuleDeclarationFactsV1::new(
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
        );
        assert!(facts.user_box_decls().is_empty());
        assert!(facts.user_box_field_decls().is_empty());
        assert!(facts.record_decls().is_empty());
        assert!(facts.enum_decls().is_empty());
    }

    #[test]
    fn declaration_snapshot_preserves_all_four_source_fact_lanes() {
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

        let facts = SealedModuleDeclarationFactsV1::new(boxes, fields, records, enums);
        assert_eq!(facts.user_box_decls()["Page"], ["used"]);
        assert_eq!(facts.user_box_field_decls()["Page"][0].name, "used");
        assert_eq!(facts.record_decls()["Pair"].name, "Pair");
        assert!(facts.enum_decls().contains_key("Option"));
    }

    #[test]
    fn btree_snapshot_order_is_independent_of_insertion_order() {
        let mut first = BTreeMap::new();
        first.insert("z".to_owned(), Vec::<String>::new());
        first.insert("a".to_owned(), Vec::<String>::new());
        let mut second = BTreeMap::new();
        second.insert("a".to_owned(), Vec::<String>::new());
        second.insert("z".to_owned(), Vec::<String>::new());

        let left = SealedModuleDeclarationFactsV1::new(
            first,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
        );
        let right = SealedModuleDeclarationFactsV1::new(
            second,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
        );
        assert_eq!(left, right);
    }
}
