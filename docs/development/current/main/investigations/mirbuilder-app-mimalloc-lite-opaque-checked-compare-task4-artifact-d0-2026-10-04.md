# mimalloc-lite opaque checked-compare task-4 artifact D0 (dominated-view use coverage)

Status: TASK4-ARTIFACT-S0 landed (ad1fc53776) — dominated-view
  positions admitted end-to-end for `HakoAllocPage.allocate/1`; the app
  EXE lane advanced to the next designed boundary
  (`HakoAllocHandleResult` Birth plan — object-typed `handle` formal
  stored to a field). Frontier census + Decision recorded on
  `mirbuilder-app-mimalloc-lite-object-formal-field-store-d0-2026-10-04.md`.
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARTIFACT-D0`
Related:
  mirbuilder-app-mimalloc-lite-opaque-checked-compare-task4-d0-2026-10-03.md
  (parent; dominated-view series and the resolved Invoke emit census);
  mirbuilder-app-mimalloc-lite-opaque-checked-compare-normal-integer-d0-2026-10-03.md
  (grandparent; view contract and the task-4 `.set`/`+`/`new` scope
  statement);
  mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md (bundle
  inventory — slice 9 OPAQUE-SCALAR-POSITION keeps the `OpaqueHandle`
  rejection, slice 11 RESULT-NEW-OBJECT-ARGS names the `return`);
  mirbuilder-final-pipeline-ssot.md
  (`MIRBUILDER-INVOKE-LIFECYCLE-JSON-TERMINATOR-D0`).

## Frontier chain (observed 2026-10-04)

`apps/mimalloc-lite --emit-mir-json` now reaches only the designed
negative `unsupported terminator Invoke` — the generic MIR JSON lane
is exhausted by Decision
(`MIRBUILDER-INVOKE-LIFECYCLE-JSON-TERMINATOR-D0`: do not add `Invoke`
to `src/runner/mir_json_emit`; the lifecycle triplet serializes
through `published_backend_view/physical_program_json.rs`). The real
production lane is `--backend mir --emit-exe` (pure-first → lifecycle
V4 → LLVM C API → EXE, per `tools/smokes/.../mimalloc_lite_exe.sh`).
First observation on that lane:

```text
[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]
```

## Attribution (read-only worker census + temporary diagnostic, reverted)

- `HakoAllocPage.allocate/1` block 43 emits
  `Call{BirthConstructor HakoAllocHandle.birth/3, receiver:%130}` —
  the `return new HakoAllocHandle(me.page_id, block_id,
  requested_size)` at `page_heap_box.hako:102`. `lifecycle_bindings`
  for the owner is empty (`expected=[]`).
- The claim exists and is correctly owned:
  `LocalCommitV1::Result(NewResultCommitV1)` taken by
  `try_take_result_new_claim` (`raw_ordinary_new_claim.rs:511-535`,
  `ordinary_new_ledger.rs:211-268`) — but
  `prepare_result_new_emission` found
  `home_prefix = Err(PrefixNotCovered(Body(8)))`
  (`emission_prepare.rs:140-189`) → `NewEmissionProgress::
  RetainedUnavailable`.
- `Body(8)` = `me.requested_sizes.set(block_id, requested_size)`
  (L93): `requested_size` is an unannotated formal → installed
  `StoredLocal::Handle` → `OrdinaryObservation::Handle`, which
  `argument_subtree_neutral`
  (`home_new_prefix_field_call.rs:96-189`) deliberately rejects
  (bundle slice 9: "Keep rejection; select checked
  operation/borrow/Fault contract before widening").
- A retained claim emits via the raw lane (`lower_ordinary_raw_new_
  with_port_v1`, `ordinary_new_admission.rs:19-121`): raw
  `MirInstruction::NewBox` (not lifecycle-counted) plus
  `Call(BirthConstructor)` (lifecycle-counted,
  `invoke.rs:14-24`) — recording zero bindings by design. The
  coverage gate then correctly rejects: this freeze is the designed
  fail-closed terminal, not a bug.
- Pinned census test `page_heap_fixture_result_claim_census`
  (`brand_catalog_mixed_result_class_tests.rs:790-848`): 9 retained
  result claims; the `HakoAllocHandle` row is `construction()` +
  `argument_rows()` Ok, `home_prefix()` Err(`Body(8)`).
- `Emitted{bindings}` can never be empty
  (`record_new_emission`, `emission_prepare.rs:240-241`), so this is
  a retained claim, not a truncated record.

## Why the emit lane missed it

`--emit-mir-json` runs finishing validation with `artifact=false`
(`validate_after_compiler_finishing`) — `validate_artifact_lifecycle_coverage`
never executes, so "the whole app passes" on that lane ends at the
designed negative only. The production lane runs artifact coverage
per function; `allocate` is the first uncovered site.

## allocate's remaining dominated-view uses (parent-card scope)

`requested_size` (OpaqueHandle; view lent by `> me.block_size` at
L75, region dominated on the Normal edge):

- `.set` argument L93 — `Body(8)`, the direct blocker.
- `+` operand L96 (`me.requested_bytes + requested_size`).
- `new` constructor argument L102 (also bundle slice 11
  RESULT-NEW-OBJECT-ARGS).

## Census resolution — `home_prefix` coverage contract (2026-10-04)

`scan_statement_flow` walks each statement; an uncovered one poisons
every later selected/result row (`unavailable` propagates). For
`HakoAllocPage.allocate/1` the complete coverage census is:

Covered by existing rules (Body(0)..Body(7), Body(9)..Body(10),
Body(12) `if`, and the `return new` row):

- `if requested_size > me.block_size { return null }` —
  `contains_field_request` is false (`requested_size` Variable leaf;
  `me.block_size` is a `RootedHandle` receiver, not `GuardedFormal`)
  → the order-compare condition is unchecked; the `return null` arm
  is covered. The `me.` order-compare `if` at L98 behaves the same.
- `if me.free_top == 0 { return null }` — `==` selects
  `observe_scalar_expression`; the `me.free_top` request is proven
  scalar by `local_field_read`.
- `me.<i64|usize> = <scalar rhs>` writes (L83/94-95) —
  `observe_receiver_field_write`; `me.<field>` RHS reads proven by
  `scalar_field` (`receiver_scalar_field`, numeric-integer set).
- `local block_id = me.free_stack.get(me.free_top)` and
  `local use_count = me.use_counts.get(block_id)` —
  `observe_local_field_call`: `container_field` proves the `me.`
  `ArrayBox` field, `.get`/1 resolves in `CORE_METHOD_CONTRACT_ROWS_V2`
  to `I64Value`, args neutral → installs `TrivialLocal`.
- `me.block_used.set(block_id, 1)`,
  `me.use_counts.set(block_id, use_count + 1)` —
  `observe_statement_field_call`: `NoValue` manifest row +
  `argument_subtree_neutral` (Trivial leaves only).
- `if use_count > 0 { me.reuse_count = me.reuse_count + 1 }` — no
  field request → condition unchecked; arm is a covered field write.

Uncovered (the only two):

- `me.requested_sizes.set(block_id, requested_size)` (Body(8)) —
  `requested_size` leaf observes `OrdinaryObservation::Handle`;
  `argument_subtree_neutral` admits only
  Integer/Bool/Null/TrivialLocal/non-Home BoundValue.
- `me.requested_bytes = me.requested_bytes + requested_size`
  (Body(11)) — `observe_receiver_field_write` requires
  `is_trivial()` Variable leaves; `Handle` fails. Would be the next
  `PrefixNotCovered`.

`return new HakoAllocHandle(me.page_id, block_id, requested_size)`:
`argument_rows()` is already Ok — `SelectedNewArgumentKindV1::Handle`
admits the opaque formal leaf. No prefix work needed there.

## Census resolution — dominated-view draft layer (2026-10-04)

`draft_borrowed_formal_uses_v1` classifies `requested_size`'s four
uses; three already admit under the existing envelopes:

- `requested_size > me.block_size` (L76) → `CompareOperand`
  (`Greater(NormalInteger, NormalInteger)`; `me.block_size` proves
  `normal_integer_operand` via `receiver_scalar_field` on `usize`).
  Registers the compare guard — source-order dominance then covers
  all later uses in the same body sequence.
- `me.requested_bytes + requested_size` (L96) → `AddOperand` ✓.
- `new HakoAllocHandle(.., requested_size)` (L102) → `NewArgument` ✓.
- `me.requested_sizes.set(block_id, requested_size)` (L93) →
  `array_element_value_kind` **rejects**: the index arg `block_id`
  is a `Local` whose binding is not an opaque origin —
  `normal_integer_operand` admits only Integer literal / opaque
  origin / `me.<numeric field>`; a `.get`-result local falls to
  `UnresolvedArgument`. The graph-closure loop then finds `.set`
  absent from the lexical-instance `calls` map → marks the whole
  owner outside-profile → `definitions` drops `allocate` → **no
  lent view for `requested_size` at all** (compare/add/new
  admissions lost with it; the raw opaque lane continues).

The physical whitelist is already complete
(`borrowed_call_uses.rs`: `set_uses`/`add_uses`/`ctor_uses`/
`compare_uses` with dominance cones and exact coverage counts —
`ArrayElementWrite{Set}`, `BinOp{Add}`, `Call`/`Invoke`
`BirthConstructor`). The entry projection exposes all six kinds
(`BorrowedOrdinaryEntrySourceRefV1`).

## Decision — TASK4-ARTIFACT-D0

```text
Decision: Admit the three dominated-view use positions for
  `allocate`'s OpaqueHandle `requested_size` end-to-end: draft-side
  `.set` index proof plus prefix-side admitted-use consult.
