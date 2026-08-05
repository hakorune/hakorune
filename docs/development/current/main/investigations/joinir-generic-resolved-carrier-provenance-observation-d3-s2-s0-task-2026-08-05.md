Status: selected implementation task — cfg(test)-only
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-OBSERVATION0-D3-S2-S0`
ParentCurrentCard: `docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md`
Exception: this compact child is selected because the D3-S2 design stop requires a source-backed observation before any neutral snapshot or opaque selection product; it is not a production handoff.

# Typed provenance observation task

## Change

Add one private, non-Clone `cfg(test)` witness that consumes the existing
parsed natural-Both source through the compiler-side source projector:

```text
parsed source
-> VerifiedResolvedFunctionV1
-> resolver-issued loop forest/frame
-> exact BindingRefV1 source relations
-> observation witness
```

The witness records only function/forest/frame identity, outer-to-inner parent
membership, exact write/read source sites, the associated `BindingRefV1`, and
the strict-ancestor relation. Resolver remains the sole source identity owner;
the compiler-side projector owns AST navigation and exact-site lookup.

## Explicit non-claims

```text
GenericCarrierFactsSnapshotV1 production issuer = 0
Generic LoopBindingKeyV1 issuer              = 0
InvocationSeal / PreflightSeed               = 0
opaque selection input / selector            = 0
CanonicalLoopFacts / labels / route IDs      = 0
ValueId / PHI / Return / Home / debt         = 0
Builder / MIR / Recipe / runtime caller      = 0
```

Do not infer a logical key from `BindingId`, local ordinal, name, AST path,
route, plan `ValueId`, or diagnostic label. A future key relation and neutral
snapshot require a separate design/owner.

## Required evidence

- natural parsed Both source succeeds and preserves exact owner/forest/frame;
- shadowing, foreign owner, foreign forest/frame, missing, and duplicate role
  relations reject before any Builder effect;
- no synthetic source, DTO mutation, failure injection, or runtime result;
- production caller/import census remains zero;
- artifact manifest records `artifact = none`.

## Verification and closeout

Run the focused `generic_d3_s2_s0` test, generic resolved-carrier suite,
`cargo check --lib`, rustfmt, `git diff --check`, TOML/pointer guard, and the
800-line check for every touched Rust/test/check file. Keep the new test below
800 lines. The implementation closeout must update
`src/mir/loop_structural_facts/README.md`, resolved-semantics README, the
Generic stage/reference entry, `docs/development/current/main/CURRENT_STATE.toml`,
`10-Now.md`, and the active workstream in the same commit;
preserve all D3-S2 non-claims. No production cutover is permitted.

## Stop boundary

If implementing the witness requires a Generic logical-key assignment,
neutral snapshot issuer, seed pairing, opaque input, full Return/Home/debt
proof, or a production caller, stop and return to the D3-S2 design card.
