---
Status: open__design_stop__2026-09-15
Task: MIR-CALL-RESOLVER-IF-STRING-RESULT-PROFILE-D4
Date: 2026-09-15
Priority: co-seal the source-result profile and finite String operation vocabulary
Parent: mir-call-resolver-if-string-result-authority-d3-2026-09-15.md
NextCard: TBD after profile and consumer handoff audit
Implementation permission: false until the profile, expression JoinSig, and resolved-lowering consumer are co-sealed
---

# Expression If source-result profile design stop

## Six-line brief

```text
Decision: keep resolver expression-If rows deferred while defining one sibling source-result profile for the finite I64/String branch inventory.
Source authority + canonical issuer: resolved_value_profile issues the owner-branded profile from exact source relations; source-call catalogs supply only sealed target/manifest/result rows.
Non-authority: AST/name rescans, receiver-only facts, Loop-only contracts, callable-result nominal fallback, helpers tables, MIR ValueId/type inference, VM, defaults, and compatibility fallback.
Fail-fast boundary: foreign owner/site, missing target or manifest, unsupported operation, unknown or mixed branch class, effectful call, duplicate row, result-site drift, non-empty branch prelude, missing tail, or unsealed consumer handoff.
Smallest next slice: define the operation rows and result fact for the five deferred callables, then co-seal one expression-If product with its result-carrying JoinSig and one resolved-lowering output port.
Non-claims: resolver admission, statement If changes, f64/nested/prelude branches, publication, VM, fallback, caller cutover, or legacy-edge retirement.
```

## Required product fields

The D4 profile must carry, in one owner-branded product:

1. function/body identity and the exact outer expression-If site;
2. condition site plus a source-backed Bool fact;
3. exactly one `then` and one `else` empty-prelude `BlockExpr`, with exact tail
   sites;
4. one source operation row for each tail, including its value class and any
   call target/manifest/result relation;
5. equal branch class `I64` or `String`;
6. one outer consumer relation (`Return.Value`, `LocalInitializer(index)`, or
   assignment `Rhs`); and
7. a result-carrying JoinSig whose output is that consumer value.

The product must not expose a MIR identity or let the consumer re-read the AST.

## Finite operation vocabulary

The five-callable census is the complete initial inventory:

| Operation row | Required evidence | Initial class |
| --- | --- | --- |
| literal | exact literal site and source literal kind | `I64` or `String` |
| String-preserving `+` | exact lhs/rhs sites, both String facts, and source operator | `String` |
| binding read | owner-branded binding and reaching definition | inherited `I64` or `String` |
| `StringHelpers.int_to_str` | exact static-call site, target/header, result relation, manifest/ABI brand | `String` |
| required StringBox call | exact call/receiver/argument/result sites, target schema, effect policy, and non-Loop placement | only if a non-Loop authority is present |

An operation is deferred when any relation is inferred from a name, receiver
class alone, MIR value, embedded JSON payload, or Loop-only contract. The two
branch classes must match before the JoinSig is issued.

## Ordered audit and exit

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Profile schema | All required sites/relations have one owner-branded field and duplicate/foreign checks. |
| 2 | Five-row mapping | Every deferred callable receives `Admit` or a named fail-fast reason for each tail operation. |
| 3 | Consumer handoff | One `resolved_lowering` result port consumes the sealed product without type inference. |
| 4 | Negative matrix | Missing/foreign/unknown/mixed/effectful/duplicate/drift/prelude/consumer cases reject before admission. |
| 5 | Exit | Open a fast implementation card only when the profile, JoinSig, and consumer are finite and co-sealed; otherwise retain `NoSafeSlice`. |

No code, fixture, fallback, or new semantic receipt is authorized while this
card remains in `design_stop`.
