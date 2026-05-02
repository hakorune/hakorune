---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P182b, JsonFrag instruction array normalizer shape classification
Related:
  - docs/development/current/main/phases/phase-29cv/P182A-GENERIC-STRING-CLASSIFIER-BOUNDARY.md
  - src/mir/global_call_route_plan/jsonfrag_normalizer_body.rs
  - src/mir/global_call_route_plan/model.rs
---

# P182b: JsonFrag Instruction Array Normalizer Shape

## Problem

P182a fixed the boundary: `generic_string_body` must not absorb the
`JsonFragNormalizerBox._normalize_instructions_array/1` collection-normalizer
body.

The next step needs a separate shape owner so route metadata can distinguish:

- a generic string flow body
- a collection normalizer body that currently is not lowerable by the generic
  string emitter

## Decision

Add `jsonfrag_instruction_array_normalizer_body` as a dedicated body shape.

This card classifies the structural normalizer pattern only. It does not make
the target DirectAbi-lowerable yet.

Required structural facts:

- one string-handle-compatible parameter and string-handle-compatible return
- at least two `ArrayBox` births
- at least one `MapBox` birth
- `ArrayBox.push` flow
- `MapBox.get` and `MapBox.set` flow
- string surface
- null/void sentinel const surface
- value return

This remains structural. There is no `JsonFragNormalizerBox` or
`_normalize_instructions_array` by-name exception in `generic_string_body`.

## Lowering Contract

`jsonfrag_instruction_array_normalizer_body` is a known shape but not yet a
DirectAbi target.

That means:

- `target_shape=jsonfrag_instruction_array_normalizer_body`
- `tier=Unsupported`
- `reason=missing_multi_function_emitter`
- no `return_shape`

The C emitter must be added in a later card after its collection method support
is explicit.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_jsonfrag_instruction_array_normalizer_shape --lib
cargo test -q refresh_module_global_call_routes_accepts_self_recursive_generic_pure_string_body --lib
cargo test -q runtime_methods --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
if rg -n 'JsonFragNormalizerBox|_normalize_instructions_array' src/mir/global_call_route_plan/generic_string_body.rs; then exit 1; fi
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
