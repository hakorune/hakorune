# MIRBUILDER-FIELD-ARRAY-GET-I64-RESULT-D0 — Page `me.<ArrayBox f>.get(i)` I64 result authority

Row: MIRBUILDER-FIELD-ARRAY-GET-I64-RESULT-S0
Parent card: `mirbuilder-stored-child-borrowed-call-receiver-d0-2026-10-05.md`
(its "Field Array.get I64 result integrated Decision" and "consumer-shape
refinement" sections are the accepted Decision and remain authoritative
history; this card carries the execution record).

## Integrated Decision (restated)

Read-only worker audit on the parent card established: the real app target
is `HakoAllocPage.block_used: ArrayBox` (page_heap_box.hako40; sites
110/142/170 all use the `handle.block_id` index). `HakoAllocPageModel`'s
`DirectArrayI64` fields are a different family — no `.hako` box exists for
it and `new DirectArrayI64()` providers stop at `FieldContractUnsupported`.

Decision: admit direct `me.<ArrayBox field>.get(<proven-i64 index>)` as an
armed i64 result only when the field's element-i64 is source-proven by a
whole-field integer-store census; keep manifest `Dynamic` otherwise.

- Source authority + canonical issuer: sealed MethodCall/FieldAccess/Me
  shapes, the field declaration, the sole birth provider store, and the
  owning-box integer-store census over the field (direct `me.<f>` and
  proven-alias writers). The refinement proved the named-array callable
  contract is loop-scoped (`Body`/`Condition` placement only), so the arm
  issues inside the existing prefix field-call manifest lane —
  `proven_field_call` in `home_new_prefix_field_call.rs` — as canonical
  issuer; same contract vocabulary (`CoreMethodResultKindV1::I64Value`)
  as every other manifest i64 row.
- Non-authority: manifest `Dynamic` row alone, i64 return annotations,
  MIR/ValueId tags, route-plan origin, `DirectArrayAccessPlan` metadata,
  `owned_object_residences`, class-name-only or physical-tag-only element
  claims.
- Fail-fast boundary: `PrefixNotCovered` for unproven receivers/indices;
  `source-not-i64` through `BorrowedFormalIngress` for an armed borrowed
  `return` of an unproven get source; vetoed fields keep `Dynamic`.
- Smallest slice: direct-receiver arm + whole-field census + the local and
  borrowed-result consumers of the armed contract.
- Siblings kept out: `handle.block_id` formal-field-read index admission,
  declared-DirectArrayI64 arm (blocked on the `FieldContractUnsupported`
  provider prerequisite), condition-position and nested-receiver
  admission, and the physical read owner for the `Callee::Method` get.

## FORMAL-INDEX cohort landed / 2026-10-06 (condensed)

`array_i64_fields.rs` census + FORMAL-INDEX-S0 landed earlier this card:
`me.<f>` fields declared `ArrayBox` with exactly one `new ArrayBox()`
birth-store and integer-source write coverage are proven; formal
`handle.<field>` reads join the integer-source leaf only when the field
resolves to exactly one non-weak `i64` declaration across the ordinary
box coverage. Evidence at the time: 15 catalog/view pins green,
regression battery 1158 PASS / 6 FAIL — all six identical on baseline
`36b13d8d8e` (classified known baseline debt); unchanged
`apps/mimalloc-lite` `--emit-exe` stopped at
`artifact-unowned-lifecycle-site` identical to baseline.

Non-claims preserved: `isLiveHandle`/`release`/`resizeInPlace` stayed
unadmitted; declared-DirectArrayI64 arm blocked on the
`FieldContractUnsupported` provider prerequisite; `Callee::Method` get
publication deferred to the physical read owner (landed below).

## `me`-receiver call coverage integrated Decision / 2026-10-06

Design-stop audit for `field-array-get/me-receiver-coverage-decision`
(read-only, same-thread; all uncertainty resolved against sealed source).
`me` reaches `call.receiver()` as `Lexical(Local(me_binding))` with
`record.kind() == BindingKindV1::Receiver` (`BodyMeReceiverV1::Lexical`;
static `me` is `CurrentOwner` and can never carry an instance call).
`prepare_lexical_source_targets_v1` drops it at the `_ => continue` arm,
so a name+arity match against an armed borrowed definition freezes
`draft_borrowed_incoming_calls_v1` on `UnresolvedCaller`. The receiver's
class needs no inference: it is exactly the caller declaration's own box,
readable from the caller's selected key.

