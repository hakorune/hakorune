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
- Next acceptance connection: original self Nullable observation/local-flow →
  existing full object packet → common final Call visitor; V799 below.
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
- [ ] Independent strict consumer deficit: V753 added whole compiled-entry
  demand exposed Heap.realloc optfalse `published-lifecycle/borrowed-use/forbidden-operand`.
  Not proven baseline. Removing that broad demand from the correspondence test
  did not discharge it. Re-evaluate at the real final bridge; coordinate and
  instruction diagnostics now exist in both final scans.
- [ ] Rebuild canonical CLI, run unchanged mimalloc with pinned current runtime
  and FFI, execute and compare existing suite stdout/exit. Re-pin a negative only
  at its truthful rejected boundary. Complete required fixed acceptance.
- [ ] Name and retire replaced exclusive edges/adapters after their production
  caller cutover, caller-zero and required evidence in this same series.

## Latest focused evidence and current verification

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

## Current construction and measured consumer frontier

V790 source-declaration connection WIP: qualified source re-lends the original
catalog kind/ordinal/BindingRef/entry Value and source receiver offset. Declared
Object additionally requires is_declared/class/canonicalobject; opaque requires
original OpaqueHandle plus no annotation. Compiled-entry corroborator joins
that object to the existing closed-use map keyed by parameter ValueId. Tagged
backend branch keeps wiretype/carrier/arity/incoming checks and demands this
same declaration correspondence, including unannotated headers (erasure refused).
No second catalog, inferred-view permission, name-only permit or C ABI change.
Existing 96 stored-child source cohort now demands the parameter backend gate;
its original declared case adds wrong/erased header, ValueId, wiretype and
receiver-role negatives. No new source fixture or test function. V790 pending.

V790 build PASS64.927497s. Existing stored-child source cohort PASS20.91s
(96 original variants/both opts plus five declared-header/Value/role negatives),
binary/source pins in v790.focused.json. Worker found no definite safety gap,
interface mismatch or necessary profile over-refusal. C wire/ABI is unchanged
by this source-to-backend validation join. CLI144 build pending; app execution
acceptance and V753 optfalse strict-consumer debt remain open.

CLI144 build PASS147.681373s; canonical unchanged app started with FFI787 and
runtime757. Additional existing Generic G0 numeric parameter positive and
declaration/carrier/contract negatives PASS (.02/.03s), same V790 root binary;
this checks preservation of the disjoint numeric branch, not a full-suite PASS.
No source edits after the successful CLI build; currently no Cargo/rustc.

V791 behavior-preserving normalization within the same declaration owner:
read-only audit found V790 requalified the same function's whole input for
each tagged formal. The source getter now lends qualified entry/catalog/
values/receiver mode in one scope for all tagged declarations. Backend gathers
the same already-checked logicalordinal/ValueId/header requests and joins every
result to the same closed-use view. All original row, class, binding and scope
checks remain; duplicate requested ordinals refuse. Per-formal query API was
removed, without cache/registry/adapter. Reuse V790's existing 96+5 acceptance.
CLI144 remains running on V790 source/binary; its longer elapsed time alone
does not attribute the cause or prove V791 speed improvement. V791 pending.

CLI144 terminal: build147.681373s/app474.585156s rc1/no EXE. Original typed
parameter gate passed; canonical unchanged app reaches the independent C
consumer for the first time in this series. First-stop published-lifecycle-v4/
unsupported-cohort; binary80e4f207dc16d2b3e749f200b09e4e6092eef1fd3dce76290b5a2d1d1ab5b085,
app33d3/FFI787/runtime757. C measurements: structural V2 validation305.239363s,
V4 admission.006287s, emission0; 30functions/2019blocks/2847rows, indexed
functions26/blocks304/values534, resolves0 and flow0. V4 count increments at
function start: first failure is fi25 inventory, before kind/producer/flow.
Read-only audit locates fixed256blocks/4097value constraints, but no raw counts
for fi25 yet: capture original C-boundary payload before choosing a repair.
Do not raise limits alone: fixed workspaces and Map2+origin collide with
Birth258/259/Mixed260/Owned513/String1024. Mixed missing from index ordinary
role membership is a separate confirmed consumer omission, not measured first
stop cause yet. Existing source authority, all arms and both edges remain owed.
V791 build65.912811s/cohort96+5 PASS20.79s; G0 numeric positive/negative both
PASS. Different first-stop workloads mean CLI144 elapsed is not a V790 param
performance regression measurement; the C trace identifies the dominant305s.
CLI145 build pending; capture next SAME canonical invocation's temporary JSON
verbatim for independent C diagnosis. No repaired wire or source workaround.

