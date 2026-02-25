---
Status: Active (RDM-1 done, RDM-2 done)
Decision: provisional
Date: 2026-02-17
Scope: runtime lane に残る Rust 側の意味決定点（Program->MIR / shape 受理）を赤棚卸しし、1項目ずつ `.hako` SSOT へ移管する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md
  - tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh
---

# De-Rust Runtime Meaning-Decision Red Inventory (SSOT)

## Purpose

- runtime lane で Rust が意味決定をしている箇所を赤棚卸しする。
- 1 red item = 1 commit で `.hako` へ移管し、mainline は compat fallback なしを維持する。

## Rule lock

- BoxCount（受理形追加）と BoxShape（責務整理）を同コミットで混ぜない。
- 受理境界を変えたコミットは fixture + gate まで同コミットで固定する。
- mainline gate FAIL 中は inventory 追加だけを先行しない。

## Current red inventory (2026-02-24)

| ID | Status | Boundary | Rust-side meaning decision (red) | Evidence | Current guard | Target state | Exit criteria |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RDM-1 | done (2026-02-17) | Selfhost Stage-A runtime route | `.hako` mirbuilder miss 時に `Program(JSON v0)` を Rust `json_v0_bridge` で MIR 化し、runtime route が shape 受理を決めている | `src/runner/selfhost.rs:416`, `src/runner/selfhost.rs:481`, `src/runner/route_orchestrator.rs:223` | `tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`, `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh` | Program->MIR の shape/meaning は `.hako` compiler lane のみが決める。runtime route は MIR(JSON) 受理専用。 | `LANE_COMPAT_RUST_JSON_V0_BRIDGE` は `NYASH_VM_USE_FALLBACK=1` 明示時のみ許可（mainline default は fail-fast）+ lane gate 緑 |
| RDM-2 | done (min1..min5: 2026-02-24) | Runtime dispatch direct-v0 bridge | `--parser ny` / `NYASH_USE_NY_PARSER=1` による Program->MIR direct-v0 bridge 入口を mainline から撤去（CLI flag removed, env is legacy no-op） | `src/cli/args.rs`, `src/runner/dispatch.rs`, `src/runner/modes/common_util/selfhost/runtime_route_contract.rs` | `tools/checks/phase29y_direct_v0_retirement_guard.sh`, `tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh`, `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`, `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh` | Program->MIR 変換は lane B（`.hako` compiler/mirbuilder）へ一本化。dispatch は実行経路選択のみ。 | parser flags entrypoints removed and lane gates stay green |

## RDM-2 post-delete note

- direct-v0 bridge の rollback 導線は削除済み。
- `--parser ny` は削除済み（CLI reject）。
- `NYASH_USE_NY_PARSER=1` は legacy no-op（mainline route 不変）。
- mainline は lane gate（quick/full）で retired 境界を継続監視する。

## Next single task (fixed)

1. app-first 運用: 新規実装は `.hako` app/compiler lane を主軸にし、Rust 側は boundary guard と fail-fast のみ扱う。
2. acceptance commands (mainline lock):
   - `bash tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh`
   - `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`
   - `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
