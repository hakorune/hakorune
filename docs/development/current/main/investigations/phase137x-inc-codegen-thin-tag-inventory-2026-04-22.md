# Phase 137x / `.inc` Codegen Thin Tag Inventory

Status: Active inventory
Date: 2026-04-22
Scope: `.inc` codegen cleanup after value/corridor/runtime ownership cleanup.

## Reading

Claude review and worker inventory agree with the local scan:

- `.hako`: value-only surface is clean enough for this question.
- MIR / corridor / Rust kernel: current value-first and metadata-owned routes
  are clean enough for this question.
- `.inc` codegen remains the only large owner leak: it still performs MIR JSON
  shape analysis and route selection in C.

Current local inventory:

- `lang/c-abi/shims`: 82 files total.
- `.inc`: 76 files, 19,481 lines.
- whole shim directory: 20,134 lines.
- analysis-debt baseline: 30 `.inc` files, 324 debt lines.

The count is intentionally a no-growth baseline, not a deletion target by
itself. The cleanup target is owner ratio: route legality must move to MIR
metadata and `.inc` must only consume a pre-decided tag and emit/fail-fast.

## Main Remaining Owners

1. `hako_llvmc_ffi_pure_compile.inc`
- owns the exact-seed dispatch ladder through `hako_llvmc_match_*_seed`.

2. `hako_llvmc_ffi_generic_method_get_window.inc`
- owns raw instruction-window analysis such as
  `analyze_array_rmw_window_candidate` and
  `analyze_array_string_len_window_candidate`.

3. `hako_llvmc_ffi_generic_method_get_lowering.inc`
- mixes MIR-owned route metadata consumption with legacy C-side window
  analyzers.

4. `hako_llvmc_ffi_string_concat_match.inc`
- still reconstructs concat / substring / direct-set relations from raw MIR.

5. exact seed families
- Array, UserBox, and Sum seed files still match full function shapes in C.

## Existing Good Path

The repo already has the right transport shape:

- MIR computes metadata in `src/mir/semantic_refresh.rs`.
- Function metadata lives in `src/mir/function/types.rs`.
- MIR JSON exposes metadata in `src/runner/mir_json_emit/root.rs`.
- `.inc` already consumes several MIR-owned routes:
  - `array_text_loopcarry_len_store_routes`
  - `array_text_edit_routes`
  - `array_text_residence_sessions`
  - `array_text_observer_routes`
  - `array_text_combined_regions`
  - `array_string_store_micro_seed_route`
  - `concat_const_suffix_micro_seed_route`
  - `substring_views_micro_seed_route`
  - `array_text_state_residence_route`
  - `placement_effect_routes`
  - `value_consumer_facts`

That means the cleanup should extend the metadata-first model, not invent a new
C-side planner.

## Route Tag Contract

MIR must own the pre-decided tag. The tag must contain enough data for `.inc`
to validate location and emit, without rediscovering semantic legality.

Minimal tag fields:

- `route_id`: stable route name, for example `array.rmw_add1.window`.
- `block`: selected block.
- `instruction_index`: selected begin instruction.
- `skip_instruction_indices`: covered MIR instructions to skip after emit.
- `proof`: MIR-owned proof label.
- `effect`: visible effect class, if needed for fail-fast validation.
- `emit_symbol`: runtime/helper symbol or enum selected by MIR.
- `operands`: only concrete values needed by emitter.

Forbidden in `.inc` once a route tag exists:

- scanning later instructions to prove legality
- deciding receiver family from helper names
- deriving alias/publication/consumer legality from raw JSON
- selecting runtime helper variants from benchmark/source names

Allowed in `.inc`:

- read the tag
- validate stable fields
- emit the selected helper call
- mark covered instructions skipped
- fail fast if required tag fields are missing or inconsistent

## First Implementation Order

1. Guardrail: land a no-growth check for current `.inc` analysis debt.
2. `array_rmw_window`: move `analyze_array_rmw_window_candidate` to MIR-owned
   metadata and let `.inc` consume it first, with the old analyzer as temporary
   fallback.
3. `array_string_len_window`: move
   `analyze_array_string_len_window_candidate` to MIR-owned metadata.
4. Generic method route policy: expose the route classification result in MIR
   JSON and demote C policy mirrors to fallback-only.
5. String concat / direct-set windows: consume `string_kernel_plans`,
   `placement_effect_routes`, and `value_consumer_facts` only.
6. Exact seed ladders: convert Array/UserBox/Sum exact seeds to
   function-level backend route tags, then delete the matching ladders one
   family at a time.

## Guard

Added guard:

```bash
tools/checks/inc_codegen_thin_shim_guard.sh
```

Contract:

- new `.inc` files with analysis-debt patterns fail
- per-file analysis-debt count growth fails
- reductions are accepted and reported as allowlist prune opportunities

This preserves behavior while preventing the remaining `.inc` owner leak from
growing during cleanup.
