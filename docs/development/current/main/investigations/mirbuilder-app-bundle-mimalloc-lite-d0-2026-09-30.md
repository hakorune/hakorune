# MirBuilder app bundle — mimalloc-lite completion D0

Status: active; Required Object Forward S0 unaccepted; cumulative source WIP unlanded.
Date: 2026-10-16
Scope: the existing mimalloc-lite source-to-final-consumer series; no new lane.
Related: [CURRENT_STATE](../CURRENT_STATE.toml), [RULES](../../../RULES.md),
[final pipeline](../design/mirbuilder-final-pipeline-ssot.md),
[workstream](../workstreams/mirbuilder-inplace-replacement-current.md).
Parent: `mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md`.

## History and acceptance boundary

The complete previous card, including V158–V758 preparation notes, is preserved
at **e7271fae86** (this path). That docs-only checkpoint preserves the source
history and evidence; it does not land or accept the cumulative source WIP.
Earlier accepted claim/catalog/Static/Loop premises and retirement receipts stay
at their referenced commits in that checkpoint. Do not re-task superseded stops.
Compression creates no new card, archive, proof ledger or completion claim.

The selected row remains `MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-S0`,
`work_mode=fast`. First milestone: unchanged mimalloc canonical direct EXE,
execution, expected output and exit: **PASS on pinned WIP at V798/app148**.
This is not a source landing or whole-row acceptance. Full `MIRBUILDER-FINAL-PIPELINE-v1` remains
separate and requires its fixed suite, integration/cutover/retirement, R6/R7,
Loop, authority, parser and nondelegating selfcompile conditions.

## Current Decision

- Source authority: original imported/resolved source, full original incoming
  and parameter catalog, canonical result/constructor/field facts, Completion,
  Home/Normal/Fault obligations and original affine Call/field packets.
- Sole path: source → Facts/Recipe/Verify → existing physical writer → original
  LocalSSA relations and FinishedBindings → both independent final consumers.
- Non-authority: MIR type/Box name, role/layout alone, opcode counts, nonnullness,
  copied witnesses, an emptied lowering pool or a convenient fixture.
- Next acceptance connection: V803 acquired Mixed caller source/Home/actual
  transport is under acceptance. V799 old Nullable writer retirement and
  V800/V801 source/final consumer repair are verified pinned premises below.
  CLI152 unchanged M189 passes original class agreement and stops at Mixed
  arguments-missing. V805 common argument projection has 13 focused passes;
  CLI154 passes the Main argument stop and package issuance; first stop is
  callable-main/lowering print-expression/source-unavailable. V807b source/input
  tests15 PASS. V809 received Print/final connection remains WIP; V810c numeric
  field→New→Birth closure passed; V811 shares original Array reads with New and
  Return/recipes, focused16 PASS. CLI157 passes page_id/block_id and stops at
  requested_size source-missing; non-Me writer attribution is next. Typed Nullable
  Print also lacks complete original input. No M189 EXE/runtime or source landing.
- Current series: received Mixed result → exact returned-New argument and
  child relation → formal support → ABI/JSON/C residence and conditional End.
- Completed if-merge normalization is owned by fb512d0549; preserve its source
  Phi and independent-consumer boundaries while connecting the result transport.
- Nullable keeps its own acquired nullable Home law. Mixed preserves Null,
  Fresh and Forward; removing its Normal obligation is not unconditional ownership.
- Fresh moves only on Normal; Fault retains the caller obligation. Forward
  retains its original support and never acquires or releases an owned Home.
- Current formal support uses the same relation/witness Rc, complete source
  census and original parameter, plus the packet BorrowedRead/entry/Copy loan.
- C connection remains in the existing indexed object-result owner. Preserve
  distinct Mixed conditional obligations and Home-less BorrowedNullable results;
  substitute inner callee ordinal through the actual to the outer formal rather
  than comparing those ordinals. JSON params already exclude the receiver.
- Do not invent Main-local support: actual mimalloc Main/run has no call to
  Heap.reallocResult. Its body-internal replacement=me.realloc is real source.
  Publication alone is not incoming authority. Wider outgoing TypedHome or
  ReceivedNullable support stays refused unless an actual frontier requires it.
- No AST rewrite, .hako workaround, fallback, legacy retry, second Builder or
  competing source/classification registry. Extend the existing result contract.

## Latest unchanged product measurement

**First milestone PASS on pinned cumulative WIP:** unchanged app148 canonical
EXE generation rc0 **575.569618s**, direct execution rc0 **.001782s**, stdout
exactly matches the existing seven-line smoke expectation below. No compat
replay or source rewrite. Source landing/whole-series acceptance remains open.
All30fn/2031blocks/856values passed C classification/flow/emission. V2 structural
validation361.941263s, V4 2.083778s, emit .033693s/library .108356s. These overlap
root/native regressions and are not a performance improvement claim.

```text
target/quick/hakorune --grammar-profile canonical --backend mir \
  --emit-exe <fresh-output> --emit-exe-nyrt <runtime-v757> apps/mimalloc-lite/main.hako
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1
HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none
HAKO_AOT_FFI_LIB=<ffi-v798/libhako_llvmc_ffi.so>
```

| identity | SHA256 / source |
| --- | --- |
| unchanged app | `33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685` |
| CLI147 binary | `0c7bf6084d517c3cc5588b322a87d724bd3e9763613b22aba761c3e77234cb68` |
| FFI798 | `ec0ac2127f1b2d4a8aebef80c92086c362c0b73794b1921ce15071a9d0c4ef4d` |
| runtime757 archive | `6897c4dcf6ee871f152732f12204733947cb07a922228b0c749df6beec3eea7b` |

Source manifest/run: `cli-v147.source.json`, `app-v148.json` and
`app-v148.run.receipt.json` under the evidence
base below. Previous CLI140/141 results are retained below; earlier CLI139 is at f6f942f5d0.
App148 source/C validation overlapped V797 bothopts/native39 tests;
these durations are measured results, not a speed baseline.

The generated EXE SHA256 is
`b2e83cbedc6a162ceba64b89b8a1d3a66046148e31a5c06416da924c03205be1`.
Execution: `NYASH_DISABLE_PLUGINS=1 <fresh app148 EXE>`, clearing inherited
HAKO_/NYASH_ overrides. stdout exact, exit0; stderr only the expected plugin-
disabled warning. Expected source: existing `mimalloc_lite_exe.sh` (unchanged).

```text
mimalloc-lite
small_allocs=9 frees=3 reused=3 peak=6 free=2
medium_allocs=4 frees=1 reused=1 peak=3 free=1
requested_bytes=360
outstanding=9
summary=ok
Result: 0
```

## Verified premises; no automatic product acceptance

- V734: original guarded usize FieldWrite source/pins/store/finishing/dual scans.
- V739: raw FieldIndexSet original source/index/Integer guard and same sole Set
  writer receipt, LocalSSA/finishing and both final scans; independent final
  operand/coordinate mutations. This does not cover PlanSourcePublication Set.
- V742–750: original constructor Unit packet joins publication/ABI; Provided
  formals stay caller-owned until Birth Normal. Fault inventory omits staged
  Provided fields; exclusive Provided provider-tail variant was removed.
- V756: original Nullable/Mixed received Birth correspondence, whole child
  relation census, same packet/result/ordinal and final producer in both opts.
  Validation borrows the current affine owner via CallValidationContext;
  shared group/unwind counts and moved root exit bindings are not lost.
- V757: selected canonical ordinary object-field tuple store/read/release/End
  uses atomic residence storage together. Current C cohort is Null/Fresh only;
  reads reject Forward, never retry legacy slots. Scalar/Array/nonselected
  shared field arms retain their own responsibilities.

## Remaining same-series connections

