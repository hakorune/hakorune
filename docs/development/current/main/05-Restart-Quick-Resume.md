---
Status: Active
Date: 2026-04-15
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
cargo check --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```

## Current

- lane: `phase-29bq selfhost mirbuilder failure-driven`
- guardrail: `phase-137x string corridor / exact-keeper guardrail`
- immediate next: `compiler expressivity first`
- immediate follow-on: `phase-29bq loop owner seam cleanup`

## Current Handoff

- blocker: `none`
- residue exact shape:
  - explicit facts-local `plan_residue` under `facts/`
  - intentional top-level owner surfaces remain under `recipes / lower / verify / ssa / cleanup / facts`
  - composite keep-plan residue remains in `plan/policies`
  - isolated owner-local keep-plan bridge under `joinir/route_entry::owner_local_compat`
- next exact handoff:
  - safe tiny wrapper cleanup is exhausted for owner-preserving seams
  - keep top-level owner surfaces in `recipes / lower / verify / ssa / cleanup / facts`
  - treat `plan/policies` as composite residue until its keep-plan policies get single-owner homes
  - keep `facts::plan_residue` explicit and thin while `plan/facts/*` ownership continues to move
  - keep the isolated owner-local keep-plan bridge minimal in `joinir/route_entry::owner_local_compat`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/15-Workstream-Map.md`
5. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
6. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
7. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
8. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
9. `docs/development/current/main/phases/phase-137x/README.md`

## Current Proof Bundle

```bash
git status -sb
cargo check --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```
