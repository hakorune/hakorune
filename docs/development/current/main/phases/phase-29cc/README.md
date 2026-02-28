---
Status: Active (M0-M4 done; plugin waves done through PLG-06-min4; wasm lane done through WSM-P10-min1)
Scope: Rust -> .hako migration orchestration lane (M0-M4)
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-runtime-meaning-decision-red-inventory-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-243-runtime-route-zero-sync-closeout-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md
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
---

# Phase 29cc: Rust -> .hako Migration Orchestration

## Goal

"一気に移植" を迷走させず、M0-M4 の固定順序で Rust 依存を段階縮退する。

この phase は実装レーンを増やすための「管理レーン」であり、既存 lane（29bq/29y）を置換しない。

## Master Pointer

- 脱Rustの全体順序（lane A/B/C + done判定 + scope decision）は
  `docs/development/current/main/design/de-rust-master-task-map-ssot.md` を正本とする。
- この文書は phase-29cc（orchestration lane）の実行詳細に限定する。

## Non-goals

- 1コミットで複数 lane を横断する大規模置換
- gate 未緑のままの PROMOTE
- Rust 側 workaround を silent fallback で積むこと

## M0-M4 (fixed order)

1. M0: boundary lock（責務境界/受け入れ gate 固定）
2. M1: parser parity（Rust/.hako 同形受理）
3. M2: mirbuilder parity（failure-driven PROMOTE）
4. M3: runtime bridge thinning（Rust は橋渡し最小のみ）
5. M4: residue cleanup（残存 Rust-only 導線の可視化と撤去計画）

## Current focus

### Plugin Progress Snapshot

| Wave | Status | Lock |
|---|---|---|
| wave-1 | done (`PLG-04-min6`) | `29cc-104` |
| wave-2 | done (`PLG-05-min7`) | `29cc-112` |
| wave-3 | done (`PLG-06-min1..min4`) | `29cc-116` |

- M4 done（monitor-only closeout completed）:
  - M1 parser parity と M2 mirbuilder parity は gate 固定済み
  - M3 runtime bridge thinning の主要 gate は緑（lane gate / no-compat mainline）
  - RDM-2-min1..min5 を完了（direct-v0 bridge route retired + parser-flag entrypoints removed）
  - M4 tail cleanup（docs/guard/code/historical sync）完了
  - 進捗チェックの正本は `29cc-90-migration-execution-checklist.md` に固定
- RNR queue active（non-plugin residue, docs-first）:
  - fixed order 正本は `29cc-92-non-plugin-rust-residue-task-set.md`
  - `RNR-01` 完了（`vm_hako` compile bridge seam split）
  - `RNR-02` 完了（`shape_contract` 実体化 + payload/subset 判定統合 + call(args=2) 契約pin）
  - `RNR-03` 完了（`selfhost` JSON payload ownership を `json.rs` resolver へ集約）
  - `RNR-04` 完了（orchestrator から Stage-A 意味判定を分離し、routing 専用へ縮退）
  - `RNR-05` 完了（parser+plan single shape pack。min1..min3 done）
  - current active next は `none`（monitor-only）
- L5 scope decision は accepted（non-plugin done、plugin は separate lane）:
  - `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
- L4 strict readiness は PASS（2026-02-25）:
  - `tools/selfhost/check_phase29x_x23_readiness.sh --strict` -> `status=READY`
- non-plugin de-rust done 宣言は `29cc-94` で固定（2026-02-25）:
  - `docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md`
- plugin separate lane の準備は `29cc-95` で開始（docs-first, provisional）:
  - `docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md`
- plugin lane `PLG-01` は done（ABI/loader acceptance lock）:
  - `docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`
- plugin lane `PLG-02` は done（gate pack lock）:
  - `docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md`
- plugin lane `PLG-03` は done（wave-1 CounterBox pilot）:
  - `docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md`
- plugin lane `PLG-04-min1` は done（wave-1 ArrayBox rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md`
- plugin lane `PLG-04-min2` は done（wave-1 IntCellBox reserved-core lock）:
  - `docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`
