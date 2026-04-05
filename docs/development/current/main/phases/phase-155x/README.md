# Phase 155x: perf canonical visibility tighten

- Status: Landed
- 目的: `phase-137x` の exact perf front を Rust executor 名だけでなく canonical contract reading から追えるように固定する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`

## Goal

- perf front は次の canonical reading から先に読めること
  1. `store.array.str`
  2. `thaw.str + lit.str + str.concat2 + freeze.str`
- Rust 関数名は executor detail としてだけ扱う
- latest bundle anchor を current docs に固定する

## Current Front

1. `store.array.str`
   - current executor: `crates/nyash_kernel/src/plugin/array_string_slot.rs::array_string_store_handle_at(...)`
   - exact micro: `kilo_micro_array_string_store`
2. `const_suffix`
   - canonical reading: `thaw.str + lit.str + str.concat2 + freeze.str`
   - current executor: `crates/nyash_kernel/src/exports/string_helpers.rs::concat_const_suffix_fallback(...)`
   - exact micro: `kilo_micro_concat_const_suffix`

## Bundle Anchor

- latest summary:
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/summary.txt`
- latest asm:
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/asm/perf_report.txt`

## Exit

- `phase-137x` README / current mirrors all read the same front order
- bundle anchor is fixed in current docs
- `phase-137x` can resume without losing canonical reading
