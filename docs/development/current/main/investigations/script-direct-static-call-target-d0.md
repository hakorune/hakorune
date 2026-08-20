# SCRIPT-DIRECT-STATIC-CALL-TARGET-D0

Status: target/Facts/continuation/Recipe/Join and
`SCRIPT-DIRECT-STATIC-SOURCE-ADMISSION-I0` are landed. The previous design
stop `SCRIPT-DIRECT-STATIC-PRE-DESCENT-P0` closed as `NoSafeSlice` on the
typeop overlap; `SCRIPT-DIRECT-STATIC-TYPEOP-DISJOINT-I0` then closed the
shared predicate with focused tests and a green reusable guard. Four read-only
audits closed the physical-bridge D0 boundary, and the claim-lifecycle P0
design is now closed. Its operational claim carrier I0 is closed; the next
frontier is the physical-bridge D0 design stop. Physical Call/publication,
canonical transport, fallback, raw retirement, and production switch remain
closed.
Parent: `brand-instance-constructor-source-relation-d0.md`

## Current capsule

- `1.15` remains the current performance acceptance ceiling and `1.00` remains
  the formal long-term C-parity point target; a possible upper-95% `<= 1.03`
  claim is a separate future D0. This row is not a performance or backend task.
- Deferred Script owner repair is `NoSafeSlice`; this row does not relabel it.
- The target/result bundle, source continuation, and ScriptRoot result owner are
  Facts/source products. Recipe I0 and its Join handoff are now transported
  source products, but a local `static box Helper { value() { return 7 } }`
  direct-static input previously entered the compatibility parser arm and
  stopped at `ConstructorSourceMissing`. I0 now gives every
  compatibility-compatible initial source an explicit empty-or-complete
  constructor cohort. Mixed outer and unsupported cohorts retain compatibility
  status and do not gain a physical Script route. The typeop route premise is
  now closed by one shared pure policy; the next question is the missing
  selected-normal claim lifecycle, not source admission.
- 760 lines is the source split/design trigger and 800 is the hard stop.
  `owner_forest.rs`, `recursive_child_lowering.rs`, package install, and the
  751-line Script runtime owner are no-growth owners.

## Current six-line brief

Decision: close `SCRIPT-DIRECT-STATIC-CLAIM-LIFECYCLE-I0`. The source shape and
semantic Facts remain unchanged; this execution-local BoxShape gives the
existing Bundle/Join rows one one-shot carrier without emitting physical Call
MIR. Open the physical-bridge D0 design stop next.

Source authority + canonical issuer: `VerifiedScriptDirectStaticResultBundleV1`
issues site membership and `VerifiedScriptDirectStaticJoinHandoffV1` issues the
complete row. `ScriptDirectStaticClaimLedgerV1` co-seals those existing rows at
Script state construction and is the only operational claim issuer.

Non-authority: immutable getters, AST/name/ordinal, Join miss interpreted as
Absent, callable-key conversion, `ValueId`/`MirType`, generic Call receipt,
`ScriptPhysicalExitCommitV1`, compatibility/deferred lowering, and fallback.

Fail-fast boundary: Bundle miss returns an unchanged `Absent`; Bundle hit must
find the same-site Join row or return `Err`. `Claimed` moves a non-Clone token
before receiver/argument descent. In-flight or completed sites return
`DuplicateClaim`, never `Absent`. The operational `finish` consumes the scope
and requires zero pending and zero in-flight rows; the future physical bridge,
not this carrier-only row, owns calling it around a real candidate. No current
I0 path fabricates a consumer merely to force exhaustion; post-claim failure
remains discard-only, with no rollback/retry/ordinary escape.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-BRIDGE-D0` — design
the selected-normal StaticReceiver capability and success-only scope finish
before any physical consumer code. The carrier is seeded, rejects partial
Bundle/Join products, tombstones completed sites, and exposes only read-only
consumer views. Keep physical Call, ExactI64 publication, Return/signature,
and canonical transport closed until that D0 is accepted.

Non-claims: no new semantic `Verified*` receipt, parser/source admission,
Recipe/Join redesign, physical bridge, callable owner extension, Box/ABI
change, raw retirement, production switch, performance measurement, or
C-parity result.

## Historical target-I0 contract

### Source product

Create `src/mir/source_call_target/script_direct_static.rs` as a focused sibling
under 300 lines. It owns:

- opaque `ScriptStaticCallSourceOwnerIdV1`, issued once per selected Script
  admission;
- `VerifiedScriptDirectStaticCallSiteV1` containing owner, exact
  `SourceExprSiteV1`, receiver site, and arity;
- `VerifiedScriptDirectStaticCallTargetV1` containing the exact site and the
  canonical callee key; and
- a complete observed-call inventory with explicit static-target subset and
  noncandidate count.

The Script caller owner is not a `CanonicalSameModuleCallableKeyV1`. Existing
declaration/import catalogs may validate the callee and provide the canonical
key, but they do not issue Script caller identity.

### Issuance and retention

At the existing lifecycle point immediately after
`VerifiedStaticImportAliasViewV1::seal` and beside
`VerifiedWholeSourceStaticCallTargetInventoryV1::verify`, issue the inventory
from the retained Program AST, the sealed Script demand window, declaration
catalog, and import view. Attach it to `PreparedScriptRootAdmissionV1` and carry
it unchanged through `PreparedProgramRootWorkPlanPartsV1`.

I0 must not edit `raw_root_body_recipe.rs`, call emission, Join, static-result
publication, or runtime lowering. The later Recipe producer alone may consume
the read-only target inventory.

### Observation rules

- Walk only the retained ProgramBody statements covered by the sealed window.
- Observe every MethodCall with exact call, receiver, and argument sites.
- Accept only a qualified static receiver whose existing lexical/import rules
  select one canonical static declaration at the exact arity.
- Record instance, dynamic, unknown, reserved, and non-static receivers as
  explicit noncandidates; they are not missing rows.
- Reject duplicate site, window/AST ordinal drift, foreign catalog, ambiguous
  alias, target outside the declaration catalog, nested callable crossing, and
  receiver/argument-site mismatch before any child lowering.

### Focused acceptance

Positive: same-module qualified static call, imported alias, overload-by-arity,
two identical calls at distinct sites, and a zero-call complete inventory.

Negative: alias/local collision, `me.foo`, instance/dynamic receiver, unknown
target, nested lambda boundary, duplicate/foreign window, and AST transform
drift. Every negative leaves no catalog and no partial output.

Guard: whitelist only this new source-call child, Script admission retention,
lifecycle wiring, focused tests, and owner documentation. Reject edits to
Recipe, Join, physicalizer, method handlers, backend, and fallback paths.

## I0 closeout receipt

- Implementation: `script_direct_static.rs` issues the source-owned Script
  caller/site inventory; `normal_script_root_demand_window.rs` retains it; the
  normal default lifecycle issues it beside the existing import/target seals.
- Observation: the shared resolver traversal covers only the sealed Script
  ProgramBody window. Static qualified calls retain exact call/receiver/
  argument sites and the existing canonical callee key; bound, dynamic,
  reserved, and non-static receivers are explicit noncandidates.
- Focused gate: `CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust
  --lib script_direct_static -- --test-threads=1` — 6 passed, 0 failed.
- Reusable guard: `tools/checks/script_direct_static_target_guard.sh` pins the
  source/admission boundary, the Recipe/Join/physical non-claims, and the
  760-line split trigger.
- Boundary: no Recipe, Join, result-publication, physical call lowering,
  Deferred repair, Compatibility/RawLegacy merge, fallback, or performance
  claim was added. The next row must be selected explicitly before consuming
  this inventory.

## Later rows (not opened)

1. `SCRIPT-DIRECT-STATIC-CALL-RECIPE-R1`: consume the target catalog into one
   Script Facts/Recipe call relation and exact argument/value keys.
2. `SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-R2`: use the existing canonical call and
   result-publication owner, then retire only the selected raw Script edge.
3. Brand relationless lanes and `MethodCall/Brand.unwrap` remain independent;
   this row does not weaken their stop conditions.

## Next design stop (not opened)

`SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0` is closed as `NoSafeSlice`: the I0
inventory is retained inside the Builder work plan but has no production Recipe
consumer, and the existing scalar Recipe/result owners cannot receive it.

Decision: select one preceding Facts co-seal design before any Recipe
construction; do not add an accessor-only bridge or synthetic callable caller.

Source authority + canonical issuer: the parser-retained Program plus
`FunctionSemanticResolverSessionV1` must co-seal one AST-free Script semantic
source, target rows, caller owner, ordered argument sites, and result site.

Non-authority: target inventory alone, AST/name/arity lookup, scalar-only
`RawScriptBodyRecipeV1`, callable-keyed result publication, raw success,
assembly, timing, or a synthetic callable caller key.

Fail-fast boundary: missing Script owner, target/semantic site drift,
incomplete argument/result coverage, foreign target, or result-owner mismatch
remains `NoSafeSlice` before Builder effects.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-FACTS-COSEAL-D0` design audit
only; choose the one Facts owner and its exact transport before implementation.

