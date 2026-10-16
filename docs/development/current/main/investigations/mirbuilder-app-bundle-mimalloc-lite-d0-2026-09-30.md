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
execution, expected output and exit. Full `MIRBUILDER-FINAL-PIPELINE-v1` remains
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
- Current connection: received Mixed result → exact returned-New argument and
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

CLI139: quick build **200.906711s**, direct app attempt **182.240261s**,
**rc1, no EXE**. The previous Mixed Birth ABI refusal was passed. New first-stop:
`published-lifecycle-physical-abi/actual-kind-unavailable`, scalar-kind,
`HakoAllocPage.allocate/1`, Birth actual ordinal1, owner(1,26), Body13/Value,
Local binding2 / Value30 / MirType::Unknown. Source is
`block_id = me.free_stack.get(me.free_top)` → `new HakoAllocHandle(...block_id...)`.
This is not Mixed runtime success: ABI fails on this independent scalar argument
before the complete app reaches C. Keep all Borrowed/Mixed execution claims open.

```text
target/quick/hakorune --grammar-profile canonical --backend mir \
  --emit-exe <fresh-output> --emit-exe-nyrt <runtime-v757> apps/mimalloc-lite/main.hako
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1
HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none
HAKO_AOT_FFI_LIB=<ffi-v781/libhako_llvmc_ffi.so>
```

| identity | SHA256 / source |
| --- | --- |
| unchanged app | `33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685` |
| CLI139 binary | `9e61129c60707dcbb4d38b871df14abf2df5cc890f45cd9b82c24c2d9438f8e9` |
| FFI781 | `1f3fb9c8697630fb1737f2721c32a42c82b61394d2f627137ed7c1aa4831d18b` |
| runtime757 archive | `6897c4dcf6ee871f152732f12204733947cb07a922228b0c749df6beec3eea7b` |

Source manifest/run: `cli-v139.source.json`, `app-v139.json` under the evidence
base below. Previous CLI138 result is preserved by docs checkpoint8e6784afc9.
CLI139 build ran concurrently with owned C execution, and app with root build;
these durations are measured results, not a speed baseline.

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

## Current construction — V759 formal field support (verification pending)

The same completed-index leaf owner now lends the original Fresh returned-New
support census before/after take. Mixed correspondence joins that same relation
Rc and formal/ordinal to its original received packet; the existing borrowed
argument owner lends its original formal entry and Copy proof. Finishing checks
that exact parameter/carrier and captured Copy relation. No TypedHome support,
Mixed ABI or Forward End gate is opened. Existing source-only app inventory and
actual source table gain the corresponding formal-support/parameter-drift cases.
Root V758 verification passed on pre-V759 sources; do not conflate them.
Same heavy test changed from V756 979.80s to V758 723.39s; this measures test
execution after removing redundant source proof demands, not build throughput.
Read-only review found no concrete source/ordinal/finishing-Copy mismatch in V759;
this is construction review, not build or runtime acceptance.

V759 first build failed23.713030s on two own interface omissions: source-row
lending visibility and the retained caller-ordinal accessor. Both were connected
within the same coseal owner; V759b build PASS68.725790s. No ordinal substitution
or source-admission rule changed to fix compilation.

V759b focused10 + field-return1 PASS. Original app inventory reached and passed
the new two-Forward same-formal/same-Rc assertions, then failed4.64s at the same
pre-existing static-key None unwrap (now line322); whole test remains baseline
RED, not PASS. V759b actual source/negative test PASS729.577406s both opts,
including same original field support and caller parameter ValueId drift refusal.

V760 construction is a separate behavior-preserving C slice: existing nullable
object Birth formals receive an internal outcome argument beside the unchanged
kind/payload. All corresponding callsites supply current Null0/Fresh1 only;
Provided stores borrow the original formal/Copy outcome, Provider stores keep
their Fresh law. Constructor entry and field reads still reject Forward2.
No new external formal representation, source admission or runtime owner exists.
Reuse V757 Provided/Unit/field/stored-child C execution and Fault negatives.

