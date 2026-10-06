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

## Verified closeout / 2026-10-06

S0 landed. `array_i64_fields.rs` issues the census once in co-seal
preflight; the claim ledger transports `array_i64_fields` to the prefix
arm, the borrowed-result pending fold and test-only accessors. A field is
proven only when declared `ArrayBox`, exactly one birth-store
`new ArrayBox()` provides it (inline initializers reach birth through the
prologue row), every `set`/`push` value on attributed `me.<f>` or
proven-alias writers is integer-source (literals, integer locals, proven
`me.<i64>` reads and proven-field `get` results fixpoint), and no
escape/foreign selector/unattributed write/alias rebind/second provider
remains. `capture_field_array_get` in `ordinary_new_borrowed_formal_
result_pending.rs` seals a borrowed callee's direct or local-bound
`return`; vetoed fields keep manifest `Dynamic` coverage and an unproven
borrowed return fails `source-not-i64` through `BorrowedFormalIngress`.

Evidence: 11 brand-catalog pins + 4 published-view pins green — i64
install, `me.<i64>` index, chained get-as-index, borrowed direct/local
return seal; vetoes for non-integer writes, second provider, escape,
rebind, foreign selector, non-integer/null/homes index; condition-position
keeps the raw-compare boundary; non-borrowed `return me.<f>.get(..)` mints
no terminal relation; `Callee::Method` get stays unpublished until the
physical read owner. Regression battery 1158 PASS / 6 FAIL, all six
reproduced identically on baseline 36b13d8d8e — zero current-change
failures. Unchanged `apps/mimalloc-lite` `--emit-exe` stops at
`artifact-unowned-lifecycle-site`, identical to baseline (app sites use
the `handle.block_id` sibling index). Scope guard stops at the unchanged
root1351 baseline; touched sources stay under 800 (coseal_issue 793).

Staged: `array_i64_fields.rs`; `brand_catalog_array_i64_tests.rs`;
`array_i64_field_call_tests.rs`; census/issue wiring in
`ordinary_new_coseal{,_issue,_issue_lexical,_issue_source}`; ledger field
+ accessors in `ordinary_new_ledger`; predicate/re-export in
`ordinary_new_terminal_home`; profile param in
`ordinary_new_borrowed_formal_profile`; capture arm in
`ordinary_new_borrowed_formal_result_pending`; census plumbing in
`home_new_prefix{,_scan,_arguments,_branch,_field_call}` and
`function_control_new_homes`; module registrations
(`brand_catalog_tests`, `map_value_completion_tests`,
`borrowed_source_publication_tests`); `normal_callable_semantic_package`
README paragraph; this card and the pointer.

Non-claims: `handle.block_id` formal-field-read index admission is the
named sibling — app `isLiveHandle`/`release`/`resizeInPlace` stay
unadmitted; declared-DirectArrayI64 arm still blocked on the
`FieldContractUnsupported` provider prerequisite; no physical read owner
or `Callee::Method` publication route; no condition-position or
nested-receiver admission; production caller switch, selected legacy
retirement, unchanged-app frontier and the finite product goal remain
owed. Parked lanes unchanged; protected sibling WIP intact.

## Formal-field index integrated Decision / 2026-10-06

Design-stop audit for `field-array-get/formal-index-decision` (read-only,
same-thread; no worker needed — all uncertainty resolved against sealed
source). App inventory: every `handle` formal in `release`/`isLiveHandle`/
`resizeInPlace` is untyped (`OpaqueHandle`); `handle.block_id` reads are all
dominated by `if handle == null` guards (admitted `FieldReadOperand` draft
shape already — liveness is that lane's product, not this arm's). The only
index leaf missing from `integer_source_at` is a formal field read:
`handle` carries no declared type and its actuals are Dynamic
(`handles.get(0)`), so neither parameter contracts nor call-site provenance
can prove the class. Within the package's ordinary-box coverage `block_id`
is declared exactly once, non-weak `i64`, on `HakoAllocHandle`;
`HakoAllocFastPathHandle.block_id` lives in a different module outside the
coverage. `handle.page_id`/`requested_size` are ambiguous or write-only
and stay unproven — their sites are conditions/foreign stores already
owned elsewhere.

Decision: admit `formal.<field>` as an integer source when the formal
binding is a package parameter (non-`me`, non-local) and `field` resolves
to exactly one non-weak `i64` declaration across
`ParserOrdinaryBoxSourceCoverageV1::rows()` via the existing
`source_declared_field` lookup — issued as one new leaf inside
`integer_source_at` shared by the census write-value check, the prefix
get-index arm and `capture_field_array_get`. Execution row
MIRBUILDER-FIELD-ARRAY-GET-I64-FORMAL-INDEX-S0.

Source authority + canonical issuer: sealed `FieldAccess` object binding
(`ResolvedLexicalRefV1::Local` parameter kind), the ordinary-box coverage
row set, and constructor-batch field declarations; canonical issuer
`integer_source_at` in `array_i64_fields.rs` with the coverage lookup
carried on the existing co-seal closure channel.

Non-authority: declared parameter types (absent), call-site actual
classes (Dynamic for `handles.get(0)`), null-guard narrowing (liveness,
not class), MIR/layout tags, typed-formal `DeclaredObject` proofs.

Fail-fast boundary: zero or ambiguous declarations, weak or non-i64
declared type, unknown type name, local/alias receivers, and `me`-rooted
objects all keep the existing `false` — `source-not-i64` for armed
borrowed returns, manifest `Dynamic` elsewhere.

Smallest next slice: the single leaf plus focused pins — `get`/`set`
index positions and `set`/`push` value positions in the census fixpoint
(`free_stack.set(me.free_top, handle.block_id)`), ambiguous `page_id`
staying unproven, and the unchanged app frontier observation.

Non-claims: `handle.page_id` ambiguity resolution, formal field stores
(`handle.requested_size = x`), typed-parameter class proofs, alias-chain
receivers (`local h = handle; h.block_id`), condition-position admission,
the physical read owner, app EXE acceptance, production switch and the
finite goal.

## FORMAL-INDEX-S0 landing / 2026-10-06

Shipped boundary differs from the integrated Decision in one place:
`capture_field_array_get` does not credit the formal leaf. Arming it
sealed `return me.block_used.get(handle.block_id)` inside
`HakoAllocPageModel.isLiveHandle`, which grounded the callee and forced
incoming coverage for `me.small_page.isLiveHandle(handle)`; `me.<name>`
receiver calls are outside the lexical need inventory, so the real app
package froze on `borrowed-formal/incoming-coverage` then
`stored-child/result-source-missing`. That receiver-call coverage is its
own bounded row (needs → targets → argument edges), so this slice keeps
the leaf for exactly the census write-value check, the prefix get-index
arm and the argument-neutrality gate, and pins the capture decline as
`source-not-i64` — identical to the pre-leaf Dynamic-get boundary.

Landed evidence: `cargo test --lib -- array_i64` 27/27 green (positive
formal index, `set`/`push` write value, ambiguous/non-integer/local-alias
declines, unarmed borrowed-return pin, and the pre-leaf publication
rows); `page_heap_fixture` 4/4 green — matching the `c887c073d9` baseline
which fails identically only on `map_value_get_*` Unknown contract debt;
`ordinary_new_coseal_issue.rs` 796 / `_source.rs` 785 lines stay inside
the 800 hard stop via the shared `formal_i64_index_consult_v1` factory.

Next owed: `me.<name>` receiver call coverage (sibling row — enables
arming the leaf in the borrowed-result capture), then physical read
owner, production switch, legacy retirement.

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
