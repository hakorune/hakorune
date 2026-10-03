# mimalloc-lite opaque checked-compare task-4 D0 (dominated view uses)

Status: accepted task-4 Decisions; I64RESULT-S0 and ROOTSOURCE-S0 landed; next USESIZE-T0 (implementation pending).
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-D0`
  and its bounded PARAMFIELD prerequisite/acceptance series.
Related: docs/development/RULES.md; CURRENT_STATE.toml;
  mirbuilder-app-mimalloc-lite-opaque-checked-compare-normal-integer-d0-2026-10-03.md
  (parent; view lending rules and S0 receipts);
  mirbuilder-app-mimalloc-opaque-formal-ingress-d0-2026-10-01.md (grandparent);
  docs/reference/language/function-exit-and-entry-result.md;
  docs/reference/language/dynamic-operators.md;
  docs/reference/abi/nyrt_c_abi_v0.md.

## Decision

The checked compare's kind==1 proof extends the lent Normal-Integer *view*
to the same formal's uses dominated by the compare's block: the original
`ArraySet` element, the ordered `Add` operand, and the constructor
argument. One lent view class, one ledger — no second authority, no
per-instruction reinterpretation, no `StoredLocal`/parameter-contract
change. The tagged carrier (`borrowed_kind_payload_v1`) is retained end to
end; usize range authority stays at the checked write lane
(`verification/numeric_substrate.rs`), which rejects out-of-range payloads
at `me.<usize field>`/array-element/birth-field stores.

Because the compare's site check dominates both successors, a dominated
use can read the view payload directly; the emitter still re-checks
`k==1` per tagged operand site (uniform with compare), so a projected or
drifted operand can never bypass the fault boundary.

Source authority + canonical issuer:
- `expression_source.rs`/`body_shape` keep the site identities: `binaries()`
  for `+`, `method_calls()` for `me.<ArrayBox>.set`, `constructions()` for
  `new`. The guarding relation is the existing `if` region
  (`with_if_region_for_condition`) plus block dominance
  (`compute_dominators`/`dominates` in `verification/utils.rs`; the
  `MapSetScalarI64DominatesNoEscape` proof is the precedent).
- `dynamic_operator_contract` gains `Add(NormalInteger, NormalInteger)`
  with a fresh-integer result class (no lifecycle obligation) — the
  existing operation owner, consumed through the same borrowed-formal
  co-seal; `Add(Dynamic, I64)` is untouched.
- `ordinary_new_borrowed_formal_uses.rs` gains dominated-use draft kinds
  beside `CompareOperand`: `AddOperand`, `ArrayElementValue`,
  `ConstructionArgument` — each requiring the same binding, a proved
  Normal-Integer sibling where applicable, and a guarding admitted compare
  whose region dominates the use site.
- `ordinary_new_borrowed_formal_result.rs` gains a nullable-handle result
  source class (`null` literal and `new`-construction return sites) —
  constructor use cannot be proved while `borrowed-result` requires i64.
- Physical: `borrowed_call_uses` extends the one-step view closure from
  `Compare` operands to `BinOp{Add}`, `ArrayElementWrite{Set}.value`, and
  constructor-call args at proved ordinals — only inside blocks dominated
  by the admitted compare's block; coverage stays per distinct operand.

Non-authority: `usize`/U64Bits storage spelling is never the proof;
`DynamicOperatorValueClassV1::I64` stays a lane name; a `.set` argument is
not an `UnresolvedArgument` forward candidate (generic method route, not a
same-module lexical call — the silent `outside` drop stays rejected by
class, not retried); `new` argument transport must not reuse the
literal-scalar `scalar_actual_kind` wall — non-literal birth actuals get
the same `{"kind":..., "value": vid}` encoding class as `ordinary_call`,
extended by the new view tag, never a permissive literal fallback.

## Pinned census (read-only worker + probes, integrated)

Site: `lang/src/hako_alloc/memory/page_heap_box.hako` — `allocate(requested_size)`:
- L75 `requested_size > me.block_size` — compare view (landed S0).
- L93 `me.requested_sizes.set(block_id, requested_size)` — `.set` arg is a
  `method_calls()` row with a non-`Me` receiver (`ResolvedMethodCallReceiverSourceV1::Other`);
  today classifies `UnresolvedArgument`, then the owner is silently dropped
  (`ordinary_new_borrowed_formal_source.rs` `outside` insertion), because
  `ArrayBox.set` is a generic-method-route call, not a lexical instance
  call. Physical `MirInstruction::ArrayElementWrite{Set}` is absent from
  the ordinary supported whitelist; JSON has no `array_set` op kind; C has
  no validator/index/emit arm — a new operation kind end to end.
- L96 `me.requested_bytes = me.requested_bytes + requested_size` — `+` rhs
  site hits `UnsupportedUse` (operator not `Greater`); first observed
  decline. Physical `BinOp{Add}` is whitelist-admitted; `map_value_domains`
  admits only `I64×I64`; C `add` requires `LV4_I64` both operands.
- L102 `new HakoAllocHandle(me.page_id, block_id, requested_size)` — `new`
  is a `constructions()` row, not `method_calls()`; arg site hits
  `UnsupportedUse`. Wider frontier: `scalar_actual_kind` admits literal
  Integer/Bool only (`physical_abi.rs`), so `me.page_id` (I64Field) and
  `block_id` (Local) are independently unavailable — non-literal birth
  actuals are a general gap, not borrowed-specific.
- Result lane: `return null`/`return new HakoAllocHandle` hits
  `borrowed-result/source-not-i64` — functions carrying admitted borrowed
  formals currently require Integer-literal or ExactTrivial(I64) return
  sites; a nullable-handle class is owed.

Empirical frontier (bisect probe, since removed):
- `me.items.set(0, requested)` -> `TerminalHomesUnavailable` (owner
  silently dropped at draft; callee unclaimed -> `new Store()` unowned).
- `me.total = me.total + requested` -> `TerminalHomesUnavailable`
  (`UnsupportedUse`, same propagation).
- `new Pair(requested)` -> `TerminalHomesUnavailable` (`UnsupportedUse`).
- `return new Pair(1)` (literal arg, ctor only) ->
  `[ordinary-new/borrowed-result/source-not-i64]` — confirming the result
  lane is an independent fourth gate.

## Contract and fail-closed boundary

View scope (parent rule preserved): same binding/ValueId; the use site
must be dominated by the compare's block (both successors carry kind==1 —
the site check precedes the branch). Reject: pre-check uses, non-dominated
uses (including the true-branch interior — it returns), Fault-edge uses,
foreign-owner values, rebound loans, sibling-free operands — all stay
`UnsupportedUse`/`forbidden-operand`.

Physical gates, all named and fail-closed:
1. `draft_borrowed_formal_uses_v1`: `AddOperand` only for `operator() ==
   Add` with a `normal_integer_operand` sibling and a guarding admitted
   compare dominating the use; everything else stays `UnsupportedUse`.
2. `borrowed_call_uses`: view copies may feed `BinOp{Add}` operands only in
   blocks dominated by their formal's compare block; coverage counts
   distinct operand values per admitted use (edge-port re-evaluation is
   one use).
3. `invoke.rs`/`physical_program_field_ref`: the `me.<usize>` sibling read
   stays inside the existing borrowed-tagged corridor; the Add result
   writes back through the checked usize write lane.
4. C: `lv4_indexed_rows` `add` arm admits `LV4_TAGGED` operands only when
   the function carries the borrowed corridor; `lv4_emit` emits the same
   `k==1` site check per tagged operand before `add i64` — uniform with
   compare, no cross-instruction state.

## S0 acceptance boundary (ordered-Add view use)

- Focused positives: opaque formal `>` check dominating
  `me.<usize field> = me.<usize field> + requested` — published JSON shows
  the tagged view feeding `add`; EXE executes: in-range arg yields
  field+arg, `-1` faults at the usize write (range authority stays at
  write), bool/object fault at the compare site before the add.
- Negatives: add operand without a guarding compare, add outside the
  dominated region, wrong sibling class, view escape into edge args —
  decline/fault.
- `.set` and `new` uses stay `UnsupportedUse`/`forbidden-operand` in this
  slice; `borrowed-result` stays i64-only (`check` returns literals).
- No retirement; additive view extension only.

Follow-on slices inside task-4 (named, ordered): ArraySet element view
(new `array_set` op kind end to end), then constructor argument view +
non-literal birth actual transport + `borrowed-result` nullable-handle
class. `allocate` admission completes only after all three.

## Non-claims

No `ArraySet`/`NewBox`/birth-actual admission in S0; no nullable-handle
borrowed result in S0; no `LessEqual`/`GreaterEqual`/`Equal`/other binary
siblings; no unsigned lane; no general usize field publication; no
`allocate`/`resizeInPlace`/`realloc` admission claim; no app EXE PASS or
MirBuilder completion claim; no retirement.

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ADD-S0)

Landed (commit pending on `codex/birth-definition-publication`):

- `dynamic_operator_contract` gained `Add(NormalInteger, NormalInteger)`
  with the fresh `NormalInteger` result class (no lifecycle obligation).
- Source draft `AddOperand`: an ordered `+` operand of the tracked formal
  or alias admits only beside a proven Normal-Integer sibling (Integer
  literal, tracked formal copy, or `me.<field>`) and only inside the
  bounded dominated region — the statements strictly after the guarding
  `if` in its own sequence; uses inside the guard's own arms, the
  condition, pre-guard positions, wrong operators, and unproved siblings
  stay `UnsupportedUse`.
- Physical closure: view copies may feed `BinOp{Add}` operands only where
  a compare site of the same formal dominates the add block (strictly
  earlier in the same block); `undominated-view`/`add-coverage` close it,
  coverage counts distinct operand values (edge-port re-evaluation is one
  use).
- Bare `MirInstruction::FieldSet` publication end to end: field_ref route
  projection (`slot_store_i64` or `slot_store_u64` inside the borrowed-
  tagged corridor), whitelist admission, `field_set` JSON row with
  `exact_numeric_runtime_check`, abi-input check coordinates, diagnostic
  site, C v2/indexed-flow/emit arms — `dynamic_integer_range/usize`
  discharges by an emitted negative-value branch to
  `NYRT_FAULT_REASON_FIELD_RANGE_V1` (113) before
  `nyash.object.checked_field_set_v1`; the kernel still owns slot/storage
  identity.
- `exact_numeric_backend_capability/lifecycle` binds the same contract to
  bare rows (usize storage spelling, exact field/layout/slot proof,
  birth-formal coverage skipped for the computed i64 value lane) while
  the invoke form keeps i64-only.

Evidence pins (release `--lib`):
`dominated_add_admits_guarded_integer_sibling_and_aliased_operands`,
`add_operand_rejects_unguarded_undominated_and_unproved_sibling_uses`,
`borrowed_use_dominated_add_view_passes`,
`borrowed_use_rejects_undominated_and_drifting_add_view`,
`dominated_add_view_publishes_from_original_source` (5 JSON variants),
`lang/c-abi/tests/published_lifecycle_v4_add_view_execution_test.py`
(hi=1, over=0, neg/bool/object=70 Fault).

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARRAYSET-S0
(the dominated ArraySet element view — new `array_set` op end to end).

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARRAYSET-S0)

Landed (commit pending on `codex/birth-definition-publication`):

- Source draft `ArrayElementValue`: `me.<ArrayBox field>.set(index, value)`
  admits only the ordinal-1 value operand of the tracked formal or a proven
  copied view, only when the receiver resolves through the proven ArrayBox
  field (`receiver_array_field`), the ordinal-0 index is a Normal-Integer
  operand, and the value use sits in the bounded dominated region of the
  same checked compare; non-ArrayBox receivers, wrong ordinals, borrowed
  receiver/index, and undominated uses stay `UnresolvedArgument`/
  `UnsupportedUse`. Draft unit pin:
  `set_element_value_without_entry_receiver_loan_stays_unresolved` (no
  fabricated admission without receiver proof).
- Finalized projection `borrowed_ordinary_array_element_uses_v1` projects
  finalized `.set` value uses as (binding, formal, call site) triples.
- Physical closure: `ArrayElementWrite{Set}` consumes two operands; the
  value admits only under checked-compare dominance, receiver/index may
  not carry the borrowed lane, `borrowed-use/set-coverage` closes on
  distinct value counts (edge-port duplication counts once).
- Bare `ArrayElementWrite{Set}` whitelist admission is route-gated:
  `producer == MethodCall`, `index.is_some()`, route kind `ArrayStoreAny`,
  exact block/instruction coordinate, matching `array_write_site_id`,
  receiver, and index; anything else fails `array-set-unsupported`.
- Field route projection accepts `hako.typed_object.slot_load_handle`
  (`handle` storage, `MirType::Box("ArrayBox")`) only inside the
  borrowed-tagged corridor; i64/numeric-view routes unchanged.
- Publication emits one canonical `array_set` row
  (`array`/`index`/`value`/`site`); C v2 validates exact keys plus value
  availability; indexed flow requires `array`/`index` `LV4_I64`, `value`
  `LV4_I64` or `LV4_TAGGED`, not faulted, not birth; emit adds a
  `kind == 1` tag check for tagged values then calls
  `nyash.array.checked_set_i64_v1` (new kernel export over
  `slot_store_i64_result`; bounds → `ARRAY_SET_BOUNDS` 204, element
  contract → `ARRAY_SET_ELEMENT_MISMATCH` 203).

Evidence pins (release `--lib`):
`set_element_value_without_entry_receiver_loan_stays_unresolved`,
`borrowed_use_dominated_set_view_passes`,
`borrowed_use_rejects_undominated_and_drifting_set_view` (12-test file
green),
`dominated_set_view_publishes_from_original_source` (5 JSON variants +
projection-drift reject),
`lang/c-abi/tests/published_lifecycle_v4_set_view_execution_test.py`
(ok index-0 in-range/oversized executes; index-3 bounds fault; bool/object
kind!=1 fault; 3 checked-compare inputs unchanged).

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-CTORARG-S0
(constructor argument view + non-literal birth actual transport +
`borrowed-result` nullable-handle class — the `new HakoAllocHandle(...) +
return` admission).

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-CTORARG-S0)

Landed (commit pending on `codex/birth-definition-publication`):

- S0a non-literal i64 birth-actual transport: `scalar_actual_kind` now takes
  the emitted value plus the caller's `value_types` and admits `Local`,
  `BoundValue`, and `I64Field` actuals only when the physical lane is
  exactly `MirType::Integer`; null/handle/non-scalar actuals still reject
  with `actual-kind-unavailable`. `me.<i64 field>` admission rides the
  exact entry-receiver-field proof (`entry_receiver_field`), so guarded
  bodies (`if … { return 0 }` before the `new`) keep the same authority.
- Readiness probe coherency: the verified-vs-bounded lane probe now runs
  `verify_function_completion_v1(input)` and feeds
  `control.explicit_sites()` — without exit sites a guarded `return`
  poisoned the joined prefix as `PrefixNotCovered`, forced
  `verified_walk=false`, and silently selected the bounded sibling lane
  (all-false argument predicates → `ArgumentNotTrivial`). Completion
  failure keeps the existing graceful fallback (`readiness=false`).
- S0b source draft `NewArgument`: a `new <Child>(...)` argument site admits
  only inside an admitted checked compare's dominance of the same formal;
  the `(new site, ordinal)` pair pins the sole admitted argument position.
  Draft pins: `dominated_new_argument_admits_exact_site_and_ordinal`,
  `new_argument_rejects_unguarded_and_inside_arm_uses`.
- Finalized projection `borrowed_ordinary_new_argument_uses_v1` projects
  admitted uses as `(binding, formal, new site, ordinal)` tuples;
  `has_borrowed_ordinary_entry_v1` membership keeps non-borrowed birth
  callers on ordinary scalar validation instead of demanding entry values
  they never recorded (`entry-values-missing` stays the borrowed-only
  contract).
- Physical closure: bare `Call` and `Invoke`-wrapped `BirthConstructor`
  argument positions consume the lent view only inside the admitted
  compare's dominance cone; receiver/callee-operand and fault-frame lanes
  reject tracked roots and copied views alike (`callee-operand`,
  `callee-or-fault-frame`, `undominated-view`, `ctor-coverage` on distinct
  operand values). The invoke arm's operand check now folds the view lane
  into the common forbidden scan.
- Publication: `issue_tagged_birth_actuals` admits `"tagged"` only for
  exact `(new site, ordinal)` rows whose argument kind is
  `Handle { binding }` matching the admitted formal, rejects duplicate
  tagged coordinates, and the wire spells `{"kind":"tagged","value":N}`
  against the callee's `kind_payload_v1` param; C v2 validates the pair,
  indexed flow requires the caller's own borrowed tagged formal, and emit
  re-proves `kind==1` at the call edge before the `(k,v)` pair reaches
  `hako_lifecycle_birth_*`.
- Artifact ownership: finishing-checked children carrying ledger-owned
  `ObjectFieldGet` (`validate_field_reads`) are exact-read owners beside
  the retained root — `unowned-exact-field-read` no longer mis-fires on
  `me.<field>` argument reads inside admitted child bodies.

Evidence pins (test profile `--lib`):
`selected_new_arguments_admit_entry_receiver_i64_field` plus the existing
selected-new suite (5/5),
`dominated_new_argument_admits_exact_site_and_ordinal` /
`new_argument_rejects_unguarded_and_inside_arm_uses`,
`borrowed_use_dominated_ctor_view_passes` /
`borrowed_use_rejects_undominated_and_drifting_ctor_view` (18/18 file
green),
`nonliteral_i64_birth_actuals_publish_integer_payload_tag`,
`dominated_new_argument_view_publishes_tagged_birth_actual` (4 JSON
variants + projection-drift reject),
`undominated_new_argument_view_still_rejects`,
`nonscalar_birth_actuals_still_reject`,
`lang/c-abi/tests/published_lifecycle_v4_new_argument_execution_test.py`
(ok=1/over=0 executes; bool/object kind!=1 fault at the compare).

Baseline reds observed while gating (all already classified):
`artifact_child_rejects_retained_unavailable_commit_before_lifecycle_coverage`
and `test_weak_handle_lifecycle` are manifest rows 12/91 —
`literal-physical-drift` is the upstream `ReceiverNonEscape` boundary
reproducing at `ee30614975`/`bdbe0202fb` HEAD identically; the two
`array_source_*` rows are batch-only flakes passing in isolation. The
guard's only structural debt remains `brand_catalog_tests.rs=961` at HEAD.

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-RESULT-S0
(borrowed-result nullable-handle class — `return null` and `return new`
inside the same corridor, completing `HakoAllocPageModel.allocate`'s
`return new HakoAllocHandle(me.page_id, block_id, requested_size)`).

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-RESULT-S0)

Landed (commit pending on `codex/birth-definition-publication`):

- Source class `BorrowedResultClassV1::{I64, Nullable}`: every explicit
  value-return site classifies exactly once — `return null` and
  `return new ..` spell `Nullable`, mixed scalar/nullable exits freeze
  `source-class-mixed`, and a null-only body stays `source-not-i64`
  (no `return new` exit means no `NullableObject` claim can name the
  carried class — the old literal-i64 default hole stays closed).
- Corroboration: `Nullable` demands an unannotated result contract plus
  the sealed `NullableObject` claim and an exact observed-site inventory
  match; disagreement mints `result-contract-mismatch` instead of
  admitting.
- Caller-side lexical membership: `issue_lexical_nullable_local_call`
  (before the i64 issuer) mints `LocalCallResultClassV1::Nullable` for
  `local x = recv.m(..)` whose callee carries the claim; the predicate
  `lexical_nullable_result_call` re-proves claim-local receiver,
  `NullableObject`, and all-null-or-new return sites — never MIR types.
- Physical emission: `emit_local_lexical_nullable` validates site /
  `NullableHandle` result / sealed relation / receiver Home / arity,
  reuses the i64 lane's argument preparation for literal/scalar/borrowed
  actuals and nested call rows, unwinds prior Homes on the fault edge,
  emits `Invoke { result: NullableHandle }` + `InvokeNormalResult`, and
  records the binding group the finalized-call inventory and exit
  cleanup both claim.
- Checked cleanup: the received binding installs as an owned nullable
  Home — exits owe exactly one `HomeReleaseIfLive`, never unconditional.
  `validate_call_received_emission` takes arity from the receiver
  observation or the lexical flow relation; projection packets carry the
  emitted result kind instead of a hard-coded `I64`, and the finalized
  visitor admits nullable lexical nodes.
- Deterministic child return-type inference: `finalize_function_draft`
  collected `Return` operands from `HashMap` blocks in arbitrary order,
  so a `return null`/`return new` callee could infer `Void` or the box
  type per run; block iteration is now sorted by `BasicBlockId` with the
  concrete-over-Void rule the module root already used — this was the
  root cause of the intermittent `ordinary-membership-drift`.

Evidence pins (test profile `--lib`):
`borrowed_nullable_result_lexical_call_publishes_checked_release`
(callee `ordinary_nullable_handle`, `const_null`, invoke
`nullable_handle`, exactly one `home_release_if_live`, deterministic ×3
plus 5× probe),
`borrowed_nullable_result_rejects_mixed_and_unproved_returns`
(mixed → `source-class-mixed`, string/bool → `source-not-i64`),
`borrowed_nullable_result_frontiers_stay_fail_closed`
(`me.` receiver → `artifact-source-unavailable`, field-forwarding →
`literal-physical-drift`),
`borrowed_call_refuses_old_literal_i64_defaults_for_other_source_domains`
(`return null` re-pinned `source-not-i64`),
`ordinary_new` suite 230/230, `lexical` suite 208/208,
`nullable_receiver_call` family 17/17,
guard `mirbuilder_qualified_route_scope_guard.sh` extended with the
RESULT-S0 pin block and the new file list.

Baseline reds observed while gating (all already classified):
`artifact_child_rejects_retained_unavailable_commit_before_lifecycle_coverage`
and `test_weak_handle_lifecycle` are manifest rows 12/91;
`map_value_get_missing_key_stays_unknown_after_typed_write` and
`map_value_get_mixed_value_results_stay_unknown` reproduce at clean
`549a54c811` HEAD identically. The guard's only structural debt remains
`brand_catalog_tests.rs=961` at HEAD.

Open frontiers (deliberately out of this slice):
`me.`-receiver nullable calls inside nested callable lowering
(`artifact-source-unavailable`/`TerminalHomesUnavailable`) and nullable
field-read forwarding (`literal-physical-drift`).

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDREAD-S0
(nullable field-read forwarding — `handle.block_id` on a received
nullable Home — the `literal-physical-drift` boundary blocking
`release(handle)` and `allocate`'s `.get`/`handle` uses).

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDREAD-S0)

Landed (commit pending on `codex/birth-definition-publication`):

- Scope taken: guarded field read on a *local* received nullable —
  `local h = s.check(5); if h == null { return .. }; return h.id` and
  the `local v = h.id` initializer form. `handle.block_id` on a formal,
  `me.`-receiver calls, and `.get` element reads stay named follow-ons.
- Path-sensitive non-null narrowing in `PrefixLocalFlow`: a dominating
  `binding == null` guard whose null arm terminates marks the binding
  in `nonnull`; the mark lives only on `ReceivedNullable` locals (the
  source class stays the authority — a bare proof set cannot mint it).
  Centralized `store()` drops the mark on any rebinding, and branch
  joins intersect marks across surviving sides — a terminated branch
  contributes nothing, sibling fall-through requires both sides.
- Admission reuses the field issuer: `FieldReadReceiverV1::
  ReceivedNullable` + `LocalFieldReadRequestV1.nullable`; the request
  carries `home == receiver` and no alias class. The issuer resolves
  the class from the sealed `NullableObject(class)` claim
  (`nullable_received_result_class`), never from MIR types or layout,
  and proves the field through `prove_local_field_read_batch`.
- Terminal lane: `return_scalar`'s FieldAccess arm resolves the receiver
  via `field_home` (non-null + ReceivedNullable), and the terminal
  field-return helper gains the nullable integer-field arm so an
  `I64Field` exit still feeds `result_abi` unification (the null arm
  must return a field-class-compatible value — `return s.limit` — a
  literal-only null arm diverges the ABI and stays rejected).
- Physical owner unchanged: the claimed read is consumed by
  `take_terminal_field_read`/`take_local_field_read` (whose
  `installs_ordinary` relaxed to `installs` so a `CallReceived` commit
  satisfies the receiver), emitted exactly once as `ObjectFieldGet`
  with base = the `invoke_normal_result` value; cleanup stays one
  `home_release_if_live` per return exit, zero unconditional releases
  on the nullable result.

Evidence pins (test profile `--lib`):
`nullable_field_read_publishes_guarded_object_field_get` (terminal +
initializer: `ordinary_call`/`nullable_handle`, `object_field_get`
base == `invoke_normal_result` dst, checked releases == return exits,
no unconditional release on the invoke result),
`nullable_field_read_stays_fail_closed` (unguarded, inside-null-arm,
unknown field → `IncompleteOrdinaryNewCoverage`),
`resolved_semantics` 349/349, `field_read` family 29/29,
`borrowed_source_publication` 14/14, deterministic ×3.
Baseline reds observed while gating (all already classified):
`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
`main_static_child_port_consumes_all_role_rows_once`,
`qualified_call_map_argument_reaches_the_named_capability_boundary`, and
`refresh_module_user_box_method_routes_accepts_loop_carried_nullable_object_return`
reproduce at clean `8f9fde4bdd` HEAD identically.

