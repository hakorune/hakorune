# MirBuilder app bundle — mimalloc-lite completion D0 (2026-09-30)

Status: active bundle — Static validation scope S0 accepted; required Object forward S0 in progress, not accepted.
Scope: mimalloc-lite completion bundle; selected source contracts and retirement only.
Related: CURRENT_STATE.toml; docs/development/RULES.md; parent Gate-1 card below.

Parent card: `mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md`
(the Gate-1 thinning ledger). This card carries the one-app completion
bundle for `mimalloc_lite_exe` from the fixed `real-apps-exe-boundary`
suite.

## Current product and unfinished integration

| item | current evidence / next obligation |
| --- | --- |
| Selected product | Acceptance target: unchanged `apps/mimalloc-lite/main.hako` must reach canonical MIR/direct `--emit-exe`, execute, and match the existing suite output. This bundle remains unfinished; binary-trees is not selected by this card. |
| Source identity / latest app result | SHA256 `33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685`; latest fresh app probe exits1/no EXE at owner34 Body(1)/Initializer(0) → callee33 `HakoAllocHeap.allocate/1`. Completion is Ready; guarded input cohort is still uncompleted. |
| Required Object Forward S0 | Semantic WIP remains uncommitted and unaccepted. The later build-time card records95 focused PASS, including the original source-identity producer and all seven corruption refusals; both final physical publication positives remain owed. |
| Common input owner next | Inspect SAME original signature/reconstruction/whole-incoming gates in the existing typed Object input finisher. Do not retry a missing-Completion hypothesis or add a sibling result qualification. |
| Physical membership next | `Maker.qualified/0` still rejects in final admission. Align original declaration / incoming participant / execution membership through their own Completion, result and cleanup authorities; preserve actual call sets and both final consumers. Do not invent an OrdinaryCallSite or permit all-module execution. |
| Session state | User-requested design consultation and goal pause remain in effect. This update authorizes no implementation/build or resume; CURRENT_STATE points here for the paused integration consultation; host-tooling results remain in their own card. |

One integration series owns its source family through the final consumer. Focused
preparation is internal evidence; product progress is the original source's changed
first-stop or successful execution, production caller cutover, and actual retirement.
A feature addition is recorded as added coverage, not legacy deletion. Replacement
series retire their exclusive old edges and temporary assets when RULES sections4/6
prove caller-zero and required checks; shared owners and unselected callers remain.
No second Builder or whole-project rewrite is selected. An existing owner may be
rewritten coherently within the same production responsibility.

## Decision — MIRBUILDER-APP-BUNDLE-MIMALLOC-LITE-D0 (accepted)

Decision:
  `mimalloc_lite_exe` becomes one completion bundle: ordered internal
  slices, each with its own D0/S0 + positive/negative/Focused-gate
  verification; the app closes only when `mimalloc_lite_exe` passes
  end-to-end (MIR-JSON negative pin re-worded for the drifted terminal +
  pure-first EXE parity run).

Source authority + canonical issuer:
  Empirical probe census (fail-closed ladder terminals) +
  `ObjectDestructionDispositionV1` (`object_definition.rs`),
  `issue_construction_plan` (`instance_construction.rs`), the generated
  `CORE_METHOD_CONTRACT_ROWS_V2` manifest, and the sealed resolver facts
  already used by the prefix/home machinery. No MIR/runtime inference.

Non-authority:
  The 6-FAIL count is not a task count; VM-route behavior is not
  evidence for the EXE lifecycle lane; a passing focused unit test is
  not a production claim.

Fail-fast boundary:
  Every slice keeps `RetainedUnavailable`/`PrefixNotCovered` truthful —
  no permissive arms, no fallback emission. Unsupported shapes keep
  their designed terminal.

Probe census (post-`1eba1bcb11`, `--emit-exe --backend mir pure-first`,
minimal drivers — each isolates one family):

| probe | shape | first terminal |
| --- | --- | --- |
| `new Page{v:i64}` + `return 0` | baseline | compiles (nyrt gate) |
| `v: usize` field | usize scalar field | `artifact-source-unavailable` |
| `extra: usize = 0` default | usize + default | `artifact-source-unavailable` |
| `items: ArrayBox = new ArrayBox()` | container field | `artifact-source-unavailable` |
| `c: Child = new Child(1)` field-init | object field + user `new` init | `artifact-source-unavailable` |
| `new HakoAllocPage(0,8,4)` (real `using`) | usize+ArrayBox fields | `artifact-unowned-lifecycle-site` |
| `LayoutBox.class_size(0)` | static-box call | `artifact-root-completion-unavailable` |
| `c.get(): i64` no-arg | instance call | compiles (existing claim) |
| `c.add(5)` scalar arg | argumented call | `artifact-source-unavailable` |
| `new Outer(i)` object arg | object argument | `artifact-source-unavailable` |
| `local s = i.v` | `handle.<field>` read | `artifact-source-unavailable` |
| `return false/true` in unused method | Bool literals | compiles (needs reachable-fn recheck) |

Claim internals for the `usize` probe (pending-claims census):
`home_prefix=Ok`, `construction=Ok`, `argument_rows=Ok`,
`destruction=Unavailable(FieldType)` — the only failing component is
`object_definition.rs`'s closed `declared_type_name == "i64"` profile.
The claim's object must end its Home at every exit; a non-`i64` field
makes that unprovable today, so the row stays `RetainedUnavailable`.

Bundle slice inventory (each becomes its own bounded D0/S0; order is
the dependency order, not a strict serialization):

