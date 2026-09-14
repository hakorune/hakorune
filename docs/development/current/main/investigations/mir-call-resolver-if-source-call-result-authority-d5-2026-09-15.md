---
Status: closed__NoSafeSlice__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-CALL-RESULT-AUTHORITY-D5
Date: 2026-09-15
Priority: define source-backed String call results and nullable/String Bool facts
Parent: mir-call-resolver-if-string-result-profile-d4-2026-09-15.md
NextCard: mir-call-resolver-if-source-result-product-d6-2026-09-15.md
Implementation permission: false until call-result/Bool authority and the expression profile handoff are co-sealed
---

# Expression If source call-result authority design stop

## Six-line brief

```text
Decision: keep expression If admission deferred while one source authority is defined for String-returning calls and String/null comparisons used by the finite five-callable inventory.
Source authority + canonical issuer: the sibling resolved_value_profile transaction consumes exact resolver call/target rows and issues the call result class, manifest/ABI brand, effect policy, and Bool comparison fact.
Non-authority: method names, receiver class alone, Loop-only contracts, callable-result nominal fallback, embedded JSON payloads, MIR ValueId/type inference, VM, defaults, and compatibility fallback.
Fail-fast boundary: missing/foreign call row, target/header drift, missing manifest/ABI relation, unsupported placement, effectful or nullable-unknown result, duplicate site, result-site drift, mixed branch class, and unsealed profile handoff.
Smallest next slice: classify `StringHelpers.int_to_str`, non-Loop `StringBox.length`/`substring`, and `sval != null` into one finite source-result/Bool vocabulary with exact source sites and consumers.
Non-claims: expression-If resolver admission, String physical representation, JoinSig emission, publication, VM, fallback, caller cutover, or legacy-edge retirement.
```

## Required authority fields

Each call-result row must carry:

1. owner and exact call site;
2. receiver and ordered argument sites;
3. canonical target/header identity;
4. manifest/ABI brand and result relation;
5. result class (`I64`, `String`, or an explicit unavailable reason);
6. effect/placement policy, including the non-Loop requirement; and
7. the exact consumer site that reads the result.

The nullable comparison row must additionally carry the exact String operand
site, the exact Null literal site, the comparison operator, and the fact that
the result is source Bool. It must reject an unknown or mixed nullable state;
`null` is not a default String class.

## Bounded inventory

| Source form | Required outcome |
| --- | --- |
| `StringHelpers.int_to_str(dst)` | either a sealed same-module static target plus String result relation, or a named fail-fast reason |
| `StringBox.length` / `substring` outside Loop | a non-Loop manifest-backed target with exact String receiver/result relation, or a named fail-fast reason |
| `sval != null` | a source Bool comparison over one owner-branded nullable String binding and one exact Null literal |
| unknown/mixed/effectful call | reject before the expression-If product is issued |

The authority must consume existing source target/catalog products by brand;
it may not extend the Loop-only contract by silently dropping its placement
guard. The result row is an input to the D4 sibling profile, not a second
expression matcher or a physical MIR receipt.

## Static authority audit

The existing products do not satisfy this boundary:

* `ExactTrivialScalarAbiV1` and the normal-callable package result cohort carry
  only exact `i64` ABI information. They cannot represent a source String
  result.
* `VerifiedCallableResultRepresentationV1` has only `ExactI64` and
  `ExactNominalBox`. The callable-result proof treats a core method with a
  `StringValue` result as `KnownNonI64`, so it intentionally emits no String
  result row. `CoreStringMethod` therefore remains an i64-only observation.
* The generated `StringBox` target issuer already proves the
  `StringSubstring/2` String result relation, but the existing resolver core
  method contract additionally requires Loop membership and placement. A
  sibling non-Loop contract is required; dropping that guard would mix two
  authorities.
* `StringHelpers.int_to_str` has a sealed same-module target identity, but its
  declaration is unannotated and the source-body result proof has no canonical
  String product. The target row alone is not a return-result proof.
* `null` is present in the trivial value vocabulary, while a source-branded
  nullable String binding and a comparison Bool fact are absent. The
  `sval != null` row cannot be admitted by combining a Null literal with an
  inferred String type.
* `ShadowResolverV0::resolve_expr` has no expression-`If` arm. The current
  fallback records `UnsupportedExpression`, so the exact outer If, condition,
  empty-prelude branch wrappers, and tail sites are not yet a source product.

These are independent missing authorities, not implementation failures in the
existing i64 or Loop lanes. No code, fixture, fallback, production switch, or
new semantic receipt is authorized from this card.

## Ordered design and exit

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Source resolver relation | Expression If and call child sites are available as exact AST-free relations; statement If remains separate. |
| 2 | Static String result | `int_to_str` target/header/result/brand relation is either admitted or explicitly rejected. |
| 3 | Non-Loop core call | `length`/`substring` placement and result relation are either admitted or explicitly rejected without Loop widening. |
| 4 | Nullable Bool | `String/null` comparison has one source Bool fact and exact operand coverage. |
| 5 | Handoff | D4 profile consumes these rows with one owner/brand and no MIR inference. |
| 6 | Exit | Retain `NoSafeSlice` and name the missing source-result product owner; open a fast implementation card only after the source rows and physical String representation are co-sealed. |

No code, fixture, fallback, production switch, or new semantic receipt is
authorized from this design-stop card.

## D5 exit

D5 closes as `NoSafeSlice`. The source target/manifest products are reusable,
but no existing owner issues the needed String result class, non-Loop core
method contract, nullable String Bool fact, or expression-If source relation.
The next bounded design card must co-seal those source facts before the D4
profile can admit any of the five rows.
