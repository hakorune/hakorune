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

### I0 closeout

Landed. One 218-line owner now compares the package-issued callable source
inventory and parser-issued constructor semantic rows against each exact
instance-Box Program ordinal before Builder creation. The Script root window
records `InstanceBoxSemanticOwner`; the resolver skips that subtree while
retaining all surrounding Script expressions. A foreign constructor and
incomplete method coverage reject, and the legacy raw Brand probe remains.
The two focused tests, reusable guard, quick check, pointer guard, formatting,
diff check, and all touched line-count fences are green.

## INSTANCE-CONSTRUCTOR-SEMANTIC-CONSUMER-D0

Decision: Accept one behavior-preserving BoxShape consumer: every selected-normal constructor demand must use one move-only physical ticket to borrow its existing semantic row; raw Brand routing stays unchanged.
Source authority + canonical issuer: `ConstructorSourceIdV1` selects the resolver-issued `VerifiedInstanceConstructorSemanticRowV1`; the work-plan `VerifiedInstanceConstructorPhysicalDemandManifestV1` is the sole issuer of demand role/cardinality.
Non-authority: `NormalInstanceConstructorSourceKeyV1`, box/key/name, batch order, repeated lookup, `CompilationContext::brand_decls`, and raw `is_brand_declared` cannot select or reissue a semantic row.
Fail-fast boundary: consume exactly one expected `(source_id, role)` ticket, loan one matching forest, install/restore its request-local semantic scope before body effects, and reject missing/foreign/duplicate/swapped tickets or rows with no raw fallback.
Smallest next slice: `INSTANCE-CONSTRUCTOR-SEMANTIC-LOAN-CONSUMER-I0` — pass the non-Clone ticket through a focused capture trait, add adapter-owned manifest exhaustion, exact source-ID loan, and constructor scope around the existing raw body lowering.
Non-claims: No raw Brand-probe deletion, Compatibility/RawLegacy/Deferred closure, bare or unlocated `FunctionCall` coverage, unwrap activation, nominal Brand representation, runtime ABI, or backend change.

Consumer contract:

- App selected-normal instance Box consumes `ImmediateDeclaration` exactly once.
- Non-app plain consumes `ImmediateDeclaration` plus `ScriptRuntimePrefix` exactly once;
  non-plain uses `ScriptRuntimeFullLifecycle` instead. Both roles borrow the same
  immutable semantic forest and never reissue it.
- Zero-constructor manifests complete with zero consumption. A missing, foreign,
  duplicate, already-consumed, or role-swapped ticket rejects before constructor
  observation/body effects. Compatibility and RawLegacy never receive typed tickets.
- Adapter completion requires both demand-manifest exhaustion and the existing
  callable-package completion. The enclosing Script/callable semantic scope must
  be restored after each constructor demand.

### D0 closeout

Accepted. Worker audits agree that the exact selected-normal admissions are App
Immediate, plain Immediate+Prefix, and non-plain Immediate+FullLifecycle; raw and
Compatibility lanes issue no typed ticket. The next execution row is the bounded
consumer I0 above. Global raw Brand retirement remains blocked until all
relationless admissions (including bare/unlocated calls) have their own authority.

## INSTANCE-CONSTRUCTOR-PHYSICAL-SOURCE-TRANSFER-P0

Decision: Accept one behavior-preserving BoxShape prerequisite: carry the
parser-issued constructor source ID from the installed semantic package into
every SelectedNormal immediate/runtime physical demand.
Source authority + canonical issuer: `ConstructorSourceIdV1` and the installed
`VerifiedInstanceConstructorSemanticBatchV1` rows issued by the parser-backed
resolver package; the work plan may only project and validate those rows.
Non-authority: sorted AST constructor maps, `(statement, box, key)`, demand
ordinal, physical symbol/arity, lineage, and `CompilationContext` name maps.
Fail-fast boundary: before either physical demand is emitted, every prepared
constructor must bind exactly one source ID with matching final Box ordinal,
Box/key, and function declaration; missing/duplicate/foreign/swapped rows
reject, with no legacy identity reconstruction or fallback.
Smallest next slice: add a focused source-ID transfer cohort and make the
immediate plus Script-runtime batches clone that opaque identity; leave the
package consumer and raw Brand probe unchanged until the transfer is green.
Non-claims: no semantic reissuance, consumer switch, raw-probe retirement,
Compatibility/RawLegacy closure, unwrap activation, nominal Brand value,
runtime representation, or backend change.

