# design/

`docs/development/current/main/design/` は、長期参照する設計図（SSOT 寄り）を置く場所。

- 原則: “Phaseの作業ログ/完了報告” は `../phases/` に置く。
- 原則: “不具合調査ログ” は `../investigations/` に置く。

## Read First Now

- current restart anchor: `CURRENT_TASK.md`
- one-screen work order: `docs/development/current/main/15-Workstream-Map.md`
- canonical rough task order: `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
- current axis / artifact / task placement: `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
- current `K2-wide` technical detail:
  - `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
  - `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`

## Current Kernel Replacement Docs

- `kernel-implementation-phase-plan-ssot.md`
- `kernel-replacement-axis-ssot.md`
- `execution-lanes-and-axis-separation-ssot.md`
- `stage2-selfhost-and-hako-alloc-ssot.md`
- `stage2-hako-owner-vs-inc-thin-shim-ssot.md`
- `final-metal-split-ssot.md`

## Full Design Inventory

## 現役の設計図（入口）

- JoinIR の地図（navigation SSOT）: `docs/development/current/main/design/joinir-design-map.md`
- route physical path lane / historical token ledger: `docs/development/current/main/design/route-physical-path-legacy-lane-ssot.md`
- loop route detection physical-path retirement: `docs/development/current/main/design/loop-route-detection-physical-path-retirement-ssot.md`
- JoinIR 拡張の固定順序（dual-route 契約 SSOT）: `docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`
- Join-Explicit CFG Construction（north star）: `docs/development/current/main/design/join-explicit-cfg-construction.md`
- Compiler Pipeline（north star; 箱の責務/入口の最終形SSOT）: `docs/development/current/main/design/compiler-pipeline-ssot.md`
- EdgeCFG Flow Fragments（Structured→CFG lowering SSOT）: `docs/development/current/main/design/edgecfg-fragments.md`
- Catch / Cleanup / Async（設計メモ）: `docs/development/current/main/design/exception-cleanup-async.md`
- Loop Canonicalizer（設計 SSOT）: `docs/development/current/main/design/loop-canonicalizer.md`
- ControlTree / StepTree（構造SSOT）: `docs/development/current/main/design/control-tree.md`
- Normalized ExprLowerer（式の一般化 SSOT）: `docs/development/current/main/design/normalized-expr-lowering.md`
- CorePlan Skeleton/Feature（箱増殖を止めるSSOT）: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`
- RecipeTree + Parts（Recipe-first / Verified-only boundary SSOT）: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- Recipe-first entry contract（runtime は Facts→Recipe→Verifier→Lower、historical planner-payload wording は note-only）: `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- Condition observation（no rewrite SSOT）: `docs/development/current/main/design/condition-observation-ssot.md`
- generic_loop_v1 acceptance by Recipe（ShapeId hint-only SSOT）: `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md`
- Policy: compiler expressivity first（selfhost workaround を止める）: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- Policy: lego composability first（完成品キット増殖を止める）: `docs/development/current/main/design/lego-composability-policy.md`
- Selfhost parser/mirbuilder migration order（single-developer 進行順SSOT）: `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
- De-Rust compiler thin-rust roadmap（selfhost closeout 後の境界縮退SSOT）: `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md`
- De-Rust post-G1 runtime plan（runtime lane の実行順序/保守契約SSOT）: `docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md`
- Hako runtime C ABI cutover order（`.hako` 完結へ向けた移行順SSOT）: `docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md`
- Kernel replacement axis（`K0 / K1 / K2` build/runtime stages / `K2-core` and `K2-wide` task packs inside `K2` / current active order is `stage / docs / naming` -> `K1 done-enough` -> `K2-core` -> `K2-wide deferred` -> `zero-rust default`）: `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
- Stage2 `.hako` owner vs `.inc` thin shim（stage2 主体化 / thin boundary SSOT）: `docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md`
- Hako fullstack host-ABI completion（意味論`.hako`集約 + host最小ABIの最終形SSOT）: `docs/development/current/main/design/hako-fullstack-host-abi-completion-ssot.md`
- Execution lanes and axis separation（stage/owner/artifact-lane の親SSOT）: `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- Artifact policy（`llvm-exe` mainline / current `vm-hako` reference / future interpreter reservation の child SSOT）: `docs/development/current/main/design/artifact-policy-ssot.md`
- Execution lanes migration task pack（cross-phase 実装順SSOT）: `docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md`
- Execution lanes legacy retirement inventory（移行中に見つかった legacy/delete 候補の全体台帳）: `docs/development/current/main/design/execution-lanes-legacy-retirement-inventory-ssot.md`
- Hakoruneup release distribution（self-contained bundle + package manager + explicit system LLVM dev mode）: `docs/development/current/main/design/hakoruneup-release-distribution-ssot.md`
- Stage2 selfhost and hako-alloc（stage軸 + `hako_core/alloc/std` layering SSOT）: `docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md`
- Stage2 AOT/native thin path（current native perf/mainline の thin-owner design note）: `docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md`
- Value repr and ABI manifest（current value classes / ownership / manifest row truth）: `docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md`
- Stage2 Fast Leaf Manifest（backend-private fast lane row contract）: `docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md`
- Stage2 AOT-Core proof vocabulary（future AOT-Core MIR staged lock / current proof vocabulary SSOT）: `docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md`
- Stage2 optimization debug bundle（route/window/IR/symbol/perf を same artifact で束ねる SSOT）: `docs/development/current/main/design/stage2-optimization-debug-bundle-ssot.md`
- Thread and TLS capability（helper-shaped current TLS + language-level final TLS SSOT）: `docs/development/current/main/design/thread-and-tls-capability-ssot.md`
- Rust kernel export surface strata（compat / runtime-facade / substrate split SSOT）: `docs/development/current/main/design/rust-kernel-export-surface-strata-ssot.md`
- RawMap truthful native seam inventory（HashMap backend で live にしてよい語彙の棚卸し）: `docs/development/current/main/design/raw-map-truthful-native-seam-inventory.md`
- Atomic/TLS/GC truthful native seam inventory（seam-first widening の棚卸し）: `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
- Hako host facade contract（HostFacade 単一入口/I-F/昇格ゲートSSOT）: `docs/development/current/main/design/hako-host-facade-contract-ssot.md`
- WASM `.hako`-only output roadmap（WASM出力の Rust->`.hako` 移行順SSOT）: `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
- PyVM retreat（historical/opt-in 契約SSOT）: `docs/development/current/main/design/pyvm-retreat-ssot.md`
- Planner Entry Guards（Facts の reject_reason/handoff 構造化）: `docs/development/current/main/design/planner-entry-guards-ssot.md`
- Type System Policy（MirType/RuntimeTypeTag/TypeView の責務分離SSOT）: `docs/development/current/main/design/type-system-policy-ssot.md`
- Campaign: compiler cleanliness（compiler-first / BoxShape-first の運用SSOT）: `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
- Compiler task map（日々の固定順序 + round pack SSOT）: `docs/development/current/main/design/compiler-task-map-ssot.md`
- PhiInputStrategy（loop PHI 入力形の設計SSOT）: `docs/development/current/main/design/phi-input-strategy-ssot.md`
- PHI Lifecycle（PHIの予約→定義→入力確定SSOT）: `docs/development/current/main/design/phi-lifecycle-ssot.md`
- Feature helper map（cross-pipeline 地図）: `docs/development/current/main/design/feature-helper-cross-pipeline-map.md`
- CoreLoop ContinueTarget slot（continue の飛び先を slot 化）: `docs/development/current/main/design/coreloop-continue-target-slot-ssot.md`
- Code-side registry（実装とSSOTの対応表）: `src/mir/builder/control_flow/plan/REGISTRY.md`

## Historical Migration Ledgers

- Historical payload-lane cleanup tracker: `docs/development/current/main/design/archive/domainplan-thinning-ssot.md`
- Historical payload-lane residue ledger: `docs/development/current/main/design/archive/domainplan-residue-ssot.md`
- Historical recipe-first entry migration notes: `docs/development/current/main/design/archive/recipe-first-entry-contract-history.md`
- Historical recipe-first phased proposal: `docs/development/current/main/design/archive/recipe-first-migration-phased-plan-proposal.md`

## Retirement / Inventory Supporting Docs

- `route-physical-path-legacy-lane-ssot.md`
- `loop-route-detection-physical-path-retirement-ssot.md`
- `execution-lanes-legacy-retirement-inventory-ssot.md`
- `selfhost-smoke-retirement-inventory-ssot.md`
- `joinir-smoke-legacy-stem-retirement-ssot.md`
- `joinir-legacy-fixture-pin-inventory-ssot.md`
- `joinir-frontend-legacy-fixture-key-retirement-ssot.md`
- `mir-callsite-retire-lane-ssot.md`
- `pyvm-retreat-ssot.md`
- `selfhost-bootstrap-route-evidence-and-legacy-lanes.md`

## Diagnostics / Contracts（入口）

- Freeze / debug tag SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- MIR diagnostics contract SSOT（freeze builder + caller/mir_dump 標準化）: `docs/development/current/main/design/mir-diagnostics-contract-ssot.md`
- MIR VM/LLVM instruction contract fix SSOT（命令契約の修正順序）: `docs/development/current/main/design/mir-vm-llvm-instruction-contract-fix-ssot.md`
- MIR instruction diet ledger SSOT（kept/lowered-away/removed 台帳）: `docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md`
- MIR canonical callsite lane SSOT（call-site 統一の実行指示）: `docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md`
- Stage1 MIR dialect contract SSOT（Stage0 keep / Stage1 mainline の call dialect 分離）: `docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md`
- Stage1 MIR authority boundary SSOT（`.hako authority / Rust materializer / native consumer` の境界固定）: `docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md`
- String helper density optimization SSOT（substring/concat/indexOf/length 最適化責務）: `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
- Helper boundary policy SSOT（host handle / string span cache tuning 集約）: `docs/development/current/main/design/helper-boundary-policy-ssot.md`
- Optimization portability classification SSOT（最適化の移植可能性分類）: `docs/development/current/main/design/optimization-portability-classification-ssot.md`
- AutoSpecializeBox SSOT（MIRCall 自動分岐 v0 契約）: `docs/development/current/main/design/auto-specialize-box-ssot.md`
- Code retirement/history policy SSOT（退役コード保存方針）: `docs/development/current/main/design/code-retirement-history-policy-ssot.md`
- normalized_dev removal SSOT（dev-only normalized lane の隔離→削除順序）: `docs/development/current/main/design/normalized-dev-removal-ssot.md`
- JoinIR frontend legacy fixture key retirement SSOT（by-name fixture key の alias-first 撤去順序）: `docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md`
- Selfhost smoke retirement inventory SSOT（Mini-VM / Stage-B canary の retire/archive 分類）: `docs/development/current/main/design/selfhost-smoke-retirement-inventory-ssot.md`
- vm-hako array shim contract SSOT（array_get/array_set の interim 契約）: `docs/development/current/main/design/vm-hako-array-shim-contract-ssot.md`
- MIR callsite retire lane SSOT（post-canonical 削除順序/契約）: `docs/development/current/main/design/mir-callsite-retire-lane-ssot.md`
- `.hako` mirbuilder Load/Store minimal contract SSOT（B1 docs-first）: `docs/development/current/main/design/hako-mirbuilder-load-store-minimal-contract-ssot.md`
- Copy emission SSOT（直Copy禁止/CopyEmitter）: `docs/development/current/main/design/copy-emission-ssot.md`
- PlanLowerer entry SSOT（CorePlan→MIR 入口allowlist）: `docs/development/current/main/design/plan-lowering-entry-ssot.md`
- Builder emit facade SSOT（生emitの層境界/可視性契約）: `docs/development/current/main/design/builder-emit-facade-visibility-ssot.md`
- Fini/Cleanup execution contract SSOT（Stage-B JSON v0 bridge 実行契約）: `docs/development/current/main/design/fini-cleanup-execution-contract-ssot.md`
