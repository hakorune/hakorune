---
Status: SSOT
Date: 2026-03-24
Scope: main ラインの current summary と正本リンクだけを置く薄い mirror/dashboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/20-Decisions.md
  - docs/development/current/main/30-Backlog.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は docs 側の薄い mirror/dashboard だよ。
- 置くのは current summary、実行入口、正本リンクだけ。
- 進捗履歴や長文ログは `CURRENT_TASK.md`、phase README、design SSOT に逃がす。

## Root Anchors

- Root anchor: `CURRENT_TASK.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Layout contract: `docs/development/current/main/DOCS_LAYOUT.md`

## Current Read

- Compiler lane: `phase-29bq` は landed / monitor-only、active blocker は `none`。
- Execution lane policy: parent SSOT is `execution-lanes-and-axis-separation-ssot.md`; `stage1` は bridge/proof only、`stage2+` が final distribution target。
- Runtime lane: `phase-29y` が current anchor。operational reading は `llvm-exe` daily / `vm-hako` reference-debug-bootstrap-proof / `rust-vm` bootstrap-recovery-compat。active acceptance は `phase29y_vm_hako_caps_gate_vm.sh` のみで、throughput probes は archived monitor evidence として扱う。
- Kernel capability lane: `phase-29ct` は stop-line reached、`C5` は deferred。
- Bootstrap retire lane: active mainline front is `phase-29cj`; the latest Rust source-route authority / output projection split is landed in `src/host_providers/mir_builder/handoff.rs`, and the current exact `.hako` cut is `MirBuilderBox` loop-force route isolation above `module_to_mir_json(...)`, while bridge/helper waves stay closeout-ready.
- Rune lane: `phase-29cu` は active current lane。grammar activation は Rust parser / `.hako` parser 両方前提で、carrier は declaration-local `attrs.runes` -> direct MIR attrs、`ny-llvmc` は selected-entry only。
- Rune source-route note: `.hako` の source-route keep は selected-entry attrs を synthetic `Main.main` def で一時輸送してよいが、Program(JSON v0) root/body は no-widen のまま。
- JSON v0 reading: `Program(JSON v0)` は Rune の retire/no-widen target、`MIR(JSON v0)` は current interchange / gate boundary。
- De-rust lane: `phase-29cc` は closeout 済みで、follow-up は `phase-29ce` / `phase-29cf` に分離済み。

## Clean-Shape Status

1. `stage1/stage2` artifact semantics の整理（landed）
2. `ABI/export manifest + generated shim` 化（landed）
3. `hako_alloc` root の物理再編（landed）
4. transitional Rust export の daily-path 退役（landed）
5. handle/provider/birth の substrate-only 化（docs-locked）
6. `Stage3` gate 追加（landed）
   - build lane compares re-emitted Program/MIR payload snapshots from a known-good seed plus `.artifact_kind`
   - skip-build lane compares an explicit prebuilt pair

## Exact Links

- Execution lane policy: `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- Execution lane task pack: `docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md`
- Execution lane legacy inventory: `docs/development/current/main/design/execution-lanes-legacy-retirement-inventory-ssot.md`
- Bootstrap route SSOT: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Compiler structure SSOT: `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- Stage axis SSOT: `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md`
- Rune final shape SSOT: `docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md`
- Rune v0 rollout SSOT: `docs/development/current/main/design/rune-v0-contract-rollout-ssot.md`
- Stage3 same-result gate: `tools/selfhost/stage3_same_result_check.sh`
- ABI inventory: `docs/development/current/main/design/abi-export-inventory.md`
- JSON v0 inventory: `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
- Route split note: `docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md`
- Phase 29ci closeout: `docs/development/current/main/phases/phase-29ci/README.md`

## Restart Reminder

- 最初に `git status -sb` を見る。
- 次に `CURRENT_TASK.md` を読む。
- 詳細は `10-Now.md` を増やさず、phase README / design SSOT を開く。
