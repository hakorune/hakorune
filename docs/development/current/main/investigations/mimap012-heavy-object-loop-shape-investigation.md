---
Status: Active
Date: 2026-05-15
Scope: MIMAP-012 heavy object-loop shape investigation and follow-up row split.
Related:
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/design/mimalloc-object-lifecycle-queue-ssot.md
  - docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md
---

# MIMAP-012 Heavy Object-Loop Shape Investigation

## Summary

MIMAP-012 landed with a bounded object-backed lifecycle queue proof. The green
shape intentionally avoids the heavier dynamic loop scan:

```hako
loop i < page_count {
    local page = pages.get(i)
    if me.last_selected_page == null {
        me.considerPage(page)
    }
    i += 1
}
```

The heavy prototype combined too many compiler stressors at once. Treat it as a
MIR acceptance series, not as a MIMAP-013 blocker. The first split row
(`MIR-ROW-A`) is now green through LLVM/EXE after `MIR-ROW-A-FIX`.

## Green corridors already proven

```text
mimalloc-page-queue-proof:
  object retention plus loop scan, but simpler active/free selection

vm-lim-object-queue-identity-probe:
  minimal ArrayBox.push/get -> method receiver identity

mimalloc-object-lifecycle-queue-proof:
  lifecycle object route through bounded fixed slots
```

## Heavy shape stressors

```text
1. loop body contains a guarded structured branch
2. pages.get(i) creates dynamic object receiver flow
3. object local is passed into helper call considerPage(page)
4. nullable object field stores the selected page
5. dense proof reads many queue/page fields after the hard shape
```

Likely issue family:

```text
MIR planner / emit object-flow shape pressure
```

This is not VM-only. If the route cannot produce MIR JSON, or if LLVM lowering
cannot execute the generated route, EXE is affected too. The investigation must
therefore split acceptance into:

```text
MIR JSON acceptance:
  can the compiler produce the route?

LLVM/EXE acceptance:
  can the primary MIMAP backend execute the route?

VM smoke:
  diagnostic-only object-heavy reference check
```

Not the primary hypothesis:

```text
VM object identity bug
```

VM remains diagnostic-only for this object-heavy route. LLVM/EXE remains the
primary acceptance backend.

## Split rows

### MIR-INV-MIMAP012

Pin this investigation and keep the failed prototype shape out of MIMAP-013.

Acceptance:

```text
taskboard references this investigation
MIMAP-013 stop line says not to combine all heavy stressors
```

### MIR-ROW-A

Minimal fixture:

```hako
loop i < page_count {
    local page = pages.get(i)
    if selected < 0 {
        selected = i
    }
    i += 1
}
```

Purpose:

```text
prove loop + if guard + pages.get(i) with scalar result only
```

Acceptance:

```text
MIR JSON guard passes
LLVM/EXE guard passes
VM is optional diagnostic only
```

Current evidence:

```text
script:
  tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh

MIR JSON:
  pass
  [mimap012-row-a-mir-json] ok

LLVM/EXE:
  pass
  [k2-wide-mimap012-object-loop-row-a-exe] ok
```

Interpretation:

```text
Dynamic pages.get(i) preserves enough shape for MIR JSON. The landed fix now
recovers local collection element origin facts from ArrayBox push/set writes, so
the returned object is known as HakoAllocPageModel when lowering page.freeCount().
```

Landed compiler row:

```text
MIR-ROW-A-FIX:
  preserve or recover typed user-box receiver facts after dynamic ArrayBox.get(i)
  so page.freeCount() lowers as HakoAllocPageModel.freeCount/0
```

Validated guard:

```text
bash tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh
```

### MIR-ROW-B

Add helper call:

```hako
me.considerPage(page)
```

Constraint:

```text
selected state remains scalar
no nullable object field yet
```

Acceptance:

```text
MIR JSON guard passes
LLVM/EXE guard passes
VM is optional diagnostic only
```

Status:

```text
ready sidecar after MIR-ROW-A
```

### MIR-ROW-C

Add nullable object field publication:

```hako
me.last_selected_page = page
```

Purpose:

```text
prove object field selection and return path
```

Acceptance:

```text
MIR JSON guard passes
LLVM/EXE guard passes
VM is optional diagnostic only
```

### MIR-ROW-D

Restore dense proof reads after object selection is already green.

Purpose:

```text
separate proof-read volume from object-flow acceptance
```

Acceptance:

```text
MIR JSON guard passes
LLVM/EXE guard passes
VM is optional diagnostic only
```

## Stop line

```text
Do not reintroduce dynamic scan + helper call + nullable object field +
dense proof reads in one MIMAP row.
```

## MIMAP-013 policy

MIMAP-013 should compose the allocator facade over the bounded-slot
object-backed lifecycle queue proven by MIMAP-012. It should not wait for the
dynamic object-loop acceptance series.