V760b C build PASS6.544666s after correcting one own format-argument count
error (initial build RED6.165270s). Provided3 PASS24.390685s, Unit7
PASS45.485937s, fieldReturn6 PASS38.844303s, storedChild96 PASS17.245536s;
FFI cfbd77e0b7011a28f9225d2bdead5d0dd9184aff7fb220b4d02a0601717b81e4.
Review found no missing Birth caller/LLVM name mismatch; no Copy admission added.
V760d final refinement refuses a missing nullable-formal origin (-3) or tagged
store loan instead of deriving Fresh. Build PASS6.555858s; Provided3/all selected
Fault and mutation cases PASS23.801344s. FFI760d SHA
c10cad74216f4d3ea142d34027a73ade5e2eebab5dab16b12b263f815df0292e.
V760c preceding tagged-only guard build/Provided PASS is superseded by this
explicit original-formal check. Unit/field/stored results above belong to V760b;
only the original-formal lookup/refusal changed afterward. No new Copy admission.

V761 unbuilt test-demand change restores the independent whole compiled-entry
consumer after the same received-Birth positives, to diagnose the retained
Heap.realloc optfalse forbidden-operand with exact coordinate/instruction.
This does not change admission or classify that deficit as baseline. V759b
PASS predates this demand and V760; retain its own manifest/results.

V761 also partitions the 787-line JSON owner into function/instruction transport
(486) and its existing operation responsibility (306), before adding result
transport. Inverse move reproduces V759b source SHA exactly; only module paths
and helper visibility adapt. BoxShape only, with no changed rejection/admission.
Reuse all existing physical_program_json::tests and the actual source test.
V761 build PASS57.946779s. JSON module33:30 PASS/3 RED (.70s):
nullable_receiver_call_serializes_nullable_handle_and_checked_release refuses
borrowed-declaration/missing-retained-call; ordinary_instance_function_publishes_
canonical_receiver_object refuses ordinary-field-read/root-exit-phase;
owned_array_fields_release_in_reverse_order_before_home_release expects two
field releases on a construction Fault but sees reclaim+return_fault only.
Classify as own unresolved cumulative WIP cohort/expectation debts, not a module
PASS or a claimed exact-baseline run. The first two stop before JSON; operation
port inverse SHA proves unchanged operation bodies, not baseline acceptance.
Retain them with existing borrowed-entry/field/Array obligations; no new lane.

The strict whole compiled-entry consumer exposed unanimous formal Phis from the
original if-variable producer. Diagnosis, safety boundary, temporary diagnostic
retirement and producer fix are landed at **fb512d0549**, together with exact
focused36 and both-opts actual-source evidence. Its common merge replaces the
pin-only second writer; preserved source Phi semantics and final consumer
refusals remain. Refer to that commit for the completed V761–V764 history.
The cumulative Mixed/field/Array source WIP is separate and remains unaccepted.

Next implementation mapping (read-only owner review completed): retain outgoing
field supports on the existing FinalizedObjectReturnOutcome, extending existing
Handle/Nullable roles without another packet enum or source registry. Reuse
checked_callable_object_result_v1's original complete terminal census for an
already-selected callable, including uncalled reallocResult. Direct Fresh lends
returned_new_field_supports_v1; relay lends the SAME normal-return handoff proof
after recheck. Retain original relation/witness Rc and exact formal/Copy/finished
correspondence. Existing indexed object-result C owner checks those source-issued
Return coordinates and substitutes callee formal through each actual for Call,
relay, field read and complete End. No incoming authority comes from publication;
outer LIVE ownership does not own Forward children. Wider TypedHome remains closed.

