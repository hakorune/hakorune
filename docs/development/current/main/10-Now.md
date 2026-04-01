---
Status: SSOT
Date: 2026-04-01
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

- current lane is still `policy-refresh`
- exact current order is owned by `CURRENT_TASK.md` and `15-Workstream-Map.md`
- `K-axis`:
  - `K0 = all-Rust hakorune`
  - `K1 = .hako kernel migration stage`
  - `K2 = .hako kernel mainline / zero-rust daily-distribution stage`
  - `K2-core` / `K2-wide` are task packs inside `K2`

## Current Read

- `K2-core` is closed as the accepted `RawArray first truthful substrate` stop-line.
- next structural step is `K2-wide`.
- current `K2-wide` focus is metal keep review / boundary-shrink planning.
- landed rows are tracked in `CURRENT_TASK.md` and the technical SSOTs below.
- portability split stays explicit:
  - `.hako` owns capability facades
  - final OS VM / TLS / atomic / GC leaf glue stays native keep
- artifact reading stays:
  - current reality: `target/release/hakorune`, `target/selfhost/hakorune`, `lang/bin/hakorune`
  - target contract: `target/k0|k1/`, `artifacts/k0|k1/`, `dist/k2/<channel>/<triple>/bundle/`
  - migration tasks do not live in artifact roots

## Exact Links

- rough order / next slices:
  - `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
- axis / artifact / placement:
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- current `K2-wide` technical detail:
  - `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
  - `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/final-metal-split-ssot.md`
  - `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
- current phase detail:
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29bq/README.md`

## Restart Reminder

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read the current SSOT for the active slice
4. run `tools/checks/dev_gate.sh quick`
