---
Status: Accepted (top-level closeout done; optional separate-repo follow-up only)
Decision: accepted
Date: 2026-03-09
Scope: 脱Rust selfhost lane の「完了済み」と「残タスク」を checkbox で固定する。`CURRENT_TASK.md` は薄い入口のまま保つ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md
  - docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-179-plg07-min1-min2-filebox-binary-rust-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-204-plg07-min7-filebox-retire-execution-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-209-plg-hm1-core8-module-provider-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-210-plg-hm2-core-wave2-rust-recovery-line-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-213-plg-hm3-next-blocker-candidate-memo.md
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-253-source-zero-static-link-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md
  - docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md
---

# 29cc-260 De-Rust Selfhost Task Checklist

## Purpose

- 脱Rust selfhost の完了済み領域と、まだ残っている aftercare / future-wave を 1 枚で固定する。
- `CURRENT_TASK.md` には blocker と next order だけを書き、詳細な棚卸しはこの checklist を正本にする。
- plugin lane が「まだ未着手」ではなく「かなり done」であることを、wave 単位で見えるようにする。

## 0) Current normalized snapshot (2026-03-09)

- [x] non-plugin de-rust done declaration accepted（`29cc-94`）
- [x] 脱Rust selfhost orchestration lane active next = `none`（monitor-only）
- [x] runtime lane active next = `none`（monitor-only）
- [x] selfhost / planner-required gate green
- [x] no-compat mainline VM gate green

## 1) Completed foundations

### 1.1 Non-plugin lane

- [x] `RNR-01` vm_hako compile bridge seam split
- [x] `RNR-02` shape/payload contract consolidation
- [x] `RNR-03` selfhost JSON payload ownership consolidation
- [x] `RNR-04` orchestrator meaning-decision retirement
- [x] `RNR-05` parser + plan single-shape pack
- [x] non-plugin de-rust done sync accepted

### 1.2 Plugin lane rewrite waves

- [x] `PLG-01` ABI / loader acceptance lock
- [x] `PLG-02` plugin gate pack lock
- [x] `PLG-03` wave-1 CounterBox pilot
- [x] `PLG-04` wave-1 core rollout complete
  - [x] ArrayBox
  - [x] IntCellBox reserved-core lock
  - [x] MapBox
  - [x] StringBox
  - [x] ConsoleBox
  - [x] FileBox
- [x] `PLG-05` wave-2 utility rollout complete
  - [x] Json
  - [x] TOML
  - [x] Regex
  - [x] Encoding
  - [x] Path
  - [x] Math
  - [x] Net
- [x] `PLG-06` wave-3 rollout complete
  - [x] PythonCompiler
  - [x] Python
  - [x] PythonParser
  - [x] Egui
- [x] `PLG-07` FileBox binary retire execution lock complete

### 1.3 Plugin post-wave route hardening

- [x] `PLG-HM1` Core8 module-provider route lock complete
- [x] `PLG-HM2` Rust recovery line + route matrix lock complete
- [x] `PLG-HM3` residue classification lock complete

## 2) Fixed keep / explicit non-goals

- [x] `plugins/nyash-integer-plugin`
  - classification: `mainline keep`
- [x] `plugins/nyash-fixture-plugin`
  - classification: `test-only keep`
- [x] plugin lane is not a blocker for non-plugin done
- [x] plugin full replacement is a separate lane / future decision, not part of current done declaration

## 3) Remaining de-rust tasks

### 3.1 Source-zero / bootstrap boundary

- [x] `DRC-01` source-zero final wave inventory refresh
  - goal: re-list which Rust runtime/plugin sources are still intentionally kept under `source keep`
  - authority: `29cc-220`, `29cc-253`
  - result: source-zero/source-keep buckets are fixed as follows

| Bucket | Exact kept areas (current paths) | Classification | Authority |
| --- | --- | --- | --- |
| runtime keep | `src/runtime/plugin_loader_v2/enabled/**`, `src/runtime/plugin_loader_unified.rs`, `src/runtime/semantics.rs`, `src/runtime/box_registry.rs`, `src/runtime/host_api/common.rs`, `crates/nyash_kernel/src/plugin/**`, `crates/nyash_kernel/src/exports/string.rs`, `crates/nyash_kernel/src/hako_forward.rs`, `crates/nyash_kernel/src/hako_forward_bridge.rs` | `source keep` during route-zero / source-zero prep; future source-zero retire target | `29cc-220`, `29cc-245`, `29cc-253`, `29cc-244` |
| bootstrap keep | `crates/nyash-llvm-compiler/src/main.rs`, `src/runner/modes/common_util/exec.rs`, `src/runner/modes/mir.rs`, `src/runner/modes/llvm/harness_executor.rs` | `bootstrap keep` until static-link/bootstrap boundary retire phase | `29cc-253`, `29cc-245`, `10-Now.md` |
| plugin keep | `plugins/nyash-integer-plugin/**` | `mainline keep` | `29cc-213`, this checklist §2 |
| test-only keep | `plugins/nyash-fixture-plugin/**` | `test-only keep` | `29cc-213`, `phase-29cc/README.md`, this checklist §2 |