Open frontiers (deliberately out of this slice):
formal/parameter field reads (`handle.block_id` inside
`release(handle)`), `.get` element reads (`me.pages.get(..)`), and
`me.`-receiver nullable calls (`artifact-source-unavailable`/
`TerminalHomesUnavailable`).

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-PARAMFIELD-S0
(parameter-field read on a guarded nullable formal — the
`handle.block_id` boundary inside `release(handle)` toward
`releaseLocal` admission).

## PARAMFIELD frontier census (2026-10-03, before fork Decision)

Before the accepted Decision below, `release(handle)` decomposed into at least
five stacked frontiers with no closed construction mapping. Probed boundary per shape
(`parameter_field_frontiers_stay_fail_closed` pins all of them
fail-closed):

| Shape | Observed terminal |
| --- | --- |
| callee `return 0` (unannotated i64) + `local r = s.release(h)` | `borrowed-result/result-contract-mismatch` — the `I64` corroboration demands a declared `: i64`; `Nullable` admits an unannotated contract plus the sealed claim, `I64` does not |
| callee `release(handle): i64` + `local h = new ..; local r = s.release(h)` | `artifact-actual-root-source-missing` — `main`'s birth actual reaches the local-commit handoff without a `root_source` |
| callee `if handle == null` | `TerminalHomesUnavailable` — `compare_operand_kind` admits only `>` + NormalInteger; an opaque formal's null compare has no envelope |
| callee `handle.page_id` (guarded or not) | `TerminalHomesUnavailable` — no `BorrowedFormalUseDraftKindV1` arm carries a field read, and an `OpaqueHandle` formal has no class authority to prove the field |
| caller `s.release(null)` / `s.release(h)` (h = `ReceivedNullable`) | `TerminalHomesUnavailable` / `IncompleteOrdinaryNewCoverage` — `BorrowedCallActualValueV1`/`BorrowedFormalActualSourceV1` have no null or received-nullable arm |
| caller `s.release(new Handle(..))` inline `new` actual | `borrowed-actual/unsupported-or-unavailable` — inline `new` is not an inventoried Home binding |
| caller `return s.release(h)` | `lexical-instance-call/terminal-result-mismatch` — terminal forward requires the callee's `Some(I64)` result row |