Decision: admit the `Receiver` kind as a need in
`prepare_lexical_source_targets_v1` — class = `selected
.key_for_batch_slot(caller_slot).owner()` (the caller's own box),
`unique_instance_target(own_box, selector, arity)` — issued as the
existing `LexicalInstanceCallSourceTargetV1` with
`receiver = LexicalInstanceCallReceiverV1::Lexical(me_binding)`, so the
whole downstream chain (incoming-call draft self-edge, disposition,
emission) reuses the lexical path unchanged. Same slice: flip
`capture_field_array_get`'s `formal_i64_field` consult from `|_| false`
to the shared `formal_i64_index_consult_v1` — the two changes are one
production edge; either alone freezes the page-heap package
(`incoming-coverage` without coverage, `stored-child/result-source-
missing` without the armed leaf).

Source authority + canonical issuer: the sealed `Receiver` binding kind,
the caller's selected key owner, `unique_instance_target`; canonical
issuer `prepare_lexical_source_targets_v1` (one match arm).

Non-authority: receiver class from call-site actuals or layout;
`CurrentOwner` receivers (static `me`); `QualifiedUnbound`/`Other`
receivers — `stored_child_source_v1` keeps owning `me.<field>` receivers.

Fail-fast boundary: ambiguous or missing self target → `Ok(None)`
unarmed (structural, same as every declined class proof); the `rebound`
census stays uniform (a `Receiver` binding is never an assignment
target). Emission only activates where the caller-side scan mints an
observation — `local_lexical_i64_call` consults armed targets, so
`local x = me.allocate(size)` shapes can demand the i64 disposition and
`emit_local_lexical_i64` must accept a `Receiver`-kind binding through
`take_exact_lexical_read` + `prior_homes`; that acceptance is the
slice's focused-gate obligation, not a fallback site.

Smallest next slice: the `Receiver` arm + capture-leaf flip as one edge,
focused pins (positive `me.<def>` self-edge coverage, non-definition
`me.<name>` target arming, page_heap fixture issuing end-to-end,
negative ambiguous/undeclared selector), README + card receipt.

Non-claims: condition-position call-result admission (`if
me.isLiveHandle(h) == 0` keeps its dynamic result — coverage is
structural only), `QualifiedUnbound` receivers, `me` stores,
DirectArrayI64, physical read owner, production switch, app EXE
acceptance, finite goal.

## `me`-receiver coverage landed / 2026-10-06

`MIRBUILDER-ME-RECEIVER-CALL-COVERAGE-S0` landed on
`codex/birth-definition-publication`. What shipped vs the Decision:

- `Receiver`-kind bindings enter `prepare_lexical_source_targets_v1`
  exactly as decided; class = caller's selected key owner, target =
  `unique_instance_target`. `CurrentOwner`, `QualifiedUnbound` and other
  receiver shapes keep declining; the rebound census needs no arm (a
  `Receiver` binding is never assigned).
- Shipped shape differs on one point: the issued row carries a new
  `LexicalInstanceCallReceiverV1::SelfReceiver(binding)` variant instead
  of reusing `Lexical`. Reason found in implementation: `local x =
  me.m(..)` is *already* emitted by the sealed receiver-call lane
  (`ReceiverCallClassObservation` → `emit_receiver_nullable` →
  `record_handle_call_emission`). A self row that also routed a
  lifecycle binding group created a second bookkeeping owner for the
  same site and froze `local-commit/local-call-binding-sequence` on the
  real app. `SelfReceiver` rows are coverage-only: they name incoming
  self-edges (`lexical_instance_call_covered`, `UnresolvedCaller`
  resolution, terminal/dependency checks) but
  `issue_lexical_instance_call_dispositions` never calls
  `record_lifecycle_local_call_site` for them — one emission owner.
- The borrowed-result capture leaf stayed `|_| false`. Arming it was
  probed: `incoming-coverage`, `result-source-missing` and
  `local-call-binding-sequence` all resolve, but a grounded callee's
  named incoming edges include calls inside prefix-failed `if` branch
  subtrees (e.g. `return me.small_page.isLiveHandle(handle)` under an
  unprovable `handle.page_id == 0` guard) and condition-position calls —
  sites the borrowed-actual statement-flow walk never stages, freezing
  `borrowed-actual/selected-incoming-unobserved`. Staging actuals for
  unobserved-position edges is a separate responsibility.
- Evidence: focused battery 184 pass / 3 fail, all three pre-classified
  baseline debt (`direct_array_extent` refresh-links, `map_value_get` ×
  2). Self-edge pin `me_receiver_self_edge_covers_borrowed_definitions`,
  negative `me.take(me.block_used)` → named `borrowed-actual`,
  unresolved-incoming retained as named error. App probe
  (`mimalloc-lite --emit-exe`) with the leaf unarmed: frontier
  `artifact-unowned-lifecycle-site` — baseline parity, i.e. me-receiver
  coverage adds no app-visible regression by itself.
- Boundary for the armed leaf now recorded in
  `ordinary_new_borrowed_formal_result_pending.rs`: arm only together
  with unobserved-position incoming-edge staging.

Next owed: `MIRBUILDER-BORROWED-ACTUAL-UNOBSERVED-POSITION` — stage
borrowed actuals for named incoming edges whose callsite sits in a
prefix-failed branch subtree or condition position (then arm the formal
leaf in the capture), then physical read owner, production switch,
legacy retirement.

## Unobserved-position incoming edges integrated Decision / 2026-10-06

Design-stop audit for `borrowed-actual/unobserved-position-edge-staging`.
`observe_borrowed_call_actuals(site, locals, prefix_known)` already
stages any call site — nested argument calls ride its recursion,
identical re-staging is a no-op and drift freezes `repeated-walk-drift`;
`prefix_known=false` degrades non-literal actuals to `Unknown`, the same
fail-closed shape uncovered paths already use. Two positions never reach
it: (1) `observe_if_statement` returns early when a field-request
condition fails scalar observation — the verified `IfRegionBundleV1`
already proved branch structure, so branch interiors are skipped for no
structural reason and calls inside (e.g.
`return me.small_page.isLiveHandle(handle)` under
`if handle.page_id == 0`) stage nothing; (2) calls inside `if` condition
subtrees (`if me.release(handle)`, `me.isLiveHandle(h) == 0` operands)
are not statement `call_root`s and never stage. Both become named
`selected-incoming-unobserved` freezes the moment the armed formal leaf
grounds a callee.

Decision: borrowed-actual staging is owed for every source callsite
that incoming coverage can name. Walk the `if` branches even when the
scalar-condition observation declines — `PrefixNotCovered` is recorded
first and rides the fork, so interior actuals stage `Unknown` — and
stage the outermost calls under the `if` condition subtree through the
same `observe_borrowed_call_actuals` + `borrowed_actuals` callback.
Same slice: arm `capture_field_array_get`'s `formal_i64_field` consult —
the armed leaf is the production edge that names these edges.

Source authority + canonical issuer: sealed `method_calls()` site
inventory for condition-subtree membership; the verified
`IfRegionBundleV1` + body-match as the only branch-walk admission;
`observe_borrowed_call_actuals`/`borrowed_actuals` as the sole staging
issuer.

Non-authority: inferring the unprovable condition; treating `Unknown`
actuals as proven; `loop` bodies/conditions (no walk exists — a named
edge inside stays `selected-incoming-unobserved`); map-literal subtrees;
`bundle`-missing / `SourceMismatch` ifs.

Fail-fast boundary: `subtree_has_map_literal`, missing bundle and body
mismatch keep skipping interiors — unobserved edges there stay named
freezes, never silent skips; a `loop`-interior named edge likewise.

Smallest next slice:
`MIRBUILDER-BORROWED-ACTUAL-UNOBSERVED-POSITION-S0` — branch walk on
scalar decline + condition-subtree call staging + capture-leaf arm as
one edge; pins: formal-index borrowed return seals, condition-call edge
staged, return-in-unprovable-branch edge staged; negatives: `loop`
interior and map-literal-subtree edges keep `selected-incoming-unobserved`.

Non-claims: `loop` statement interiors/conditions, condition-position
result admission (stays dynamic), physical read owner, production
switch, DirectArrayI64, finite goal.

## Unobserved-position staging landed / 2026-10-06

`MIRBUILDER-BORROWED-ACTUAL-UNOBSERVED-POSITION-S0` landed. One revision
inside the slice: the Decision's "walk branches on scalar decline" ran
the emission-coupled walk and minted terminal-related facts on uncovered
paths — the real app froze `ordinary-new/local-commit/literal-physical-drift`.
Replaced with a staging-only traversal:
`stage_unobserved_statement_actuals` enumerates the sealed
`method_calls()` inventory under a source prefix, keeps outermost calls
only, and issues exactly one `observe_borrowed_call_actuals` +
`borrowed_actuals` callback per call on the pre-statement
`prefix_known` basis — no claims, terminal relations, Home joins, or
physical rows.

- `observe_if_statement` captures `prefix_known` once, stages the
  condition subtree, and stages the full statement subtree on
  bundle-missing / map-literal / scalar-decline exits; covered branch
  interiors are never re-staged.
- `scan_statement_flow` stages unadmitted statement kinds (loop,
  assignment, match, ...) before `PrefixNotCovered`. A `loop`-interior
  named edge now stages `Unknown` actuals and freezes
  `borrowed-actual`, not `selected-incoming-unobserved` — same
  fail-closed class, tighter token than the Decision sketched.
- Uncovered paths keep entry-stable bindings observable (`Parameter`,
  `Receiver`/self-rooted, trivial scalars); everything else is
  `Unknown`.
- New `BorrowedFormalActualSourceV1::DeclaredFormal`: a caller's sealed
  `DeclaredObject` parameter contract is the sole class authority for
  typed formals that `origins`/`forwards` never carried; wired through
  lexical-i64 prep, entry class validation, root-call-entry projection,
  and JSON transport (existing typed-handle tag 3).
- Companion fix surfaced by the walk: `emit_terminal_integer_literal_return`
  consumed the relation and emitted `Const` but owed no physical
  `Return` on the generic lane (`prepare_root_home_exit` is
  all-or-nothing per function), so `return 0` inside a covered `if`
  silently merged as a join yield. The site now completes through
  `emit_return_from_value`.
- `capture_field_array_get` consults `coverage_unique_i64_field` — the
  formal `me.<ArrayBox>.get(h.<unique-i64-field>)` leaf is armed.

Evidence: `unobserved_branch_incoming_edge_stages_borrowed_actuals`,
`unobserved_branch_incoming_edge_unproven_actual_stays_fail_closed`
(`borrowed-view`/`borrowed-actual` — the unknown actual never issues),
`condition_position_incoming_edge_stages_borrowed_actuals` green;
cohort 27+4+138+124 focused green. Frontier pins updated to the honest
boundaries (`me-receiver` -> `i64-result-mismatch`, `forward` ->
`source-not-i64`). App `--emit-exe` probe reaches
`artifact-unowned-lifecycle-site` = baseline parity with the leaf armed.
Baseline debt (reproduces on `36b13d8d8e`/`1c0d68497e`):
`direct_array_extent_fact::refresh_links_array_field_receiver_to_same_receiver_capacity_range`,
`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
`qualified_call_map_argument_reaches_the_named_capability_boundary`.

Next owed: physical read owner for `Callee::Method` get publication,
production caller switch, selected legacy retirement.

## Callee::Method get physical read owner integrated Decision / 2026-10-06

Design-stop audit for `physical-read-owner/callee-method-get-publication`.
The emitted `me.<ArrayBox field>.get(i)` is a `call_method` —
`Callee::Method{ArrayBox, get}` — whose dst type is `Unknown`; it has no
published row (`PublishedLifecycleCheckedOperationKindV1` has no
array-read, `validate_instruction_supported` whitelists only the checked
invoke vocabulary + bare `ArrayElementWrite`/field ops) so publication
freezes `instruction-unsupported`. Meanwhile `set`/`push`/`insert` already
own `MirInstruction::ArrayElementWrite` via
`try_emit_known_array_method_write(dst, receiver, method, args)` at three
sites — `boxcall_emit`, `unified_emitter`, plan `effect_emission` — and
the mir_interpreter executes it by delegating to
`execute_method_callee("ArrayBox", ..)`, i.e. the instruction is the
validated physical carrier while the runtime op is the same surface.
`ArrayBox.get` is `PureRead` (`invoke_surface` → `self.get(index)`): like
`field.get` it needs no fault edge and no lifecycle-validation row —
`field.get` is already a bare plain instruction outside the checked-op
vocabulary.

Decision: `Callee::Method{ArrayBox, get}` retires to a bare
`MirInstruction::ArrayElementRead { site_id, dst, receiver, index }`
minted by a sibling of `try_emit_known_array_method_write` at the two
emit sites `Callee::Method{ArrayBox}` actually reaches —
`unified_emitter` and `boxcall_emit` (plan `effect_emission` is
`push|set|insert`-gated on `RuntimeDataBox`; `get` never arrives) — a
shape swap on exactly the `call_method` sites where the receiver's
resolved box name is `ArrayBox` today, never an admission widening. The
dst keeps today's declared result type (the i64 seal rides the semantic
result lane, not the physical type — upgrading the physical dst type is
a follow-up).

Source authority + canonical issuer: `ArrayMethodId::Get` name+arity at
the existing emit arm (sole issuer); the published view collects
`array_element_reads` and C transport adds
`PublishedCallKindV1::ArrayGet` — receiver/index/dst — mirroring
`array_element_writes`.

Non-authority: selector-name matching at publication; inferring element
type from the instruction; `generic_method_route_plan`'s
`match_generic_get_route` (legacy plan for the remaining `call_method`
sites — unproven receivers only); `MapBox.get`; `DirectArrayI64`;
physical `Integer` dst typing; bounds-check semantics change.

Fail-fast boundary: a `get` whose receiver is not proven `ArrayBox`
keeps `call_method` and stays unpublished — `instruction-unsupported`,
unchanged. OOB stays the runtime's `ArrayBox.get` surface behavior
(`NullBox`/strict string, no Fault) — no new fault edge is minted, so
the read is not a `PublishedLifecycleCheckedOperationKindV1`.

Smallest next slice: `MIRBUILDER-ARRAY-ELEMENT-READ-OWNER-S0` —
instruction variant + printer/`array.read` + `Get` arm at the two emit
sites + `execute_array_element_read` (delegating to the same
`execute_method_callee` surface) + backend-mode coverage +
`validate_instruction_supported` admission + JSON transport +
`PublishedCallKindV1::ArrayGet` C row + the
`array_i64_get_call_stays_unpublished_until_read_owner` pin flips to a
positive published-row check. Negative pin: unproven-receiver get keeps
`instruction-unsupported`.

Non-claims: OOB/bounds semantics, `Integer` physical dst typing,
MapBox/DirectArrayI64 reads, non-`me` receiver element typing, production
switch, retirement, finite goal.

### ArrayElementRead owner landed / 2026-10-06

`MirInstruction::ArrayElementRead` is the sole physical owner of
`Callee::Method{ArrayBox, get}` sites. Sole issuer
`try_emit_known_array_method_read(dst, receiver, method, args)` fires at
the two `Callee::Method{ArrayBox}` emit arms; `Get/1` fails `from_name_and_arity`
for every other selector, so `MapBox.get` and unproven receivers keep
`call_method` untouched.

Landed surface:
- `array.read #N get receiver=%r index=%i` printer + `ArrayReadSiteId`;
  `EffectMask::READ`, `dst()`/`used_values`/`dst_value` arms, SSA
  `def_inst_kind`, `value_uses`/`query`/`value_consumer`/
  `value_consumer_used_values`/`JoinIrIdRemapper` arms.
- mir_interpreter `execute_array_element_read` delegates to the same
  `execute_method_callee("ArrayBox", "get", ..)` surface — instruction
  is the validated carrier, runtime op unchanged.
- MIR JSON emit `array_element_read` + `mir_json_v0` decode round-trip;
  `is_supported_mir_json_instruction`/`is_supported_vm_instruction`
  allowlists + `instruction_tag`/`instruction_diet_cohort` vocabulary.
- Published view `array_element_reads` rows +
  `PublishedCallKindV1::ArrayGet` (9) C transport; physical program
  `validate_instruction_supported` admits the bare read;
  `physical_program_json` encodes `array_get`.
- Shared recognizer `match_array_get_call` projects `ArrayElementRead`
  as `ArrayBox` get (same pattern as `match_method_set_call`'s
  `ArrayElementWrite` arm); `array_rmw_add1_leaf_seed` `op_name` +
  expected tables see `array_read`.
- Focused evidence: `array_i64_get_publishes_through_array_element_read`
  (view row + ABI input issue), `array_get_lowers_to_explicit_read_operation`
  (production lowering emits `array.read`, zero residual `call_method`),
  `vm_array_element_read_delegates_to_array_surface` (returns element),
  rmw/seed/observer plan tests green on the canonical form.

Boundaries honestly retained: the lifecycle-v4 C shim has no `array_get`
arm yet (`nyash.array.checked_get_*` export does not exist) — published
modules containing reads fail closed at C compile until that consumer
contract lands; `dst: None` reads encode `"dst": null` which the C
shape validator rejects. App `--emit-exe` probe (mimalloc-lite) stays at
`artifact-unowned-lifecycle-site` = baseline parity. Touched-region
battery: 19 failures, all reproduced identically on baseline
`858d00d09a` (corridor benchmarks, `mir_locals`, residence-release
tests, FFI-dependent object test) — zero current-change failures.

Next owed: `MIRBUILDER-ARRAY-READ-C-LIFECYCLE-CONSUMER-S0`
(Decision `physical-array-read/c-lifecycle-consumer`: kernel
`checked_get` export + v4 validate/flow/emit arms for the published
`array_get` row), then production caller switch and selected legacy
retirement.

## array_get v4 lifecycle consumer Decision / 2026-10-06

Design-stop audit for `physical-array-read/c-lifecycle-consumer` —
the published `array_get` row's C consumer in the v4 lifecycle lane.
The audit revises the previous next-owed sketch: no `checked_get`
kernel export is owed, and no diagnostic site is minted — the read
stays a plain op in the checked lane exactly as the landed Decision
recorded.

Audit findings:

- `nyash.array.slot_load_hi(handle, idx)` already exists
  (`array_slot_load.rs` → `array_get_index_encoded_i64`): i64-or-handle
  `MixedI64OrHandle` carrier, miss → `0`. It is the *same* surface the
  generic lane already emits for `call_method{ArrayBox, get}` via the
  generic route descriptors — one runtime surface, zero new semantics.
- The v4 lane's null sentinel is `i64 0` on the wire (`const_null` →
  `add i64 0, 0`), so `slot_load_hi`'s miss → `0` is the faithful
  NullBox encoding — no fault is owed on OOB. Read-miss → `0`/Normal
  is also the lane's own convention (`map_checked_get` writes `0` on a
  missing key; `array_set`'s bounds fault is a write-contract property,
  not a read one).
