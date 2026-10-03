# mimalloc-lite opaque checked-compare task-4 D0 (dominated view uses)

Status: accepted task-4 Decisions; I64RESULT-S0, ROOTSOURCE-S0, USESIZE-T0, NULLCOMPARE-S0, NULLACTUAL-S0 and FIELDSIZE-T0 landed; next PARAMFIELD-S0 (implementation pending).
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

## S0 landed records (task-4 dominated-view series — archived)

The five landed records of the original dominated-view series —
ADD-S0, ARRAYSET-S0, CTORARG-S0, RESULT-S0 and FIELDREAD-S0 — moved
verbatim to
`mirbuilder-app-mimalloc-lite-opaque-checked-compare-task4-s0-landed-records-2026-10-03.md`
when this card crossed the 1000-line active-document boundary during
NULLACTUAL-S0 closeout. Their evidence pins, EXE receipts and frontier
notes are unchanged.

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
Every row is one responsibility; I64RESULT-S0, ROOTSOURCE-S0,
USESIZE-T0, NULLCOMPARE-S0, NULLACTUAL-S0 and FIELDSIZE-T0 are
landed and PARAMFIELD-S0 is currently selected.

Selected construction row:
`MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-PARAMFIELD-S0`.
Next construction row after its closeout:
`MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDRESULT-S0`.

| Order | Suffix | Responsibility / predecessor |
| --- | --- | --- |
| 1 | I64RESULT-S0 | A: unannotated source-proven I64; landed |
| 2 | ROOTSOURCE-S0 | Retain verified main source for birth/local-call actuals independently of terminal-map presence; landed |
| 3 | USESIZE-T0 | BoxShape: extracted operation-use classifiers to `ordinary_new_borrowed_formal_use_operands.rs`; landed |
| 4 | NULLCOMPARE-S0 | C: exact null-equality source envelope and physical use; landed |
| 5 | NULLACTUAL-S0 | D: literal null and received-nullable borrowed actuals; landed |
| 6 | FIELDSIZE-T0 | BoxShape: extract existing field-read batch issuer/stager from the 770-line source parent; landed |
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

### 3 / 6 — the two invariant size prerequisites (both landed)

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

### 4 — NULLCOMPARE-S0 (landed)

- Owners: source-use child, `dynamic_operator_contract`, original borrowed view
  ledger, compiled-entry use verifier, JSON and existing V2/V4 index/emitter.
  Install only exact equality rows, preserving greater/add permissions.
- Positive: guarded ignored/formal-only I64 body with live TypedHome, Integer(0)
  and Bool(false) actuals; all compare unequal to null without an Integer fault.
  Check both source operand orders, original null site and Bool branch result.
- Negative: arbitrary tagged equality, non-null sibling, forged zero-as-null,
  original/final site or operand drift and unsupported use reject. No field
  permission arises from non-null. Null-true source EXE belongs to row 5.

### 5 — NULLACTUAL-S0 (landed)

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

#### USESIZE-T0 — landed record

BoxShape extraction, no semantic change: `ordinary_new_borrowed_formal_uses.rs`
(771 → 552 lines) keeps the draft types, `draft_borrowed_incoming_calls_v1`,
`join_borrowed_forward_uses_v1` and the main use loop; the six operand
classifier/helper bodies (`compare_operand_kind`, `normal_integer_operand`,
`add_operand_kind`, `is_call_argument`, `use_dominated_by_if`,
`sequence_ordinal`) move verbatim into the new private child
`ordinary_new_borrowed_formal_use_operands.rs` (246 lines). Only private
visibility changed (`pub(super)` + one `use operands::{...}` re-export in the
parent so `array_element`/`new_argument` children resolve their existing
`super::` imports untouched). Evidence: git-verbatim diff of moved bodies
(only the `fn`→`pub(super) fn` prefix and one trailing blank line differ);
`borrowed_` sweep 209/209 identical to baseline; package sweep 575/578 —
the same 3 baseline reds; scope guard pins all moved symbol spellings and
the parent shrink, PASS except `brand_catalog_tests.rs=961` known structural
debt; pointer guard PASS. No null/field admission, no test changes.

#### NULLCOMPARE-S0 — landed record

Landed (commit pending on `codex/birth-definition-publication`):

- Semantic envelope: `dynamic_operator_contract` gains
  `DynamicOperatorFamilyV1::Equal`, `DynamicOperatorValueClassV1::Null` and
  `DynamicOperatorSuspensionV1::NonSuspending`; the issuer seals exactly one
  new envelope `Equal(Dynamic, Null)` — `TrivialBool` result, no carrier
  lifecycle obligation, both source operand orders. Existing envelopes keep
  `MaySuspend` and are unchanged.
- Source admission (`ordinary_new_borrowed_formal_use_operands.rs::
  null_compare_operand_kind`): a formal use drafts
  `NullCompareOperand{binary}` only when exactly one binary expression
  contains the candidate site, the operator is `Equal`, the binary is a
  direct `if` condition, the sibling is the exact
  `ResolvedLiteralSourceV1::Null` literal, and the envelope issues. `!=`,
  integer `0`, Bool `false`, another binding as sibling, ambiguous multiple
  matches and equality outside a direct `if` all stay `UnsupportedUse` —
  the unchanged fail-closed artifact boundary.
