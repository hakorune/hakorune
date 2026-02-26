---
Status: SSOT
Date: 2026-02-25
Scope: main ラインの「現在地」と「実行入口」だけを置く薄いインデックス。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/design/private-doc-boundary-migration-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-116-plg06-egui-wave3-min4-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-117-wsm01-wasm-unsupported-inventory-sync-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-119-wsm02a-assignment-local-unblock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md
  - docs/tools/README.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は入口専用。進捗履歴や長文ログは phase/design へ置く。
- Next task の正本は `phase-29y/60-NEXT-TASK-PLAN.md` に固定する。

## Quick Restart Pointer

- 再起動直後の最短導線は `docs/development/current/main/05-Restart-Quick-Resume.md` を正本とする。
- 実行順と blocker の参照先を 1 画面で辿れる状態を保つ。

## Current Snapshot

- Compiler lane: `phase-29bq`（JIR-PORT-00..07 done / active blocker=`none` / next=`none`）
- Lane A status mirror SSOT: `CURRENT_TASK.md`（この文書は mirror）
- Lane A mirror sync helper: `bash tools/selfhost/sync_lane_a_state.sh`（`CURRENT_TASK.md` を唯一入力に同期）
- Runtime lane: `phase-29y`（Current blocker / Next fixed order は `phase-29y/60-NEXT-TASK-PLAN.md` を正本とする）
- Runtime operation policy: `LLVM-first / vm-hako monitor-only`（日常の runtime 検証は LLVM 主経路、vm-hako は blocker 検知の monitor lane）
- JoinIR port mode（lane A）: monitor-only（failure-driven）
- JoinIR parity probe pin（JIR-PORT-01）:
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port01_parity_probe_vm.sh`
- App-first: APP-1（Gate Log Summarizer）acceptance PASS 済み
- App-first: APP-2（Controlflow Probe）acceptance PASS 済み
- App-first: APP-3（MIR Shape Guard）acceptance PASS 済み
- Runtime diagnostic pin（non-gating）:
  - `phase29y_continue_assignment_in_continue_stale_guard_vm.sh`（stale-guard contract: `FINAL=7`）
- Compiler pipeline diagnostic pin（non-gating）:
  - `phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`（blocked contract: `--hako-emit-mir-json` internal timeout fail-fast marker）
  - `phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`（monitor-only: pre-emit I/O cold/hot observation; `--strict` は手動実行）
  - `phase29y_hako_emit_mir_binary_only_ported_vm.sh`（repo外 `--hako-emit-mir-json` ported contract）
- Compiler pipeline focus（lane B）:
  - `binary-only --hako-emit-mir-json` 契約を優先（SSOT: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`）
  - active next: `none`（B-TERNARY-03 decision fixed: non-gating維持）
  - task SSOT: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` の `Lane-B Nested Ternary Debt Pack (B-TERNARY-01..03)`
- De-rust orchestration lane（phase-29cc）:
  - `plugin-wave-rollout`（RNR-05 complete; PLG-03 done; PLG-04-min1..min6 done; wave-1 complete; PLG-05-min1/min2/min3/min4/min5/min6/min7 done; PLG-06-min1/min2/min3/min4 done）
  - scope decision（L5）: `docs/development/current/main/design/de-rust-scope-decision-ssot.md`（accepted）
  - strict readiness（L4）: `tools/selfhost/check_phase29x_x23_readiness.sh --strict` -> `status=READY`（2026-02-25）
  - done declaration（non-plugin）: `docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md`（accepted）
  - plugin lane bootstrap（docs-first）: `docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md`（provisional）
  - plugin lane ABI lock（PLG-01 done）: `docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`（accepted）
  - plugin lane gate pack lock（PLG-02 done）: `docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md`（accepted）
  - plugin lane wave-1 pilot lock（PLG-03 done）: `docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md`（accepted）
  - plugin lane wave rollout lock（PLG-04-min1 done）: `docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md`（accepted）
  - plugin lane wave rollout lock（PLG-04-min2 done）: `docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`（accepted）
  - plugin lane wave rollout lock（PLG-04-min3 done）: `docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md`（accepted）
  - plugin lane wave rollout lock（PLG-04-min4 done）: `docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md`（accepted）
  - plugin lane wave rollout lock（PLG-04-min5 done）: `docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md`（accepted）
  - plugin lane wave rollout lock（PLG-04-min6 done）: `docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md`（accepted）
  - post-wave1 route lock（accepted）: `docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md`
  - plugin wave-2 entry lock（PLG-05-min1 done）: `docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md`（accepted）
  - plugin wave-2 rollout lock（PLG-05-min2 done）: `docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md`（accepted）
  - plugin wave-2 rollout lock（PLG-05-min3 done）: `docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md`（accepted）
  - plugin wave-2 rollout lock（PLG-05-min4 done）: `docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md`（accepted）
  - plugin wave-2 rollout lock（PLG-05-min5 done）: `docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md`（accepted）
  - plugin wave-2 rollout lock（PLG-05-min6 done）: `docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md`（accepted）
  - plugin wave-2 rollout lock（PLG-05-min7 done）: `docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md`（accepted）
  - plugin wave-3 entry lock（PLG-06-min1 done）: `docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md`（accepted）
  - plugin wave-3 rollout lock（PLG-06-min2 done）: `docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md`（accepted）
  - plugin wave-3 rollout lock（PLG-06-min3 done）: `docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md`（accepted）
  - plugin wave-3 rollout lock（PLG-06-min4 done）: `docs/development/current/main/phases/phase-29cc/29cc-116-plg06-egui-wave3-min4-ssot.md`（accepted）
  - plugin lane active next: `none`（monitor-only）
  - wasm lane status SSOT（active next / latest lock / lock history）: `docs/development/current/main/phases/phase-29cc/README.md`
  - wasm lane G2 task plan: `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md`
  - wasm `.hako`-only output roadmap SSOT: `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
  - `docs/development/current/main/phases/phase-29cc/README.md`
  - `docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md`（progress SSOT）
  - `docs/development/current/main/phases/phase-29cc/29cc-91-worker-parallel-playbook.md`
  - `docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md`（fixed order）
  - `docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md`（shape contract）
  - de-rust done judgement matrix SSOT:
    - `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`（X32/X33/X34/X35 replay）
