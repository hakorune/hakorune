---
Status: closed__design__2026-09-15
Task: MIR-CALL-RESOLVER-IF-STRING-RESULT-AUTHORITY-D3
Date: 2026-09-15
Priority: define one canonical source authority for String results used by expression If joins
Parent: mir-call-resolver-if-value-join-physical-consumer-d2-2026-09-15.md
NextCard: mir-call-resolver-if-string-result-profile-d4-2026-09-15.md
Implementation permission: false until source result authority and its consumer handoff are co-sealed
---

# Expression If String-result authority design stop

## Six-line brief

```text
Decision: keep expression If admission deferred, and place the source-backed String-result authority in a sibling profile under `resolved_value_profile` rather than widening the declaration-level callable-result catalog.
Source authority + canonical issuer: the sibling profile issues exact source site, operation, target/manifest relation, and `I64 | String` class before physical lowering; existing source-call catalogs are consumed only for their sealed target/result relations.
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

## Static census of the five deferred callables

The source read gives this bounded split; it does not itself issue a result
class:

| Callable | Expression-If result shape | Additional source facts required |
| --- | --- | --- |
| `PatternUtilBox.find_local_bool_before` | six comparison rows selecting `I64` (`1`/`0`) | Bool comparison condition and two I64 literal tails |
| `JsonNumberCanonicalBox.canonicalize_f64` | String-preserving concat or String binding read | `String` bindings, `+` operands, and `.length()` condition results |
| `JsonFragNormalizerBox._normalize_instructions_array` | String-preserving concat/binding rows | `StringHelpers.int_to_str`, `.length()`, and source call result relations |
| `JsonFragNormalizerBox._canonicalize_f64_str` | String-preserving concat/binding rows | `.length()`, `.substring()`, and exact String result contracts |
| `LowerMethodArrayGetSetBox.try_lower` | String result on both ternary branches, even where embedded JSON says `i64` | `sval != null` Bool fact and String concat/literal facts |

The census confirms why an I64-only profile would leave four rows deferred.
The embedded JSON type text in the last row is payload data and cannot change
the outer expression result class. Any call result used by a condition or
branch must carry its source site, target/manifest relation, result class, and
effect policy from a canonical issuer.

## Expression-If shape and authority decision

The parser shape is bounded to `ASTNode::If { else_body: Some(..) }` used as an
expression. Both branches are exactly one `BlockExpr` with an empty prelude and
one tail expression. The product therefore records the outer If site, condition
site, both wrapper sites, both tail sites, and one consumer relation
(`Return.Value`, `LocalInitializer(index)`, or assignment `Rhs`). A missing
`else_body`, a non-empty prelude, or a branch without one tail is a named
rejection; statement-position `if` remains the existing statement owner.

The result join is parametric over the source class, not an i64-only special
case. The condition has a separate source-backed Bool fact. The two branch
tails must carry the same class (`I64` with `I64`, or `String` with `String`),
and the result-carrying JoinSig exposes that class at the one outer consumer.
The physical layer must consume this sealed class and never infer it from a
MIR `ValueId` or embedded JSON payload text.

The canonical issuer is a sibling source-result profile under
`resolved_value_profile`. `callable_result_representation` remains the
declaration-level exact-i64/nominal-Box catalog: extending it would still not
issue literal, concat, binding, or branch-tail facts, and would mix a general
expression authority into a call-result catalog. The sibling profile may
consume its sealed source target/manifest/result relation for a static call,
but it owns the complete expression operation vocabulary and branch-class
join. Receiver-only String facts, Loop-only Core method contracts, names,
helpers tables, and MIR inference remain non-authorities.

## Existing evidence and missing owner

`src/mir/resolved_semantics/expression_source.rs` already carries literal and
binary source relations, and `body_shape.rs` carries neutral method-call child
relations. Neither issues value classes or result targets. The current
`resolved_value_profile` issues only `TrivialRepresentationV1` (without
String), while `source_core_receiver` proves a receiver rather than a call
result. `resolver_core_method_callable_contract.rs` proves only the Loop
placement contract. These products may be inputs to the new authority, but
none alone can claim the String result.

The selected sibling profile under `resolved_value_profile` must preserve the
statement If recipe, callable-result fallback, and compatibility lanes
unchanged. D4 owns the profile's concrete schema and its non-Loop call-result
relation; no existing Loop contract is widened to satisfy this row.

## Bounded investigation order

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Inventory | Every deferred String branch and call result has an exact owner/site row. |
| 2 | Operation vocabulary | Literal, concat, binding, static call, and required core call forms are explicitly classified. |
| 3 | Authority choice | One issuer and one owner-branded product are selected; receiver-only/Loop-only products are excluded. |
| 4 | Rejection matrix | Unknown, mixed, effectful, foreign, duplicate, and result-site drift fail before admission. |
| 5 | Handoff | The product fields needed by the D2 expression JoinSig and resolved-lowering consumer are named. |
| 6 | Exit decision | Authority is selected, but implementation remains stopped until the sibling profile schema, operation vocabulary, and consumer handoff are co-sealed in D4. |

## Nonclaims

This card does not authorize changes to `ShadowResolverV0`, the trivial
representation enum, statement `IfRecipeV1`, Loop Core method contracts,
StringBox, VM, compatibility fallback, publication, or production routing.

## D3 exit

The finite census and parser-shape audit are complete. D3 closes with a named
authority choice, not resolver admission: the next bounded card defines the
sibling profile's source operation rows, String call-result relation, rejection
matrix, and handoff fields. No semantic receipt or code change is authorized
until that product and the D2 physical consumer are co-sealed.