- Projections: `null_compare_uses` (entry) and
  `borrowed_ordinary_null_compare_uses_v1` (finalized source projection)
  reuse the same owner/function binding checks as the existing borrowed-use
  projections.
- Use verifier: `FunctionUses.null_admissions`, `Scan.null_uses` and
  `ViewScan.null_consts` (exact `ConstValue::Null` producers). `Compare{Gt}`
  keeps the checked normal-integer compare path; `Compare{Eq}` admits a
  tracked or lent operand only beside an exact null producer; every other
  compare operator touching a tracked/view operand stays
  `borrowed-use/forbidden-operand`; per-admission coverage closes at
  `borrowed-use/null-coverage`. The null-verification block lives in the new
  `borrowed_call_uses_null_compare.rs` child so the parent stays at 785
  lines (<800).
- Physical: the JSON emitter spells admitted null equality as
  `borrowed_null_compare` (`predicate: "eq"`, original lhs/rhs order, exact
  `const_null` sibling) while ordinary integer equality keeps `compare`.
  The C parser registers `const_null`/`borrowed_null_compare`
  destinations, validates the exact five-key shape and eq-only predicate;
  the index seeds the row as `LV4_BOOL`; the flow admits exactly one
  carrier (TAGGED/HANDLE/I64/BOOL lane) beside the exact null producer and
  admits `const_null` in a function carrying the dedicated row; emission
  reads the carrier's kind/payload lane — never the Integer view — so a
  well-formed non-null kind answers false without a fault branch.
- Publication/EXE witness: `null_compare_publishes_the_dedicated_physical_row`
  emits three source programs (`handle == null`, `null == handle`, object
  argument) — `Store.check/1` keeps `borrowed_kind_payload_v1`, no `compare`
  row appears, two `borrowed_null_compare` rows (edge-port model) carry the
  exact `const_null` sibling. The C execution test compiles all three to
  .o, links the runtime probe and executes them: every non-null carrier
  answers false (exit 3, no fault); seven forged physical rows (wrong
  predicate, ordinary-compare spelling, carrier/carrier, null/null,
  undefined operand, extra key, foreign `const_null`) reject.

Evidence pins (test profile `--lib`):
`null_compare_admits_direct_if_equal_with_null_sibling`,
`null_compare_rejects_wrong_operator_sibling_and_position`,
`borrowed_use_null_compare_view_passes_in_either_operand_order`,
`borrowed_use_rejects_null_compare_operator_and_sibling_drift`,
`borrowed_use_null_compare_coverage_is_per_admission`,
`non_null_equalities_stay_fail_closed_before_publication` (`!=`, `== 0`,
`== false`, `q == handle` all stop at `artifact-source-unavailable`),
`parameter_field_frontiers_stay_fail_closed` re-pinned (`pa-eqnull-newarg`
now reaches `borrowed-actual/unsupported-or-unavailable`,
`pa-eqnull-localnewarg` reaches `admission-function-not-birth` — both
correct frontier advances), dynamic-operator contract suite,
`borrowed_` sweep 216/216, package/`resolved_semantics` sweep — the same
3 baseline reds reproduce identically without this change
(`birth_receiver_non_escape...`, `main_static_child_port...`,
`qualified_call_map_argument...`); all four existing V4 execution tests
rerun green; `published_lifecycle_v4_null_compare_execution_test.py`
PASS; qualified-route scope guard PASS except `brand_catalog_tests.rs=961`
known structural debt; pointer guard PASS.

Open frontiers (deliberately out of that slice): `null`/ReceivedNullable
actuals (NULLACTUAL-S0 — landed below), formal field reads under the
non-null successor (PARAMFIELD-S0), and everything the frontier test
still pins.

#### NULLACTUAL-S0 — landed record

Landed (commit pending on `codex/birth-definition-publication`):

- Candidate vocabulary (`home_local_call_borrowed_actuals.rs`): the exact
  `ResolvedLiteralSourceV1::Null` literal observes as its own
  `BorrowedCallActualValueV1::Null` — never Integer(0)/Bool(false) — and
  a stored `ReceivedNullable` binding
  (`PrefixLocalFlow::is_received_nullable`, no non-null narrowing mark
  required — the actual carries both runtime states) observes as
  `BorrowedCallActualValueV1::ReceivedNullable(binding)` only when the
  bound value is the exact binding itself; `local q = h` aliases stay
  `Binding` and keep their own rejections.
- Ledger (`ordinary_new_borrowed_formal_actuals.rs`):
  `BorrowedFormalActualSourceV1::Null` and
  `::ReceivedNullable{binding, class}`; the class comes solely from the
  sealed `NullableObject(C)` claim through
  `lexical::nullable_received_result_class` — a fake, foreign, consumed
  or unclaimed producer closes at
  `borrowed-actual/nullable-class-unavailable`.
