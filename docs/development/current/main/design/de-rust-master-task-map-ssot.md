---
Status: SSOT
Decision: accepted
Date: 2026-03-14
Scope: 脱Rust完了までの全体順序（lane A/B/C + 29cc orchestration）と done 判定を 1 枚で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md
  - docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/phases/phase-29cf/README.md
  - docs/development/current/main/phases/phase-29x/29x-45-rust-optional-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
---

# De-Rust Master Task Map (SSOT)

## 0. Purpose

- 「脱Rustの本筋」が複数文書に分散して見える状態を解消する。
- 実作業の順序は lane A/B/C を維持しつつ、完了判定だけはこの文書で一本化する。
- 1 blocker = 1 commit の運用を壊さず、daily と closeout の判断距離を短くする。

## 1. Scope Boundary (fixed)

In scope:
1. compiler meaning（lane A）
2. compiler pipeline（lane B）
3. runtime port（lane C）
4. orchestration / residue cleanup（phase-29cc, non-plugin）

Out of scope (separate decision required):
1. plugin 実装の全面 `.hako` 置換
2. 新規言語仕様の拡張
3. fallback を常用する運用

Separate lane pointer:
- plugin 移植準備は `phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md` を正本とする。
- plugin lane `PLG-01` acceptance は `phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md` を正本とする。
- plugin lane `PLG-02` gate pack lock は `phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md` を正本とする。

## 2. Current Snapshot (2026-03-09)

1. lane A: monitor-only（blocker=`none`）
2. lane B: monitor-only（current focus は binary-only contract 維持）
3. lane C: monitor-only（LLVM-first / vm-hako parity monitor）
4. phase-29cc: accepted monitor-only（`RNR-01..05` 完了、top-level closeout done）
5. L4 strict readiness: PASS（`tools/selfhost/check_phase29x_x23_readiness.sh --strict`, 2026-02-25）
6. L5 scope decision: accepted（`de-rust-scope-decision-ssot.md`）
7. de-rust done declaration (non-plugin): accepted（`phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md`）
8. `phase-29ce`: accepted（live compat retirement closeout）
9. `phase-29cf`: accepted monitor-only（`VM fallback compat lane` / `bootstrap boundary reduction` follow-up）

## 2.5 Full Rust 0 Tracking Split (2026-03-14)

1. top-level pointer:
`docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md`
   - operational reading: `stage0 Rust bootstrap keep / stage1 proof / stage2+ 0rust mainline`
2. runtime-zero:
`accepted pointer / inventory-ready`
3. backend-zero:
`accepted pointer / phase-29ck queued`
   - final boundary SSOT: `de-rust-backend-zero-boundary-lock-ssot.md`
   - final shape is `.hako -> thin backend C ABI/plugin boundary -> object/exe`
   - `native_driver.rs` stays bootstrap seam only
   - post-B1/B3 by-name cleanup is a separate `phase-29cl` follow-up for kernel/plugin/backend boundary only; it does not replace `phase-29ce` frontend fixture-key retirement
4. remaining Rust bucket inventory:
`docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md`
5. remaining Rust task pack:
`docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md`
6. rule:
この split は visibility 用であり、lane A/B/C の current blocker や non-plugin done scope を上書きしない。
   - `stage0` Rust bootstrap keep は done failure ではなく、`stage2+` mainline owner cutover が `0rust` の target である。

## 3. Fixed Completion Ladder

1. L1: lane boundary lock
`de-rust-lane-map-ssot.md` の A/B/C 境界と triage rule を維持する。

2. L2: non-plugin residue closeout
`phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md` の fixed order を完了し、monitor-only へ移行する。

3. L3: runtime/route handshake closeout
`phase-29x/29x-62-derust-done-sync-ssot.md` の X32/X33/X34/X35 を replay して証跡を固定する。

4. [done] L4: Rust-optional done readiness
`phase-29x/29x-45-rust-optional-done-sync-ssot.md` の strict readiness（`check_phase29x_x23_readiness.sh --strict`）を満たす。

5. [done] L5: scope decision closeout
plugin を de-rust done の必須条件に含めない（non-plugin done）契約を
`de-rust-scope-decision-ssot.md` で固定する。

## 4. Done Declaration Contract

de-rust done 宣言は次をすべて満たした時だけ許可する。

1. lane A/B/C が monitor-only かつ blocker=`none`
2. X32-X35 replay が PASS
3. Rust-optional strict readiness が PASS
4. done の対象範囲が `de-rust-scope-decision-ssot.md` に明記済み
5. done 宣言証跡が `phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md` で固定済み
6. `stage0` Rust bootstrap keep と `stage2+` daily selfhost mainline が docs 上で切り分けられている

## 5. Failure-Driven Reopen Rules

次のどれかが FAIL したら、対応 lane を blocker 化して fixed order へ戻す。

1. `phase29y_vm_hako_caps_gate_vm.sh` -> lane C
2. `phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh` -> lane B
3. `phase29bq_fast_gate_vm.sh --only bq` -> lane A
4. `phase29x_derust_done_matrix_vm.sh` -> de-rust handshake lane（X32-X35）

## 6. Canonical Commands

Daily (light):
1. `tools/checks/dev_gate.sh quick`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`

Milestone (closeout evidence):
1. `tools/checks/dev_gate.sh milestone-runtime`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_done_matrix_vm.sh`
3. `tools/selfhost/check_phase29x_x23_readiness.sh --strict`

## 7. Entry Wiring Rule

次の入口文書は「詳細を再掲せず、この文書を先頭ポインタにする」。

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/design/de-rust-lane-map-ssot.md`
4. `docs/development/current/main/phases/phase-29cc/README.md`
