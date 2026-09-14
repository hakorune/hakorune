---
Status: open__design_stop__2026-09-15
Task: MIR-CALL-RESOLVER-IF-STRING-RESULT-AUTHORITY-D3
Date: 2026-09-15
Priority: define one canonical source authority for String results used by expression If joins
Parent: mir-call-resolver-if-value-join-physical-consumer-d2-2026-09-15.md
NextCard: TBD after String authority and operation vocabulary decision
Implementation permission: false until source result authority and its consumer handoff are co-sealed
---

# Expression If String-result authority design stop

## Six-line brief

```text
Decision: keep expression If admission deferred while selecting one source-backed String-result authority for the finite callable inventory.
Source authority + canonical issuer: one resolver-owned result product must issue exact source site, operation, target/manifest relation, and String class before physical lowering.
Non-authority: receiver-only String proofs, Loop-only Core method contracts, callable-result nominal fallback, Hako names, helpers_pure_value tables, MIR type inference, VM, defaults, and compatibility fallback.
Fail-fast boundary: missing/foreign target, missing manifest relation, unknown or mixed branch class, effectful or unsupported operation, duplicate site, result-site drift, and unsealed owner handoff.
Smallest next slice: census the five deferred callables and classify literal, String-preserving concat, binding read, StringHelpers.int_to_str, and any required StringBox call result using one finite vocabulary.
Non-claims: resolver acceptance, expression JoinSig, physical lowering, publication, f64/nested/prelude branches, VM, caller cutover, or old-edge retirement.
```

## Required finite inventory

The authority decision must cover the actual deferred rows, without silently
dropping the String cases:

| Source family | Required result evidence |
| --- | --- |
| String literal | exact source literal site and String class |
| String-preserving `+` | exact lhs/rhs sites, both String facts, and the source operator |
| String binding read | owner-branded binding and reaching String definition |
| `StringHelpers.int_to_str` | exact source call, callable target/header, result relation, and manifest/ABI brand |
| `StringBox.substring` or other required core call | exact resolver call row, target schema/result relation, effect policy, and non-Loop placement |

The product must reject an operation when any relation is inferred from a
method name, receiver class alone, MIR `ValueId`, or a Loop-only contract.
The five callable inventory is finite; each row receives one of `Admit` or a
named fail-fast reason before the expression If product is widened.

## Existing evidence and missing owner

`src/mir/resolved_semantics/expression_source.rs` already carries literal and
binary source relations, and `body_shape.rs` carries neutral method-call child
relations. Neither issues value classes or result targets. The current
`resolved_value_profile` issues only `TrivialRepresentationV1` (without
String), while `source_core_receiver` proves a receiver rather than a call
result. `resolver_core_method_callable_contract.rs` proves only the Loop
placement contract. These products may be inputs to the new authority, but
none alone can claim the String result.

The D3 exit must name whether the authority is a sibling source-result profile
under `resolved_value_profile` or an existing resolver result owner extended
with an explicitly non-Loop relation. It must preserve the statement If
recipe, callable-result fallback, and compatibility lanes unchanged.

## Bounded investigation order

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Inventory | Every deferred String branch and call result has an exact owner/site row. |
| 2 | Operation vocabulary | Literal, concat, binding, static call, and required core call forms are explicitly classified. |
| 3 | Authority choice | One issuer and one owner-branded product are selected; receiver-only/Loop-only products are excluded. |
| 4 | Rejection matrix | Unknown, mixed, effectful, foreign, duplicate, and result-site drift fail before admission. |
| 5 | Handoff | The product fields needed by the D2 expression JoinSig and resolved-lowering consumer are named. |
| 6 | Exit decision | Open a fast implementation card only if the authority and result vocabulary are finite and co-sealed; otherwise record the missing owner. |

## Nonclaims

This card does not authorize changes to `ShadowResolverV0`, the trivial
representation enum, statement `IfRecipeV1`, Loop Core method contracts,
StringBox, VM, compatibility fallback, publication, or production routing.
