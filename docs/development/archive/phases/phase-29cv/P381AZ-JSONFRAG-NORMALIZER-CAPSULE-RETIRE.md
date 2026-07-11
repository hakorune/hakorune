---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `JsonFragInstructionArrayNormalizerBody` as a distinct `GlobalCallTargetShape` capsule
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/model.rs
  - src/mir/global_call_route_plan/jsonfrag_normalizer_body.rs
  - src/mir/global_call_route_plan/tests/jsonfrag_normalizer.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P381AZ: JsonFrag Normalizer Capsule Retire

## Problem

`JsonFragInstructionArrayNormalizerBody` still existed as a temporary public
shape even though its published route contract already matched the existing
generic pure string lane:

- same lowering tier and emit kind: direct same-module function call
- same return contract: `string_handle`
- same Stage0 origin handling and selected-set emission path

That left an unnecessary public split between MIR classification and the
published LoweringPlan/Stage0 contract.

## Decision

Retire `JsonFragInstructionArrayNormalizerBody` as a public route shape and
collapse accepted routes into `GenericPureStringBody`.

Implemented:

- removed the public enum/proof surface for the jsonfrag normalizer capsule
- kept `jsonfrag_normalizer_body.rs` only as a narrow MIR-side acceptance shim
- updated route tests to expect `typed_global_call_generic_pure_string`
- removed Stage0 jsonfrag-specific direct-call readers/branches so generic
  string handling now owns the public contract

## Boundary

Allowed:

- collapsing a temporary public string-handle capsule into an existing
  contract-equivalent string shape
- preserving the narrow MIR-side recognizer while source-owner cleanup is still
  pending

Not allowed:

- widening Stage0 with JsonFrag-specific semantics
- teaching the backend array/map normalization policy
- changing unrelated object-return capsules in the same card

## Acceptance

```bash
cargo fmt --all
cargo test --release jsonfrag_normalizer -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Result

Done:

- `JsonFragInstructionArrayNormalizerBody` no longer exists as a public
  MIR/LoweringPlan shape
- accepted jsonfrag-normalizer routes now reuse `generic_pure_string_body`
- Stage0 no longer carries a dedicated public jsonfrag direct-call contract

Next:

1. continue capsule retirement with the next thin collection/object candidate
2. current best next candidate is `StaticStringArrayBody`
