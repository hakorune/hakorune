//! Shared fixture corpus for Language v1 grammar-contract conformance.

use crate::contract::{GrammarProfile, NormalizedSyntaxNode, ParseWitness};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrammarContractFixture {
    pub fixture_id: String,
    pub row_id: String,
    pub profile: GrammarProfile,
    pub source: String,
    pub expected: ParseWitness,
}

const CORPUS_FRAGMENTS: &[&str] = &[include_str!(
    "../../../grammar/language-v1-grammar-contract-corpus/foundation.toml"
)];

fn string(value: &toml::value::Table, field: &str) -> String {
    value
        .get(field)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("grammar fixture missing string field `{field}`"))
        .to_owned()
}

fn optional_string(value: &toml::value::Table, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

fn normalized_form(value: &toml::value::Table) -> Option<NormalizedSyntaxNode> {
    let form = value
        .get("normalized_form")
        .and_then(toml::Value::as_table)
        .unwrap_or_else(|| panic!("grammar fixture missing normalized_form"));
    normalized_node(form)
}

fn normalized_node(form: &toml::value::Table) -> Option<NormalizedSyntaxNode> {
    let kind = string(form, "kind");
    let children = form
        .get("children")
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("grammar normalized_form missing children"))
        .iter()
        .map(|child| {
            normalized_node(
                child
                    .as_table()
                    .expect("grammar normalized_form child must be a table"),
            )
            .expect("normalized_form child cannot be empty")
        })
        .collect::<Vec<_>>();
    if kind.is_empty() {
        assert!(
            children.is_empty(),
            "empty normalized_form cannot have children"
        );
        None
    } else {
        Some(NormalizedSyntaxNode::branch(kind, children))
    }
}

fn profile(value: &str) -> GrammarProfile {
    match value {
        "Canonical" => GrammarProfile::Canonical,
        "Compat2025" => GrammarProfile::Compat2025,
        _ => panic!("unsupported grammar fixture profile `{value}`"),
    }
}

pub fn shared_corpus() -> Vec<GrammarContractFixture> {
    CORPUS_FRAGMENTS
        .iter()
        .flat_map(|fragment| {
            let document: toml::Value = fragment
                .parse()
                .expect("parse grammar contract corpus fragment");
            document["fixtures"]
                .as_array()
                .expect("grammar contract corpus fixtures")
                .iter()
                .map(|value| {
                    let value = value.as_table().expect("grammar contract fixture table");
                    let fixture_id = string(value, "fixture_id");
                    let row_id = string(value, "row_id");
                    let profile = profile(&string(value, "profile"));
                    let accepted = value["accepted"]
                        .as_bool()
                        .expect("grammar fixture accepted boolean");
                    let expected = if accepted {
                        let normalized_form = normalized_form(value)
                            .expect("accepted grammar fixture requires normalized_form");
                        if normalized_form.kind == "CompatibilityTransport" {
                            let transport_ref = optional_string(value, "migration_transport_ref")
                                .expect(
                                "compatibility transport fixture requires migration transport ref",
                            );
                            assert!(
                                normalized_form.children.is_empty(),
                                "compatibility transport fixture must not expose semantic children"
                            );
                            ParseWitness::accepted_transport(row_id.clone(), profile, transport_ref)
                        } else {
                            ParseWitness::accepted(row_id.clone(), profile, normalized_form)
                        }
                    } else {
                        assert!(
                            normalized_form(value).is_none(),
                            "rejected grammar fixture cannot expose normalized_form"
                        );
                        ParseWitness::rejected(
                            row_id.clone(),
                            profile,
                            string(value, "stable_reject_tag"),
                        )
                    };
                    GrammarContractFixture {
                        fixture_id,
                        row_id,
                        profile,
                        source: string(value, "source"),
                        expected,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::contract::find_row;
    use crate::generated_contract::LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS;

    #[test]
    fn corpus_is_keyed_by_unique_fixture_and_registered_row() {
        let fixtures = shared_corpus();
        let ids = fixtures
            .iter()
            .map(|fixture| fixture.fixture_id.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), fixtures.len());
        for fixture in fixtures {
            assert!(find_row(&fixture.row_id, fixture.profile).is_some());
        }
    }

    #[test]
    fn corpus_covers_every_generated_fixture_reference() {
        let fixture_ids = shared_corpus()
            .into_iter()
            .map(|fixture| fixture.fixture_id)
            .collect::<BTreeSet<_>>();
        for row in LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS {
            for fixture_id in row
                .positive_fixture_ids
                .iter()
                .chain(row.negative_fixture_ids.iter())
            {
                assert!(
                    fixture_ids.contains(*fixture_id),
                    "missing fixture {fixture_id}"
                );
            }
        }
    }
}