- plugin lane `PLG-04-min3` は done（wave-1 MapBox rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md`
- plugin lane `PLG-04-min4` は done（wave-1 StringBox rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md`
- plugin lane `PLG-04-min5` は done（wave-1 ConsoleBox rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md`
- plugin lane `PLG-04-min6` は done（wave-1 FileBox rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md`
  - active next: `none`（monitor-only）
- plugin lane `PLG-05-min1` は done（wave-2 Json entry lock）:
  - `docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md`
- plugin lane `PLG-05-min2` は done（wave-2 TOML rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md`
- plugin lane `PLG-05-min3` は done（wave-2 Regex rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md`
- plugin lane `PLG-05-min4` は done（wave-2 Encoding rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md`
- plugin lane `PLG-05-min5` は done（wave-2 Path rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md`
- plugin lane `PLG-05-min6` は done（wave-2 Math rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md`
- plugin lane `PLG-05-min7` は done（wave-2 Net rollout）:
  - `docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md`
- plugin lane `PLG-06-min1` は done（wave-3 entry lock, PythonCompiler）:
  - `docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md`
- plugin lane `PLG-06-min2` は done（wave-3 rollout, Python plugin）:
  - `docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md`
- plugin lane `PLG-06-min3` は done（wave-3 rollout, PythonParser plugin）:
  - `docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md`
- plugin lane `PLG-06-min4` は done（wave-3 rollout, Egui plugin）:
  - `docs/development/current/main/phases/phase-29cc/29cc-116-plg06-egui-wave3-min4-ssot.md`
  - active next: `none`（monitor-only）
- post-wave1 route lock（accepted）:
  - `docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md`
  - wasm lane lock（WSM-02b-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-121-wsm02b-min1-console-warn-extern-ssot.md`
  - wasm lane lock（WSM-02b-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-122-wsm02b-min2-console-error-extern-ssot.md`
  - wasm lane lock（WSM-02b-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-123-wsm02b-min3-console-info-extern-ssot.md`
  - wasm lane lock（WSM-02b-min4 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-124-wsm02b-min4-console-debug-extern-ssot.md`
  - wasm lane lock（WSM-02c-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-125-wsm02c-min1-boxcall-console-info-ssot.md`
  - wasm lane lock（WSM-02c-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-126-wsm02c-min2-boxcall-console-debug-ssot.md`
  - wasm lane lock（WSM-02c-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-127-wsm02c-min3-boxcall-console-warn-ssot.md`
  - wasm lane lock（WSM-02c-min4 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-128-wsm02c-min4-boxcall-console-error-ssot.md`
  - wasm lane lock（WSM-02d-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-129-wsm02d-min1-boundary-fastfail-tests-ssot.md`
  - wasm lane lock（WSM-02d-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md`
  - wasm lane lock（WSM-02d-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-131-wsm02d-min3-demo-unsupported-boundary-lock-ssot.md`
  - wasm lane lock（WSM-02d-min4 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-132-wsm02d-min4-milestone-gate-promotion-lock-ssot.md`
  - wasm lane P7 done locks:
    - `docs/development/current/main/phases/phase-29cc/29cc-184-wsm-p7-min1-hako-only-done-criteria-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md`
  - wasm lane P8 compat bridge retire lock（accepted-but-blocked done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md`
  - wasm lane P9 non-native shrink locks（accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md`
  - wasm lane P9 bridge blockers（accepted-but-blocked done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-192-wsm-p9-min3-canvas-advanced-bridge-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-193-wsm-p9-min4-bridge-retire-refresh-lock-ssot.md`
  - wasm lane P10 locks（min1 accepted-but-blocked, min2/min3/min4/min5/min6/min7/min8/min9/min10 accepted）:
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
  - playground runtime contract（new）:
    - static-first route: `selected example -> prebuilt wasm -> main()`
    - prebuilt assets: `projects/nyash-wasm/prebuilt/*.wasm`（`build.sh` で生成・更新）
  - wasm G4 WasmCanvasBox re-promotion locks（min9/min10 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-206-wsm-g4-min9-webcanvas-wasmbox-repromotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-207-wsm-g4-min10-canvas-advanced-wasmbox-repromotion-lock-ssot.md`
  - wasm freeze locks（min1/min2/min3 done）:
    - `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min1_route_policy_rust_native_env_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min2_route_trace_always_on_vm.sh`
    - `docs/development/current/main/phases/phase-29cc/29cc-208-wsm-freeze-min3-route-policy-scope-lock-ssot.md`
  - wasm lane G2 task plan: `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md`
  - wasm latest locks（recent 3）:
    - `docs/development/current/main/phases/phase-29cc/29cc-206-wsm-g4-min9-webcanvas-wasmbox-repromotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-207-wsm-g4-min10-canvas-advanced-wasmbox-repromotion-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-208-wsm-freeze-min3-route-policy-scope-lock-ssot.md`
  - wasm full lock history pointer:
    - `docs/development/current/main/10-Now.md`（Read First Order: phase-29cc lock list）
  - wasm `.hako`-only output roadmap SSOT:
    - `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
    - Rust WASM lifecycle（fixed）: `Stop -> Freeze -> Retire`
  - wasm route governance（fixed 3 routes）:
    - `hako_native`（default）/ `rust_native`（parity）/ `legacy_bridge`（monitor-only）
    - 詳細: `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md` の `Route Governance (fixed)`
  - wasm freeze gate profiles（new）:
    - `tools/checks/dev_gate.sh wasm-freeze-core`
    - `tools/checks/dev_gate.sh wasm-freeze-parity`
  - plugin de-rust cutover order SSOT（new）:
    - `docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md`
  - plugin de-rust filebox binary lock（PLG-07-min1/min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-179-plg07-min1-min2-filebox-binary-rust-parity-lock-ssot.md`
  - plugin de-rust filebox `.hako` parity lock（PLG-07-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-180-plg07-min3-filebox-binary-hako-parity-lock-ssot.md`
  - plugin de-rust filebox dual-run gate lock（PLG-07-min4 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-181-plg07-min4-filebox-binary-dualrun-gate-lock-ssot.md`
  - plugin de-rust filebox default switch lock（PLG-07-min5 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-182-plg07-min5-filebox-default-switch-lock-ssot.md`
  - plugin de-rust filebox retire readiness lock（PLG-07-min6 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md`
  - plugin de-rust filebox retire execution lock（PLG-07-min7 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-204-plg07-min7-filebox-retire-execution-lock-ssot.md`
  - plugin module-provider lock（PLG-HM1 min1..min5 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-209-plg-hm1-core8-module-provider-lock-ssot.md`
    - lock運用: `plugin-module-core8-light` は `phase29cc_plg_hm1_contract_tests_vm.sh`（min1..min4 集約）を正本にする
    - dev gate:
      - `tools/checks/dev_gate.sh plugin-module-core8-light`
      - `tools/checks/dev_gate.sh plugin-module-core8`
  - plugin HM2 recovery line lock（PLG-HM2-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-210-plg-hm2-core-wave2-rust-recovery-line-lock-ssot.md`
  - plugin HM2 route matrix lock（PLG-HM2-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-211-plg-hm2-min2-core6-static-wave2-compat-ceiling-lock-ssot.md`
  - plugin HM2 route policy matrix lock（PLG-HM2-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-212-plg-hm2-min3-route-policy-matrix-lock-ssot.md`
  - plugin lane active next: `none`（HM2 complete; monitor-only, failure-driven reopen）
  - HM2 closeout evidence（2026-02-28）:
    - `bash tools/checks/phase29cc_plg_hm2_rust_recovery_line_guard.sh`
    - `bash tools/checks/phase29cc_plg_hm2_min2_core6_wave2_ceiling_guard.sh`
    - `bash tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh`
    - `tools/checks/dev_gate.sh plugin-module-core8`
  - plugin residue classification lock（29cc-213, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-213-plg-hm3-next-blocker-candidate-memo.md`
    - fixed:
      - `nyash-fixture-plugin` = test-only keep
      - `nyash-integer-plugin` = mainline keep（IntCellBox）
      - `nyash-math` = retire（legacy duplicate; `nyash-math-plugin` is active line）
  - runtime source-zero cutover lock（29cc-220, active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md`
    - zero definition（fixed）:
      - long-term goal = source-zero（runtime/plugin の Rust実装撤去 + mainline/CI no-compat）
      - phase done = route-zero + stability（no-delete-first）
      - execution-path-zero は中間マイルストーンとして扱う
    - latest cleanup（2026-02-28）:
      - `enabled/route_resolver.rs` を追加し、`ffi_bridge`/`instance_manager` の type/birth/method route 解析を単一責務へ集約
      - `enabled/types.rs` の `PluginHandleInner` helper で drop/finalize/clone の host_bridge route 呼び出しを集約
      - `enabled/loader/singletons.rs` の type_id/fini route 判定を `route_resolver` に統一
      - `enabled/loader/metadata.rs` の type_id/fini route 判定重複を `route_resolver` に統一
      - `enabled/loader/metadata.rs` の type逆引き（type_id->lib/box）も `route_resolver` に統一
      - `enabled/method_resolver.rs` の method_id/returns_result/handle 解決を `route_resolver` へ統一し、method route SSOT を固定
      - `enabled/compat_method_resolver.rs` へ legacy file fallback を隔離し、compat呼び出し点を `method_resolver` 1箇所へ固定
      - `enabled/compat_host_bridge.rs` へ shim invoke fallback を隔離し、`ffi_bridge` invoke non-zero code を fail-fast 化
  - runtime route-zero-sync closeout lock（29cc-243, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-243-runtime-route-zero-sync-closeout-lock-ssot.md`
    - fixed:
      - route-zero + stability 判定同期は closeout
      - runtime lane は monitor-only（failure-driven reopen）
      - next handoff は selfhost `.hako` migration（29bq: mirbuilder first / parser later）
  - runtime execution-path observability lock（29cc-215, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md`
    - guard:
      - `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
  - runtime VM+AOT route lock（29cc-217, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-217-runtime-vm-aot-route-lock-ssot.md`
    - guard:
      - `bash tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh`
      - `tools/checks/dev_gate.sh runtime-exec-zero`
  - runtime V0 ABI slice lock（29cc-216, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md`
    - guard:
      - `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
  - plugin method resolver fail-fast lock（29cc-218, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-218-plugin-method-resolver-failfast-lock-ssot.md`
  - instance manager boundary lock（29cc-219, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-219-instance-manager-boundary-lock-ssot.md`

