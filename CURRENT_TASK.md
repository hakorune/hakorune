# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-12
Scope: repo root から current lane / current front / restart read order に最短で戻るための薄い pointer。

## Purpose

- root から active lane / next lane に最短で戻る
- landed history / rejected perf evidence は phase docs と investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-163x/README.md`
4. `git status -sb`
5. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- expected worktree:
  - clean
- active lane:
  - `phase-163x primitive and user-box fast path`
- sibling guardrail:
  - `phase-137x` string corridor / exact-keeper guardrail
- immediate next:
  - `semantic simplification bundle lane B2`
  - overwritten `Store` pruning for definitely private carrier roots
- immediate follow-on:
  - lane C0 observer/control docs-only inventory
  - lane C1 `Debug` policy decision
- current stop-lines:
  - do not mix lane B with lane C (`Debug` / terminator observer cleanup)
  - do not mix lane B with `generic placement / effect`
  - do not mix parked `phase-96x` backlog into the active optimization lane
- parked corridor:
  - `phase-96x vm_hako LLVM acceptance cutover`
  - only remaining backlog is monitor-policy decision for the frozen `vm-hako-core` pack

## Design Owners

- implementation lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- roadmap SSOT:
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- DCE lane split owner:
  - `docs/development/current/main/phases/phase-190x/190x-90-remaining-dce-boundary-inventory-ssot.md`
- generic memory lane-B contract owner:
  - `docs/development/current/main/design/generic-memory-dce-observer-owner-contract-ssot.md`
- primitive / user-box fast-path owner:
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- lifecycle / value parent:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
- string guardrail owner:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- post-primitive enum / generic owner:
  - `docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md`

## Recent Landed Summary

- `phase-165x` / `phase-166x`
  - semantic refresh, generic relation owners, and compat recovery quarantine are landed
- `phase-167x` / `phase-168x`
  - user-box method determinism and exact-route refresh are landed
- `phase-169x` through `phase-180x`
  - sibling string guardrail, `StringKernelPlan`, publication slices, and seam cleanup are landed
  - only the final emitted-MIR return-carrier cleanup stays parked on `phase-137x`
- `phase-176x` / `phase-177x` / `phase-181x` / `phase-182x` / `phase-183x` / `phase-184x` / `phase-185x` / `phase-186x` / `phase-187x` / `phase-188x` / `phase-189x` / `phase-190x` / `phase-191x` / `phase-192x` / `phase-196x`
  - semantic simplification bundle is landed through DCE lane A2
- `phase-178x` / `phase-193x` / `phase-194x`
  - BoxShape-only structure cuts are landed
- `phase-195x` / `phase-197x`
  - roadmap regroup and pointer hygiene are landed
- `phase-198x` / `phase-199x`
  - root restart surfaces are compressed and lane-B docs/facts are fixed
- `phase-200x`
  - dead `Load` pruning is now landed for definitely private carrier roots with copy-only alias propagation
  - first cut keeps `Store`, `Debug`, terminator, and generic placement/effect widening out of scope

## Current Checks

1. `git status -sb`
2. `tools/checks/dev_gate.sh quick`
3. `git diff --check`

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-163x/README.md`
4. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
5. `docs/development/current/main/phases/phase-137x/README.md`

## Current Front

- read [phase-163x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-163x/README.md) for the active implementation lane
- read [phase-137x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-137x/README.md) for the sibling string guardrail
- read [phase137x-substring-rejected-optimizations-2026-04-08.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md) before retrying any substring-local perf cut