Source authority + canonical issuer:
  `draft_borrowed_formal_uses_v1` stays the sole use-kind issuer
  (same lexical owner); `array_element_value_kind`'s index proof
  gains a `.get`-result arm reusing `receiver_array_field` +
  `CORE_METHOD_CONTRACT_ROWS_V2` (the same authorities
  `proven_field_call` consumes); `scan_statement_flow` stays the
  sole prefix-coverage authority, extended by one caller-supplied
  predicate — "the sealed draft admits a dominated-view value use
  at this exact leaf site" — consulted by `argument_subtree_neutral`
  and `observe_receiver_field_write`; `LocalCommitV1::Result` +
  `borrowed_call_uses` stay the claim/physical owners unchanged.
Non-authority: MIR value types, runtime layout, `.get` spelling,
  the generic MIR JSON emitter.
Fail-fast boundary:
  `.set` index without the manifest-proven `.get` initializer (or a
  rebound local) keeps `UnresolvedArgument`; a `Handle` leaf without
  an admitted `ArrayElementValue`/`AddOperand`/`NewArgument` draft
  row keeps `PrefixNotCovered`; unadmitted physical uses keep the
  existing `borrowed-use/*` faults.
Smallest next slice:
  MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARTIFACT-S0
  — implement both halves as one admission (draft index proof +
  prefix consult predicate), unit pins at both layers, then the real
  `mimalloc_lite_exe.sh` observation.