Non-claims: no Recipe/Join/physical implementation, scalar Recipe widening,
fallback, promotion, or production switch is authorized by the completed I0.

### FACTS-COSEAL-D0 audit receipt

- The target inventory is issued at
  `normal_default_root_catalog_lifecycle.rs:448-466` and retained through
  `PreparedProgramRootWorkPlanPartsV1`, but `program_root_lowering.rs` does not
  consume it.
- `VerifiedScriptSemanticSourceV1` owns resolver forests and lowering
  projections, but not target rows, caller identity, argument/value relations,
  or result publication. Its existing product is the only plausible semantic
  co-seal point; no name/key re-pairing is allowed.
- `normal_source_plan::script_recipe.rs` consumes the separate
  scalar-only `RawScriptBodyRecipeV1`; it is not the selected Script target
  consumer. MethodCall/argument/result Recipe widening is a later design, not
  an implementation permission here.
- Existing static result publication is keyed by
  `(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1)` and
  `RawInvocationRootLineageV1::Cataloged`; ScriptRoot has no valid caller
  mapping. A synthetic callable key is forbidden.
- Positive design witnesses must co-seal same-module and alias targets,
  ordered arguments, result site, and zero-call completeness. Negatives must
  reject foreign catalog/window, owner/site drift, argument gaps, nested-owner
  crossing, and missing result ownership before effects.
- This audit closes the Recipe row without code changes and selects only the
  next design consultation. The existing I0 guard and six focused tests remain
  the evidence for the observation product.

### FACTS-COSEAL-D0 design conclusion

Decision: keep the target inventory observation-only and select the resolver's
existing `VerifiedResolvedMethodCallSourceV1` as the sole Script Facts input.
The Facts product may co-seal Script owner/site, receiver, ordered argument
sites, result site, and the exact canonical target, but no Recipe key or
physical value.

Source authority + canonical issuer: `resolve_script_forest_with_declaration_views`
and its `VerifiedResolvedScriptV1` product issue the semantic rows; the Facts
child validates the I0 target row against those rows once. AST re-scan, names,
arity, and the `ScriptStaticCallSourceOwnerIdV1` alone cannot issue meaning.

Non-authority: the scalar-only `RawScriptBodyRecipeV1`, callable-keyed static
result owner, `ScriptRoot` raw success, synthetic callable keys, assembly,
timing, and Deferred/Compatibility routes.

Fail-fast boundary: no Complete Script forest, foreign owner, target/receiver/
argument/result-site drift, missing or duplicate row, or nested-owner crossing
may publish Facts. ScriptRoot must not be passed to the existing callable
result-publication owner.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-ISSUER-D0`
design only — specify the Script query and Script-specific result/publication
owner before Facts I0. Recipe and physical R1/R2 stay closed.

Non-claims: no Facts receipt implementation, Recipe/Join, result handoff,
physical lowering, raw-edge retirement, production switch, or C parity.

### RESULT-OWNER-D0 audit receipt and closeout

Decision: `SCRIPT-DIRECT-STATIC-CALL-RESULT-OWNER-D0` is `NoSafeSlice`.
The existing result owner is callable-only and requires
`(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1)`; converting
`ScriptRoot` into that caller key would create a second, false authority.

Source authority + canonical issuer: the resolver's
`VerifiedResolvedScriptV1`/`VerifiedResolvedMethodCallSourceV1` rows issue the
exact Script owner, call/receiver site, ordered argument sites, and result
site. The Script target inventory issues the canonical callee; an existing
callee result catalog may provide representation data but cannot issue the
Script caller or publication owner.

Non-authority: `RawInvocationRootLineageV1::ScriptRoot`, AST/name/arity
lookup, synthetic callable keys, `BodyEffectShapeV1`,
`ScriptRootReturnExitAdmissionV1`, `EffectMask`/`FunctionSignature`,
`ScriptSemanticLoweringState`/`ValueId`, raw success, assembly, or timing.

Fail-fast boundary: missing Complete Script forest/query, target or result
representation drift, missing/duplicate/foreign owner/site/argument/result or
return row, nested-owner crossing, or callable-owner contamination rejects
before Builder effects. No fallback or partial Script publication is allowed.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-ISSUER-D0`
design only. Define one Script-specific source/result/publication owner keyed
by Script owner plus exact site, with one explicit handoff to later Facts and
Recipe work. Do not widen the callable owner or touch lowering.

Non-claims: no Facts I0, Recipe/Join, physical handoff, raw retirement,
production switch, threshold change, or performance conclusion.

### SCRIPT-RESULT-ISSUER-D0 design decision

Decision: keep the callable-keyed result owner Callable-only and define a
separate Script owner before any Script Facts or Recipe implementation. This
design is source-bound but is not itself a physical lowering permission.

Source authority + canonical issuer: a future sibling issuer receives the
Complete Script resolver forest, the already-issued target inventory, the
declaration/import-branded target catalog, and the callable result catalog as
a representation provider. It iterates the resolver's
`VerifiedResolvedMethodCallSourceV1` rows, validates the exact target row once,
and issues one owner keyed by
`(ScriptStaticCallSourceOwnerIdV1, SourceExprSiteV1)`. Each handoff retains
the Script owner/site, receiver site, ordered argument sites, result site,
canonical callee key, and only the provider-issued result representation.
The handoff contains no Recipe key, ValueId, MIR type, or physical block.

Non-authority: `CanonicalSameModuleCallableKeyV1` as a Script caller,
`RawInvocationRootLineageV1::ScriptRoot`, AST/name/arity re-resolution,
`BodyEffectShapeV1`, `ScriptRootReturnExitAdmissionV1`,
`ScriptSemanticLoweringState`, and the callable result catalog's caller rows.
The result catalog may provide callee representation only; it cannot mint a
Script caller, source site, or argument relation.