- [ ] Mixed received Birth actual → ABI/JSON/C. Use
  `ordinary_new_local_commit/finalized_received_birth_actual.rs` and
  `published_backend_view/physical_abi_birth_actuals.rs`; retain original result
  ordinal and exact acquisition/received packet/child Rc/finished producer.
  V781 connects the gate in WIP; complete source-issued C execution is still owed.
- [ ] Forward field support/anchor → paired store/read/conditional child End.
  Reuse `ordinary_new_return_leaf.rs`, `ordinary_new_return_handoff.rs` and
  `ordinary_new_borrowed_formal_entry_incoming.rs`; canonical full support census
  and call-hop composition remain mandatory. Keep anchored-field End gates
  closed until the complete source-to-consumer envelope is checked.
- [ ] Mixed/Borrowed Call JSON/C and complete ObjectEndIfOwned. Existing explicit
  V781 connects Null/Fresh/Forward output, residence, move and complete End in
  WIP; independent source-issued runtime positive/negative evidence is still owed.
  Do not replace original evidence with a wire tag, role or metadata permission.
- [ ] Array residence execution. Rust Push/Get/raw Set connections and selected
  object residence C757 are premises; broader Array C execution is still owed.
- [ ] PlanSourcePublication Set. Retain exact FieldGet source locator before
  Plan descent; connect the same affine read/LocalSSA owner and sole recorded
  Set writer/census at the existing source-aware Loop/Plan emission port.
- [ ] Required field Birth/full support and remaining real FieldWrite pins.
  Reuse accepted bounded V734/V739 connections; do not reopen them as new tasks.
  Retain selected entry/result/receiver/cleanup/publication and all real incoming.
- [x] V753 specific Rust module/published operand deficit: V797 same unchanged
  app bothopts compiled-entry→ABI→backend demand PASS1634.930994s, with both
  whole-instruction scans and Mixed/Borrowed mutations still mandatory. This
  is RED→PASS at that boundary, not a clean-HEAD baseline or full suite claim.
- [x] Pinned CLI147 + FFI798/runtime757: unchanged app148 canonical EXE,
  execution and exact existing stdout/exit0 (V798 first milestone).
- [ ] Complete required fixed acceptance. Re-pin a negative only at its truthful
  rejected boundary; the first app does not replace this suite.
- [ ] Name and retire replaced exclusive edges/adapters after their production
  caller cutover, caller-zero and required evidence in this same series.

## Earlier focused premises (superseded frontier)

Evidence base: `/tmp/mirbuilder-required-forward-resume/`.

| run | result and meaning |
| --- | --- |
| V756 root build | PASS67.775355s; same root test binary reused |
| V756 focused | 11 PASS, including actual source test below; three additional existing field/stored-child source tests PASS |
| actual `owned_call_results_publish_original_child_handle_contracts` | PASS979.80s, both opts, RUST_MIN_STACK=16777216; Nullable/Mixed argument correspondence, ValueId/ordinal drift, original Return/conditional End and final mutations |
| runtime757 build | PASS234.799502s concurrent with actual test; not a speed baseline; residence set/get exports verified |
| C Provided3 + wire/Fault mutations | PASS23.983533s; original before/current/after-store Faults and exact acquisition/release counts |
| C Unit7 + Fault/mutations | PASS41.643294s; constructor stores/effects/cleanup and malformed input refusal |
| fresh C fieldReturn6 | PASS35.538991s, both opts and all selected store Faults |
| fresh C stored-child96 | PASS14.904483s, original OBJ/canonical negatives and both-opts callee Fault cleanup |
| V758 | build PASS65.079179s; focused11 + actual1 PASS; actual723.391039s both opts |

Runtime757 SHA: `6897c4dcf6ee871f152732f12204733947cb07a922228b0c749df6beec3eea7b`.
FFI757 SHA: `943053b6918f2d149dae0acef9ced3359e439159eccf8f2cada21b6d7114367a`.
Detailed receipts: `v756.focused.json`, `v756.extra.focused.json`,
`runtime-v757.receipt.json`, `*-residence*-v757.receipt.json` and source manifests.
Kernel/header files match V627 exactly: reuse its atomic identity/mutation/C-output
and ABI geometry evidence; do not claim unchanged whole compiler sources.

V758 extracts the existing final object Return census for production/test reuse.
Pure final-MIR mutations reuse original retained outcomes; Mixed outer parameter
ValueId drift keeps its separate source-entry check. Original source positives,
complete Return census and independent negative boundaries remain. Cheap exact
actual/source correspondence precedes expensive shared received-call validation;
success still requires all original checks. No cache, skip flag or new test fn.

## Red classification and preservation

- V751 synthetic Maker/Holder expectation: own unaccepted cohort, removed rather
  than widening source admission. Scratch retained for landing disposition.
- V753 default-stack abort: environment condition; 16MiB run exposed the strict
  optfalse deficit above. Never a PASS or proven baseline.
- V755 consumed-packet missing: own finalized consumer used Lowering context;
  fixed by V756 current-owner validation and verified by both-opts actual test.
- Initial old field/stored-child JSON parser failures reproduced with FFI752.
  Informational stale captures: reissued original fixtures via existing V756
  source tests, then fresh C runs passed; no JSON repair or acceptance waiver.
- Historical confirmed ArrayGet/code-test and full borrowed-ingress baseline reds
  remain documented in e7271fae86; no full-suite PASS is claimed here.

Preserve all cumulative WIP, including inherited unrelated ownership.md changes.
No source landing/S0 acceptance is implied by this card or docs commits.
At landing, retain needed commands/results in the implementation commit and
compress completed history to that commit, not /tmp+SHA alone. Review temporary
receipts/build outputs, `v751-unlanded-received-birth-source.rs` and adapters:
retain only with a named owner/reason/retire_when; retire exclusive predecessors
once replacement/caller-zero/gates hold. Shared owners, unselected callers and
independent safety verification remain. Parked lanes keep CURRENT_STATE conditions.
Stored-child sibling and Gates2–4 stay parked. Gate1 reallocResult315/318 retains
`mirbuilder-result-new-site-provenance-d0-2026-10-06.md`; opaque followups retain
`mirbuilder-app-mimalloc-opaque-formal-ingress-d0-2026-10-01.md` boundaries.

## Retained construction checkpoint — V759–V789

Full implementation mapping, commands, pins, failed attempts and focused
results are preserved at **122a2824bf** (this path); e7271fae86 owns earlier
history. These are docs-only checkpoints, not source landing or acceptance.
The following unfinished contracts and evidence remain live:

- V759–V781 extends original outgoing Return field support, ordered witness
  groups, ABI/JSON/C substitution, borrowed0/2 and Mixed conditional complete
  End. Same relation/witness Rc, full original terminal census and final scans
  remain mandatory. Retain the current affine owner; publication is not incoming
  authority. Mixed/Borrowed source-issued runtime acceptance is still owed.
- V765/V768/V769 original app/both-opts source correspondence passed; latest
  V769 actual1074.29s. These are preparation evidence, not canonical EXE success.
- V782 JSON32/35 leaves three own cumulative-WIP reds: nullable_receiver_call_
  serializes_nullable_handle_and_checked_release (borrowed-declaration/missing-
  retained-call); ordinary_instance_function_publishes_canonical_receiver_object
  (ordinary-field-read/root-exit-phase); owned_array_fields_release_in_reverse_
  order_before_home_release (construction Fault expects2 releases, sees reclaim
  plus return_fault). No whole-module PASS or exact clean-HEAD baseline claim.
- V775–777 unaccepted tiny Borrowed fixture was physically removed, since its
  scalar Root Return fails ReturnValueNotCovered outside this app entry. Do not
  invent another Root fixture/lane. Failed scratch files under the evidence
  base, v775-unaccepted-borrowed-microfixture.rs and its -c-mode.py companion,
  stay under this series: retain for already-owed Root/fixed-suite analysis;
  retire_when=that boundary is reconciled or its exact input is retained by its
  real owner. They never authorize source acceptance. Existing actual8/app
  remain the sole original-source authority for this connection.