Non-claims: no `normal_integer_operand` global widening for
  compare/add siblings beyond the `.get`-result arm; no new use
  kind; no root/child terminal-parity change (Q3 stays open); no
  `.get` read lane, field-write lane, `Alias` read, method forward,
  or production switch.
```

## Ordered task

1. `MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARTIFACT-S0`
   — `.set` index `.get`-result proof in `array_element_value_kind`
   (sole initializer is a `me.<ArrayBox>.get`/1 with manifest
   `I64Value` result and no `BindingRebind`); `Handle`-leaf
   dominated-view consult threaded into `argument_subtree_neutral` +
   `observe_receiver_field_write`; draft/prefix unit pins; real
   `apps/mimalloc-lite` EXE-lane observation expecting `allocate`
   Result claim → lifecycle-bound `Call(BirthConstructor)` → next
   named terminal.
2. Open question kept: child-vs-root `RetainedUnavailable` terminal
   parity (Q3) — separate decision, not this slice.
3. Maintenance row: re-pin `mimalloc_lite_exe.sh`'s expected
   terminal (`emission-binding-drift` → current named terminal) in
   the S0 closeout.

## Non-claims

No `Invoke` JSON vocabulary, no `OpaqueHandle` widening, no `.get`/
`.set` named-array lane change, no new builtin-call authority, no
object-typed `Alias` read, no app EXE PASS or production switch claim.

## Landed — TASK4-ARTIFACT-S0 (<commit>)

Implementation (both halves of the accepted Decision):

- `get_result_index` in
  `ordinary_new_borrowed_formal_use_array_element.rs`: the `.set` index
  arm admits a local whose sole initializer is a `me.<ArrayBox>.get`/1 —
  `receiver_array_field` proof plus `CORE_METHOD_CONTRACT_ROWS_V2` row
  membership — with no `BindingRebind`. Correction to the D0 wording:
  the manifest row yields `Dynamic`, not `I64Value`; the arm proves
  row membership and the exact receiver, never a `Dynamic` element type
  — `normal_integer_operand` is unchanged.
- `PreparedBorrowedFormalIngressV1::dominated_view_sites`
  (`BTreeSet<OwnedExprSiteV1>`): dominated-view use sites
  (`ArrayElementValue`/`AddOperand`/`NewArgument`) recorded at
  draft-classification time, before borrowed-transport selection.
  `dominated_view_use_at` reads that classification set — not
  post-closure `definitions`. Reason, discovered at runtime: `allocate`
  is only invoked through `me.<field>` receivers, which never enter the
  lexical incoming map, so the `no-incoming-call` closure drop removes
  it from `definitions`. Dominated-view value uses are a
  source-classification fact independent of transport membership; the
  physical `borrowed_call_uses` whitelist (which does require the
  borrowed-entry profile) is untouched.
- `dominated_view_use_consult_v1` (shared helper in
  `ordinary_new_coseal_issue_source.rs`) supplies the `view_use`
  predicate to the verified walk and the source probe; bounded sibling
  lanes keep fail-closed `|_| Ok(false)` stubs. The predicate is
  threaded through `scan_statement_flow`, field calls, field writes,
  branch walks, new-home scans, and argument scans.

Evidence:

- `page_heap_fixture_result_claim_census` re-pinned: the
  `HakoAllocHandle` claim's `home_prefix` is now Ok (was
  `PrefixNotCovered(Body(8))`); the eight `HakoAllocHandleResult` rows
  keep `construction()` Err and the 6/2 prefix split.
- New pins in `ordinary_new_borrowed_formal_source_tests.rs`:
  `field_receiver_callee_keeps_dominated_view_sites_outside_transport`
  (owner absent from `definitions`, two dominated-view sites present)
  and `get_result_index_rejects_plain_copy_of_get_result`
  (plain-copy index yields no `.set` row; `+` row remains).
- Regression: `mir::normal_callable_semantic_package` 582/585 (same 3
  known baselines), `mir::resolved_semantics` +
  `mir::resolved_control_flow` 396/396.
- App lane (`--backend mir --emit-exe`, pure-first): advances past
  `allocate`'s `Call(BirthConstructor)` — now stops at owner slot 34's
  `HakoAllocHandleResult.birth/3` call, the designed
  `construction()`-Err boundary (object-typed `handle` formal stored to
  a field — `HakoAllocHandleResult.birth`'s `me.handle = handle`; bundle
  slice 11 RESULT-NEW-OBJECT-ARGS family).
- `--emit-mir-json` lane moved back to the designed negative
  `unsupported terminator Invoke` (the sealed claim cleared the earlier
  `emission-binding-drift` gate); `mimalloc_lite_exe.sh`'s negative pin
  re-pointed there per ordered task 3.

Non-claims held: no transport-profile widening (the closure still
drops `allocate`), no app EXE PASS, no production switch.

Guard record (`mirbuilder_qualified_route_scope_guard.sh`): the S0 pin
section covers `get_result_index` + `receiver_array_field` +
`CORE_METHOD_CONTRACT_ROWS_V2` + `BindingRebind` in the `.set` draft,
`dominated_view_sites`/`dominated_view_use_at` in the source map,
`dominated_view_use_consult_v1` in the shared helper, the `view_use`
predicate threading in `home_new_prefix_field_call.rs`/
`home_new_prefix_field_write.rs` (both added to the 800-line watch
list), and the two new test pins. Known baseline debt, not
current-change: `brand_catalog_tests.rs` sits at 961 lines — it was
already 961 at 29adaa7b84 and is untouched by this slice, so the
watch-list stop reproduces at the parent. It blocks
the guard from reaching later checks; every S0 pin and every touched
file's line count was verified individually green.