V765 construction (unaccepted): existing Handle/Nullable physical roles now
retain the same complete original Return outcomes. A new owned-projection child
borrows checked_callable_object_result_v1 and completed catalog/Completion/
terminal identity; no incoming Call is invented for an uncalled selected body.
The existing composition loop lends immediate callee→caller support pairs before
ordinal substitution information is lost; its caller-zero wrapper was removed.
Received-Birth kind and support projection share one validation nucleus.
Support rows retain SAME relation/witness Rc, entry, finished Copies and exact
Call Invoke/Normal plus Fresh outer Birth actual binding. A read-only review
identified those latter bindings as necessary for independent post-projection
mutation rejection; they are now retained, not replaced by an origin walk.
New source-owner README lives beside the owned Return projection rather than
growing the oversized inherited package README.

V765 initial build RED79.412246s on own missing TerminalRelationV1 import; fixed
in V765b, build pending. Current JSON/C does not yet consume field support rows;
new ordinary Return result_origin serialization must be connected to the existing
C schema/index checker before C acceptance. This is an explicit unfinished
same-series consumer connection, not an accepted C regression or EXE result.
All Mixed Birth/Forward residence gates remain closed.

V765b build PASS96.506146s. JSON33 remains30 PASS/3 own cumulative WIP reds
(the same classified borrowed-declaration, field root-exit and construction
Fault expectation cases); leaf10/handoff11 PASS. Actual source test running.
No field support may disappear at JSON: a nonempty retained support census now
explicitly refuses field-support-transport-unconnected until the complete wire/C
connection exists (post-V765b source edit, not yet built).

V766 C construction: ordinary Handle/Nullable Return result_origin is now a
required schema column. Existing object-result index checks exact source-issued
producer, Invoke/Normal landing, ordered complete Copy chain and final Return,
then existing class/lease laws; all function Return rows are mandatory. No
missing-column fallback. C build PASS6.440517s; focused original16 source-issued
owned-result execution and new missing/foreign/extra-proof negatives running.
Synthetic physical Copy insertion with unchanged source correspondence is now
an explicit negative; the existing original received-result fixtures retain the
positive Copy cases. Read-only review found no coordinate/census admission hole.
Mixed/Forward and nonempty source field-support wire transport remain closed.

V766 original owned-result C execution PASS126.785509s: all16 source-issued
typed/opaque, Direct/Received, Handle/Nullable and both-opts fixtures, every
selected allocation/cleanup Fault, existing role/class/lease negatives and new
missing/foreign/extra original-correspondence negatives. FFI SHA
b8abd0f9541f5652fdef889b38aa2c9a563463b819fe1c65942895e4706d8691;
runtime757 reused (6897c4dc...). This accepts the bounded ordinary Return
correspondence C connection only, not Mixed/Forward/field-support transport.

V766 fresh fieldReturn6 PASS36.813159s and storedChild96 PASS16.122369s
(including original negatives/Faults); source reissued by V765b existing tests,
field2/stored2 all PASS. These are bounded regression evidence, not app EXE.

V767 BoxShape construction: current internal ordinary Handle/Nullable Call
signatures carry outcome beside payload; all caller slots and arguments join
that same signature. Only original InvokeNormalResult reads/guards Null0/Fresh1.
Return uses the same mandatory result_origin (Null/Fresh/OwnedCall companion).
Existing original Copy-root projection lends that companion to Birth actuals,
Provided stores and nullable cleanup; ordinary borrowed formals, receivers,
field borrows and Arrays gain no fabricated outcome. External/root/runtime ABI,
scalar/Unit/Map, source admission and Forward2 refusal remain unchanged.
V767b C build PASS6.537870s (V767 first build6.480848s superseded before tests).
Owned16/Provided3/Unit7 execution pending. Rust post-V765b changes also add the
independent retained-wrapper inner Call/Birth/formal mutation cases and explicit
nonempty-support JSON refusal; rebuild is queued after V765b test binary users
finish. Do not overwrite that still-running binary.

