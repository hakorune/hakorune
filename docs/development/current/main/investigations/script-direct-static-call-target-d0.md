# SCRIPT-DIRECT-STATIC-CALL-TARGET-D0

Status: I0 complete; the observation-only implementation row is closed. Recipe
and physical rows remain unopened.
Parent: `brand-instance-constructor-source-relation-d0.md`

## Current capsule

- `1.15` remains the current performance acceptance ceiling; this row is not a
  performance or backend task.
- Deferred Script owner repair is `NoSafeSlice`; this row does not relabel it.
- The target catalog is a source product only. Recipe, Join, result publication,
  and physical call lowering stay closed until a later row.
- Every touched Rust source must stay below 760 lines; `owner_forest.rs`,
  `recursive_child_lowering.rs`, and package install are no-growth owners.

## Six-line brief

Decision: Accept one BoxCount prerequisite: issue an AST-free exact Script
caller/site-to-static-target catalog, without issuing a Recipe or lowering.

Source authority + canonical issuer: the retained Script ProgramBody occurrence
and its resolver traversal co-issue a Script-specific caller owner, exact call /
receiver / argument sites, and the existing canonical static callee key.

Non-authority: callable-owner keys, receiver/name/arity lookup, spans, `using`
spelling, Deferred status, raw success, and module result-publication ownership.

Fail-fast boundary: duplicate/foreign caller or site, alias/local collision,
overload mismatch, dynamic/instance receiver, nested-owner drift, or unknown
target/arity rejects before child effects and never falls back to raw lowering.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-TARGET-I0` adds only the source
catalog, admission retention, and focused positive/negative guards.

Non-claims: no Deferred owner repair, callable-key reuse, by-name fallback,
Recipe/Join issuance, physical switch, backend optimization, or promotion.

## I0 contract

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

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-RESULT-OWNER-D0` design only —
name a Script-specific source/result/publication owner first. After that design
closes, a separate Facts I0 may be considered; Recipe and physical R1/R2 stay
closed.

Non-claims: no Facts receipt implementation, Recipe/Join, result handoff,
physical lowering, raw-edge retirement, production switch, or C parity.
