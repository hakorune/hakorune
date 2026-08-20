# SCRIPT-DIRECT-STATIC-CALL-TARGET-D0

Status: target I0, result-owner/Facts I0, continuation D0, continuation I0,
and ScriptRoot result-owner I0 are closed. Recipe D0 is the current design
stop; Join, physical, and raw retirement rows remain unopened.
Parent: `brand-instance-constructor-source-relation-d0.md`

## Current capsule

- `1.15` remains the current performance acceptance ceiling and `1.00` remains
  the formal long-term C-parity point target; a possible upper-95% `<= 1.03`
  claim is a separate future D0. This row is not a performance or backend task.
- Deferred Script owner repair is `NoSafeSlice`; this row does not relabel it.
- The target/result bundle, source continuation, and ScriptRoot result owner are
  Facts/source products only. Recipe and physical call lowering stay closed
  until their owners are named in this design stop.
- Every touched Rust source must stay below 760 lines; `owner_forest.rs`,
  `recursive_child_lowering.rs`, and package install are no-growth owners.

## Six-line brief

Decision: Accept one BoxCount source-product slice: retain the resolver-issued
Script body shape and issue an exact continuation inventory without issuing a
Recipe or lowering. The prior Recipe D0 remains NoSafeSlice until this product
and a Script-specific result-publication owner are available.

Source authority + canonical issuer: the resolver's existing
`ShadowBodyShapeDraftV0`/Script owner seal is the only candidate issuer for
statement, parent, Return, and expression relations. It must co-seal with the
same `VerifiedResolvedScriptV1`; the later Recipe producer alone issues Recipe
keys and rows.

Non-authority: scalar `RawScriptBodyRecipeV1`, AST re-projection, callable-owner
keys, receiver/name/arity lookup, spans, `using` spelling, Deferred status, raw
success, `ValueId`, MIR/ASM, and callable result-publication ownership.

Fail-fast boundary: missing/partial Script body-shape coverage, owner/root
drift, missing parent/Return/terminal relation, bundle/source mismatch,
unsupported result representation, Deferred/Compatibility/RawLegacy input, or
attempted Recipe publication without the Script result owner rejects before
Builder effects; no AST/name or raw fallback is allowed.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-I0`
retains the already-issued Script body shape through the forest/source handoff
and builds a focused source-only continuation sibling. Only after it closes
may `SCRIPT-DIRECT-STATIC-CALL-RECIPE-I0` be selected; scalar Recipe/Join/
physical code stays unchanged.

Non-claims: no Recipe-I0, Deferred owner repair, callable-key reuse, by-name
fallback, physical JoinSig inference, raw-edge retirement, backend
optimization, promotion, or C-parity claim.

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
5. **PARKED — `SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0`.** Define the dedicated
   Script Recipe producer/transport sibling before implementation. It must
   consume one source row per owner row and keep `RawScriptBodyRecipeV1`,
   `normal_source_plan::script_recipe`, and the existing scalar path unchanged.
6. **PARKED — Join/physical/result handoff.** After Recipe I0, open separate
   owner cards for source Join/continuation, Script result publication, and
   physical lowering. Never turn a physical `JoinSig`, `ValueId`, or block
   label into source meaning.
7. **PARKED — raw retirement/production.** Retire the old Script edge only when
   every admitted Script family has an exact Recipe/result owner and the new
   path is selected. Deferred, Compatibility, RawLegacy, and nested families
   remain explicit non-claims until separately closed.
8. **PARKED — performance.** Keep current exact/meso gates unchanged. Treat
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

Next design stop: `SCRIPT-DIRECT-STATIC-CALL-RECIPE-D0`. Define one Script
Recipe producer from this owner only after a separate design audit; do not
start Recipe/Join/physical/result publication or performance work here.
