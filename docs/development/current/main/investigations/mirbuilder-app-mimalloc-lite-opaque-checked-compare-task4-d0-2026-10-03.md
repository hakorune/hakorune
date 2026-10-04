# mimalloc-lite opaque checked-compare task-4 D0 (dominated view uses)

Status: accepted task-4 Decisions; all nine PARAMFIELD-series rows landed (I64RESULT-S0 through PARAMFIELD-ACCEPTANCE-R0); bounded series complete — next work needs a new accepted Decision.
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
Every row is one responsibility; all nine rows —
I64RESULT-S0, ROOTSOURCE-S0, USESIZE-T0, NULLCOMPARE-S0, NULLACTUAL-S0,
FIELDSIZE-T0, PARAMFIELD-S0, FIELDRESULT-S0 and
PARAMFIELD-ACCEPTANCE-R0 — are landed. This bounded series is complete;
any further work needs a new accepted Decision.

Selected construction row: none — series complete.

| Order | Suffix | Responsibility / predecessor |
| --- | --- | --- |
| 1 | I64RESULT-S0 | A: unannotated source-proven I64; landed |
| 2 | ROOTSOURCE-S0 | Retain verified main source for birth/local-call actuals independently of terminal-map presence; landed |
| 3 | USESIZE-T0 | BoxShape: extracted operation-use classifiers to `ordinary_new_borrowed_formal_use_operands.rs`; landed |
| 4 | NULLCOMPARE-S0 | C: exact null-equality source envelope and physical use; landed |
| 5 | NULLACTUAL-S0 | D: literal null and received-nullable borrowed actuals; landed |
| 6 | FIELDSIZE-T0 | BoxShape: extract existing field-read batch issuer/stager from the 770-line source parent; landed |
| 7 | PARAMFIELD-S0 | B: complete class view + guarded scalar field initializer; landed |
| 8 | FIELDRESULT-S0 | Exact scalar field-return result proof using 7's view; landed |
| 9 | PARAMFIELD-ACCEPTANCE-R0 | Selected source-to-EXE null/object/nullable cases and inverse failures; landed |

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

### 7 — PARAMFIELD-S0 (landed)

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

### 8 — FIELDRESULT-S0 (landed)

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

## PARAMFIELD prerequisite landed records (I64RESULT-S0 .. FIELDSIZE-T0 — archived)

The six earliest landed records of the PARAMFIELD series —
I64RESULT-S0, ROOTSOURCE-S0, USESIZE-T0, NULLCOMPARE-S0, NULLACTUAL-S0
and FIELDSIZE-T0 — moved verbatim to
`mirbuilder-app-mimalloc-lite-opaque-checked-compare-task4-paramfield-landed-records-2026-10-04.md`
when this card crossed the 1000-line active-document boundary at the
post-series frontier census. Their evidence pins, EXE receipts and
frontier notes are unchanged.

#### PARAMFIELD-S0 — landed record

Landed (`02927e1f60` on `codex/birth-definition-publication`):

- Flow: `PrefixLocalFlow::mark_nonnull` now also narrows a self-rooted
  `Handle` stored local (not only `ReceivedNullable`), so the surviving
  join of the exact `formal == null` terminating arm marks the formal
  non-null; `FieldReadReceiverV1::GuardedFormal` spells that receiver —
  `Parameter` binding kind + non-null mark + live self root — and the
  request carries `formal = true` with `home = receiver`.
- Draft: `BorrowedFormalUseDraftKindV1::FieldReadOperand` admits exactly
  one `FieldAccess` receiver site dominated (same-sequence, strictly
  later) by an admitted null compare of the same formal; the guard
  pre-pass collects each formal's `== null` `if` sites. Unguarded,
  inside-arm, ambiguous and `!=`-operator uses stay `UnsupportedUse`.
- Co-sealed class view (`BorrowedFormalObjectViewV1`): each incoming
  actual is classified from exact source — claim-local `new` candidate,
  entry receiver, sealed `NullableObject` received value, resolved
  forward — while the exact `null` literal proves `Void` and never a
  class; mixed classes, scalars, unsupported or unresolved forwards mint
  no row. Class->object resolves through the same ordinary-box coverage
  and source object definition the field issuer consults. Views mint
  only for formals whose draft admits a field read.
