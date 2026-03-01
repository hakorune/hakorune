---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の kernel B3（future/invoke/mod）完了条件を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-239-kernel-b3-min1-future-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-240-kernel-b3-min2-invoke-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/mod.rs
---

# 29cc-241 Kernel B3 Closeout Lock

## Purpose

kernel residue B3（`future.rs` / `invoke.rs` / `mod.rs`）の route 契約を closeout し、kernel residue 全体 closeout へ進む。

## B3 Closeout Criteria (fixed)

1. B3-min1（future route lock）と B3-min2（invoke route lock）が完了している。
2. `plugin::mod` の public wiring で B3 対象の入口が維持され、ABI export 面を変更しない。
3. invalid/unresolved 入力の fail-fast 戻り値契約（`0` / `0.0`）がテストで固定されている。
4. source 削除は行わない（no-delete-first 維持）。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `tools/checks/dev_gate.sh runtime-exec-zero` green
4. `phase29y_no_compat_mainline_vm.sh` green

## Execution update

- 2026-03-01: `crates/nyash_kernel/src/plugin/mod.rs` に
  `b3_public_wiring_contract_compiles` を追加。
  B3対象 entrypoints の crate-root re-export を function signature bind で固定し、
  mod wiring drift を compile-time/test で fail-fast 検知できるようにした。
- 2026-03-01: `invoke_core` / `compat_invoke_core` の未使用 placeholder helper を撤去し、
  B3 compat payload encode 経路を `encode_legacy_vm_args_range()` へ一本化した。
- 2026-03-01: `tools/checks/dev_gate.sh runtime-exec-zero` に
  `plugin::wiring_tests::b3_public_wiring_contract_compiles` を組み込み、
  B3 wiring lock を日常ゲートに昇格。

## Next Boundary (fixed)

1. kernel residue closeout（B1/B2/B3 統合 lock）