- `object_field_get`/`nyash.object.type_id_h` establish that plain
  (non-frame) calls sit in the v4 emit — a non-faulting read needs no
  `site`, no `PublishedLifecycleCheckedOperationKindV1` arm, and no
  change to `issue_diagnostic_sites`.
- The `array_get` JSON already emitted —
  `{"op","array","index","dst"}` — matches the required plain-op shape;
  `dst: None` encodes `"dst": null`, which the C validator's
  `hako_physical_u32` rejects — fail-closed boundary is already honest.
- v4 wiring needed: `physical_v2` validate arm (exact keys
  `{"op","array","index","dst"}`, `array`/`index` u32 +
  `value_available_at`, `dst` u32), `v4_index` seed arm (`array_get`
  dst → `LV4_I64` — the lane's i64/handle carrier; an encoded handle
  result flows as opaque i64 and downstream live-lease consumers stay
  fail-closed exactly as the `call_method` lowering today),
  `v4_indexed_flow` arm (mirror `array_set`'s operand rules: `array`
  and `index` `LV4_I64`, not faulted/birth, `lv4_slot` present for the
  dst), `v4_emit` arm (`%v<dst> = call i64 @nyash.array.slot_load_hi`
  — both native and non-native declare lists gain the declare).
- Lane A (`hako_llvmc` published rows): `PublishedCallKindV1::ArrayGet`
  rows reach `valid_kind` in `hako_llvmc_ffi_published_static_method.inc`
  and are rejected today — `valid_kind` + `take_array_read_row_v1` +
  an `EMIT_PUBLISHED_ARRAY_READ_ROW` (same `slot_load_hi`) is a sibling
  consumer boundary, not this slice.

