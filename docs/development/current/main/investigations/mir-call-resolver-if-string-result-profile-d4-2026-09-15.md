---
Status: closed__NoSafeSlice__2026-09-15
Task: MIR-CALL-RESOLVER-IF-STRING-RESULT-PROFILE-D4
Date: 2026-09-15
Priority: co-seal the source-result profile and finite String operation vocabulary
Parent: mir-call-resolver-if-string-result-authority-d3-2026-09-15.md
NextCard: mir-call-resolver-if-source-call-result-authority-d5-2026-09-15.md
Implementation permission: false until the profile, expression JoinSig, and resolved-lowering consumer are co-sealed
---

# Expression If source-result profile design stop

## Six-line brief

```text
Decision: keep resolver expression-If rows deferred. The sibling profile schema, finite operation vocabulary, and logical result port are defined, but implementation remains NoSafeSlice until source call-result/Bool authorities and the String physical consumer are co-sealed.
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
6. one exact consumer relation: `Return.Value`, `LocalInitializer(index)`,
   assignment `Rhs`, or a nested expression parent plus its canonical child
   role (`BinaryLeft`/`BinaryRight` and the equivalent existing source role);
   and
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

## Five-callable operation rows

The source census fixes the first implementation inventory. These rows are
source observations, not yet admitted products:

| Callable | Operation row | Consumer / unresolved relation |
| --- | --- | --- |
| `PatternUtilBox.find_local_bool_before` | comparison Bool condition; `I64Literal(1)` and `I64Literal(0)` tails | `Return.Value`; no String dependency |
| `JsonNumberCanonicalBox.canonicalize_f64` | String literal, String binding, and String-preserving `+` tails | `Return.Value` rows plus `out4` initializer `BinaryRight`; `.length()`/substring conditions still need call-result facts |
| `JsonFragNormalizerBox._normalize_instructions_array` | String literal `"?"` versus `StringHelpers.int_to_str(dst)` | `LocalInitializer(dstr)`; static-call String result relation is missing |
| `JsonFragNormalizerBox._canonicalize_f64_str` | String literal, String binding, and String-preserving `+` tails | `Return.Value` rows plus `out4` initializer `BinaryRight`; `.length()`/`.substring()` conditions need non-Loop authority |
| `LowerMethodArrayGetSetBox.try_lower` | `sval != null` Bool condition; String concat versus String literal tails | `LocalInitializer(json)`; nullable/String comparison fact is missing; embedded JSON `i64` text is payload only |

Nested ternary branches therefore need the exact parent relation, not only a
top-level return or initializer label. The existing `ExprChildRoleV1` and
`FunctionSourceViewV1` are the relation vocabulary; the profile must carry the
sealed relation instead of reconstructing it from AST nesting.

## Audit: why D4 cannot open a fast implementation card

The source resolver has no expression-If arm in `ShadowResolverV0::resolve_expr`;
the current fallback reports `UnsupportedExpression`, and
`body_shape_resolver` records the node as neutral `Other`. The passive
expression-source inventory has no If row. Extending this path requires an
owner-branded source relation for the outer If, condition, branch bodies, and
tails before the profile can issue a product.

The existing value profile has no String representation, stops on String
literals, and only derives numeric binary operations. The callable-result
catalog is exact-i64/nominal-Box and does not issue String results. The
`StringHelpers.int_to_str` call has a source target identity but no canonical
String-result row, while general `StringBox.length`/`substring` contracts are
Loop-only. The `sval != null` condition has no source Bool fact for a nullable
String value.

The natural physical entry is `CanonicalTrivialSsaLowererV1::lower_expr`, but
it has no expression-If arm or String result port. Existing If materialization
is statement-only and binds a `BindingRefV1`; it cannot be widened to hide a
result-carrying expression JoinSig. A future consumer may project the sealed
class explicitly (`I64 -> MirType::Integer`, `String -> MirType::String`), but
it must not infer the class from an incoming MIR value.

## Ordered audit and exit

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Profile schema | All required sites/relations have one owner-branded field and duplicate/foreign checks. |
| 2 | Five-row mapping | Every deferred callable receives `Admit` or a named fail-fast reason for each tail operation. |
| 3 | Consumer handoff | One `resolved_lowering` result port consumes the sealed product without type inference. |
| 4 | Negative matrix | Missing/foreign/unknown/mixed/effectful/duplicate/drift/prelude/consumer cases reject before admission. |
| 5 | Exit | Retain `NoSafeSlice`; the profile shape is bounded, but source call-result/Bool authorities and a String-capable consumer are not co-sealed. |

No code, fixture, fallback, or new semantic receipt is authorized while this
card remains in `design_stop`.

## D4 exit

D4 closes as `NoSafeSlice`, with the schema and finite five-row inventory
recorded. The next bounded design card selects one source call-result and
nullable-Bool authority for `StringHelpers.int_to_str`, required non-Loop
StringBox calls, and `String/null` comparisons. It must also state how that
authority is handed to the sibling profile without reopening names or MIR
inference. No resolver admission or production switch is claimed.
