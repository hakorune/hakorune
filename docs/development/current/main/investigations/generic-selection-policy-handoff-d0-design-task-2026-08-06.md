# GENERIC-SELECTION-POLICY-HANDOFF-D0

Status: design accepted; implementation remains caller-zero and separately taskized.

## Decision target

Define the smallest source-authoritative handoff from the closed
`VerifiedGenericCandidateEnvelopeV1` witness to the existing Generic G0
policy row and common selector. This is a new shallow boundary, not another
D4 suffix and not a production promotion.

## Premise

The candidate envelope currently seals a resolver lease plus typed
Carrier/Condition/Step/BodyEffect/Coverage-Exit evidence. The existing G0
policy requires a different, richer `VerifiedGenericTypedSourceBundleG0`:
parameter/result header, four literal roles, outer/inner Condition and Step
roles, tail binding, complete source coverage, and the numeric/progression
substrate. Casting or wrapping the two products after the fact would permit
AST reread, owner/site re-pairing, or duplicated policy logic.

## Required design

One resolver/source projector must co-seal a move-only product, tentatively
named `VerifiedGenericG0PolicyHandoffV1`, containing:

```text
source-unit/function brand
+ resolver-owned source lease
+ existing typed G0 role bundle and BindingRefs
+ explicit numeric target/projection
+ exact return-expression/ PostLoopRead relation
+ complete loop forest/frame/coverage evidence
```

Policy mode/profile/coverage may be attached by the outer policy owner, but
must not cause a second AST read or name-based role pairing. The handoff is
consumed once by the existing Generic G0 policy observation, then the already
landed common admission assembler and selector may be used unchanged.

## Hard boundaries

Do not add a production caller, Generic demand, Recipe/JoinSig, Builder/MIR,
new selector, `NoCandidate`, retry/fallback, or legacy deletion. Do not make
the current test-only envelope look production-ready: its fixture source
view lacks a source-unit brand, and its Coverage-Exit proof does not yet bind
the exact return expression to the lease role. Those are explicit design
requirements, not implementation guesses.

## Worker-reviewed decision

The sole issuer is the existing compiler-side Generic G0 source projector,
because it is the only owner that can read the exact
`ResolvedFunctionLoweringInputV1` and issue the structural, source-type,
numeric, and return-ABI products together. The product name is fixed to
`VerifiedGenericG0PolicyHandoffV1`; do not add a separate
`GENERIC-SELECTION-CANDIDATE-HANDOFF-D0` row or a wrapper around the old
candidate envelope.

The handoff must mint one opaque `GenericG0SourceBrandV1` containing the
source-unit/function, origin, source kind, root site, and frame identity. It
also seals every role `BindingRef` and the exact post-loop return-expression
relation internally. Caller-supplied sites, names, or a second AST read are
not accepted. The current `FunctionSyntaxViewV1`-based envelope cannot be
promoted because it lacks this brand; it remains a test witness only.

The policy owner may attach mode/profile/coverage metadata, but it consumes
the handoff by value and must retain/move the complete proof into the
candidate observation. It may not re-resolve roles or discard the brand
before the existing observation → Ready assembler → selector chain.

## Acceptance before implementation

- worker review confirms one co-seal issuer and one-shot move API;
- natural typed G0 fixture is paired with the envelope by the issuer;
- foreign identical-AST, shadow/role mismatch, and return-binding mismatch
  counterexamples are rejected before policy publication;
- policy consumes the sealed handoff once and does not re-resolve source;
- the handoff survives into the Generic candidate observation without a
  second source product or a bare-bundle downgrade;
- production selector, demand, Recipe, Builder, MIR, and legacy callers stay
  at zero;
- when a future implementation cell is opened, its same commit must update
  `docs/reference/**`, exact design mirrors, taskboard/current pointers, and
  module READMEs.

The implementation cell is
`docs/development/current/main/investigations/generic-selection-policy-handoff-i0-r0-implementation-task-2026-08-06.md`.