- Verified-walk ownership: `formal_field_read_target` (draft admits a
  read AND its view minted) both excludes the callee from
  `seed_completion` and joins it to the verified walk — one completion
  authority proves the guard and issues the read. A viewless callee
  (null-only/conflicting callers) stays on the bounded sibling: the read
  is truthfully unclaimed and legacy `FieldGet` fails `unproved-copy`.
  `prove_local_field_read_batch` gained the `formal_class` arm feeding
  the same `alias_class` path; entry ledger corroborates sealed actuals
  against the view (`borrowed-entry/object-view-drift`).
- Physical/ABI: `borrowed_call_uses` gained `field_admissions` +
  `object_views` state and an `ObjectFieldGet` arm — dominated by the
  formal's non-null successor (anchored at each borrowed null compare's
  `else_bb`) and naming exactly the sealed canonical object
  (`undominated-view`/`object-view`/`field-coverage` close it).
  `finish` returns `param value -> object` per function index; the
  contract carries `borrowed_object_views` and the JSON encoder writes
  `object_view` only on `borrowed_kind_payload_v1` params. C:
  `hako_physical_params` admits the optional third key only with that
  representation and a declared layout; `lv4_borrowed_object_formal`
  admits the tagged-param base whose `object_view` equals the row's
  `object_id`. Extraction: `path_dominates`, `finish` and
  `verify_published` live in `object_field`/`closure` children
  (681-line parent under the 800 boundary).

Evidence pins (test profile `--lib`):
`guarded_formal_field_read_publishes_object_view` (param
`borrowed_kind_payload_v1` + sealed `object_view` naming a declared
layout; exactly one `object_field_get` on the formal base; zero
release/end on the formal; emits `hako-issued-param-field-handle.json`),
`parameter_field_frontiers_stay_fail_closed` (null-only caller →
`unproved-copy`; terminal field-return variants →
`borrowed-result/source-not-i64` pending FIELDRESULT-S0; inline new,
aliases, `!=`, unguarded and malformed uses keep their named stops),
`dominated_add_view_publishes_from_original_source` +
`dominated_set_view_publishes_from_original_source` 4/4 focused,
`borrowed` family 220/220, `normal_callable_semantic_package` family
green (the page_heap fixture's null-only callers stay bounded and its
claim census unchanged), scope guard pins PASS except
`brand_catalog_tests.rs=961` +
`normal_default_root_catalog_lifecycle_tests.rs=1351` known structural
debt; C witness
`published_lifecycle_v4_param_field_execution_test.py`: one issued
program executes (exit 1, `page_id=5` read), seven forged rows
(view dropped/changed, wrong transport, foreign layout, extra param
key, duplicate read, wrong object) reject.
Baseline reds observed while gating (all already classified):
`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
`main_static_child_port_consumes_all_role_rows_once`,
`qualified_call_map_argument_reaches_the_named_capability_boundary`
reproduce at clean `8fbfffd487` HEAD identically.

Open frontiers (deliberately out of this slice): terminal
`return handle.page_id` (FIELDRESULT-S0), `!=` guards, inside-arm
reads, `me.` receivers, `.get` element reads, argument-position field
reads, mixed/forward-chain views without a class seed, and EXE
acceptance (PARAMFIELD-ACCEPTANCE-R0).

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDRESULT-S0
(exact scalar field-return result proof over this view).

#### FIELDRESULT-S0 — landed record

Landed (`6f34f4c6c4` on `codex/birth-definition-publication`):

- Flow (`PrefixLocalFlow::field_home`): a `Handle` observation rooted at
  a `Parameter`-kind binding is live only while the binding is in the
  path-sensitive `nonnull` set — `me` (Receiver) and rooted local handles
  keep their unconditional answer, so an unguarded formal read no longer
  counts as a live terminal home.
- Field authority (`field_is_integer` in both the verified walk and the
  probing/source walk): after the owned-home and received-nullable arms,
  the class falls back to the co-sealed
  `source.formal_object_view(home)`; `nullable_result_integer_field`
  (re-export widened through `ordinary_new_coseal`) resolves the view's
  sealed class against the same ordinary-box coverage + canonical object
  definition and requires an exact `i64` declaration before the read is
  staged — one field-declaration authority, no runtime layout reuse.
- Result classification (`ordinary_new_borrowed_formal_result.rs`):
  `guarded_formal_i64_field` admits `return formal.field` as I64 only
  when the body shape is an exact `FieldAccess`, the receiver is the
  exact declared parameter binding, that FieldAccess site is admitted in
  the draft as `FieldReadOperand`, the co-sealed object view exists and
  the canonical declaration is `i64`.
  `prepare_borrowed_i64_results_v1` now takes
  `&VerifiedInstanceConstructorSemanticBatchV1` so the classifier
  consults the same authority the issuer uses; uniformity across all
  explicit value-return sites is unchanged.
- Physical lane unchanged: the existing `I64Field` terminal relation and
  `take_terminal_field_read` consume the exact staged read; the ledger's
  home check additionally accepts a formal home proven by its co-sealed
  object view (no local commit needed — the physical verifier already
  checks the emitted base is the exact formal root in the non-null
  cone). One `object_field_get` + `Return`, zero release/end on the
  formal.

Evidence pins (test profile `--lib`):
`guarded_formal_field_read_publishes_object_view` (now two variants —
`handle`: initializer form returns 1; `return`: direct
`return handle.page_id` returns the emitted field-get dst; emits
`hako-issued-param-field-{handle,return}.json`),
`parameter_field_frontiers_stay_fail_closed` (rebound alias →
`borrowed-result/source-not-i64`; object-typed field → same; unguarded
and `!=` guards → `IncompleteOrdinaryNewCoverage`; mixed i64/nullable
returns → `borrowed-result/source-class-mixed`; inline-new/null-arg
actual arms keep `borrowed-actual/unsupported-or-unavailable` and
`artifact-source-unavailable`), focused suite 4/4,
`resolved_semantics` 349/349, `published_backend_view` 158/160 (the 2
are classified baseline), `normal_callable_semantic_package` 578/581
(same 3 baseline reds), scope guard pins PASS except
`brand_catalog_tests.rs=961` +
`normal_default_root_catalog_lifecycle_tests.rs=1351` known structural
debt; pointer guard PASS; C witness
`published_lifecycle_v4_param_field_execution_test.py` now runs two
issued programs — initializer answers 1, direct return answers 5
(`page_id=5`) — and seven forged rows still reject.
Baseline reds observed while gating (all already classified):
`per_new_actuals_survive_definition_dedup_and_are_consumed_once`,
`published_array_write_typed_contract_rejects_before_object`,
`birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
`main_static_child_port_consumes_all_role_rows_once`,
`qualified_call_map_argument_reaches_the_named_capability_boundary`
reproduce at clean `02927e1f60` HEAD identically.