- V783–785 normalizes the existing checked FieldArrayRead source away from a
  Return-only wrapper and connects same Get/read Rc → Local/Copy → Birth actual
  through sole checked writer, LocalSSA/FinishedBindings and both final scans.
  V785 original app/both-opts plus four Get/local/field/Birth drift negatives
  PASS1156.055888s. No Integer type stamp at ABI or blanket dynamic permission.
- V786–787 joins retained original handoff ingress (generic lost-handoff still
  refuses), common C provider/read-use suffix, Array Get/Set/Append consumers
  and shared nonnative/native schema. Get2/Set5, receiver/shape/Fault and
  owned16 peer regressions passed on FFI787. Current ownership/source permits
  remain distinct from wire metadata. Source `claimed_loop_push_rejects_on_
  unretained_plain_lane` is RED at field-array-append/loop-source-missing:
  own cumulative-WIP source-lender deficit, never a clean-HEAD baseline claim.
- V786 extended app test demanded full backend gate and was RED714.270958s
  optfalse at Birth numeric entry-unclosed. V789 fixes the observed producer
  using original compiled Birth formals/physical bindings and exact USize range
  contract, keeping guarded-borrowed and bare stores distinct. V789 focused4
  PASS, and CLI143 crosses that stop. This does not yet establish a new PASS
  for the full expanded app test; V753 strict-consumer debt also stays open.
- CLI140/141/142/143 canonical unchanged app rc1/no EXE: scalar-kind → retained
  ingress → numeric Birth → declared parameter. Latest CLI143 build143.011864s,
  app169.026962s, first-stop HakoAllocHeap.realloc/2 physicalslot1/logical0.
  Original source parameter ValueId, physical slot and JSON ordinal must never
  be conflated; source declaration corroboration follows below.

## Retained V790–V798 contracts and acceptance

Full observations, Decisions, commands, exact pins/timings, harness corrections
and named negatives are preserved at **8508d9d37d** (this same path).
That commit lands only common Array declaration emission and its README;
it does not land or accept the cumulative Rust/runtime/C source series.
CLI140–147 failure histories are diagnostic premises, superseded as product
frontier by app148 above. Never count repaired JSON, a stale fixture or an
interrupted/zero-filter run as source or product acceptance.

Live binding contracts retained from that investigation:

- V790/791: original per-function declared header/catalog and source borrower
  remain class/entry authority. Header normalization lends the same original
  declaration across both consumers, without broadening a source-less module.
- V792: indexed flow workspace uses checked actual function block counts,
  one enqueue and exact state/Fault joins. uint64 state domains separate LIVE,
  Birth/owned child marks, Mixed conditional state, Map transfer and String rank.
  Native bounded state and unrelated caps remain. Dynamic field suffix walks
  keep both original-prefix and full loop visits. No source registry/cache.
- V793: typed child FieldGets share nonowning field-loan identity in all ordinary
  result roles; only the original receiver is call-live. A loan joins unique
  provider New, canonical field path, exact read/use and surviving parent/Birth
  suffix. It never grants Home, transfer, owned Return, End or new argument lane.
- V794: original Copy plus acyclic unanimous Phi shares one pervalue alias root,
  separate from kind union. Exact complete predecessor census and same original
  producer required; differing/cyclic/missing roots refuse. HANDLE Phi still
  checks current object/Array liveness, including unused Phi after End. Scalar
  I64/Bool behavior unchanged, other carrier Phi closed. Old Copy walker retired.
- V795: a completed destination-less Fresh Return owes fault-only complete End
  if a prior Normal cleanup faults. Reuse original Construction/NewResultCommit,
  completed Birth and source destruction/children; prepend generic fault-only
  prefix to existing RootHome graph. Normal/acquisition Fault exclude it, child
  End precedes outer End, later Fault continues residuals. No fake Home/Call
  source, double-End of Provided arguments or construction reclaim substitution.
  RetainedUnavailable remains diagnostic-only, no End/artifact permit.
- V796: borrowed tagged formal membership uses existing ordinary-role predicate,
  preserving static profile, exact original param/object_view and lifetime gates.
- V797: DeclaredFormal is class authority, not constant tag3. Same caller original
  root entry/value/installed BorrowedTagged carrier joins before tagged transport;
  preserve incoming tag including null. Selected missing join refuses, never
  retries fixed3. Other source actual kinds retain their own transport. A function
  entry does not imply every root is selected. Current app changed exactly six
  kind fields (realloc4/reallocResult2); values/CFG/cleanup unchanged.
- V798: common plain Array declarations precede native/indexed split once. Other
  declaration owners and all source/type/flow admission stay unchanged. Duplicate
  branch declarations removed, net5 emitter lines deleted; landed8508d9d37d.

Verified evidence at that checkpoint (not a whole-suite claim):

| selected evidence | measured result |
| --- | --- |
| original source header normalization, stored-child96 + G0 | V791 PASS20.79s |
| owned16 / fieldReturn6 / stored96 / String / ownedArray / Map normal+Fault+wire refusals | V792–794 PASS; Map includes >256blocks; exact cases/timing at8508d9d37d |
| same-root Copy/Phi, differing-root/unused-after-End refusals | V794 owned131.009003s / stored14.825800s PASS |
| Direct cleanup3/order5/graph5, Fresh original+joint final mutation | V795 PASS; both original/FinishedBindings contracts remain |
| instance8, source move/Fault inventory, nullable Birth JSON, joint consumer mutation | V795c PASS (.10/.33/.08/.81s) |
| Bool discard true/false original source→C execution and callee Fault | V796 PASS12.335907s after original fixture reissue |
| declared/direct-copy×null/object×bothopts, carrier/producer mutations | V797 source .91s; original forwarded/receiver .12s PASS |
| declared source C20 / class+wire+release refusals | V797 PASS196.347264s |
| physical JSON35 | V797 32PASS/3RED; repair mapping below, no waiver |
| original unchanged app compiled-entry→ABI→backend, both final scans and Mixed/Borrowed mutations | V797 PASS1634.930994s bothopts |
| native39 production host + independent physical/runtime/Fault/malformed | V798 PASS519.883085s; host archive9f6…, Python observer757 (separate pins) |
| isolated HEAD+declaration-only C build/native source EXE/exit30 | V798 PASS; native compile103.006463s; no other source WIP |
| unchanged mimalloc canonical EXE/run/stdout/exit0 | V798 app148 PASS; full command/pins/output above |

V753 specific Rust optfalse forbidden-operand RED is now PASS at the SAME
restored whole compiled-entry/ABI demand. Both module-all-instruction and
published operand scans and exact coverage remain mandatory, as do Mixed8rows/
7physical mutations and Borrowed8rows/5mutations. No clean-HEAD baseline or
separate negative-suite rerun is claimed. Other source debts below remain.

Required same-series regression mappings (read-only reviewed):

1. Self Nullable literal: original observation plus exact local-flow selects the
   existing full object packet; original whole input/result/Completion and common
   final Call visitor remain mandatory. Keep the positive, no fake empty
   declaration. Old dedicated writer has one production fallback caller; after
   selected switch/acceptance, remove both plus exclusive nullable unwind wrapper.
   Nullable already triggers Home walk. Handle observation alone never widens it.
2. Pair.sum child I64Add: normalize owner+exact exit relation/prepare and raw
   reservation/completion census. Reuse sole selected writer. Extend existing
   I64Field FinishedBindings protocol to original two reads+Add+same Return and
   both child final consumers; removing root guard alone leaves a verification gap.
3. Array Birth Fault: partial source-owned child discharge is inside Birth,
   newest committed field first. Caller reclaim is storage-only. Preserve Normal
   reverse End and exitFault assertions, then inspect same issued Birth per-store
   Fault paths rather than requiring both children before either store commits.
   Existing source inventory/Provided exclusion tests retain independent laws.

