# Closed App static physical selection S0

Status: closed S0
Date: 2026-10-10
Scope: MIRBUILDER-CLOSED-APP-STATIC-SELECTION-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-gate1-object-guarded-outgoing-transport-d0-2026-10-10.md

## Decision

Source authority + canonical issuer: the parser-sealed App Main relation,
the source-backed complete callable catalog, and its one
`VerifiedWholeSourceStaticCallTargetInventoryV1`. The inventory issues an
owned same-brand zero-incoming static projection before its target subset is
consumed. Do not rescan source. Keep every declaration, result, and original
call row; project only the physical selected callable membership.

Selected behavior: only a closed App EXE may omit a non-Main static method
whose complete MethodCall observation has no gap and whose same-selector
calls all have exact *other* canonical static targets. Any exact incoming,
same-selector non-exact call, explicit visibility/ABI annotation, or
non-App compilation retains the method. `Public` alone is metadata, but
retaining it is conservative. The parser App Main and its required children
remain selected. Match exact canonical keys, not method names.

Bare `helper()` syntax cannot call a static box method on this route:
`function_call_preflight_route` returns `bare-static-method-retired` for both
current-owner and unique other-owner candidates before child lowering. Its
existing focused rejection test passes. Thus the MethodCall inventory covers
the executable static-call vocabulary used by this projection; restoring a
bare static execution route would require revisiting this completeness proof.

Production caller and replaced responsibility: the source-backed catalog's
all-declarations selected identities currently feed the selected batch map,
root work plan, and `NormalCallableSemanticPackagePortV1::complete()`. Replace
that physical membership with one same-brand projection at package issue.
The work plan must lower precisely those selected non-Main static methods;
the package ledger and port must agree. Do not filter final JSON after
lowering, change the declaration catalog, or drop source call evidence.

Fail-fast: foreign brand, incomplete observation, uncertain incoming,
inconsistent selected map/work plan/ledger, and final physical membership
drift retain the method or stop with a typed error. The existing
`class_id` Static seed veto is unchanged; reconciling the now-unselected
`accepts -> class_id` source row is the following slice.

Acceptance: focused positive on the unchanged mimalloc-lite imported source
shows `LayoutBox.accepts/1` absent from physical selection while its source
declaration and `accepts -> class_id` observation remain. Preserve all 15
Heap callers and the exact `Heap -> class_id` target. Focused negatives cover
an added `LayoutBox.accepts` call, an observation gap, an unresolved
same-selector call, foreign brand, and non-App/public-ABI cases. Existing
selected work-plan and package `complete()` tests must pass. Probe the
unchanged app and compare its actual first stop; this S0 does not promise an
EXE or make opaque actual transport executable.

Non-claims: no condition packet, no borrowed Static transport, no Page edge,
no Loop Mul, no old-edge retirement, and no MirBuilder goal completion.

## Evidence

- Source projection positive/negative 2/2; source-backed App catalog 4/4;
  selected Script parity 1/1; bare static rejection 1/1.
- Physical selected-membership test 1/1. The real imported mimalloc-lite
  source test passes 1/1: `LayoutBox.accepts/1` is absent physically while its
  declaration and original `accepts -> class_id` row remain, and all 15 Heap
  callers remain in the source inventory.
- Quick CLI build passes. Unchanged mimalloc-lite EXE probe still exits 1 at
  `ordinary-new/borrowed-entry/source-only-object-actuals`; this is the
  separate guarded outgoing transport frontier, not S0 acceptance.
- One attempted exact-name test filter selected zero tests and was not counted;
  rerunning the full filter selected and passed the bare static rejection.
