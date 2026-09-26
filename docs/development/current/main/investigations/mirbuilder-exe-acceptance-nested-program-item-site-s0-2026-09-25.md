# MIRBUILDER-EXE-ACCEPTANCE-NESTED-PROGRAM-ITEM-SITE-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D11 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: D11 Decision. `body_item_site`
  (raw_invocation_source_item_site.rs) is the single lane-side
  item-site projection; walk-registered `SourceNodeSiteV1`
  (`[stmt, ProgramBody(i)]`, rootless) is the canonical identity.

## Slice

1. `src/mir/builder/raw_invocation_source_item_site.rs`
   `body_item_site`: for `SourceBodyKindV1::Program`, drop the
   `ProgramBodyRoot` tail when the body-root site is nested
   (`site.segments().len() > 1`). The absolute script-root form
   (`[ProgramBodyRoot]`, len == 1) keeps
   `[ProgramBodyRoot, ProgramBody(i)]`.
   Both `body_statement` (context.rs:65) and `child_statement`
   (context.rs:283) consume this single projection — one change
   covers every nested-program statement/expr site on the lane.

## Overlap analysis (required, source-read)

- Walk registers nested-program items rootless:
  `resolve_program_block` -> `stmt_body_item_path(statement,
  path, ProgramBody, i)` -> `[path, ProgramBody(i)]` — all
  maps (`locals`, `variables`, `initializers`, `resolved_exits`,
  `statement_sites`) key on this form.
- The body-ROOT site `[stmt, ProgramBodyRoot]` stays rootful on
  both sides (`child_body` -> `append_root_path`; walk scope
  origin `[Body(0), ProgramBodyRoot]` pinned in
  scope_container_tests) — unchanged by this slice.
- Rootless kinds already drop their root via the same
  `is_rootless_item_site_kind` branch; this slice extends the
  identical collapse to nested Program roots — no new
  vocabulary, no consumer-side translation.
- Absolute `[ProgramBodyRoot]` (script root) is length-1 and
  keeps the rootful item form — required by script-root dispatch
  and the receipt packs' exact 2-segment patterns.

## Pins (required before close)

- Positive (body_item_site unit): nested
  `body_item_site(Program, [Body(2), ProgramBodyRoot], 0)` ==
  `[Body(2), ProgramBody(0)]`.
- Positive (existing): absolute
  `program_items_keep_the_explicit_program_root` unchanged —
  `[ProgramBodyRoot, ProgramBody(3)]`.
- Positive (integration): a `local` inside a nested `{ }` on the
  callable lane finds its `locals` row — `local_placement` no
  longer returns `placement-local-missing` for a nested-program
  local (fixture mirroring `{ local x = ... }` inside a
  cataloged function).
- Negative: `body_item_site(Scope, ...)` and the other
  rootless-kind behaviors unchanged; `Function` keeps its
  absolute `root_body` arm.
- Guard: extend `mirbuilder_qualified_route_scope_guard.sh` —
  pin the nested-Program collapse, the absolute-root exception,
  and the new test names; register touched files in the
  800-line list.
- Real app: json_stream_aggregator advances past
  `placement-local-missing` — record the next honest terminal.

## Fail-fast boundary

- Absolute script-root program keeps rootful items.
- Other rootful-capable body kinds (Lambda/Try/Catch) are NOT
  claimed by this slice — unprobed; if a nested form surfaces,
  it is a separate named boundary.
- Any non-`locals` lookup inside nested programs (`variables`,
  `initializers`, exits) now resolves through the same rootless
  site — a downstream map lookup failure is the next honest
  terminal, not a regression of this slice.

## Evidence (landed — filled at close)

- Implementation: `body_item_site` now routes through
  `is_rootless_item_site(kind, site)` — a `Program` body root with
  `site.segments().len() > 1` collapses `ProgramBodyRoot` before
  appending `ProgramBody(i)`; the absolute `[ProgramBodyRoot]`
  (len == 1) keeps the rootful 2-segment form. Single producer
  change; `body_statement`/`child_statement` unchanged.
- Unit pins (`raw_invocation_source_item_site`, 4 tests green):
  `nested_program_items_drop_the_program_body_root` —
  `[Body(2), ProgramBodyRoot]` + item 0 → `[Body(2), ProgramBody(0)]`;
  `program_items_keep_the_explicit_program_root` unchanged —
  `[ProgramBodyRoot]` + item 3 → `[ProgramBodyRoot, ProgramBody(3)]`.
- Integration pin (`map_local_tests`, 18 tests green):
  `nested_program_local_registers_rootless_statement_site` —
  `function caller() { { local inner = 1 } return 0 }` registers
  `locals` under `[Body(0), ProgramBody(0)]` and
  `local_initializer` resolves the row (no
  `placement-local-missing`).
- Guard: `mirbuilder_qualified_route_scope_guard.sh` extended —
  pins `is_rootless_item_site`, the nested-Program `len() > 1`
  collapse, both unit test names, and the map_local integration
  test; both touched files registered in the 800-line list
  (`raw_invocation_source_item_site.rs` = 124,
  `map_local_tests.rs` = 191). Guard green.
- Real app: `json_stream_aggregator` (debug binary, VM + EXE
  boundary) advances past `callable-semantic-lowering/
  placement-local-missing` — `local dbg` inside the bare `{ }`
  block resolves its `locals` row — and now reaches the next
  honest terminal:
  `[plan/freeze:contract] loop winner selection declined: zero
  selected family candidates` (loop family routing authority —
  D12 census material).
- Regression classification: `cargo test --lib raw_invocation`
  shows 8 `raw_invocation_port_preserves_*` /
  `collects_static_and_instance_box_methods` failures —
  identical 8 reproduce at HEAD with the change stashed
  (`raw-invocation/missing-expression-source-receipt`), all
  baseline debt. `normal_callable` 10 failures likewise
  baseline (unchanged from the review-verified HEAD set).

## Non-claims

- Does not change walk registrations or scope/region origins.
- Does not claim Lambda/Try/Catch/Cleanup nested item sites.
- Does not claim `StringHelpers`/`json_stream_aggregator` green.
