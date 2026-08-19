# Brand Instance Constructor Source Relation D0

Status: parser-source prerequisite landed; instance relation design resumes
Parent: `brand-constructor-relationless-admission-d2.md`
Row: `BRAND-INSTANCE-CONSTRUCTOR-SOURCE-RELATION-D0`
Classification: Design stop; candidate implementation is one BoxCount

## D1 execution brief

Decision: Issue one instance-constructor semantic owner batch by carrying the
landed parser constructor cohort through final callable-source sealing; do not
pair it from Builder keys.
Source authority + canonical issuer: `ParserBoxSourceSealV1` constructor rows
and their parser invocation/Box paths are occurrence authority;
`FunctionSemanticResolverSessionV1` is the sole constructor-root and nested
lambda owner/Brand-relation issuer.
Non-authority: Final AST lookup, sorted constructor maps,
`NormalInstanceConstructorSourceKeyV1`, symbols, arity, physical-demand order,
callable anchors, and `CompilationContext::brand_decls` issue no occurrence or
semantic identity.
Fail-fast boundary: Final-source sealing rejects parser brand, Box path,
constructor body/key/origin/trigger drift; semantic package issuance rejects
missing, duplicate, foreign, or incomplete owner rows before Builder creation,
without Compatibility or name fallback.
Smallest next slice: `INSTANCE-CONSTRUCTOR-SEMANTIC-OWNER-I0` carries one
AST-free constructor cohort through Prepared/Final callable source, validates
the final transform, issues one non-Clone semantic batch, and exposes its
read-only package loan at the constructor port boundary.
Non-claims: No raw-probe deletion/consumption, accepted syntax change,
Compatibility/Deferred/nested-method/Main/RawLegacy repair, unwrap physical
activation, nominal Brand value typing, runtime/backend, or production route
switch.

## Required mapping

```text
ParserBoxSourceSealV1 constructor occurrence
  -> parser-issued opaque constructor source catalog
  -> Prepared/VerifiedFinalCallableProgramSourceV1
  -> transform-time exact constructor closure
  -> effective Brand catalog loan
  -> resolver-owned owner/SourceExprSite Brand relation batch
  -> NormalCallableSemanticPackage read-only loan
  -> later exact physical consumer cutover
```

The immediate, Script prefix, and full-lifecycle demands may later borrow the
same issued row. They are not separate semantic owners. Nested lambdas remain
inside the same constructor owner forest and retain exact expression sites.

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

## Landed parser prerequisite

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

No Brand relation or physical consumer changed. D1 now replaces the invalid
Builder-reconstructed premise with the landed parser inventory.

## Selected implementation row

Row: `INSTANCE-CONSTRUCTOR-SEMANTIC-OWNER-I0`
Classification: one BoxCount

Change:
  Move one parser-branded AST-free constructor cohort through Initial,
  Prepared, and VerifiedFinal callable source. Revalidate exact Box/path,
  key, function body, Direct/Generated origin, and trigger coverage after the
  callable-preserving transform. The existing semantic resolver issues one
  constructor-root plus nested-lambda owner forest and Brand relation batch.

Contract:
  Parser rows remain sole occurrence authority and the resolver remains sole
  semantic issuer. The final package owns one non-Clone batch and exposes only
  a read-only exact-source loan. Compatibility has no empty/default cohort.
  Duplicate physical demands never resolve or issue the semantic owner again.

Done:
  Zero/one/many direct, gated, and generated constructors retain exact lineage;
  constructor-only macro mutation, Box relocation, missing/foreign rows,
  body/key/origin/trigger drift, duplicate owners, and incomplete nested-lambda
  coverage reject before Builder. One reusable Brand-source guard and owner
  README/reference receipt are green; every touched source remains below 760.

Stop:
  Return to design if final transform cannot prove constructor-body identity,
  Compatibility would need a guessed empty cohort, semantic issuance requires
  callable-catalog widening, physical demand reissues an owner, or any owner
  reaches 760 without a responsibility split. Do not consume or delete the raw
  Brand probe in this row.

## I0 closeout receipt