V792 Decision after verbatim canonical C-boundary capture: app-v145.c-input.json
SHA da49c9ca322aff8176960bb1d47aa04b50f61af816fcaca4052a66014cfaaa07,
producer manifest cli-v145.source.json, FFI787/runtime757. fi25 is MiWorkload.run/0
with1480blocks/274rows/capacity276/274distinctvalues: fixed block256 cap is the
observed refusal. No value-duplicate/capacity repair or source rewrite is needed.
Use checked actual-count indexed workspace, preserving one-enqueue/exact-revisit
state+Fault, complete inventory and unique source producer checks. Private
state alias separates Map transfer and String rank from fixed Birth/Mixed/Owned
markers with disjoint tags; all consumers use common helpers. Count conversion,
n²/n+1/byte overflow and allocation failure refuse. Native recursive cohort
keeps its separate fixed depth. Borrowed predecessor suffix uses checked sized
scratch with its special prefix/full loop visit preserved. Common ordinary
role predicate includes already-defined Mixed across index/flow/emitter.
No change to JSON, runtime ABI, source capability or cleanup meaning. Retain
all old focused source/C regression/Fault and malformed-field/Return tests;
require original app capture to pass independent C and canonical direct EXE.
This is the same required final consumer series, not an optional backend lane.

V792 implemented checked actual-count indexed state/queue/scratch and dynamic
borrowed suffix storage; private64bit disjoint tags and common ordinary-role
membership replace the old cap/fixed indexed arrays and duplicated role lists.
Checked value-budget arithmetic and String push refuse overflow. Native stays
bounded256/4096; unrelated function/value limits remain. Read-only worker found
no actionable cleanup/queue/revisit/tag/membership gap. C build PASS5.986063s;
original captured app input plus shared owned/Map/String regressions pending.
CLI145 terminal pins above: original input capture is diagnostic evidence only,
not a second source authority. No EXE or series acceptance yet.

V792 shared regression evidence: owned16 plus original-return/correspondence and
Fault negatives PASS138.751473s; indexed/native String execution/LIFO Fault and
malformed inputs PASS; owned Array Normal/append/child-End Fault/reverse End and
malformed lifetime inputs PASS (last two not separately timed). FFI792 SHA
2ff3e2662ae51484bbb761456062b280858e7eae913e3e4283f267b5afa843a3,
runtime757 unchanged. Map physical fixture initially refused abi-layout;
identical refusal reproduced on FFI787, so this is stale synthetic schema,
not a V792 regression. Existing field-free fixture now declares empty required
owned_residences; FFI path is explicit. Its existing cohort adds one padded
>256block Map transfer case, no source acceptance claim/new proof file. Map
recheck and original app C input remain running. Original source Map Call
regenerated by reused V791 root test: PASS1/.05s both source opts. Pointer,
no-fallback, builder-layer, thin-shim and diff guards PASS.

V792 Map physical suite PASS: seven linked EXE30 cases including >256blocks,
reversed document order, Call/Return transfer and existing runtime Fault/kind/
borrow/lifetime refusals;27 malformed inputs preserve prior output. Synthetic
schema update claims physical representation safety only. App raw C validation
continues; no source/EXE completion claim. Successful String/OwnedArray/Map
receipts record unmeasured durations honestly rather than estimating speed.