- Perf lane（phase-21.5）:
  - monitor-only（劣化検知時のみ failure-driven で再起動）
  - selfhost/de-rust を優先し、日常ループでは最適化タスクを proactive に起動しない
- De-Rust lane map: `A=Compiler Meaning / B=Compiler Pipeline / C=Runtime Port`
  - SSOT: `docs/development/current/main/design/de-rust-lane-map-ssot.md`

## Read First Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/design/de-rust-master-task-map-ssot.md`
3. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
4. `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
5. `docs/development/current/main/design/de-rust-lane-map-ssot.md`
6. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
7. `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
8. `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
9. `docs/tools/README.md`
10. `docs/development/current/main/phases/phase-29cc/README.md`
11. `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md`
12. `docs/development/current/main/phases/phase-29cc/29cc-134-wsm-g2-min1-bridge-run-loop-lock-ssot.md`
13. `docs/development/current/main/phases/phase-29cc/29cc-135-wsm-g2-min2-headless-run-lock-ssot.md`
14. `docs/development/current/main/phases/phase-29cc/29cc-136-wsm-g2-min3-guide-alignment-lock-ssot.md`
15. `docs/development/current/main/phases/phase-29cc/29cc-137-wsm-g3-min1-gap-inventory-lock-ssot.md`
16. `docs/development/current/main/phases/phase-29cc/29cc-138-wsm-g3-min2-canvas-clear-lock-ssot.md`
17. `docs/development/current/main/phases/phase-29cc/29cc-139-wsm-g3-min3-canvas-strokerect-lock-ssot.md`
18. `docs/development/current/main/phases/phase-29cc/29cc-140-wsm-g3-min4-canvas-beginpath-lock-ssot.md`
19. `docs/development/current/main/phases/phase-29cc/29cc-141-wsm-g3-min5-canvas-arc-lock-ssot.md`
20. `docs/development/current/main/phases/phase-29cc/29cc-142-wsm-g3-min6-canvas-fill-lock-ssot.md`
21. `docs/development/current/main/phases/phase-29cc/29cc-143-wsm-g3-min7-canvas-stroke-lock-ssot.md`
22. `docs/development/current/main/phases/phase-29cc/29cc-144-wsm-g3-min8-canvas-setfillstyle-lock-ssot.md`
23. `docs/development/current/main/phases/phase-29cc/29cc-145-wsm-g3-min9-canvas-setstrokestyle-lock-ssot.md`
24. `docs/development/current/main/phases/phase-29cc/29cc-146-wsm-g3-min10-canvas-setlinewidth-lock-ssot.md`
25. `docs/development/current/main/phases/phase-29cc/29cc-147-wsm-g3-min11-fillcircle-drawline-gap-lock-ssot.md`
26. `docs/development/current/main/phases/phase-29cc/29cc-148-wsm-g3-min12-canvas-fillcircle-lock-ssot.md`
27. `docs/development/current/main/phases/phase-29cc/29cc-149-wsm-g3-min13-canvas-drawline-lock-ssot.md`
28. `docs/development/current/main/phases/phase-29cc/29cc-150-wsm-p1-min1-emit-wat-cli-lock-ssot.md`
29. `docs/development/current/main/phases/phase-29cc/29cc-151-wsm-p1-min2-wat-parity-lock-ssot.md`
30. `docs/development/current/main/phases/phase-29cc/29cc-152-wsm-p2-min1-wat2wasm-bridge-lock-ssot.md`
31. `docs/development/current/main/phases/phase-29cc/29cc-153-wsm-p3-min1-import-object-lock-ssot.md`
32. `docs/development/current/main/phases/phase-29cc/29cc-154-wsm-p4-min1-binary-writer-doc-lock-ssot.md`
33. `docs/development/current/main/phases/phase-29cc/29cc-155-wsm-p4-min2-binary-writer-skeleton-lock-ssot.md`
34. `docs/development/current/main/phases/phase-29cc/29cc-156-wsm-p4-min3-hako-writer-entry-parity-doc-lock-ssot.md`
35. `docs/development/current/main/phases/phase-29cc/29cc-157-wsm-p4-min4-hako-writer-const-parity-lock-ssot.md`
36. `docs/development/current/main/phases/phase-29cc/29cc-158-wsm-p4-min5-neg-const-parity-lock-ssot.md`
37. `docs/development/current/main/phases/phase-29cc/29cc-159-wsm-p4-min6-shape-table-lock-ssot.md`
38. `docs/development/current/main/phases/phase-29cc/29cc-160-wsm-p5-min1-default-cutover-doc-lock-ssot.md`
39. `docs/development/current/main/phases/phase-29cc/29cc-161-wsm-p5-min2-route-policy-lock-ssot.md`

## Daily Commands

- `cargo check --bin hakorune`
- `bash tools/selfhost/run_lane_a_daily.sh`
- `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`（診断補助。quick既定セットには含めない）

## Milestone Commands

- `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- `tools/checks/dev_gate.sh portability`（cross-platform preflight）
- `bash tools/checks/windows_wsl_cmd_smoke.sh --build --cmd-smoke`（WSL週次Windows smoke）

