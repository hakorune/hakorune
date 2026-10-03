# mimalloc-lite opaque checked-compare task-4 D0 (dominated view uses)

Status: accepted Decision; S0 bounded to the dominated ordered-Add view use.
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-D0`.
Related: docs/development/RULES.md; CURRENT_STATE.toml;
  mirbuilder-app-mimalloc-lite-opaque-checked-compare-normal-integer-d0-2026-10-03.md
  (parent; view lending rules and S0 receipts);
  mirbuilder-app-mimalloc-opaque-formal-ingress-d0-2026-10-01.md (grandparent).

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