V792 raw app C terminal rc1/334.395836s, no OBJ. Inventory now covers all30
functions/2019blocks/856values; V2 validation334.372210s, V4 .013289s,
resolves152/flow0. Fixed workspace refusal is crossed, but kind closure fails
at fi11 HakoAllocHeap.allocate/1 (OrdinaryNullable), Value11/block348:
object_field_get of exact entryreceiver0/object3/field0, then Page.allocate Call.
The sorted complete inventory independently maps resolve152 to that row.
Read-only worker is checking entry child classification and downstream lifetime/
argument/Return/End boundaries before the next common-owner correction.
V792 remains unaccepted overall; no canonical EXE retry until this refusal is
resolved. This does not erase the prior focused successes or existing debts.

V793 Decision, same required C consumer: original entry-receiver field loan
must not depend on the function's result kind. Sole physical authority remains
the original object_field_get/Copy, canonical owning tuple/provider and exact
receiver or original New root. Non-authority: ordinary role alone, negative
origin, layout/class identity alone. Normalize all child loans to the existing
nonowning field-borrow origin; only the true canonical receiver keeps -2.
Shared readability checks each original edge's unique Birth provider and both
read/use suffixes. Exact ordinary receiver terminates at entry; Birth keeps
existing committed-field/childBirth restriction; ownedNew keeps original unique
Birth Normal and live parent. No Home/Return/End/argument permission is issued.
Read-only audit rejected the smaller role-only patch because it would retain
unconditional call-live child shortcuts. Reuse stored-child96 and fieldReturn6
source captures plus their negative lifetime/identity/slot cases, then original
app C and canonical EXE. No new app fixture/AST rewrite/registry is required.

V793 C build PASS6.007161s. FieldReturn6 source-issued Normal/Fault execution
PASS36.094181s. Stored-child96 originals compiled; first negative run RED3.26s
only because foreign-child now refuses earlier at the provider boundary.
Read-only review confirms no acceptance hole: update only that mutation to
strict unsupported-cohort; missing-child retains strict receiver-object-mismatch.
No combined-error acceptance or shape sniff. Rerun PASS14.950507s, including
all original identity/argument/slot/lifetime refusals and both-opts real callee
Normal/Fault cleanup. FFI793 SHA6fd137dfa336c562d18ea1c90d0a8b346a21bbd0d4072d341fe02f456db32862.
Worker concrete review found no new Home/Return/End/argument permission or nested
Birth expansion. Actual16 child reads all use Heap field0/1→Page providerNew
with ordinaryreceiver, ownedHeap or Birthdirect roots; no Provided typed child.
App C validation continues on unchanged original captured input.

V793 raw terminal rc1/306.062540s/noOBJ: all indexed ordinary functions now
pass child seeding; resolves809/flow0 stops within MiWorkload kind closure.
Raw first non-scalar Phi114–118 carry unchanged Array holder, Heap New, exact
receiver and two Page field loans, each with identical incoming original value.
Existing kind closure admits only I64/Bool Phi. Source if-merge semantics and
original Phi were already preserved by fb512d0549; do not rewrite AST/JSON.

V794 Decision: source remains original Phi + exact predecessor/value rows.
Normalize physical alias identity inside the existing pervalue index, bounded
Copy plus acyclic unanimous Phi only; kind union never issues identity. Require
nonempty exact predecessor census, no duplicate/missing/cycle/differing root,
and final HANDLE root-kind agreement. Keep I64/Bool scalar Phi unchanged and
other carrier Phi closed. All existing Copy-root consumers must borrow the
same validated original root/origin/object; flow must check current readability
or existing owned-Array holder liveness even for unused Phi after End. No new
Home/state, source registry, rewritten input or LLVM writer: retain phi i64.
Read-only review confirms actual roots and current consumer/emit gaps. Reuse
original source app plus existing stored-child/owned-result C families and their
physical mutations for alias/lifetime refusals; no new proof app.

