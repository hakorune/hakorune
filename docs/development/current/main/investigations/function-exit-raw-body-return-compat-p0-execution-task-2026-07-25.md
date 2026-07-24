# Function-exit Raw body-return compatibility P0

Decision authority: `FUNCTION-EXIT-SEMANTICS-prime-r1`

Status: parked/reserved. This is not the current executable row.

## Purpose

Preserve the historical Legacy App any-statement tail observation as bounded
migration evidence without promoting it to Hakorune function/Main semantics.

The accepted canonical contract is:

```text
ordinary function/method = F1 ExplicitReturnOnly
source Main.main          = ordinary function semantics
Script root               = ScriptLastExpressionOrUnit
```

Legacy happens to connect the last ValueId returned by statement lowering to
the physical root Return. That observation may be useful while comparing old
and new pipelines, but it is not a semantic policy.

## Evidence shape

The only allowed compatibility vocabulary is test-only:

```rust
struct LegacyObservationOracleV1 {
    observed_tail: LegacyObservedTailV1,
    provenance: LegacyObservationProvenanceV1,
}

struct LegacyBodyReturnParityWitnessV1 {
    oracle: LegacyObservationOracleV1,
    raw_observation: RawBodyReturnObservationV1,
}
```

These are conceptual test-fixture products. If implemented, they remain under
`#[cfg(test)]` or in focused `_p0.rs` fixture code and have no production
constructor or consumer.

They may record:

```text
Legacy observed tail ValueId relation
tail statement kind
Legacy signature/Return relation
Raw observation under the tested route
normalized parity or intentional canonical difference
```

They may not select or alter:

```text
RawRootBodyRecipeV1
RawRootExitPolicyV1
function/Main semantics
physical root lowering
postprocess or public adaptation
runtime mode
normal/public ingress
JSON or Program(JSON v0)
executor, selfhost, fastmem, or CUT0
```

There is no executable `RawLegacyAppAnyStatementTailParityV1` profile and no
caller-selectable compatibility policy. The historical relation is named only
as evidence:

```text
historical observation =
  LegacyAnyStatementValueOrUnit

evidence owner =
  LegacyObservationOracleV1
```

## Required parity rows

The disconnected proof may cover:

```text
empty App
Expr tail
Print tail
Local tail
Assignment tail
CompoundAssignment tail
helper + scalar Main
same-compiler reuse
```

Each row must distinguish:

```text
Legacy observed behavior
canonical F1 expected behavior
whether equality is expected
whether the difference is an intentional semantic correction
```

A mismatch with canonical F1 is evidence, not a reason to repair Raw through
Legacy fallback or to weaken the normative contract.

## Structural contract

```text
LegacyObservationOracleV1 production constructor       = 0
LegacyObservationOracleV1 public constructor           = 0
LegacyObservationOracleV1 test-only constructor        <= 1
LegacyBodyReturnParityWitnessV1 production consumer    = 0

AppLastValueOrVoid canonical producer                  = 0
AppLastValueOrVoid public production consumer          = 0
LegacyAnyStatementValueOrUnit implicit selector        = 0
executable compatibility policy/profile                = 0

recipe/BODY/postprocess/public-adapter repair           = 0
Legacy fallback                                        = 0
normal-entry/JSON/executor/CUT0 consumer                = 0
```

## Sunset and proof budget

```text
ceremony_tier =
  T2; explicit compatibility-observation boundary with a positive,
  sunset-bound proof delta

sunset_id =
  RAW-BODY-RETURN-COMPAT-SUNSET-001

owner of the retirement decision =
  FUNCTION-EXIT-COMPAT-RETIRE0

sunset_row =
  RAW-BODY-RETURN-COMPAT-RETIRE0-S0

proof_inventory_before =
  AppFixedVoid production implementation;
  historical Legacy App tail observations;
  no AppLastValueOrVoid implementation

new_proofs =
  at most one test-only Legacy observation oracle
  + one focused parity witness family
  + one exact consumer/sunset guard

retired_or_merged_proofs =
  none until RAW-BODY-RETURN-COMPAT-RETIRE0-S0

net_proof_delta =
  bounded test-only evidence only; production semantic delta zero

sunset_budget =
  one test-only oracle, one parity witness family, one guard

retire_when =
  normative function-exit SSOT accepted
  + ordinary function/method F1 conformance green
  + Main explicit-return/Unit-fallthrough green
  + ScriptLastExpressionOrUnit green
  + physical entry/result projection accepted
  + VM/MIR-interpreter/LLVM process-projection parity green
  + normal-entry profile explicitly selected
  + LegacyAnyStatementValue canonical consumer zero
  + LegacyAnyStatementValue public production consumer zero
  + AppLastValueOrVoid symbol and caller zero

budget_repayment_evidence =
  test-only oracle consumer zero
  + parity witness consumer zero
  + compatibility files/guard deleted
  + AppLastValueOrVoid and LegacyAnyStatementValue symbols zero
```

Promotion is forbidden under accepted F1. Reopening
`LegacyAnyStatementValueOrUnit` as language semantics requires a new normative
language decision; parity evidence cannot promote it.

## Sunset namespace separation

`RAW-BODY-RETURN-COMPAT-SUNSET-001` owns only the function/Main tail-result
compatibility evidence described in this card.

`RAW-PUBLICATION-SUNSET-001` owns only the old Raw
publication/finalization/run_raw/ledger-root-only evidence chain.

Neither sunset closes, aliases, or satisfies the other. They may share a
normal-entry-selection prerequisite, but their retirement owners, zero-caller
evidence, and deletion rows remain independent.

## Non-claims

```text
FUNCTION-EXIT-F1-RETURN0-S0 implementation
executable compatibility profile
canonical or public App last-value semantics
normal-entry cutover
JSON / Program(JSON v0)
executor / selfhost / fastmem
old Raw publication-chain retirement
public adapter repair
CUT0
```
