---
Status: Active
Date: 2026-04-12
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane:
  - `phase-163x primitive and user-box fast path`
- sibling guardrail:
  - `phase-137x main kilo reopen selection`
- immediate next:
  - `observer/control lane C2b`
  - legacy in-instruction-list control-anchor seed cleanup
- immediate follow-on:
  - `observer/control lane C2c`
  - simplification-handoff wording lock, then return to the next layer step
- stop-lines:
  - keep lane B separate from lane C (`Debug` / terminator-adjacent operand/control liveness cleanup)
  - keep lane B separate from `generic placement / effect`
  - do not mix parked `phase-96x` backlog into the current lane

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-163x/README.md`
4. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
5. `docs/development/current/main/phases/phase-137x/README.md`

## Current Evidence

- semantic refresh / generic relation ownership is landed:
  - `phase-165x`
  - `phase-166x`
- string guardrail / seam cleanup is landed through:
  - `phase-169x` to `phase-180x`
- semantic simplification bundle is landed through DCE lane A2:
  - `phase-176x`
  - `phase-177x`
  - `phase-181x` to `phase-192x`
  - `phase-196x`
- roadmap regroup / pointer sync is landed:
  - `phase-195x`
  - `phase-197x`
  - `phase-198x`
  - `phase-199x`
  - `phase-200x`
  - `phase-201x`
  - `phase-202x`
  - `phase-203x`
  - `phase-204x`

## First Design Slices

- `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- `docs/development/current/main/design/observer-control-dce-owner-contract-ssot.md`
- `docs/development/current/main/phases/phase-190x/190x-90-remaining-dce-boundary-inventory-ssot.md`
- `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
- `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