Transport already exists for the object lane:
`BorrowedFormalActualSourceV1::TypedHome`/`EntryReceiver` encode kind `3`
(object payload) and `Forwarded` encodes `"tagged"` — the caller-edge
carrier is not the blocker; the use vocabulary, result contract, and
actual arms are.

Accepted restart brief (2026-10-03; supersedes the pending fork brief):

```text
Decision: A = unannotated + complete source-I64 proof; B = complete
  incoming object-class view; C = dedicated null equality + kind 0;
  D = exact Null/ReceivedNullable actuals. Keep source declarations,
  the tagged carrier and one original ledger. Select I64RESULT-S0 only.
Source authority + canonical issuer: sealed return/use sites and retained
  Completion; existing borrowed result/source/actual issuers, operation
  issuer, field-read issuer and final lifecycle ABI owner named below.
Non-authority: absent annotation alone, first caller, field names, MIR
  types, payload zero, runtime layout and .hako annotation workarounds.
Fail-fast boundary: each selected source/actual/result/use relation must
  close before publication; later frontiers remain rejected on demand.
Smallest next slice: TASK4-I64RESULT-S0; then ROOTSOURCE, source-use size
  extraction, NULLCOMPARE, NULLACTUAL, field-issuer size extraction,
  PARAMFIELD, FIELDRESULT and PARAMFIELD-ACCEPTANCE.
Non-claims: no implementation or EXE result from this Decision; no Bool
  result, inline-new actual, .get, me.-receiver, app or migration completion.
```