## M4 Tail Cleanup (retired parser flags)

対象（M4 tail cleanup）:
- CLI flag: `--parser ny`（mainline 入口から削除）
- ENV flag: `NYASH_USE_NY_PARSER=1`（legacy no-op 化）

方針:
1. M4中は parser flag 入口を削除し、silent fallback は入れない。
2. まず docs/guard を固定して「使えない入口」を明示する。
3. 削除は lane B/C gate 緑を前提に、1 commit = 1 boundary で進める。

撤去前提（全部満たす）:
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh` 緑
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh` 緑
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh` 緑
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh` 緑

撤去順序（fixed）:
1. docs: `--parser ny` / `NYASH_USE_NY_PARSER` の削除/no-op 状態を明記（入口文書同期）
2. guard: retired route guard が削除対象を監視できることを固定
3. code: dispatch 側の parser flag 受理入口を削除（done）
4. cleanup: env catalog / cli docs / historical note を同期（done）

## Worker parallel policy

1. Explorer-A: residual inventory（Rust-only 経路の棚卸し）
2. Worker-B: parser parity 実装（1受理形ずつ）
3. Worker-C: gate/fixture pin（PROMOTE 専用、コード変更禁止）
4. Parent: 最終統合（fast gate green を受理条件）

禁止:
- 同一ファイル同時編集
- BoxCount と BoxShape の同シリーズ混在