Fail-fast boundary: issue only for a Complete Script forest. Missing or
duplicate resolver/target rows, foreign owner or catalog branding, receiver/
argument/result-site drift, target declaration/import drift, unavailable
callee representation, nested-owner crossing, or a callable-owner handoff
attempt rejects before Builder effects. Deferred/Compatibility paths receive
no empty owner and never fall back to name lookup.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-BUNDLE-D0`.
Fix one source bundle that co-seals the resolver Script view, target inventory,
and declaration/result catalog brands before implementing the owner. The
owner/Facts I0 remains closed until that bundle has an exact issuer and
consumer.

Non-claims: no Script Recipe admission, result publication to MIR, argument
type proof, physical call, raw edge retirement, production switch, threshold,
or performance claim. No guessed owner, empty catalog, or AST re-pairing is
allowed while the source bundle is unspecified.

### SCRIPT-RESULT-BUNDLE-D0 design stop (closed)

Decision: `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-ISSUER-D0` is not yet an
implementation row. It is a `BoxCount` candidate, because admitting a Script
direct-static result demand would add a new source-to-result shape. First fix
the source bundle and its provenance; do not add a `Verified*` owner from
separately branded inputs.

Source authority + canonical issuer: the resolver's
`resolve_script_forest_with_declaration_views` must expose a Script-specific
method-call view containing exact owner/site, receiver, ordered argument sites,
and result site. The target inventory issues the canonical callee target, while
the declaration/whole-source result catalogs provide only their already-sealed
representation data. A single bundle issuer must co-seal these inputs and
their admission/window/catalog brands before the owner is issued.

The canonical issuer is named
`VerifiedScriptDirectStaticCallResultBundleV1::issue`. Its source owner is the
unique `FunctionOwnerIdV1` of the Script root in the sealed semantic forest;
the fixed `ScriptStaticCallSourceOwnerIdV1::ROOT` is only a target-observation
coordinate and is not caller authority. The issuer consumes the Complete
Script forest, the same Program/window admission, the target inventory, the
declaration/import catalog brand, and the callable result catalog. It emits
owned rows containing the Script owner, exact call/receiver/ordered argument
sites, result site, canonical callee key, callee representation, and provider
required-argument ordinals. It stores no AST, Recipe key, ValueId, MIR type,
JoinSig, or physical block.

The target inventory must either carry exact Program/window/declaration/import
identity and a complete iterator, or be generated inside this issuer. A
`VerifiedResolvedScriptV1::method_calls()` query is a read-only resolver view;
it is not a second issuer. Bundle validation pairs rows by exact site only:
target rows supply the target, resolver rows supply all source sites, and the
result catalog supplies only the callee representation.

Non-authority: the target inventory's AST walk by itself, `ScriptStaticCallSourceOwnerIdV1`,
`CallableSemanticSourceLedgerView::as_function`, synthetic callable keys,
AST/name/arity re-resolution, `RawInvocationRootLineageV1::ScriptRoot`,
`EffectMask`, `FunctionSignature`, `ValueId`, Join, and raw success.

Fail-fast boundary: no Script method query, missing/foreign/duplicate bundle
brand, target/resolver site drift, receiver or argument coverage gap, result
catalog drift, Deferred forest, nested-owner crossing, or unavailable callee
representation may reach Builder effects. The target inventory cannot be
treated as a second semantic issuer or silently paired by name.

Acceptance matrix: zero static rows yields one Complete empty bundle; a
qualified same-module or imported-alias row yields one exact demand; two call
sites to one target remain two rows; ExactI64 and ExactNominalBox dispositions
are retained; noncandidate instance/dynamic/reserved calls stay outside the
static subset; a call in a root Return leaves Return ownership with the
existing resolver. Foreign forest/window/program/catalog, fixed-ROOT versus
forest-owner mismatch, missing/duplicate/ordered argument rows, receiver or
result-site drift, target arity/namespace drift, unavailable or recursive
callee result, Deferred Script, and synthetic callable-key conversion all
reject before effects. Duplicate take and wrong target reject; no partial or
raw fallback is published.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-BUNDLE-D0`
design only. Specify the bundle owner, Script query, exact brands, and the
owner/Facts consumer boundary. Only after this closes may
`SCRIPT-DIRECT-STATIC-CALL-RESULT-OWNER-FACTS-I0` be selected.

Non-claims: no result owner/receipt, Facts/Recipe/Join implementation,
physical handoff, raw retirement, production switch, or performance evidence.

### RESULT-OWNER-FACTS-I0 closeout receipt

Decision: close `SCRIPT-DIRECT-STATIC-CALL-RESULT-OWNER-FACTS-I0` as one
bounded Facts product. The Script result bundle is now attached to the
Complete Script semantic source; Recipe, Join, result publication, and
physical lowering remain unopened.

Source authority + canonical issuer: the resolver's forest-root
`FunctionOwnerIdV1` and exact `VerifiedResolvedScriptV1::method_calls()` rows
issue source sites. The target inventory supplies only the already-sealed
callee target, and the result catalog supplies only callee representation and
required argument ordinals. `VerifiedScriptDirectStaticResultBundleV1::issue`
co-seals those inputs and carries the bundle into
`VerifiedScriptSemanticSourceV1`.

Non-authority: `ScriptStaticCallSourceOwnerIdV1::ROOT`, AST/name/arity
re-pairing, callable-key conversion, raw success, `ValueId`, Recipe key,
assembly, timing, and Deferred/Compatibility/RawLegacy routes.

Fail-fast boundary: target inventory provenance must match the exact retained
Program/window/declaration/import views; the forest must have one Script root;
resolver rows, target rows, receiver/argument/result sites, namespace/arity,
and callee representation must match by exact site. Missing, foreign,
duplicate, drifted, unavailable, or Deferred inputs reject before Builder
effects. No empty-row fallback is issued for a missing semantic source.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0` design only. Define
the Script Recipe/Join/result-publication owner from this bundle before any
physical consumer is opened.

Evidence: `normal_script_direct_static_result_bundle.rs` is 271 lines and the
focused sibling test file is 148 lines. The positive Complete Script fixture
closes an empty static-target inventory and checks forest-root/source identity;
the foreign-program inventory is rejected. `normal_script_semantic_lowering_projection.rs`
now consumes the shared owner-core facts for Script as well as Function, which
closes the existing Script projection guard without creating a second source
authority. Gates used:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick -p nyash-rust
CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust --lib normal_script_direct_static_result_bundle -- --test-threads=1
CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust --lib literal_program_seals_one_shared_script_owner_and_projection -- --test-threads=1
```

Non-claims: no nonempty Script static Recipe, result publication, physical
call, raw-edge retirement, production switch, performance result, or C parity
claim is made by this closeout.

### RECIPE-D0 audit closeout

Decision: `SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0` is `NoSafeSlice`; no Recipe row
is issued until a Script-owned source continuation and result-publication owner
exist.

Source authority + canonical issuer: `VerifiedResolvedScriptV1` already owns
MethodCall call/receiver/ordered-argument/result rows. The resolver's existing
Script body-shape seal must additionally retain statement/parent/Return/terminal
relations; a later Recipe sibling alone may issue Recipe keys.

Non-authority: AST order or re-scan, `VerifiedScriptRootDemandWindowV1` ordinal,
`ScriptRootReturnExitAdmissionV1` alone, `ValueId`, `JoinSig`, raw lineage,
callable result owner, target/name lookup, or the scalar Script Recipe.

Fail-fast boundary: missing body-shape coverage, owner/forest/window drift,
missing parent or final Return relation, bundle transport loss, unsupported
representation, Deferred/Compatibility/RawLegacy input, or missing Script
result owner stops before Builder effects; no empty/default/fallback row.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-D0`
design-only; after acceptance, implement its resolver-issued product as a
bounded sibling and then reopen Recipe I0. Keep the existing scalar Recipe,
callable result owner, physical lowering, and raw retirement unchanged.

Non-claims: no Recipe-I0, Script physical call, Join/ABI publication, raw-edge
retirement, production switch, performance result, or C-parity claim.

### SOURCE-CONTINUATION-D0 design brief

Decision: design one resolver-issued, AST-free Script continuation product
before reopening Recipe. This is a new source shape (`BoxCount`), not a
physical Join refactor.

Source authority + canonical issuer: the existing Script shadow traversal and
`seal_script_owner_with_maps` jointly own `BodyShapeRelationV1`, Return exit
records, and the exact Script owner. The new product must be co-sealed at that
boundary with the same forest root; it may not reissue MethodCall rows.

Non-authority: AST order or a second AST walk, `VerifiedScriptRootDemandWindowV1`
ordinal alone, source-site inventory without parent relations, `ValueId`,
`JoinSig`, MIR blocks, raw lineage, or the callable result-publication owner.

Fail-fast boundary: every retained Script statement/expression and every
parent/receiver/argument/Return relation must be covered exactly once; owner,
forest root, window, and terminal Return must agree. Nested-owner crossing,
missing relation, duplicate relation, partial body shape, Deferred input, or
missing Script result destination rejects before Builder effects.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-I0`
implements a focused sibling that retains the already-issued Script body shape
through the semantic source/forest handoff. It must remain below 760 lines and
leave `body_shape.rs`/`normal_script_runtime_work.rs`/`raw_invocation_source_transport.rs`
as no-growth owners where possible; scalar Recipe code stays unchanged.

Non-claims: no Recipe key, direct-static lowering, Script physical result,
raw-edge retirement, production switch, performance measurement, or C-parity
claim. Recipe I0 remains parked until this product and the Script
result-publication owner are both accepted.