## PARAMFIELD fork Decisions and ownership

This is the accepted construction plan, not an activation receipt. The census
errors above describe the unchanged implementation at this Decision. The
existing 14-variant fail-closed pin stays evidence until each selected shape
has a positive replacement; it must not become a permissive error catch-all.

### A — source I64 proof without a result annotation

Accept Unannotated with the existing source-I64 return vocabulary and exact
Completion correspondence. `ordinary_new_borrowed_formal_result.rs::
source_result`/`corroborate_borrowed_i64_result_v1` own this relation. Preserve
`DeclaredFunctionResultContractV1::Unannotated` and declaration-derived
`result_contract.rs::validate_result` returning None; executable I64 is a
borrowed projection, not a manufactured annotation. Check the actual declared
enum: None also represents Void and cannot authorize I64 by itself.

Both local-call and direct-return corroboration borrow the same complete
source-result evidence, including `corroborate_terminal_lexical_result_v1`.
Do not change Some(I64) to an unconstrained None acceptance. The initial arms
remain Integer literals and exact-I64 formals; field-derived I64 is its own
FIELDRESULT slice. Annotation-only, partial exits, foreign/duplicate sites,
mixed and unproved returns reject; other result lanes retain their authority.

### B — a borrowed class view from the complete source cohort

Choose incoming source-class composition, not a new declared UserBox formal
kind or a .hako edit. Formal kind stays OpaqueHandle. The existing
`ordinary_new_borrowed_formal_source.rs` preparation/ledger owns a per-formal
object view, keyed by original owner/binding/ordinal, and lends it to the
existing field-read issuer. This is a bounded proof for selected field uses,
not global source-level type inference or first-caller specialization.