Decision: `array_get` stays a PLAIN op in the checked lane and emits
the lane's existing `slot_load_hi` surface — no `checked_get` export,
no `site`, no checked-op kind.

Source authority + canonical issuer: the v4 shim's `array_set` sibling
arms — `physical_v2` validate, `v4_index` seed, `v4_indexed_flow`,
`v4_emit` + declares — plus the `slot_load_hi` runtime alias (sole
runtime surface shared with the generic lane).

Non-authority: `checked_get_i64_v1` kernel export (the lane's read
surface exists); fault recording on OOB (read-miss → `0` convention);
`site`/`ArrayRead` checked-op kind; `hako_llvmc` row admission
(sibling); box-element handle-lease reads; bounds semantics change.

Fail-fast boundary: `dst: None` reads → `"dst": null` → C validate
rejects (`hako_physical_u32`); `array_get` reaching a C lane without
the arms stays `return 0` / `malformed typed row` — fail closed as
today; unproven receivers keep `call_method` → unpublished upstream.

Smallest next slice: `MIRBUILDER-ARRAY-READ-C-LIFECYCLE-CONSUMER-S0` —
the four v4 arms + declare lines + `array_get` entries in
`mirbuilder_qualified_route_lifecycle_scope.inc.sh` + one execution
test mirroring `published_lifecycle_v4_set_view_execution_test.py`.
No kernel or Rust semantic change.

Non-claims: lane-A `hako_llvmc` row consumer (`valid_kind` +
`take_array_read_row_v1` + emit arm), `dst:None` dead-read transport,
box-element lease provenance, bounds-fault semantics, production
switch, app-frontier movement (still
`artifact-unowned-lifecycle-site`).

## array_get v4 lifecycle consumer landing / 2026-10-06

Slice `MIRBUILDER-ARRAY-READ-C-LIFECYCLE-CONSUMER-S0` landed as planned,
with two implementation-time corrections recorded here:

1. The planned `lv4_slot` destination check was dropped — `lv4_slot`
   resolves `object_id`/`field_ordinal` layout slots and does not apply
   to `array_get` (it returned <0 on every read). Destination validity
   is already covered by physical validation: `dst` u32 + the unique
   def registration through `hako_physical_instruction_dst`.
2. Two more consumer touch points were required beyond the four named
   arms — both inside the same fail-closed surfaces, no new vocabulary:
   `hako_physical_instruction_dst` (the value-registration whitelist in
   `physical_v2.inc`, without which `copy src=<dst>` stays unavailable)
   and `lv4_type` in `v4_admission.inc` (def-based classification,
   `array_get` → `LV4_I64`).

Landed arms:

- `physical_v2.inc`: `array_get` in `instruction_dst`; exact-key arm
  `{"op","array","index","dst"}` with operand availability.
- `v4_admission.inc`: `lv4_type` classifies `array_get` → `LV4_I64`.
- `v4_index.inc`: seed classifies `array_get` → `LV4_I64`.
- `v4_indexed_flow.inc`: arm mirrors `array_set` operand rules minus
  `value`/`lv4_slot` — `array`/`index` `LV4_I64`, not faulted/birth.
- `v4_emit.inc`: `%v<dst> = call i64 @nyash.array.slot_load_hi(i64
  %v<array>, i64 %v<index>)`; declare added to both native and
  non-native declare lists.