Open frontiers (deliberately out of this slice): `!=` guards, inside-arm
reads, `me.` receivers, `.get` element reads, argument-position field
reads, object-typed/aliased field results, mixed-class results and EXE
acceptance (PARAMFIELD-ACCEPTANCE-R0).

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-PARAMFIELD-ACCEPTANCE-R0
(selected source-to-EXE null/object/nullable cases and inverse failures).

#### PARAMFIELD-ACCEPTANCE-R0 — landed record

Landed (`cf169b2c8f` on `codex/birth-definition-publication`):

- Emit (`borrowed_source_publication_param_field_acceptance_tests.rs`):
  the three frontier shapes without a prior C witness — unused
  formal/I64 result (`release(handle) { return 0 }` + direct caller
  return), local-new TypedHome call (`local h = new Item(1, 2);
  local r = s.release(h); return r`) and the exact null guard with a
  literal-null actual (`check(handle) { if handle == null { return 7 }
  return 3 }` + `s.check(null)`) — publish unchanged and write
  `hako-issued-param-acceptance-{unused-i64,localnew,null-guard}.json`,
  pinning the borrowed carrier and each actual's wire tag (1/3/0).
- Acceptance driver
  (`published_lifecycle_v4_param_field_acceptance_execution_test.py`):
  consumes those three plus the sibling witnesses
  (`hako-issued-param-field-{handle,return}`,
  `hako-issued-null-compare-object`,
  `hako-issued-null-actual-{null,nullable}`) — eight source-issued inputs
  through `published_lifecycle_v4_driver.c` -> OBJ -> runtime-probe EXE:
  - unused-i64 exits 0 with one caller release;
  - localnew exits 0 with both homes released once;
  - null-guard answers 7 on the null arm and 3 on the object actual;
  - field-init answers 1 and field-return answers 5 through the exact
    sealed `object_view`/`object_field_get` pair;
  - field-return's Void state (issued literal mutated 5 -> 20, same
    shape) transports (0,0) and the guard arm answers 0 with no release;
  - literal null answers 0 with the sentinel never released; the live
    received nullable releases h and s once each;
  - injected `V4_PROBE_FAULT_AT=2` birth-store faults on localnew and
    field-return unwind with exit 70, one reclaim and cleanup once;
  - five forged rows (dropped object_view, drifted object_id, forged
    Integer tag on the null actual, missing const_null producer,
    out-of-range wire tag) reject before OBJ.