Acceptance:

- direct, selected-gate, overloaded, and generated `birth/0` rows preserve the
  parser `ConstructorSourceIdV1` through work-plan preparation;
- app immediate and non-app immediate/runtime duplicate demands carry the same
  source ID while remaining separate physical admissions;
- missing, duplicate, foreign, swapped, non-function, Box/key/ordinal drift,
  and old name/key-only construction reject before constructor body effects;
- the package remains the sole semantic owner and Compatibility/RawLegacy keep
  their existing untyped lane;
- focused tests, a reusable guard, formatter, pointer guard, and quick check
  are green, with every touched source below 760 lines.

### P0 closeout

Landed. `VerifiedInstanceConstructorPhysicalSourceCohortV1` now projects the
installed parser-backed semantic rows and validates final Program
Box/name/key/function coverage before SelectedNormal work preparation. The
immediate and Script-runtime constructor batches carry the same opaque
`ConstructorSourceIdV1`; they remain separate physical admissions. Missing or
foreign cohort rows fail before work-plan publication. The focused four-test
transfer suite, reusable `script_instance_box_transfer_guard.sh`, quick check,
formatter, pointer guard, diff check, and line-count fence are green. The raw
Brand consumer is unchanged; the next row is the previously identified
consumer design stop, not an automatic cutover.

## INSTANCE-CONSTRUCTOR-PHYSICAL-DEMAND-MANIFEST-I0

Decision: Accept one behavior-preserving BoxShape prerequisite: issue an
explicit physical-demand manifest before constructor lowering, with exactly
the roles `ImmediateDeclaration`, `ScriptRuntimePrefix`, and
`ScriptRuntimeFullLifecycle`.
Source authority + canonical issuer: package-issued `ConstructorSourceIdV1`,
root app/script disposition, `VerifiedScriptInstanceBoxTransferCohortV1`, and
the existing Prefix/Full classification; the single work-plan pass that
classifies the Program and splits immediate/runtime work is the issuer.
Non-authority: lower-time AST rematching, `is_app_mode` rereads, clone count,
`Option` presence, constructor keys/symbols, collector state, and runtime
terminal choice cannot issue a role.
Fail-fast boundary: each source row gets exactly one immediate ticket; only
non-app Prefix/Full rows get the matching second ticket. Missing/duplicate/
foreign/swapped role tickets, app runtime tickets, or work-item coverage drift
reject before Builder effects and never fall back to raw lowering.
Smallest next slice: `INSTANCE-CONSTRUCTOR-PHYSICAL-DEMAND-MANIFEST-I0`
adds a source-ID keyed move-only role ticket/manifest to the existing physical
batches; package semantic loan and raw consumer cutover remain later.
Non-claims: no physical-demand reduction, semantic forest reissuance, package
consumer activation, raw-key retirement, Compatibility/RawLegacy change, Brand
probe retirement, MIR/ABI, runtime, or backend change.

Manifest contract:

- app mode: every constructor row gets `ImmediateDeclaration` only;
- non-app Prefix: `ImmediateDeclaration` plus `ScriptRuntimePrefix`;
- non-app Full: `ImmediateDeclaration` plus `ScriptRuntimeFullLifecycle`;
- no constructors and Compatibility rows issue no tickets;
- the complete ticket set is checked against work-plan placement before output.

Acceptance:

- direct, overloaded, and generated `birth/0` rows receive the same role
  rules and retain their parser source IDs;
- each selected physical batch owns only its assigned ticket, while immediate
  and runtime tickets for one source remain distinct roles;
- missing/duplicate/foreign/swapped IDs or roles, Prefix/Full mismatch, app
  runtime demand, and ticket/work-item count drift reject before body effects;
- no role is inferred from clone count, `Option`, symbol, key, or lower-time
  AST matching; raw Brand consumer and semantic package loan stay untouched;
- focused positive/negative tests, reusable guard, formatter, pointer guard,
  quick check, and all touched source line counts remain green/below 760.

### I0 closeout

