use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Stage1FieldDecl {
    pub(super) name: String,
    pub(super) declared_type_name: Option<String>,
    pub(super) is_weak: bool,
}

pub(super) struct Stage1UserBoxDecl {
    name: String,
    fields: Vec<String>,
    field_decls: Vec<Stage1FieldDecl>,
}

pub(super) struct Stage1UserBoxDecls {
    decls: Vec<Stage1UserBoxDecl>,
}

impl Stage1UserBoxDecl {
    fn from_json_value(decl: &serde_json::Value) -> Option<Self> {
        let name = Self::parse_name(decl)?;
        let fields = Self::parse_fields(decl);
        let field_decls = Self::parse_field_decls(decl, &fields);
        Some(Self {
            name,
            fields,
            field_decls,
        })
    }

    fn parse_name(decl: &serde_json::Value) -> Option<String> {
        let name = decl.get("name")?.as_str()?.trim();
        if name.is_empty() {
            return None;
        }
        Some(name.to_string())
    }

    fn parse_fields(decl: &serde_json::Value) -> Vec<String> {
        decl.get("fields")
            .and_then(serde_json::Value::as_array)
            .map(|fields| {
                fields
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn parse_field_decls(decl: &serde_json::Value, fields: &[String]) -> Vec<Stage1FieldDecl> {
        let parsed = decl
            .get("field_decls")
            .and_then(serde_json::Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Self::parse_field_decl_entry)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if parsed.is_empty() {
            return fields
                .iter()
                .cloned()
                .map(|name| Stage1FieldDecl {
                    name,
                    declared_type_name: None,
                    is_weak: false,
                })
                .collect();
        }

        parsed
    }

    fn parse_field_decl_entry(value: &serde_json::Value) -> Option<Stage1FieldDecl> {
        let name = value.get("name")?.as_str()?.trim();
        if name.is_empty() {
            return None;
        }
        let declared_type_name = value
            .get("declared_type")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string);
        let is_weak = value
            .get("is_weak")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        Some(Stage1FieldDecl {
            name: name.to_string(),
            declared_type_name,
            is_weak,
        })
    }

    fn from_name(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
            field_decls: Vec::new(),
        }
    }

    fn into_typed_metadata_entry(self) -> (String, Vec<crate::mir::function::UserBoxFieldDecl>) {
        (
            self.name,
            self.field_decls
                .into_iter()
                .map(|decl| crate::mir::function::UserBoxFieldDecl {
                    name: decl.name,
                    declared_type_name: decl.declared_type_name,
                    is_weak: decl.is_weak,
                })
                .collect(),
        )
    }

    #[cfg(test)]
    fn into_json_value(self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "fields": self.fields,
            "field_decls": self.field_decls.into_iter().map(|decl| serde_json::json!({
                "name": decl.name,
                "declared_type": decl.declared_type_name,
                "is_weak": decl.is_weak,
            })).collect::<Vec<_>>()
        })
    }
}

impl Stage1UserBoxDecls {
    fn new(decls: Vec<Stage1UserBoxDecl>) -> Self {
        Self { decls }
    }

    pub(super) fn from_program_value(program_value: &serde_json::Value) -> Self {
        Self::new(Self::resolve_decls(program_value))
    }

    fn resolve_decls(program_value: &serde_json::Value) -> Vec<Stage1UserBoxDecl> {
        Self::explicit_from_program_value(program_value)
            .unwrap_or_else(|| Self::compat_from_program_value(program_value))
    }

    fn explicit_from_program_value(
        program_value: &serde_json::Value,
    ) -> Option<Vec<Stage1UserBoxDecl>> {
        let decls = Self::explicit_decl_values(program_value)?;
        Some(Self::collect_explicit_decls(decls))
    }

    fn explicit_decl_values(program_value: &serde_json::Value) -> Option<&Vec<serde_json::Value>> {
        program_value.get("user_box_decls")?.as_array()
    }

    fn collect_explicit_decls(decls: &[serde_json::Value]) -> Vec<Stage1UserBoxDecl> {
        decls
            .iter()
            .filter_map(Stage1UserBoxDecl::from_json_value)
            .collect()
    }

    fn compat_from_program_value(program_value: &serde_json::Value) -> Vec<Stage1UserBoxDecl> {
        Self::collect_compat_decl_names(program_value)
            .into_iter()
            .map(Stage1UserBoxDecl::from_name)
            .collect()
    }

    fn collect_compat_decl_names(program_value: &serde_json::Value) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        seen.insert("Main".to_string());
        Self::insert_compat_def_box_names(program_value, &mut seen);
        seen
    }

    fn insert_compat_def_box_names(program_value: &serde_json::Value, seen: &mut BTreeSet<String>) {
        if let Some(defs) = program_value
            .get("defs")
            .and_then(serde_json::Value::as_array)
        {
            for def in defs {
                if let Some(box_name) = Self::compat_def_box_name(def) {
                    seen.insert(box_name);
                }
            }
        }
    }

    fn compat_def_box_name(def: &serde_json::Value) -> Option<String> {
        let box_name = def.get("box").and_then(serde_json::Value::as_str)?;
        if box_name.is_empty() {
            return None;
        }
        Some(box_name.to_string())
    }

    pub(super) fn into_metadata_maps(
        self,
    ) -> (
        HashMap<String, Vec<String>>,
        HashMap<String, Vec<crate::mir::function::UserBoxFieldDecl>>,
    ) {
        let mut fields = HashMap::new();
        let mut field_decls = HashMap::new();
        for decl in self.decls {
            let names = decl.fields.clone();
            let name = decl.name.clone();
            fields.insert(name.clone(), names);
            let (typed_name, typed_fields) = decl.into_typed_metadata_entry();
            field_decls.insert(typed_name, typed_fields);
        }
        (fields, field_decls)
    }

    #[cfg(test)]
    pub(super) fn into_decl_values(self) -> Vec<serde_json::Value> {
        self.decls
            .into_iter()
            .map(Stage1UserBoxDecl::into_json_value)
            .collect()
    }
}
