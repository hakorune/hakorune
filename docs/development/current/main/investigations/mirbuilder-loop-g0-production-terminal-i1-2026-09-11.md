Status: closed__Implementation__GenericG0ProductionTerminalHardening__2026-09-11
Task: LOOP-G0-PRODUCTION-TERMINAL-I1-HARDENING-R0
Date: 2026-09-11
Priority: repair the selected I1 acceptance gaps before source-to-exe publication
Parent: mirbuilder-loop-g0-canonical-issuer-i0-2026-09-11
NextCard: LOOP-G0-SOURCE-TO-EXE-PUBLICATION-D0
---

# Generic G0 production terminal I1

## Six-line brief

```text
Decision: accept one bounded I1 hardening implementation; the production spine remains unchanged while existing reject products retain typed identity and the selected real-path invariants gain mutation evidence.
Source authority + canonical issuer: CanonicalGenericG0PlanV1 and its source parent; the I1 lowerer may borrow existing Generic physical admission products but may not reissue source facts.
Non-authority: generic_g0_physical_emitter_session test helper, route_loop, MIR/name/arity inference, direct publication, retry, and legacy Generic routes.
Fail-fast boundary: source-parent/cohort/admission rejects retain typed stage identity through the canonical boundary; computed-left and missing-carrier mutations reject before draft publication; no debug-string collapse or fallback.
Smallest next slice: promote existing reject enums into a typed canonical boundary, replace the tautological computed-left test with a `prepare_recursive_after_v1` integration test, and add a producer/dispatch carrier-absence mutation test.
Non-claims: no all-family Loop cutover, backend parity, source-to-exe success, old-edge deletion, or whole-MIRBuilder completion.
```

## Authorized implementation cells

1. The existing `CanonicalGenericG0PlanV1`/source-parent handoff and one
   Generic-only physical admission owner. Reuse existing admission and common
   segment/layout owners; do not create a second source parent or physical
   identity issuer.
2. One production sibling under
   `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/` that consumes
   the co-sealed source products and returns the existing `MirFunction` draft
   shape. The test-only `generic_g0_physical_emitter_session` is not promoted.
3. The existing package `consume`, Single collector, completion, manifest
   drain, finalization, postprocess, external commit, and `publish_once`
   callers only. No new token, continuation, manifest, or publication route.
4. Focused positive/negative tests and one reusable guard proving the
   no-test-session/no-route-loop/no-second-owner boundary, plus this README and
   the canonical Loop reference.

If the lowerer cannot consume the co-sealed source products without
reconstructing an effect, ABI, entry lane, or physical identity, return to a
design stop and name the missing issuer; do not add a default or fallback.

Existing separate follow-ups remain open and are not silently folded into I1:
the LocalSSA failure cache, measurement-off compile cost, and view re-scan;
the G0 production connection itself is also not claimed by I0.

## Prior implementation evidence (retained, hardening does not erase it)

I1 is implemented within the authorized cells. The production compiler now
switches the marked Generic G0 plan into one source-bound cutover, consumes the
existing Generic physical admission in a production lowerer, and returns via
the existing Single collect/complete/drain, finalization, postprocess,
external-commit, and `publish_once` lifecycle. The old
`generic_g0_physical_emitter_session` remains `cfg(test)` only.

The source header keeps logical symbol `generic_g0/2`; the declared-instance
fixture physically contains three `i64` lanes (receiver, `i`, `j`). The
source-owned storage-lane projection supplies the explicit physical arity to
the existing Single collector and drain manifest. No MIR/name/arity inference,
route-loop re-entry, direct publication, fallback, retry, or second owner was
introduced.

Prior implementation evidence:

```text
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test -j1 generic_g0 --lib
72 passed; 0 failed
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo check -j2
passed (existing warning baseline only)
bash tools/checks/rust_mirbuilder_generic_g0_production_terminal_i1_guard.sh
passed
bash tools/checks/current_state_pointer_guard.sh
passed
git diff --check
passed
source sizes: generic_lowerer.rs 215, resolved_generic_g0_cutover.rs 78,
source_bound_package.rs 704, source_bound_package_generic_g0.rs 89,
generic_g0_capability_tests.rs 126
```

The positive test asserts one published function with logical name `/2` and
three physical parameter lanes. The negative test drops the prepared external
commit product and verifies that the builder has no current module/function/
entry state. These facts remain valid; the hardening evidence below closes the
reopened I1 acceptance only. Source-to-exe acceptance, backend parity,
all-family Loop activation, and legacy retirement remain unclaimed.

## I1 hardening acceptance (closed 2026-09-11)

This is one bounded correctness slice over the existing I1 owners. It adds no
semantic authority, new route, or backend. The typed boundary is a canonical
Generic G0 stage enum that owns existing reject values; it is not a second
source or physical authority. The selected cells are:

1. `resolved_generic_g0_cutover.rs` must preserve the existing typed reject
   identity for source-parent, cohort, admission, and canonical lowerer
   failures. A canonical error may wrap an existing typed error, but must not
   replace it with `format!("{error:?}")`.
2. The computed-condition-left test must build the actual completed dispatch,
   call `prepare_recursive_after_v1`, and prove that a computed left operand
   is accepted without requiring the left operand itself to be a `Read`.
3. A Generic producer/dispatch mutation that removes the carrier `Read`
   relation must reject at its named admission boundary before the lowerer can
   publish a draft. The lowerer is not granted a second source-fact issuer.

The guard must inspect the production admission edge as well as the existing
no-test-session/no-route-loop/no-second-owner boundary. Header-read variants,
`take_tail` panic cleanup, HashMap-order stabilization, dead producer removal,
and source-size cleanup remain separate latent/cleanup findings and are not
silently folded into this slice.

Hardening evidence:

```text
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test -j2 generic_g0
74 passed; 0 failed
bash tools/checks/rust_mirbuilder_generic_g0_production_terminal_i1_guard.sh
passed
bash tools/checks/current_state_pointer_guard.sh
passed
git diff --check
passed
```

The computed-left case now mutates the sealed test product to use a
`BinaryI64` result as the outer CompareI64 left operand, completes the actual
dispatch, and calls `prepare_recursive_after_v1`; it does not assert a helper
predicate in isolation. The carrier mutation drops one source entry after
descriptor/control preparation and is rejected as `EntryCoverageMismatch` by
admission before the lowerer receives a draft. The cutover maps the existing
source-parent, cohort, admission, lowerer, lifecycle, and commit errors through
one typed Generic G0 boundary; no `bridge_error` or debug-string stage bridge
remains. Current bounded source sizes include
`resolved_generic_g0_cutover.rs` 170,
`generic_g0_physical_operation_cohort/emitter_admission.rs` 492, and
`source_bound_package.rs` 716 lines; all remain below the 800-line hard stop.

NextCard: LOOP-G0-SOURCE-TO-EXE-PUBLICATION-D0

## Fixed terminal mapping

```text
CanonicalGenericG0PlanV1
  -> one Generic physical admission/lowerer
  -> LoweredCanonicalPlanV1::Single
  -> existing collect -> complete -> prepare_drain -> drain
  -> finalization -> postprocess -> external commit -> publish_once
```

The next card is source-to-exe acceptance only after this lowerer has both
positive draft/collection evidence and zero-publication rejection evidence.
