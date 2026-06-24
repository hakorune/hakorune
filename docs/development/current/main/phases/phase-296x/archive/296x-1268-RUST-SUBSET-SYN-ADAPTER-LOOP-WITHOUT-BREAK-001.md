# RUST-SUBSET-SYN-ADAPTER-LOOP-WITHOUT-BREAK-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape support

## Decision

Accept Rust `loop { ... }` only when the loop body contains no `break` or
`continue`.

No new RustSubset statement node is introduced. The adapter maps the shape to
the existing `While` statement with a literal boolean `true` condition:

```text
Rust:       loop { body }
RustSubset: {"kind":"While","cond":{"kind":"Literal","type":"bool","value":true},"body":[...]}
.hako:      loop(true) { body }
```

Loops containing `break` or `continue` remain compiler Recipe/CorePlan backlog
and are represented as `Unsupported` by this app-front lane.

## Implementation

Added:

```text
Expr::Loop -> loop_to_json()
block_has_break_or_continue()
```

Added fixture:

```text
apps/rust-subset-to-hako/examples/loop_forever_input.rs
apps/rust-subset-to-hako/examples/loop_forever_subset.json
apps/rust-subset-to-hako/examples/loop_forever_expected.hako
apps/rust-subset-to-hako/convert_loop_forever_fixture.hako
```

## Evidence

```text
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/loop_forever_subset.json
```

Acceptance gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not accept break or continue in this app-front row
do not add a RustSubset Loop node
do not desugar loop with break into while/if source text
do not mix compiler Recipe/CorePlan acceptance with source-shape transport
```

## Report

```text
output_contract=rust-subset-syn-adapter-loop-without-break-v0
selected_shape=loop_without_break
schema_node_added=0
while_schema_reused=1
loop_true_condition_used=1
break_continue_supported=0
break_continue_unsupported_handoff=1
fixture_added=loop_forever
compiler_recipe_acceptance_changed=0
summary=ok
```