V765b actual-source test PASS1073.67s, both opts, 16MiB stack. The restored
independent compiled-entry consumer and new owned Return/field-support source
retention both pass; binary14ac9cc1163b617525b0b3620927a1bd3a415fd6776f94af3bb8838dd15705db.
Its retained-role model precedes the post-build JSON refusal/test-only mutation
additions. V768 rebuild now covers those additions. Do not merge these manifests.

V767b Owned16/all selected Faults and original-correspondence negatives
PASS127.886835s; Provided3 + nullable-wire/Fault/mutations PASS28.053694s;
Unit7/Fault/mutations PASS47.261806s. FFI SHA
453e5de585accec3a49583339de31f45af3707945ae024b0f3d11f1a8f71f50c;
runtime757 reused. Field/stored regressions still running. These preserve the
closed Null/Fresh cohort; source Mixed/Forward and field-support transport owed.


V768 build PASS64.169731s; focused raw9/parity6/BindingSSA17/repair4 PASS.
Actual source/strict whole-consumer test PASS1040.24s, both opts, 16MiB stack,
including retained uncalled-wrapper inner Call, outer Birth and formal mutation
refusal. Nonempty source field-support JSON still refuses explicitly.

V767b fieldReturn6 PASS34.933173s, storedChild96 PASS16.073704s.
Read-only audit found an own regression: Array/script-arguments conditional
release entered the new object-outcome helper. V767c restores their existing
separate indexed Array release arm. Exact same backend inputs: V766 accepts,
V767b refuses module-write, V767c accepts (both Array and script arguments).
No new Array outcome or retry. V767c build PASS6.531686s, FFI SHA
7bdbd3974b49ea84414efde5f7536fd39a24a142f28f7462ac9288b4674f00b1.
Provided3/Fault/mutations PASS25.923082s; existing argv4 source cases plus two
explicit physical-only Array peer probes PASS23.752541s. Initial probe harness
RED25.977710s was own temporary-directory indentation error, corrected before
that PASS. These physical mutations do not claim new Rust source admission.

V769 unaccepted BoxShape: normalize retained field support to one physical
dependency group containing ALL ordered original witness loans. Same relation
issuer, outer formal/ordinal, entry, Copies, Invoke/Normal, inner callee ordinal
and optional Birth must agree; only adjacent relay rows merge. Original source
census flattens all loans, never just representatives. The existing actual
wrapper case demands one dependency retaining its two distinct witnesses.
Read-only responsibility review agrees: C must verify complete physical Return
law, formal substitution and field/read/End anchors; it does not need a second
Rust witness-lineage registry. Existing source authority and all closed Mixed/
Forward/field-transport gates remain. Build/focused validation pending.

V769 build PASS68.142233s; read-only review found no source-order/identity or
callsite type mismatch. The existing both-opts actual test is running with the
one-group/two-witness assertion. No source gate has opened.
V770 separate BoxShape centralizes existing structural ordinary-result/callee-
role mapping for schema and indexed flow; Handle/Nullable result predicates move
from emitter-only scope to the same early transport owner. Unknown results
explicitly refuse; source/class/lease/conditional End permissions are unchanged.
C build PASS6.536299s; original owned16/Unit7/argv4+Array-peer regressions running.

V770 owned16/Fault/original-correspondence negatives PASS131.908847s;
Unit7/Fault/mutations PASS47.875639s; argv4/Array-peer PASS27.984978s.
FFI09ce7b9ed12118229263f556a324cafebbd0db2d4c18fa86b78c0a8ca80800bc;
runtime757 unchanged. Read-only review confirms null-safe unknown refusal,
correct include order and no old predicate callers. No source admission changed.
BorrowedNullable is a distinct required non-owning consumer responsibility; its
real current positive is the unchanged resizeInPlace eight-Return projection.
The similarly named lexical nullable test is owned Null/Fresh, not that proof.
Do not turn extracted/rewritten app JSON into source-positive evidence. Full
app C acceptance still needs the same Mixed, field dependency and End connection.

