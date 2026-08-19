# Brand Instance Constructor Source Relation D0

Status: parser-source prerequisite landed; instance relation design resumes
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

## D0 correction

`NormalInstanceConstructorSourceKeyV1` is not parser-issued.  The parser puts
constructor declarations into a `HashMap`; Builder later sorts the surviving
keys and constructs the key from statement index, Box spelling, and map key.
That is a deterministic physical selector, but it cannot recover written
member order, overwritten duplicate rows, selected-gate provenance, or a
synthetic `birth/0` source.  Instance-constructor semantic issuance therefore
remains `NoSafeSlice` until the parser owns a total constructor inventory.

## Selected implementation row

Row: `PARSER-BOX-CONSTRUCTOR-SOURCE-INVENTORY-I0`
Classification: one BoxCount

Change:
  Add a parser-invocation-branded constructor source inventory beside the
  method source seal. Direct constructors are committed at the active member
  site; generated `birth/0` records its initializer-trigger provenance. Final
  sealing validates exact selected AST-map coverage and rejects duplicate or
  malformed rows instead of overwrite/drop.

Contract:
  `OpenBoxMethodSourceTransactionV1` is the sole issuer. Constructor key,
  written/gate member site, order, and `Direct | GeneratedBirthInitializer`
  provenance come from that transaction. Builder sorting, AST map membership,
  names, and physical demand count remain non-authority. No Brand semantic
  owner or consumer is added in this row.

Done:
  Direct `init/pack/birth` overloads retain exact sites and order; duplicate
  same-key and selected-gate collision reject; generated `birth/0` is explicit;
  missing/extra/non-function/tampered coverage rejects before resolver or
  Builder. Focused tests, one reusable parser-source guard, line counts, and
  parser/source-owner README receipt are green.

Stop:
  Return to design if generated constructors cannot name their exact trigger
  source, gate merging requires HashMap overwrite, final AST coverage requires
  key/name inference, or any source owner reaches 800 lines. Do not implement
  instance Brand relations or consume `is_brand_declared` in this row.

## I0 receipt

`PARSER-BOX-CONSTRUCTOR-SOURCE-INVENTORY-I0` is landed. The parser transaction
now records direct constructor rows before map insertion, rebases selected-gate
sites, records exact stored-field and `birth once` triggers for generated
`birth/0`, and rejects duplicate keys before overwrite. The final source seal
revalidates exact constructor key/function coverage after postpass selection.
The focused 12-test guard, 5 finalizer tests, 3 delegate-source tests, formatter,
pointer guard, and diff check are green. The main source owner is 749 lines and
the constructor child is 222 lines.

No Brand relation or physical consumer changed. The next design row is
`BRAND-INSTANCE-CONSTRUCTOR-SOURCE-RELATION-D1`: replace the invalid
Builder-reconstructed occurrence premise with the landed parser inventory and
name the exact one-owner loan into constructor-body semantic resolution.
