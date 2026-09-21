---
Status: design_stop__2026-09-22__StaticResultAuthorityMissing
Task: MIR-CALL-PARSER-STATIC-RESULT-AUTHORITY-D1
Date: 2026-09-22
Parent: mir-call-parser-publication-result-family-d0-2026-09-22.md
Implementation permission: false; design the missing source-owned static-call result contract
NextCard: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
---

# Parser static-call result authority D1

## Six-line brief

```text
Decision: keep parser publication stopped until every target-only row has a
  source-owned result proof and either a physical result handoff or a typed
  pre-effect terminal.
Source authority + canonical issuer: the declaration/result catalogs and the
  resolver-issued source call/return relations, co-sealed by one new or
  explicitly extended static-call result owner selected by this D1.
Non-authority: MIR ValueId/type inference, declared return type alone,
  target-only filtering, name/ordinal rules, GenericLoop retry, VM, or fallback.
Fail-fast boundary: exact caller/site/target/package brand, callee body proof,
  result representation, effect/ABI relation, and one physical consumer or
  typed terminal for every admitted row.
Smallest next slice: design the finite six-class owner contract and choose the
  first implementation-ready family; no code or new receipt is authorized.
Non-claims: publication, caller switch, old-edge deletion, warning cleanup,
  and acceptance beyond the named static-result boundary.
```

## Reopen evidence from D0

The merged parser census is finite: 57 target-only rows, 18 callers, 31
caller-to-target pairs, 10 rows under `LoopBody`, and five callers with a loop
row. The result-disposition partition is:

| disposition | rows | missing relation |
| --- | ---: | --- |
| `Unavailable(UnknownExpression)` | 19 | complete source expression/return proof for the callee |
| `Unavailable(StaticCallTargetAuthorityUnavailable)` | 20 | exact nested caller/site/target authority |
| `Unavailable(StaticCallResultUnavailable)` | 7 | callee result disposition and call-site result row |
| `Unavailable(KnownNonI64Return)` | 8 | exact representation and physical non-I64 consumer |
| `Unavailable(RecursiveDependency)` | 2 | recursive-result authority or a typed terminal retaining identity |
| `Unavailable(UnsupportedStatementKind)` | 1 | supported body shape or typed pre-effect terminal |

The selected `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3`
rows are later `ExactI64` rows. The same caller also contains target-only
`ParserDeclarationBox.parse_or_null/3` and `RuneContractBox.invalid_placement_tag/1`
rows, so a selected `starts_with` handoff cannot claim whole-package coverage.

The source-shape read gives one useful subpartition without issuing a result
claim. The eight `KnownNonI64Return` rows contain six String-shaped targets
(`ParserDeclarationBox._profile/1`, `GrammarContractProjection.status/2`,
`GrammarContractProjection.stable_reject_tag/2`,
`ParserDeclarationBox._profile_name/1`, `RuneContractBox._extract_name/1`,
and `RuneContractBox._extract_entry_arg0/1`) and two Bool-shaped targets
(`StringHelpers.is_space/1` and `StringHelpers.is_digit/1`). The two
`RecursiveDependency` rows are the self-recursive `ParserStringUtilsBox.i2s/1`
and `StringHelpers.int_to_str/1`. The single `UnsupportedStatementKind` row is
`StringHelpers.starts_with/3`. These are source-shape candidates only: a
declared or visually inferred type cannot cross the D1 authority boundary.

The physical GlobalCall planner does have a `Bool -> ScalarI64` projection,
and the language reference contains historical Boolean-as-integer examples.
Neither is a source-owned callable result proof for this lane. D1 must name a
semantic Bool/result relation before that physical projection can be consumed;
otherwise Bool would be silently coerced into the existing `ExactI64` family.

## Existing-owner comparison

| owner | what it can issue | why it cannot close this D1 |
| --- | --- | --- |
| `VerifiedSameModuleCallableResultCatalogV1` | `ExactI64`, `ExactNominalBox`, and typed `Unavailable` per declaration | records a disposition but does not preserve a consumable source call row for the target-only sites |
| `VerifiedSourceResultProductV1` | source `I64`/`String` operations and static/dynamic/absent route observations | source-level scaffold has no production caller/site/target/result/effect publication consumer; declared return type is not body proof |
| `VerifiedStaticCallResultPublicationOwnerV1` | one-shot physical handoff for an existing general call row or exact-i64 projection | no-general-row projection admits only `ExactI64`; `TargetOnly` retains only target identity and drops the six reason classes |
| `CoreMethod` result table | bound receiver core-method contracts | does not cover same-module static targets |

The missing chain is therefore explicit:

```text
callee body proof
  -> exact source caller/site/target result product
  -> representation + effect/ABI relation
  -> physical publication or typed pre-effect terminal
```

No existing owner currently seals all four links for the finite inventory.
The `ExactNominalBox` enum is evidence of a catalog representation, not proof
that a nominal/String result can enter the LoopBreak physical consumer.

## Bounded design work

1. Keep the six disposition classes separate and retain exact caller/site/target
   identity and package brand in every proposed product.
2. Define which source relation can prove a callee body result without using a
   declared type alone, MIR inference, or an AST/name rescan.
3. For the six String-shaped and two Bool-shaped candidates, identify the
   resolver-issued body/result rows and the distinct physical representation
   contract; do not merge Bool into I64 by convention or by reading the
   GlobalCall/MIR projection.
4. Decide whether the canonical issuer is a new static-call result owner or an
   explicit extension of an existing source-result owner; do not create a
   second publication bridge.
5. For each admitted class, name the physical consumer and representation/
   effect/ABI fields. For every non-admitted class, name a typed terminal that
   rejects before Builder effects while retaining the source identity.
6. Add duplicate, foreign-brand, missing-result, recursive, unsupported-body,
   residual, and one-shot obligations to the design. Do not implement them in
   this card.
7. If no single bounded owner can cover a finite family, close that family as
   `NoSafeSlice` with an observable reopen trigger rather than widening
   `ExactI64` or filtering target-only rows.

## Exit and queue

This card exits only with one owner/consumer decision for a finite family or a
named `NoSafeSlice` with its exact reopening condition. Until then, I3 task 4
publication, I3 task 5 caller switch, and R0 task 6 old-edge deletion remain
queued. The warning cohort stays closed at I147 (`unused_imports=17`);
`dead_code` is owner debt to resume after the semantic sequence.

## Non-claims

No resolver admission, new semantic receipt, source-to-MIR publication,
caller switch, VM/compatibility change, fallback, legacy retirement, or
warning sweep is claimed by this design card.
