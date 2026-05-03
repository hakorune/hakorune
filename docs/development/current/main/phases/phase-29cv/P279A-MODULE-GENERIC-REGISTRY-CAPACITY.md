---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P279a, module generic definition registry capacity
Related:
  - docs/development/current/main/phases/phase-29cv/P278A-COUNT-PARAM-CALLSITE-TEXT-MATERIALIZATION.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
---

# P279a: Module Generic Registry Capacity

## Problem

After P278a, the active source-execution probe reaches a pure-first boundary:

```text
[llvm-pure/unsupported-shape] ... reason=no_lowering_variant
```

The route metadata shows the previous concrete blocker is gone:

```text
LowerLoopCountParamBox.try_lower_text/1	generic_pure_string_body	string_handle	DirectAbi
```

The next failure happens before a fresh `.ll` is produced. A reachability
inventory over `tmp/nyash_cli_emit.json` finds 432 DirectAbi module symbols
reachable from `main`, while the C-side module generic definition registry has
fixed 256-entry storage:

```text
planned_module_generic_string_symbols[256]
emitted_module_generic_string_symbols[256]
```

That capacity failure currently returns `-1` without recording a precise
unsupported-shape reason, so it surfaces as generic `no_lowering_variant`.

## Decision

Raise the registry storage to 1024 entries and keep capacity overflow
fail-fast with an explicit existing-trace diagnostic reason:

```text
module_generic_registry_full
```

This is a capacity/diagnostic fix only. It does not add any new accepted body
shape, method semantics, or emitter path.

## Non-Goals

- no new `GlobalCallTargetShape`
- no collection/string method widening
- no body-specific C emitter addition
- no source behavior change

## Acceptance

- `ny-llvmc --in tmp/nyash_cli_emit.json --emit obj ...` no longer stops before
  `.ll` generation because the module generic registry is full.
- If the registry ever fills again, `[llvm-pure/unsupported-shape]` reports
  `module_generic_registry_full` rather than bare `no_lowering_variant`.
- `cargo build -q --release --bin hakorune`
- `bash tools/build_hako_llvmc_ffi.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

The fresh source-execution probe now produces a captured `.ll` file and reports
the next explicit module-generic blocker:

```text
[llvm-pure/unsupported-shape] ... reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderDelegateFinalizeBox._build_user_box_decls_from_program_json/1
```

The previous anonymous registry-capacity failure is no longer the first stop.
The captured partial `.ll` still contains a later invalid-IR issue in
`BodyExtractionBox.extract_balanced_body/2` (`%r294`), but this card leaves that
as a separate blocker so the registry capacity fix stays behavior-neutral.