### SOURCE-CONTINUATION-D0 acceptance / I0 contract

Decision: the design stop is accepted as one bounded `BoxCount`; the next
implementation row is `SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-I0`.

Source authority + canonical issuer: `FunctionSemanticResolverSessionV1`'s
`seal_script_owner_with_maps` issues the owner and `VerifiedBodyShape` from one
shadow traversal. `ResolveScriptForestOutcomeV1::Complete` must carry the
already-issued root/nested body-shape map; a new sibling validates and projects
the root Script continuation. No owner ID or MethodCall row is reissued.

Non-authority: AST re-scan, target inventory ROOT, window ordinal pairing,
`ScriptRootReturnExitAdmissionV1` alone, raw lineage, `ValueId`, `JoinSig`,
physical blocks, and the scalar Recipe path.

Fail-fast boundary: `body_shape.owner()` must equal its forest owner; exactly
one root shape must cover all Script statements/expressions/relations; every
MethodCall receiver/argument and every Return value must be covered; nested
owner rows stay separate. Missing/duplicate/foreign shape, Deferred forest,
window drift, or dangling parent relation rejects before lowering effects.

Smallest I0 seam: add body-shape retention to the Script resolver outcome and
forest handoff; add `normal_script_source_continuation.rs` (target <300 lines)
to issue source-only continuation rows; keep existing 751/666-line runtime and
transport owners, scalar Recipe, result publication, and physical lowering
unchanged. Add focused positive/negative/foreign-owner tests and one reusable
guard.

Non-claims: no Recipe key, Script result-publication owner, ValueId, physical
call, raw retirement, production switch, performance result, or C-parity claim.

### SOURCE-CONTINUATION-I0 closeout receipt

- The Script resolver product now retains the exact `VerifiedBodyShape` issued
  by the same shadow seal; the old forest handoff no longer drops it.
- `normal_script_source_continuation.rs` validates the root owner and canonical
  Script demand window, then projects only resolver-issued parent relations and
  terminal Return/sequence rows. It does not scan AST or mint Recipe/Join data.
- `record_statement_shape` is called only for `Resolved` Script demands. The
  validator therefore accepts explicit transparent/diagnostic/transferred
  boundaries without pretending they have semantic body rows.
- `VerifiedScriptSemanticLoweringInputV1` carries the lowering projection,
  continuation, and existing direct-static result bundle together so the
  source products cannot be silently dropped at the transport seam.
- Focused gate: `CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p
  nyash-rust --lib normal_script_source_continuation_tests` — 2 passed, 0
  failed.
- Reusable guard: `bash tools/checks/script_direct_static_target_guard.sh` —
  green. `bash tools/checks/current_state_pointer_guard.sh` — green.
- Owner documentation: `src/mir/builder/README.md` records the continuation
  boundary and keeps Script result publication as the next design stop.
- The broader `normal_script_semantic_source` filter has no continuation
  boundary failures after this slice; its remaining
  `mir/instance-constructor-source/cohort-missing` failures are pre-existing
  baseline debt and are not reclassified by this I0.

## Current TODO ledger (2026-08-20)

This is the single short list for the remaining work. Historical target/Facts
I0 sections above remain evidence and are not executable rows.

1. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0` → `NoSafeSlice`.** The
   existing MethodCall Facts row has call/receiver/argument/result sites but no
   retained Script-owned parent/Return/terminal continuation. The old scalar
   Recipe and callable result owner cannot fill that gap.
2. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-D0`.** The
   resolver-issued body-shape retention boundary is accepted: preserve the
   existing owner/shape pair and validate a root continuation sibling. No
   Recipe or physical meaning is issued by this design.
3. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-I0`.** The same
   resolver-issued body-shape owner is retained in the Script product and
   carried through `VerifiedScriptSemanticLoweringInputV1`; the sibling
   continuation validates the root/window boundary and projects exact rows.
   Transparent/diagnostic entries remain explicit non-shape boundaries. The
   focused continuation tests pass 2/2, and the reusable guard is green.
4. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-ISSUER-I0`.** The
   source/Facts-only ScriptRoot owner now co-seals the existing target/result
   bundle with the resolver-issued continuation by exact Script owner and
   `SourceExprSiteV1`. Missing/foreign source, owner, or continuation rows
   reject before any Recipe or physical effect; the focused owner tests and
   reusable guard are green.
5. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0`.** The new Recipe shape is
   limited to a final direct-static call whose result is either the Script
   terminal value or the root Return value. A dedicated producer issues the
   Recipe-local key and row from exactly one source/Facts owner row; existing
   scalar and Loop Recipes remain untouched.
6. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-RECIPE-I0`.** The dedicated sibling
   producer issues only its opaque Recipe-local key, validates final Sequence/
   root Return value relations, and transports the retained Recipe through the
   Complete Script lowering input/state. Focused Recipe/owner/bundle/continuation
   tests and the reusable guard are green; the broader lifecycle suite still
   reproduces the known `mir/instance-constructor-source/cohort-missing` baseline.
7. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-RECIPE-TERMINAL-I0`.**
   The Join audit found a boundary defect: a bare final expression statement
   has no parent `Value` relation, while a final local or assignment does. I0
   admits only a call equal to the terminal Sequence statement (zero parents),
   or a direct root Return value (exactly one `Value` parent); nested/local/
   assignment/control flow rejects before Recipe. The focused Recipe suite and
   guard are green.
8. **CLOSED — `SCRIPT-DIRECT-STATIC-CALL-JOIN-I0`.** The new source/Facts
   handoff consumes exact Recipe and result-owner rows, preserves FinalSequence
   and RootReturn destinations, and rejects foreign, missing, duplicate, or
   drifted rows. It is carried through Script lowering input/state without
   issuing physical meaning.
9. **CLOSED — `SCRIPT-DIRECT-STATIC-PRE-DESCENT-P0` -> `NoSafeSlice`.** The
   selected-normal ingress and joined handoff preserve exact call, receiver,
   ordered argument, and `FinalSequence`/`RootReturn` sites. However, the
   parser accepts `is`/`as` as ordinary identifiers, while
   `build_method_call_from_input_v1` selects the one-string-argument typeop
   route before `MemberCallRoutePlan::StaticReceiver`; the target issuer only
   excludes reserved routes. A legal static `is/as` method can therefore be
   both a target-inventory candidate and an earlier typeop. The bridge is not
   authorized.
10. **CLOSED — `SCRIPT-DIRECT-STATIC-TYPEOP-DISJOINT-P0`.** The design proof
   selects one neutral source-method typeop predicate as the sole route
   authority. It must classify `is/as` plus one extractable string (including
   the existing StringBox form) as noncandidate, while ordinary `is/as` with
   other arguments remains eligible. Builder and the Script target issuer will
   consume the same predicate; no physical meaning is issued here.
11. **CLOSED — `SCRIPT-DIRECT-STATIC-TYPEOP-DISJOINT-I0`.** One pure typed
   policy now owns the `is/as` route predicate. Builder typeop routing and the
   Script target inventory delegate to it; typeop-shaped one-string calls are
   explicit noncandidates while ordinary `is/as` arguments remain eligible.
   The focused policy/target tests and reusable guard are green. No parser,
   Recipe/Join, physical, fallback, or production code changed.
12. **NEXT DESIGN STOP — `SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-BRIDGE-D0`.**
   Before implementation, specify the selected-normal claim/publication
   boundary at the ordinary `StaticReceiver` arm head, exact terminal witness,
   unified Call receipt, Script ExactI64 publication, failure discard, and
   ledger exhaustion. Canonical transport and raw retirement remain parked.
13. **PARKED — `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PHYSICAL-INPUT-D0`.** Name
   one complete AST-free argument/terminal input for the detached canonical
   Script session. The scalar `RawScriptBodyRecipeV1`, AST lookup, and the
   selected-normal ledger cannot fill this gap by inference.
14. **PARKED — canonical physical/exit integration.** Only after (13) closes
   may the detached candidate reuse the same Call/result kernel and delegate
   the final Return/signature write to `ScriptPhysicalExitCommitV1`.
15. **PARKED — raw retirement/production.** Retire the old Script edge only
   when every admitted Script family has an exact Recipe/result owner and the
   new path is selected. Deferred, Compatibility, RawLegacy, and nested
   families remain explicit non-claims until separately closed.
16. **PARKED — performance.** Keep current exact/meso gates unchanged. Treat
   `Hako/C <= 1.00` as the long-term same-corridor point target; a possible
   upper-95% `<= 1.03` claim requires a new predeclared batch/D0. No WSL/native
   rerun, PMU attribution, threshold change, SIMD work, or backend BoxShape is
   authorized by this card.

## SCRIPT-RESULT-ISSUER-D0 accepted design brief (2026-08-20)

Decision: Keep the callable-keyed publication owner callable-only and add a
separate ScriptRoot result-publication owner as one future BoxCount; do not
open Recipe or physical lowering in this row.

Source authority + canonical issuer: the resolver's Complete
`VerifiedResolvedScriptV1`/`VerifiedResolvedMethodCallSourceV1` rows and the
landed `VerifiedScriptSourceContinuationV1` issue caller/site/destination;
`VerifiedScriptDirectStaticResultBundleV1` supplies the exact target and
provider-issued callee representation.

Non-authority: `RawInvocationRootLineageV1::ScriptRoot`, synthetic callable
keys, AST/name/arity re-resolution, window ordinals, `ValueId`, `MirType`,
`JoinSig`, physical blocks, timing, and the callable-only result owner.

Fail-fast boundary: missing/foreign/duplicate owner, call/result/receiver/
argument/parent/Return drift, target or representation mismatch, Deferred or
unsupported input, duplicate take, or Script-to-callable conversion rejects
before Builder effects; no empty/default owner or raw fallback.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-ISSUER-I0`, a
new sibling owner that co-seals bundle rows with continuation rows by
`(FunctionOwnerIdV1, SourceExprSiteV1)` and emits only a source/Facts handoff.
Recipe keys, physical `ValueId`/`MirType`, Join, and publication to MIR remain
for later rows.