- `mirbuilder_qualified_route_lifecycle_scope.inc.sh`: `array_get` /
  `slot_load_hi` pins + exec-test existence.

Evidence:

- `array_i64_get_publishes_through_array_element_read` emits
  `hako-issued-get-view-{ok,oob}.json` and pins the row shape
  (`dst`/`array`/`index` u64, no `site`).
- `published_lifecycle_v4_get_view_execution_test.py` drives both
  fixtures through driver → object → probe: `ok` exits 7 (slot read),
  `oob` exits 0 (miss → null sentinel); `dst:null` and extra-`site`
  mutated rows reject at validation — no Fault in normal runs.
- Regression: `set_view`, `checked_compare`, `add_view` execution
  tests unchanged; all four `array_i64_field_call_tests` green.
- The route-scope guard's pre-existing 800-line violation
  (`normal_default_root_catalog_lifecycle_tests.rs` = 1351 at HEAD)
  predates this slice — unchanged baseline debt.

Non-claims unchanged: lane-A `hako_llvmc` row consumer, `dst:None`
transport, lease provenance, production switch, app frontier
(`artifact-unowned-lifecycle-site` parity).

Next owed: `MIRBUILDER-ARRAY-READ-LANE-A-CONSUMER-S0` — the Lane-A
`hako_llvmc` published-row consumer: `valid_kind` +
`take_array_read_row_v1` + static row emitter over the existing
`PublishedCallKindV1::ArrayGet` transport.

## array_get Lane-A consumer Decision / 2026-10-06

The `hako_llvmc` static published-row lane (Lane A) is the second
consumer of `PublishedCallKindV1::ArrayGet` rows — Rust transport
`c_transport.rs` already emits `kind=9` rows
(`receiver`/`index`/`dst`/`site_id`, `INDEX_PRESENT` always,
`DST_PRESENT` when the read has a dst). The C side rejects kind 9 at
`valid_kind` today (`malformed typed row`), so any published program
carrying an `ArrayElementRead` fails closed before object emission.

Runtime surface is already decided: `nyash.array.slot_load_hi`, whose
Lane-A declare exists as `needs.arr_slot_load`
(`prescan.inc:642` — `declare i64 @nyash.array.slot_load_hi(i64,i64)`
with readonly attrs). `MIR_CALL_NEED_ARRAY_GET` already sets
`arr_get + arr_slot_load`, so the read arm mirrors the existing
call_method need bundle — no new runtime symbol, no new declare list.

Required wiring, all inside the existing fail-closed surfaces:

1. `hako_llvmc_ffi.h`: `HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_GET 9u`.
2. `published_static_method.inc`:
   - `valid_kind` admits `ARRAY_GET`;
   - payload = `target_symbol == NULL && arity == 0` (the generic
     else-branch already covers it);
   - new `take_array_read_row_v1` mirroring `take_array_write_row_v1`:
     peek by exact site → kind `ARRAY_GET` → instruction
     `op == "array_element_read"` with u32 `site_id`/`receiver`/`index`/
     `dst` equal to row fields, `INDEX_PRESENT` and `DST_PRESENT` flags
     required → one exact-site take. `dst: null` (no `DST_PRESENT`)
     stays malformed — the dead-read transport non-claim holds.
   - `HAKO_LLVMC_EMIT_PUBLISHED_ARRAY_READ_ROW` macro:
     `%r<dst> = call i64 @nyash.array.slot_load_hi(i64 <recv>, i64
     <idx>)` through `append_i64_arg_ref` (copy/const resolution shared
     with the write macro).
3. Emit sites (mirror `array_element_write` exactly):
   - `pure_compile_generic_lowering_op_dispatch.inc`: required arm —
     non-READY is `GEN_ABORT` (the row always accompanies the op).
     `set_type(dst, T_I64)`.
   - `same_module_typed_field_rmw_emit.inc`: optional arm — ABSENT
     falls through, MALFORMED is `-1`, READY emits and returns 1.
   - `pure_compile_generic_lowering_prescan.inc`: `needs.arr_get =
     needs.arr_slot_load = 1` for `array_element_read` — the same
     declare bundle as `call_method` `ArrayBox.get`.
4. `published_rows_preartifact_test.c`: positive take + second-take
   reject + wrong-kind/op/shape rejects (mirroring
   `test_array_row_rejects_second_take`).
5. Scope pins in `mirbuilder_qualified_route_lifecycle_scope.inc.sh`.

Decision: Lane A consumes `array_element_read` through the published
`ArrayGet` row → one exact-site take → `slot_load_hi` emit; the read
stays a plain op (no site-diagnostic consumer, no checked kind).

Source authority + canonical issuer: `published_static_method.inc`'s
row admission/take + the two physical instruction walkers; the row
itself is issued by `c_transport.rs` from `array_element_reads`.

Non-authority: kernel changes; a second runtime read surface;
`dst:None` dead-read transport; selector-name dispatch; MapBox.get.

Fail-fast boundary: missing/duplicate/mismatched row →
`malformed typed row` / `GEN_ABORT` / `-1`; `dst` absent → malformed
at take; unproven receivers stay upstream `call_method`.

Smallest next slice: `MIRBUILDER-ARRAY-READ-LANE-A-CONSUMER-S0` — the
five items above plus an end-to-end `--emit-exe` probe of an
`ArrayElementRead`-carrying program if the lane is reachable.

Non-claims: production caller switch; selected legacy retirement;
app-frontier movement beyond the existing
`artifact-unowned-lifecycle-site` baseline; `dst:None` transport.

## array_get Lane-A consumer landed / 2026-10-06

`MIRBUILDER-ARRAY-READ-LANE-A-CONSUMER-S0` landed — the `hako_llvmc`
static published-row lane now consumes `PublishedCallKindV1::ArrayGet`
rows end to end:

- `hako_llvmc_ffi.h`: `HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_GET 9u`
  matches the Rust transport discriminant.
- `published_static_method.inc`: kind-9 admission plus an exact payload
  pin (`INDEX_PRESENT | DST_PRESENT`, `dst != UINT32_MAX`, `value`
  empty) — a dst-less dead read is rejected at the typed-row boundary,
  never repaired at emit time. `take_array_read_row_v1` does one
  exact-site take: function/block/instruction coordinate →
  `op == "array_element_read"` with u32 `site_id`/`receiver`/`index`/
  `dst` all equal to row fields and no stray `value`/`kind` keys →
  mark consumed. `HAKO_LLVMC_EMIT_PUBLISHED_ARRAY_READ_ROW` emits
  `%r<dst> = call i64 @nyash.array.slot_load_hi(...)` through the shared
  `append_i64_arg_ref` copy/const resolution.
- `op_dispatch.inc`: required arm — a non-READY take is
  `published_array_read_row_mismatch` → `GEN_ABORT` (no fallback).
- `same_module_typed_field_rmw_emit.inc`: the same take+emit pair for
  the same-module instruction walker.
- `prescan.inc`: `array_element_read` publishes `needs.arr_get +
  needs.arr_slot_load`, the same declare bundle the generic
  `call_method` `ArrayBox.get` route already used — one declare list,
  no second symbol-resolution authority.
