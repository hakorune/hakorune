---
Status: SSOT
Decision: provisional
Date: 2026-03-22
Scope: repo の物理構造を docs/設計の責務分離に追いつかせるための BoxShape cleanup 順序を固定する。即時の `src/mir` crate split や broad rename は扱わない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cr/README.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# Repo Physical Structure Cleanup (SSOT)

## Goal

- 設計文書の美しさを、repo の物理構造でも読める形へ寄せる。
- root / `CURRENT_TASK.md` / `src/mir` の認知負荷を下げる。
- cleanup を `BoxShape` として進め、受理形追加や broad rename と混ぜない。

## Pressure Snapshot

Local snapshot on 2026-03-22:

- `src/**/*.rs`: `1789` files / `342813` lines
- `lang/**/*.hako`: `451` files / `54853` lines
- `src/mir/**/*.rs`: `1031` files / `210851` lines
- `src/mir/builder` subdirectories: `92`

Current reading:

- 設計哲学は先に整っている
- 物理構造がまだ追いついていない
- まず必要なのは crate split ではなく、入口と衛生の整理

## Reading Rule

This wave is **BoxShape cleanup**.

Do:

- root hygiene
- entry-point thinning
- archive policy
- module/folder responsibility cleanup
- README / SSOT strengthening

Do not mix:

- new language acceptance
- runtime semantics change
- immediate `src/mir` crate split
- broad `nyash -> hako` rename

## Fixed Order

### P0. Root hygiene

Goal:

- repo root を “作業残骸置き場” ではなく “再起動入口” に戻す

Safe first buckets:

- `.gitignore` candidates:
  - `*.err`
  - `*.backup*`
- keep-root allowlist:
  - `basic_test.hako`
  - `test.hako`
- landed archive move targets:
  - docs archive:
    - `CURRENT_TASK_ARCHIVE_2026-01-23.md`
    - `HAKORUNE_RUST_CLEANUP_CAMPAIGN.md`
    - `NUMERIC_CORE_PHI_FIX_SUMMARY.md`
    - -> `docs/archive/cleanup/root-hygiene/`
  - tools archive:
    - `test_joinir_debug.rs`
    - `test_numeric_core_phi.sh`
    - `test_simple_windows.c`
    - `test_using.nyash`
    - `test_len_any`
    - `nyash.toml.backup2`
    - `build.err`
    - `check.err`
    - `llvm.err`
    - `vm.err`
    - `boxbase_identity_consultation_bundle.zip`
    - -> `tools/archive/root-hygiene/`

Rule:

- root の非 allowlist 新規追加は禁止
- 一時物は `tools/archive/root-hygiene/` or scratch
- 履歴物は archive

### P1. `CURRENT_TASK.md` slim

Goal:

- root pointer を cheap restart file に戻す

Keep in root:

- current blocker
- current priority
- exact next files
- reopen conditions
- recent accepted decisions only

Move out:

- long historical residue
- parked lane lore
- completed detail logs

Archive policy:

- archive when the slice is done
- archive when reopen condition is absent
- archive when SSOT/phase README pointers are enough to resume

### P2. `src/` top-level cleanup

Goal:

- flat top-level Rust scatter を減らす

Primary candidates:

- box-ish roots:
  - `box_trait.rs`
  - `box_arithmetic.rs`
  - `box_operators.rs`
  - `method_box.rs`
  - `type_box.rs`
- core-ish roots:
  - `value.rs`
  - `environment.rs`
  - `instance_v2.rs`

Rule:

- facade/re-export first
- physical move second

Landed first slice:

- `box_arithmetic.rs` -> inline facade
- `box_operators.rs` -> `src/boxes/operators/`
- `runner_plugin_init.rs` -> `src/runner/plugin_init.rs`

P2 landed:

- `box_trait.rs` -> `src/boxes/box_trait.rs`
- `operator_traits.rs` -> `src/boxes/operator_traits.rs`
- `channel_box.rs` -> `src/core/channel_box.rs`
- `environment.rs` -> `src/core/environment.rs`
- `exception_box.rs` -> `src/core/exception_box.rs`
- `finalization.rs` -> `src/core/finalization.rs`
- `instance_v2.rs` -> `src/core/instance_v2.rs`
- `method_box.rs` -> `src/core/method_box.rs`
- `scope_tracker.rs` -> `src/core/scope_tracker.rs`
- `type_box.rs` -> `src/core/type_box.rs`
- `value.rs` -> `src/core/value.rs`
- `ast.rs` -> `src/ast/mod.rs`
- `benchmarks.rs` -> `src/benchmarks/mod.rs`
- `wasm_test.rs` -> `src/wasm_test/mod.rs`