Non-claims: no Recipe/Join/physical implementation, raw-edge retirement,
production switch, performance result, C-parity claim, or widening of the
callable result owner. This D0 is documented and accepted; I0 is bounded to
source/Facts co-sealing only.

**Closed execution row:** `SCRIPT-DIRECT-STATIC-CALL-SCRIPT-RESULT-ISSUER-I0`.
The I0 published no Recipe key, physical value, or fallback. The next row is
the explicit Recipe design stop below.

## SCRIPT-RESULT-ISSUER-I0 closeout (2026-08-20)

Decision: close the source/Facts-only ScriptRoot result-publication owner as
one bounded BoxCount. The owner consumes no Recipe, Join, ValueId, MIR type,
physical block, result ABI, fallback, or performance authority.

Source authority + canonical issuer: the existing Complete Script semantic
forest supplies the unique ScriptRoot owner; `VerifiedScriptDirectStaticResultBundleV1`
supplies target/representation and `VerifiedScriptSourceContinuationV1` supplies
the exact destination/parent/terminal rows. The new sibling
`VerifiedScriptDirectStaticResultPublicationOwnerV1` co-seals those products.

Fail-fast boundary: Script-root cardinality/product, source identity, owner,
continuation owner, exact call site, missing/duplicate/foreign rows all reject
before Recipe or physical work. No callable-key conversion or empty/default
owner is permitted.

Evidence: `cargo test --profile quick -p nyash-rust --lib
normal_script_direct_static_result_publication_owner`; the source/Facts
continuation/bundle guards; and the current-state pointer guard must all be
green. The positive fixture is a complete empty Script window (zero direct
static rows); the negative fixture pairs a bundle from a foreign source and
requires `BundleSourceMismatch`. Nonempty static-row coverage remains owned by
the existing bundle/continuation tests and is not claimed by this owner test.

Closed design stop: `SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0`. The next bounded
implementation row is `SCRIPT-DIRECT-STATIC-CALL-RECIPE-I0`; do not start
Join/physical/result publication or performance work there.

## RECIPE-D0 design decision (2026-08-20)

Decision: accept one new `BoxCount` for a Script direct-static Recipe, without
changing the existing scalar/Loop Recipe vocabularies. The first accepted
shape is intentionally narrow: the call result must be the final Script
terminal value or the value of the root `Return` statement.

Source authority + canonical issuer: the landed
`VerifiedScriptDirectStaticResultPublicationOwnerV1` is the sole Facts input.
Its demand row already co-seals the Script owner/site, receiver and ordered
argument sites, result site, parent relations, terminal, canonical target,
callee representation, and required argument ordinals. A new dedicated Recipe
producer is the only issuer of the opaque Recipe key and Recipe row.

Non-authority: `RawScriptBodyRecipeV1`,
`normal_source_plan::script_recipe`, Loop Recipe types, AST re-scan, source-site
key guessing, callable-key conversion, target/name/arity lookup,
`ScriptRoot` raw lineage, `ValueId`, `MirType`, `JoinSig`, physical blocks,
or the later physical result-publication owner.

Fail-fast boundary: reject before Recipe publication when the owner is absent,
foreign, duplicated, or drifted; when target/representation/site/argument
coverage differs; or when the continuation is not exactly a final Sequence
value or root Return value. Local initializer, assignment, print, nested
expression, branch/loop, discarded call, Deferred, Compatibility, RawLegacy,
and nested owners are not empty/default Recipe rows and remain `NoSafeSlice`.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-RECIPE-I0` adds a new sibling
under 300 lines plus a focused verifier/test sibling. It consumes one owner row
at a time, issues one Recipe-local key, retains all source/Facts payloads
unchanged, and transports the resulting Recipe without touching the scalar
Recipe, raw transport, Join, physicalizer, or route selector.

Acceptance: zero owner rows produce a valid empty Recipe only for a Complete
Script source; each accepted owner row produces exactly one Recipe row; final
Sequence and root Return fixtures preserve exact destination/representation;
two sites to one target remain distinct; duplicate/foreign/missing rows and
non-final uses reject with no partial output. Recipe key issuance is confined
to the new producer, and the old Script edge remains selected until a later
physical/production cutover row.

Non-claims: no general call expression Recipe, local/assignment/print support,
nested or control-flow support, JoinSig, physical result publication,
ValueId/MIR type, raw retirement, production switch, performance, or C parity.

## JOIN-D0 design brief (2026-08-20, closed by JOIN-I0)

Decision: accept `SCRIPT-DIRECT-STATIC-CALL-JOIN-I0` as one source/Facts-only
BoxCount. A Script-specific handoff consumes the corrected Recipe and preserves
both `FinalSequence` and `RootReturn` destinations without reclassification.

Source authority + canonical issuer: `VerifiedScriptSourceContinuationV1` owns
terminal/parent facts; `VerifiedScriptDirectStaticResultPublicationOwnerV1`
owns target/representation; `VerifiedScriptDirectStaticRecipeV1` alone issues
Recipe keys. The new sibling consumes those exact rows and issues one handoff
row per Recipe key.

Non-authority: Loop `JoinSig`, scalar Recipe, AST/ordinal inference, callable
keys, `ValueId`, `MirType`, ABI, raw lineage, physical labels, and any
post-hoc destination reconstruction.

Fail-fast boundary: source identity/owner, Recipe key/site, target,
representation, argument sites, terminal, and parent relations must match
one-to-one. Sequence requires statement-node equality plus zero parents;
Return requires final ReturnExit plus exactly one direct Value parent. Missing,
foreign, duplicate, non-final, or drifted rows reject before any physical
effect; zero rows produce an empty handoff.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-JOIN-I0` — add one focused
sibling under 300 lines, transport it through Script lowering input/state, and
add positive/negative tests plus the reusable guard. Do not touch JoinSig,
physical entry/exit, raw retirement, or performance.

Non-claims: no physical blocks/values, result ABI, production switch, raw
retirement, backend optimization, measurement, or C-parity claim.

## RECIPE-TERMINAL-D0 design brief (2026-08-20)

