# RUST-SUBSET-SYN-ADAPTER-MATCH-UNSUPPORTED-HANDOFF-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape handoff

## Decision

Accept Rust `match` only as an explicit unsupported handoff.

No RustSubset `Match` node is introduced and no match-arm semantics are
implemented. The syn adapter emits:

```json
{"kind":"Unsupported","reason":"Rust match expression is out of v0 scope"}
```

The Python and `.hako` converters then emit a TODO comment. This keeps the
external adapter stable for real Rust input while preserving fail-fast for
unknown schema kinds.

## Implementation

Added:

```text
Expr::Match -> Unsupported("Rust match expression is out of v0 scope")
```

Hardened:

```text
converter_core.hako now preserves Unsupported.reason for statement/expression
TODO comments, matching the Python reference converter.
```

Added fixture:

```text
apps/rust-subset-to-hako/examples/match_unsupported_input.rs
apps/rust-subset-to-hako/examples/match_unsupported_subset.json
apps/rust-subset-to-hako/examples/match_unsupported_expected.hako
apps/rust-subset-to-hako/convert_match_unsupported_fixture.hako
```

## Evidence

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  apps/rust-subset-to-hako/examples/match_unsupported_input.rs \
  --module match_unsupported_fixture \
  -o apps/rust-subset-to-hako/examples/match_unsupported_subset.json

python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/match_unsupported_subset.json \
  | diff -u apps/rust-subset-to-hako/examples/match_unsupported_expected.hako -
```

Acceptance gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not implement match semantics
do not add a RustSubset Match node
do not desugar match arms into if/else
do not accept break/continue through this app-front row
do not mix compiler Recipe/CorePlan acceptance with source-shape handoff
```

## Report

```text
output_contract=rust-subset-syn-adapter-match-unsupported-handoff-v0
selected_shape=match_unsupported_handoff
schema_node_added=0
match_semantics_enabled=0
unsupported_reason_stable=1
converter_core_preserves_unsupported_reason=1
fixture_added=match_unsupported
compiler_recipe_acceptance_changed=0
summary=ok
```
