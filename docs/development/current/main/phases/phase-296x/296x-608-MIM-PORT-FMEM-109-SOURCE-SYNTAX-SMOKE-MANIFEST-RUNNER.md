---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-109.
Related:
  - docs/development/current/main/phases/phase-296x/296x-607-MIM-PORT-FMEM-108-PAGEMAPRELEASE-POINTER-LOOKUP-PREFLIGHT-SELECTION.md
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/fastmem_check_smoke.sh
---

# 296x-608 MIM-PORT-FMEM-109 Source-Syntax Smoke Manifest Runner

## Purpose

Reduce the per-body overhead of hako_alloc FastMemory migration.

The next hako_alloc implementation work should add compact fixture/expectation
entries instead of appending another large handwritten block to
`fastmem_source_syntax_smoke.sh`.

## Chosen Mode

```text
BoxShape
```

## Required Boundary

```text
do not change FastMemory semantics
do not add a new MemOp family
do not open PageMapRelease pointer lookup yet
do not weaken existing report/check expectations
do not remove the existing smoke coverage in this row
```

## Implementation Shape

```text
Add a manifest-driven source fixture runner for new hako_alloc fastmem bodies:

fixture manifest:
  name
  source path
  inventory expectations
  MIR inventory expectations
  producer profiles
  producer report expectations

runner:
  emits AST JSON
  emits MIR JSON
  runs fastmem-capability-inventory on AST/MIR
  runs fastmem-mir-to-llvm-producer-report for listed profiles
  runs fastmem-check for listed reports
```

## 608A Status

```text
seed manifest runner landed:
  tools/hako_check/fastmem_source_manifest_runner.py
  tools/hako_check/fastmem_source_syntax_smoke.sh now calls it for the landed
  same/remote free publish body seed fixture
  legacy shell assertions stay in place
```

## 608B Status

```text
new-body manifest routing landed:
  same/remote free publish body now runs through the manifest runner instead of
  a bespoke shell block
  new hako_alloc fastmem bodies must enter through manifest entries
  legacy shell coverage remains for the rest of the smoke, but the new body is
  no longer duplicated there
```

## 608C Status

```text
legacy group migration landed:
  local-free alloc body now runs through the manifest runner instead of a
  bespoke shell block
  the smoke keeps the legacy shell coverage for the remaining groups
  608D now focuses on splitting expectation data out of the shell
```

## 608D Status

```text
expectation data split landed:
  manifest runner now covers free-head read, local-free-head failure, and
  local-free-memop failure expectations through small data files
  the shell lost the corresponding bespoke blocks
  608E can now retire more migrated legacy blocks
```

## 608E Status

```text
bespoke-block retirement landed:
  the remaining migrated fixture rows now run through the manifest runner
  the smoke keeps the legacy shell wrapper only for the still-unmigrated groups
  source-syntax smoke cleanup is complete enough to return to PageMapRelease
  implementation work
```

## Migration Rule After This Row

```text
new hako_alloc fastmem body:
  add .hako source
  add manifest entry / expected key file
  avoid growing fastmem_source_syntax_smoke.sh by another large bespoke block

if a new body is not yet MIR-inventoryable:
  carry the failure as a manifest row
  keep AST inventory fixed
  record the MIR inventory failure as kv output instead of a bespoke shell block
```

## Non-Goals

```text
full rewrite of fastmem_source_syntax_smoke.sh
retiring existing legacy shell assertions
moving fastmem_check_smoke.sh
PageMapRelease pointer lookup implementation
product activation / hook / global allocator / winner behavior
```

## Acceptance Sketch

```text
manifest runner can execute at least one landed hako_alloc source fixture
manifest runner invokes AST inventory, MIR inventory, producer report, and
fastmem-check through shared code
existing fastmem_source_syntax_smoke.sh remains green
no existing expected key is weakened
CURRENT_STATE points at the next hako_alloc implementation row after closeout
```

## Full Cleanup Backlog

This row only creates the entry path. The full cleanup is intentionally split
so hako_alloc migration can resume after the new path exists.

```text
608A seed manifest runner:
  add Python helper/runner surface
  run one landed fixture from a compact manifest
  keep all legacy shell assertions intact

608B new-body rule:
  route all newly added hako_alloc fastmem bodies through the manifest runner
  stop appending large bespoke blocks to fastmem_source_syntax_smoke.sh

608C migrate legacy groups gradually:
  layout/table pilots
  owner/runtime pilots
  local_free/free_head bodies
  atomic remote head bodies
  route/body-join/product ladder reports

608D split expectation data:
  move grep-style key expectations into small expected-key files
  keep shell as orchestration only
  keep fastmem-check as the policy gate

608E retire bespoke blocks:
  delete migrated shell blocks only after equivalent manifest coverage is green
  keep a small top-level smoke wrapper for the still-unmigrated groups
  keep failure output short and fixture-scoped
  this row is landed; the lane can return to PageMapRelease implementation work
```

## Anti-Goals For Cleanup

```text
do not do a one-shot rewrite of the 4000+ line smoke
do not weaken exact key expectations while moving them
do not mix PageMapRelease implementation with smoke cleanup
do not move fastmem_check_smoke.sh in the first cleanup row
```

## Verification

```bash
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```
