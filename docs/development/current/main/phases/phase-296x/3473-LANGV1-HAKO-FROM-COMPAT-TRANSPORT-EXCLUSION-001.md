# 3473 - LANGV1-HAKO-FROM-COMPAT-TRANSPORT-EXCLUSION-001

## Status

Active code-facing implementation card after 3471 accepts Decision A.

Decision: accepted.

## Selected Contract

```text
Hako semantic-parser conformance:
  normalization_mode = compatibility_transport -> ExplicitlyExcluded

transport producer:
  Rust migration tooling only

Hako transport producer:
  forbidden
```

The registry has no `transport_owner` column. Do not infer transport ownership
from fixture names, source spelling, reject tags, missing Hako routes, or test
counts. The exclusion derives only from the typed
`NormalizationMode::CompatibilityTransport` contract and the grammar contract.

## Structural Implementation

1. Add a parser-neutral conformance-scope projection in
   `hakorune_frontend_grammar`:

   ```text
   participant = HakoSemanticParser
   normalization = CompatibilityTransport
   result = ExplicitlyExcluded(RustMigrationToolingOnly)
   ```

2. Make the Python corpus adapter parse `grammar/unified-grammar.toml` and
   project the same row/profile classification.
3. Report excluded rows explicitly with fixture ID, row ID, profile, stable
   tag, and transport owner.
4. Keep excluded rows out of the Hako adapter invocation set while retaining
   them in the report and count checks.
5. Include both Compat2025 `from` families in the full guard and assert the
   explicit exclusion count and tag.
6. Keep the existing Rust migration-transport and semantic-entry veto tests
   green.

Do not add a Hako transport adapter, alter Hako parser behavior, or create an
AST, MIR, runtime, or backend path for compatibility transport.

## Report Contract

```text
row_status = excluded
stable_reject_tag = parser/hako_transport_row_excluded
transport_owner = RustMigrationToolingOnly
hako_adapter_invoked = false
```

Fail fast when a registry row is missing, a profile differs, transport reaches
the Hako adapter, or a non-transport row is excluded.

## Stable Tags

```text
parser/hako_transport_row_excluded
parser/hako_transport_producer_forbidden
parser/hako_transport_scope_drift
parser/hako_non_transport_row_exclusion_forbidden
parser/transport_to_semantic_forbidden
parser/transport_to_ast_forbidden
parser/transport_to_mir_forbidden
parser/transport_to_runtime_forbidden
parser/transport_to_backend_forbidden
```

## Fixture Matrix

```text
Hako semantic conformance:
  box_from_inheritance_compat_transport -> explicit exclusion
  box_from_inheritance_semantic_entry_reject -> explicit exclusion
  from_super_call_compat_transport -> explicit exclusion
  from_super_call_semantic_entry_reject -> explicit exclusion

Rust migration tooling:
  both compatibility-transport fixtures -> CompatibilityTransport retained

Rust semantic parser:
  both semantic-entry fixtures -> parser/from_compat_transport_only

Hako adapter invocation set:
  all explicitly excluded fixtures absent
```

Canonical Hako `from` rejection is not changed or closed by this card because
Canonical rows are not compatibility-transport rows.

## Acceptance

```text
hako_compatibility_transport_explicit_exclusion = 1
hako_transport_exclusion_owner_count = 1
rust_migration_tooling_only_transport_producer = 1
silent_transport_skip = 0
hako_transport_adapter_invocation_count = 0
compat_transport_ast_authorized = 0
from_semantic_lowering = 0
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

Verification:

```bash
cargo test -q -p hakorune-frontend-grammar
python3 -m unittest tools.language_v1.test_hako_corpus_batch
bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
LANGV1_HAKO_PROFILE_FULL=1 bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-Claims

```text
hako_from_transport_implemented = 0
hako_transport_producer = 0
hako_parser_behavior_changed = 0
hako_canonical_from_rejection_closeout = 0
hako_parse_witness_conformance = 0
language_v1_grammar_closeout = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Next

After this card is green, proceed directly to the accepted
`LANGV1-HAKO-MATCH-RECORD-DELIMITER-OWNER-001` task. Do not open an inventory,
rerun, or second consultation card between them.