1. `DESTRUCTION-SCALAR-FIELD` — admit integer-scalar declared field
   types (`usize` etc., via `classify_numeric_type_name` authority) as
   trivially releasable alongside `i64`. Design Q: extend
   `PlainI64NoHook` or rename disposition; verify the physical release
   path treats `usize` identically. This is the universal first gate —
   every allocator box (`HakoAllocHandle.requested_size`,
   `HakoAllocPage`'s usize fields) trips `FieldType`.
2. `DESTRUCTION-ARRAYBOX-FIELD` — owned `ArrayBox` fields (4× on
   `HakoAllocPage`): child release plan at object end.
3. `DESTRUCTION-OBJECT-FIELD` — owned user-object fields
   (`HakoAllocHeap.small_page`/`medium_page`): nested release.
4. `FIELD-INIT-USER-NEW` — `small_page: HakoAllocPage = new
   HakoAllocPage(LayoutBox.class_size(0), …)`: the
   `ProviderConstruction` store arm admits builtin zero-arg `new` only;
   user-class field-init `new` with call arguments needs its own
   construction row. (Today → `FieldContractUnsupported`.)
5. `STATIC-CALL-CLAIM` — `LayoutBox.class_size/class_id/class_capacity`
   static-box calls: `artifact-root-completion-unavailable`.
6. `INSTANCE-CALL-SCALAR-ARG` — `heap.allocate(8)`, `heap.release(h)`:
   argumented instance calls (0-arg i64 already compiles).
7. `CALL-RESULT-ARG-POSITION` — `handles.push(heap.allocate(8))`: a
   live Invoke inside a core-method argument.
8. `HANDLE-FIELD-READ` — `handle.block_id`, `heap.small_page`,
   `small.alloc_count`: `handle.<field>` reads.
9. `OPAQUE-SCALAR-POSITION` — `requested_size` in `.set` args and
   `requested_bytes + requested_size` (allocate Body(8)+).
10. `OBJECT-FIELD-RECEIVER-CALL` — `me.small_page.seedBlocks()`,
    `me.small_page.allocate(size)`: `me.<ObjectField>.m(..)` (the
    ArrayBox lane of the nested-receiver S0 does not cover object
    fields — needs callee-claim authority, not the core manifest).
11. `RESULT-NEW-OBJECT-ARGS` — `return new HakoAllocHandle(me.page_id,
    block_id, requested_size)`: `me.<i64 field>` arg partially covered;
    BoundValue/OpaqueHandle arg positions remain.
12. `FORWARDED-RESULT` — `return me.small_page.allocate(size)`,
    `return same`, nullable `Handle`/`NullableHandle` results.
13. `SEEDBLOCKS-LOOP-ALIAS` — `local free_stack = me.free_stack` +
    `loop(i < capacity) { free_stack.push(i) }`: container-field alias
    locals + calls inside loop bodies (callable-loop domain).
14. `MIR-JSON-PIN-REWORD` — the negative leg now freezes
    `emission-binding-drift` (drifted inside the fail-closed ladder);
    re-pin when the app approaches.

Normal/Fault verification per slice: positive + negative pin tests in
`brand_catalog_*_tests.rs`, the `page_heap_fixture_result_claim_census`
frontier pin, official quick-profile serial, and the 3× emit-exe smoke.
Product completion requires the unchanged app EXE and execution result. Each
replacement series records its named production caller switch and retires replaced
exclusive edges / temporary assets in that same bounded series when caller-zero
and required acceptance/guards are proven under RULES sections4/6. Shared owners
and unselected callers remain; feature additions claim coverage, not deletion.
Retirement is not deferred wholesale until the entire app passes.

Non-claims:
  No promise on ordering beyond "destruction admission is empirically
  first"; slices may merge/split at their own D0s. No claim that
  unused-by-app methods (`realloc*`, `allocateResult`, `isLiveHandle`,
  `resizeInPlace`, `HakoAllocHandleResult` births) are required — the
  reachable-set boundary is a per-slice question. Bool literals are
  provisionally unblocked (probe_s) pending a reachable-function
  recheck.

Landed slices 1-4 (DESTRUCTION-SCALAR-FIELD, DESTRUCTION-ARRAYBOX-FIELD,
DESTRUCTION-OBJECT-FIELD, FIELD-INIT-USER-NEW — D0 decisions and S0
landed receipts) moved to
`mirbuilder-app-bundle-mimalloc-lite-landed-2026-10-01.md` to keep this
active card under the 1000-line pointer-guard bound.


## Accepted claim and scalar-expression history

Completed STATIC-CALL-CLAIM, INSTANCE-CALL-SCALAR-ARG,
CALL-RESULT-ARG-POSITION and HANDLE-FIELD-READ decisions/receipts are retained in
this card at `7171048755`. The current owner README retains their contracts.
Their focused positives/negatives passed; original app remained unavailable.
No Handle-argument ownership, nullable container escape, whole-app completion
or shared-owner retirement permission follows from those accepted slices.

Completed OPAQUE-SCALAR-POSITION's COSEAL-SIZE-T0 and
SCALAR-EXPRESSION-POSITION-S0 decisions, transaction/lifetime/condition refinements
and exact acceptance are retained in this card at `741d8a37cb` (sections with
those names). T0 publication: `0d60daa3f59155990aa216b207df3cf5da9d49f6`.
Scalar-expression12/12, existing local_field_read9/9 and selected-new/frontier
regressions passed. Final full-lib verification was KNOWN BASELINE:
8151PASS/126FAIL/56ignored; unchanged deterministic failure SHA256
`eed5d558e18359d7d8502c87a0bd54b42cdbcb8215f3ab505671169c34e6e497`.
The earlier nullable flake and qualified-route guard size debt remain historical
classified reds, not waived. Current pure scalar/field contract is in owner README:
whole-root proof before staging, exact sites, live borrowed alias roots,
Integer/Bool distinction and conditional RHS placement. Shared Dynamic readers
remain; no physical deletion credit or opaque/call/String-root admission.

Remaining opaque operation work is owned by
[opaque-formal card](mirbuilder-app-mimalloc-opaque-formal-ingress-d0-2026-10-01.md).
Bundle tasks10-12, mixed call/String roots, original unchanged app EXE acceptance,
production cutover and selected old-edge retirement remain required; only the
CURRENT_STATE-selected row grants current execution scope.

## Accepted input preparation slices (2026-10-12)

`96624eb74e` owns sibling S0 (51PASS/1 baseline) and corrected batch/owner mapping;
`c82ee43752` + `4d202191a8` own qualified input D0/S0 (53PASS/1 baseline).
Generic discovery borrows passive SourceObject input through SourceInstance;
dedicated qualification, execution/Normal/result/ABI gates remain. No source rewrite
or shared-owner physical deletion. Full receipts, times and negatives in those
commits; current contract is in owner README. Original unchanged app SHA256
`33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685` now stops
at LexicalInstanceCall/source-only-object-actuals,0.050s/no EXE. Test/CLI snapshots
match. Parent two-exit/document S0 `2f6f0a8a1a` remains accepted; its JSON/readback/
native1/2/73 evidence is not whole app or global retirement proof.

## Static final-incoming D0 — current boundary (2026-10-12)

Parent: `MIRBUILDER-APP-MIMALLOC-LITE-STATIC-FINAL-INCOMING-D0`.
Original app, current tested CLI bytes, pure-first/replay-none. Read-only trace:
- Requested Static: LayoutBox.class_id(owner17/batch13), Body(0)/Initializer(0)
  -> SizeClassBox.good_size(owner11/batch7).
- Rejecting SourceObject: HakoAllocHeap.allocateResult(owner34/batch30),
  Body(1)/Initializer(0) -> allocate(owner33/batch29).
- Different callee cohorts, same source graph. Sealed contract rows provide
  batch-slot joins; header enumeration alone is not owner identity.
Evidence: `/tmp/mimalloc-static-final-incoming-site.log`, `...-contract.log`,
`/tmp/mimalloc-static-final-current-map.log`; current-binary disassembly validates
memory offsets. Those offsets are diagnostic only, not authority/portable ABI.

Current checker deliberately validates whole-final-incoming before lending only
requested owner rows. Static packet checks original qualified Rc/call/parameters/
Completion first. README's opaque Object/entry global execution contract remains.
Decision: do not skip SourceObject, filter the shared checker, grant execution or
turn rejection into missing evidence. Explore a Static-specific split inside the
same owner: global source/inventory validation plus selected-callee execution
closure and original forward dependencies. Other opaque entry/Object contracts
must remain unchanged. SourceObject is retained, not promoted.

Initial D0 questions (resolved by the selected-S0 Decision below):
1. Loan T0 below binds the canonical catalog; Static source validation must
   borrow it, not copy authority or use executable ordered checks for SourceObject.
2. Define shared global uniqueness/definition/cardinality/error/missing/snapshot
   checks; all selected callee callers and required forward dependencies still
   require executable proof. Audit existing object-view agreement obligations.
3. Keep original qualification/Completion and validate-all-before-mark ordering.
4. Positive: unrelated healthy SourceObject retained, selected Static closes.
   Negative: same-callee later caller/forward missing, unrelated Err/missing or
   changed source snapshot, foreign original, missing Completion, partial mark.
5. Focused entry/packet/Object regression and unchanged original app first-stop;
   no physical deletion permission or Object activation inferred from passage.
Read-only worker supports the split as an explicit owner contract change, not
an already-authorized weakening. Canonical catalog loan and validation domains
are resolved by the selected-S0 Decision below; no external wait or completion.

## Original catalog loan T0 — accepted (2026-10-12)

The existing Owned parameter catalog is one immutable Rc allocation shared by
Verified/Installed package, completed context and ordinary ledger. Issuer attaches
its same loan once before finish/co-seal; missing/duplicate attachment grants no
permission. Owned records remain non-Clone, no copied authority or new receipt.
Test-only malformed descriptors preserve all earlier rejection assertions.

Evidence: quick test build/link494.796s; 95 focused PASS/1 pre-change NoIncoming
baseline. Same-allocation/context lifetime, foreign/missing/duplicate/ordinal,
entry/Static packet/Object input and all adapted mutation families covered.
`/tmp/mimalloc-original-catalog-t0-v2-runtime-receipt.json`.
CLI build/link232.574s; unchanged original app0.050s/rc1/no EXE, identical
LexicalInstanceCall/source-only-object-actuals first-stop. All13 changed Rust
bytes match test/CLI/current snapshots; CLI receipt:
`/tmp/mimalloc-original-catalog-t0-cli-v1-runtime-receipt.json`.
Pointer/diff/new-hunk format PASS; coseal/install whole-file format debt matches
parent (`/tmp/mimalloc-original-catalog-t0-format-baseline.json`), not cleaned here.
Warning signatures unchanged (812 recorded in both test builds); no suppression.
No phase/selection/global execution change, old-edge retirement or app success.

Parent Static final-incoming D0 borrows this canonical catalog. The Decision
below closes its global source versus selected-callee execution domains and
reverse formal forward dependencies. Preserve unrelated class-view agreement: only view-free
SourceObject may be passively revalidated; required dependency stays executable.
Pre-T0 same-source read-only view lookup for allocate formal(owner33/binding1)
is None (`/tmp/mimalloc-static-final-object-view.log`); diagnostic old-binary
provenance, not fresh T0/S0 execution or permission. No automatic promotion.

## Static final-incoming D0 Decision — selected S0 (2026-10-12)

Decision: one existing entry owner validates the complete final incoming source
graph, but qualified opaque Static packet execution obligations cover the selected
callee and its reverse original-formal forwarding dependencies. This changes only
the Static packet contract; ordinary entry/Object whole-final execution stays.

Source authority + canonical issuer: the SAME retained original parameter catalog,
source incoming inventory, final definition/origin drafts, original forward rows,
qualified Static Rc and SAME callee Completion. No new authority/receipt.
Non-authority: pending phase alone, method names, header numbering, runtime tags,
old diagnostic memory offsets, missing class-view rows as execution evidence.
Fail-fast boundary: global duplicate/definition/cardinality/Err/missing checks;
required executable phase, exact source identity, forward coverage and class-view
agreement remain mandatory. SourceObject passive validation lends no executable
actual/result/Normal/ABI/artifact permission. Validate all before any route mark.

Smallest next slice: `MIRBUILDER-APP-MIMALLOC-LITE-STATIC-FINAL-INCOMING-S0`.
Production mapping: existing borrowed_static_packet_actuals_v1 -> entry owner
common core, for LayoutBox.class_id -> SizeClassBox.good_size. Replace that
Static call's whole-final execution obligation; preserve the normal/Object wrapper.
Shared owner and other callers stay; no physical deletion permission in this S0.

1. Keep checked_borrowed_entry_incoming as unchanged default wrapper. A private
   core accepts optional execution-owner scope; only the qualified opaque Static
   branch selects scope. Put closure validation in existing entry_owner_view child
   and SourceObject revalidation in existing formal_actuals owner. Do not mint a
   second checker/receipt or broaden field visibility. Source760/800 still apply.
2. Seed selected callee. Compute finite reverse closure from ORIGINAL forwards
   whose callee_formal.owner is included, never from pending actual variants. Match
   exactly one final incoming/opaque actual by call/target/site/ordinal/binding/
   source_formal/callee_formal; corroborate both drafts' origins. Require the
   pending actual's original Forwarded variant; reject orphan pending forwards.
   Add source_formal.owner, repeat to fixed point. Dependencies require final
   definitions and nonempty incoming, not invented source-only completion.
3. Project the SAME original inventory over that execution set, preserving its
   global/selected-owner vetoes. Compare one-to-one with final calls, including
   only the existing requested target_static/owner_view extension. Compare call,
   callee, full arguments, Static Rc pointer or full Instance target identity.
   Missing/extra/duplicate required calls reject. Sanctioned omitted Static
   callers were excluded at issuance and stay excluded: this proves inventory
   coverage, not a new all-source-callable census.
4. Unselected SourceObject may remain passive only when exact original call/
   callee/arguments/Instance target agree, SAME catalog regeneration equals the
   entire pending snapshot, and its opaque callee formals have no object_views
   membership. Existing view membership rejects rather than skipping agreement.
   Required/dependency SourceObject still rejects. All other phases use existing
   executable ordered checks; qualification/Completion/mark ordering stays.
5. Existing packet tests: unrelated healthy SourceObject+selected Static positive;
   source phase/candidate/ordered/original drift, Err/missing, class-view member,
   selected source-only and default global rejection negatives. Forward negatives:
   pending Forwarded->Integer, orphan/duplicate/root/site/ordinal/target drift,
   dependency missing definition/incoming, required original call removed. Check
   failed co-seal leaves every route unmarked. Reuse entry/packet/Object regressions.
6. Build once, focused positive/negative filters nonzero, classify reds and record
   source/binary snapshots. Fresh unchanged original app first-stop before/after.
   Required pointer/diff checks; qualified-route guard's unchanged raw_loop caller
   count debt remains reported, never waived. No full suite/CI required here.

Non-claims: S0 acceptance, production cutover closeout, caller-zero/old-edge
retirement, whole SizeClassBox/app/MirBuilder completion remain unproved.
Read-only worker confirmed original-inventory projection and sanctioned omissions;
no remaining external dependency or required future-green construction condition.

### STATIC-FINAL-INCOMING-S0 receipt — accepted (2026-10-12)

Landed `7901d434fa`; full acceptance/provenance is retained in that commit.
Only qualified opaque Static uses original-forward reverse execution closure;
view-free unrelated SourceObject requires SAME catalog/full-snapshot revalidation.
Default entry/Object and required inputs keep whole-final executable checks.
No new authority/receipt/source rewrite/ABI/result/artifact grant or deletion credit.

- V7 focused82PASS/1 original NoIncoming baseline: packet15, entry18, Object
  identity9/port2/slot12, actual26; no rebaseline. Exact runtime receipt and snapshot:
  `/tmp/mimalloc-static-scoped-s0-v7-runtime-receipt.json` and `...-snapshot-v7.json`.
  Forward/phase/catalog/view/coverage corruptions reject before partial route marks.
- V7 reflow-only equivalence: `...-format-proof.json`; fresh CLI matches final6 Rust
  files. Existing whole-file fmt debt remains; new actuals/owner-view/scope clean.
- CLI build251.974s; original app SHA256
  `33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685`.
  pure-first/replay-none probe0.050s/rc1/noEXE; same LexicalInstanceCall/
  source-only-object-actuals. `...-cli-v1-runtime-receipt.json` records provenance.
- Fresh trace: scoped=true, GoodSize11/batch7, rejecting34->33. Original17->11 and
  33->17 make allocate33/batch29 REQUIRED. SAME sealed joins in `...-contract.log`,
  `...-map.log`, `...-forward-graph.log`, `...-tuple.log`; offsets diagnostic only.
- Warning signatures test812/CLI1735 unchanged. Pointer/diff pass; unchanged
  qualified-route guard debt raw_loop_child_entry count3 vs4 is NOT PASS/waived.
  Sources below760 (entry752, test parent556/child329).

Production calls scoped contract; full cutover/caller-zero/retirement and whole
SizeClassBox/app/MirBuilder remain unproved. Shared/default owners stay.

## Required Object forward D0 — current boundary (2026-10-12)

Selected row: `MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-D0`.
The internal input-closure gap is allocateResult(owner34/batch30),
Body(1)/Initializer(0), argument0 `size` -> allocate(owner33/batch29), opaque
formal1, REQUIRED by good_size <- class_id <- allocate reverse dependency.
The source has `if size <= 0 { return new HakoAllocHandleResult(0,2,null) }`
before the call. That syntax is not Integer/entry/execution authority by itself.

Smallest next design: inspect SAME source-only draft/original incoming and existing
operation/Normal proof owner for that guarded forward. Establish whether the
existing checked argument or borrowed entry finish can lend executable input at
this exact site without inventing class authority or reissuing the formal catalog.
Keep OpaqueHandle/tagged ABI, source evaluation/Fault order and true/false scopes;
Integer Normal proof and usize range checking remain distinct. Callee source-only
phase is not permission; all required callers/dependencies must still validate.

Worker checks the concrete authority gap and existing producer/consumer, then main
records one Decision and smallest implementation slice here. No source annotation
workaround, pending-phase cast, global execution waiver, new receipt chain, parked
lane reopening or whole-app claim. This is internal design work, no external wait.

### Owner-selected milestone and D0 refinement (2026-10-12)

Near milestone: unchanged mimalloc-lite, selected pure-first/replay-none physical
route, EXE generation -> execution -> exact suite output. Full retirement and
selfhost remain later REQUIRED milestones of MIRBUILDER-FINAL-PIPELINE-v1, not
removed or counted complete. Current work stays ONLY Required Object Forward D0.
After its smallest S0: focused tests, then the SAME original app; record first-stop
by terminal AND owner/site plus EXE/run result. If it does not move, audit repeated
proof construction/common owner before stacking another small patch. Do not add
unrelated improvement/audit lanes or caller-specific authority/receipts.

Read-only worker found existing Integer Normal authority: SAME CheckedIntegerGuard
Rc and BorrowedGuardedActual's exact call/ordinal/site/binding reach survive source
pruning. Current consumer uses it only as a domain seed. Compare materialization
of source-only definitions accepts completed STATIC scalar cohorts; Instance owner34
has no matching consumer. Typed Object finish requires ExactI64 scalar parameters,
so it cannot activate this OpaqueHandle input unchanged.

Decision direction: declaration-bound tagged carrier binding in existing entry
owner, checked Normal continuation in existing compare owner, and exact guarded
forward reconstruction in existing actual/input owner. Keep formal OpaqueHandle
and kind/payload ABI; use existing original catalog/signature, not caller consensus.
An unused declaration's complete-empty observed incoming may bind its DECLARED
carrier, never certify executable incoming or Integer/class/ownership/whole body.
Required callee33 incoming stays complete; no outgoing edge is omitted.

### MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-S0 Decision and bounded tasks (2026-10-12)

Decision: connect the existing declaration-tagged entry, checked Compare Normal
and guarded argument owners for this required forward. No caller consensus,
class inference, source rewrite, phase cast or separate proof ledger.
Source authority + canonical issuer: SAME original parameter catalog/signature,
source use draft and CheckedIntegerGuard/BorrowedGuardedActual; existing entry,
compare and actual/input owners issue and lend their corresponding obligations.
Non-authority: source-only phase, comparison syntax, diagnostic owner numbers,
physical metadata alone, missing packets or completed-empty fabricated inventory.

Bounded profile: original source-only InstanceBoxMethod with OpaqueHandle formals;
at least one checked Compare and one exact guarded outgoing Object input. EVERY
formal use must be CompareOperand or an exact guarded UnresolvedArgument; roots,
ordinals, bindings and declaration mode must match SAME original catalog. Project
original incoming with its vetoes preserved; only complete-empty observed incoming
may bind this declaration's tagged carrier. Nonempty real incoming stays covered.
Neither case grants an executable incoming loan, Integer entry or whole-body permit.

Smallest next slice (one responsibility: guarded required input forwarding):
1. Existing entry owner lends that bounded declaration view as SelectedTagged.
   Check supplied parameter kinds as well as roots/ordinals/bindings against the
   original catalog. Receiver law comes from its original declaration mode;
   all nonempty incoming modes agree. Do not infer mode from physical params.
2. Existing guarded-actual owner lends exact call/ordinal/site/binding/formal and
   SAME Compare Rc correspondence to the draft. No retained batch/signature copy
   is added solely to repeat source validation already closed at issuance.
3. Existing Compare owner accepts that Instance declaration view; physical lowering
   still emits the original checked comparison, preserves operand order/tag Fault,
   and final use closure independently verifies its Normal CFG dominance.
4. Existing actual/input owner reconstructs the guarded outgoing input from SAME
   original target/arguments/catalog/signature. Keep callee33's complete incoming
   and original forwarding dependencies. Update SourceObject demand-time rechecks
   to recognize only this reconstructed guarded contract; never allow arbitrary
   Executable phase replacement. Preparation proves a plan; final publication
   independently owes the actual Compare/Normal path, not a preconstruction result.
5. Validate zero-call tagged DECLARATIONS independently in BOTH final consumers:
   compiled_entry_contract/borrowed_call_incoming.rs and invoke_borrowed_calls.rs.
   Published target -> handoff.callables().owner_for_canonical_key; MIR side uses
   the unique canonical definition key/symbol -> root.callables() lookup. Demand
   finished source/owner/symbol, bounded profile, complete-empty original incoming,
   recorded entry values and exact params/carriers/receiver correspondence. Reuse
   BorrowedCallUses::entry for published full-use closure. Keep incoming==original,
   real-call actual coverage and physical/witnessed coordinate equality unchanged;
   add no fake packet, call slot or coordinate. Share the existing source lender.
6. Existing-owner positives/negatives cover the guarded declaration/forward and
   zero-call final verification. Refuse foreign/missing catalog/signature/guard,
   supplied kind drift, rebinding, pre-compare/branch mismatch, omitted real calls,
   missing physical Compare/dominance and outgoing execution after guard Fault.
   Bool/object fail at the original Compare; usize range remains its original
   write boundary. Body/constructor/result/cleanup residual gates stay mandatory.
7. Focused nonzero filters and required guards, classify reds, then rerun SAME
   unchanged mimalloc-lite with fresh binary provenance. Record terminal AND
   owner/site and EXE/run/output. If unchanged, inspect common proof owner before
   another patch. Commit/push only selected files; keep this card below1000 lines.

Read-only worker independently confirmed both zero-call gaps and existing canonical
owner lookup (also used by birth_calls.rs). This closes the D0 correspondence;
S0 is selected, not accepted. Changed source beyond760 requires responsibility
split before800; put bounded entry logic in its existing owner-view child.
Non-claims: Integer/class/ownership entry, whole body completion, S0 acceptance,
EXE success, production closeout/caller-zero/retirement and full product completion.

### User-requested restart handoff (2026-10-12)

Stopped during S0; HEAD `741d8a37cb` is pushed. Four UNCOMMITTED Rust files:
entry/owner_view, guarded_actual and compare_materialization (original draft).
Draft declaration profile/SAME guard correspondence/Compare lender are connected;
NO Cargo/build/test/app rerun yet. Pointer/diff PASS; no Cargo/rustc live at stop.
Resume tasks4-7 above: exact guarded input reconstruction and demand rechecks,
BOTH zero-call final consumers, focused negatives, then unchanged original app.
Retain/review these drafts; S0 NOT accepted. Compact landed history before adding
receipts; keep source below800 and card below1000; do not extend parked lanes.

### Restart preparation — responsibility split and history compaction (2026-10-12)

User requested this before input reconstruction/final-consumer work. Entry's
complete incoming checker moves unchanged into its private incoming child;
source projection, owner selection and entry values keep their existing owners.
Only sibling visibility of the shared core changes; no admission/phase/ABI change.
Completed early claim/scalar-expression history above now points to immutable
commits; selected Required Object Forward tasks1-7 and unfinished bundle conditions
remain here. Four original semantic draft files remain unaccepted and preserved.
Entry651 + incoming114 lines; card remains below500. Original checker body is
identical after module-path/visibility/rustfmt normalization; final parent blank
cleanup is token-equivalent. Existing quick/serial `borrowed_formal_entry` filter
62PASS/0FAIL (includes Static packet/scope negatives), build/link418s, tests0.12s.
`/tmp/mirbuilder-entry-split-tests.log` retains the run. The additional
`borrowed_static_packet` filter matched0 and is NOT evidence; no rerun needed
because the first filter already includes those module tests. Test worktree
includes the four retained unaccepted semantic drafts; this receipt accepts ONLY
the proven responsibility move, not their behavior or S0. No new tests added.
Pointer/diff/child-format and immutable-history-reference checks PASS. Scope guard
FAIL at unchanged raw_loop_child_entry count3 vs4 (HEAD bytes identical), known
baseline; not waived. Initial child-wrapper visibility error corrected before
successful build. Warning count812; no suppression. Split `a03d2c8597` contains only the responsibility move and owner routing;
this card commit owns history compaction and evidence. Semantic
draft hunks remain unstaged. No S0/EXE/retirement claim.

### Required guarded forwarding implementation refinement (2026-10-12)

Decision: reuse one original declaration/use lender for entry selection and exact
input reconstruction. Existing SourceObject identity remains retained in its
reconstructed actual phase; a phase substitution without SAME source/candidate/
ordered/guard reconstruction refuses. No separate signature/batch copy or ledger.
Source authority: original catalog, source_incoming and SAME guarded actual/Compare
Rc; selected original signature and callee Completion remain preparation gates.
Source-only definitions were excluded before transport forwards were joined.
Thus guarded34->33 is retained in original incoming/guard evidence but absent
from transport forwards. Reverse closure must add its exact revalidated source
owner, accepting only its bounded complete-empty declaration; never skip the
edge or demand fabricated incoming. Required33's all incoming remains projected.
Final orphan checks admit an existing transport forward OR this exact reconstructed
guarded source edge. Global source snapshot/Err/missing/view checks stay mandatory.

Both final consumers resolve uncovered declarations through canonical key/owner,
finished symbol, original empty inventory, recorded ordinals/values and physical
parameter/carrier/receiver correspondence. Published full-use closure maps original
guarded call/ordinal to final coordinates after all visitation; each final scan
requires the SAME physical Compare to dominate that outgoing argument. Backend
checked Compare sends wrong tags to Fault before icmp, so no synthetic MIR Normal
successor is issued. Unguarded ordinary forwards retain their original law.
Existing actual source-only selection helper moved into its Object-source child
for room below800; its logic is unchanged. Final projection lender now contains
its dedicated declaration correspondence check. Relevant owner contract docs and
focused tests are still being completed; this draft grants no S0/EXE acceptance.

### User-requested design consultation stop (2026-10-12)

Goal pause requested; HEAD remains pushed `3272688876`. All S0 changes and four
new Rust child/test files remain UNCOMMITTED. No Cargo/rustc live at stop; external
same-checkout release/LLVM build also ended (its exit/result was not observed).
No S0 acceptance, fresh CLI/app result, production retirement or product completion.

Evidence: check-v2 lib PASS. Guarded tests-v2 build361s,10PASS/4FAIL:
2 existing guard tests reproduce identical NoIncoming on SHA-verified prior V7
binary; 2 new declaration tests lacked the Object qualification fixture, now
corrected but untested. V2 saved binary regressions: Object input10PASS,
use closure24PASS, Static packet14PASS/1NEWFAIL, actual26PASS/1original baseline.
New Static failure is guarded declaration execution projection rejecting original
NoIncoming. V3 test compile rejected Vec-vs-Box metadata test; corrected, not rebuilt.
New physical zero-declaration and Static guarded-phase corruption tests await build.

Design issue for consultation: original inventory NoIncoming is an execution veto,
but only a clean registered completed-scan sentinel can lend an empty DECLARATION.
New project_declaration_v1 preserves other/global vetoes and rejects missing,
duplicate/mismatched sentinel or sentinel+real rows. Guarded lender and reverse
closure now use declaration versus executable projections; these latest edits are
UNBUILT/UNTESTED. Read-only audit confirmed sentinel may coexist with failed callers,
so no generic NoIncoming waiver is allowed; normal project remains unchanged.
Also review scope: current input finisher reconstructs every eligible guarded Object
cohort, while existing unrelated-Object regression expects passive inputs. The old
fixture itself meets the new bounded profile. Decide generic admission versus
required-consumer demand consistently before changing fixture expectations; no
fixture change or selected-demand restriction has been made for this issue.

Resume after consultation: settle that scope in one Decision, finish declaration
inventory negative matrix, then focused build/tests and SAME unchanged app with
fresh binary provenance. Current new source/lender/consumer paths are drafts.
Pointer/diff checks passed before pause; latest added inventory API still needs
format/type checks. Preserve the current worktree; do not commit unverified S0.

### Resumed scope Decision (2026-10-12)

Decision: retain generic preparation of every original Object cohort satisfying
the selected guarded profile. The existing issuer finishes Object inputs after
signature issuance and before Static finishers; Static demand is not an authority
for source/catalog/Completion/signature preparation. No demand selector is added.
The old independent-Static fixture's guarded relay meets the new profile: cover
its reconstructed plan and a genuinely unguarded passive input separately, keeping
global missing/Err/phase/snapshot/catalog/view vetoes and selected Static closure.
Source authority remains the SAME original declaration, inventory and guard;
plans still owe physical Compare dominance, tag Fault and real-call coverage.
The clean registered NoIncoming sentinel lends only declaration correspondence;
ordinary execution projection retains its veto. Lost/duplicate/wrong sentinel,
global failure, failed caller and sentinel+real rows must refuse.
Read-only worker independently confirmed this scope and owner order. No app source
change, new semantic receipt, S0 acceptance or whole-product completion follows.
Fresh pre-resume CLI (C-1 binary SHA `2b63b1a1604fce8d084df51afbdd0f95f601dd550fb702f0fdae96a51a9d52f7`)
still stops at LexicalInstanceCall/source-only-object-actuals in 0.035s, no EXE;
unchanged source SHA `33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685`.
Provenance/logs: `/tmp/mirbuilder-required-forward-resume/app-before.*`.

V4 verification: quick test build/link317s PASS; inventory9, typed-input10 and
physical-use24 PASS. New zero-call positive exposed a checker mismatch: raw MIR
retains receiver `Box(original class)` and opaque formal `Unknown|Integer`, while
the carrier column records receiver ExistingCallableI64/formal BorrowedTaggedValue.
Corrected final lender receives the canonical key from each original owner lookup;
receiver class/arity/mode, unique params and recorded tagged formals stay exact.
Tests now reject receiver Integer/foreign class and formal Bool in both consumers.
Static regression's unguarded Instance was incorrectly inspected by Static scalar
finishers. The entry owner now demands a unique original Static declaration mode
before those finishers; guarded Instance admission runs first. No NoIncoming waiver.
V4 broad publication45PASS/5FAIL: four old failing names reproduce on SHA-recorded
pre-S0 V7 binary; the new positive above is corrected, awaiting V5. Guard filter
13PASS/3FAIL includes two pre-S0 NoIncoming failures. Entry63PASS/1FAIL is the
unguarded mode collision above. Zero-match `compiled_entry_contract::borrowed_call_uses`
is not evidence; corrected `borrowed_call_incoming::borrowed_uses` matched24.
Logs/snapshots and baseline comparison remain in the resume directory. V5 is
building; S0 is unaccepted and its semantic changes remain uncommitted.

V5 verification exposed the remaining producer/consumer join: the new positive
passes declaration correspondence, then original real incoming still includes
relay Body(1)/Initializer(0) -> make, but final packet visitation omits that site.
All recorded local groups are moved at final handoff; the missing group was never
retained. Original unqualified sibling remains ExistingBorrowedResult, while the
receiver Object packet gate only selects a call's own Object source requirement.
Decision: the same bounded original guarded input may select the existing receiver
Object packet retention path; this is source membership, not result permission.
Demand regenerates guarded actuals; existing full-slot callee result, Completion,
teardown and caller CallReceived observation gates still issue the packet. Keep
its SAME original target/requirement, no copied sibling return qualification and
no omitted real call or fabricated packet. Retain packet+binding group through
the current emitter and final scans. Read-only worker confirms this exact join.
Static scalar selection also calls its cohort checker directly, so its SAME
original Static mode gate must live in that checker, not just the entry wrapper.
V5 324s build PASS; inventory/input/use positives remain green, two new failures
above remain unresolved until the next focused run. No S0 acceptance yet.

V6 was intentionally interrupted at76.66s after finding two unconverted consumers;
its Cargo/rustc ended before source edits. V7 build312s PASS: Static scope3,
inventory9, input10 and physical-use24 PASS; earlier guard NoIncoming tests now
pass under original-mode selection. Two new failures remain: packet is SourcePending
and physical receiver-object/local-source-missing. No changed input/guard waiver.
Holistic audit found source receiver observation precedes Completion, but only
Nullable observations select the homes-aware walk. The error's OR does not prove
that the earlier receiver observation is absent; plain Completion lacks the local
Handle/Home flow. Existing Handle self scan already handles this source.
Decision: the SAME original bounded guarded input plus the original Object receiver
observation selects that existing homes-aware walk before Completion. Reuse one
original guarded input query in the packet and walk selectors. The verifier must
issue the original CallReceived flow, prior-home unwind and cleanup; no completed
rows are fabricated afterward. Selected mapping/receiver/class producers remain
unchanged, as the audit found no permission gap there. No new scan arm or fixture
rewrite. Full-slot preparation and both final consumers still owe their checks.


V8 verification: quick test build/link310.81s PASS; guarded16PASS/1NEWFAIL,
entry65, inventory9, typed-input10, Static scope3 and physical-use24 PASS.
Broad publication45PASS/5FAIL: same four pre-S0 failures plus the new physical
positive. Fresh llvm CLI build233.09s PASS; binary SHA
`ae990a1117384319b67cd3b8fe3dbf0d712de77bfd91f2eca61a20e88d2f960e`.
SAME unchanged mimalloc-lite SHA above, canonical MIR/direct emit-exe,
pure-first/compat-none and fresh C-1 runtime/FFI: emit exit1 at0.04928s,
LexicalInstanceCall/source-only-object-actuals, no EXE. Exact argv/env, source
manifest and logs: `/tmp/mirbuilder-required-forward-resume/{cli-v8-*,app-v8.*}`.
No production first-stop advance or S0 acceptance is claimed.

Read-only V8 debugger attribution: final admission's first birth-validation
rejection is `Maker.qualified/0`, not the guarded relay. Saved unit binary and
log `/tmp/mirbuilder-required-forward-resume/admission-v8.log`; entry was inspected
with the Rust sret ABI name pointer, not guessed from MIR signature. This caller
owns the retained original terminal packet -> make and participates in its full
original incoming coverage despite having no incoming call itself. Source result
must use its own Direct Object-return lender, not uniform_call_result_kind:
that helper defaults empty maps and OwnedCall to I64. Zero-call declaration entry
alone still cannot grant result/body/cleanup permission.

Decision: before changing admission/projection, identify the SAME app input's
actual call/callee and missing preparation gate. The SourceObject demand fault
now appends only its original call, callee owner and canonical target; the phase,
acceptance and existing diagnostic tag are unchanged. This is attribution only,
not a semantic receipt or completion proof. Next: focused source-only rejection
checks and fresh unchanged native probe; then map pending cause at the original
input finisher. Separate physical membership work must retain original packet
caller participants and bounded declarations using their own Completion/terminal
result and exact cleanup proofs, never fabricated OrdinaryCallSite or all-module
admission. Both final consumers' positives are still owed; code is uncommitted.


V9 attribution-only CLI build154.17s PASS; binary SHA
`37c30bf7060124f337cb5040aed088f67e4839b8392be69eca75b65dedb38b55`.
Unchanged app emit exit1 at0.04903s, no EXE and same source-only tag; exact source
call owner34 Body(1)/Initializer(0), callee33 `HakoAllocHeap.allocate/1` now appears
in the original demand fault. This confirms the prior target hypothesis only,
not which Pending gate or an input/result cycle. Logs: `cli-v9-*`, `app-v9.*`.

Attribution follow-up: factor the existing typed Object input Completion gate
once inside its current finisher. Preparation still returns Pending/Refused/Ready
with identical acceptance; the existing SourceObject entry demand can borrow its
named pending cause (missing Completion/root flow/exits/exit row/terminal). No
stored diagnostic/semantic receipt or new proof census. If that gate is Ready,
say only that the input cohort remains uncompleted, not a guessed later reason.
Extend the existing missing-Completion regression to check original owner/site
attribution while preserving SourceObject and refusal. Focused V10 build started;
no acceptance/commit or membership change yet.

V10 build72.26s FAIL at inaccessible phase type; V11 build22.79s FAIL at
finisher diagnostic method visibility. Both Cargo/rustc ended before corrections.
Keep phase internals private: actuals owner lends only a source-only predicate;
finisher lends diagnostic text to its lexical parent subtree. V12 awaiting build.

V12 test build294.08s PASS; input10/entry65/packet11 PASS. Identity8PASS/1FAIL:
unqualified_object_sibling_forward_retains_only_original_source_input expects
SourceObject after the generic input finisher. This is a changed-scope expectation,
NOT a pre-S0 baseline failure: SHA-verified pre-S0 V7 binary
`d9aa05dd43d6f57f5f0a1d834932f5604e7e88ac558cb75bd59f0fb44bd68684`
passes that exact test; logs `identity-v12-baseline.*`. Its original relay satisfies
the accepted generic guarded profile, as the earlier scoped-Static fixture does.
Keep the source unchanged and all seven original snapshot/phase mutation refusals:
assert exact finished ExecutableGuardedObject, then call the SAME original source
producer with retained original candidates to verify the earlier SourceObject view
separately. Source-only remains non-executable and borrows no sibling result loan.
This test correction awaits V13; it does not widen the selected admission.


User priority handoff: measure/install shared compiler cache before continuing S0.
CLI V12 build152.30s PASS; binary SHA
`d4b5d8e86527e7ca9a2e5df2c8903fa7d1d814e2e3e9c94fb6e578eb54946d94`.
Unchanged app emit exit1 at0.05141s, same owner34 Body(1)/Initializer(0)->callee33;
new original gate diagnostic is **completion-ready; input cohort still uncompleted**.
No missing Completion/root-flow/terminal gate is demonstrated; inspect downstream
signature/whole-incoming actual reconstruction at this SAME input finisher next.
Logs `cli-v12-*`, `app-v12.*`; no EXE. Identity correction was then UNBUILT;
the subsequent build-time card records95 focused PASS including that correction
and all seven corruption refusals. Physical membership positive still rejects
Maker.qualified/0; both final publication positives remain owed. S0 remains unaccepted
and all semantic WIP remains uncommitted. No Cargo/rustc live at priority switch.
Selected handoff: MIRBUILDER-HOST-SCCACHE-I0 in existing build/link observation card;
then restore Required Object Forward S0 with these unfinished conditions intact.