Seeds are exact constructor-backed TypedHome classes, declaration-backed entry
receiver classes, and the sealed `NullableObject(C)` claim of an exact received
call result. Null contributes no class; forwarded formal/Copy edges preserve
identity. Close all incoming edges before claiming one C. A finite worklist
unions source class alternatives through forwarding; no first-seed success is
published while dependencies remain unresolved. A seeded forwarding component
may close after all its edges agree; an unseeded cycle, all-null component,
conflicting classes, scalar/unknown arm or outside-cohort edge rejects the
object-view profile. Mixed scalar/object transport remains valid elsewhere.

Source preparation observes only sealed producer/result identities, not live
Homes or a completed caller flow. Both existing prefix walks corroborate each
actual against those same seeds and prove root liveness through the call's
Normal/Fault outcome. Only complete co-seal activates the view. This separates
source discovery from lifetime proof and avoids a callee-result/prefix cycle.

The field-read request borrows this view through an explicit receiver arm;
do not inject its class into arbitrary alias strings or globally classify
StoredLocal::Handle as an owned object. Reuse `prove_local_field_read_batch`
and `terminal_home::local_read_field` for declaration/slot proof. The initial
capability is guarded scalar-I64 field read, with a terminating `== null` arm,
surviving-path non-null coverage and normal rebind/join invalidation. Non-null
alone never supplies class or lifetime. A borrowed formal owes no callee End.