Source, selected temporary adapters and /tmp evidence stay retained pending full
series acceptance and landing disposition. app148 first EXE/input/run receipts
are preserved until that decision. Native override test distinguishes host9f6
from observer757; product app pins757 only. Source WIP remains unaccepted.

## Current acceptance connection

V799 construction begun: existing observation owner joins exact self Nullable
local flow/site/callee/destination/argument ordinal+source sites/canonical class
to mandatory full-packet selection, before checking input availability. Missing
input remains selected refusal; Handle/Borrowed/Mixed/scalar selection unchanged.
Existing full packet checker retains argument values/domain, callee input/
Completion/teardown and final group authority. Old dedicated writer is retained
only until this switch's focused acceptance, then delete its sole caller and
exclusive nullable unwind wrapper in this same slice. No extra fixture/test.
V753 boundary review confirms current same named app test restored full demand
and both module/published scans with exact coverage; old optfalse red is now
resolved at this boundary (not baseline, no separate C implication).

V799 build PASS58.375619s. First focused run exposes own selector-join drift:
nullable JSON positive RED, instance4PASS/4RED (allsame new source-drift token).
Old writer retained; no retirement/acceptance yet. A guessed lexical_object_slot
filter matched0 and stopped the helper: invalid, not PASS. Installed result-class
ordering is already correct; do not remove authority based on that hypothesis.
Add failure-only source diagnostics to identify the exact conflicting original
component before repairing its existing mapping. Both independent workers
confirmed selector law; this is an implementation join defect, not new design.
Completed V790–798 history compressed to8508d9d37d with live laws/debts retained.

V799a diagnostic build PASS52.515786s, exact source reproduction RED .02s:
callee/destination/class/ordinal/sites all match; receiver Home flow_arity0.
Original home_receiver_local_call issuer intentionally emits empty arguments;
it owns Home/cleanup, not actual authority. Remove only false Home-arity
comparison, preserve original receiver argument/site and full packet checks.
This is a confirmed join correction, no class gate removal or refusal waiver.
Proper object-slot test module is lexical_instance_call::object_slot::tests.

V799b build PASS38.648260s. Original nullable JSON positive PASS .05s;
receiver observation PASS .02s; objectslot11/12, instance5/8. Remaining
SourcePending raw fixtures use init{v}, unlike JSON's declared v:i64. Worker
confirmed independent canonical object/destruction issuers require declared
field type (Unavailable(FieldType)); brand source tests already pin this.
Raw/Omitted is not a type/End permit. Preserve both original untyped fixtures
as source-unavailable variants and same typed/untyped caller argument positives
with declared-field variants in one existing test (retire one duplicated fn).
Keep original packet/source pending after input removal in the old legacy-
preservation test; never restore its obsolete Ready/no-ingress assumption.
Duplicate-Birth-store negative now declares its unrelated field types to isolate
the existing repeated-store refusal, not hide an untyped field failure; original
untyped fixtures remain above and independent brand laws still required. No
.hako application change, inferred layout or expanded untyped admission.

V799c pre-retirement build PASS66.338338s; nullable JSON1 (.06s), instance7
(.11s, including both original unavailable and declared-field caller variants),
objectslot12 (.09s), unavailable-descriptor source1 (.01s), receiver observation1
(.02s) PASS. Pointer/diff and route/layer/thin-shim guards PASS. This establishes
selected switch evidence; whole source series remains unaccepted.
V799d removes the sole old receiver-nullable fallback and its dedicated writer
plus exclusive prior-home unwind helper. Common packet emission and its original
Call/binding-group/conditional End own the selected edge. Missing selected packet
fails receiver-object/packet-unavailable, never generic retry. Shared lexical
nullable begin/record and nonselected callers stay. Original names' source census
is zero after deletion; post-deletion focused validation pending.
V799d post-retirement build PASS58.763186s; same focused1+7+12+1+1 all PASS.
Physical JSON parent-module filter is33 tests (31PASS/2RED), not35: nested layout
submodule is outside this exact filter. Remaining named reds are Pair.sum child
I64Add root-exit-phase and owned Array caller Birth-fault obsolete expectation;
nullable positive now passes after retirement. Route/layer/thin-shim/diff PASS.
Read-only worker confirms deleted writer/helper have no other consumers and
common packet retains the same source/prior-Homes/conditional End plus final
bindings. No whole-series/source landing claim. /tmp V799 manifests retained
until cumulative acceptance; sole app milestone remains app148 pinned evidence.

V800 same-series remaining Pair.sum mapping: original owner+explicit exit replaces
root-only preparation; retain exact read reservation. Existing terminal-result
owner contributes read/Add originals to FinishedBindings, checked in both draft
and final root/child consumers. Existing root-exit receipt remains sole exact
Return authority; no additional writer, registry or field-read permission.
V800 build PASS69.892118s; child field final-mutations2 PASS; original Pair JSON
still RED at same root-exit-phase, terminal source22 18PASS/4RED (root instance
Pair variants). These source tests expose missing original Me classification:
terminal return_scalar and field_home only consult variable_ref(Local), so Me
Add has no relation to prepare. Read-only worker confirms upstream gap. Repair
same terminal field consult with original body-shape Me/Lexical Receiver binding,
exact owner/declaration/current live rooted-handle state, then SAME canonical
integer-field lender. Do not broaden generic observe/argument/ownership classes.
V800a build PASS59.584042s, same red, field final mutations2 PASS. V800b diagnostic
build PASS52.758064s. Worker traced exact upstream loss: source issuer merges
I64Add's reads, then retain_child_terminal_relation drops Add from child index.
Original Me already has variable_ref(Local), so the proposed Me helper was
unnecessary and physically removed. Retain original I64Add beside I64Field;
keep original receiver/classifier unchanged. Existing declaration-order source
case now also covers both no-map orders. Extend existing child-final mutation
case with unchanged Pair app: original reads/Add/Return and missing/swapped/
duplicate Add or redirected/masked Return must fail. No new test function.
V800c build PASS62.668637s; source22 18PASS/4RED old root-instance-row
expectations still under review. Existing child-final mutation2 PASS. This
includes unchanged Pair source plus five Add/Return and six field mutation
boundaries. Pair JSON now reaches publication and stops only at old test's
FieldGet lookup; sole writer emits canonical ObjectFieldGet. Update existing
positive to read that original canonical field, retain exact JSON/layout/object
identity assertions. No replacement generic FieldGet or runtime field inference.
Worker reviewed four remaining terminal source expectations: original source
scalar now selects the lexical packet even at zero arguments; old root row is
intentionally absent. Keep the exact four sources, update to source-scalar/
lexical owner and SAME result loan+real cohort corroboration. Two-Pair initializer
identity remains checked from original binding/initializer and pending claim.
Unannotated Integer is source-proved; missing annotation is no longer missing
semantic contract. Existing real Missing/foreign/uncorroborated result negatives
remain required; no positive converted into blanket refusal.

V801 Array JSON assertion repair is test-only: preserve caller Normal2/1/Home
and releaseFault1/Home; caller BirthFault reclaims only outer storage. Inspect
same issued call.target Birth and each original allocation/store Fault: field0
store/first arrayNew/field1 store owe none, second arrayNew/field2 store owe
committed field1 only. No candidate field2 may End before successful store.
Worker independently checked source-issued owned-slot JSON with same pattern.
V801 build RED80.738702s: test-only missing OwnedExprSite import and private
corroboration helper call (worker API suggestion was outside its module scope).
Correct import; use existing affine Take's source/actual corroboration and same
loan assertion, keep real contract negative tests at their existing owner. Do
not widen production helper visibility for this test. No product run claimed.
V801a build PASS68.006218s; JSON33 (.69s), terminal source22 (.13s), child-final2
(.28s), uncorroborated-result1 and constructor prior-commit/Provided exclusions
1+1 (.01s each) PASS. One independently requested final-result gate RED before
its assertion: original Unit(q:i64) discard has no zeroarg UnitDiscardSource and
now correctly refuses unit-value-use at issuance. Worker confirms existing
zeroformals/zeroargs source profile, not a new permission to widen. Keep exact
original source as precise early-Unit refusal, then same Float declaration/call
without that earlier invalid-profile call reaches late result mismatch; retain
original real Missing/foreign nested-source mutation checks unchanged. No
catch-any error, fixture allowlist or product claim for arity1 Unit discard.

