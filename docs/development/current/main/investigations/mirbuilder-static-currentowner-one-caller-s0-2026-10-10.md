# Static CurrentOwner one-caller execution S0

Status: selected implementation
Date: 2026-10-10
Scope: MIRBUILDER-STATIC-CURRENTOWNER-ONE-CALLER-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-closed-cohort-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/README.md

## Selected replacement

Admit the original `SizeClassBox.size_to_bin -> normalize_size(size)`
CurrentOwner Body(0) local initializer from unchanged imported mimalloc-lite
source. Its `normalize_size` selected incoming cohort has exactly one caller.
Replace that call's `SourceStatic`-only actual with a checked executable
borrowed actual, tagged entry, and the existing sole
`CallPacketSourceV1::Static` publication path. Source authority remains
`QualifiedStaticCallClaimIndexV1::issue`, the original incoming inventory,
Home `LocalCallObservationV1`, the selected physical signature and result
Completion. This row does not create a new claim, packet owner, or route.

Retain `SourceStatic` during source preparation. After signature/Completion,
the finisher corroborates the original one-row cohort, `Rc`/site/target/
ordinal/formal, Home ordered argument, checked forwarded I64 input,
exact-I64 result and physical signature. Only a completed row may pass the
generic actual constructor's CurrentOwner exception, borrowed-entry target
loan, publication handoff, and Static packet validation. The existing
tagged carrier enters the callee; numeric payload use checks the I64 tag.
Normal yields I64 and Fault follows the existing call failure edge.

Reject mismatched/foreign original source, target/site/ordinal/brand drift,
missing/duplicate cohort, non-I64 payload, or absent result/Completion before
emission. Other CurrentOwner callers and unsupported condition contexts remain
source-only. No `.hako` rewrite or fallback.

## Acceptance

1. Existing real imported-source test retains the exact one-caller
   `normalize_size` cohort and original checked-input facts.
2. Focused Static actual, entry, packet, result positive and negative cases
   show executable transport only for the completed cohort; changed identity,
   missing completion, and non-I64 payload reject.
3. Existing physical JSON/ABI family validates the tagged call and I64
   Normal/Fault result. The unchanged mimalloc-lite EXE probe records its
   first stop before and after; app progress is claimed only if that stop moves.
4. Required guard and regressions pass; new reds are classified, and selected
   code, tests, and owner contract changes are committed and pushed.

The D0 at the Related path owns the Decision and evidence. `size_to_bin`'s
two-caller cohort, `bin_size` condition calls, Heap-to-Page object outgoing,
and whole MirBuilder completion remain outside this S0.
