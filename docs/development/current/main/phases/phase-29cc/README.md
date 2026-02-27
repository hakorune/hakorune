---
Status: Active (M0-M4 done; plugin waves done through PLG-06-min4; wasm lane done through WSM-P8-min1)
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
  - wasm lane active next: `none`（monitor-only）
  - wasm lane G2 task plan: `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md`
  - wasm latest locks（recent 3）:
    - `docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md`
  - wasm full lock history pointer:
    - `docs/development/current/main/10-Now.md`（Read First Order: phase-29cc lock list）
  - wasm `.hako`-only output roadmap SSOT:
    - `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
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
  - plugin lane active next: `PLG-07-min7`（retire execution lock, accepted-but-blocked）

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