`INSTANCE-CONSTRUCTOR-SEMANTIC-OWNER-I0` is landed. One parser-branded,
non-Clone constructor catalog now survives Initial, Prepared, and final
callable source; the exact final transform rejects constructor-body drift.
The package invokes the existing Brand-aware resolver before consuming the
final source and retains one owner forest per constructor, including nested
lambda owners. A leading non-Box program item exposed and closed an ordinal
transport bug: final Box identity now carries the actual program statement
ordinal rather than its index inside the filtered Box list.

The focused test covers two constructor rows, one nested lambda, and three
Brand relations. The reusable guard, formatter, quick check, pointer guard,
line-count fence, and diff check are green. Physical consumption remains zero;
the legacy raw Brand name probe is intentionally unchanged. Remaining
Deferred/Compatibility/nested-method/Main/RawLegacy admissions return to a
fresh design census rather than receiving an empty or guessed cohort.

## BRAND-CONSTRUCTOR-REMAINING-ADMISSION-D3

Decision: Audit the remaining relationless admissions and select exactly one source-backed family; global raw-probe retirement stays closed.
Source authority + canonical issuer: Each admitted source family must retain its own parser/resolver owner plus exact `SourceExprSiteV1`; the existing Brand catalog remains declaration authority.
Non-authority: Builder names, compatibility lineage, raw AST shape, checked-in caller count, empty cohorts, and `is_brand_declared` cannot issue missing relations.
Fail-fast boundary: A family with incomplete owner/site coverage, guessed compatibility identity, or fallback to the raw probe remains `NoSafeSlice` before child effects.
Smallest next slice: Read-only census of Deferred, Compatibility, nested-method/Main, and RawLegacy entrypoints, followed by one bounded six-line implementation brief.
Non-claims: No consumer cutover, unwrap activation, nominal Brand value, runtime/backend change, fallback, or production probe retirement.

### D3 accepted decision

Decision: Accept one BoxCount: `InstancePrefixCompatibility | NonPlainInstanceFullLifecycle` becomes an exact transferred Script boundary only when installed method and constructor semantic coverage co-seal the whole Box subtree.
Source authority + canonical issuer: `ScriptRootSemanticDecisionV1` owns the Program-item disposition; installed callable and constructor batches prove transferred coverage; the existing Script resolver remains sole owner/site relation issuer outside that subtree.
Non-authority: Deferred status, statement ordinal, AST name/span, raw success, `brand_decls`, constructor key, and `Option::None` cannot issue a transferred boundary or partial Script owner.
Fail-fast boundary: Missing/foreign method or constructor coverage, callable Compatibility, nested raw member, another unresolved deferred statement, or a later Script-ledger descent into the transferred subtree rejects before Builder effects.
Smallest next slice: `SCRIPT-INSTANCE-BOX-TRANSFERRED-BOUNDARY-I0` adds one verified transfer witness and one resolver-consumed boundary arm; no physical Brand consumer changes.
Non-claims: No partial Script owner, Brand-only side traversal, callable Compatibility/RawLegacy repair, nested/static-Main issuance, raw-probe cutover, unwrap activation, runtime, or backend change.

### Selected implementation row

Row: `SCRIPT-INSTANCE-BOX-TRANSFERRED-BOUNDARY-I0`
Classification: one BoxCount

Change:
  Co-seal one instance-Box Program occurrence with the installed ordinary-method
  semantic rows and parser-issued constructor semantic rows. Project that exact
  occurrence as a transferred boundary into Script root construction, so the
  Script resolver owns all surrounding expressions but never re-enters the Box.

Done:
  Exact one-Box and mixed Script positives seal one Script owner; transferred
  Box expressions are absent from the Script inventory. Missing/duplicate/
  foreign method or constructor rows, Compatibility package, second unresolved
  deferred item, and subtree re-entry all reject. Focused tests, guard, README,
  pointer guard, and every touched source below 760 are green.

Stop:
  Return to design if coverage requires name/key reconstruction, a partial
  Script product, a second Brand traversal, or growth of a 760-line owner. Do
  not consume `is_brand_declared` or widen Compatibility in this row.