P3 first slice landed:

- `src/mir/README.md`
- `src/mir/builder/README.md`
- `src/mir/join_ir/README.md`
- `src/mir/loop_canonicalizer/README.md`
- `src/mir/passes/README.md`
- `src/mir/control_tree/README.md`
- `src/mir/control_tree/step_tree/README.md`
- `src/mir/control_tree/normalized_shadow/README.md`

P4 first slice landed:

- `src/mir/builder/control_flow/plan/normalizer/helpers_pure_value.rs`
- `src/mir/builder/control_flow/plan/normalizer/helpers_layout.rs`
- `src/mir/builder/control_flow/plan/normalizer/helpers_value.rs`
- `src/mir/passes/rc_insertion.rs` facade
- `src/mir/passes/rc_insertion_helpers.rs` implementation split
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_common.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_break_if.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_realworld.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_local.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_condition.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_loop.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_trim_whitespace_helpers.rs`

Next safe slice:

- P5 crate split prep: `src/mir/README.md` / `src/mir/builder/README.md` / `src/mir/passes/README.md`

P5 first packaging slice landed:

- `hakorune_mir_core` package with `types.rs` / `value_id.rs`
- `src/mir/types.rs` / `src/mir/value_id.rs` became thin re-export wrappers

P5 substrate ID slice landed:

- `hakorune_mir_core` package now also owns `basic_block_id.rs` / `binding_id.rs`
- `src/mir/basic_block.rs` re-exports the substrate IDs
- builder / edgecfg / optimizer / tests now use public `crate::mir::{BasicBlockId, EdgeArgs}`
- backend/mir_interpreter now uses public `crate::mir::BasicBlock` / `BasicBlockId`
- remaining README cleanup landed for `contracts/`, `control_tree/`,
  `join_ir_vm_bridge/`, `join_ir_vm_bridge_dispatch/`, and `policies/`

### P3. `src/mir` navigation-first cleanup

Goal:

- `src/mir` を crate split 前に読めるようにする

First non-destructive unit:

- strengthen `src/mir/builder/README.md`
- fix `builder/control_flow/plan/` reading order
- make the top-level map explicit:
  - `core`
  - `builder`
  - `join_ir`
  - `passes`
  - `policies`
  - `verifier`

Rule:

- entry modules and README first
- physical split later

### P4. `src/mir` physical clustering

Goal:

- giant files and local sprawl を減らす

Do:

- split oversized files
- separate helpers / tests / patterns from mixed owner files
- reduce direct deep-path reading

### P5. `src/mir` crate split preparation

Goal:

- only after P0-P4, prepare crate boundaries

Prep doc:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Future targets:

- `hakorune-mir-core`
- `hakorune-mir-builder`
- `hakorune-mir-joinir`
- `hakorune-mir-passes`

Rule:

- do not split before the public/internal API seam is documented

### P6. Naming cleanup

Goal:

- finish `nyash -> hako` cleanup after structure is calmer

Rule:

- naming cleanup is late polish, not the first cleanup wave

Current naming slice:

- MIR substrate packages are now named `hakorune-mir-core` and `hakorune-mir-defs`
- future MIR crate candidates use the `hakorune-mir-*` naming family

## Non-Goals

- immediate `src/mir` crate split
- broad `nyash -> hako` rename
- mixing cleanup with active runtime/compiler blocker work
- turning `CURRENT_TASK.md` into a historical archive again

## First Safe Execution Unit

1. root hygiene contract
2. `CURRENT_TASK.md` archive/slim contract
3. `src/` / `src/mir` cleanup pointers

This is intentionally smaller than crate split.

## Acceptance

- a dedicated phase plan exists
- `CURRENT_TASK.md` points to it
- `10-Now.md` mentions the fixed order
- the P0 first batch is landed: root archive relocation + `*.err` ignore policy
