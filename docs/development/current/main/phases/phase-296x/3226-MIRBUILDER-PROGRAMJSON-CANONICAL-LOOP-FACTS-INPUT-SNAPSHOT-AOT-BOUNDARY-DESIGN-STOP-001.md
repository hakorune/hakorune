# 3226 - MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-AOT-BOUNDARY-DESIGN-STOP-001

Status: design-stop

Decision: CONSULTATION_REQUIRED

## Stop Reason

The next Layer4 task is still correct:

```text
ProgramJSON verified_recipe -> CanonicalLoopFacts input snapshot
```

But the first implementation attempt hit an AOT publication boundary choice.
The `.hako` traversal can be written, but the public callable boundary is not
yet settled:

```text
string summary boundary:
  fails AOT with module_generic_prepass_failed for build_summary/1

MapBox snapshot boundary:
  requires a stable map_handle publication contract for this owner
```

This is not a RecipeMatcher design problem. It is an AOT/MIR route publication
boundary problem for the next snapshot owner.

## WIP

The attempted implementation is intentionally not committed because its fast
gate is not green.

```text
stash:
  wip/3226-canonical-loop-facts-input-snapshot-aot-boundary-design-stop
```

## Candidate Decisions

```text
A_MAPBOX_SNAPSHOT_PUBLICATION_BRIDGE
   Extend/declare a narrow map_handle publication contract for
   ProgramJsonCanonicalLoopFactsInputSnapshotV1, like the RecipeBodies runtime
   publication bridge.

B_COMPLEX_STRING_SUMMARY_AOT_ROUTE
   Teach AOT to emit this complex same-module string-return helper instead of
   routing it through the failing module-generic prepass.

C_VM_ONLY_TRAVERSAL_GATE
   Rejected for now. It would prove traversal shape but not the AOT boundary
   needed by this lane.
```

## Recommended Default

```text
A. MapBox snapshot publication bridge
```

Reason: this follows the 3223 RecipeBodies publication precedent and keeps the
snapshot as structured data. It should be scoped narrowly to this owner and
must not claim RecipeMatcher execution.

## Forbidden Until Resolved

```text
RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
runtime fallback
Source Selfhost claim
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_canonical_loop_facts_input_snapshot_aot_boundary_design_stop_guard.sh
```