V801b build PASS53.558616s. Final-result refusal1 (.02s), physical JSON33 (.68s),
terminal source22 (.13s), child-final2 (.27s) and uncorroborated-result1 (.01s)
PASS. Constructor partial-commit and Provided source negatives passed V801a;
unchanged by the test-only final refusal repair. Pointer/diff and route/layer/
thin-shim guards PASS. No unclassified red remains in these selected filters.
Root binary e278670849caf97e43d0bb971bcc01ae4683630741b0aa0fa340d844d324202c pins source manifest v801b.source.json;
all 9199 tests are not claimed green. V799 sole old writer/helper remain absent.
Now running original unchanged app compiled-entry/ABI/backend bothopts+mutations
on that same binary; CLI149 rebuild then canonical fresh EXE/run are pending.
These will re-pin final related source, not replace required fixed acceptance
or imply cumulative source landing. Active source/test evidence retained until
series acceptance; no source adapter was added by V799–801.

Runtime reconciliation (read-only): V797 connects original Mixed/Borrowed source
through final Rust consumers; app148 validates ALL30 C bodies and links, but its
workload does not execute realloc/reallocResult/resizeInPlace. native39 and
owned16 retain their distinct Array/owned Null-Fresh coverage. Remaining boxes
therefore mean connected, REACHED runtime/outcome/Fault/independent C mutation
acceptance owed, not missing JSON transport. Do not infer it from app stdout.
Existing unchanged M189 mimalloc-object-return-api-proof supplies real incoming
realloc0/1/2 and resize0/2; M190 mimalloc-result-contract-proof supplies wrapper
Forward-field support and later reads/outer End. Reuse their existing guard
output oracles, but SAME canonical direct emit-exe route/pins as app148; old
VM/emit-MIR helper routes are not canonical evidence. First-stop measurement
next; no app rewrite, new fixture, fake Main support or optional tiny-Root lane.
Existing C call-result observer is the future runtime/negative owner; do not
count its current owned16 as Mixed coverage. Actual first app validation runs
continue on pinned source; source acceptance remains open.
CLI149 build PASS165.413057s, SHA eed8da06fbd8ccf4e0bbcf24d25a8a1435491a4e1ce8bddc146a5c81a2b6e109;
its7463 source hashes equal root v801b manifest exactly. Existing
integration/apps/typed_object_method_min_exe.sh PASS with that CLI, FFI798,
runtime757, RUN_TIMEOUT_SECS900: selected V4/LLVM-CAPI/EXE and expected exit30.
Its command elapsed was not separately measured; no timing or full-suite claim.

Pinned M189 first measurement RED .050279s: original Main Body2/Initializer0
(local same=heap.realloc(h0,16)), borrowed-nullable/caller-key, no artifact.
Pinned M190 RED .166698s: Main Body1/Initializer0 (local alloc=heap.allocateResult(8)),
borrowed-result/source-not-i64, no artifact. Source SHA respectively
705a535cab75360c59c8e69e666b4c0560913453539d30d3dc5bf4f635a6c7bb and
598e0edb2fc0a60214f3f11dd91654517138ee34083528afbf35addd3cbfa9e4.
Same direct canonical emit-exe argv/env/runtime757/FFI798; receipts m189-v149 /
m190-v149 under evidence base. Independent workers now trace original Main
caller/key ingress and owned wrapper result selection to final consumers;
actual Main sources above, never app148's nonexistent Main realloc incoming,
select this required runtime acceptance. No new key, fallback or hako rewrite.
App149 compile and v801b whole-source bothopts verification still running;
no code modified since those pinned source snapshots.
App149 PASS: canonical direct compile rc0 539.602955s, run exit0 .001702s,
raw stdout exactly same7 lines as app148. EXE SHA
c05043feb597733618b2e135811c030816f49d3d5d54dcaa613b9a0f1c07b340.
Original C input unchanged SHA5408d9a3c2cc87e086c4655a788d5722eb684bca98cf77a2041f595adc44d3ca;
all30fn/2031blocks/856values pass C. V2 334.163450s/V4 2.091475s/emit.033618s/
library.103499s, overlapping whole-source test; no speedup claim. CLI/runtime/
FFI/app source pins above. No binary or source modified during this run.

M189 full-boundary audit: original_local_call_source rejects AppMain despite
existing BorrowedAppMainSourceLoan; normalize to same original caller, never a
fabricated key. Beyond that first stop, MixedLocal source formal_actual,
Home installer is_self_rooted_handle and outgoing witness composition are
formal-only. Its caller anchor must be separated from unchanged callee roots/
fixedordinal/Fresh class+complete End. Reuse original received_call_acquisition,
nullable/Mixed packet/source/teardown and current Home state; class alone is
not liveness. Chained same→moved needs original ReceivedMixed actual carried
through allactual classifier/exact read+Copy/ABI/JSON. C ordinary actual and
field-anchor recursion must lend existing Mixed260 readability, preserving
conditional ownership, not numeric3 or unconditional LIVE. Keep outgoing
Return/constructor field escapes formal-only until separately authorized;
local borrowed anchor never becomes a dying-local Return permission.
M190 first ownedWrapper result likewise needs complete original incoming/result
packet, not an all_formals_exact_i64 exception or source-not-i64→None. Workers
are resolving existing common lenders and delete-set before construction.

V802 detailed Main-source construction/build/debug history is preserved at
**48e2ad16d2** (this path). Retain its contract: existing Main initial/retained
loan supplies unique declaration/owner/slot/key/Static contract/ordered binding;
AppMain is selected only for selected=None, not a failed Cataloged/nonCataloged
lookup. No persistent Main registry, synthetic key/call or Home permission.
Main input5 and terminal7 PASS; original identity/wrong slot/mode/foreign source
checks stay required. CLI150 advanced unchanged M189 caller-key→formal-root,
without EXE/runtime acceptance. V801b/app149 remain pre-V802 pinned premises:
whole source compiled-entry/ABI/backend and both independent final scans with
existing Mixed/Borrowed mutations PASS bothopts1532.750634s; no newer full claim.
Completed Static scalar families require original executable receipt presence
before whole-cohort demand; once present, all incoming/veto/signature checks stay.
No NoIncoming catch, empty executable permission or fake Main call. Original
foreign-constructor/changed-ordinal negatives use the existing source fixture
seam (relation8, foreign rejection0, ordinal rejection7); exclusive full-package
rebuild helper was removed. They do not replace the full page_heap assertion.
Full page_heap package prefix6/2 remains RED after importing unchanged LayoutBox
and transitive SizeClassBox: free_top field closure stop is gone; unused
Layout.normalize_size source now refuses required-integer-source-unproved. Its
8relation/prefix6/2 assertions remain unwaived; see later V806 current own reds.
Acquired support keeps its original local-call initializer and current Home/
observation/covered Call/priorHomes, while Formal keeps substitution/all roots.
No Formal-failure retry as Acquired; local support never grants outgoing Return/
constructor-field escape. Conditional Home, CallFault and ObjectEndIfOwned stay
sole runtime owners. V803 below carries this same required runtime connection.

