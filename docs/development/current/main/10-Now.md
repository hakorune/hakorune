---
Status: SSOT
Date: 2026-04-02
Scope: main ラインの current summary と正本リンクだけを置く薄い mirror/dashboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は docs 側の薄い mirror/dashboard。
- current summary、next exact read、正本リンクだけを置く。
- 長い landed history、acceptance detail、phase ledger は owner doc に逃がす。

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`
- layout contract: `docs/development/current/main/DOCS_LAYOUT.md`

## Immediate Resume

- current lane is `phase-30x backend surface simplification`
- exact current order is owned by `CURRENT_TASK.md` and `15-Workstream-Map.md`
- axis details are canonical in:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md`

## Current Read

- `K2-core` is closed as the accepted `RawArray first truthful substrate` stop-line.
- `K2-wide` boundary-shrink lock-down is closed enough for handoff.
- `zero-rust default operationalization` is landed; `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` are already landed and `page_size` stays parked.
- `stage2plus entry / first optimization wave` is accepted; the active front has moved to `phase-30x backend surface simplification`.
- boundary audit result: `RuntimeDataBox` remains facade-only and delete stays on `MapBox` / `RawMap`.
- `phase-29x` W4/W5/W6 is landed; explicit helper deletion is closed and the active docs front is no longer `29x`.
- current active step is `phase-30x backend surface simplification`.
- current backend reading is role-first:
  - `llvm/exe` = `product`
  - `rust-vm` = `engineering/bootstrap`
  - `vm-hako` = `reference/conformance`
  - `wasm` = `experimental`
- `rust-vm` still has deep pressure in bootstrap/selfhost, plugin/macro/dev tooling, smoke/test, and docs/help.
- dangerous early flips stay frozen around launcher/default/orchestrator sites such as `src/cli/args.rs`, `src/runner/dispatch.rs`, `src/runner/modes/common_util/selfhost/child.rs`, `lang/src/runner/stage1_cli/core.hako`, and `tools/selfhost/run.sh`.
- `30xA1`, `30xA2`, `30xB1-30xB4`, `30xC1`, `30xC2`, and `30xC3` are landed.
- `30xC2` grouped plugin/macro/tooling pressure into `engineering/tooling keep` and `manual residue watch`.
- `30xC3` grouped smoke/test pressure into `engineering smoke keep`, `mixed orchestrator keep`, and `manual residue watch`.
- current active micro task is `30xC4 rust-vm docs/help inventory`.
- next queued micro task is `30xD1 default/dispatch freeze`.
- `phase29cc_wsm` families are experimental smoke lanes, not product-mainline evidence.
- `compat/llvmlite-monitor-keep` is compat/probe keep only, not `llvm/exe` product evidence.
- `tools/smokes/v2/configs/matrix.conf` now reads `vm/llvm` as engineering/product only.
- `30xC1` found no archive/delete candidate in bootstrap/selfhost; all current hits remain keep surfaces.
- review intake lives in `phase-30x`; this mirror only carries the open deltas.
- current LLVM follow-up is organized separately from `K2-wide`; see backend lane docs for the live lane names.
- landed rows are tracked in `CURRENT_TASK.md` and the technical SSOTs below.
- portability split stays explicit:
  - `.hako` owns capability facades
  - final OS VM / TLS / atomic / GC leaf glue stays native keep
- artifact reading stays:
  - current reality: `target/release/hakorune`, `target/selfhost/hakorune`, `lang/bin/hakorune`
  - target contract: `target/k0|k1/`, `artifacts/k0|k1/`, `dist/k2/<channel>/<triple>/bundle/`
  - migration tasks do not live in artifact roots
- folder structure and smoke taxonomy docs are synced; the next optimization wave can read the current layout without extra prep

## Backend Surface Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `30xC4 rust-vm docs/help inventory` | group stale main-narrative docs/help pressure |
| Next | `30xD1 default/dispatch freeze` | lock the first dangerous early flips |
| Later | `30xD2-30xF` | selfhost/plugin freeze, user-facing main switch, backend default gate |

## Backend Surface Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `30xA role taxonomy lock` | landed | current lane, labels, and mirrors |
| `30xB smoke taxonomy split` | landed | role-first gate/smoke reading |
| `30xC rust-vm dependency inventory` | active | internal `--backend vm` pressure map |
| `30xD dangerous-early-flip lock` | queued | launcher/default/orchestrator freeze |
| `30xE user-facing main switch prep` | queued | `llvm/exe` first docs/help/examples |
| `30xF backend default decision gate` | queued | raw CLI default/backend flip decision last |

## Exact Links

- rough order / next slices:
  - `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
- axis / artifact / placement:
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- current `K2-wide` technical detail:
  - `docs/development/current/main/design/raw-map-substrate-ssot.md`
  - `docs/development/current/main/design/raw-map-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
  - `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/final-metal-split-ssot.md`
  - `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
- folder / smoke layout:
  - `lang/README.md`
  - `docs/development/current/main/design/smoke-taxonomy-and-discovery-ssot.md`
  - `docs/development/testing/smoke-tests-v2.md`
  - `docs/how-to/smokes.md`
- current phase detail:
  - `docs/development/current/main/phases/phase-30x/README.md`
  - `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
  - `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29bq/README.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`

## Restart Reminder

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read the current SSOT for the active slice
4. run `tools/checks/dev_gate.sh quick`
5. if working on the active blocker-free lane, inspect `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