## Runtime Diagnostic Pins (non-gating)

- `bash tools/smokes/v2/profiles/integration/apps/phase29y_continue_assignment_in_continue_stale_guard_vm.sh`

## Compiler Diagnostic Pins (non-gating)

- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_var_values_lock_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/mir_shape_guard_vm.sh`

## Runtime Next (SSOT Pointer)

- Current blocker と next fixed order は `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` を正本とする。
- `CURRENT_TASK.md` とこの文書には要約のみを置き、Next の重複転記を禁止する。

## Key SSOT Pointers

- De-rust master task map: `docs/development/current/main/design/de-rust-master-task-map-ssot.md`
- De-rust lane map (A/B/C): `docs/development/current/main/design/de-rust-lane-map-ssot.md`
- De-rust scope decision (L5): `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
- De-rust done declaration (non-plugin): `docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md`
- De-rust plugin lane bootstrap: `docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md`
- De-rust plugin ABI lock (PLG-01): `docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`
- De-rust plugin gate pack lock (PLG-02): `docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md`
- De-rust plugin wave-1 pilot lock (PLG-03): `docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min1): `docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min2): `docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min3): `docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min4): `docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min5): `docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min6): `docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md`
- De-rust post-wave1 route lock: `docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md`
- De-rust plugin wave-2 entry lock (PLG-05-min1): `docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min2): `docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min3): `docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min4): `docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min5): `docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min6): `docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min7): `docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md`
- De-rust plugin wave-3 entry lock (PLG-06-min1): `docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md`
- De-rust plugin wave-3 rollout lock (PLG-06-min2): `docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md`
- De-rust plugin wave-3 rollout lock (PLG-06-min3): `docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md`
- De-rust plugin wave-3 rollout lock (PLG-06-min4): `docs/development/current/main/phases/phase-29cc/29cc-116-plg06-egui-wave3-min4-ssot.md`
- De-rust wasm lane lock (WSM-01): `docs/development/current/main/phases/phase-29cc/29cc-117-wsm01-wasm-unsupported-inventory-sync-ssot.md`
- De-rust wasm grammar/map lock: `docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md`
- De-rust wasm lane lock (WSM-02a): `docs/development/current/main/phases/phase-29cc/29cc-119-wsm02a-assignment-local-unblock-ssot.md`
- De-rust wasm demo-goal lock: `docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md`
- De-rust wasm lane lock (WSM-02b-min1): `docs/development/current/main/phases/phase-29cc/29cc-121-wsm02b-min1-console-warn-extern-ssot.md`
- De-rust wasm lane lock (WSM-02b-min2): `docs/development/current/main/phases/phase-29cc/29cc-122-wsm02b-min2-console-error-extern-ssot.md`
- De-rust wasm lane lock (WSM-02b-min3): `docs/development/current/main/phases/phase-29cc/29cc-123-wsm02b-min3-console-info-extern-ssot.md`
- De-rust wasm lane lock (WSM-02b-min4): `docs/development/current/main/phases/phase-29cc/29cc-124-wsm02b-min4-console-debug-extern-ssot.md`
- De-rust wasm lane lock (WSM-02c-min1): `docs/development/current/main/phases/phase-29cc/29cc-125-wsm02c-min1-boxcall-console-info-ssot.md`
- De-rust wasm lane lock (WSM-02c-min2): `docs/development/current/main/phases/phase-29cc/29cc-126-wsm02c-min2-boxcall-console-debug-ssot.md`
- De-rust wasm lane lock (WSM-02c-min3): `docs/development/current/main/phases/phase-29cc/29cc-127-wsm02c-min3-boxcall-console-warn-ssot.md`
- De-rust wasm lane lock (WSM-02c-min4): `docs/development/current/main/phases/phase-29cc/29cc-128-wsm02c-min4-boxcall-console-error-ssot.md`
- De-rust wasm lane lock (WSM-02d-min1): `docs/development/current/main/phases/phase-29cc/29cc-129-wsm02d-min1-boundary-fastfail-tests-ssot.md`
- De-rust wasm lane lock (WSM-02d-min2): `docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md`
- De-rust wasm lane lock (WSM-02d-min3): `docs/development/current/main/phases/phase-29cc/29cc-131-wsm02d-min3-demo-unsupported-boundary-lock-ssot.md`
- De-rust wasm lane lock (WSM-02d-min4): `docs/development/current/main/phases/phase-29cc/29cc-132-wsm02d-min4-milestone-gate-promotion-lock-ssot.md`
- De-rust done judgement matrix (X32-X35): `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`
- Compiler task order: `docs/development/current/main/design/compiler-task-map-ssot.md`
- Compiler pipeline: `docs/development/current/main/design/compiler-pipeline-ssot.md`
- De-rust compiler roadmap: `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md`
- JoinIR port task pack (lane A): `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
- Dev tools quick entry: `docs/tools/README.md`
- Runtime GC policy/order: `docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md`
- Selfhost migration order: `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
- Runtime lane README: `docs/development/current/main/phases/phase-29y/README.md`

## Historical / Deep Logs

- Phase 29x runtime history: `docs/development/current/main/phases/phase-29x/README.md`
- Runtime milestone archive: `docs/development/current/main/phases/phase-29x/29x-84-current-task-runtime-milestone-archive.md`
- Task board archive: `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`

## Maintenance Rule

- この文書に「完了ログの箇条書き」を追加しない。
- 進捗は phase 文書へ記録し、ここはリンクだけを更新する。
