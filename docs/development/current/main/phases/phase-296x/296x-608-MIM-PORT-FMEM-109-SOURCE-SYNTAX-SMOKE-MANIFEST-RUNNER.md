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

## Migration Rule After This Row

```text
new hako_alloc fastmem body:
  add .hako source
  add manifest entry / expected key file
  avoid growing fastmem_source_syntax_smoke.sh by another large bespoke block
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

## Verification

```bash
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```
