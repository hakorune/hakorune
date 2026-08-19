# SCRIPT-DIRECT-STATIC-CALL-TARGET-D0

Status: design accepted; implementation row is bounded and not yet started.
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

## Later rows (not opened)

1. `SCRIPT-DIRECT-STATIC-CALL-RECIPE-R1`: consume the target catalog into one
   Script Facts/Recipe call relation and exact argument/value keys.
2. `SCRIPT-DIRECT-STATIC-CALL-PHYSICAL-R2`: use the existing canonical call and
   result-publication owner, then retire only the selected raw Script edge.
3. Brand relationless lanes and `MethodCall/Brand.unwrap` remain independent;
   this row does not weaken their stop conditions.