- Emission: `Source::Null` emits an exact `ConstValue::Null` and rides
  `BorrowedLiteral` projection (verified against the exact producer);
  `Source::ReceivedNullable` rides the binding-read lane exactly like
  Scalar/TypedHome. `emission_validation` now counts recorded `Emitted`
  bindings (clean + fault twins) plus root-local-call binding groups for
  owed exit releases — the `actual=2/owed=1` drift `pa-trivial-handlearg`
  exposed was recorded-group bookkeeping, not a second emitted release.
- Wire: `Null` spells kind `0`, `ReceivedNullable` spells
  `nullable_typed_object` in `borrowed_kind_payload_v1` args. The C call
  transport admits numeric tags 0..3 plus the symbolic kind; indexed
  flow requires the exact `const_null` producer for tag 0 and a live
  HANDLE-origin value for `nullable_typed_object`; the dominated
  tagged-formal gate and the physical formal-transport gate now serve
  `ordinary_nullable_handle` callees as well as `ordinary_i64`; the
  callee prologue admits the (0,0) pair; the caller emits
  `payload != 0 ? 3 : 0` before the call line so the callee's own
  prologue re-proves the pair.
- Publication/EXE witness:
  `null_and_nullable_actuals_publish_their_wire_forms` — the literal-null
  2-box fixture's arg carries kind 0 beside its sole `const_null`, the
  `Handle/Store.check/Store.release` fixture's arg carries
  `nullable_typed_object` beside `invoke_normal_result`. The C execution
  test compiles both and runs three outcomes through the runtime probe:
  literal null releases HOME=1 (root Store only — the sentinel never
  owns a lease), live nullable releases HOME=2 (`s` + `h` via
  `home_release_if_live` exactly once), the null-state nullable
  transports (0,0) and releases nothing; the callee End stays zero in
  every case. Seven forged physical mutations (tag over-range, forged
  tag 0 on a `const_i64` producer, stale literal tag 3, scalar-lane
  value, extra key, result-role drift, `const_null` in a function with
  no admitted null lane) all reject before artifact or at admission.
- Frontier: `pa-trivial-handlearg` is the promoted positive — removed
  from `parameter_field_frontiers_stay_fail_closed`'s fail-closed list;
  the `local q = h` alias variant keeps the exact-binding rejection.

Evidence pins (test profile `--lib`):
`exact_null_actual_selects_the_null_source_not_a_scalar_payload`,
`null_and_nullable_actuals_publish_their_wire_forms`,
`non_null_domains_keep_their_own_tags`; borrowed-actual sweeps 219/219
and package/publication sweeps green; package/`resolved_semantics`/
`publication` sweeps — the same 6 baseline reds reproduce identically
on clean `160cd13a8e` (dynamic-loop prepare, dynamic compare+add
rebind, `birth_receiver_non_escape`, `main_static_child_port`,
`qualified_call_map_argument`,
`reserves_four_mechanical_lanes_without_builder_publication` —
`ObjectDefinitionsNotConsumed` on a plain string-scan fixture,
reproduced clean); C preartifact parser test RC=0, nested lifecycle
test RC=0, all existing V4 execution tests rerun green;
`published_lifecycle_v4_nullable_actual_execution_test.py` PASS;
`published_lifecycle_v4_execution_test.py`'s stale `bad_add` expectation
documents pre-existing `ee30614975` add-view behavior (reproduces on
clean baseline) and is untouched by this slice; qualified-route scope
guard PASS except `brand_catalog_tests.rs=961` known structural debt;
pointer guard PASS.

Open frontiers (deliberately out of this slice): guarded formal field
reads (PARAMFIELD-S0), the field-return result proof
(FIELDRESULT-S0), the bound-alias nullable variant and everything the
frontier test still pins.

#### FIELDSIZE-T0 — landed record

Landed (commit pending on `codex/birth-definition-publication`):

- BoxShape only: `prove_local_field_read_batch` and
  `stage_local_field_read_batch` move verbatim into the new private
  `field_batch` child `ordinary_new_coseal_issue_source_field_batch.rs`
  (111 lines); `issue_source` drops 780 -> 681 lines and re-exports both
  (`pub(super) use field_batch::{..}`), so the `source_claims::` call
  sites in `ordinary_new_coseal_issue.rs` and the `use super::*`
  staged-read tests resolve unchanged. Moved functions carry
  `pub(in crate::mir::normal_callable_semantic_package)` — the only
  required visibility change; no predicate, evaluation order, error arm
  or signature changed. Verbatim diff proven
  (`git show` body comparison identical modulo the visibility keyword).
  No null/field admission moved with it.

Evidence pins (test profile `--lib`):
`field_batch` staged-drift tests 2/2, `field_read` 29/29, `coseal`
274/274 identical; qualified-route scope guard PASS (pins relocated to
the child spelling) except `brand_catalog_tests.rs=961` known structural
debt; pointer guard PASS.

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-PARAMFIELD-S0
(complete class view + guarded scalar field read).
