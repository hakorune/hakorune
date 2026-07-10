use std::{env, fs, path::PathBuf};

fn string_field<'a>(row: &'a toml::value::Table, field: &str) -> &'a str {
    row.get(field)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("grammar contract row missing string field `{field}`"))
}

fn string_list(row: &toml::value::Table, field: &str) -> Vec<String> {
    row.get(field)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("grammar contract row missing array field `{field}`"))
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("grammar contract `{field}` must contain strings"))
                .to_owned()
        })
        .collect()
}

fn variant(kind: &str, value: &str) -> &'static str {
    match (kind, value) {
        ("profile", "Canonical") => "GrammarProfile::Canonical",
        ("profile", "Compat2025") => "GrammarProfile::Compat2025",
        ("status", "canonical") => "GrammarStatus::Canonical",
        ("status", "compatibility_only") => "GrammarStatus::CompatibilityOnly",
        ("status", "reserved") => "GrammarStatus::Reserved",
        ("status", "rejected") => "GrammarStatus::Rejected",
        ("normalization_mode", "canonical_shape") => "NormalizationMode::CanonicalShape",
        ("normalization_mode", "compatibility_alias") => "NormalizationMode::CompatibilityAlias",
        ("normalization_mode", "compatibility_transport") => {
            "NormalizationMode::CompatibilityTransport"
        }
        ("normalization_mode", "none") => "NormalizationMode::None",
        _ => panic!("unsupported grammar contract {kind} `{value}`"),
    }
}

fn emit_rows(content: &str) -> String {
    let document: toml::Value = content
        .parse()
        .expect("parse grammar/unified-grammar.toml for Language v1 contract rows");
    let rows = document
        .get("language_v1_grammar_contract")
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("rows"))
        .and_then(toml::Value::as_array)
        .expect("missing [[language_v1_grammar_contract.rows]]");

    let mut generated = String::from(
        "use crate::contract::{GrammarContractRow, GrammarProfile, GrammarStatus, NormalizationMode};\n\n",
    );
    generated
        .push_str("pub static LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS: &[GrammarContractRow] = &[\n");
    for value in rows {
        let row = value
            .as_table()
            .expect("grammar contract row must be a TOML table");
        generated.push_str("    GrammarContractRow {\n");
        for field in [
            "row_id",
            "family",
            "spelling_id",
            "production",
            "normalized_shape",
            "semantic_owner",
            "stable_reject_tag",
            "rust_support",
            "hako_support",
        ] {
            generated.push_str(&format!(
                "        {field}: {:?},\n",
                string_field(row, field)
            ));
        }
        for (field, kind) in [
            ("profile", "profile"),
            ("status", "status"),
            ("normalization_mode", "normalization_mode"),
        ] {
            generated.push_str(&format!(
                "        {field}: {},\n",
                variant(kind, string_field(row, field))
            ));
        }
        for field in ["positive_fixture_ids", "negative_fixture_ids"] {
            let values = string_list(row, field)
                .iter()
                .map(|value| format!("{value:?}"))
                .collect::<Vec<_>>()
                .join(", ");
            generated.push_str(&format!("        {field}: &[{values}],\n"));
        }
        generated.push_str("    },\n");
    }
    generated.push_str("];\n");
    generated
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let grammar = manifest_dir.join("../../grammar/unified-grammar.toml");
    println!("cargo:rerun-if-changed={}", grammar.display());
    let content = fs::read_to_string(&grammar).expect("read unified grammar registry");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("generated_contract.rs"), emit_rows(&content))
        .expect("write generated grammar contract projection");
}