Landed. The selected-normal work-plan pass now issues one opaque
`ConstructorSourceIdV1` ticket per `ImmediateDeclaration` and, only for the
matching non-app Script admission, one `ScriptRuntimePrefix` or
`ScriptRuntimeFullLifecycle` ticket. A non-Clone manifest compares the complete
ticket set with the prepared immediate/runtime work before Builder effects;
duplicate, foreign, swapped, missing, and app-runtime tickets reject without
fallback. The focused transfer and admission tests, reusable guard, formatter,
quick check, pointer guard, diff check, README, and line-count fences are
green. The semantic package loan and raw Brand consumer remain unopened; the
next row is the consumer design stop.

## INSTANCE-CONSTRUCTOR-SEMANTIC-LOAN-CONSUMER-I0

Decision: Accept one selected-normal BoxShape consumer: physical constructor
work borrows the existing semantic owner through the exact demand ticket; the
raw Brand name probe remains unchanged.
Source authority + canonical issuer: `ConstructorSourceIdV1` selects the
package-issued `VerifiedInstanceConstructorSemanticRowV1`; the physical
demand manifest alone issues role/cardinality tickets.
Non-authority: Box/key/name, batch order, repeated lookup, AST rematching,
`CompilationContext::brand_decls`, and `is_brand_declared` cannot select or
reissue a semantic row.
Fail-fast boundary: a non-Clone ticket is consumed once, the matching forest
scope is installed before constructor observation/body effects, and missing,
foreign, duplicate, swapped, or unconsumed tickets reject with no raw fallback.
Smallest next slice: thread the ticket through the existing capture trait,
loan the exact row by source ID in a package child, scope the existing raw body,
and require manifest exhaustion plus package completion.
Non-claims: No global raw Brand retirement, Compatibility/RawLegacy/Deferred
closure, bare/unlocated FunctionCall coverage, unwrap, nominal Brand typing,
runtime ABI, or backend change.

Acceptance:

- App uses only `ImmediateDeclaration`; plain Script uses Immediate+Prefix;
  non-plain uses Immediate+FullLifecycle. Each role is consumed exactly once,
  while repeated roles borrow the same immutable forest.
- Missing/foreign/duplicate/reused/swapped tickets, source-key drift, and
  missing semantic rows reject before body effects. Empty manifests complete.
- Adapter completion requires demand-manifest exhaustion and existing package
  completion. Compatibility/RawLegacy cannot receive typed tickets.
- Focused admission, transfer, and demand-loan tests plus the reusable guard,
  formatter, quick check, pointer guard, and line-count fences are green.

### I0 closeout

Implemented and closed on 2026-08-20. `cargo check --profile quick`, the
focused constructor admission/transfer/demand-loan tests, formatter, pointer
guard, diff check, and `script_instance_box_transfer_guard.sh` are green. The
existing broad semantic-package and MIR Brand-constructor suites still report
the known pre-I0 `ConstructorSourceMissing`/`cohort-missing` baseline because
their fixtures do not provide the landed constructor semantic cohort; this
slice does not widen those fixtures or reinterpret the red. No raw Brand probe,
Compatibility/RawLegacy path, or production selector was changed.

The executable I0 is complete. The next state is design-stop for the remaining
relationless admissions; no automatic raw-Brand cutover or rerun is authorized.

## BRAND-CONSTRUCTOR-RELATIONLESS-ADMISSION-D3

Decision: Keep global raw-Brand retirement at design-stop after the selected-
normal constructor loan closes; remaining admissions need their own source
authority before any consumer deletion.
Source authority + canonical issuer: the next bounded row must name the exact
Compatibility, Deferred, RawLegacy, nested/Main, or unlocated admission and its
source-owned relation issuer; this card does not invent one.
Non-authority: caller-zero guesses, raw names, AST rematching, assembly, local
green, or the completed selected-normal ticket ledger cannot close the gap.
Fail-fast boundary: no fallback to `is_brand_declared`, no partial retirement,
and no new semantic receipt until one remaining family is fully covered.
Smallest next slice: a fresh worker premise audit selecting exactly one
relationless admission family, or `NoSafeSlice` if its issuer is not nameable.
Non-claims: no raw cutover, Compatibility/Deferred/RawLegacy behavior change,
unwrap, nominal Brand typing, promotion, or production switch.

## EXACT-CALLABLE-BARE-FUNCTION-CALL-LOCATION-P0

