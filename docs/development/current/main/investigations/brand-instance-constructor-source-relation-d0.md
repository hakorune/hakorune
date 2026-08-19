# Brand Instance Constructor Source Relation D0

Status: selected
Parent: `brand-constructor-relationless-admission-d2.md`
Row: `BRAND-INSTANCE-CONSTRUCTOR-SOURCE-RELATION-D0`
Classification: Design stop; candidate implementation is one BoxCount

## Execution brief

Decision: Select instance-constructor bodies as the first relation-less family;
issue one move-only AST-free semantic batch rather than inferring Brand
membership in raw lowering.
Source authority + canonical issuer: The parser-owned constructor-map
occurrence identified by `NormalInstanceConstructorSourceKeyV1`, the effective
Brand catalog, and `FunctionSemanticResolverSessionV1` jointly issue one
owner/site product per constructor source occurrence.
Non-authority: Constructor lineage/key alone, normalized method names,
duplicate physical demands, AST spelling, `brand_decls`, raw success, and the
ordinary callable catalog cannot issue Brand membership.
Fail-fast boundary: Before Builder entry require exact constructor
count/key/source-shape and exactly one semantic row; missing, duplicate,
foreign, or re-paired rows reject before body or argument effects, with no name
fallback.
Smallest next slice: Design the bounded batch/loan seam from
`PreparedInstanceBoxConstructorBatchV1` through the resolver to the sole
`lower_normal_instance_constructor_v1` edge; the same source product may serve
multiple physical demands but must never be reissued.
Non-claims: No raw-probe deletion or consumer cutover, nested method, Deferred
Script, callable Compatibility, RawLegacy, unwrap activation, nominal Brand
typing, runtime, backend, or callable-catalog widening.

## Required mapping

```text
parser constructor-map source occurrence
  -> NormalInstanceConstructorSourceKeyV1
  -> effective Brand catalog loan
  -> resolver-owned owner/SourceExprSite Brand relation batch
  -> exact source-keyed loan around lower_normal_instance_constructor_v1
  -> existing physical constructor demands
```

The Script prefix and full-lifecycle demand may borrow the same issued row.
They are not separate semantic owners.  Nested lambdas remain inside the same
constructor owner and must retain exact expression sites.

## Acceptance for the later I0

- Zero, one, and multiple constructor rows preserve deterministic parser keys
  and exact source occurrence identity.
- Natural `Brand(value)` in a constructor body, including inside a nested
  lambda, receives one exact declaration/owner/call/operand relation before
  Builder effects.
- Every production call to `lower_normal_instance_constructor_v1` carries the
  matching semantic loan; duplicate physical demand does not duplicate issue.
- Wrong count/key/owner/source shape, missing or duplicate relation, foreign
  catalog, and operand-site drift reject before body lowering.
- Existing physical behavior is unchanged: arity rejects before child descent;
  success descends exactly one child.
- No relation is reconstructed from constructor symbol, lineage, AST name, or
  mutable `CompilationContext` state.

## NoSafeSlice

Stop if parser normalization cannot retain a one-to-one source occurrence, if
the two physical demands require separate semantic issuance, if exact nested
expression sites cannot be retained by the resolver, or if implementation
requires adding constructors to the ordinary callable catalog.  Do not repair
any failure with a raw name probe or an empty/default semantic row.
