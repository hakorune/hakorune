---
Status: Accepted (monitor-only)
Decision: accepted
Date: 2026-03-09
Scope: 脱Rust selfhost closeout 後の `VM fallback compat lane` と `bootstrap boundary reduction` を独立管理する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-260-derust-task-checklist.md
  - docs/development/current/main/phases/phase-29cf/P0-VM-FALLBACK-AND-BOOTSTRAP-BOUNDARY-INVENTORY.md
  - docs/development/current/main/phases/phase-29cf/29cf-10-vm-fallback-bootstrap-retirement-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - src/runner/route_orchestrator.rs
  - src/runner/modes/vm_fallback.rs
  - tools/selfhost/lib/identity_routes.sh
  - tools/selfhost/build_stage1.sh
---

# Phase 29cf: De-Rust Selfhost Follow-up

## Goal

`phase-29cc` の top-level closeout を再オープンせず、残っている

1. `VM fallback compat lane`
2. `bootstrap boundary reduction`

を docs-first で棚卸しし、`keep` / `future retire target` / `monitor-only` を固定する。

## Closeout Decision

1. `compat-fallback` は current mainline route ではなく、explicit compatibility keep として固定する
2. Stage0 / Stage1 / Stage2 の bootstrap boundary は inventory-fixed とし、実削減は future-wave へ分離する
3. 本 phase は `monitor-only` で維持し、route authority か bootstrap dependency に具体的な削減候補が出た時だけ再オープンする

## Master Pointer

- `phase-29cc` は closeout 済みの orchestration lane として維持する
- この phase は closeout 後の follow-up を独立管理する
- checkbox 正本:
  - `docs/development/current/main/phases/phase-29cf/29cf-10-vm-fallback-bootstrap-retirement-checklist.md`

## Non-goals

- `phase-29cc` の done judgment を巻き戻すこと
- silent fallback を増やすこと
- current mainline route を `compat-fallback` へ戻すこと

## Fixed Workstreams

1. `VM fallback compat lane`
   - `vm` / `vm-hako` / `compat-fallback` の current contract を固定する
   - `NYASH_VM_USE_FALLBACK=1` を explicit compat keep として扱う
2. `bootstrap boundary reduction`
   - Stage0 / Stage1 / Stage2 の keep boundary を明示する
   - `stage1-cli` と default bootstrap lane の dependency を分けて読む

## Current Snapshot

- current mainline route:
  - `vm`
  - `vm-hako`
- explicit compat keep:
  - `compat-fallback`
  - `stage0` recovery in `identity_routes.sh`
- bootstrap boundary:
  - `stage1` route is current selfhost identity route
  - `stage0` / `auto` are compat-only
  - Stage2 still depends on default bootstrap lane and remains a future reduction target
- `docs/private` drift is out of scope here

## Exit Criteria

- `VM fallback compat lane` の keep/reject/future-retire bucket が checklist で固定されている
- `bootstrap boundary reduction` の Stage0/1/2 keep matrix が checklist で固定されている
- current docs / comments が `compat keep` と `mainline keep` を混同しない
