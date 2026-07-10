//! Shared fixture corpus for Language v1 grammar-contract conformance.

use crate::contract::{GrammarProfile, ParseWitness};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrammarContractFixture {
    pub fixture_id: String,
    pub row_id: String,
    pub profile: GrammarProfile,
    pub source: String,
    pub expected: ParseWitness,
}

const CORPUS: &str = include_str!("../../../grammar/language-v1-grammar-contract-corpus.toml");

fn string(value: &toml::value::Table, field: &str) -> String {
    value
        .get(field)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("grammar fixture missing string field `{field}`"))
        .to_owned()
}

fn strings(value: &toml::value::Table, field: &str) -> Vec<String> {
    value
        .get(field)
        .and_then(toml::Value::as_array)
        .unwrap_or_else(|| panic!("grammar fixture missing array field `{field}`"))
        .iter()
        .map(|item| {
            item.as_str()
                .expect("grammar fixture array must be strings")
                .to_owned()
        })
        .collect()
}

fn profile(value: &str) -> GrammarProfile {
    match value {
        "Canonical" => GrammarProfile::Canonical,
        "Compat2025" => GrammarProfile::Compat2025,
        _ => panic!("unsupported grammar fixture profile `{value}`"),
    }
}

pub fn shared_corpus() -> Vec<GrammarContractFixture> {
    let document: toml::Value = CORPUS.parse().expect("parse grammar contract corpus");
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
                ParseWitness::accepted(
                    row_id.clone(),
                    profile,
                    string(value, "normalized_kind"),
                    strings(value, "normalized_children"),
                )
            } else {
                ParseWitness::rejected(row_id.clone(), profile, string(value, "stable_reject_tag"))
            };
            GrammarContractFixture {
                fixture_id,
                row_id,
                profile,
                source: string(value, "source"),
                expected,
            }
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