V803 source/ordered Home construction starts inside the same required runtime
series, after read-only caller-anchor review. Mixed source loan separates
callee original roots/fixedordinal/Fresh object from private Formal/Acquired
anchor. Original Parameter/Local kind selects one path; Formal failure never
retries Local. Acquired retains sole original initializer source and its root
Rc census, no parent Mixed loan or Completion cycle. Formal-only outgoing
Birth field support explicitly rejects Acquired. Existing formal composition,
foreign snapshot and ordinal drift checks remain. Original M189 same/moved/fail
source cases reuse unchanged app +page_heap and existing source fixture helper;
all three prove exact prior acquisition and no caller-formal witness, with
same-spelling foreign facts rejected. No new fixture or test function.
Home install joins original call/declaration/binding, Nullable/Mixed role,
covered acquisition/current priorHomes and SAME parent Mixed loan. Support
consumption/rebinding invalidates source availability. Subsequent Mixed actual
observer/packet/ABI/C joins are still owed; is_received_mixed_v1 is retained
only for that next same-series connection, not current execution permission.
V803a build PASS100.008268s; V803b PASS61.590709s. Existing full-facade source
test (now with original M189 acquisition cases)1PASS; changed ordinal1PASS;
borrowed nullable2PASS. Wrong home_prefix_received_call::tests filter was zero,
not evidence; exact existing received_call_tests:: run next. Full page_heap
source/unused static input RED above remains unresolved, not a baseline waiver.
V803b received_call_tests5PASS (correct filter); pointer/diff checks remain required.
V803c connects ReceivedMixed as its own passive/actual variant through current
Home-chain observation, SAME ingress Mixed source loan, shared original
Completion acquisition lender, source domain/entry/read/Copy and JSON
nullable_typed_object transport. This is not ReceivedNullable/TypedHome or an
ordinary LIVE permission. Build PASS73.080896s; focused actual26/object argument4/
received identity5/original M189+formal source1 PASS. Parent packet/full result/
completeEnd remain required final gates, not replaced by the class lender.
Borrowed actual owner795 lines: considered split at760; no further growth past800.
C803 uses the same mixed-result reader with a decreasing Mixed↔anchor budget;
ordinary borrowed actual checks exact conditional state/Normal/fixedordinal,
Fresh domain and recursive Forward anchor domain. Tagged uses original object
formal, HANDLE uses original provider/class plus readability, exact null ends
without a class. Neither edge consumes support or changes260 toLIVE.
FFI803 build PASS6.041393s. Runtime Mixed execution/independent mutations still
owed; existing owned16 regression and fresh CLI151/M189 measurement next.

CLI151 build PASS202.925506s, SHA76213100182748b899aa43f040eb596b03629bf814c17688f2fb691b57e86dd3.
C803 owned16 +original Normal/new/Home Faults and mutations PASS206.985156s,
FFI SHA2bb0b89dd59b250f23a02205f93062ffd1ed8d3be0c0eb0a9b879a5001e313cb,
runtime757 unchanged. These overlap CLI build; no speed claim/Mixed coverage.
Read-only C review additionally requires existing canonical group.field child
class at field-anchor recursion. C803b passes that SAME child_object_id rather
than classless anchor permission; build PASS6.042351s, SHA
027fa1a5e249bf6e58f0709a1082ee263a8f27851136425ab9e5685c4efd2a4b.
CLI151 Rust source is pinned by cli-v151.source.json; external C803b separately
by ffi-v803b.source.json. No claim that the earlier CLI manifest includes the
later C-only tightening. C field-return6 regression running on C803b.
Unchanged M189 canonical direct EXE RED .170363s on that pair/runtime757,
no artifact: stop advances from Main formal-root to owner38 Body3Initializer0
borrowed-view/declared-object-class. Parent source-class agreement is the next
same-series join to inspect; do not relax DeclaredObject to accept a different
class or fall back to untyped. Receipt m189-v151.json keeps original sourceSHA.

C803b field-return6 execution and existing mutations PASS32.747569s with runtime757.
The current Main/Local source, domain and C transport is newer unaccepted WIP;
none of these regressions establishes reached Mixed outcome/End runtime evidence.
Current original class agreement must be normalized at original_formal_seeds →
selected_views before transport pruning, not repaired from the later ABI/source
actual variant. Worker traces the original incoming/lender correspondence.

Next same-series source normalization (worker confirmed): early original seed
classifier only recognizes received NullableObject through legacy projection.
Mixed same/moved becomes Conflict before later actual transport; original
inventory already retains its exact MixedLocal loan. Extend the SAME original
seed issuance with sole initializer/binding/call + unique retained loan and its
original support seed. Fold Fresh class AND Forward support through existing
whole-domain agreement; exact Null contributes no class. Fresh-only Class seed
would be unsound. Share SourceInteger/SourceI64/SelectedTransport projection;
retain original loan/root Rc and reject cycles/missing/foreign/mismatch. No new
class registry/solver, late view overwrite or declared-contract relaxation.
This normalization is not yet implemented. Source runtime/field End acceptance
and the separate unused Layout static-input/prefix6/2 RED remain owed.
Docs-only checkpoint 9c1e7b34f8 pushed; cumulative implementation remains WIP.

V804–V806 detailed measurements and intermediate build/test repair history are
preserved at **378ddaafcd** (this path). This compresses preparation history,
not source acceptance; cumulative implementation remains WIP. Retained contracts:

- V804 original Mixed seed retains exact loan and recursively classified Forward
  anchor. Fresh class AND anchor constraints enter the SAME whole-domain fold
  for SourceInteger/SourceI64/SelectedTransport. Null contributes no class.
  Soleinitializer/binding/call/argordinal/frame/rootsRc and direct/Upvar-rebind
  checks stay; cycles, Conflict and foreign/scalar anchors refuse. No late
  class overwrite, separate solver or Fresh-only permission. Existing extended
  conjunction/cycle/declared-negative and original M189 source tests passed.
- CLI152 class stop advanced to Main43 Body2Initializer0 mixed/arguments-missing.
  V805 unified Nullable/Mixed consumers on original-target argument projection;
  SourceObject/SourceInstance/Executable phases retain source/snapshot checks,
  SourceOnly vs Observed and exact Err/Unavailable refusal. No retry or new
  execution permission. Focused13 passed; CLI153 did not advance that stop.
- CLI153 worker GDB pinned missing Main pending row and matching transport
  incoming; an earlier SourceInstance Mixed query succeeded. Its source hashes,
  binary/FFI/runtime/argv, offsets and no-inferior-call receipt are in that commit.
  Original Observe ran, but Instance array/zero/producer/formal-draft selection
  omitted Main's affirmative Mixed request. V807 below addresses that mapping.
- V806 imported-source regression repair remains unaccepted. Same original
  omitted Static declaration/parameter identity is checked, no checked-input
  marker is invented, original CurrentOwner calls stay. Loop raw loans have
  moved into the preissued V2 product; the test checks SAME rootRc/placement,
  one product take, missing raw rows, swapped CallSlots and outside-loop refusal.
  Canonical Static source cohort is compared with complete final transport,
  including SAME Rc/site/callee/ordered arguments. StaticOne input lender is
  checked before/after missing-actual mutation, plus selected packet refusal.
- Latest V806f build passed52.819211s, test RED4.67s at old Heap transport-
  definition absence; final mutation assertions were not reached. Other own
  WIP reds remain: full page_heap prefix at unused Layout.normalize_size static
  input; pure domain and declared-domain fixtures stop in whole-package
  Instance borrowed-result/source-not-i64 before assertions. None is baseline
  waived or whole-suite PASS. Do not keep flipping expectations without source
  census/entry/actual proof correspondence; production frontier stays selected.

V807 construction connects original Main Mixed demand to existing SourceInstance
Observe/phase checks, fixed support ordinal/acquisition and native input selection.
The retained Mixed source owner corroborates Nullable Fresh+Null/class or exact
parent Mixed source/destination/object/all root Rc. Passive class uses that same
source loan; executable reconstruction still requires existing Ledger proof.
Existing M189 source3 test now includes wrong destination, Nullable/Mixed recast
and foreign-parent Rc negatives. No new phase, solver or QualifiedObject grant.
Build PASS60.081811s; original Instance snapshot1/M189 source1/input finish13 PASS.
Worker confirms no authority hole, and identifies remaining mixed-parameter frame
join: realloc/reallocResult have DeclaredObject+Opaque parameters. Exact original
Mixed demand now borrows existing generic Instance parameter-frame lender (same
implementation previously lent to Object); all source/domain/census checks stay.
V807b root build next; source/runtime acceptance remains unproven.

