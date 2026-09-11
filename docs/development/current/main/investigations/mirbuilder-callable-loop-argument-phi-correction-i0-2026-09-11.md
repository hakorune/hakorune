Status: closed__Implementation__CallableLoopArgumentPhiCorrection__2026-09-11
Task: MIR-CALLABLE-LOOP-ARGUMENT-PHI-CORRECTION-I0
Date: 2026-09-11
Priority: correct caller/callee parameter-index validation and strengthen Loop PHI evidence
Parent: mirbuilder-loop-g0-canonical-issuer-i0-2026-09-11
NextCard: LOOP-COMMON-AFTER-CONDITION-RELATION-D0
---

# Callable Loop argument and PHI correction I0

## Six-line brief

```text
Decision: keep the existing Callable Prelude type check but use caller binding index and callee argument ordinal in their own domains, then assert the emitted Add-to-header-PHI backedge relation.
Source authority + canonical issuer: resolver caller binding/source declarations and resolver callee header; the existing Callable Prelude argument issuer remains sole owner.
Non-authority: parameter-count/name guesses, MIR ValueId numbering as ABI proof, recursive_after condition scanning, route_loop, fallback, and a new type/lifecycle layer.
Fail-fast boundary: caller/callee ABI mismatch rejects before Prelude materialization; an emitted Loop whose Add is not the header PHI backedge rejects the unpublished canary session.
Smallest next slice: fix the swapped lookup, make the existing production canary use a second caller parameter passed to a one-argument callee, and add the exact PHI assert.
Non-claims: common After generalization, all Loop shapes, G0 physical lowering, source-to-exe acceptance, backend parity, or whole-MIRBuilder completion.
```

## Authorized cells

1. `src/mir/compiler/callable_single_loop_prelude_arguments.rs`: validate
   caller `param_decls[index]` from the caller binding and callee
   `header.signature().params()[ordinal]` from the call order. Do not add a
   new type or layer.
2. The existing Callable production canary fixture/test: use
   `work(unused: i64, value: i64) -> helper(value)` with a one-argument
   `helper`, and verify one Add destination is the header PHI input from its
   producing backedge block.
3. Update the compiler README and this card only for this contract. Keep the
   common After redesign as a separate worker-audited design stop.

## Acceptance

```text
two-parameter caller -> second binding -> one-parameter callee ordinal 0: pass
caller/callee ABI mismatch: typed reject before physical materialization
Add destination == header PHI input at the Add block: assert
existing late-failure/discard tests: remain green
```

The next design consultation is `LOOP-COMMON-AFTER-CONDITION-RELATION-D0`:
move Callable-specific header-binding requirements to the Callable owner and
make common After consume an explicit condition/branch relation. G0 I1
physical lowering remains queued after this correction and that design stop.

## Implementation evidence

- `callable_single_loop_prelude_arguments.rs` now checks the caller declaration
  with the resolver binding's `Parameter { index }` and the callee header with
  the call argument `ordinal`; the two index domains are not interchangeable.
- The existing production canary uses a two-parameter caller and passes its
  second binding to a one-parameter callee, which is the regression shape for
  the swapped lookup. It also asserts the exact `(Add block, Add dst)` pair is
  present in the header induction PHI inputs.
- `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test -j1
  callable_production_canary --lib`: 3 passed, 0 failed.
- Earlier same-slice gates remain green: `generic_g0 --lib` 71 passed,
  `cargo check -j2`, pointer guard, and `git diff --check`. Source-size checks
  remain below the 760-line split trigger and 800-line hard stop.
- The real-app record is corrected separately: 3/11 passed including the
  unsupported-boundary probe, seven are named baseline boundaries, and the
  NewBox failure is an unavailable FFI-library environment boundary rather
  than evidence of a compiler regression.

This implementation row is closed without claiming common After
generalization, G0 physical lowering, source-to-exe acceptance, or
whole-MirBuilder completion.
