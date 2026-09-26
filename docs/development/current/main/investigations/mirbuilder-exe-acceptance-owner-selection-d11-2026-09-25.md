# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D11

Status: design_stop__2026-09-25
Date: 2026-09-25
Parent: workstream row H (unified resume, gate 1)
Mode: design_stop — census and Decision only.

## Blocking observation

```text
[freeze:contract][callable-semantic-lowering/placement-local-missing]
```

`json_stream_aggregator` advanced past
`static-result-ingress/no-exact-static-target` (env.get now emits
ExternCall) and now stops at the next honest terminal inside the
callable-semantic lowering lane.

## Known context

- `local_placement` (map_local.rs) looks up
  `self.locals[statement][ordinal]`; `locals` is populated from
  `owner.declaration_sites()` → `SourceBindingSiteV1::Local` rows
  in the resolved-semantics declaration inventory.
- `placement-local-missing` = a `local` statement was lowered on
  the callable lane whose statement site is absent from the
  declaration inventory.
- Prime suspect: `local dbg = env.get("HAKO_STAGEB_DEBUG")` inside
  a **bare block** `{ ... }` in
  `lang/src/shared/common/string_helpers.hako` (`starts_with`,
  `starts_with_kw`). Bare-block locals may not produce
  `Local` declaration sites, or their site identity may differ
  from the lowered statement site.
- Second suspect: any local statement whose site key differs
  between the resolved walk's registration and the callable
  lane's `current_callable_site_v1` locator.

## Census questions

1. Which exact statement hits the freeze (function + site path)?
2. Does the resolved-semantics walk register `local` declarations
   inside bare `{ }` blocks? If yes, under which site key?
3. Is the gap a walk coverage issue (locals inside bare blocks
   never registered) or a site-identity mismatch (registered
   under a different key)?
4. Existing owner for nested-scope locals — is there a designed
   scope-resolution authority, or is bare-block local a newly
   observed shape needing a bounded slice?

## Bounded surface

Read-only census across `resolved_semantics` declaration-site
issuance, `CallableSemanticLoweringState` locals population, and
the callable lane's site locator. Does not touch the env route
(landed), the ordinary-new lane, or static-result ingress.

## Census results (main-investigator, 2026-09-25)

1. **Failing statement identified** (temporary freeze-site
   instrumentation, reverted): `local dbg = env.get(...)` inside the
   bare `{ }` (`ASTNode::Program`) at `Body(2)` of
   `StringHelpers.starts_with`. Queried site:
   `[Body(2), ProgramBodyRoot, ProgramBody(0)]`; registered
   declaration key: `[Body(2), ProgramBody(0)]`.
2. **Walk coverage exists**: `resolve_program_block`
   (resolved_semantics/shadow/stmt.rs) descends into nested
   `ASTNode::Program` bodies and registers locals — under the
   rootless item-site convention `[stmt_site, ProgramBody(i)]`
   (pinned: `standalone_program_has_exact_lexical_lifetime_...`,
   `program_block_inside_loop_...`). The scope/region origins are
   rootful `[stmt, ProgramBodyRoot]`; item sites deliberately drop
   the root segment.
3. **The gap is site identity, not coverage**: the lane projects
   body items via `raw_invocation_source_item_site::body_item_site`,
   which keeps `Program` "explicitly rootful" — designed for the
   absolute program root `[ProgramBodyRoot]` where no parent
   statement site exists. For a nested program it emits
   `[stmt, ProgramBodyRoot, ProgramBody(i)]`, diverging from every
   walk-registered map (`locals`, `variables`, `initializers`,
   `resolved_exits`, `statement_sites`) — all rootless.
   Both encodings project to the same AST node; they are two
   spellings of one statement.
4. **Consumers of rootful nested-program sites: none.** All
   `ProgramBodyRoot` pins in builder code/tests are the absolute
   2-segment `[ProgramBodyRoot, ProgramBody(i)]` script-root form.
   Every lane consumer that looks up walk-registered maps wants
   rootless — the rootful nested form is simply unreachable
   vocabulary at the map boundary.

## Decision (accepted 2026-09-25)

```text
Decision: normalize nested-Program item sites to the rootless
          canonical form at the single lane-side producer.
Source authority + canonical issuer:
          `raw_invocation_source_item_site::body_item_site` — the
          one item-site projection used by `body_statement` and
          `child_statement`. Walk-side `SourceNodeSiteV1`
          registration (rootless `[stmt, ProgramBody(i)]`) is the
          canonical identity it must match.
Non-authority: shadow `stmt_body_item_path` (already canonical),
          `append_root_path` (the body-ROOT site `[stmt,
          ProgramBodyRoot]` stays rootful — the walk's scope
          origin uses the same shape), any per-consumer site
          rewrite.
Fail-fast boundary: absolute `[ProgramBodyRoot]` (len==1, script
          root) keeps the rootful item form — unchanged; other
          body kinds keep their existing rootless behavior; a
          body-root site whose last segment is not the kind's
          root segment falls through to `child(item)` as today.
Smallest next slice:
          MIRBUILDER-EXE-ACCEPTANCE-NESTED-PROGRAM-ITEM-SITE-S0 —
          extend `is_rootless_item_site_kind` handling so a
          `Program` body root with a parent (site.len() > 1) drops
          `ProgramBodyRoot` at the item level; pins: nested
          Program item site = `[Body(2), ProgramBody(0)]`,
          absolute stays `[ProgramBodyRoot, ProgramBody(i)]`,
          end-to-end `local` inside `{ }` resolves placement.
Non-claims: no walk/declaration-inventory change, no scope-origin
          change, no other rootful body kinds (Lambda/Try/Catch)
          — unprobed, named boundaries if they surface.
```

## Exit

- [x] Failing statement identified; declaration walk covers
      bare-block locals — the gap is lane-side site spelling.
- [x] One bounded S-card emitted:
      MIRBUILDER-EXE-ACCEPTANCE-NESTED-PROGRAM-ITEM-SITE-S0.
