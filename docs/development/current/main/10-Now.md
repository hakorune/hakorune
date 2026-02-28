---
Status: SSOT
Date: 2026-02-28
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
- 研究/将来案（Python系を含む）は `Current blocker` に混ぜず、`30-Backlog.md` を正本にして管理する。

## Quick Restart Pointer

- 再起動直後の最短導線は `docs/development/current/main/05-Restart-Quick-Resume.md` を正本とする。
- 実行順と blocker の参照先を 1 画面で辿れる状態を保つ。
- 最短コマンド（kilo VM+AOT route lock 同期済み）:

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 \
PERF_GATE_AOT_SKIP_BUILD_CHECK=1 \
PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 \
PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 \
PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK=1 \
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh
```

## Current Snapshot

- Compiler lane: `phase-29bq`（JIR-PORT-00..07 done / active blocker=`none` / next=`none`）
- Lane A status mirror SSOT: `CURRENT_TASK.md`（この文書は mirror）
- Lane A mirror sync helper: `bash tools/selfhost/sync_lane_a_state.sh`（`CURRENT_TASK.md` を唯一入力に同期）
- Runtime lane: `phase-29y`（Current blocker / Next fixed order は `phase-29y/60-NEXT-TASK-PLAN.md` を正本とする）
- Runtime operation policy: `LLVM-first / vm-hako monitor-only`（日常の runtime 検証は LLVM 主経路、vm-hako は blocker 検知の monitor lane）
- Optimization policy (runtime): de-rust 経路固定（29cc-220/215/216/217 guard 緑）まで perf/AOT 最適化は monitor-only。
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
  - plugin de-rust PLG-07 locks（accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-179-plg07-min1-min2-filebox-binary-rust-parity-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-180-plg07-min3-filebox-binary-hako-parity-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-181-plg07-min4-filebox-binary-dualrun-gate-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-182-plg07-min5-filebox-default-switch-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-204-plg07-min7-filebox-retire-execution-lock-ssot.md`
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
  - plugin lane active next: `none`（PLG-07 closeout complete; monitor-only）
  - runtime source-zero cutover lock（29cc-220 active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md`
    - zero definition: long-term=source-zero / phase done=route-zero + stability（no-delete-first）
  - fullstack completion SSOT（meaning in `.hako`, host as minimal ABI）:
    - `docs/development/current/main/design/hako-fullstack-host-abi-completion-ssot.md`
  - runtime route-zero-sync closeout lock（29cc-243 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-243-runtime-route-zero-sync-closeout-lock-ssot.md`
    - fixed:
      - route-zero + stability 判定同期は完了
      - de-rust runtime lane は monitor-only（failure-driven reopen）
      - next handoff は selfhost `.hako` migration（29bq, mirbuilder first）
  - runtime execution-path observability lock（29cc-215 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md`
    - guard: `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
  - runtime VM+AOT route lock（29cc-217 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-217-runtime-vm-aot-route-lock-ssot.md`
    - guard: `bash tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh`
  - runtime V0 ABI slice lock（29cc-216 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md`
    - guard: `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
  - runtime/plugin residue inventory lock（29cc-221 active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md`
  - runtime A1-min1 method_resolver route cutover lock（29cc-222 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-222-runtime-a1-min1-method-resolver-route-cutover-lock-ssot.md`
  - runtime A1-min2 instance_manager route cutover lock（29cc-223 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-223-runtime-a1-min2-instance-manager-route-cutover-lock-ssot.md`
  - runtime A2-min1 ffi_bridge route hardening lock（29cc-224 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-224-runtime-a2-min1-ffi-bridge-route-hardening-lock-ssot.md`
  - runtime A2-min2 host_bridge route cutover lock（29cc-225 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-225-runtime-a2-min2-host-bridge-route-cutover-lock-ssot.md`
  - runtime A3-min1 loader metadata route hardening lock（29cc-226 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-226-runtime-a3-min1-loader-metadata-route-hardening-lock-ssot.md`
  - runtime A3-min2 types handle route cutover lock（29cc-227 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-227-runtime-a3-min2-types-handle-route-cutover-lock-ssot.md`
  - runtime A3-min3 globals/errors/extern fail-fast lock（29cc-228 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-228-runtime-a3-min3-globals-errors-extern-failfast-lock-ssot.md`
  - runtime A3-min4 PluginBoxMetadata route-aware lock（29cc-229 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-229-runtime-a3-min4-pluginboxmetadata-routeaware-lock-ssot.md`
  - runtime A3 closeout lock（29cc-230 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-230-runtime-a3-closeout-lock-ssot.md`
  - kernel B1-min1 invoke/birth route cutover lock（29cc-231 active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-231-kernel-b1-min1-invoke-birth-route-cutover-lock-ssot.md`
  - kernel B1-min1 closeout lock（29cc-232 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-232-kernel-b1-min1-closeout-lock-ssot.md`
  - kernel B1-min2 runtime state route lock（29cc-233 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-233-kernel-b1-min2-runtime-state-route-lock-ssot.md`
  - kernel B1-min3 instance lifecycle route lock（29cc-234 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-234-kernel-b1-min3-instance-lifecycle-route-lock-ssot.md`
  - kernel B1 closeout lock（29cc-235 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-235-kernel-b1-closeout-lock-ssot.md`
  - kernel B2-min1 value_codec encode/decode route lock（29cc-236 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-236-kernel-b2-min1-value-codec-encode-decode-route-lock-ssot.md`
  - kernel B2-min2 borrowed_handle route lock（29cc-237 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-237-kernel-b2-min2-borrowed-handle-route-lock-ssot.md`
  - kernel B2 closeout lock（29cc-238 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-238-kernel-b2-closeout-lock-ssot.md`
  - kernel B3-min1 future route lock（29cc-239 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-239-kernel-b3-min1-future-route-lock-ssot.md`
  - kernel B3-min2 invoke route lock（29cc-240 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-240-kernel-b3-min2-invoke-route-lock-ssot.md`
  - kernel B3 closeout lock（29cc-241 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-241-kernel-b3-closeout-lock-ssot.md`
  - kernel residue closeout lock（29cc-242 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-242-kernel-residue-closeout-lock-ssot.md`
  - plugin method resolver fail-fast lock（29cc-218 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-218-plugin-method-resolver-failfast-lock-ssot.md`
  - instance manager boundary lock（29cc-219 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-219-instance-manager-boundary-lock-ssot.md`
  - wasm lane status SSOT（active next / latest lock / lock history）: `docs/development/current/main/phases/phase-29cc/README.md`
  - wasm lane G2 task plan: `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md`
  - wasm `.hako`-only output roadmap SSOT: `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
  - wasm route governance（fixed）: `hako_native`（default）/ `rust_native`（parity）/ `legacy_bridge`（monitor-only）
    - 詳細SSOT: `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md` の `Route Governance (fixed)`
  - wasm P7 hako-only locks（accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-184-wsm-p7-min1-hako-only-done-criteria-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md`
  - wasm P8 compat bridge retire lock（accepted-but-blocked done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md`
  - wasm P9 non-native shrink locks（accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md`
  - wasm P9 bridge blockers（accepted-but-blocked done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-192-wsm-p9-min3-canvas-advanced-bridge-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-193-wsm-p9-min4-bridge-retire-refresh-lock-ssot.md`
  - wasm P10 locks（min1 accepted-but-blocked, min2/min3/min4/min5/min6/min7/min8/min9/min10 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-194-wsm-p10-min1-loop-extern-native-emit-design-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-195-wsm-p10-min2-loop-extern-matcher-inventory-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-196-wsm-p10-min3-loop-extern-writer-section-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-197-wsm-p10-min4-single-fixture-native-promotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-198-wsm-p10-min5-expansion-inventory-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-199-wsm-p10-min6-warn-native-promotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-200-wsm-p10-min7-info-native-promotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-201-wsm-p10-min8-error-native-promotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-202-wsm-p10-min9-debug-native-promotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-203-wsm-p10-min10-native-promotion-closeout-lock-ssot.md`
  - wasm lane active next: `none`（P10 closeout complete; monitor-only）
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
40. `docs/development/current/main/phases/phase-29cc/29cc-162-wsm-p5-min3-default-hako-lane-lock-ssot.md`
41. `docs/development/current/main/phases/phase-29cc/29cc-163-wsm-p5-min4-hako-lane-bridge-shrink-lock-ssot.md`
42. `docs/development/current/main/phases/phase-29cc/29cc-164-wsm-p5-min5-native-helper-lock-ssot.md`
43. `docs/development/current/main/phases/phase-29cc/29cc-165-wsm-p5-min6-shape-expand-lock-ssot.md`
44. `docs/development/current/main/phases/phase-29cc/29cc-166-wsm-p5-min7-shape-trace-lock-ssot.md`
45. `docs/development/current/main/phases/phase-29cc/29cc-167-wsm-p5-min8-legacy-retire-readiness-lock-ssot.md`
46. `docs/development/current/main/phases/phase-29cc/29cc-168-wsm-p5-min9-legacy-retire-execution-lock-ssot.md`
47. `docs/development/current/main/phases/phase-29cc/29cc-169-wsm-p5-min10-legacy-hard-remove-lock-ssot.md`
48. `docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md`
49. `docs/development/current/main/phases/phase-29cc/29cc-171-wsm-g4-min1-nyash-playground-console-baseline-lock-ssot.md`
50. `docs/development/current/main/phases/phase-29cc/29cc-172-wsm-g4-min2-nyash-playground-canvas-primer-lock-ssot.md`
51. `docs/development/current/main/phases/phase-29cc/29cc-173-wsm-g4-min3-webcanvas-fixture-parity-lock-ssot.md`
52. `docs/development/current/main/phases/phase-29cc/29cc-174-wsm-g4-min4-canvas-advanced-fixture-parity-lock-ssot.md`
53. `docs/development/current/main/phases/phase-29cc/29cc-175-wsm-g4-min5-headless-two-example-parity-lock-ssot.md`
54. `docs/development/current/main/phases/phase-29cc/29cc-176-wsm-g4-min6-gate-promotion-closeout-lock-ssot.md`
55. `docs/development/current/main/phases/phase-29cc/29cc-177-wsm-p4-min7-buffer-file-binary-contract-lock-ssot.md`
56. `docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md`
57. `docs/development/current/main/phases/phase-29cc/29cc-179-plg07-min1-min2-filebox-binary-rust-parity-lock-ssot.md`
58. `docs/development/current/main/phases/phase-29cc/29cc-180-plg07-min3-filebox-binary-hako-parity-lock-ssot.md`
59. `docs/development/current/main/phases/phase-29cc/29cc-181-plg07-min4-filebox-binary-dualrun-gate-lock-ssot.md`
60. `docs/development/current/main/phases/phase-29cc/29cc-182-plg07-min5-filebox-default-switch-lock-ssot.md`
61. `docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md`
62. `docs/development/current/main/phases/phase-29cc/29cc-184-wsm-p7-min1-hako-only-done-criteria-lock-ssot.md`
63. `docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md`
64. `docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md`
65. `docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md`
66. `docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md`
67. `docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md`
68. `docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md`
69. `docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md`
70. `docs/development/current/main/phases/phase-29cc/29cc-192-wsm-p9-min3-canvas-advanced-bridge-lock-ssot.md`
71. `docs/development/current/main/phases/phase-29cc/29cc-193-wsm-p9-min4-bridge-retire-refresh-lock-ssot.md`
72. `docs/development/current/main/phases/phase-29cc/29cc-194-wsm-p10-min1-loop-extern-native-emit-design-lock-ssot.md`

## Daily Commands

- `cargo check --bin hakorune`
- `bash tools/selfhost/run_lane_a_daily.sh`
- `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
- `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
- `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`（診断補助。quick既定セットには含めない）

## Milestone Commands

- `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- `tools/checks/dev_gate.sh portability`（cross-platform preflight）
- `tools/checks/dev_gate.sh runtime-exec-zero`（execution-path-zero observability。source-zero までの中間ゲート）
- `bash tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh`（PLG-07 retire execution）
- `bash tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh`（WSM-P7 default hako-only）
- `bash tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh`（WSM-P8 bridge retire readiness）
- `bash tools/checks/phase29cc_wsm_p9_non_native_inventory_guard.sh`（WSM-P9 non-native shrink）
- `bash tools/checks/phase29cc_wsm_p9_bridge_retire_refresh_guard.sh`（WSM-P9 bridge retire refresh）
- `bash tools/checks/phase29cc_wsm_p10_loop_extern_native_emit_design_guard.sh`（WSM-P10 loop/extern native emit design lock）
- `bash tools/checks/phase29cc_wsm_p10_loop_extern_matcher_inventory_guard.sh`（WSM-P10 loop/extern matcher inventory lock）
- `bash tools/checks/phase29cc_wsm_p10_loop_extern_writer_section_guard.sh`（WSM-P10 loop/extern writer section lock）
- `bash tools/checks/phase29cc_wsm_p10_single_fixture_native_promotion_guard.sh`（WSM-P10 single fixture native promotion lock）
- `bash tools/checks/phase29cc_wsm_p10_expansion_inventory_guard.sh`（WSM-P10 expansion inventory lock）
- `bash tools/checks/phase29cc_wsm_p10_warn_native_promotion_guard.sh`（WSM-P10 warn native promotion lock）
- `bash tools/checks/phase29cc_wsm_p10_info_native_promotion_guard.sh`（WSM-P10 info native promotion lock）
- `bash tools/checks/phase29cc_wsm_p10_error_native_promotion_guard.sh`（WSM-P10 error native promotion lock）
- `bash tools/checks/phase29cc_wsm_p10_debug_native_promotion_guard.sh`（WSM-P10 debug native promotion lock）
- `bash tools/checks/phase29cc_wsm_p10_native_promotion_closeout_guard.sh`（WSM-P10 native promotion closeout lock）
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
