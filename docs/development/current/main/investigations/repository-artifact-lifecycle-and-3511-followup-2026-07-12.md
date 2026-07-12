# Repository Artifact Lifecycle and 3511 Follow-up

Status: Verified queued follow-up; implementation not started.
Date: 2026-07-12
Active lane remains: BoundedBodyAnalysisSnapshotV0 SnapshotAlgebraV0 closure.

Decision: accepted minimal PR-only ratchet and evidence-only 3511 repair.

## Purpose

Record two verified leftovers without reopening the active 3457 authority
selection or adding a new semantic owner. The first is a real repository
artifact manifest freshness failure. The second is a 3511 contract-to-test
inventory mismatch with one apparent orphan test module.

## Tasks

### OLF-0 — Reverification ledger (closed)

Reverified on 2026-07-12:

```text
failure_outcome_semantic_site_graph --check:
  EXIT=0, evidence=602, sites=54

repository_artifact_lifecycle --check:
  EXIT=0 with WARNING inventory drift

repository_artifact_lifecycle --check --strict:
  EXIT=1 with ERROR inventory drift

cargo test -q --lib hako_mem_free -- --list:
  EXIT=0, four collected tests

current_state_pointer_guard:
  EXIT=0
```

Failure/Outcome manifest drift needs no task. Do not regenerate a green
manifest merely because this follow-up runs.

### OLF-1 — Refresh and minimally ratchet repository artifact lifecycle

The checked-in
`tools/checks/manifests/repository_artifact_lifecycle_v0.json` is stale. The
current inventory reports drift; default `--check` only warns and exits 0,
while `--check --strict` exits 1. The existing `docs_slim_001` guard invokes
the check, but that guard is not connected to the normal PR gate.

Scope:

1. Regenerate the checked-in lifecycle manifest from the current repository.
2. Preserve the existing warning-mode local behavior unless the chosen gate
   explicitly passes `--strict`.
3. Add one PR-only Python check to the existing minimal gate. Do not add a
   push trigger, `dev_gate`, pre-commit hook, cargo build, LLVM setup, or a
   second workflow.
4. Prove deliberate drift fails the exact PR command, not only the standalone
   strict command.

Acceptance:

```bash
python3 tools/docs/repository_artifact_lifecycle_inventory.py --write
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check --strict
git diff --check
```

Expected result: the checked-in manifest is current, and the PR gate fails on
deliberate manifest drift without adding a new push-time cost.

### OLF-2 — Reconcile 3511 fixture names and test wiring

The 3511 card lists sixteen descriptive fixture names, but the executable
Rust-side names differ. Five `hako_mem_free`-related names are discoverable
from the implementation history/inventory; the
`refresh_function_extern_call_routes_records_hako_mem_free_route` test lives in
`src/mir/extern_call_route_plan/tests/hako_mem.rs`, while that test tree is not
currently wired from the parent module. A filtered cargo test can therefore
return success without running that orphan test.

Decision:

- Keep the descriptive 3511 fixture labels as contract evidence labels.
- Add an explicit mapping to the real executable test IDs rather than renaming
  the semantic contract vocabulary.
- Wire the existing orphan test tree if the parent test boundary accepts it;
  otherwise add the narrowest equivalent collection guard.
- Require `--list` evidence before accepting a filtered test run.

Scope:

1. Choose one contract: either rename the card entries to real executable test
   IDs or explicitly classify them as fixture/guard contract labels.
2. Confirm whether `tests/mod.rs` is intentionally dormant. If not, wire the
   test module through the existing parent test boundary.
3. Make the acceptance command prove that the intended tests are collected,
   not merely that a filter exits 0.
4. Keep the hako_mem_free semantic owner, backend support, and non-claims
   unchanged.
5. Add a machine-readable evidence table for all sixteen descriptive labels:
   `ExecutableTest`, `GuardAssertion`, or `ContractLabelOnly`, with the exact
   real test ID/guard path for every executable claim.
6. Wire `src/mir/extern_call_route_plan/tests/mod.rs` from
   `extern_call_route_plan.rs` under `#[cfg(test)]` unless a narrower existing
   test owner is proven. The currently orphaned
   `refresh_function_extern_call_routes_records_hako_mem_free_route` must be
   visible in `--list` evidence.
7. Change the 3511 card status from `Active implementation task` only after
   the evidence table, collection proof, and existing guards are green. Use a
   closed status that distinguishes semantic completion from evidence repair.

Acceptance:

```bash
cargo test -q --lib hako_mem_free -- --list
cargo test -q --lib hako_mem_free
```

Expected result: every claimed executable test is listed and run, or the card
clearly labels non-executable names as contract fixtures. No zero-test
filtered command is accepted as evidence.

Additional acceptance:

```text
collected hako_mem_free tests >= 5
orphan refresh route test appears in --list output
all sixteen labels have one explicit evidence classification
3511 card no longer says Active implementation task
3511 semantic/runtime claims remain unchanged
```

## Ordering

1. Active SnapshotAlgebraV0/S3 lane reaches a clean commit boundary.
2. OLF-1 manifest refresh and one PR-only strict gate.
3. OLF-2 evidence mapping, orphan test wiring, and 3511 status repair.

Do not mix OLF-1 and OLF-2 with compiler snapshot implementation commits.

## Non-claims

```text
3457_fact_plan_boundary_owner_changed = 0
failure_outcome_semantic_owner_changed = 0
3511_semantic_contract_changed = 0
global_Unit_runtime_carrier = 0
source_selfhost_claim = 0
push_ci_cost_increased = 0
```

## Stop boundary

If OLF-1 requires more than one CI workflow or if OLF-2 requires changing the
3511 semantic contract rather than its evidence mapping, stop and return to
design consultation.