V794 implemented original-row alias roots in existing value cells, separate
from kind union. This structural identity is available to classification;
seed still requires original terminal HANDLE/receiver/New/class conditions,
and full mandatory kind closure precedes flow. Other carrier Phi stays closed.
All old Copy-root callers now use lv4_value_alias_root; old helper/owned-result
Copy walker removed. Phi row borrows existing object/field/Array liveness,
without state transition; existing phi i64 emission is unchanged. Read-only
concrete audit found no cache/cycle/scalarloop or authority gap. C build
PASS5.970196s. Stored96 + same-receiver Copy/Phi wire witnesses, malformed
Phi and original callee Fault/identity/lifetime negatives PASS14.825800s.
FFI794 SHA1e1217b0639b547a3a5ad2c3ca7f3096739b5c20f34090d881350a84ee44f107.
Owned16 suite now includes two-Copy same-root Phi Normal/Fault positive and
strict differing-root/unused-after-End Phi refusal with preserved output.
Initial owned run deliberately interrupted28.246258s after discovering missing
required branch edge-args in the new physical harness; not a PASS/product red.
Harness corrected; owned rerun and unchanged app C validation are pending.

V794 owned16 + original-return/Fault/metadata cases and added same-root Phi
Normal/Fault, differing-root and unused-after-End refusals PASS131.009003s.
Raw C terminal311.329558s rc1/noOBJ: all856 values resolved, flow118/rows172.
V2 validation310.463128s/V4 .858067s. BFS of original edges maps stop to fn12
HakoAllocHeap.allocateResult/1 bb392 ReturnFault: completed result21 remains
LIVE after prior nullable12 release at387 faults. It is the Null arm, not
Provided-success branch400. Null/runtime-noFault pruning is no waiver: the
published CFG's both edges remain checked. C correctly refuses the open debt.

V795 Decision (read-only source and independent C audits agree): existing
RootHomeCleanupOrder owns pending Return semantics. Direct owned Call already
prepends complete result End to full, excluding normal/acquisition Fault;
Received uses its existing destination Home; Borrowed/Null/scalar add none.
Fresh destination-less Construction is missing that same fault-only connection.
Use original terminal Construction site, SAME NewResultCommit/emitted result,
completed Birth and original object/destruction/sealed children. Project full
End through ordinary_new_object_end::operations_v1, then existing sole root
cleanup graph and independent original/FinishedBindings consumers. Preserve
New/Birth Fault reclaim separately; a completed result uses complete End.
If result End faults, continue residual cleanup without retry. Source normal
Homes already exclude arguments moved into completed Birth; never double-End
those arguments after result's child teardown. No C waiver, fakeBinding,
fakeDirectCallsource, new Home registry or .hako change. Reuse existing root
order/graph/return-new/source app tests and physical fault/operand mutations.

V795 implementation uses original Result commit directly; no new holder/enum.
Generic fault-only prefix prepend is now shared with existing Direct Call;
selected no-ingress Return binds Fresh only when Normal cleanup is nonempty.
Common record/final entry hook recomputes its source plan; separate original/
FinishedBindings producer checker remains. Worker concrete audit found no
identity/phase/Provided-double-End or consumer-source mismatch. Build
PASS66.353660s; Direct order3/order5/graph5 and existing joint final-consumer
mutation test (extended original Fresh body with prior Spare) PASS.
Initial filter spelling matched0 tests: invalid selection, never a PASS.
Instance-value cohort6PASS2RED: intentional duplicate-Birth-store fixture is
RetainedUnavailable and must remain diagnostic-only; V795 initially rejected it
before that phase check. Fix preserves original Fresh checker's explicit
RetainedUnavailable None, without End/artifact permit; proper Emitted checks
remain. Separate Float argument refusal is borrowed-compare/integer-source-
unproved before this fresh cleanup, an unresolved cumulative-WIP diagnostic
expectation red, not a proven clean-HEAD baseline or a reason to relax checks.
V795b rebuild pending; no full cohort PASS/landing or product EXE claimed.

V795b build PASS38.853313s. Fresh whole final-consumer mutation test PASS .74s;
retained-unavailable New source claim test PASS .01s, no End permit. Duplicate-
Birth diagnostic fixture now passes the new cleanup binding but remains RED at
IncompleteOrdinaryNewCoverage (instead of own cleanup/construction stop).
This is another unresolved cumulative-WIP unsupported raw completion expectation,
not a proven clean-HEAD baseline; neither red is waived and no full cohort PASS
is claimed. V795's completed-production source/Fault mutation evidence is green;
CLI146 required quick build is next. V753 strict optfalse and broader original
suite/source-lender reds remain separate open obligations at eventual closeout.

