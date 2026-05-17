---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-ROW-CADENCE-001 mimalloc / hako_alloc row validation cadence.
Related:
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
  - docs/tools/check-scripts-index.md
  - tools/checks/run_proof_app.sh
  - tools/checks/run_row_guard.sh
  - tools/checks/lib/pure_first_exe_guard.sh
---

# Mimalloc Row Validation Cadence

## Decision

Mimalloc / hako_alloc rows stay proof-first, but each row should use the
smallest sufficient validation level.

The rule is:

```text
validate the row's new contract directly
validate touched upstream/downstream compatibility narrowly
avoid broad gates unless the row changes a broad default or closeout boundary
```

This SSOT does not weaken existing guards. It prevents every new row from
guessing that all previous proof apps, allocator-wide gates, and full closeout
guards must run.

## Validation Levels

| Level | Use for | Required evidence | Do not run by default |
| --- | --- | --- | --- |
| `L0 pointer` | planning rows, docs-only selection rows | `bash tools/checks/current_state_pointer_guard.sh`, `git diff --check` | proof apps, EXE guards |
| `L1 syntax` | source cleanup that uses an already-live language feature | feature-specific syntax guard plus focused touched-row guard | allocator-wide, unrelated row guards |
| `L2 proof` | new `.hako` allocator behavior with VM/MIR/EXE proof | one dedicated proof app through `run_proof_app.sh --only <id>` and its public guard | broad closeout pack |
| `L3 compatibility` | behavior row touches an owner used by earlier rows | only the prior rows whose owner/report contract was touched | all historical guards |
| `L4 closeout` | explicit closeout rows | manifest-backed closeout guard via `run_row_guard.sh --only <id>` | behavior changes |
| `L5 broad` | provider/host replacement, dev_gate defaults, or release checkpoints | `dev_gate.sh quick`, allocator-wide, or larger suites as named by the card | daily row work |

## Row-Type Rules

### Planning Rows

Planning rows select exactly one next row. They do not add allocator behavior.

Evidence:

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Planning rows must not require proof apps unless they also changed a durable
policy SSOT.

### Allocator Behavior Rows

Behavior rows add or change one contract in one owner family.

Evidence:

```text
bash tools/checks/run_proof_app.sh --only <ROW_ID>
bash tools/checks/<public-row-guard>.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The public row guard should own:

- file existence and executable checks;
- owner / card / SSOT / index / manifest wiring;
- VM output for the new proof shape;
- MIR JSON checks for the new contract surface;
- pure-first EXE parity for the proof app;
- stop-line leak checks for the row's forbidden concepts.

### Compatibility Guards

If a behavior row changes a report object, owner field, method return shape, or
shared counter used by older rows, run only the affected older guards.

Examples:

```text
release report changed
  -> run MIMAP-097A release guard
  -> run MIMAP-100A recycle guard if recycle consumes the release report

readiness report changed
  -> run readiness guard
  -> run modeled consume guard
```

Do not run unrelated reclaim, purge, facade, or provider guards for a local
segment-ledger report change.

### Closeout Rows

Closeout rows freeze a group boundary. They should not add allocator behavior.

Evidence:

```text
bash tools/checks/run_row_guard.sh --only <closeout-id>
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The public `k2_wide_*` wrapper may remain stable, but common routing should stay
manifest-backed where the family has already migrated.

### Broad Gates

Broad gates are explicit, not reflexive.

Use them when:

- the card changes `dev_gate.sh`, allocator-wide, or manifest defaults;
- a provider / host allocator replacement boundary is reopened;
- a release checkpoint asks for broad assurance;
- a row touches cross-family runtime substrate.

Do not use broad gates to compensate for an underspecified row guard. Improve
the row guard instead.

## Stop-Line Guarding

Rows should keep stop-line leak checks focused on the forbidden concepts for
that row. A segment-ledger row should not grow a generic repository-wide grep
bundle.

Common forbidden families for current segment-ledger rows:

```text
real segment allocation/free
free-list mutation
arena backing allocation
raw pointer residence
segment-map pointer membership / lookup
atomic bitmap execution
page-source / OSVM execution
thread scheduling / worker spawning
source-level concurrency feature changes
provider activation / hooks / host allocator replacement
backend .inc app/name matchers
```

## Current Practice

For the current segment allocation modeled lane:

```text
planning row:
  L0

behavior row touching segment_allocation_modeled_ledger_box.hako:
  L2 dedicated proof/guard
  L3 only for MIMAP-097A / MIMAP-100A if release report or recycle-visible
  behavior changes

cleanup row using C199 compound assignment:
  L1 C199 guard
  L3 touched segment guards only

closeout row:
  L4 manifest-backed closeout guard
```

This is the expected cadence until a card explicitly reopens real segment
execution, allocator-wide defaults, provider activation, or host allocator
replacement.
