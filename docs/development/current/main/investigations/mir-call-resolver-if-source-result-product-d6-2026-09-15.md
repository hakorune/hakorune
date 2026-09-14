---
Status: open__design_stop__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-RESULT-PRODUCT-D6
Date: 2026-09-15
Priority: define one source result product for expression-If admission
Parent: mir-call-resolver-if-source-call-result-authority-d5-2026-09-15.md
NextCard: TBD after source result and nullable Bool co-seal
Implementation permission: false until the source relation, result product, and physical String port are co-sealed
---

# Expression If source result product design stop

## Six-line brief

```text
Decision: keep expression If admission stopped until one owner-branded source product carries the outer If relation, source call results, String/null Bool facts, and a parametric I64/String result class.
Source authority + canonical issuer: a sibling resolved_value_profile source-result transaction consumes exact resolver/source-call rows and issues the product; target catalogs and core manifests are borrowed evidence, not issuers.
Non-authority: names, receiver class alone, Loop-only contracts, callable-result i64/nominal fallback, embedded JSON payloads, MIR ValueId/type inference, VM, defaults, and compatibility fallback.
Fail-fast boundary: missing/foreign expression site, non-empty branch prelude, missing tail, target/header/manifest drift, unsupported placement/effect, unknown or mixed branch class, nullable-unknown comparison, duplicate site, and unsealed profile handoff.
Smallest next slice: source-resolve the five finite rows, issue StringHelpers.int_to_str and non-Loop StringBox result relations where proven, issue one source Bool fact for sval != null, and retain named rejection for any row without proof.
Non-claims: physical MIR lowering, JoinSig emission, publication, VM, fallback, caller cutover, or legacy-edge retirement.
```

## Product boundary

The D6 product must carry, under one source owner and one catalog brand:

1. the exact expression `If` site, its condition site, and both empty-prelude
   `BlockExpr` wrappers with their tail sites;
2. one source operation row for every condition and tail call/literal/binding,
   including target/header/manifest evidence where a call is present;
3. a result class `I64` or `String` for each tail and a same-class join result;
4. the exact consumer relation (`Return.Value`, initializer, assignment RHS, or
   nested parent plus `ExprChildRoleV1`); and
5. for `String/null`, the owner-branded String operand, exact Null literal,
   comparison operator, and source Bool fact.

The physical consumer may project `I64 -> MirType::Integer` and
`String -> MirType::String` only after consuming this product. It must not
reconstruct a class from an incoming MIR value.

## Finite source rows

| Row | Required source proof | If proof is absent |
| --- | --- | --- |
| `PatternUtilBox.find_local_bool_before` | Bool comparison and i64 literal tails | reject only the row, without widening the numeric profile |
| `JsonNumberCanonicalBox.canonicalize_f64` | String literals/bindings/concat plus `length`/`substring` conditions | retain a named source-result stop |
| `JsonFragNormalizerBox._normalize_instructions_array` | String literal versus `StringHelpers.int_to_str(dst)` | require a String return product for the static call |
| `JsonFragNormalizerBox._canonicalize_f64_str` | String literals/bindings/concat plus core String calls | require the same source rows as the preceding row |
| `LowerMethodArrayGetSetBox.try_lower` | nullable String operand, exact Null literal, and source Bool `!=` | reject unknown/mixed nullable state |

`StringHelpers.int_to_str` is a source-body result problem even though its
target identity is already sealed. `StringBox.length`/`substring` may reuse the
generated manifest target issuer, but only through a new sibling non-Loop
contract; the existing Loop contract remains unchanged.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Expression source relation | `ASTNode::If` expression sites, condition, branch wrappers, tails, and child paths are issued once by the resolver; statement `If` remains separate. |
| 2 | Source return-result proof | `StringHelpers.int_to_str` has a branded String result row or an explicit fail-fast reason; no name-only rule. |
| 3 | Non-Loop core relation | `StringBox.length`/`substring` carry manifest/schema/result/effect evidence without dropping Loop guards. |
| 4 | Nullable Bool fact | `sval != null` has exact operands and one source Bool row; unknown and mixed states reject. |
| 5 | D4 handoff | The five-row profile consumes these products with no AST rescan or MIR inference. |
| 6 | Exit | Only after source rows and the physical String port are co-sealed may a fast implementation card open. |

No code, fixture, fallback, production switch, or new semantic receipt is
authorized while D6 remains in `design_stop`.