Physical projection keeps the existing ObjectFieldGet/CanonicalFieldRef owner
and original base ValueId. The aligned parameter descriptor lends optional
`borrowed_object_view` = canonical object ID; transport-only parameters omit
it. Source class is resolved through canonical object membership once. C's
existing index validates incoming object identity against that row and field
identity against CanonicalFieldRef, never against the first field read. A
tagged base additionally needs exact use/dominance coverage and a kind-3 check
before the existing checked accessor. No new field execution path is created.

### C — null equality is a separate operation envelope

Keep `Greater(NormalInteger, NormalInteger)` strict. Extend the existing
`dynamic_operator_contract` owner with the bounded borrowed-value/null equality
envelope documented in `dynamic-operators.md`. First admit direct-if `==` with
one original formal/Copy and an exact null literal in either operand order.
Its normal result is TrivialBool, with no owned result or operand mutation;
well-formed non-null kinds compare false rather than faulting as non-Integer.
The owner must represent this envelope's non-suspending/read-only semantics,
not inherit the unrelated Dynamic Add envelope by its storage spelling.

Use the physical `borrowed_null_compare` row with original lhs/rhs/dst and
`predicate: eq`. Validate the null producer separately from Integer(0).
Emission tests the carrier's kind, retaining source evaluation order. The
false successor supplies non-null only; class/lifetime and Integer views need
their own proofs. General tagged equality, != and implicit check hoisting are
outside this first slice. Null/void's existing value law is unchanged.

### D — null/received-nullable actuals preserve ownership

Extend the same candidate -> actual -> final physical call chain. Keep
`borrowed_kind_payload_v1`, one source/MIR formal, and existing tags 1/2/3;
add tag 0 with payload exactly 0 for an authorized none value. A literal null
actual is `{kind:0,value:<exact null ValueId>}`. An exact ReceivedNullable
actual is `{kind:"nullable_typed_object",value:<Normal projection ValueId>}`;
the source `NullableObject(C)` claim and live caller-owned root authorize the
ABI owner's null-or-kind-3 selection. Forwarded pairs stay `kind:"tagged"`.
No arbitrary zero payload, unclassified HANDLE lane or host handle authorizes
either new arm. Birth transport and runtime/plugin ABI do not change.

Reuse caller checked cleanup once on Normal/Fault, with no callee retain,
transfer or End. ReceivedNullable remains an owned caller Home even when lent;
a source class claim by itself proves neither this ownership nor liveness.
Exact producer, call/result site, argument ordinal, root and complete incoming
coverage must agree through Rust, JSON, C index and emission together.

## PARAMFIELD ordered construction tasks

Token prefix below:
`MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-`.
Every row is one responsibility; I64RESULT-S0 and ROOTSOURCE-S0 are
landed and USESIZE-T0 is currently selected.

Selected construction row:
`MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-USESIZE-T0`.
Next construction row after its closeout:
`MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-NULLCOMPARE-S0`.

| Order | Suffix | Responsibility / predecessor |
| --- | --- | --- |
| 1 | I64RESULT-S0 | A: unannotated source-proven I64; landed |
| 2 | ROOTSOURCE-S0 | Retain verified main source for birth/local-call actuals independently of terminal-map presence; landed |
| 3 | USESIZE-T0 | BoxShape: extract existing operation-use classifiers from the 771-line source-use parent; after 2, before new null arm |
| 4 | NULLCOMPARE-S0 | C: exact null-equality source envelope and physical use; after 3 |
| 5 | NULLACTUAL-S0 | D: literal null and received-nullable borrowed actuals; after 4 |
| 6 | FIELDSIZE-T0 | BoxShape: extract existing field-read batch issuer/stager from the 770-line source parent; after 5 |
| 7 | PARAMFIELD-S0 | B: complete class view + guarded scalar field initializer; after 6 |
| 8 | FIELDRESULT-S0 | Exact scalar field-return result proof using 7's view; after 7 |
| 9 | PARAMFIELD-ACCEPTANCE-R0 | Selected source-to-EXE null/object/nullable cases and inverse failures; after 8 |

### 1 — I64RESULT-S0 (landed)

- Production change: selected borrowed lexical callees' result and direct-return
  corroboration in `ordinary_new_borrowed_formal_result.rs`; existing local,
  discard/nested and terminal lenders consume that same proof. Source I64
  classification and `result_contract.rs` declaration interpretation stay owned
  where they are. No global header/result/default change.
- Positive: unannotated Integer-literal and exact-I64-formal returns, multiple
  complete I64 exits, existing annotated I64, and original local/discard/nested/
  direct-return call forms. Assert source declaration remains Unannotated while
  the selected disposition/Invoke/result projection is I64.
- Negative: Void/other annotation, implicit/bare return, Bool/Text/null-only,
  mixed return classes, missing/extra/duplicate exit, wrong owner and unproved
  projection. Keep nullable-result and all-ordinal actual regressions.
- Publication/EXE witness: unused opaque formal with immediate actual and a
  direct caller Return; this avoids independently blocked main local-new
  actuals. Package-level continuation tests do not claim those actuals execute.
  The newly accepted local-new shape may advance to ROOTSOURCE's named stop.
- Scope: result owner/tests, exact publication positives/negatives, owner README
  and existing focused guard pins. Other 14-variant frontiers retain exact
  rejection evidence; promote only result-covered cases and re-pin their next
  named stop. No full-field or app acceptance is owed by this row.

### 2 — ROOTSOURCE-S0

- Production owner: `ordinary_new_local_commit/finalized_root_handoff.rs::
  seal_finalized_root_birth_handoff`, its existing finalized-source projection
  and compiled-entry borrower. Source-handoff presence follows verified main
  identity/Completion plus retained birth/local-call inventories; it must not
  depend solely on `!terminal_relation.is_empty()`.