V771 unaccepted consumer construction: existing result-coordinate owner now
shares the exact complete ordered Copy-chain checker between owned Returns and
the fixed-formal borrowed Return law. The new borrowed child checks logical
params[ordinal], exact nullable-source Null producer or SAME formal, all source-
issued Copies and final Return. Every Return participates; Fresh/another formal/
Call inference is refused for this source cohort (resizeInPlace seven Null,
one formal). Build PASS6.507162s; owned16 regression running. Worker review found
no correspondence/census issue. C function/call ingress and Rust JSON remain
closed until dedicated non-owning result kind, original actual anchor, entry
tag/domain guard, Normal-only0/2 output and no-Home flow all connect. This is
construction inside the same required series, not Borrowed C acceptance.

V771 owned16/all selected Faults/original-correspondence negatives
PASS125.699372s, FFI15837b92e72fb3e4edbd0abe55f7bf1603d124677c08e89644117e0c6bd0cc7c.
V772–774 unaccepted borrowed consumer construction: dedicated non-owning kind,
exact Call fixed ordinal→actual→original Copy-root anchor, Normal/Copy/null
comparison readability, no LIVE result slot, all other Homes ended on Return.
Worker caught a missing independent Normal execution proof after removing LIVE:
require SAME Invoke Normal edge and exactly one Normal predecessor (fixed).
Output carries exactly Null0/Forward2 on Normal only; original fixed-formal
Return guards tag/payload before output and entry guards0/3 before effects.
Known object_view retains the existing class guard; unknown class grants none.
Current source law remains Null/formal only, not a new borrowed Call relay cohort.
V772 build PASS7.249559s; V773 PASS6.613938s; V774 PASS6.532727s. Latest original
owned16 regression running. Function/Call schema, parameter profile and Rust
JSON admission still closed: connect required ordinal transport, source-issued
positive and independent anchor/Normal/ownership negatives before opening.
C owner contract moved from the near-limit shims README to published_mir/README;
parent retains its link. No new phase/card/registry or product acceptance.

V769 actual-source test PASS1074.29s, both opts,16MiB stack: one physical group
retains both original Forward witnesses, strict compiled-entry and retained-
wrapper independent Call/Birth/formal mutation refusals all pass. Its source
manifest predates V770+ C work and V775+ wire edits; keep identities separate.
V774 owned16/Fault/correspondence regressions PASS126.181510s,
FFI1371236715738f6c8eed8237b5f423d00a97d455d98abd8c8cf77b9fcf50dbf2.

V775–776 unaccepted connection now adds borrowed-only required8-key function
and4-key Call schemas, fixed logical ordinal range/carrier and exact callee
agreement; native refuses this object cohort. Rust JSON emits the same ordinal
and corroborates exact callee result law. Mixed/completeEnd/nonempty field
transport remain explicitly refused. C775 build PASS6.597810s; static borrowed
caller classification subsequently aligned with its admitted nullable profile.
Worker found an omitted outer Rust operation match arm before build; added the
same Borrowed result to the existing ordinary Call encoder arm.
A new independent source/C boundary test emits original tiny object/null and
both-opts forward-formal programs through the existing canonical harness; no
app source or emitted JSON is repaired. The existing C call-result test owner
adds a borrowed mode with exact original Home counts, allocation/cleanup Faults,
mandatory ordinal/Return mutations and borrowed-release refusal. New test function
is justified by the previously untested non-owning final C boundary, rather than
another owned-nullable proof. V776 source/FFI build and new positives are pending;
this code is construction, not accepted Borrowed transport or full app progress.

