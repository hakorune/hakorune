---
Status: Active
Date: 2026-04-22
Scope: `.inc` codegen thin-tag cleanup design brief.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - docs/development/current/main/investigations/phase137x-inc-codegen-thin-tag-inventory-2026-04-22.md
---

# 292x-90: `.inc` Codegen Thin Tag Design Brief

## Problem

The value/runtime/corridor cleanup is mostly metadata-owned, but `.inc` still
contains C-side MIR shape analysis. That makes codegen a hidden planner:

- it scans `blocks` / `instructions` / `op`
- it reconstructs instruction windows
- it decides route legality in C
- it selects helper variants from local pattern matches

That is the wrong owner. `.inc` is the C ABI boundary and should stay glue.

## Target Shape

MIR emits a backend route tag before codegen:

```text
route_id
block
instruction_index
skip_instruction_indices
proof
effect
emit_symbol
operands
```

`.inc` consumes the tag:

```text
read metadata
validate required fields
emit selected helper call
skip covered instructions
fail fast when metadata is inconsistent
```

## Non Goals

- no public ArrayBox/StringBox/MapBox ABI change
- no runtime-owned legality inference
- no benchmark-name route dispatch
- no broad `.inc` rewrite before one route family proves the shape
- no removal of legacy fallback until the replacement route is pinned by smoke

## First Route Family

Start with `array_rmw_window` because it is a small, visible owner leak:

- current C owner:
  - `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_window.inc`
  - `analyze_array_rmw_window_candidate`
- current lowering consumer:
  - `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_lowering.inc`
- expected MIR owners:
  - `src/mir/function/types.rs`
  - `src/mir/semantic_refresh.rs`
  - `src/runner/mir_json_emit/root.rs`

## Acceptance

- `.inc` analysis-debt guard stays green
- `array_rmw_window` route metadata appears in MIR JSON for the active fixture
- `.inc` prefers the route tag and only falls back to the old analyzer when the
  tag is absent
- behavior and existing smokes stay green
- route trace proves `reason=mir_route_plan` or equivalent stable tag wording
