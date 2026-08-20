---
Status: accepted implementation — P1A closed; compatibility admission remains parked
Date: 2026-08-21
Decision: MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1A
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: selected-normal Script direct-static bridge and detached physical kernel; no new caller
ReplacementCell: preserve typed bridge/publication errors until one existing outer diagnostic boundary
Classification: BoxShape refactor; no accepted shape, semantic authority, route, or physical owner change
Execution row: MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1A
---

# MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1A

## Six-line brief

Decision: Keep the existing Script direct-static behavior and retain typed
errors through the bridge/publication boundary; stringify only once at the
existing route/session outer edge. Do not convert the whole Builder error API.

Source authority + canonical issuer: the claimed Join/claim ledger owns the
selected row, the canonical target owns route identity, the unified emitter
owns receipt errors, and the source representation/publication owner owns
publication errors. The new error enums only transport those facts.

Non-authority: diagnostic strings, `format!("{error:?}")`, AST names/spans,
`ValueId`/`MirType`, compatibility success, raw fallback, and performance
results cannot classify a route or manufacture a typed error.

Fail-fast boundary: target validation stays before argument descent. After a
claim, every argument, receipt, publication, or claim-completion error is a
terminal candidate failure; it must not return to an ordinary route, retry, or
rollback the claim.

Smallest next slice: add typed bridge/publication errors and map them only at
the existing `member_route`/detached-kernel outer boundary. Keep the ordered
argument driver, unified receipt emitter, claim ledger, loan, and manifest
interfaces unchanged.

Non-claims: no compatibility admission, Brand/constructor cutover, loan or
manifest refactor, raw retirement, `MirInstruction::Call` rewrite, ABI,
backend, performance, or production-switch change.

## Finite classification and error table

| state | authority / issuer | before effects / transition | allowed terminal | fallback |
|---|---|---|---|---|
| `Unavailable` | ingress state owned by the source transport when no semantic ledger exists | bridge is not entered | existing compatibility owner | no bridge error is converted into `Unavailable` |
| `Absent` | exact ScriptRoot source site is available and the claim ledger has no row | bridge is not entered | existing ordinary/static owner | no bridge error is converted into `Absent` |
| `Claimed` | exact Join row and claim ledger | validate canonical static target before ordered argument descent | proceed to argument driver | no name re-resolution |
| `TargetMismatch` | claimed canonical target | reject before child effects | typed bridge error; discard candidate | no ordinary/static retry |
| `ArgumentFailure` | existing ordered argument driver | stop at first child failure after claim | typed bridge error; discard candidate | no re-descent or rollback |
| `UnifiedDisabled` | existing unified receipt emitter | stop before publication when unified calls are disabled | typed bridge error; discard candidate | no legacy/rewrite/BoxCall fallback |
| `Emission` | existing unified receipt emitter | stop when generic Call emission fails | typed bridge error; discard candidate | no alternate emitter or retry |
| `AlternateRoute` | existing unified receipt emitter | reject a rewrite/BoxCall/legacy result as non-generic | typed bridge error; discard candidate | no route substitution |
| `FinalDestinationMissing` | existing unified receipt emitter | stop when the receipt has no final destination | typed bridge error; discard candidate | no `ValueId` inference |
| `RepresentationMismatch` | source-bound result representation | reject before publication write | typed publication error; discard candidate | no `MirType` inference |
| `DuplicatePublication` | publication owner/type context | reject before a second type write | typed publication error; discard candidate | no finalizer repair |
| `ClaimCompletionError` | claim ledger completion state | no successful bridge result is published | typed bridge error; discard candidate | no claim reinsertion/retry |
| `Completed` | existing emitter + publication + claim completion owners | return the one published `ValueId` to the existing completion owner | existing selected-normal completion | no alternate owner |
| `NoSafeSlice` | design boundary when loan/manifest/common-port scope is requested | stop before opening a broader refactor | parked design stop | never widen this row implicitly |

`Unavailable` and `Absent` are ingress outcomes, not bridge errors. A bridge
error must never be collapsed into either state. `NoSafeSlice` is a development
stop, not a runtime disposition.

## Scope and ownership census

P1A may touch only these existing owners:

- `src/mir/builder/calls/script_direct_static_physical_bridge.rs`
- `src/mir/builder/normal_script_direct_static_physical_publication.rs`
- the existing `StaticReceiver` route and detached direct-static kernel only at
  their outer `Display`/String boundary

P1A must not change `RecursiveChildLoweringPortV1`, the raw dispatcher, source
transport, `normal_callable_semantic_loan_port.rs`, constructor manifest/loan,
or any semantic/Recipe/Join issuer. Each named source remains below the 760
line design trigger and below the 800 line hard stop.

## Acceptance and stop line

- unified receipt variants remain distinguishable through the bridge;
- target mismatch remains pre-descent;
- non-ExactI64 and duplicate publication remain distinguishable;
- claimed failures cannot reach ordinary route, retry, or rollback;
- the existing ordered argument driver and sole receipt emitter are each used
  once; no second matcher/driver/emitter is introduced;
- focused bridge/publication/kernel tests, pointer guard, diff check, and the
  reusable classification guard are green;
- `format!("{error:?}")` is present only at the named outer boundary;
- if retaining a typed error requires changing the common recursive-port
  signature or mixing loan/manifest errors, stop as `NoSafeSlice` and open P1B
  separately.

No semantic receipt, source admission, fallback, or production route may be
added by this row. The compatibility cohort census remains an independent
`NoSafeSlice` design stop.

## P1A closeout receipt

- The bridge now transports `ScriptDirectStaticPhysicalBridgeErrorV1` and the
  publication sibling transports `ScriptDirectStaticPublicationErrorV1` until
  the existing `member_route` or detached-kernel string boundary.
- `validate_claimed_target_v1` returns the bridge error type directly; no
  target decision is reconstructed from a diagnostic string.
- Focused evidence is green:
  `CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust --lib
  normal_script_direct_static_physical_publication`,
  `... --lib script_direct_static_physical_bridge`, and
  `bash tools/checks/script_direct_static_target_guard.sh`.
- `cargo check --profile quick -p nyash-rust`, the current-state pointer guard,
  the classification-completeness guard, and `git diff --check` are green.
- The reusable guard now checks the typed bridge/publication variants and the
  named detached-kernel outer boundary. No common port, loan, manifest,
  compatibility route, semantic receipt, or production switch changed.

P1A is a BoxShape refactor only. The next design stop is
`CALLABLE-COMPATIBILITY-COHORT-STATE-CENSUS-D0`; its missing source issuer and
named consumer remain `NoSafeSlice`.