- Preserve exact existing owner, app-main identity, target, construction site,
  destination, receiver and finished producer joins. An actually empty terminal
  map is legitimate transport data, not a fabricated terminal or cleanup proof.
  Do not remove the missing-source checks without supplying this original loan.
- Positive: `local h = new Handle(..); local r = s.release(h); return r`
  with unused formal/I64 body, plus local/discard main uses without a terminal
  Call. Birth and actual source remain inventoried; cleanup is once on outcomes.
- Negative: missing/foreign root source, wrong main owner/identity, changed birth
  site/receiver/arguments and a missing original producer reject before JSON.

### 3 / 6 — the two invariant size prerequisites

USESIZE-T0 moves existing compare/add/source-operand classifier bodies into a
private child of the same borrowed-use owner; FIELDSIZE-T0 moves the existing
field-read batch issuer/stager into a private child of its source issuer.
Keep exact predicates, evaluation/error order, signatures, forwarding closure
and all old rejection arms; change only required private visibility/imports and
relocate existing guard source pins. Do not mix null or field admission into
these T0s. Validate moved-code equivalence and nonzero existing focused suites.
Use responsibility-specific test children; the publication-new-argument parent
is already 727 lines and physical use parent 720. Split further only when the
selected implementation needs it; never compress or exceed the 800-line stop.

### 4 — NULLCOMPARE-S0

- Owners: source-use child, `dynamic_operator_contract`, original borrowed view
  ledger, compiled-entry use verifier, JSON and existing V2/V4 index/emitter.
  Install only exact equality rows, preserving greater/add permissions.
- Positive: guarded ignored/formal-only I64 body with live TypedHome, Integer(0)
  and Bool(false) actuals; all compare unequal to null without an Integer fault.
  Check both source operand orders, original null site and Bool branch result.
- Negative: arbitrary tagged equality, non-null sibling, forged zero-as-null,
  original/final site or operand drift and unsupported use reject. No field
  permission arises from non-null. Null-true source EXE belongs to row 5.

### 5 — NULLACTUAL-S0

- Owners: `home_local_call_borrowed_actuals.rs`, both prefix walks, original
  borrowed actual/result lenders, final incoming verifier and physical/C call
  transport consumers. No nullable argument may be repaired as constant tag 3.
- Positive: exact null, owned ReceivedNullable in both runtime states, existing
  TypedHome and original formal/Copy forwarding; null compare returns true only
  for the null state. Verify identical source arity and intact pair transport.
- Negative: Integer(0)/Bool(false) substituted for null, malformed 0/nonzero
  payload, fake/foreign nullable producer or class, consumed root, wrong site/
  ordinal, missing incoming arm and host/boxed domain reject before artifact.
  Callee End is zero and caller release-if-live is exactly once on each outcome.

### 7 — PARAMFIELD-S0

- Owners: existing borrowed source/actual ledger for the complete class view;
  existing prefix non-null flow and field-read batch/terminal-home issuer for
  exact scalar field; existing parameter descriptor, incoming index and
  ObjectFieldGet consumer for physical corroboration. No declared formal kind.
- Positive: `if handle == null { return 0 }; local id = handle.page_id; return 0`
  with literal null, TypedHome(C), ReceivedNullable(C) and seeded forwarding.
  Null returns before any access; non-null publishes one exact ObjectFieldGet
  based on the original tagged formal/Copy with matching object/slot.
- Negative: unguarded/null-arm/non-dominated read, rebind or lost join proof,
  unknown field, conflicting same-layout classes, missing incoming proof,
  all-null/unseeded cycles, scalar/nonobject arm, wrong canonical object/slot,
  missing view row and physical base substitution reject. Borrow is no-End.
- Accept initializer form first: terminal `return handle.page_id` remains row
  8's named result frontier, so result admission cannot conceal field-read debt.

### 8 — FIELDRESULT-S0

- Borrow the exact source field-read scalar proof in the existing I64 result
  preparation/Completion corroboration and lexical/terminal consumers. Arrange
  passive declaration/view preparation before prefix walks and final co-seal
  afterwards; do not infer the result from emitted FieldGet/MirType.
- Positive: unannotated guarded `return handle.page_id` with an I64-compatible
  null-arm literal; all complete scalar field/literal exits have one executable
  I64 projection while the declaration stays Unannotated.
- Negative: object/unknown field, missing or unguarded field proof, mixed
  classes/results, wrong owner/site, partial return coverage and result-value
  substitution reject. Returning a borrowed object is not admitted here.

### 9 — PARAMFIELD-ACCEPTANCE-R0 and shared checks

The selected witness is the original frontier shapes, promoted one by one:
unused formal/I64 result, local-new TypedHome call, exact null guard, guarded
field initializer/terminal, literal null and received-nullable arguments.
Execute their normal/null/fault cases through the real Rust -> physical JSON
-> C -> OBJ/link/EXE path, not an injected receipt as source authority. Check
one caller cleanup per required exit and no callee disposal or retry. Keep
inline `new` actuals, general field/result aliases, .get, me.-receiver calls and
Bool-returning production release outside this series with named failures.

Each S0 requires source positives/negatives, original-MIR and publication
mutation negatives, its real selected physical witness, touched-owner
regressions, format/diff/pointer and qualified-route guard. Run Rust tests with
`CARGO_BUILD_JOBS=4 cargo test --profile quick --lib <nonzero focused filter>`
serially. Use the existing documented V4/OBJ/EXE driver and record exact command
and result on implementation; no new one-off check script is required.
Record named current-lib baseline comparison at each semantic closeout;
`brand_catalog_tests.rs=961` remains known structural debt, not guard PASS.
Update the touched owner README/reference with the code slice. Retirement
credit requires a real selected old edge and caller-zero; none is claimed by
this design or an additive capability row.

Design-only validation (2026-10-03): source/result/handoff/actual/field and C
consumers inspected. Post-edit pointer guard, TOML/selected-row/nine-task-order
checks and git diff --check PASS; added local Markdown links 3/3 PASS. Scope
check confirms exactly five documentation/pointer files changed. No code,
fixture, Cargo, physical compilation or EXE result is introduced by this plan.

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-I64RESULT-S0)

Landed (commit pending on `codex/birth-definition-publication`):

