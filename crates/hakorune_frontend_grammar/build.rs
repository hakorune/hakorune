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
        .expect("parse grammar/language-v1-registry.toml");
    let rows = document
        .get("rows")
        .and_then(toml::Value::as_array)
        .expect("missing [[rows]] in Language v1 registry");

    let mut generated = String::from(
        "use crate::contract::{GrammarContractRow, GrammarProfile, GrammarStatus, NormalizationMode};\n\n",
    );
    generated
        .push_str("pub static LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS: &[GrammarContractRow] = &[\n");
    for value in rows {
        let source_row = value
            .as_table()
            .expect("Language v1 source row must be a TOML table");
        for (profile_key, profile_name) in
            [("canonical", "Canonical"), ("compat2025", "Compat2025")]
        {
            let profile_contract = source_row
                .get(profile_key)
                .and_then(toml::Value::as_table)
                .unwrap_or_else(|| {
                    panic!(
                        "Language v1 source row `{}` missing required {profile_key} contract",
                        string_field(source_row, "row_id")
                    )
                });
            generated.push_str("    GrammarContractRow {\n");
            for field in ["row_id", "family", "spelling_id", "production"] {
                generated.push_str(&format!(
                    "        {field}: {:?},\n",
                    string_field(source_row, field)
                ));
            }
            generated.push_str(&format!(
                "        profile: {},\n",
                variant("profile", profile_name)
            ));
            for (field, kind) in [
                ("status", "status"),
                ("normalization_mode", "normalization_mode"),
            ] {
                generated.push_str(&format!(
                    "        {field}: {},\n",
                    variant(kind, string_field(profile_contract, field))
                ));
            }
            for field in ["normalized_shape", "semantic_owner", "stable_reject_tag"] {
                generated.push_str(&format!(
                    "        {field}: {:?},\n",
                    string_field(profile_contract, field)
                ));
            }
            for field in ["positive_fixture_ids", "negative_fixture_ids"] {
                let values = string_list(profile_contract, field)
                    .iter()
                    .map(|value| format!("{value:?}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                generated.push_str(&format!("        {field}: &[{values}],\n"));
            }
            generated.push_str("    },\n");
        }
    }
    generated.push_str("];\n");
    generated
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let grammar = manifest_dir.join("../../grammar/language-v1-registry.toml");
    println!("cargo:rerun-if-changed={}", grammar.display());
    let content = fs::read_to_string(&grammar).expect("read Language v1 grammar registry");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("generated_contract.rs"), emit_rows(&content))
        .expect("write generated grammar contract projection");
}