V776 root build PASS64.025555s, FFI PASS6.573929s. New tiny source test
RED0.03s at artifact-source-unavailable/TerminalHomesUnavailable: direct Root
reception from an owned local is outside the proven formal-entry cohort. Own
unaccepted fixture boundary, not baseline and not app success. Worker confirmed
using observe(p:Item):i64 to call forward(p), null-test locally, return scalar
aligns the actual realloc→resizeInPlace formal-entry responsibility without
widening outgoing TypedHome or escaping the borrowed result. Main still owns
its Item across observe. V777 makes that explicit fixture scope; no compiler
source authority is widened to force the fixture through. Earlier direct Root
Borrowed reception remains unproved. Existing C test uses the same source-issued
observe body for ownership-negative mutation; full app/Mixed/field claims stay
separate. V777 build/positive/C execution pending.

V777 build PASS40.472160s; tiny formal-entry positive still RED0.03s at the same
root TerminalHomesUnavailable. Stop speculative fixture changes; read-only
source-owner audit identifies exact root pending-return versus all-exits-ready
branch. This token describes root observation, not necessarily the observe body.
JSON33 remains30 PASS/3 own cumulative WIP reds: borrowed declaration missing
retained Call, field root-exit phase, Array construction-Fault expected releases.
Existing borrowed_local_call_results_publish_grounded_chain_in_both_orders also
RED0.02s at root-exit-source-missing, before JSON. Classify as own unresolved
cumulative source WIP; no exact-baseline claim and no transport module PASS.
The new tiny boundary/C borrowed mode is owned by this same series, not a new
app/proof task. Retain while its exact source responsibility is diagnosed;
retire or consolidate if the original full-app canonical C execution supplies
all the same non-owning boundaries. Never keep it as an accepted substitute for
the original eight-Return/app or full Mixed field/End connection.
C776 original owned16 regression is running against the completed schema build;
new Borrowed runtime positives/negatives cannot run until source publication
succeeds. Latest app measurement remains CLI138 rc1/no EXE. No product stop
movement or Borrowed execution success is implied by the source/C builds.

C776 original owned16/all selected allocation+cleanup Faults/correspondence
negatives PASS116.384666s, FFI
ff9816b7e033a6076f6e5cbb3878e79f43bc5679a098752a490bcdf8930cf576,
runtime757 unchanged. Script SHA74d2aaec... includes the then-unrun borrowed
mode; its owned branch is unchanged. Do not call that borrowed execution.

Decision after bounded read-only V777 diagnosis: root owner(1,4) pending object
Return=0, all_exits_ready=0, exact source error ReturnValueNotCovered([Body(2)]),
`return recv.observe(item)`. Same before and after finishing/artifact recheck.
This is a distinct root scalar-call Return boundary, not borrowed0/2 C failure.
Retire ONLY the newly introduced unaccepted tiny fixture/test module and its
unused C borrowed mode now; do not create a Root repair lane or another fixture
workaround. Existing historical tests and original8/app tests remain intact.
Scratch source/C mode retained under this series owner as
v775-unaccepted-borrowed-microfixture.rs and its -c-mode.py companion;
reason=preserve exact failed
experiment for the already-owed Root/fixed-suite analysis, retire_when=that
source boundary is reconciled or its exact input is retained by its real owner.
These are failed construction evidence, never acceptance receipts. Source8 and
unchanged full app remain the source authority for completing borrowed/Mixed/
field/complete-End transport together. No optional microprobe is a new prerequisite.
Next: wire the same grouped field dependencies and Mixed conditional result
through JSON/indexed C substitution/read/End, then canonical unchanged app EXE.
No fallback, Root workaround, incoming permission from publication or new witness
registry. Current source/C WIP remains unlanded; root test binary V777 still
contains the retired unaccepted test, so future filtered evidence must name its
exact manifest. Production Rust wire code is built, but no new Borrowed C
positive or app success has been established. Only test removal/comment/owner-doc
changes follow V777/C776; no source-admission implementation changed afterward.


