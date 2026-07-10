# 3461 - LANGV1-GRAMMAR-CONTRACT-SUBSTRATE-001

## Status

Active implementation card. Build the grammar-contract observation substrate
without changing parser acceptance or enabling either profile.

## Decision

`LANGV1-GRAMMAR-CONTRACT-BASIS-001` is accepted. The normative owner is
`docs/reference/language/grammar-contract.md`.

## Structural Scope

Keep one physical registry source by evolving `grammar/unified-grammar.toml`.
Do not create a parallel registry. Preserve existing generated APIs while the
new contract projection is introduced.

If `build.rs` would approach 800 lines, put the new generator in a focused
`build_support/` module. Every new source file must remain below 800 lines.

## Ordered Work

1. Add the Language v1 registry row schema and the four closed families:
   guard, exception, match, and delegation.
2. Generate typed contract rows into `hakorune-frontend-grammar` without
   changing tokenizer/parser behavior.
3. Add the parser-neutral span-free `ParseWitness` type.
4. Add one shared positive/negative corpus keyed only by fixture ID and row ID.
5. Add a Rust parser adapter that reports current behavior as evidence.
6. Add an independent Hako parser adapter that reports current behavior as
   evidence; do not share parser implementation code.
7. Add a strict comparator whose unit fixtures prove missing row, missing
   witness, reject-tag drift, profile drift, and normalized-shape drift fail.
8. Emit a deterministic current-drift report for the four families.

The live parser witnesses are expected to disagree until later migration work.
This card proves that disagreement is observable; it does not waive it or turn
the current behavior into authority.

## Four-Family Closed Set

```text
guard:
  guard expr else
  guard let ... else

exception:
  postfix catch
  postfix cleanup
  fini
  statement try

match:
  match
  peek

delegation:
  delegate field exposes
  box Child from Parent
  from Parent.method()
```

Every spelling has both Canonical and Compat2025 expectations.

## Acceptance

```text
grammar_contract_basis_accepted = 1
physical_registry_source_count = 1
four_family_registry_closed = 1
normalization_mode_present = 1
typed_contract_projection_generated = 1
parse_witness_schema_implemented = 1
shared_corpus_count = 1
independent_parser_adapter_count = 2
strict_comparator_unit_green = 1
current_drift_report_deterministic = 1
parser_behavior_changed = 0
```

Required verification:

```bash
bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
python3 tools/language_v1/grammar_contract_drift_report.py --bin target/debug/hakorune --hako-timeout-sec 1
```

The report records current Rust/Hako evidence only. A Hako adapter timeout is
reported as `parser/hako_adapter_timeout`; it is not converted into acceptance
or replaced with the Rust parser. Do not create rerun cards.

## Fail-Fast Boundary

```text
missing registry row -> parser/registry_row_missing
missing witness -> parser/witness_missing
Rust/Hako or expected witness drift -> parser/witness_drift
missing reject tag -> parser/stable_reject_tag_missing
profile mismatch -> parser/profile_mismatch
compat transport semantic entry -> parser/from_compat_transport_only
```

No warn-only path, implicit Compat2025 retry, or missing-row success is allowed.

## Non-Claims

```text
canonical_default_activated = 0
compat2025_activated = 0
live_parse_witness_conformance = 0
rust_parser_acceptance_migrated = 0
hako_parser_acceptance_migrated = 0
parser_sharing = 0
broad_parser_rewrite = 0
from_to_delegation_normalization = 0
runtime_backend_fallback = 0
type_contract_activation = 0
failure_model_change = 0
selfhost_claim = 0
```

## Follow-Up Order

After this substrate is green, keep the same grammar macro row and select:

```text
Rust Canonical/Compat2025 migration
-> Hako Canonical/Compat2025 migration
-> live strict dual-parser conformance
-> exhaustive v1 registry expansion and generated-view closeout
```

Do not pre-create cards for those steps.
