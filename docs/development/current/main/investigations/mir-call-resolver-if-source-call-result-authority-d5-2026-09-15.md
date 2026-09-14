---
Status: open__design_stop__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-CALL-RESULT-AUTHORITY-D5
Date: 2026-09-15
Priority: define source-backed String call results and nullable/String Bool facts
Parent: mir-call-resolver-if-string-result-profile-d4-2026-09-15.md
NextCard: TBD after source authority and physical representation audit
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

## Ordered design and exit

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Source resolver relation | Expression If and call child sites are available as exact AST-free relations; statement If remains separate. |
| 2 | Static String result | `int_to_str` target/header/result/brand relation is either admitted or explicitly rejected. |
| 3 | Non-Loop core call | `length`/`substring` placement and result relation are either admitted or explicitly rejected without Loop widening. |
| 4 | Nullable Bool | `String/null` comparison has one source Bool fact and exact operand coverage. |
| 5 | Handoff | D4 profile consumes these rows with one owner/brand and no MIR inference. |
| 6 | Exit | Open a fast implementation card only if all required rows and the physical String representation are co-sealed; otherwise name the remaining owner. |

No code, fixture, fallback, production switch, or new semantic receipt is
authorized while D5 remains in `design_stop`.