- `published_rows_preartifact_test.c`: `test_array_read_row_take`
  covers the happy take, duplicate-take rejection, wrong-kind
  malformed, and dst-less boundary rejection.
- `static_v2_array_read_execution_test.py` (new): MIR+row fixtures
  drive the real static-v2 ABI — stored index exits 7, out-of-range
  index exits 0 through the same null sentinel; missing row, wrong
  kind, dst-less flags and missing `index` field all reject; the
  object's IR contains the `slot_load_hi` call.
- `mirbuilder_qualified_route_lifecycle_scope.inc.sh` pins all of the
  above.

Boundary discovered during verification, recorded honestly: the
same-module *prepass* whitelist does not admit `array_element_*` ops at
all — `array_element_write` already faced exactly that gate before
this slice. The nested case therefore stops at
`module_generic_prepass_failed` (pinned as a negative in the execution
test); same-module admission for array element ops is a separate
upstream decision, not silently bypassed here. The rmw walker arm is
wired so the row contract is identical in both walkers the moment that
admission arrives.

Evidence: `static_v2_array_read_execution_test.py` 8/8 (2 execute +
6 fail-closed/pinned), `published_rows_preartifact_test` PASS,
`published_lifecycle_v4_get_view_execution_test.py` 5/5 regression,
`cargo test --release --lib array_i64` 27/27.

Next owed: `MIRBUILDER-ARRAY-READ-SAME-MODULE-PREPASS-S0` — audit the
same-module prepass whitelist for `array_element_*` ops (the boundary
is shared with `array_element_write`, so admission is one family
decision, not a read-only patch). After the consumer surface closes,
the remaining frontier items — the production caller switch, selected
legacy route retirement, and whole-goal acceptance — continue under
`MIRBUILDER-FINAL-PIPELINE-v1`.

## array_element_* same-module prepass Decision / 2026-10-06

The Lane-A same-module walker is the physical emitter for nested
functions (`emit_same_module_function_definitions` runs before the
entry body), so a proven `me.<field>.set/get` inside a same-module
method hits `module_generic_prepass_failed` today — the prepass
whitelist contains no `array_element_*` arm at all. The boundary is
shared: `array_element_write` was never admitted there either, so this
is one family admission, not a read-only patch.

Precedent is already in-file: `hako_llvmc_published_intrinsic_array_peek_v1`
validates a row + full instruction shape WITHOUT consuming it;
`intrinsic_array_take_v1` is `peek + take_row`; the `newbox` prepass
arm accepts on `peek > 0` and rejects otherwise. The same pattern fits
exactly:

1. `published_static_method.inc`: extract
   `peek_array_write_row_v1` / `peek_array_read_row_v1` — each is the
   corresponding take's full shape validation minus consumption; both
   takes become `peek + take_row` so the shape contract stays in one
   owner (mirrors `intrinsic_array_peek_v1`/`take_v1` split).
2. `same_module_prepass.inc`: `array_element_write` accepts on write
   peek READY; `array_element_read` accepts on read peek READY and
   registers `set_type(dst, T_I64)` (mirroring `field_get`). ABSENT or
   MALFORMED peeks return 0 → the existing
   `module_generic_prepass_failed` hard stop — no row-less admission,
   no fallback.
3. Emit unchanged: the rmw walker arms already take the same rows, so
   consumption stays exactly-once; `rows_finish` still rejects
   residual rows.
4. `static_v2_array_read_execution_test.py`: `nested-prepass-boundary`
   flips to positive stored/oob execution through the same-module
   walker; a nested no-row case keeps `module_generic_prepass_failed`
   as the absence boundary.
5. Scope pins: peek owners, both prepass arms, nested exec cases.

Decision: admit `array_element_*` to the same-module prepass through
the existing peek-row precedent; one family admission covering the
previously unwired write arm as well.

Source authority + canonical issuer: `same_module_function_prepass_instruction`
whitelist + the shared peek/take owner in `published_static_method.inc`;
rows remain issued by `c_transport.rs`.

Non-authority: entry-walker changes; new row kinds; direct-array plan
fusion (`match_array_slot_direct_op_plan` only consumes `mir_call`
shapes, no overlap); lifecycle-v4 lane.

Fail-fast boundary: absent/malformed row → prepass return 0 →
`module_generic_prepass_failed`; emit-time take stays the consumption
authority; `rows_finish` residual check unchanged.

Smallest next slice: `MIRBUILDER-ARRAY-READ-SAME-MODULE-PREPASS-S0` —
the five items above.

Non-claims: production caller switch; selected legacy retirement;
entry-path behavior; direct-array fusion routes; `dst:None` transport.

## same-module prepass admission landing / 2026-10-06

`MIRBUILDER-ARRAY-READ-SAME-MODULE-PREPASS-S0` landed as the accepted
Decision above — one family admission, read and write together:

- `published_static_method.inc`: `hako_llvmc_published_array_write_peek_v1`
  and `hako_llvmc_published_array_read_peek_v1` own the full site-shape
  validation; both take functions are now `peek + take_row`, so prepass
  admission and emit-time consumption share one shape owner (the
  `intrinsic_array_peek_v1`/`take_v1` split, generalized).
- `same_module_prepass.inc`: `array_element_write` accepts on write
  peek READY; `array_element_read` accepts on read peek READY and
  registers `set_type(dst, T_I64)`. ABSENT/MALFORMED → return 0 →
  `module_generic_prepass_failed`. No row-less admission.
- `static_v2_array_read_execution_test.py` 11/11: entry + nested
  stored(7)/oob(0) execute through `slot_load_hi`; `nested-prepass-no-row`
  keeps the absence boundary; malformed rows still fail closed.
- `published_rows_preartifact_test` all PASS — including the
  second-take `found == NULL` contract (take now NULLs `out_row` before
  peek, regression caught and fixed in-slice).
- `static_v2_execution_test.py` full suite green (formal, control,
  copy, nested, backedge, operation-True same-module paths, original,
  boxed, entry) — no write-family regression.
- Scope pins added for both peek owners, both prepass arms, and the
  nested execution cases.

Environment repair recorded honestly: `target/release/libnyash_kernel.a`
was transiently rebuilt `--no-default-features` during verification.
The canonical archive is the default `legacy-entry` build — required
for native exe linking (`--emit-exe`/`ny-llvmc`) — so the probe-linked
suite instead uses a separately built no-main archive
(`--no-default-features --features lifecycle-core`, own CARGO_TARGET_DIR).
A minimal `static_v2_ny_main_entry.c` supplies `main` for plain
value-returning fixtures since the trap-observation probe entry is
specialized. `TMPDIR` pointed at the workdisk after a transient
/tmp-full link failure; all are environment artifacts, not lane
changes.

Next owed: `MIRBUILDER-ARRAY-READ-PRODUCTION-CALLER-S0` — audit whether
a real production caller (app `--emit-exe` / host static invocation)
reaches an `ArrayElementRead` site through publication into the Lane-A
or v4 consumers; the earlier app frontier froze upstream at baseline
(`artifact-unowned-lifecycle-site`), so reachability is a design-stop
question, not an assumed one. Remaining frontier items — the caller
switch, selected legacy route retirement, and whole-goal acceptance —
continue under `MIRBUILDER-FINAL-PIPELINE-v1`.