## Acceptance gates (phase-level)

- `cargo check --bin hakorune`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`
- runtime 変更を含む場合のみ:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`

## Entry points

- checklist: `docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md`
- worker playbook: `docs/development/current/main/phases/phase-29cc/29cc-91-worker-parallel-playbook.md`
- residue task-set: `docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md`
- inventory memo: `docs/development/current/main/design/de-rust-runtime-meaning-decision-red-inventory-ssot.md`
- RNR-05 shape contract: `docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md`
- non-plugin done sync: `docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md`
- plugin lane bootstrap: `docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md`
- plugin ABI lock (PLG-01): `docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`
- plugin gate pack lock (PLG-02): `docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md`
- plugin wave-1 pilot lock (PLG-03): `docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md`
- plugin wave rollout lock (PLG-04-min1): `docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md`
- plugin wave rollout lock (PLG-04-min2): `docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`
- plugin wave rollout lock (PLG-04-min3): `docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md`
- plugin wave rollout lock (PLG-04-min4): `docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md`
- plugin wave rollout lock (PLG-04-min5): `docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md`
- plugin wave rollout lock (PLG-04-min6): `docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md`
- post-wave1 route lock: `docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md`
- plugin wave-2 entry lock (PLG-05-min1): `docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md`
- plugin wave-2 rollout lock (PLG-05-min2): `docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md`
- plugin wave-2 rollout lock (PLG-05-min3): `docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md`
- plugin wave-2 rollout lock (PLG-05-min4): `docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md`
- plugin wave-2 rollout lock (PLG-05-min5): `docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md`
- plugin wave-2 rollout lock (PLG-05-min6): `docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md`
- plugin wave-2 rollout lock (PLG-05-min7): `docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md`
- plugin wave-3 entry lock (PLG-06-min1): `docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md`
- plugin wave-3 rollout lock (PLG-06-min2): `docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md`
- plugin wave-3 rollout lock (PLG-06-min3): `docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md`
- plugin wave-3 rollout lock (PLG-06-min4): `docs/development/current/main/phases/phase-29cc/29cc-116-plg06-egui-wave3-min4-ssot.md`
- wasm lane lock (WSM-01): `docs/development/current/main/phases/phase-29cc/29cc-117-wsm01-wasm-unsupported-inventory-sync-ssot.md`
- wasm grammar/map lock: `docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md`
- wasm lane lock (WSM-02a): `docs/development/current/main/phases/phase-29cc/29cc-119-wsm02a-assignment-local-unblock-ssot.md`
- wasm demo-goal lock: `docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md`
- wasm lane lock (WSM-02b-min1): `docs/development/current/main/phases/phase-29cc/29cc-121-wsm02b-min1-console-warn-extern-ssot.md`
- wasm lane lock (WSM-02b-min2): `docs/development/current/main/phases/phase-29cc/29cc-122-wsm02b-min2-console-error-extern-ssot.md`
- wasm lane lock (WSM-02b-min3): `docs/development/current/main/phases/phase-29cc/29cc-123-wsm02b-min3-console-info-extern-ssot.md`
- wasm lane lock (WSM-02b-min4): `docs/development/current/main/phases/phase-29cc/29cc-124-wsm02b-min4-console-debug-extern-ssot.md`
- wasm lane lock (WSM-02c-min1): `docs/development/current/main/phases/phase-29cc/29cc-125-wsm02c-min1-boxcall-console-info-ssot.md`
- wasm lane lock (WSM-02c-min2): `docs/development/current/main/phases/phase-29cc/29cc-126-wsm02c-min2-boxcall-console-debug-ssot.md`
- wasm lane lock (WSM-02c-min3): `docs/development/current/main/phases/phase-29cc/29cc-127-wsm02c-min3-boxcall-console-warn-ssot.md`
- wasm lane lock (WSM-02c-min4): `docs/development/current/main/phases/phase-29cc/29cc-128-wsm02c-min4-boxcall-console-error-ssot.md`
- wasm lane lock (WSM-02d-min1): `docs/development/current/main/phases/phase-29cc/29cc-129-wsm02d-min1-boundary-fastfail-tests-ssot.md`
- wasm lane lock (WSM-02d-min2): `docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md`
- wasm lane lock (WSM-02d-min3): `docs/development/current/main/phases/phase-29cc/29cc-131-wsm02d-min3-demo-unsupported-boundary-lock-ssot.md`
- wasm lane lock (WSM-02d-min4): `docs/development/current/main/phases/phase-29cc/29cc-132-wsm02d-min4-milestone-gate-promotion-lock-ssot.md`
  - runtime/plugin residue inventory lock（29cc-221, active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md`
    - fixed:
      - plugin_loader_v2 residue と kernel plugin residue を責務単位で棚卸し
      - retire order を `1 boundary = 1 commit` で固定
  - final-wave non-target discovery lock（29cc-244, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-244-final-wave-nontarget-discovery-lock-ssot.md`
    - fixed:
      - Non-target 7 files の entry/caller/ABI/complexity を統合棚卸し
      - 7 commit slices（handle_helpers -> module_string_dispatch -> array -> map -> string -> intarray -> console）を固定
      - execution done: `e8e9e2d79` / `ea54764df` / `a53c9a53d` / `5a575c503` / `ecd44c43d` / `ca0d82dd0` / `5f191ff25`
  - runtime A1-min1 method_resolver route cutover lock（29cc-222, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-222-runtime-a1-min1-method-resolver-route-cutover-lock-ssot.md`
  - runtime A1-min2 instance_manager route cutover lock（29cc-223, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-223-runtime-a1-min2-instance-manager-route-cutover-lock-ssot.md`
  - runtime A2-min1 ffi_bridge route hardening lock（29cc-224, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-224-runtime-a2-min1-ffi-bridge-route-hardening-lock-ssot.md`
  - runtime A2-min2 host_bridge route cutover lock（29cc-225, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-225-runtime-a2-min2-host-bridge-route-cutover-lock-ssot.md`
  - runtime A3-min1 loader metadata route hardening lock（29cc-226, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-226-runtime-a3-min1-loader-metadata-route-hardening-lock-ssot.md`
  - runtime A3-min2 types handle route cutover lock（29cc-227, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-227-runtime-a3-min2-types-handle-route-cutover-lock-ssot.md`
  - runtime A3-min3 globals/errors/extern fail-fast lock（29cc-228, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-228-runtime-a3-min3-globals-errors-extern-failfast-lock-ssot.md`
  - runtime A3-min4 PluginBoxMetadata route-aware lock（29cc-229, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-229-runtime-a3-min4-pluginboxmetadata-routeaware-lock-ssot.md`
  - runtime A3 closeout lock（29cc-230, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-230-runtime-a3-closeout-lock-ssot.md`
  - kernel B1-min1 invoke/birth route cutover lock（29cc-231, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-231-kernel-b1-min1-invoke-birth-route-cutover-lock-ssot.md`
  - kernel B1-min1 closeout lock（29cc-232, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-232-kernel-b1-min1-closeout-lock-ssot.md`
  - kernel B1-min2 runtime state route lock（29cc-233, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-233-kernel-b1-min2-runtime-state-route-lock-ssot.md`
  - kernel B1-min3 instance lifecycle route lock（29cc-234, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-234-kernel-b1-min3-instance-lifecycle-route-lock-ssot.md`
  - kernel B1 closeout lock（29cc-235, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-235-kernel-b1-closeout-lock-ssot.md`
  - kernel B2-min1 value_codec encode/decode route lock（29cc-236, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-236-kernel-b2-min1-value-codec-encode-decode-route-lock-ssot.md`
  - kernel B2-min2 borrowed_handle route lock（29cc-237, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-237-kernel-b2-min2-borrowed-handle-route-lock-ssot.md`
  - kernel B2 closeout lock（29cc-238, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-238-kernel-b2-closeout-lock-ssot.md`
  - kernel B3-min1 future route lock（29cc-239, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-239-kernel-b3-min1-future-route-lock-ssot.md`
  - kernel B3-min2 invoke route lock（29cc-240, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-240-kernel-b3-min2-invoke-route-lock-ssot.md`
  - kernel B3 closeout lock（29cc-241, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-241-kernel-b3-closeout-lock-ssot.md`
  - kernel residue closeout lock（29cc-242, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-242-kernel-residue-closeout-lock-ssot.md`
