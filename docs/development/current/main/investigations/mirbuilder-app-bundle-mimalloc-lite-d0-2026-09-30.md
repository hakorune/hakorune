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
- Next acceptance connection: V799 self Nullable common packet switch/old writer
  retirement passed focused checks; V800 child Add retention/final consumer and
  V801 Birth Fault assertions are under acceptance below.
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
