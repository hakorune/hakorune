# Static CurrentOwner one-caller execution S0

Status: S0 closed; app frontier remains open
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

Source preparation may place the callee in the transport candidate set to
retain its body use proof. It must still stage the incoming call as
`SourceStatic`; the existing selected Static site set records demand, not
readiness. The post-signature finisher alone supplies executable actuals and
route permission. The original forwarded-actual issuer reads the caller's
draft through `source_definition_for`, since the real `size_to_bin` caller is
still source-only and an independently executable fixture caller may be in
the final partition. Neither partition is a second source authority.

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
3. An independently executable same-shape caller fixture and the existing
   physical JSON/ABI family validate the tagged call and I64 Normal/Fault
   result. The unchanged mimalloc-lite EXE probe records its first stop
   before and after; `size_to_bin` has other unsupported calls, so this
   prerequisite alone need not move the app stop.
4. Required guard and regressions pass; new reds are classified, and selected
   code, tests, and owner contract changes are committed and pushed.

The D0 at the Related path owns the Decision and evidence. `size_to_bin`'s
two-caller cohort, `bin_size` condition calls, Heap-to-Page object outgoing,
and whole MirBuilder completion remain outside this S0.

## Closeout evidence

- The unchanged `size_class_box.hako` source issues the single original
  `size_to_bin -> normalize_size` row. Focused original-source and executable
  same-shape caller tests passed; changing the ordered argument or removing
  callee Completion rejects the finished actual and packet. The Static packet
  family passed 11/11; the final `current_owner_` family passed 36/36, including
  source-only condition contexts and the physical JSON test.
- Physical JSON from the same-shape caller has one tagged Static invoke, an
  I64 Normal result and a Fault edge. With the current C shim built from this
  checkout, EXE generation succeeded and the executable exited 0. Passing
  `true` to `size_to_bin` was rejected by direct MIR verification as
  `mir/invoke/call-argument-type-drift` at the caller.
- The unchanged `apps/mimalloc-lite/main.hako` still stops at
  `ordinary-new/borrowed-entry/source-only-object-actuals` on Heap-to-Page;
  this prerequisite does not move the whole-app first stop.
- Package filter: 997 passed, 3 failed. The three names exactly match the
  pre-selection baseline recorded at `6f90996ea3` in the explicit EXE Static
  selection S0 card: birth receiver non-escape, Main static child port, and
  qualified Map argument capability. No new package red observed.
- The focused pointer guard and `git diff --check` passed. This S0 adds a
  checked CurrentOwner exception to the existing packet owner; no legacy edge
  is exclusively superseded by this one-caller prerequisite, so this closeout
  claims no old-path retirement or whole-app advancement.