V796 bounded C preflight (not a claimed new measured stop): full actual30fn
role/parameter/consumer audit identifies one remaining inherited role omission:
borrowed_tagged_formal excludes ordinary_bool. Actual Heap.release/1 and
Page.release/1 both need original declared-formal field reads/tagged forwarding.
Normalize receiver-bound membership to existing lv4_is_ordinary; preserve
staticI64/Borrowed/Mixed profile restriction, exact param representation, same
alias root/object_view and all lifetime/Call checks. Result-specific Return/
Mixed/Borrowed/Birth arms intentionally remain separate. No new source permit
or hypothetical Unit-param task. CLI146 build PASS152.029184s; use current Rust
binary plus rebuilt pinned FFI for canonical unchanged app with raw capture.

V796 C build PASS6.052680s; FFI SHA
0e37ac3a5ec8b5ef7603e39fb336113a12a4ae5b69614a2f6b1bcb396a723363.
Initial Bool-discard C run rejected stale captured function-body input before
V4; no JSON repair. Reissued original Rust source fixtures with current V795b
root binary: existing borrowed statement-discard test PASS1/.42s. C rerun
PASS12.335907s, both Bool arms/real execution and existing refusal checks.
This does not claim the app's guarded Bool field-read/forwarding path green.
CLI146 canonical unchanged app is running against FFI796/runtime757, with
verbatim input capture. Capture observer's initial glob-directory race failed
before a product result; child stopped and absence of output verified, then
observer fixed and fresh app run started. No product failure counted for it.
Pointer/no-fallback/builder-layer/thin-shim guards and diff check PASS.

V795c regression reconciliation (read-only worker audited both source owners):
Bool→formal integer comparison is correctly denied by original domain agreement,
so replace stale nullable-carrier diagnostic expectation with exact integer-source-
unproved. Duplicate-Birth-store fixture deliberately has RetainedUnavailable;
its former successful raw completion and release-count1 conflict with the
current artifact coverage boundary. Keep the same fixture as exact
IncompleteOrdinaryNewCoverage negative, not an End permit or production repair.
Existing owned_call_results_publish_original_child_handle_contracts verifies
actual nullable→completed Birth original ordinal/value correspondence in both
opts; original_main_and_real_children_document_handoff_survive_contraction
checks source move-before/Normal/Fault inventories. Existing birth-handle C
suite checks allocation/Birth/store Fault balance (its nullable wire mutation
is not independent nullable source issuance). Preserve these separate positives.
No clean-HEAD baseline claim; test reconciliation awaits rebuild/focused run.

V795c root build PASS69.069733s (overlapped live app C validation; no speed
baseline). Instance-value cohort now8/8 PASS .10s; existing source contraction
move/Fault inventory PASS .33s; nullable Birth physical publication PASS .08s;
joint independent final-consumer mutation PASS .81s. No acceptance waiver or
new supported construction shape. Earlier two reds are reconciled expectations;
V753/JSON3/plain-loop source-lender obligations remain open. Root binary hash
and source manifest are in v795c.focused.json; CLI146 binary remains untouched.

CLI146 verbatim C payload SHA
fbd25f988ae14b3cb2baec922580a9d5a22bf6dd1198b7379c9f750b7dfda544,
30fn/2031blocks. Live original temporary JSON was copied read-only after the
observer omitted hidden .tmp dirs; corrected the scratch observer for later
runs, without restarting/mutating this product process. Independent review:
allocateResult21 Birth386 Normal387 release12 Normal397 Return21; Fault398
field2 child End→390 outer21 End→389 ReturnFault, both child/outer Faults
continue residuals. Existing construction-Fault383 reclaim21→380 release12
retained. Finite static walk of9 completed Fresh results finds no missing outer
End on ReturnFault. Structural audit only; live C/EXE result remains pending.

