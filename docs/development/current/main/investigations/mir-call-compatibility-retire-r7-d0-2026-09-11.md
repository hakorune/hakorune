---
Status: Design stop — owner-unit retirement boundary
Date: 2026-09-11
Decision: MIR-CALL-COMPATIBILITY-RETIRE-R7-D0
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: none selected; existing compatibility callers remain
ReplacementCell: one existing compatibility owner per selected row
---

# MIR-CALL-COMPATIBILITY-RETIRE-R7-D0

## Six-line brief

Decision: Design the R7 retirement as finite owner-unit Stop/Promote/Delete
rows. Do not open aggregate `LegacyCallV0` deletion until every selected
writer, reissuer, reader, re-entry, and environment route has an owner and a
caller-zero proof.

Source authority + canonical issuer: canonical `MirInstruction::Call(MirCall)`
and its typed `Callee` own native meaning. Existing `LegacyCallV0` producers,
reissuers, readers, and compatibility terminals are classified from the
resynchronized observation manifest and their real callers.

Non-authority: lexical counts, `func`, `ValueId::INVALID`, names, generic
reachability, test-only Loop-PHI files, and the observation manifest itself.
They classify evidence but never issue or repair a call target.

Fail-fast boundary: selected native admission rejects mixed legacy input before
artifact creation; explicit compatibility ingress keeps its existing terminal
until a replacement is switched. No writer or shared carrier is deleted while
an in-scope reader or re-entry remains reachable.

Smallest next slice: choose one existing M7-S compatibility owner from the
finite manifest, record its caller set and exclusive delete-set, then decide
Stop/Promote/Delete without changing Call meaning or adding a second resolver.

Non-claims: no aggregate R7 deletion, `func`/`Option<Callee>` removal, JSON-v0
retirement, VM/WASM parity, environment global removal, Loop-PHI production,
OBJ/EXE completion, or performance result.

## Finite boundary and state table

```text
canonical/compatibility MIR ingress + compile profile
  -> LegacyCallV0 constructors/reissuer/readers/egress
  -> selected artifact reject or explicit compatibility terminal
```

Included: the current manifest's 243 production lexical rows across 127 files,
five compile-environment route anchors across four families, and the known
compatibility ingress families. Excluded: canonical typed `Call` definition,
ordinary tests, unrelated runtime/startup environment, runtime hook registry,
future VM/WASM parity, and test-only Loop-PHI residue except as inventory.

| State | Issuer/owner | Pre-effect terminal | Allowed action |
| --- | --- | --- | --- |
| `SelectedNative` | canonical typed call owner | named admission reject on legacy mix | retain typed route |
| `ExplicitCompatibility` | existing compatibility ingress | existing compatibility terminal | Stop or Promote only with caller proof |
| `MechanicalReissuer` | existing ID remapper | preserve IDs or named failure | retain until all callers close |
| `ReaderProjection` | existing serializer/backend projection | explicit compatibility output | replace/delete only at caller-zero |
| `EnvironmentRoute` | existing invocation/profile owner | restore or named failure | move to invocation-owned state in its own row |
| `TestOnlyInventory` | manifest observer | no production terminal | separate test cleanup decision |
| `UnclassifiedOrDrifted` | no authority | design stop | do not infer or delete |

## Acceptance for the next owner-unit row

Before implementation, the selected owner must have one source-backed issuer,
one real production caller set, one named terminal/consumer, and one exclusive
delete-set. The row must preserve the existing compatibility behavior or stop
it before effect, and the same series must remove its old edge after the
replacement or terminal is proven. Positive evidence covers the supported
caller; a one-point mutation negative identifies the owner-specific reject.

R7 aggregate closure requires all in-scope production writers, reissuers,
readers, re-entry paths, and environment routes to be caller-zero, with no
unconsumed replacement product. Until then, `LegacyCallV0` and shared
compatibility carriers remain live. Reopen this design if the manifest drifts,
a new non-test constructor/reissuer appears, a selected native reader consumes
legacy input, or an external caller cannot be assigned to an owner.

## Worker audit (2026-09-11)

The read-only owner audit found the R7 aggregate is not yet safe to open as an
implementation row. The current manifest must first be reconciled to the
selected admission source change (243/127 Legacy, five environment anchors,
four test-only Loop-PHI files, 252 rows). After that, select one owner-unit
Stop/Promote/Delete from the existing M7-S inventory; do not repeat the broad
census or delete the shared schema early.