Decision: Accept one behavior-neutral BoxShape prerequisite: preserve exact
`Body(i)` source sites for bare `FunctionCall` statements in installed,
source-backed callable owners only; do not consume Brand relations yet.
Source authority + canonical issuer: the existing resolver-owned expression
inventory and owner forest issue the site; raw source transport only carries
that already-issued site into `Body(i)` and never derives one from a name/span.
Non-authority: AST spelling/span, statement ordinal alone, lineage strings,
`CallObject`, compatibility success, and `is_brand_declared` cannot issue it.
Fail-fast boundary: a semantic-backed owner must preserve one exact body site and
its child path before argument effects; missing/foreign/site drift rejects.
Raw-only, Compatibility, Deferred, nested/Main, and unlocated-only paths gain
no semantic authority or fallback from this slice.
Smallest next slice: move bare `FunctionCall` out of the finite unlocated
statement classifier only where the installed owner can prove `Body(i)`, add
transport witnesses/guard, and leave preflight/Brand consumption unchanged.
Non-claims: no relation issuance, raw Brand-probe deletion, Compatibility or
Deferred repair, nested/Main closure, `Call`/`MethodCall` widening, unwrap,
nominal Brand, runtime, backend, or production switch.

Acceptance:

- Installed callable bare calls preserve `FunctionBody -> Body(i)` and the
  exact `CallArgument(0)` child path; ordinary nested expressions remain stable.
- The transport keeps only the exact `Body(i)`/`CallArgument(0)` path; the
  later installed-owner consumer must reject missing/foreign owner, duplicate
  site, or path drift before argument effects. Raw-only and compatibility ports
  remain `CallObject` and cannot claim a site.
- Main, nested Box, Deferred, and unlocated-only families are explicit
  nonclaims; no name/key/lineage fallback is added.
- Focused transport tests, reusable guard, formatter, pointer guard, and line
  count fences are green before the later callable Brand consumer I0 opens.

### P0 closeout receipt

`EXACT-CALLABLE-BARE-FUNCTION-CALL-LOCATION-P0` is landed as one
behavior-neutral BoxShape. Installed `Cataloged`, `TopLevel`, and
`InstanceConstructor` roots now carry a bare `FunctionCall` as the exact
`FunctionBody -> Body(i)` site, and the existing child-role transport carries
`CallArgument(n)` from that site. Raw `Main`/`ScriptRoot`, nested compatibility,
`MethodCall`, indirect `Call`, and explicit extern rows remain `CallObject`.
The raw Brand preflight still owns the old name probe; no relation was issued
or consumed in this slice.