V797 Decision (two independent read-only source→ABI→C reviews):
DeclaredFormal proves original root/alias/DeclaredObject class, not fixed
nonnull tag3. Its source issuer differs from Forwarded only in proof entry.
Actual realloc formal1 has already-closed borrowed_kind_payload/object_view0;
fixed3 erases original %k1. Normalize to tagged only after same original caller
entry root/value/carrier join; nonselected i64-handle corridor retains its own
representation. Preserve existing ABI source/finished operand validation and C
exact formal/alias/object_view gates; no null-guard-only C permit. Reuse
declared_and_forwarding_only_domains_publish_exact_class_layouts (direct/copy,
null/object,bothopts), existing carrier/producer mutations and actual app.
No new registry or proof fixture. Slice paths: existing JSON call transport,
declared publication tests, publisher README, this card/current pointer.

V797 implementation joins DeclaredFormal to same original root entry and
caller parameter ValueId/installed BorrowedTagged carrier before tagged JSON;
selected missing/duplicate/carrier drift refuses. Nonselected declared-handle
transport remains3. Existing original Call/caller/callee corroboration and
both operand scans retain authority; read-only concrete review found no gap.
Root build PASS61.894016s. Existing declared/direct-copy×null/object×bothopts
source/JSON and carrier/producer mutations PASS .91s; original forwarded/
receiver/typed-forward source cases PASS .12s. Physical JSON cohort32/35 PASS
.70s, same3 named cumulative reds. Worker preliminary classification: reverse
Array caller Birth-Fault expectation is stale (partial cleanup now callee-owned),
but callee obligations must be verified before changing it. Nullable fetch and
Pair.sum sources remain legitimate and expose actual retained-call/root-phase
deficits; never convert them to expected refusal. CLI147 build, same original
app bothopts independent final-consumer test and declared C execution pending.
C declared test gains existing HAKO_TEST_LIFECYCLE_ARCHIVE override to pin
runtime757 instead of unrelated mutable target archive; no input repair.

V797 review clarification: function membership does not imply every root has
BorrowedTagged carrier (issuer replaces only selected slots). The encoder
requires the exact original root/value/carrier join; missing selected root
refuses rather than retrying fixed3. A mixed nonselected i64 formal has no %k
and already fails existing C typed-live, so this issues no unsupported carrier.
Nonselected whole-function old transport stays unchanged, without acceptance claim.

V797 existing declared C suite PASS196.347264s with FFI796/runtime757:
20 original source-issued both-opt programs, Normal and each caller/Birth
Fault cleanup once; null retains its tag, foreign-class runtime refusal and
15 malformed original-view/alias/release/scalar actual refusals preserved.
CLI147 build PASS165.534475s; unchanged canonical app running against same
FFI/runtime with corrected observer. Bothopts actual source/independent final
consumer test remains running on immutable V797 root binary; do not overwrite.
Broader JSON debts mapped by worker: old receiver Nullable path omits finalized
lexical packet (needs same emit_receiver_object owner); Pair.sum child I64Add
has root-only terminal preparation/phase responsibility (retain positive);
Array BirthFault caller reclaim-only is intended, verify original per-store
callee cleanup before updating stale assertion. No test retirement or waiver.

Required regression repair mapping (read-only, not started/new lane):
- Self Nullable literal source: requires_object_packet_v1 currently selects
  self receiver only for original forwarding-input requirement. Extend its
  existing exact result observation law (same nullable source/callee/local
  destination/class), then checked_object_lexical_slot lends original whole
  input/result/Completion packet and existing emit_receiver_object/group.
  Do not fabricate an empty declaration or retry old emit_receiver_nullable;
  retire old dedicated writer only after all affected callers switch/zero.
- Pair.sum child I64Add: owner+exact exit relation/prepare must replace root-
  only gate. Reuse sole selected writer; owner-indexed raw reservation and
  completion census. Extend existing FinishedBindings I64Field protocol to
  original two reads+Add+same Return; current child final validation omits Add.
  Removing root gate alone would leave independent final mutation unchecked.