## production caller reachability Decision / 2026-10-06

Audit result — the routing spine already reaches `ArrayElementRead`:

- `is_lifecycle_instruction` does not cover `ArrayElementRead`/`Write`;
  a module carrying `array_element_reads` (and no lifecycle instruction,
  no `has_non_lifecycle_unsupported`) routes `CanonicalTyped` →
  `emit_published_view_body` → `compile_published_static_v2` → the
  landed Lane-A kind-9/`slot_load_hi` consumer. A module that also
  carries lifecycle instructions takes the v4 lane where the landed
  `array_get` consumer emits the same `slot_load_hi`.
- Real production entries: `--emit-exe` (`modes/mir.rs`
  `emit_published_view_exe`), `ny-llvmc` lib/bin
  (`emit_published_static_method_exe`), object
  (`try_compile_published_static_method_object`).
- Write-family precedent: `compile_array_element_writes_object_on_large_stack`
  already drives the REAL entry (view → JSON → C ABI → .o, optional exe
  when the archives exist). The read family has publication-level tests
  and C-driver fixture tests, but no production-entry object evidence —
  that is the one honest gap.
- App-level `--emit-exe` remains frozen upstream at baseline
  (`artifact-unowned-lifecycle-site`) — a separate blocker, not this
  lane's gap.

Decision: close the parity gap — add the read twin of the write
production-entry test. A module with `newbox ArrayBox`, one `set`, one
`ArrayElementRead` and `return dst` drives
`try_compile_published_static_method_object` to a real `.o` through the
actual Rust→C host path; when `libhako_llvmc_ffi.so` and the canonical
legacy-entry `libnyash_kernel.a` exist, `emit_published_static_method_exe`
produces a runnable executable whose exit is the stored value — one
execution path, real callers.

Source authority + canonical issuer: `PublishedMirBackendView::try_new`
admission + `c_transport.rs` row issuance (already landed); this slice
only observes them through the production entry.

Non-authority: app `--emit-exe` frontier (upstream baseline freeze);
v4 lane; source-level acceptance; legacy route retirement.

Fail-fast boundary: the test asserts `Ok(true)` + `.o` exists — any
row/admission failure surfaces as the existing typed error, never a
repaired object; the optional exe branch keeps the same archive-gated
pattern as the write twin.

Smallest next slice: `MIRBUILDER-ARRAY-READ-PRODUCTION-CALLER-S0` — the
read production-entry object/exe test.

Non-claims: production cutover of real apps; whole-goal acceptance;
v4 exe entry; `dst:None` transport.

## production caller landing / 2026-10-06

`MIRBUILDER-ARRAY-READ-PRODUCTION-CALLER-S0` landed. The landed test is
stronger than the card's planned hand-built module:
`published_array_element_read_compiles_through_production_entry` parses
real source (`values.get(0)`), compiles it through `MirCompiler`,
refreshes/validates at the Verifier boundary, asserts exactly one
`ArrayElementRead` (the sole physical owner — no residual
`Callee::Method{ArrayBox, get}`), then drives the real production entries:

- `try_compile_published_static_method_object` -> `Ok(true)`, non-empty
  `.o` through the actual Rust view -> JSON -> C ABI -> Lane-A
  `slot_load_hi` path;
- when `target/release/libnyash_kernel.a` (canonical legacy-entry) and
  `target/release/libhako_llvmc_ffi.so` exist, `emit_published_static_method_exe`
  links a runnable executable -> exit status `7` (`Result: 7` observed).

Evidence chain closed: source -> compiler -> `ArrayElementRead` ->
published view -> production C ABI entry -> object -> executable ->
stored value. The optional exe branch keeps the write twin's
archive-gated pattern, so environments without the archives still gate
on the real object emission.

Adjacent baseline repair in the same family: the write twin's
`published_array_write_typed_contract_rejects_before_object` was red at
baseline — `cd3810403c` moved the non-lifecycle backend-capability
enforce inside `compile_published_view_object`, so the EXE entry hit the
`--emit-exe-nyrt` requirement before the typed-array gate (restored the
`78b0a873b2` contract "capability reject before executable transport").
Fix: `emit_published_view_exe` now runs
`enforce_published_backend_supported(view, "ny-llvmc-exe")` right after
`select_published_route` for non-lifecycle modules — same check the
object path already runs, just earlier and only for the canonical typed
route; lifecycle modules keep their existing enforce order (typed-array
enforce already runs inside `select_published_route`, input-bound
enforces still run inside compile).

Known environment caveat (not a lane defect):
`HAKO_BACKEND_COMPILE_RECIPE` is process-global state mutated by
`llvm_provider_flags` unit tests; running the published backend test
group in parallel can surface `no_lowering_variant` in the optional exe
branches. Run this family with `--test-threads=1`; a real serialization
fix is test-infra work outside this lane.

Next owed: `MIRBUILDER-ARRAY-READ-LEGACY-ROUTE-S0` — design-stop census
of residual `ArrayBox.get` consumers outside the sole physical owner:
unproven-receiver `Callee::Method{ArrayBox,get}` lowering, mir_json_v0
generic-lane reads, and any runtime dispatch that still interprets the
source-level call instead of `ArrayElementRead`; select the bounded
retirement from that inventory. The VM interpreter already owns an
`ArrayElementRead` handler (`mir_interpreter/handlers`, `exec/block`),
so the audit is about residual callers, not a missing consumer. Wider
frontier items (selected legacy route retirement, whole-goal
acceptance) continue under `MIRBUILDER-FINAL-PIPELINE-v1`.

## legacy route census / 2026-10-06 (design_stop audit)

`MIRBUILDER-ARRAY-READ-LEGACY-ROUTE-S0` audit — residual `ArrayBox.get`
consumers outside the sole physical owner. Census boundary: producers of
`Callee::Method`-carried gets and every consumer that can still execute
them. Excludes `DirectArrayI64`/`RuntimeDataBox`/`MapBox` `get` (separate
semantic surfaces — the 296x direct-array lane and the dynamic facade
own those).

### Producer inventory

| Producer | Emits | Status |
|---|---|---|
| Builder `emit_box_or_plugin_call` + `unified_emitter` + indexing load, receiver proven `ArrayBox` | `ArrayElementRead` | sole canonical path — `try_emit_known_array_method_read` intercepts before any callee emission at every entry |
| Builder, receiver proven `DirectArrayI64` | `Call`+`Callee::Method{DirectArrayI64,get,Known}` | separate surface (296x) |
| Builder, receiver unproven | `Call`+`Callee::Method{RuntimeDataBox,get,Union}` | dynamic facade — different meaning ("call get on whatever") |
| mir_json_v0 `boxcall` | `Call`+`Callee::Method{box_name,get,Union}` | only compat ingress that can spell `box_name:"ArrayBox"`; `call`/`mir_call` ops already freeze at decode |
| `LegacyCallV0`+`Callee::Method` | — | **no production ingress** (R7-S6; `callsite_canonicalize` header: no production ingress mints `LegacyCallV0`) |

### Consumer inventory