Focused command:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust raw_invocation_source_statement_classification --lib
```

Result: 5 tests passed. The reusable
`tools/checks/exact_callable_bare_function_call_location_guard.sh`, formatter,
pointer guard, existing constructor-transfer guard, line fences, and diff
check are green. The later consumer must still query the installed owner
relation before preflight; this row does not authorize a fallback or raw-probe
retirement.

## BRAND-CONSTRUCTOR-INSTALLED-CALLABLE-CONSUMER-I0

Decision: Open one design-stop BoxShape: let only an installed, source-backed
callable consume its exact `Constructor|NonBrand` relation before raw
FunctionCall preflight; keep all other admissions on their existing paths.
Source authority + canonical issuer: the verified callable owner forest and
its exact-site Brand projection issue the disposition; the source transport
only supplies the current site and never relooks up a name.
Non-authority: `CompilationContext::is_brand_declared`, AST names/spans,
`CallObject`, Compatibility/Deferred/RawLegacy/nested/Main roots, and child
ValueIds cannot issue or repair a missing relation.
Fail-fast boundary: the installed callable port must prove owner, current
site, relation kind/name, arity, and `CallArgument(0)` before child descent;
missing/foreign/duplicate/site drift rejects with no raw fallback.
Smallest next slice: a focused read-only design for the port/query seam and
preflight handoff; do not delete the legacy probe until every selected-normal
callable body has an exact disposition and the other admissions have an
explicit nonclaim.
Non-claims: no Script/Deferred/Compatibility/RawLegacy/nested/Main repair,
no unwrap, nominal Brand value, runtime/backend change, or production switch.

### Installed callable consumer I0 — accepted implementation brief

Decision: Implement one bounded BoxShape: only installed, source-backed
callable roots consume their exact `Constructor|NonBrand` relation before raw
`FunctionCall` preflight; relationless compatibility lanes retain their old
behavior and are not treated as exact consumers.
Source authority + canonical issuer: the active callable semantic ledger and
its exact transported `SourceNodeSiteV1`; the sibling query port owns no new
semantic rows and only projects the existing resolver-issued disposition.
Non-authority: `CompilationContext::is_brand_declared`, AST names/spans,
lineage/root names, argument count alone, physical ValueIds, and absent ledgers.
Fail-fast boundary: exact callable site, owner, call name, arity, and verified
`CallArgument(0)` operand site must match before child descent; query errors
never become Relationless/raw fallback. Exact NonBrand bypasses the mutable
Brand map and continues TypeOp → Math → FastMem → ordinary handling.
Smallest next slice: add a private `FunctionCall` Brand source-demand port,
thread its three-way result into raw dispatch/preflight, and lower an exact
constructor operand under the existing child-source scope. Keep
`recursive_child_lowering.rs` (794 lines) untouched.
Non-claims: no Complete Script/Deferred/Compatibility/RawLegacy/nested/Main
repair, no global raw-probe retirement, no MethodCall/unwrap, nominal Brand
typing, runtime/ABI/backend, or production route switch.

Implementation acceptance:

- TopLevel, Cataloged static/instance, App Main, and ticketed constructor
  scopes query exact Constructor/NonBrand; nested lambda owner sites remain
  exact through the existing callable ledger.
- Exact Constructor preserves pre-effect arity rejection and lowers only the
  verified operand once; exact NonBrand never calls `is_brand_declared`.
- Missing/foreign site, owner/name drift, operand-site drift, duplicate
  projection, and query error reject before child effects. Relationless
  Compatibility/Deferred/RawLegacy/nested/Main paths retain the old route.
- The new InstalledNonBrand route test, consumer guard, relation projection
  guard, callable-location guard, formatter, pointer guard, and line fences
  are green. The broader legacy route test still has one pre-existing red:
  `rejecting_routes_precede_children_and_typeop_uses_one_child` observes one
  child for the old `externcall` fixture. The exact parent commit
  `dc81a64dc7` reproduces the same 4-pass/1-fail result, so this is recorded as
  baseline debt and is not attributed to this I0.

### Follow-up TODO ledger (ordered; do not reopen landed rows)

The list below is the durable handoff. “Parked” means the row is remembered but
must not be run by inference; it needs the stated external premise or a new
accepted D0. The completed rows remain in the receipts above and in
`CURRENT_STATE.toml`'s landed tail.

#### Done

- **S6C-C-PARITY-NORTH-STAR-D0** — docs-only policy landed. Existing `1.15`
  promotion gates remain unchanged; `Hako/C <= 1.00` is the long-term point
  target, not a current measurement claim. A future `1.03` confidence margin
  requires its own D0.
- **Brand source/consumer chain through selected-normal** — parser constructor
  inventory, source transfer, demand manifest, semantic owner/loan, exact bare
  callable location, and installed-callable consumer are all landed. No global
  raw-name retirement was implied.

#### Parked evidence

1. **`S6C-MESO-HWCOUNTER-PC-ATTRIBUTION-A0`** — blocked until a new immutable
   native batch produces eligible clean pairs. The prior `NoSafeSlice` receipt
   (0 accepted pairs) is final for that plan; no rerun, subset, ratio repair,
   assembly guess, or backend owner is allowed.
2. **`HAKO-INSPECT-MIR-LLVM-BLOCK-ORIGIN-SIDECAR-I0`** — after the production
   frontier, provide issuer-emitted MIR→LLVM block/edge UX. LLVM→ASM exact
   correspondence stays explicitly unavailable without a real backend address
   issuer.

#### Next design order (one row at a time)

3. **`BRAND-COMPLETE-SCRIPT-CONSUMER-D0`** — audited and closed as
   `NoSafeSlice`: the Complete Script lexical profiles reject `FunctionCall`
   expressions, while a bare root `FunctionCall` is an explicit Deferred
   responsibility. There is no accepted Complete-Script Brand shape to
   consume; do not invent an empty projection or fallback.
4. **`BRAND-DEFERRED-SCRIPT-OWNER-ISSUANCE-D0`** — audited and closed as
   `NoSafeSlice`: the resolver returns `Deferred` before issuing any Script
   owner, so a Brand consumer cannot be added without a new semantic source
   product. Compatibility, RawLegacy, and nested/Main remain separate rows.
5. **`SCRIPT-DIRECT-STATIC-CALL-TARGET-D0/I0`** — landed as a source-only
   production-frontier observation. It issues an exact Script caller/site
   target catalog; Recipe and physical retirement remain later rows.
6. **`SCRIPT-DIRECT-STATIC-CALL-FACTS-COSEAL-D0`** — current design stop after
   target I0. Select one AST-free Script Facts/caller/result co-seal before any
   Recipe consumer; the target inventory alone is not a Recipe authority.
7. **`BRAND-METHODCALL-UNWRAP-D0`** — separate semantic and physical design for
   MethodCall/`Brand.unwrap`; do not combine it with constructor consumer work.
8. **`BRAND-CONSTRUCTOR-RAW-NAME-PROBE-R0`** — only after every production
   FunctionCall admission has an exact `Constructor|NonBrand|Unavailable`
   disposition. Until then, `CompilationContext::is_brand_declared` remains a
   compatibility authority on the relationless lanes.

#### Explicit non-goals

No threshold relaxation, WSL/native promotion claim, SIMD guess, C-reference
rewrite, production selector change, unwrap activation, or broad legacy-suite
reenactment is implied by this ledger. The known `externcall` 4-pass/1-fail
baseline remains recorded separately and is not silently converted into a new
task.

### I0 closeout

`BRAND-CONSTRUCTOR-INSTALLED-CALLABLE-CONSUMER-I0` is landed as one bounded
BoxShape. The installed callable path now consumes the resolver-issued exact
disposition; relationless lanes and the mutable-map compatibility behavior are
unchanged. The new consumer test and guards are green. The five-test legacy
preflight suite remains 4-pass/1-fail on the old `externcall` child-count
assertion, and the same failure reproduces on parent `dc81a64dc7`; it is
baseline debt, not an I0 regression. No follow-up TODO is opened in this
closeout; the numbered ledger above is the explicit handoff.

### S6C-C-PARITY-NORTH-STAR-D0 closeout

Decision: Keep the existing promotion thresholds and name `Hako/C <= 1.00`
as the long-term point target for the same sealed S6C corridor.
Source authority + canonical issuer: Existing sealed Hako/C candidate and
paired validator own future observations; this row only records policy.
Non-authority: WSL results, one run, assembly/PMU counts, p95 alone, and best
session selection cannot issue parity or compiler-owner meaning.
Fail-fast boundary: Future claims require a new immutable batch and native
authority; no current gate, corpus, C oracle, or old receipt is rewritten.
Smallest next slice: A separate future D0 may predeclare a statistical C-class
margin such as upper-95% `<= 1.03`; it is not a gate or schema field yet.
Non-claims: No current C parity result, no strict no-slower proof, SIMD,
backend BoxShape, promotion, production switch, or C-reference change.

### BRAND-COMPLETE-SCRIPT-CONSUMER-D0 closeout

Decision: `NoSafeSlice`; do not open a Complete-Script Brand consumer row.
Source authority + canonical issuer: the existing Script shadow traversal and
its `ScriptLexicalCoreV1` / `ScriptLambdaLeafV1` profiles remain the sole
source-shape issuers; their accepted expression vocabulary excludes
`FunctionCall`.
Non-authority: the already-present empty Brand projection, the root admission
label `DirectPortAwareExpression`, raw name lookup, and a deferred residual
registry cannot manufacture a Complete-Script call owner.
Fail-fast boundary: a bare Script-root `FunctionCall` remains Deferred before
child traversal, and any unsupported nested call remains outside the Complete
owner; no partial Script relation, empty cohort, or raw fallback is allowed.
Smallest next slice: audit `Deferred/Compatibility/RawLegacy` as a separate
relationless family D0; this closeout opens no code or execution row.
Non-claims: no Script Brand consumer, no `is_brand_declared` retirement, no
MethodCall/unwrap activation, no resolver profile widening, and no production
or runtime change.

### BRAND-RELATIONLESS-ADMISSIONS-D0 design stop

Decision: Do not combine Deferred, Compatibility, and RawLegacy into one
consumer. Their missing authorities are different, so the combined row is
`NoSafeSlice`.
Source authority + canonical issuer: Deferred needs a new retained Script
source/owner issuer before lowering; Compatibility needs its own final-source
semantic package; RawLegacy has no source-language issuer and remains a
compatibility transport. The existing resolver/catalog owners issue none of
these rows today.
Non-authority: `ResolveScriptOutcomeV1::Deferred`, compatibility mode labels,
AST/name spelling, `CallObject`, `CompilationContext::brand_decls`, and a
deferred residual registry cannot issue an exact Brand site.
Fail-fast boundary: missing owner/site coverage rejects before child effects;
there is no empty-cohort, relationless-to-NonBrand, or raw-name fallback in a
future exact consumer. Global `is_brand_declared` retirement remains closed.
Smallest next slice: `BRAND-DEFERRED-SCRIPT-OWNER-ISSUANCE-D0` — design only;
audit whether a complete retained Deferred source batch and owner forest can
be issued without changing the existing runtime responsibility. If not,
record `NoSafeSlice` and leave Deferred compatibility-only.
Non-claims: no Compatibility/RawLegacy/nested/Main repair, no consumer code,
no new semantic receipt, no unwrap/nominal Brand value, and no production or
runtime switch.

### BRAND-DEFERRED-SCRIPT-OWNER-ISSUANCE-D0 closeout

Decision: `NoSafeSlice`; do not create a Deferred-Script Brand owner by
relabeling the existing runtime responsibility.
Source authority + canonical issuer: the current Script resolver's shadow
profile is the only issuer, and it returns `Deferred` before `issue_owner()`
when a residual responsibility is present. No retained source/body owner or
complete deferred semantic forest exists at this boundary.
Non-authority: `ResolveScriptOutcomeV1::Deferred`, the ordinal residual
registry, raw AST/name, compatibility mode, and `brand_decls` cannot issue a
source relation or prove child-site coverage.
Fail-fast boundary: any missing owner/site or deferred residual rejects before
exact Brand consumption; no empty owner, partial Script relation, raw fallback,
or global probe retirement is permitted.
Smallest next slice: return to the production frontier with
`SCRIPT-DIRECT-STATIC-CALL-TARGET-D0`; audit a separate exact Script caller/site
target issuer before any Recipe or physical change.
Non-claims: no Deferred owner implementation, no Compatibility/RawLegacy
repair, no Script Brand consumer, no unwrap, and no runtime/production switch.

### SCRIPT-DIRECT-STATIC-CALL-TARGET-D0

Decision: Accept one BoxCount prerequisite: issue an AST-free exact Script
caller/site-to-static-target catalog, without issuing a Recipe or lowering.
Source authority + canonical issuer: the retained Script source occurrence and
its resolver traversal must co-issue a new Script owner/site, exact receiver
and argument sites, and the existing canonical static callee key before
Recipe construction.
Non-authority: callable-owner keys, receiver/name/arity lookup, spans, `using`
spelling, Deferred status, raw success, and the module result-publication owner
cannot mint a Script target.
Fail-fast boundary: missing/duplicate/foreign caller or site, alias/local
collision, overload mismatch, dynamic/instance receiver, nested-owner drift,
or unknown result/arity rejects before child effects and never falls back to
raw lowering.
Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-TARGET-I0` — implement only
the source-owned Script target catalog and completeness/negative guards; leave
Recipe, physical call/result publication, and old-route retirement unopened.
Non-claims: no Deferred owner repair, callable-key reuse, by-name fallback,
Recipe/Join issuance, physical switch, backend optimization, or promotion.

### SCRIPT-DIRECT-STATIC-CALL-TARGET-I0 closeout

The source-owned Script direct-static target inventory is landed and pushed.
It retains exact caller/receiver/argument sites and the existing canonical
callee key, while bound/dynamic/reserved receivers remain explicit
noncandidates. Six focused tests and a reusable guard are green. The inventory
is retained in the selected-normal work plan only; no Recipe, result
publication, physical lowering, fallback, or production switch was added.

### SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0 closeout

`NoSafeSlice`: the existing scalar-only `RawScriptBodyRecipeV1` and
callable-keyed result owner cannot consume ScriptRoot direct-static rows. The
target inventory is not a semantic caller/result authority and is not exposed
to a production Recipe consumer. The next design row is
`SCRIPT-DIRECT-STATIC-CALL-FACTS-COSEAL-D0`.
