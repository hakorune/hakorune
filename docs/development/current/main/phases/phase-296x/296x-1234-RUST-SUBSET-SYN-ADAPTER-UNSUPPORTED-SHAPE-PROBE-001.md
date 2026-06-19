---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Prove unsupported Rust source shapes are explicit adapter handoff nodes.
Related:
  - apps/rust-subset-to-hako/examples/unsupported_trait_input.rs
  - apps/rust-subset-to-hako/examples/unsupported_trait_expected.hako
  - apps/rust-subset-to-hako/tools/syn_adapter
  - apps/rust-subset-to-hako/converter_core.hako
  - apps/rust-subset-to-hako/smoke.sh
---

# RUST-SUBSET-SYN-ADAPTER-UNSUPPORTED-SHAPE-PROBE-001

## Decision

Unsupported Rust source items must be explicit RustSubset `Unsupported` nodes.
They must not be silently dropped and must not be encoded as unknown JSON kinds.

V0 probe:

```text
Rust trait item -> {"kind":"Unsupported","rust_kind":"Trait","summary":"Trait items are out of v0 scope"}
```

## Result

```text
unsupported_trait_handoff=explicit_node
converter_unknown_kind_failfast_preserved=1
converter_unsupported_node_comment=1
adapter_silent_drop_count=0
```

The converter emits:

```text
// TODO: Trait items are out of v0 scope
```

## Reproduction

```bash
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not silently drop unsupported Rust constructs
do not treat known unsupported Rust as an unknown converter kind
do not claim trait conversion support
do not add Rust semantic analysis to the adapter
```

## Contract

```text
output_contract=rust-subset-syn-adapter-unsupported-shape-probe-v0

unsupported_trait_input=ok
adapter_outputs_unsupported_node=1
converter_emits_todo_comment=1
unknown_json_kind_failfast_unchanged=1

summary=ok
```
