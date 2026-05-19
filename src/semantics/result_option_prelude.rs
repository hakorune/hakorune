use crate::ast::EnumVariantDecl;
use std::collections::BTreeMap;

pub fn extend_known_result_option_enums(known_enums: &mut BTreeMap<String, Vec<EnumVariantDecl>>) {
    for (name, variants) in result_option_prelude_enum_decls() {
        known_enums.entry(name).or_insert(variants);
    }
}

pub fn result_option_prelude_enum_decls() -> BTreeMap<String, Vec<EnumVariantDecl>> {
    BTreeMap::from([
        (
            "Option".to_string(),
            vec![unit_variant("None"), tuple_variant("Some", "T")],
        ),
        (
            "Result".to_string(),
            vec![tuple_variant("Ok", "T"), tuple_variant("Err", "E")],
        ),
    ])
}

fn unit_variant(name: &str) -> EnumVariantDecl {
    EnumVariantDecl {
        name: name.to_string(),
        payload_type_name: None,
        record_field_decls: Vec::new(),
        tuple_payload_type_names: Vec::new(),
    }
}

fn tuple_variant(name: &str, payload_type: &str) -> EnumVariantDecl {
    EnumVariantDecl {
        name: name.to_string(),
        payload_type_name: Some(payload_type.to_string()),
        record_field_decls: Vec::new(),
        tuple_payload_type_names: Vec::new(),
    }
}
