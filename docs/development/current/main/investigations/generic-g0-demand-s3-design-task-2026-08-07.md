# GENERIC-G0-DEMAND-S3-D0

Status: accepted design stop; implementation remains closed.
Date: 2026-08-07
Scope: the one-way handoff from `Selected(Generic)` to the Generic G0
Recipe producer. This task does not open Recipe, JoinSig, Builder, MIR, or a
production caller.

## Decision

`Selected(Generic)` is consumed exactly once into one move-only,
AST-free `VerifiedGenericRecipeDemandG0`. The demand is a source capability,
not a Recipe and not a physical lowering request.

The common selector owns the one canonical
`VerifiedLoopFamilyWindowLeaseV1`. The Generic policy handoff must not issue
or retain a second resolver lease. The handoff issuer receives a borrow of the
canonical lease and may retain only an opaque, private source-brand projection
derived from that exact lease. S3 compares that brand with the selected lease
before sealing the demand. A second resolver lookup, tuple-only re-issuance, or
silent dropping of one lease is forbidden.

```text
resolver exact lookup
  -> one canonical window lease
  -> five-row Ready window
  -> Selected(Generic)
  -> consume Generic candidate/handoff
  -> compare borrowed handoff brand with canonical lease
  -> VerifiedGenericRecipeDemandG0
```

## Authority and retained product

The demand retains exactly these authorities:

```text
canonical window lease             // exactly one
GenericG0 source brand             // derived from that lease, not reissued
VerifiedGenericTypedSourceBundleG0
  - structural loop/body/condition/update/tail sites
  - exact source BindingRef relations
  - source type inventory
  - numeric representation lease
  - exact trivial return ABI
VerifiedGenericG0PostLoopReadV1     // return site/value/BindingRef
profile = G0
mode and coverage                  // selector's canonical values
```

The typed bundle remains the owner of its source sites and BindingRefs. S3
does not copy rows into a second catalog or re-pair them by name. The target
is read from the moved numeric bundle; a third independent target field is not
added. Candidate evidence is checked against selector mode/coverage and is
not promoted to a second authority.

The demand contains no selector enum, AST, `FunctionSyntaxViewV1`,
`RecipeBody`/`RecipeBlock`, route schedule, route ID, Recipe key, JoinSig,
ValueId, PHI, Builder, MIR, retry, fallback, or legacy policy winner.

## Role and provenance contract

S3 does not issue `LoopBindingKeyV1`. It only seals the exact source roles that
S4 will consume once:

```text
outer/inner condition: lhs, rhs, operator, BindingRef
outer/inner update:    statement, target, lhs, rhs, operator, BindingRef
parameters:            i/j header BindingRefs and declarations
loop forest:           root/child owner and frame relation
tail:                  post-loop return site/value/BindingRef
derived child entry:   a typed future Recipe role, never a fabricated AST site
```

The two source bindings (`i`, `j`) and three future carriers
`(L0,i)`, `(L0,j)`, `(L1,j)` are distinct facts. The child-entry carrier is
derived Recipe glue and is not presented as a source statement. Foreign,
shadowed, duplicate, uncovered, or frame/site-mismatched roles are rejected.

## Outcome algebra

```text
Reject:
  selected family is not GenericG0
  selector overlap/failure evidence is supplied as a success input
  handoff brand/lease/owner/origin/source-kind/site/frame mismatch
  forest, return, BindingRef, role, or provenance contradiction
  duplicate or uncovered role

Unresolved:
  selector OutOfWindow
  incomplete/unsealed coverage
  opaque role/type/target capability

NoCandidate:
  never issued by S3; owned only by the M8 all-route closeout
```

No re-selection, alternate family fallback, retry, `.ok()`, or error-to-None
conversion is allowed.

## Required consuming API

The current selector and Generic candidate expose only borrowed getters. The
implementation slice must add narrow consuming methods in their owning
modules, without adding a second selector:

```text
CanonicalLoopFamilySelectionV1::into_parts
CanonicalLoopFamilyCandidateV1::into_generic_g0
VerifiedGenericG0FamilyCandidateV1::into_parts
VerifiedGenericG0PolicyHandoffV1::into_parts
```

The methods move existing sealed products; they do not reconstruct facts or
open new policy. The S3 issuer is the sole consumer of the Generic selected
variant and the sole issuer of `VerifiedGenericRecipeDemandG0`.

## Acceptance evidence for the next implementation row

`GENERIC-G0-DEMAND-S3-I0-R0` is a `cfg(test)` caller-zero witness only. The
natural nested G0 fixture must pass:

```text
resolver -> handoff -> five-row Ready -> Selected(Generic) -> Demand
```

and prove one canonical lease, exact source sites/BindingRefs/tail relation,
mode/profile/coverage parity, AST/source-lifetime freedom, and move-only
single consumption. Negative fixtures cover foreign identical AST/session,
duplicate/mixed lease, root/frame/forest/site/BindingRef/tail mismatch,
selected other family, overlap/out-of-window evidence, incomplete coverage,
and opaque target/role. Each maps to the algebra above.

The implementation commit must update the exact `docs/reference/**` receipt,
Generic/Loop SSOTs, module READMEs, workstream, `CURRENT_STATE.toml`, and
current mirrors. Public language activation remains zero. Every source and
guard file remains below 800 lines; the task/investigation document remains
below 1000 lines.

## Ordered next rows

```text
GENERIC-G0-DEMAND-S3-D0       this design stop (accepted)
GENERIC-G0-DEMAND-S3-I0-R0   caller-zero consuming witness
GENERIC-G0-RECIPE-S4-D0      Recipe/JoinSig/Core/After ownership design
GENERIC-G0-RECIPE-S4-I0-R0   deterministic caller-zero Recipe producer
```

No deeper suffix is authorized. Production selection, physical cutover,
M8 all-route proof, selfhost parity, and legacy deletion remain later rows.