Decision: repair the already-selected Recipe boundary before Join. This is one
bounded source/Facts contract correction, not a widening of scalar or Loop
Recipes.

Source authority + canonical issuer: `VerifiedScriptSourceContinuationV1` owns
the terminal and parent relations; the dedicated Recipe producer remains the
only Recipe-key issuer. For `FinalSequence`, the exact call site must equal the
terminal statement node and have zero parent relations. For `RootReturn`, the
call must be the direct Return value with exactly one `Value` relation.

Non-authority: AST statement order, `SourceStmtSiteV1` ordinals alone,
`ValueId`, `MirType`, `JoinSig`, raw lineage, and a relation guessed from a
method name or terminal label.

Fail-fast boundary: any sequence call with a parent, any return call with zero
or multiple parents, local/assignment/print/nested/control-flow use, foreign or
duplicate relation, or non-final terminal rejects before Recipe publication.
No empty/default row or fallback is allowed.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-RECIPE-TERMINAL-I0` — adjust
the dedicated producer only, add direct Sequence/Return positives and local/
assignment/nested negatives, then rerun the existing Recipe/guard gates. After
this closes, reopen `SCRIPT-DIRECT-STATIC-CALL-JOIN-I0` as a separate source-only
handoff.

Non-claims: no JoinSig, physical value/block, result ABI, raw retirement,
production switch, performance, or C-parity claim.

## RECIPE-I0 / RECIPE-TERMINAL-I0 closeout (2026-08-20)

Decision: close both Recipe rows as bounded source/Facts BoxCounts. The
producer issues an opaque key only after validating the exact terminal shape:
bare final Sequence has zero parents; direct root Return has one `Value` parent.
The Recipe remains unconsumed.

Evidence: the focused Recipe module passed 5/5 (empty producer, Sequence and
Return positives, local/assignment and nested-return negatives); the existing
bundle/continuation/result-owner gates remain green, and
`bash tools/checks/script_direct_static_target_guard.sh` passed. The broader
`normal_default_root_catalog_lifecycle` suite remains red only at the known
pre-existing `[freeze:contract][mir/instance-constructor-source/cohort-missing]`
baseline (6 failures), so it is not reclassified as a Recipe regression.

Non-claims: no physical Recipe consumption, JoinSig, ValueId/MIR type, result
ABI, raw-edge retirement, production switch, performance result, or C-parity
claim. The active pointer is now the `SCRIPT-DIRECT-STATIC-CALL-JOIN-D0`
design stop; no Join implementation was opened.

## JOIN-I0 closeout (2026-08-20)

Decision: close `SCRIPT-DIRECT-STATIC-CALL-JOIN-I0` as one source/Facts
BoxCount. The handoff verifies exact source identity/owner, Recipe key/site,
target, representation, ordered arguments, terminal, and parent relations.

Evidence: `cargo check --profile quick` passed; the focused Join module passed
3/3 (empty handoff, foreign-source rejection, and a non-empty Recipe row). The
reusable Script direct static guard now covers the new sibling and test module.
The broader lifecycle red remains the known instance-constructor cohort-missing
baseline.

Next: return to design stop for a separately authorized physical consumer row;
the Join handoff is transported but intentionally unconsumed.

## SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-CONSUMER-D0 design stop

Historical 2026-08-20 record; superseded by the accepted physical-bridge D0
below. Its direct-reuse rejection remains evidence, but its selected-normal
exit-owner premise is not current authority.

Decision: no physical consumer is selected yet; the next row must identify one
existing canonical Script result/exit owner before any JoinSig or MIR value work.
Source authority + canonical issuer: the joined Script Recipe/result handoff is
the only source input; the existing Script physical owner must be named before
implementation, not inferred from labels or lowering state.
Non-authority: `ScriptPhysicalExit`, Loop JoinSig, AST/ordinal, ValueId/MirType,
raw call success, assembly, and performance ratios cannot issue a destination.
Fail-fast boundary: missing owner, non-final destination, duplicate/foreign key,
or inability to preserve existing Return/Sequence exit semantics is NoSafeSlice.
Smallest next slice: a read-only owner/Join audit; no code, fixture, raw
retirement, production switch, or perf rerun until that design closes.
Non-claims: no physical implementation, ABI/result publication, backend change,
promotion, or C-parity claim.

### Physical-consumer audit closeout

Decision: close this owner-selection audit as `NoSafeSlice` for direct reuse.
No existing owner consumes the joined Script rows end-to-end. `ScriptPhysicalExitCommitV1`
is the sole final Return/signature writer, but it does not own static-call emission
or result publication. `PreparedStaticCallResultPublicationV1` and
`raw_static_result_publication.rs` are callable-keyed/Cataloged-only, while
`OpenScriptPhysicalEntrySessionV1` accepts only the scalar `RawScriptBodyRecipeV1`.

Source authority + canonical issuer: `VerifiedScriptDirectStaticJoinHandoffV1`
remains the only source input. A future Script-specific physical bridge must
co-seal the joined target/representation/ordered arguments with one physical
call receipt and then delegate the final Value/Return write to
`ScriptPhysicalExitCommitV1`.

Non-authority: callable-key conversion, AST/ordinal or ValueId/MirType matching,
Loop JoinSig vocabulary, raw success, disassembly, timing, and C ratios cannot
select a Script physical owner.

Fail-fast boundary: missing/foreign/duplicate joined rows, target/arity/argument
or representation drift, unsupported result representation, non-final terminal,
missing physical receipt, or duplicate Return/publication must reject before any
physical effect. No scalar Recipe widening, callable-key fallback, or raw retry.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-BRIDGE-D0`, a design
slice for a new focused Script physical-consumer/publication sibling. First
scope is `ExactI64` plus both `FinalSequence` and `RootReturn`; implementation
must remain separate from `ScriptPhysicalExit`, callable publication, raw
static publication, and the oversized runtime/transport owners.

Non-claims: no bridge implementation, Box result support, ABI change, raw
retirement, production switch, performance rerun, PC owner, or C-parity claim.

## Physical bridge decision and prerequisite (2026-08-20)

The bridge owner split remains accepted conditionally. Local code confirms
that `build_method_call_from_input_v1` executes typeop and reserved routes
before member planning, while the head of
`MemberCallRoutePlan::StaticReceiver` is still before receiver and argument
descent. The target issuer already excludes non-Ordinary reserved decisions;
P0 must close the remaining typeop and source-location premises on the actual
selected-normal ingress before I0 may edit this seam.

### `SCRIPT-DIRECT-STATIC-SOURCE-ADMISSION-I0` execution brief (landed)

Parent: `SCRIPT-DIRECT-STATIC-CALL-TARGET-D0`; accepted BoxCount boundary;
source/admission implementation only.

Change:
  Extend the compatibility arm that is already admitted to the initial
  callable lane. Reuse the existing source-seal finalizer and constructor
  catalog issuer for retained non-static boxes, allowing zero constructor rows
  as one canonical empty catalog for static-only, top-level-only, and mixed
  inputs. Attach that non-Clone catalog to the initial callable source before
  the existing `PreparedNormalCallableProgramSourceV1::issue` check. Preserve
  the outer mixed compatibility envelope; interface, record, build-gate, and
  unsupported cohorts remain compatibility-only.

Authority contract:
  `ParserBoxSourceSealV1`, final non-static box placement, and
  `ParserConstructorSourceCatalogV1` are issued by the parser postpass
  finalizer. Initial/final callable source transports the same product, or the
  canonical empty product when no retained constructor-capable box exists.
  Static target inventories and Builder constructor batches validate rows only;
  they do not create or infer an empty catalog.

Acceptance:
  `static box Helper { value() { return 7 } }` and its direct root-return form,
  top-level-only input, and mixed ordinary+static input parse as SourceBacked,
  transform unchanged, and reach `for_mir_mode_callable_source`. Zero
  constructor cohorts produce an empty catalog; ordinary constructor-bearing
  boxes preserve parser IDs; selected gates preserve retained rows; mixed
  ordinary+static keeps its outer compatibility envelope.
  Source seal/placement/catalog drift, duplicate/foreign rows, or transformed
  declarations reject before target/bundle issuance. Existing ordinary-box
  source tests remain unchanged. The reusable
  `script_direct_static_target_guard.sh` also pins the finalizer seam and the
  source-admission positive/negative tests.

