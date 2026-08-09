---
Status: accepted design — I0 selected
Date: 2026-08-09
Row: `GENERAL-STATIC-CALL-EXACT-BOX-RESULT-I1-D0`
Blocks: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1`
Mode: BoxCount / one exact result-representation family
---

# GENERAL-STATIC-CALL-EXACT-BOX-RESULT-I1-D0

## Decision

The current parser R1 source shape is valid. The compiler result authority is
too narrow: `callable_result_representation` seals only exact I64 results, so a
same-module static call whose complete body returns one constructed nominal Box
is published as `Unknown`. A later BodyManagedState GenericLoop correctly
rejects that missing upstream truth.

Generalize the existing result catalog and its sole publication owner with one
new representation:

```text
ExactI64(required parameter ordinals)
ExactNominalBox(box name)
Unavailable(reason)
```

Do not add a second result solver, a second publication owner, a source
annotation, a method/class name table, or a GenericLoop default.

## Exact source boundary

```text
complete same-module static declaration/body
+ exact source call-target catalog
        -> existing monotone callable-result solver
        -> ExactNominalBox only for:
             New(class)
             local/call forwarding of the same exact class
             branch/return merge of the same exact class
        -> exact item-keyed call-result row
        -> existing move-only static publication handoff
        -> successful physical Call receipt
        -> MirType::Box(class) publication exactly once
```

`New(A)` mixed with `New(B)`, Box mixed with I64, unresolved/recursive calls,
unsupported expressions, and conflicting paths remain unavailable. No common
supertype, dynamic Box, or fallback representation is invented.

## Authority

```text
source authority:
  sealed same-module declaration body
  exact source call target

semantic issuer:
  existing VerifiedSameModuleCallableResultCatalogV1

physical publisher:
  existing VerifiedStaticCallResultPublicationOwnerV1
  -> PreparedStaticCallResultPublicationV1

non-authority:
  fixture annotations
  source/method/class name heuristics
  FunctionSignature/MIR inspection
  GenericLoop carrier preparation
  ValueId numbering
```

The semantic product may retain a nominal Box name because it is obtained from
the exact `New(class)` source expression. Physical projection to
`MirType::Box(class)` happens only after a successful call emission receipt.

## Required implementation slice

1. add one semantic result-representation enum owned by
   `callable_result_representation`;
2. generalize expression/local/branch/return facts without creating a second
   solver;
3. issue `ExactNominalBox` from exact `ASTNode::New` and propagate identical
   nominal Box facts through same-module static calls;
4. retain representation in the existing call row and move-only publication
   handoff;
5. make the existing publication commit project I64 or nominal Box exactly
   once;
6. keep all prior ExactI64 behavior and negative gates unchanged;
7. update the owner README and MIR reference receipt in the same implementation
   commit.

## Acceptance

```text
positive:
  make() { return new ProductV1() }
    -> ExactNominalBox(ProductV1)

  caller forwards make()
    -> exact source-bound call row with ProductV1 result

  successful selected physical call
    -> destination MirType::Box(ProductV1)

  ProductV1 call result assigned/reassigned as BodyManagedState Loop carrier
    -> existing GenericLoop preparation succeeds unchanged

negative:
  New(A) / New(B) return merge -> unavailable conflict
  I64 / Box return merge       -> unavailable conflict
  unknown or recursive result  -> unavailable
  foreign/duplicate/mismatch publication -> existing fail-fast
  failed physical emission     -> no publication
```

Required gates include focused unit tests, a minimal executable exact-Box Loop
carrier canary, and the resumed parser R1 fixture.

## Nonclaims

```text
general type inference
Box subtyping or common-supertype inference
parameter-dependent/generic Box results
instance/provider/dynamic calls
recursive exact-Box inference
Home or lifecycle meaning
GenericLoop acceptance widening
Unknown -> Box/Integer default
retry/fallback
```

## Stop condition

Stop if the implementation needs MIR-body inspection, raw function names,
source annotations, or a second publisher. The exact fact must originate from
the existing source result solver and reach the existing selected physical
publication edge.
