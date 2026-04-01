---
Status: SSOT
Decision: provisional
Date: 2026-03-24
Scope: `stage axis` / `owner-substrate axis` / `artifact-lane axis` を 1 枚で切り分け、stage1 昇格 drift と runtime lane の混線を防ぐ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/hakoruneup-release-distribution-ssot.md
  - docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md
  - docs/development/current/main/design/execution-lanes-legacy-retirement-inventory-ssot.md
  - lang/README.md
---

# Execution Lanes And Axis Separation (SSOT)

## Purpose

- `stage0/stage1/stage2-mainline/stage2+` と `owner/substrate` と `artifact/lane` を別々に読む。
- `stage1 artifact が動く` を `daily mainline` や `distribution truth` と誤読しない。
- `vm-hako` の semantic/reference lane と `llvm-exe` の daily lane を混ぜない。
- `rust-vm keep` を owner migration failure と誤読しない。
- artifact-role detail は child SSOT `artifact-policy-ssot.md` に集約する。

## 1. Three Axes

### 1.1 Stage axis

| axis | meaning | fixed reading |
| --- | --- | --- |
| `stage0` | Rust bootstrap / first-build / recovery lane | explicit keep |
| `stage1` | `.hako` bridge / proof / native-proof line | proof/snapshot only |
| `stage2-mainline` | daily `.hako` mainline lane | daily mainline |
| `stage2+` | umbrella / end-state distribution target | target mainline |

Concrete reading:

- `target/selfhost/hakorune`, `stage1-cli`, `launcher-exe`, `lang/bin/hakorune` are `stage1` artifacts.
- They may be runnable, but they are not final distribution truth.
- `stage2-mainline` is the daily operational lane for the current selfhost/mainline docs.
- `stage2+` is the umbrella label for the full target state; it is not the daily lane label.

### 1.2 Owner/substrate axis

| axis | meaning | preferred owner |
| --- | --- | --- |
| semantic owner | visible meaning / policy / contract / orchestration | `.hako` |
| substrate owner | ABI / handle / GC / object layout / raw leaf / LLVM metal | native keep unless separately retired |

Child SSOTs:

- stage vs owner split: `de-rust-stage-and-owner-axis-ssot.md`
- final `.hako owner + native keep` boundary: `final-metal-split-ssot.md`
- collection owner frontier: `array-map-owner-and-ring-cutover-ssot.md`

### 1.3 Artifact/lane axis

| lane | role | default posture | non-goal |
| --- | --- | --- | --- |
| `llvm-exe` | daily / CI / distribution artifact lane | default operational lane | not bootstrap proof bookkeeping |
| `vm-hako` | semantic parity / reference / debug / bootstrap-proof lane | monitor-only unless exact blocker reopens it | not daily performance lane |
| `rust-vm` | bootstrap / recovery / compat lane | explicit keep | not daily feature-growth lane |

Important:

- `default lane` in this document means docs/wrappers/CI operational default.
- It does not automatically mean the raw CLI backend token/default has already been flipped.

## 2. Reading Rules

1. `artifact proof != owner proof`
   - `.hako` artifact だからといって `.hako` owner proof とは限らない。
2. `stage1 success != stage2-mainline complete`
   - bridge/proof line が green でも daily mainline completion ではない。
3. `vm-hako green != mainline promotion`
   - `vm-hako` は reference/debug/bootstrap-proof lane のまま読む。
4. `rust-vm keep != owner migration failure`
   - `stage0` recovery/bootstrap keep は許可された residue。
5. `distribution truth starts at stage2-mainline`
   - stage1 snapshot/stable artifacts are not final package/distribution truth.
   - `stage2+` remains the umbrella / end-state reading, not the daily lane name.

## 3. Child Ownership Map

- This parent SSOT owns the shared vocabulary only.
- `de-rust-stage-and-owner-axis-ssot.md`
  - owns the `stage vs owner` split
- `stage2-selfhost-and-hako-alloc-ssot.md`
  - owns stage/distribution layering plus `hako_core/alloc/std` reading
- `selfhost-bootstrap-route-ssot.md`
  - owns stage1 bootstrap route authority and proof boundaries
- `de-rust-lane-map-ssot.md` and `phase-29y/README.md`
  - own runtime operation policy and reopen rules
- `hakoruneup-release-distribution-ssot.md`
  - owns stage2-mainline package/distribution shape and the `stage2+` umbrella reading
- `artifact-policy-ssot.md`
  - owns artifact-role detail and future interpreter reservation
- `execution-lanes-migration-task-pack-ssot.md`
  - owns the cross-phase implementation order
- `execution-lanes-legacy-retirement-inventory-ssot.md`
  - owns migration-discovered legacy/delete-candidate triage

## 3.5 Placement Rule

Keep artifact roots, source roots, task packs, and phase logs separate.

1. artifact roots
   - `target/**`
   - `artifacts/**`
   - `dist/**`
   - own binaries / bundles / promoted snapshots only
   - do not store migration-task notes or implementation-order logs here
2. source roots
   - `lang/src/**`
   - `crates/**`
   - own implementation modules by responsibility name (`hako_kernel`, `hako_substrate`, `hako_alloc`, etc.)
   - do not create `K0/K1/K2` source trees
3. task-pack docs
   - `CURRENT_TASK.md`
   - `docs/development/current/main/15-Workstream-Map.md`
   - `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
   - own current task order / next structural step / acceptance stop-lines
4. phase logs
   - `docs/development/current/main/phases/**`
   - own narrow slice history, blockers, and reopen rules

Reading rule:

- `K0/K1/K2` folders are for artifact placement only.
- migration tasks stay under task-pack docs and phase logs.
- rough task order should not be duplicated into a new artifact-oriented SSOT.

## 4. Phase Mapping

| phase/doc owner | role in this policy |
| --- | --- |
| `phase-29ct` | stage0/native substrate reading |
| `phase-29ci` | stage1 bridge/proof boundary |
| `phase-29y` | runtime lane policy |
| `phase-29cu` | Rune close-synced keep only; not general distribution policy |

## Non-Goals

- flipping the raw CLI backend default in this docs-lock wave
- treating `stage1` artifacts as final distribution truth
- treating `vm-hako` as a daily performance lane
- treating `rust-vm` as a daily feature lane
- merging distribution policy into Rune or bootstrap-boundary docs
