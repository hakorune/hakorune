# MIRBUILDER-SCRIPT-CONTROL-SHAPE-DRAFT-COVERAGE-S12

**Row**: `MIRBUILDER-SCRIPT-CONTROL-SHAPE-DRAFT-COVERAGE-S12`
**BoxShape**: one production edge — make the sealed body-shape inventory
internally consistent for admitted script control statements (MatchControl
/ QMarkPropagation). No new admitted shape, no authority move.
**Mode**: `fast` — focused gates only.

## Evidence

`73db0233d7` (2026-08-06, `feat(resolved-semantics): seal source-site
inventory`) added `resolver.record_expression_site(path.expr())` to both
admitted control-statement dispatch arms in
`src/mir/resolved_semantics/shadow/script_root_dispatch.rs`
(`resolve_qmark_propagation`, `resolve_match_control`). The resolver
inventory therefore records the statement-level expression site
`[ProgramBody(i)]`, but neither arm records the matching draft shape row —
`seal_shadow_body_shape` then fails its draft-vs-inventory coverage check:

```text
DraftInvariant("body shape coverage does not match resolver source inventory")
```

Observed diff (temporary eprintln instrumentation, reverted after use):

```text
resolver exprs ⊋ draft exprs by exactly one site:
  SourceExprSiteV1([ProgramBodyRoot, ProgramBody(0)])
```

Both pinning tests are red since 73db0233d7 and never passed a stale-pin
classification — this is a live draft-coverage gap, not a stale test:

- `normal_script_semantic_source::match_tests::root_match_seals_all_child_sites_and_matches_legacy`
- `normal_script_semantic_source::qmark_tests::root_qmark_await_seals_the_exact_operand_and_matches_legacy`

## Fix

`record_expression_shape(statement, path.expr())` records
`ShadowExpressionShapeV0::Other { kind: <node_type> }` — the designed
catch-all shape row — giving the draft exactly the one site row the
resolver inventory already owns. Call it in both dispatch arms directly
after `record_expression_site`, so site inventory and draft stay
co-sealed by construction.

Boundary: no new admitted statement kind, no resolver relation or
source-row change, no semantic widening — the admitted set
(`match_control_sites`, `qmark_propagation_sites`) is untouched.

## Pins

- The two named tests green.
- `mir::builder::normal_script_semantic_source` module green.
- `mir::resolved_semantics` shadow tests green (site inventory unchanged
  in vocabulary).

## Non-claims

- Does not reopen script match/qmark semantics or admission breadth.
- Does not touch the retired raw-compat boundary.
- Gate-1 suite state unchanged.

## Landed evidence

- `script_root_dispatch.rs`: `record_expression_shape(statement, path.expr())`
  added to both `resolve_qmark_propagation` and `resolve_match_control`
  after `record_expression_site`. The recorded row is the designed
  `ShadowExpressionShapeV0::Other { kind }` catch-all — no new vocabulary.
- Focused: `normal_script_semantic_source::match_tests` +
  `qmark_tests` — 5/5 green (both previously-red pins now pass, nested
  Match deferral and qmark failure pins unchanged).
- `mir::resolved_semantics` module: 345/348 pass. The 3 residual reds are
  pre-existing baseline debt, not this change:
  - `accepted_vocabulary_is_closed_and_reviewable` — constant-list pin
    predates `0ac2b93e52` adding `"Program"` to
    `SHADOW_ACCEPTED_STATEMENTS_V0` (stale pin).
  - `forest_parent_rejects_unsupported_ancestry_and_orphan_scope` —
    same `IfThen`-ancestry stale premise class repaired in S9 (stale
    fixture, `440c16216d` made `IfThen` supported).
  - `resolver_seals_receiver_read_as_structural_upvar` — sibling live
    gap: `efbc22b14b`'s new `Me` seal arm accepts `Local` /
    `StaticCurrentOwner` but not the designed
    `ResolvedLexicalRefV1::Upvar` receiver ref; routed to a dedicated
    mini-slice (S13), not silently absorbed here.