- Scope taken: an unannotated borrowed callee whose complete explicit
  value-return set is source-proven Integer/exact-I64 corroborates the
  executable I64 projection. `corroborate_borrowed_i64_result_v1` admits
  `row.result() == None` only when the retained declared contract is
  `DeclaredFunctionResultContractV1::Unannotated` — `Void` shares `None`
  and stays rejected — and `corroborate_terminal_lexical_result_v1`
  admits the unannotated row only beside the same owner-issued I64-class
  proof with `contract_corroborated`. The declaration is never
  reannotated, no `Verified*`/`Prepared*` receipt is created, and the
  source-result/contract owners stay where they are.
- The accepted local-new actual promoted to a real positive: local
  `new Item(..)` + `return s.release(h)` and its discard form publish the
  I64 invoke through `TypedHome` kind-3 transport; the bound
  `local r = s.release(h); return r` terminal stays pinned at
  `artifact-actual-root-source-missing` for ROOTSOURCE-S0.
- `ordinary_new_borrowed_formal_result.rs`=517, owner tests=394,
  publication-new-argument parent=723 (I64RESULT witness moved to a new
  145-line child so the parent stays under the 760-line design boundary).

Evidence pins (test profile `--lib`):
`borrowed_call_result_accepts_unannotated_complete_i64_source` (literal,
multi-exit and exact-formal positives; declared contract stays
`Unannotated`, `result()` stays `None`, proof corroborated),
`borrowed_call_result_keeps_unannotated_i64_bounded` (mixed exits,
implicit exit, `: bool`, `: void` and opaque-return negatives — the
opaque return stays outside the borrowed profile at the use-draft
frontier and never acquires the permission),
`unannotated_borrowed_i64_result_publishes_call_forms` (direct/bound/
discard/multi-exit plus typed-home positives: `ordinary_call` `i64`,
callee `ordinary_i64` role, `borrowed_kind_payload_v1` carrier, kind-1/
kind-3 actuals and `invoke_normal_result`),
`parameter_field_frontiers_stay_fail_closed` re-pins 12 named stops —
literal-null/inline-new/received-nullable actuals
(`borrowed-actual/unsupported-or-unavailable`,
`IncompleteOrdinaryNewCoverage`, `artifact-source-unavailable`) and the
bound root-source stop (`artifact-actual-root-source-missing`),
`borrowed_` sweep 207/207, package/`resolved_semantics` sweep 922/925 —
`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
`main_static_child_port_consumes_all_role_rows_once` and
`qualified_call_map_argument_reaches_the_named_capability_boundary`
reproduce identically without this change (baseline debt);
qualified-route scope guard PASS except `brand_catalog_tests.rs=961`
known structural debt; pointer guard PASS.

Open frontiers (deliberately out of this slice): bound-result root
source, null equality, null/received-nullable actuals, formal field
read, field result, Bool result, inline-new actuals, `.get`,
`me.`-receiver calls and the app/migration route.

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ROOTSOURCE-S0
(retain verified main source for birth/local-call actuals independently
of the terminal map).

## S0 landed record (MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ROOTSOURCE-S0)

Landed (commit pending on `codex/birth-definition-publication`):

- Scope taken: `seal_finalized_root_birth_handoff` now issues the root
  source loan from the union of retained inventories — terminal
  relations, checked birth actuals and lexical local-call groups — with
  the verified main owner carried explicitly on
  `FinalizedRootSourceHandoffV1`, so presence no longer derives from
  `!terminal_relation.is_empty()` and an actually-empty relation map is
  legitimate transport data. The missing-source checks
  (`artifact-actual-root-source-missing`,
  `artifact-local-call-root-source-missing`) stay verbatim.
- The bound-i64 return class: `return_scalar` reclassifies a local bound
  to a proven-i64 source (literal store, exact formal, field-read or
  classified call result — the stored `SourceScalarKind::Integer` is the
  sole class authority) as `ReturnScalar::Integer`, and the terminal
  observer mints `TerminalI64ScalarReturnV1` (owner + exact return/value
  sites only) as `TerminalRelationV1::I64Scalar`. `result_abi` projects
  it to `FinalizedRootResultAbiV1::I64ScalarReturn` → compiled `I64`;
  `Value`/`OpaqueCall` still return `None` and a retained loan without a
  result ABI keeps failing at the unchanged
  `retained-root-result-missing` boundary.
- The card witness `local h = new Item(..); local r = s.release(h);
  return r` publishes end to end through the existing binding-group and
  generic-return lanes; bound literal/copy/trivial-add and the
  no-`new`-argument bound call publish the same way.
- File sizes: `home_terminal_relation.rs`=785,
  `home_new_prefix_terminal.rs`=632, `ordinary_new_local_commit.rs`=741,
  `finalized_root_handoff.rs`=319 — all under the 800-line stop.

Evidence pins (test profile `--lib`):
`bound_i64_scalar_issues_exact_terminal_relation` (bound literal, copy
chain and proven field-add mixes mint the exact owner/site relation),
`empty_terminal_root_source_keeps_verified_owner_without_abi` (the loan
retains owner and lexical groups with an empty terminal map; `result_abi`
honestly reports `None`),
`bound_i64_returns_publish_through_root_source` (publication witness:
`root_i64` main, `invoke_normal_result`, kind-3 typed-home actual),
`unproven_bound_returns_stay_fail_closed` (bound bool/object →
`retained-root-result-missing`; bound text → `artifact-source-unavailable`;
pure main → `artifact-root-completion-unavailable`),
`mixed_or_unproven_add_discards_terminal_and_all_staged_reads` re-pinned
(the two `local value = 1` suffixes promoted to the new positive test),
`parameter_field_frontiers_stay_fail_closed` re-pins 10 named stops,
`borrowed_` sweep 209/209, package/`resolved_semantics` sweep 924/927 —
the same 3 baseline reds reproduce identically without this change
(`birth_receiver_non_escape...`, `main_static_child_port...`,
`qualified_call_map_argument...`); qualified-route scope guard PASS
except `brand_catalog_tests.rs=961` known structural debt; pointer guard
PASS.

Open frontiers (deliberately out of this slice): pure-main bound returns
without any checked `new` (`artifact-root-completion-unavailable` is the
unchanged gate), bound bool/nullable/text/object results, null equality,
null/received-nullable actuals, formal field read, field result, Bool
result, inline-new actuals, `.get`, `me.`-receiver calls and the
app/migration route.

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-USESIZE-T0
(BoxShape extraction of the source-use classifiers).