| Consumer | Input | Route |
|---|---|---|
| VM reference lane | `Call`+Method / `LegacyCallV0` | **fail-closed** — canonical Call admits only `Callee::Global` targets; `reject_legacy_call` stops Method/Value/Extern |
| `generic_method_route_plan` | `LegacyCallV0`+Method get only (canonical `Call`+Method is not matched) | `CoreMethodOp::ArrayGet`/`ArraySlotLoadAny` WarmDirectAbi — **input-less in production** since R7-S6 |
| C same-module `emit_method_call_mir_call` | `mir_call`+Method | special cases -> `emit_generic_method_mir_call` (plan view or NULL) |
| C generic compat `mir_call_dispatch` Method arm | `mir_call`+Method | `GenericMethodRouteState` -> `runtime_array_get` runtime export — the live residual executor for `boxcall`-spelled `ArrayBox.get` |
| `SameModuleArraySlotDirectOpPlan` | `mir_call`+Method+`ArrayBox`+get/set fused at `acquire_usize/1` b45 | `slot_load_store_i64_hihi` — 296x by-name perf shim; `me.free`/`block_used` are `DirectArrayI64` so `bname=="ArrayBox"` matching is suspect for current MIR — liveness unverified while the app path is baseline-frozen upstream |
| Published `try_new` | `Call`/`LegacyCallV0` + `Callee::Method` | untracked `Some(_) => {}` passthrough into the JSON body — no flag, no stop |

### Finding

A **proven** `ArrayBox.get` has no production residual: every fresh
producer emits `ArrayElementRead`, and the only compat ingress that can
still spell `box_name:"ArrayBox"` (v0 `boxcall`) marks it `Union` —
duck-typed "call get on whatever", a different meaning correctly owned
by the generic compat family, not by `ArrayElementRead`.

But the authority boundary has one hole: a `Callee::Method{ArrayBox,get}`
that arrives with a **proof** — `Known` certainty on either `Call` or
`LegacyCallV0`, or a `Union` row whose receiver origin independently
proves `ArrayBox` — currently falls through `try_new`'s `Some(_) => {}`
into the generic compat emit instead of the sole owner. No live
producer does this today (builders canonicalize first; boxcall decode
always marks Union; `LegacyCallV0` mint is retired), so the hole is a
missing boundary rule, not a live route.

### Decision

Canonicalize proven `Callee::Method{ArrayBox,get}` residuals at the
shared `canonicalize_callsites` boundary — the one pass that runs for
both `MirCompilerPostRc` and `MirJsonV0Loader` ingresses — instead of
adding a second consumer or rejecting the meaning.

Decision: one bounded canonicalization arm — `Callee::Method{box_name ==
"ArrayBox", method == "get", receiver: Some(_), args.len() == 1}` on
`Call` **and** `LegacyCallV0` rewrites to `ArrayElementRead` when the row
carries a proof (`TypeCertainty::Known`, or `receiver` origin resolves
to `ArrayBox`); proven meaning, sole owner.

Source authority + canonical issuer: `canonicalize_callsites`
(`MirCompilerPostRc` + `MirJsonV0Loader` sites); site ids minted at the
same boundary.

Non-authority: `Union` rows without receiver-origin proof (dynamic
facade — "call get on whatever" is a different meaning and stays on the
generic compat family); `DirectArrayI64`/`RuntimeDataBox`/`MapBox` get
surfaces (296x lane / facade owners); `LegacyCallV0` carrier itself
(sunset rides `RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001`);
`SameModuleArraySlotDirectOpPlan` (296x parked perf shim — removal, if
dead, is that lane's call).

Fail-fast boundary: `Callee::Method{ArrayBox,get}` with a wrong arity,
no receiver, or no proof stays exactly as today — compat emit or
backend named-stop; no silent repair, no second semantic owner.

Smallest next slice: `MIRBUILDER-ARRAY-READ-LEGACY-ROUTE-S0`
implementation — the canonicalize arm + focused tests: hand-built
`Call`/`LegacyCallV0` `Known` rows rewrite to `ArrayElementRead`;
`Union`-without-origin rows pass through untouched; `DirectArrayI64` /
`MapBox` / `RuntimeDataBox` rows untouched; `array_element_read` still
emits `slot_load_hi` downstream.

Non-claims: v0 `call`/`mir_call` op unfreezing; `LegacyCallV0` deletion;
296x shim retirement; a second get consumer; whole-goal acceptance.

## legacy route canonicalization landing / 2026-10-06

`MIRBUILDER-ARRAY-READ-LEGACY-ROUTE-S0` landed with one boundary
correction to the Decision above: the arm covers `Call` only, not
`LegacyCallV0`. The pass's stated contract is "residual legacy rows are
never rewritten here ... this post-pass is deliberately not a second
resolver and does not launder retired carriers" — laundering a
`LegacyCallV0` row would violate that contract, and R7-S6 already
guarantees zero production ingress for the carrier, so covering it
would be dead code. Residual `LegacyCallV0` keeps hitting backend
named-stops exactly as today.

Shipped: `callsite_canonicalize/array_read.rs` —
`canonicalize_proven_array_method_reads` runs per function inside the
shared `canonicalize_callsites` boundary (both `MirCompilerPostRc` and
`MirJsonV0Loader` sites). A `Call`+`Callee::Method{ArrayBox,get}` row
rewrites to `ArrayElementRead` when `TypeCertainty::Known`, or when a
`Union` row's receiver origin independently proves `ArrayBox` via
`receiver_origin_box_name` (`def_map` built lazily, only when a Union
candidate exists). Arity accepts `get(index)` and the
receiver-duplicated `get(receiver, index)` surface only when `args[0]`
is literally the receiver value. Site ids continue above the function's
existing `ArrayReadSiteId` max; `dst` registers `MirType::Unknown` in
`metadata.value_types`, mirroring builder emission.

Evidence: 9 focused pins green — Known rewrite, Union+origin rewrite,
Union-without-origin untouched, `DirectArrayI64`/`MapBox`/
`RuntimeDataBox` untouched, `LegacyCallV0` untouched, malformed arity
untouched, receiver-duplicated surface picks the real index, site-id
continuity, and a downstream pin proving the canonicalized row reaches
`PublishedMirBackendView` `CanonicalTyped` with exactly one
`array_element_reads` row (the sole Lane-A/v4 consumer family).
`callsite_canonicalize` suite 23/23 and `mir_json_v0` suite 17/17
green — the v0 `boxcall` compat surface now canonicalizes proven
`ArrayBox.get` through the same boundary instead of the generic compat
emit.

Non-claims: unproven `Union` rows still flow to generic compat emit
(their dynamic-facade meaning is unchanged); `call`/`mir_call` v0 ops
stay frozen; `LegacyCallV0` untouched; the 296x
`SameModuleArraySlotDirectOpPlan` shim untouched; no second get
consumer; whole-goal acceptance still owed under
`MIRBUILDER-FINAL-PIPELINE-v1`.

Next owed: `MIRBUILDER-ARRAY-READ-PRODUCTION-SWITCH-S0` —
`physical-array-read/production-switch` design_stop census to select
the bounded production-caller switch edge and the legacy-route
retirement target now that every proven `ArrayBox.get` producer flows
through the sole physical owner.