Non-claims and stop:
  No bridge/claim/publication, route/lowering, compatibility fallback, raw
  retirement, constructor semantic change, Recipe/Join change, Script exit,
  Box/ABI change, or performance claim. If supporting static source requires
  AST reconstruction, mixed-cohort widening, or growth of a file to 760/800,
  stop and split the owner before editing.

### `SCRIPT-DIRECT-STATIC-PRE-DESCENT-P0` execution brief (closed `NoSafeSlice`)

Parent: `SCRIPT-DIRECT-STATIC-SOURCE-ADMISSION-I0`; design/census only. The
source-admission implementation is closed and the physical bridge remains
unopened.

Decision:
  The real selected-normal ScriptRoot ingress and joined handoff preserve exact
  `FinalSequence`/`RootReturn` source sites and ordered argument child sites.
  The route non-intersection premise is not closed: a legal static `is/as`
  method with one string argument can also be selected by the earlier typeop
  arm. Close this row as `NoSafeSlice`; do not modify source admission, route
  selection, lowering, or physical emission here.

Source authority:
  Existing source-bound Script semantic products, the installed selected-normal
  ingress, and the existing `MemberCallRoutePlan::StaticReceiver` observation
  are read-only witnesses. P0 must not mint a new `Verified*`/`Prepared*`
  receipt, infer a site from AST names/ordinals, or repair a missing relation.

Acceptance:
  A real static direct call in FinalSequence and a direct root Return each show
  one exact ScriptRoot/body or ReturnValue site before receiver/argument effects;
  each argument site is ordered and cardinality-exact; callable-package and
  direct Script ingress agree. Reserved, StaticThis, and bound/dynamic receivers
  are explicit noncandidates. The typeop negative fails, because the current
  issuer does not exclude the legal `is/as` one-string shape. Missing/foreign/
  unlocated/site-drift inputs stop before effects and never become `Absent`.
  Focused observation evidence is recorded in this card only.

Closeout:
  The source-site premise is green, but the route domains overlap without an
  effect-free classifier. `is` and `as` are not tokenizer keywords and static
  member declarations/calls accept ordinary identifiers; `build_method_call_
  from_input_v1` checks `is_typeop_method` before member planning, while
  `script_direct_static.rs` only filters reserved routes. Do not start the
  physical bridge from a local green test alone.

### `SCRIPT-DIRECT-STATIC-TYPEOP-DISJOINT-P0` execution brief (closed design)

Parent: `SCRIPT-DIRECT-STATIC-PRE-DESCENT-P0`; design/census only. The policy
owner and its exact delegation boundary are now fixed; no source admission or
bridge code is authorized by this closed row.

Decision:
  Establish one source-target/typeop contract before any receiver or argument
  effect. Typeop-shaped `is/as` calls must be explicit noncandidates for the
  direct-static target inventory; ordinary static `is/as` methods with
  non-typeop arguments must remain eligible.

Source authority + canonical issuer:
  The parser's identifier grammar and the existing `is_typeop_method` route
  predicate are the syntax/route witnesses. A later source target issuer may
  project only that typed disposition; it must not infer it from names alone.

Non-authority:
  declaration-catalog presence, reserved-route classification, AST text or
  ordinal, `ValueId`/`MirType`, and a physical bridge cannot decide whether a
  candidate is typeop-shaped.

Fail-fast boundary:
  Before receiver descent, argument descent, or MIR effects, the exact source
  row must be classified as `TypeOpNonCandidate` or `DirectStaticCandidate`.
  Missing/foreign/drifted observation is an error, never `Absent`; no ordinary
  fallback or retry is allowed after a typeop overlap is observed.

Smallest next slice:
  The focused design witness is `Helpers.is("Integer")`/
  `Helpers.as("Integer")` versus ordinary `is/as` with non-typeop arguments;
  reserved, StaticThis, bound, dynamic, and alias cases remain separate route
  negatives. This proof is complete and opens only the policy I0 below.

Non-claims:
  no parser grammar change, source admission, route implementation, Recipe/
  Join consumption, claim/publication, physical Call, Return/signature write,
  fallback, raw retirement, production switch, or performance result.

### `SCRIPT-DIRECT-STATIC-TYPEOP-DISJOINT-I0` execution brief (closed 2026-08-20)

Parent: `SCRIPT-DIRECT-STATIC-TYPEOP-DISJOINT-P0`; one behavior-preserving
BoxShape implementation. The physical bridge remains blocked.

Decision:
  Add one neutral source-method typeop policy sibling and make both the Builder
  typeop route and Script target issuer delegate to it. Preserve existing
  lowering order and do not create a second AST matcher or a source-admission
  shape.

Source authority + canonical issuer:
  `src/mir/policies/source_method_typeop_route.rs` issues the typed
  `TypeOp { kind: Is|As, type_name }` or `Ordinary` disposition. The
  existing `special_handlers::is_typeop_method` becomes a thin adapter, and
  `script_direct_static.rs` consumes the same policy before reserved-route and
  catalog checks.

Non-authority:
  literal method-name comparisons outside the policy, declaration-catalog
  presence, reserved-route checks alone, AST/name/ordinal re-pairing,
  `ValueId`/`MirType`, physical route selection, and fallback/retry.

Fail-fast boundary:
  The policy is pure and effect-free. Typeop-shaped `is/as` rows increment the
  existing explicit noncandidate inventory and never enter `targets`; ordinary
  `is/as` with non-typeop arguments follows the unchanged declaration lookup.
  Missing/foreign/drifted rows still reject and never become `Absent`.

Closeout:
  The policy module and focused tests landed; the Builder helper and Script
  issuer delegate to the same disposition, and the reusable target guard now
  checks the policy boundary plus both focused test families. Every changed
  Rust file stayed below 760 lines. No parser/source admission, Recipe/Join,
  physical bridge, fallback, or production switch changed.

Acceptance:
  `Helpers.is("Integer")` and `Helpers.as("Integer")` are explicit
  noncandidates; `Helpers.is(1)` and `Helpers.as(1)` remain target candidates;
  StringBox type names match existing Builder behavior; reserved, StaticThis,
  bound/dynamic, alias, foreign, and missing rows retain their existing
  dispositions. Builder typeop MIR output is unchanged.

Non-claims:
  no parser/source admission, MemberCall route rewrite, Recipe/Join consumer,
  claim/publication, physical Call, Return/signature write, raw retirement,
  production switch, or performance result.

### `SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-BRIDGE-D0` (closed boundary)

Change:
  At the ordinary `StaticReceiver` arm head, classify by exact bundle site and
  atomically move the matching Recipe/Join row into `Absent | Claimed | Err`.
  Remove `Clone` from the claimable row and expose no reinsert/rollback path.
  Reuse the ordered argument driver and unified generic Call receipt once, then
  publish ExactI64 through a Script-only sibling. On successful Script scope
  exit, consume the state and verify no Candidate row remains. Selected failure
  discards the invocation and cannot return to the ordinary handler.

Contract:
  `VerifiedScriptDirectStaticResultBundleV1` alone issues Candidate/Absent;
  `VerifiedScriptDirectStaticJoinHandoffV1` supplies the exact Candidate target,
  ordered sites, representation, and terminal witness. Handoff lookup failure
  never means Absent; AST text is drift-only. The bridge writes no Return or
  signature and selected-normal keeps its existing completion owners.

Done:
  Both terminal forms claim before effects, arguments lower left-to-right once,
  exactly one generic Call receipt is issued, and its destination is published
  as Integer before finalization. Absent does not mutate the ledger; Candidate
  without its handoff rejects; each handoff row is moved exactly once; every
  successful Script scope exhausts the Candidate set. Foreign/drift, duplicate
  claim/publication, alternate receipt, shortcut escape, and post-claim failure
  reject without rollback/retry; a fresh compile proves candidate isolation.

