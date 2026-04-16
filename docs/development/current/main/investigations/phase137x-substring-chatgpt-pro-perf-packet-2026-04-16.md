# Phase137x Substring ChatGPT Pro Perf Packet (2026-04-16)

Status: consult-packet
Date: 2026-04-16
Scope: `phase-137x` current front `kilo_micro_substring_concat` の current perf gap / hotspot / rejected probes を、外部設計相談へそのまま渡せる形で固定する
Related:
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-137x/README.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/investigations/optimization-structure-chatgpt-pro-consult-2026-04-16.md
- crates/nyash_kernel/src/exports/string_helpers.rs
- crates/nyash_kernel/src/exports/string_helpers/cache.rs
- crates/nyash_kernel/src/exports/string_view.rs
- src/runtime/host_handles.rs

# Purpose

- current `substring_concat` gap を exact numbers で固定する
- “どのように最適化するか” を ChatGPT Pro に相談するための perf-side packet を 1 枚で渡せるようにする
- structure consult と perf consult を分ける

# Fixed Task Card

- front:
  - `kilo_micro_substring_concat`
- accept gate:
  - `kilo_micro_substring_only`
- whole-kilo guard:
  - `kilo_kernel_small_hk`
- first commands:
  - `tools/checks/dev_gate.sh quick`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3`
  - `bash tools/perf/report_mir_hotops.sh kilo_micro_substring_concat`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat 'nyash.string.substring_hii' 3`
- judge:
  - exact front instruction win is primary
  - accept gate must stay healthy
  - whole-kilo is guard only
  - rejected probes are reverted immediately

# Current Gap Snapshot

Latest clean reread on 2026-04-16:

- `kilo_micro_substring_only`
  - C: `1,621,628 instr / 483,969 cycles / 2 ms`
  - Ny AOT: `1,665,429 instr / 986,500 cycles / 3 ms`
  - reading:
    - instructions are near C (`~1.03x`)
    - cycles are still about `~2.04x`
    - wall time is about `~1.5x`

- `kilo_micro_substring_concat`
  - C: `1,621,627 instr / 487,745 cycles / 3 ms`
  - Ny AOT: `779,095,086 instr / 289,761,186 cycles / 68 ms`
  - reading:
    - instructions are about `~480x`
    - cycles are about `~594x`
    - wall time is about `~22.7x`

Conclusion:

- `substring_only` is not the blocker anymore
- the dominant remaining gap is concentrated in `substring_concat`

# MIR / ASM Reading

Current MIR hotops:

- `Method:RuntimeDataBox.substring` x2
- `Method:StringBox.length` x2
- `Extern:nyash.string.substring_concat3_hhhii` x1

Current top symbols on `kilo_micro_substring_concat`:

- `nyash.string.substring_hii`
- `std::thread::local::LocalKey<T>::with`
- `nyash_kernel::exports::string::string_helpers::string_substring_concat3_hhhii_export_impl`
- `nyash_kernel::exports::string_view::borrowed_substring_plan_from_handle`

Main owner files:

- `crates/nyash_kernel/src/exports/string_helpers.rs`
- `crates/nyash_kernel/src/exports/string_helpers/cache.rs`
- `crates/nyash_kernel/src/exports/string_view.rs`
- `src/runtime/host_handles.rs`

Working interpretation:

- the hot gap is not “substring semantics are missing”
- the hot gap is the runtime corridor around:
  - `string_substring_hii_export_impl`
  - `substring_view_arc_cache_lookup`
  - `borrowed_substring_plan_from_handle`
  - registry / TLS access

# Counter Reading

Current counter reread with `NYASH_PERF_COUNTERS=1` on the same front:

- `str.substring.route total=600000`
- `view_arc_cache_handle_hit=0`
- `view_arc_cache_reissue_hit=0`
- `view_arc_cache_miss=600000`
- `fast_cache_hit=0`
- `dispatch_hit=0`
- `slow_plan=600000`

Related counters:

- `birth.placement borrow_view=600000`
- `birth.backend issue_fresh_handle_total=900000`
- `stable_box_demand object_with_handle_latest_fresh=299999`
- `stable_box_demand text_read_triple_latest_fresh=300000`

Interpretation:

- this front is almost entirely `view_arc_cache miss -> slow_plan`
- every loop iteration hits two `substring()` calls on the current `text`
- the main gap is not a broad optimizer-architecture mystery; it is a concrete runtime corridor problem

# Rejected Probes In This Wave

## 1. latest-fresh skip for `substring_view_arc_cache_lookup`