- Array partial Birth cleanup: original source fault_discharge is prior
  committed fields newest-first, inside Birth; caller is reclaim-only.
  Preserve Normal/reverse End and exitFault assertions, inspect same issued
  Birth per-store paths instead of assuming both children exist on every Fault.
  Existing construction_store_fault_inventory_is_prior_committed_source_order
  and caller_provided_object_store_seals_prior_owned_cleanup remain source laws.
These obligations remain open; no regression-positive is converted to refusal.

CLI147 original C capture at193.050147s: SHA
5408d9a3c2cc87e086c4655a788d5722eb684bca98cf77a2041f595adc44d3ca.
Exact realloc/2 bb438 now sends original formal1 as tagged to same target13;
its original object_view0/receiver0/Normal442/Fault443 preserved. Live C pending.
Self-Nullable repair census: old emit_receiver_nullable has one production
caller (direct_call_disposition_port fallback); after full selected switch and
acceptance, remove both (~150 duplicate lines). Keep shared record/dispatch and
other Handle paths. Nullable observation already triggers full Home walk;
Handle observation alone must not widen selection or invent source Home flow.

CLI146→147 parsed original-payload diff: exactly6 actual kind fields change
3→tagged (realloc4/reallocResult2); all other rows/values/edges/cleanup unchanged.
This is a read-only input comparison, not a repaired payload or C acceptance.

V798 Decision: existing shared plain Array operation writer also serves indexed
object programs, but its checked append/claim declarations were native-only.
Read-only full actual operation/external-symbol census: appendI64 four sites;
this is the only missing actual external declaration. Move the existing eight
Array/new/claim/append3/set/slotload/release declarations before profile split
once and remove branch duplicates. Keep all operation/type/flow acceptance and
other family declaration owners unchanged. No per-app opcode table, special
case or new registry. Reuse existing native Array fixture39+malformed/runtime
and same canonical app; indexed original app supplies real plain-field append.

V798 common Array declaration emission implemented, branch duplicates removed
(net5 emitter lines deleted; no admission change). Concrete read-only review
confirms all8 signatures preserved exactly once, native content unchanged and
other declaration owners untouched. C build PASS6.775969s; FFI798 SHA
ec0ac2127f1b2d4a8aebef80c92086c362c0b73794b1921ce15071a9d0c4ef4d.
Canonical app148 uses same CLI147+runtime757, new pinned FFI798; running.
Existing ignored native39 source host/C/malformed/runtime test also running on
immutable V797 root binary. Its host path hardcodes current target archive
9f6fe986691e9e1b754b41b632d1453fe9bc6c627b21ddbd3728699723bc7dec;
independent Python observer explicitly pins757. Receipt distinguishes them;
this native regression does not claim same-runtime product acceptance.

V798 terminal: app148 canonical compile575.569618s rc0, direct execution
.001782s rc0/exact stdout; first milestone achieved on pinned WIP. Root source
and FFI/runtime/app/EXE identities and exact command/output above form the
reviewable receipt, not /tmp path+SHA alone. Keep original input, first EXE and
run evidence until same-series landing/closeout retention decision. Native39
existing ignored source host + C physical/runtime/malformed regression PASS
519.883085s (two runtime pins distinguished above). Whole actual source bothopts
independent-consumer test remains running; no full suite/source acceptance.

Next same-series acceptance slice V799: preserve the legitimate self Nullable
source positive by joining its exact original observation/local-flow to existing
full object packet issuer and common emit_receiver_object. The Decision and
caller census above apply; no new lane/feature or change to first milestone.
Selected paths: ordinary_new_lexical_object_slot.rs, existing source observation/
packet owner if needed, direct_call_disposition_port.rs and old terminal_call.rs
writer retirement after caller-zero/verification, same existing regression
tests/owner README. Pair.sum projected Add and Array Birth-Fault assertions
remain owed after this slice. Retain whole unaccepted ObjectForward series and
all remaining R6/R7/fixedsuite/Loop/authority/parser/selfcompile conditions.
