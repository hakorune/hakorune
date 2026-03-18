---
Status: rejected
Date: 2026-03-19
Scope: `substring_hii` structure-first split trial that moved `StringViewBox` birth from planner to caller-side freeze
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- crates/nyash_kernel/src/exports/string.rs
- crates/nyash_kernel/src/exports/string_view.rs
---

# Substring Structure-First Plan/Freeze Split (Rejected)

## Trial

Tried a minimal structure-first split:

- `BorrowedSubstringPlan::Materialize(String)` -> `OwnedSubstring(String)`
- `BorrowedSubstringPlan::CreateView(StringViewBox)` -> `ViewRecipe { base_handle, base_obj, start, end }`
- `borrowed_substring_plan_from_handle(...)` stopped constructing `StringViewBox`
- `substring_hii` created `StringViewBox::new(...)` at the final match site

Intent:

- align code shape with the future `freeze` boundary design
- make planner thinner without changing the flat `<= 8 bytes` policy

## Verification

Passed:

- `cargo test -q -p nyash_kernel substring_hii -- --nocapture`
- `cargo test -q -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
- `cargo check -q -p nyash_kernel`

Perf:

- `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 9`
  - `ny_aot_cycles=267397179`
  - `ny_aot_ms=72`
- `PERF_VM_FORCE_NO_FALLBACK=1 PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small_hk auto 5 5 11`
  - round 1: `1146 ms`
  - round 2: `904 ms`
  - round 3: `901 ms`
  - round 4: `794 ms`
  - round 5: `802 ms`
  - median: `901 ms`
  - min/max: `794 / 1146`

Baseline for keep:

- stable median must stay `<= 804 ms`

## Result

Rejected and reverted immediately.

Reason:

- the split improved code shape slightly but did not create a real transient carrier
- birth still happened at the same effective density
- stable whole-program regressed from `804 ms` to `901 ms`

## Decision

Do not reopen this exact cut.

Allowed future direction:

- introduce a real transient carrier (`TStr` / freeze boundary design)
- then move birth into a single `freeze` sink

Forbidden repeat:

- planner-side `OwnedSubstring/ViewRecipe` plus caller-side `StringViewBox` freeze without a real transient/string token layer
