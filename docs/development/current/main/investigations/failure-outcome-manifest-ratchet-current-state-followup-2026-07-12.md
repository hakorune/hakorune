# Failure/Outcome Manifest and Current-State Follow-up

Status: Parked follow-up task; not the active lane.
Date: 2026-07-12
Active lane remains: 3455 MapStore caller-orientation design stop.

## Purpose

Record the verified leftovers found during the restart audit. This task does
not select a Failure/Outcome semantic owner, activate a backend, or reopen the
3455 Fact / Plan / Boundary consultation.

## Tasks

### FOM-1 — Regenerate semantic-site manifest

The checked-in semantic graph is stale after source line movement. Regenerate
the manifest from the current source and verify that the drift check is green.

Acceptance:

```bash
python3 tools/docs/failure_outcome_semantic_site_graph.py --write
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
```

Expected result: `--check` exits 0; occurrence/site counts remain evidence
derived and no semantic owner is selected.

### FOM-2 — Add the minimal PR-only semantic graph ratchet

Wire one direct check into the existing `.github/workflows/min-gate.yml`
`rust-check` job. This workflow already runs on pull requests, so do not add a
push trigger, `dev_gate quick` step, pre-commit hook, cargo build, LLVM setup,
or a second workflow. The check is Python-only and runs after checkout.

Acceptance:

```bash
rg -n "failure_outcome_semantic_site_graph.py --check" .github/workflows/min-gate.yml
```

Expected result: exactly one PR-only gate path owns the check; ordinary pushes
pay no new CI cost; duplicate ad-hoc wiring is rejected; the gate fails on a
deliberately stale manifest fixture.

### FOM-3 — Synchronize CURRENT_STATE landed tail

Add the already-landed 2026-07-12 milestones to `landed_tail` or replace the
tail with a compact pointer to the owning cards. Keep `CURRENT_STATE.toml` as
the only current-lane authority; do not copy chronology into restart mirrors.

Acceptance:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

Expected result: landed history and active pointer agree, while 3455 remains
the active design stop until a new owner is explicitly selected.

### FOM-4 — Repair `latest_card_summary` prose splice

Rewrite the mixed past/imperative fragment around the 3456/3454/3455 summary
into one grammatical current-state paragraph. Preserve the existing claims and
non-claims; this is a documentation repair only.

Acceptance:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Ordering

1. FOM-1 manifest regeneration.
2. FOM-2 gate-owner selection and ratchet wiring.
3. FOM-3/FOM-4 current-state synchronization.

FOM-2 is a gate-design boundary. If its owner or CI surface is ambiguous,
stop for design consultation rather than adding duplicate checks.

## Explicit non-claims

```text
failure_outcome_semantic_owner_selected = 0
failure_outcome_language_semantics_changed = 0
3455_fact_plan_boundary_owner_selected = 0
source_selfhost_claim = 0
```