V807b build PASS61.155951s; original Instance snapshot1/M189 source1/input13
PASS. CLI154 build PASS149.484552s, SHA
19eea33afa869d456895bac9b7bfc2236bd94112f7432fd9afbc1a62ad70e300;
all captured source hashes match root V807b exactly. Unchanged M189 canonical
EXE RED7.209341s/no artifact on C803b/runtime757: original Main arguments stop
is gone, package issuance passes, first stop is callable-main/lowering
ordinary-new/local-commit/print-expression/source-unavailable. This is product
frontier advancement, not Mixed runtime/End or whole-series acceptance.
Worker traces original Print/received field source to its final packet consumer;
getter names the exact first PrefixNotCovered Print, not generic prior failure.
M190 same CLI/C/runtime RED.135606s at Main34 Body1Initializer0 unchanged
borrowed-result/source-not-i64. No claim that the wrapper/source-return branch
is fixed by the Mixed input connection. Full own source regressions still owed.

CLI154 read-only worker pin: Main43 Body7 literal Print has its source frame;
Body8 alloc/h0 scalar-field Print is first PrefixNotCovered, not prior release.
Original h0 Nullable acquisition has no nonnull branch proof; current receiver
source classifier cannot issue that field request. Mixed lacks a receiver arm
for later same/moved prints. No Print concat/Add solver gap is established.
The same binary/hash/argv/env and no inferior calls are pinned in
m189-v154-worker.receipt.json/log; no checkout edits or worker Cargo.

Next bounded mapping (same received-result series): retain exact Nullable
acquisition or Mixed loan/current live chain on the existing field-read source
Rc; original field batch joins canonical FieldRef and class. A fallible received
read must project Normal I64 and Fault through the existing sole field writer,
Print pending-String/prior-Home cleanup writer, same emitted source row,
FinishedBindings and independent final/C consumers. New frame/site-aware getter
must follow the canonical object Fault boundary; the current checked_field_get
is explicitly nonfallible (fault_checked_object.rs) and remains for its callers.
Do not set a nonnull stamp, map null to zero or reinterpret InvalidContract as
source Fault. No new Print family, AST rewrite or .hako change. Pin diagnostic
and source read plan/Invoke correspondence before constructing the checked arm.
Acceptance must cover original M189 plus null receiver Fault/remaining teardown,
Mixed Forward vs Fresh, foreign roots/class/field, consumed source and missing
Normal/Fault binding or cleanup. Current source connection is unaccepted WIP;
first milestone and all full-goal/integration/retirement conditions stay as above.

V808 Decision: same received-result series joins the existing field batch/source
Rc to a fallible FieldGet Normal I64/Fault operation and original Print cleanup.
Source authority is original received acquisition/Mixed loan+live chain and
canonical constructor FieldRef; sole issuer is existing field batch, not ABI.
No language surface/taxonomy is added. Common Outcome kernel and checked-object
FieldSet storage-error law apply: unavailable100, object/field mismatch101;
profile/layout InvalidContract; current store also classifies badslot as101.
Rust/C source/field/layout verification must reject malformed physical input.
Existing nonfallible getter/plain provennonnull reads remain shared obligations.
Runtime read/header/kernel docs+existing read test are first internal slice;
source descriptor/physical Invoke/final/C connections remain in this same series.
Do not open C schema before explicit FieldGet dispatch/result load/flow exist:
its old default emission is release/reclaim and must never receive this operation.

V808 detailed runtime/Invoke/C construction and failed helper/harness diagnosis
are preserved at **57b9a13331** (this path), without source acceptance. Retained
premises: kernel read test PASS SafeMutex+SingleThreadExact; root Invoke module8
PASS; FFI808b build PASS; physical borrowed-ingress full script v808f PASS82.390926s
with stored37 readback, Null Fault101/site100 and remaining Home cleanup, six
field/result/cleanup refusals and all prior runtime/mutation cases. Stable pins:
C808b SHA798930caec98497736fc7a1e66b6c3104102be0517f8c2240ddd227805b183bf,
entry runtime808b SHA2b6083aac7e2d168fe8f76eb821c079900d15e69c8da0bc5eb843a05eba9222c.
Keep these archives/FFI/receipts while source series remains unlanded; the core-only
archive is not an entry archive. Physical cases never establish source cutover.
Old harness Return origins/Array peer transport/legitimate guarded tagged Return
were corrected without relaxing production checks. Diagnostic v808d has source
hash drift; final v808f stable run supersedes it. Source/final obligations remain.

Worker source/final mapping retained in this Decision: preserve exact Nullable
acquisition/ObjectCallSourceLoan or SAME Mixed source loan+all root Rc on the
existing LocalFieldRead source descriptor; numeric ExpressionFieldSourceLoan
remains independent and never supplies nonnull. Existing LocalReadProgress must
retain checked Invoke+Normal pair, exact base/read source and same descriptor.
Print ordered-bindings must include BOTH rows, otherwise its existing path
validator misses the getter Fault. Reuse that validator's acquired String then
prior Home cleanup, not a second field cleanup registry. Same field row owns
completion/all-read census and FinishedBindings projects both original bindings.
Plain Array/Bool callers stay plain. Source descriptor/issuer, sole field writer,
Print/final correspondence and production physical_program acceptance are still
unconnected/closed; original M189 last authoritative run remains CLI154 RED.

V809 construction/debug history is preserved at **b00413b514** (this path).
Retained contract: exact received acquisition/root Rc and numeric field source
remain conjunctive; SAME field row/Copy loan owns checked Invoke+Normal; Print
checks both bindings and original String/Home Fault cleanup; independent final
census and physical lender reject unbound/native opcode acceptance. Source code
is WIP, not acceptance. Builds V809/b/c/d PASS; original batch2/M189 source1 PASS.
Added typed Nullable Print cases are RED placement-handle-result-drift after
canonical target lookup/affine issuing-target borrow repairs. They still lack
full original input evidence; later existing Print negatives were not reached.
CLI155 PASS192.531071s, SHA127e14b91f43d883fdce592976d31e4adfb7c7320186cd86e7c8c7064e9730e9.
Unchanged M189 source705a535c, canonical C808b/runtime808b direct EXE RED.190502s,
no artifact: package Main43 Body8 numeric field source-identity/current-closure.
New source qualification activates that refusal; no product advancement beyond
CLI154 and no new Mixed/source/runtime/End acceptance or source landing.

Worker Decision extension, SAME received-result series, next two connections:
1. Original Nullable local acquisition input: existing raw inventory registers
   exact unique initializer/declaration/binding + SAME ObjectCallSourceLoan roots
   for all-I64 contracts, then its sole scanner collects ALL original callers of
   that callee. Retain demand on the same Instance row; Observe/native selection,
   common SourceInstance input finisher and ObjectPacketSeal borrow it. No fake
   BorrowedIncoming, empty incoming permission, standalone local argument seal,
   QualifiedObject recast or old-protocol retry. Canonical target lookup alone
   is preparation, not complete input evidence. Do not replace the typed positive
   with an easier corpus or claim that its source language is invalid.
2. Original numeric field→New formal→Birth field: Page New literal page_id →
   Page.page_id → Handle New actual me.page_id → Handle.birth formal → Handle
   .page_id. Current seed classifier requires local binding, so this field actual
   becomes Conflict and Handle Birth numeric base never closes. Retain exact
   constructor/formal ordinal/ALL New actuals from SAME new_calls Rc, join their
   original field proofs to SAME FieldCandidates.sources Rc, and extend its
   existing closure conjunction/revalidation. Keep cached integer seeds unchanged.
   Productive grounding must reach literal/existing IntegerBasis; simply adding
   conditional fields to the greatest fixed-point bases admits a rootless cycle.
   No separate solver/cache or declared-type permission. Confirm actual closure
   pins before claiming this static audit is the sole remaining app cause.
