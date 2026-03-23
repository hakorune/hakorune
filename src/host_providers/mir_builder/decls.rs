use std::collections::{BTreeSet, HashMap};

pub(super) struct Stage1UserBoxDecl {
    name: String,
    fields: Vec<String>,
}

pub(super) struct Stage1UserBoxDecls {
    decls: Vec<Stage1UserBoxDecl>,
}

impl Stage1UserBoxDecl {
    fn from_json_value(decl: &serde_json::Value) -> Option<Self> {
        let name = Self::parse_name(decl)?;
        let fields = Self::parse_fields(decl);
        Some(Self { name, fields })
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

    fn from_name(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    fn into_metadata_entry(self) -> (String, Vec<String>) {
        (self.name, self.fields)
    }

    #[cfg(test)]
    fn into_json_value(self) -> serde_json::Value {
        serde_json::json!({ "name": self.name, "fields": self.fields })
    }
}

impl Stage1UserBoxDecls {
    fn new(decls: Vec<Stage1UserBoxDecl>) -> Self {
        Self { decls }
    }

    pub(super) fn parse_program_json(program_json: &str) -> Result<Self, String> {
        let program_value = Self::parse_program_value(program_json)?;
        Ok(Self::from_program_value(&program_value))
    }

    fn parse_program_value(program_json: &str) -> Result<serde_json::Value, String> {
        serde_json::from_str(program_json)
            .map_err(|error| format!("program json parse error: {}", error))
    }

    fn from_program_value(program_value: &serde_json::Value) -> Self {
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

    fn into_metadata_entries(self) -> Vec<(String, Vec<String>)> {
        self.decls
            .into_iter()
            .map(Stage1UserBoxDecl::into_metadata_entry)
            .collect()
    }

    pub(super) fn into_metadata_map(self) -> HashMap<String, Vec<String>> {
        self.into_metadata_entries().into_iter().collect()
    }

    #[cfg(test)]
    pub(super) fn into_decl_values(self) -> Vec<serde_json::Value> {
        self.decls
            .into_iter()
            .map(Stage1UserBoxDecl::into_json_value)
            .collect()
    }
}