- [x] `DRC-02` bootstrap boundary inventory
  - goal: make Stage0 / Stage1 / Stage2 Rust dependency explicit
  - result: selfhost build path documents allowed keeps vs future retire targets

| Boundary | Current contract | Bucket | Future retire note | Authority |
| --- | --- | --- | --- | --- |
| Stage definition / G1 judgment | Stage0/Stage1/Stage2 are fixed; G1 advances only when Stage1 == Stage2 | explicit keep | keep until source-zero/bootstrap final wave | `selfhost-bootstrap-route-ssot.md`, `selfhost-parser-mirbuilder-migration-order-ssot.md` |
| Identity route | `stage1` is current mainline; `stage0` / `auto` are explicit compat routes only | explicit keep | retire compat routes only after stage1-first evidence stays green | `tools/selfhost/selfhost_identity_check.sh`, `tools/selfhost/lib/identity_routes.sh` |
| Full-mode evidence | full identity evidence requires Program+MIR on `stage1` route | explicit keep | no retire until full-mode evidence contract changes | `tools/selfhost/selfhost_identity_check.sh` |
| Artifact kind | build default is `launcher-exe`; G1/full emit identity requires `stage1-cli` artifact | explicit keep | retire only when launcher/exe and emit identity surfaces unify | `tools/selfhost/build_stage1.sh`, `selfhost-bootstrap-route-ssot.md` |
| Stage2 build dependency | even with `stage1-cli`, Stage2 build still depends on default bootstrap lane | bootstrap keep | future retire target after stage1-first build path removes default bootstrap dependency | `tools/selfhost/selfhost_identity_check.sh` |
| Stage0 bootstrap build | Rust binary acts as bootstrap when building Stage1 | bootstrap keep | future retire target; current explicit keep | `tools/selfhost/build_stage1.sh`, `29cc-253` |

### 3.2 Live compat retirement

- [x] `DRC-03` `SMOKES_SELFHOST_FILTER` semantic-only closeout
  - goal: old exact basename examples become inventory-only
  - done when: active docs / active daily commands use semantic substring or semantic fixture alias only
- [x] `DRC-04` Program JSON by-name compat key closeout
  - goal: classify remaining keys into `runtime keep` / `retire when caller=0` / `historical docs/private only`
  - authority: `src/mir/join_ir/frontend/ast_lowerer/route.rs`

### 3.3 Plugin lane aftercare / monitor-only recheck

- [x] `DRC-05` archive / recovery authority closeout
  - classification: `monitor-only fixed keep`
  - note: `29cc-210` is already `Status: Done` with `next: none (monitor-only)`; reopen only if recovery-line ownership changes
  - authority: `29cc-210`
- [x] `DRC-06` plugin residue recheck
  - classification: `monitor-only recheck`
  - note: `29cc-213` already fixes `mainline keep` / `test-only keep` / `retire` residue buckets; reopen only when selfhost/plugin work finds a new blocker
  - authority: `29cc-213`

### 3.4 Optional cleanup

- [ ] `DRC-07` `docs/private` de-rust/plugin drift sync（separate repo）
- [x] `DRC-08` micro dust sweep（comment / orphan helper / wording）
  - classification: `top-level closeout done`
  - note: `cargo check --tests` warning-free と low-risk comment/helper sweep を確認済み。今後は新しい stale residue が surfaced した時だけ reopen する

### 3.5 Separate post-closeout follow-up (does not reopen this checklist)

- [x] `DCF-01` `VM fallback compat lane` follow-up moved to `phase-29cf`
- [x] `DCF-02` `bootstrap boundary reduction` follow-up moved to `phase-29cf`
- note: `phase-29cf` は `phase-29cc` closeout を取り消さず、post-closeout inventory / keep / future-retire target を accepted monitor-only で独立管理する

## 4) Recommended execution order

1. [x] `DRC-01` source-zero inventory refresh
2. [x] `DRC-02` bootstrap boundary inventory
3. [x] `DRC-03` selfhost/live compat contract cleanup
4. [x] `DRC-04` Program JSON key classification closeout
5. [x] `DRC-05` / `DRC-06` plugin aftercare is monitor-only unless a new blocker reopens it
6. [x] `DRC-08` micro dust
7. [ ] `DRC-07` `docs/private` drift sync（separate repo）
8. [x] post-closeout `VM fallback compat lane` / `bootstrap boundary reduction` は `phase-29cf` へ分離済み

## 5) Done judgment for this checklist

- [x] `DRC-01` to `DRC-06` are either completed or explicitly reclassified as permanent keep / non-goal
- [x] active docs keep semantic-first wording
- [x] `CURRENT_TASK.md` points here as the detailed de-rust checklist
- [x] 脱Rust selfhost lane remains `monitor-only` with blocker `none`
- [x] `phase-29cf` の follow-up はこの checklist を再オープンしない