Evidence pins:
`param_field_acceptance_shapes_publish_their_issued_inputs` (3/3 emitted
inputs carry the borrowed carrier and expected actual tag); the quick-lib
emitters run with `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
param_field_acceptance` plus the sibling emit filters
(`null_compare_publishes`, `guarded_formal_field_read_publishes`,
`borrowed_source_publication_tests::nullable`) — all green — then
`python3
lang/c-abi/tests/published_lifecycle_v4_param_field_acceptance_execution_test.py
/tmp` — `8 source-issued inputs execute (normal/null/fault); five forged
rows reject`. Scope guard pins PASS except
`brand_catalog_tests.rs=961` known structural debt; pointer guard PASS.

Open frontiers (deliberately out of this series): inline `new` actuals,
general field/result aliases, `.get` element reads, `me.`-receiver calls,
Bool-returning production release, `!=` guards, inside-arm reads,
argument-position field reads, object-typed/aliased field results,
mixed-class results — all keep their named stops — plus the
`releaseLocal`/`allocate` app admission itself.

Series complete — the nine ordered rows of the bounded PARAMFIELD
prerequisite/acceptance series are all landed; further work needs a new
accepted Decision.

## Post-series frontier census (2026-10-04, before next Decision)
(MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-REAL-RELEASE-D0)

The real `osvm_backed_fast_path_heap_box.release(handle)` — the shape
this ladder exists to admit — decomposes against the landed boundary:

```hako
release(handle) {
    if handle == null { me.reject_count = me.reject_count + 1 return 0 }
    if handle.page_id < 0 { me.reject_count = me.reject_count + 1 return 0 }
    if handle.page_id >= me.next_page_id { me.reject_count = me.reject_count + 1 return 0 }
    local page = me.pages.get(handle.page_id)
    if page == null { ... }
    ...
}
```

| Source shape in the real release | Landed authority | Gap |
| --- | --- | --- |
| `if handle == null` terminating guard | NULLCOMPARE-S0 + PARAMFIELD-S0 | landed |
| `handle.page_id` dominated by the guard | PARAMFIELD-S0/FIELDRESULT-S0 (initializer/return positions) | landed for initializer/return |
| `handle.page_id < 0` / `>= me.next_page_id` | guarded read exists; `compare_operand_kind` has no field-read operand arm | **operand-position field read — unadmitted; observed terminal `published-lifecycle/borrowed-use/unproved-copy`** |
| `me.pages.get(handle.page_id)` element read | owned `me.` field reads + `.get` are separate open lanes | **open frontier** |
| `me.reject_count = me.reject_count + 1` field write | owned `me.` field writes are outside the borrowed lane | **different lane — unadmitted here** |
| `me.route.release(handle)` borrowed forward | forwarded uses exist for call arguments | **`me.`-receiver method call — open frontier** |
| `Bool` result on release | result classifier keeps I64/Nullable only | **open frontier** |

Pending Decision brief (candidate ordering; acceptance still required):

```text
Decision:
Source authority + canonical issuer:
Non-authority:
Fail-fast boundary:
Smallest next slice:
Non-claims:
```

Proposed smallest next slice: operand-position guarded field read —
`handle.page_id` as a compare operand under the same admitted `== null`
guard — extending the existing `FieldReadOperand` admission and sealed
object view rather than minting a second read authority; the `.get`,
`me.`-receiver, field-write, Bool-result and inline-new cohorts keep
their named stops until their own Decisions.