Stop:
  Do not grow `recursive_child_lowering.rs` (794), the 751/753-line owners, or
  add a second MethodCall matcher/argument driver. Keep every changed Rust file
  below 800 and split by owner at 760. If move-out or success-only finish needs
  a second authority, a generic transport redesign, or growth in a no-growth
  owner, keep the claim row at `SCRIPT-DIRECT-STATIC-CLAIM-LIFECYCLE-P0`.
  Source admission, canonical Script input, Box/ABI, raw retirement, cutover,
  and perf stay separate.

### `SCRIPT-DIRECT-STATIC-CLAIM-LIFECYCLE-P0` (closed design)

Decision:
  Do not start the physical bridge until the existing source products have one
  operational claim owner. Facts/Recipe/Join already issue the candidate seed,
  but `ScriptSemanticLoweringState` currently exposes only immutable accessors;
  it cannot prove one-shot consumption or scope exhaustion.

Source authority + canonical issuer:
  `VerifiedScriptDirectStaticResultBundleV1` issues site membership and
  `VerifiedScriptDirectStaticJoinHandoffV1` issues the complete target,
  ordered-sites, representation, and terminal row. The P0 ledger may co-seal
  those existing rows, but it must not become a second semantic source or
  invent `Absent` from a Join miss.

Operational contract:
  At `StaticReceiver` entry, inspect the active exact source site. Bundle miss
  is `Absent` and leaves the ledger unchanged; Bundle hit requires the same-site
  Join row or returns `Err`. `Claimed` moves the row into a non-Clone token;
  there is no reinsert, rollback, retry, or ordinary-route escape. The selected
  Script scope must call `finish` before restoring its parent ledger and require
  zero remaining candidate rows. A post-claim failure discards the isolated
  invocation rather than publishing partial MIR.

Smallest next slice:
  Design only the move/take token, route capability, scope finish/exhaust
  boundary, and fresh-scope isolation using the existing
  `Rc<RefCell<ScriptSemanticLoweringState>>`. Do not emit a Call, publish
  `ExactI64`, write Return/signature, or add a new semantic `Verified*` receipt.

NoSafeSlice conditions:
  A generic method-call trait is the only way to expose the claim and it cannot
  carry the exact site; state is dropped without finish verification; a row must
  be cloned/reinserted; a second AST/source matcher or canonical transport is
  needed; or Bundle/Join/active source cannot be co-sealed before effects.

Acceptance:
  Bundle miss -> `Absent`; Bundle hit plus complete same-site Join -> one
  movable candidate; missing/foreign/duplicate/drift -> `Err`; claim happens
  before receiver/argument descent; successful scope has zero candidates;
  compatibility/deferred lanes issue no claim; fresh scopes cannot observe a
  prior claim. Physical Call/publication remain unclaimed until this row closes.

### `SCRIPT-DIRECT-STATIC-CLAIM-LIFECYCLE-I0` execution brief (closed)

Classification: BoxShape. The ledger is operational state over already-issued
source products; it does not add a language shape or semantic authority.

Owner and transport:
  Add one focused sibling, `ScriptDirectStaticClaimLedgerV1`, and seed it once
  while constructing `ScriptSemanticLoweringState` from the existing Bundle and
  Join. A Bundle/Join partial pair, identity mismatch, cardinality mismatch,
  duplicate site, or foreign row fails before the Script scope executes. An
  empty pair is a valid empty ledger for a Complete Script with no candidates.
  Reuse the existing `Rc<RefCell<ScriptSemanticLoweringState>>`; do not add a
  second source transport or grow the 794-line recursive port.

Claim contract:
  `take(site)` returns `Absent`, `Claimed(non-Clone token)`, or `Err`. The token
  carries the already-sealed Join row and has no clone, reinsert, rollback, or
  retry API. `complete(token)` moves the site to a completed tombstone; a later
  take of that site is `DuplicateClaim`, not an ordinary-route `Absent`. The
  token exposes read-only target, ordered argument, representation, and
  required-callee views to a future physical consumer without re-issuing facts.

Finish/discard:
  `finish()` is a consuming operation that requires pending=0 and in-flight=0;
  the physical bridge will call it exactly once before
  `with_script_semantic_source_v1` restores its parent ledger. This carrier I0
  does not call it for non-empty candidates because doing so would fabricate a
  consumer. A lowering error in the future bridge skips finish and discards the
  isolated invocation; it never returns to the ordinary route. A fresh scope
  constructs a fresh ledger and cannot see a prior claim; completed tombstones
  are scope-local and never become semantic source data.

Acceptance:
  positive same-site Bundle+Join claim, empty Complete Script, absent site,
  missing/foreign/drift/duplicate rows, duplicate take, uncompleted token,
  pending row at finish, partial products, and fresh-scope isolation. The
  production state seeds the ledger and rejects malformed pairs; the unit
  ledger proves claim/complete/finish semantics, including the completed-site
  duplicate guard. No Call, ExactI64, Return, or publication effect is allowed
  in this I0; the later physical bridge consumes only completed claim tokens.

Stop:
  If exact-site capability cannot reach the StaticReceiver arm without a
  second matcher/transport, if finish can be bypassed, or if a claim must be
  cloned/reinserted, stop at this row and do not open the physical bridge.

### `SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-BRIDGE-D0` design brief (current stop)

Classification: BoxShape. The accepted source shape and existing Script
completion contract do not change; this row adds the missing selected-normal
physical consumer. It is not a canonical transport or production cutover.

Source-to-effect chain:

```text
VerifiedScriptDirectStaticJoinHandoffV1
  -> Script-only single-use claim
  -> existing ordered argument descent
  -> existing unified generic Call receipt
  -> Script ExactI64 publication sibling
  -> existing selected-normal completion owner
```

Implementation boundary:
  Enter only at the head of `MemberCallRoutePlan::StaticReceiver`, before
  `AssociatedMethodCallArgumentsV1`, receiver/argument descent, shortcut
  handling, or MIR effects. The claim API returns `Absent`, `Claimed`, or an
  error. `Absent` leaves the ledger untouched and uses the existing route;
  `Claimed` owns the candidate and cannot return to that route.

Authority contract:
  The Join handoff supplies canonical target, exact source/receiver/argument
  sites, `ExactI64`, and `FinalSequence|RootReturn`. The claim sibling validates
  the active Script source site and moves the row once. The unified emitter is
  the only Call issuer. A Script-specific publication sibling writes
  `MirType::Integer` once from the completed receipt destination. The bridge
  never writes Return/signature; selected-normal completion remains the owner.

Failure contract:
  Missing Candidate/Join, foreign or drifted site/target/arity/argument order,
  duplicate claim/publication, unsupported representation, alternate receipt,
  or unconsumed rows are hard errors. After claim, argument/emitter/publication
  failure discards the isolated invocation; rollback, retry, shortcut,
  ordinary fallback, and partial-success publication are forbidden.

Acceptance:
  FinalSequence and RootReturn each claim before effects; arguments lower
  left-to-right exactly once; one generic Call receipt and one Integer
  publication are observed before finalization; successful Script state has no
  Candidate rows; a fresh compile has no prior claim. Absent rows retain MIR
  parity. Focused positive/negative tests and one reusable structural guard
  cover site drift, duplicate claim, alternate route, post-claim failure,
  typeop/reserved/StaticThis nonintersection, and line limits.

Non-claims:
  no source admission, Recipe/Join redesign, canonical Script physical input,
  callable publication reuse, Box/ABI change, raw or compatibility retirement,
  production switch, Return-owner rewrite, performance measurement, or C-parity
  result. If a transport split, second AST matcher/driver, rollback mechanism,
  or >760-line semantic growth is required, stop and open
  `SCRIPT-DIRECT-STATIC-CLAIM-LIFECYCLE-P0` instead.

### Ordered continuation

```text
SOURCE-ADMISSION-I0
  -> PRE-DESCENT-P0
  -> CLAIM-LIFECYCLE-I0
  -> PHYSICAL-BRIDGE-D0
  -> CANONICAL-PHYSICAL-INPUT-D0
  -> canonical physical consumer
  -> production cutover + raw/compat caller-zero retirement
```

Only the final retirement row may claim one production Script physical owner.
The source-admission row is a BoxCount prerequisite; it does not authorize a
bridge implementation or a production route by itself.