- touched:
  - `src/runtime/host_handles.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
- idea:
  - skip the TLS cache lookup when the source handle looks like a fresh churn handle
- result:
  - `kilo_micro_substring_concat = 763,797,981 instr / 74 ms`
- verdict:
  - rejected
- why:
  - exact front did not improve
  - added predicate/hint logic did not beat the baseline

## 2. root-span cache fast path ahead of `borrowed_substring_plan_from_handle`

- touched:
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - temporary unit tests only
- idea:
  - let root `StringBox` handles reuse `string_span_cache` before entering `with_handle(...)`
- result:
  - first cut: `865,796,378 instr / 80 ms`
  - refined cut: `818,996,145 instr / 79 ms`
- verdict:
  - rejected
- why:
  - new hot owners appeared:
    - `string_span_cache_put`
    - `string_span_cache_get`
    - `Option::or_else`
    - `trace_borrowed_substring_plan`
  - extra cache/helper traffic cost more than the saved fallback work

# What Is Sufficiently Known

- the optimization method is explicit and stable
- the active front / gate / guard are pinned in SSOT
- the C gap is measured and current
- the hotspot corridor is identified
- rejected local cuts already show what not to retry blindly

This is sufficient to ask an external reviewer how the corridor should be optimized structurally.

# One High-Value Missing Observation

This is the single missing observation that would help choose the next local cut more confidently:

- split `borrowed_substring_plan_from_live_object(...)` by match-arm cost
  - `ReturnHandle`
  - `ReturnEmpty`
  - `FreezeSpan`
  - `ViewSpan`

Reason:

- current readings aggregate the slow path, but do not isolate whether the next narrow cut should target:
  - cache lookup / TLS mechanics
  - plan construction shape
  - `StringSpan` / `Arc::clone` overhead inside the live-object classifier

This is useful, but not required, before asking ChatGPT Pro.

# Pasteable ChatGPT Pro Prompt

```text
I want a hard-nosed optimization design review for Hakorune’s current substring hotspot.

Context:
- authority order is `.hako -> MIR -> Rust kernel -> LLVM`
- current optimization taxonomy has already been refreshed; MIR owns substrate, Rust is executor, LLVM is consumer
- I do not want string-specific MIR dialects or Rust-side hardcoded special cases
- I want the smallest generic mechanism that still lets the runtime corridor approach C

Current task card:
- front: `kilo_micro_substring_concat`
- accept gate: `kilo_micro_substring_only`
- whole-kilo guard: `kilo_kernel_small_hk`
- judge: 3 runs + asm, exact front instruction win first

Current measured gap:
- `kilo_micro_substring_only`
  - C: `1,621,628 instr / 483,969 cycles / 2 ms`
  - Ny AOT: `1,665,429 instr / 986,500 cycles / 3 ms`
- `kilo_micro_substring_concat`
  - C: `1,621,627 instr / 487,745 cycles / 3 ms`
  - Ny AOT: `779,095,086 instr / 289,761,186 cycles / 68 ms`

Current hotspot reading:
- MIR hotops:
  - `RuntimeDataBox.substring` x2
  - `StringBox.length` x2
  - `nyash.string.substring_concat3_hhhii` x1
- top symbols:
  - `nyash.string.substring_hii`
  - `std::thread::local::LocalKey<T>::with`
  - `string_substring_concat3_hhhii_export_impl`
  - `borrowed_substring_plan_from_handle`
- owner files:
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/cache.rs`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `src/runtime/host_handles.rs`

Current route counters:
- `str.substring.route total=600000`
- `view_arc_cache_handle_hit=0`
- `view_arc_cache_reissue_hit=0`
- `view_arc_cache_miss=600000`
- `fast_cache_hit=0`
- `dispatch_hit=0`
- `slow_plan=600000`

Interpretation:
- this front is almost entirely `view_arc_cache miss -> slow_plan`
- `substring_only` is close enough to C that it is no longer the blocker
- the remaining gap is concentrated in the substring-concat runtime corridor

Rejected probes:
1. Skip `substring_view_arc_cache_lookup` on latest-fresh handles
   - result: `763,797,981 instr / 74 ms`
   - rejected
2. Root-span cache fast path before `borrowed_substring_plan_from_handle`
   - first cut: `865,796,378 instr / 80 ms`
   - refined cut: `818,996,145 instr / 79 ms`
   - rejected
   - extra cost showed up in `string_span_cache_put/get` and helper overhead

Question:
- Given this evidence, what is the most structurally correct way to optimize this corridor without violating the architecture?
- Should the next move stay purely in the Rust runtime corridor, or should more truth move upward into MIR metadata/contracts?
- If you had to propose the next 3 cuts, what would they be, in order, and why?
- Which cuts are attractive but likely wrong, given the evidence above?

I want:
- findings first
- then a proposed end-state for the substring corridor
- then a recommended rollout order
- be explicit about what must stay in Rust, what should move to MIR, and what should be left to LLVM
```

