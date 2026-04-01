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
- active order:
  1. `stage / docs / naming` fixation
  2. `K1 done-enough` stop-line fixation
  3. `K2-core` accepted stop-line
  4. `K2-wide` next structural follow-up
  5. `zero-rust` default operationalization
- stage axis:
  - `stage0 = bootstrap/recovery keep`
  - `stage1 = same-boundary swap proof`
  - `stage2-mainline = daily mainline / distribution lane`
  - `stage2+ = umbrella / end-state label`
- `K-axis`:
  - `K0 = all-Rust hakorune`
  - `K1 = .hako kernel migration stage`
  - `K2 = .hako kernel mainline / zero-rust daily-distribution stage`
  - `K2-core` / `K2-wide` are task packs inside `K2`

## Current Read

- `K2-core` is closed as the accepted `RawArray first truthful substrate` stop-line.
- next structural step is `K2-wide`.
- current `K2-wide` sequence reads:
  1. `RawMapCoreBox`
  2. `hako.atomic`
  3. `hako.tls`
  4. `hako.gc`
  5. `hako.osvm`
  6. `hako_alloc` policy/state rows
  7. metal keep review / boundary-shrink planning
- landed rows already visible in code/tests:
  - `AtomicCoreBox.fence_i64()`
  - `TlsCoreBox.last_error_text_h()`
  - `GcCoreBox.write_barrier_i64(handle_or_ptr)`
  - `OsVmCoreBox.reserve_bytes_i64(len_bytes)`
  - `hako_alloc` handle reuse policy
  - `hako_alloc` GC trigger threshold policy
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