V778–779 unaccepted field-dependency consumer construction: Rust JSON now
borrows each group's SAME canonical field/child, constructor ordinal, outer
formal/entry, original Copies, inner Call Invoke/Normal/ordinal and optional
Birth coordinate. Three ordinal domains remain distinct; no witness IDs or
second source registry. C corroborates exact Provided constructor store and
substitution, preserves all callee Return dependencies on relay, and requires
each matching Fresh Return to retain every Mixed Birth dependency. Field reads
follow the exact parent wrapper's dependency closure; bounded recursion checks
the original actual's lifetime at read/End. Mixed local storage remains refused.
Read-only review caught per-Return census and nested parent lifetime omissions;
both fixed before V780. V778 owned16 PASS113.911744s; V779 owned16/Fault/original
correspondence negatives PASS114.032265s, FFI0190045a71dceb1ec7a1a3adb0e48738432af07289ec9839fec11d1e58a4e707.

V780 unaccepted Mixed construction reuses the existing acquired Return origin
checker plus the same original BorrowedCall actual-anchor resolver. All Returns
participate; OUTER fixed formal is checked after INNER actual substitution.
Owned class describes only acquired alternatives, never Forward. Dedicated value
kind/state260 stay outside LIVE/Home/Map/Birth domains. Copy, null compare and
Normal projection retain support readability; ordinary Home/reclaim refuse it.
C build PASS5.984681s; existing owned16/Fault/correspondence regressions
PASS114.394910s, FFI836a538ca2159b29155acc2e810c539bcd2110d7ed95395352f1237994e1f423.

V781 connects the existing Mixed Return/Birth/complete-End envelope as WIP:
mandatory fixed-formal schema and guard, Normal-only payload/outcome0/1/2,
BorrowedCall0/2 result relay, Provided residence companion, Normal-only260 move
into Birth with Fault retention. ObjectEndIfOwned carries its original complete
End steps; C corroborates every marked layout field in reverse order and outer
release, then runs the existing complete End helper only for outcome1. Null0 and
Forward2 release no children or outer storage. The old outer-only
home_release_if_live is not generalized. Generic ownership arms stay closed to
Mixed. Read-only worker verified the joined responsibility and fixed an omitted
Mixed role in the existing tagged-formal membership check; membership and fixed
Forward ordinal remain separate. C build PASS6.023752s; owned regression and
CLI139 build running. Rust ABI/JSON gates are now connected in source, not yet
execution-accepted. No Borrowed/Mixed C positive or unchanged app success is
claimed. Runtime757 is unchanged; source/C WIP remains unlanded. Next canonical
unchanged app attempt must identify the new actual first-stop before more repair.


V782 root build PASS83.933306s, binary53b2f8e31ce2364e26285b043aeb87339a200c320eb20685c44bf74d377635ec.
JSON35:32 PASS/3 own cumulative-WIP reds0.793s, same three named boundaries
above (borrowed declaration, ordinary field root-exit, Array construction Fault
cleanup expectation); not whole-module PASS or an exact clean-HEAD baseline.
C781 owned16/Fault/correspondence PASS200.842982s concurrent with CLI build;
fieldReturn6 PASS34.914811s; stored-child96 PASS13.518258s. These existing source
fixtures establish peer regressions only, not Borrowed/Mixed runtime acceptance.
Pointer, diff, no-fallback, builder-layer and thin-shim guards PASS.

Decision after CLI139 first-stop read-only audit: the existing whole-Box integer
store census already proves the free_stack Get/index. Home local flow records
TrivialLocal(Integer), but SelectedNew argument conversion loses the scalar kind;
more fundamentally, the existing checked FieldArrayReadRecipe is restricted to
Scalar Return source loans and does not reach this Object Return body's local
initializer. Do not stamp Unknown as Integer at ABI/argument use. Normalize the
existing checked read body away from its Return-use relation, retain the SAME
source read Rc for original initializer/binding/Birth actual, and route the
actual Get through the existing sole checked writer, LocalSSA/finishing/named-
array validation and both final consumers. Existing local Array connection is a
prerequisite to this same app/Forward end-to-end acceptance, not a new feature,
Root fixture lane or blanket dynamic scalar admission. Complete Mixed evidence
remains open while this exact production argument is connected.