The original acquisition-input mapping remains unimplemented. V810 below connects
the numeric field actual; Array-origin actuals remain missing. New field emission/
final consumer code stays unaccepted WIP; own regressions, mutations, source/runtime
and full goal acceptance remain owed.

V810 detailed construction/debug evidence is preserved at **48e2ad16d2** (this
path). Retained contract: SAME original New census and exact constructor/formal/
ordinal/ALL class actuals, no cached Conflict promotion; passive Me field joins
SAME field-source Rcs. Productive Birth least-grounding rejects rootless cycles;
ordinary preservation and writer traversal propagate current dependency loss.
Final use re-demands original census Rc, corroboration and forwarded I64 evidence.
V810c root PASS62.473339s; composition11/input-veto1/field-batch2/original-M189
source1 PASS15. Getter-free Bool/self-copy Print negatives now expect real source
refusal after V809 activated the demand; no permission relaxation/baseline waiver.
CLI156 PASS188.488735s, SHAa1cafc0fdd62192e487fc4969fa5f14e3babf0eb485b00498976b5026c747119.
Same-source manifests/current rehash zero drift. Unchanged M189 canonical direct
EXE on C808b/runtime808b RED.327379s/no artifact: Main43 Body8 field leaf advanced
from h0.page_id to h0.block_id. Numeric source preparation only, no EXE/runtime/
End or source landing. Own regressions/full goal and typed Nullable input remain.
Keep the evidence base/FFI/archive pins while the full source series is unlanded.

Worker next-connection mapping: new Body8/Value/Lhs/Lhs/Rhs is h0.block_id.
Handle New arg0 page_id now closes; arg1 block_id follows its sole stable local
initializer me.free_stack.get(me.free_top). Arg2 requested_size is a formal with
its own source Integer/I64 and resizeInPlace writer obligations still owed.
Existing canonical Array lender is ordinary_new_field_array_read_source.rs:
local_initializer/capture_exact_get retains Rc<CheckedFieldArrayGetSourceV1>,
whole-box array_i64_fields census, canonical field/own-Me receiver, original
index source, same Birth provider/source; recheck_census demands the original
owner/field. Existing retain_argument_reads issues ArgumentFieldArrayReadSourceV1
and the sole field-array recipe owner deduplicates the SAME source Rc by Call.
Its current capture occurs after the Home sweep (coseal_issue.rs), too late for
the numeric New/field fold. Next mapping must connect early retention on SAME
New rows with late reborrow of that existing loan; do not add another Array
classifier, accept get/opcode/ArrayBox as Integer proof, or regenerate a loan.
This phase connection needs confirmation before construction; not acceptance.
Scope boundary: scalar seed classification intentionally leaves unproved local
initializers/arithmetic Conflict. Do not open generic local inference here.
Reuse the exact single, never-rebound Array-get local source loan; its existing
Array census integer_source_at boolean is not general formal-domain authority
(loop/rebind rules differ). SAME early/late source Rc sharing remains to connect.

V811 Decision, SAME Array-get→New→Birth source responsibility:
Source authority + canonical issuer: sole preflight array_i64_field_sets is
already built before original seed capture, borrowed unchanged, then moved into
ledger.array_i64_fields. Existing capture_exact_get issues private immutable read
Rc against that census. Retain ArgumentFieldArrayReadSourceV1 on SAME original
New rows with exact local/initializer/New ordinal; one local yields one read Rc.
Scalar Return and late New arguments borrow the SAME retained read if present;
non-New Return reads keep their existing issuer. Delete late New recapture/cache.
Non-authority: generic local inference, raw integer_source_at bool, type/get/opcode,
new census copies or a new registry. No cached Conflict/Integer domain rewrite.
Fail-fast: missing/foreign original census or New/argument identity, rebound or
foreign local, source Rc conflict, non-I64 whole Array census and changed current
physical census. Numeric loan rechecks SAME original New evidence; current Array
execution/census remains mandatory in the existing late recipe/final owner.
Smallest slice: source factory→SAME New row→numeric Birth input loan, common
Return read reborrow→late argument/recipe→existing independent final validation.
Focused existing composition/field-array Return positives and Bool/rebind/census/
source-identity negatives plus unchanged M189 canonical EXE frontier are required.
No EXE/runtime/End/source-landing or whole-goal acceptance from preparation alone.
V811 root PASS117.207383s; composition11, shared Return/New Array read1,
field-batch2 and original-M189 source1 PASS15. Nonzero Birth ordinal, Bool Array
store/local-rebind veto, source loss and shared read/wrapper Rc are included.
Worker found a blocking ownership coupling before acceptance: the first late
adapter demanded whole borrowed transport even for unrelated scalar locals.
Correction retains the original New source Result/Rc on the existing field
cohort before transport selection; its passive lender survives failed sealing.
Shared exact_get_candidate selects only original own-Me canonical Array fields;
nonfamily locals never demand Array source, while selected missing source keeps
its Err and cannot recapture. Existing pending test covers both boundaries.
V811b build pending after this correction; initial test helper import errors
need fixing after Cargo/rustc terminate. No new CLI/product acceptance yet.

V811b root RED80.991275s (three test helper module paths), corrected after process
termination. Exclusive old result→New Array read lender has caller-zero and is
physically removed with the late recapture/cache; other Return capture is shared.
V811c root PASS122.836817s; composition11/shared Array Return+New1/field-batch2/
original-M189 source1/borrowed source-error1 PASS16. Includes nonfamily Local with
unavailable source, Array preserving source Err, unsealed passive SAME Rc attach,
foreign ordinal/census loss and original New source loss. Worker A+B re-review
found no new blocking issue in this cohort. CLI157 build running; EXE/runtime,
typed Nullable input, own pending regressions and full goal acceptance remain owed.

CLI157 build PASS207.240151s, SHA941d7b0e2abed219d34e002344db638e36cf5c210dd64181297498c6b557871f.
Unchanged M189 source705a535c, canonical direct EXE on same C808b/runtime808b
RED.342121s/no artifact, Main43 Body8 Value/Rhs integer-field/source-missing.
The page_id and block_id field checks passed; next leaf is h0.requested_size.
Source root/CLI manifests are identical; subsequent source edit only removes
one trailing blank line at EOF in ordinary_new_borrowed_formal_result.rs. No
semantic edit followed those pins; no new full-suite/runtime/End acceptance.
Keep v811c-root/cli-v157 manifests, focused records and m189-v157 receipt in the
existing evidence base while the source series remains unlanded.

Worker next mapping: do not infer that usize itself causes this source-missing.
The sole writer inventory attributes Me only; original resizeInPlace's
handle.requested_size=requested_size is a non-Me writer and vetoes that field.
First observe its exact value_source error and candidate I64 evidence for the
same New formal/Page.allocate/resizeInPlace value formals in the existing source
fixture. Retain every original writer and join existing formal receiver/class/
canonical field + Integer reach/source correspondence to the SAME inventory;
do not drop it, rewrite it as Me, trust a declaration alone or fake a completed
field-write frame before its phase. Existing checked_field_write_pair_v1 and
borrowed_field_write_use_for_frame_v1 already own the late correspondence.
FieldCandidates needs their original source prerequisites before closing.
If ALL actuals/writers prove signed-I64 payload, preserve usize declaration and
range checks and reuse current I64 Print/FieldGet/storage. SourceInteger-only
usize or u64's upper half cannot be NormalInteger; no i64_source relaxation or
new unsigned-to-signed cast. Existing whole-caller/field/Fault bounds remain.
Missing early class/source lender is internal mapping work, not external block.
Typed Nullable full original input remains a separate required same-series gap.
