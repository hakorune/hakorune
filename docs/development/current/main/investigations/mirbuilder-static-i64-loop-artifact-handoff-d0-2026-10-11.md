# MIRBUILDER-STATIC-I64-LOOP-ARTIFACT-HANDOFF-D0

Status: coverage S0 unaccepted; scalar correspondence construction verified; physical activation pending
Date: 2026-10-12
Scope: selected Static LoopV2 handoff; unpaid coverage acceptance and source scalar operation correspondence
Related: `mirbuilder-main-static-terminal-ingress-d0-2026-10-11.md`, `mirbuilder-root-exit-static-two-exit-s0-2026-10-11.md`

Original artifact row: `MIRBUILDER-STATIC-I64-LOOP-ARTIFACT-HANDOFF-S0`.
Current row: `MIRBUILDER-STATIC-I64-SCALAR-PHYSICAL-PUBLICATION-S0`.

## Decision

Existing source/publication batch and body scalar packet own these calls.
Their checked physical Invoke, Normal result projection and Fault Return must
be retained in the existing selected body packet and lent to local_commit's
pending and collected lifecycle bindings. No new semantic authority/receipt.
Read-only worker static_two_exit_closeout_review independently confirmed the
missing header retention and body Invoke-only projection.

Source authority + canonical issuer: original LoopV2 semantic source calls,
selected publication batch and existing body prepacket. Canonical SSA/CFG
remains the sole physical writer. Artifact ownership consumes their exact
bindings, not a scan-derived whitelist.

Non-authority: function names, expected bins, backend selection, arbitrary MIR
shape, new source annotations or skipping requires_lifecycle_validation.

Fail-fast boundary: exact source/site/target, I64 result, Invoke/Normal linkage,
Fault landing/frame and unique projection. Preflight before collection;
retention commits only after successful collection. No partial publication,
retry or validator relaxation. Entry Invoke/Normal bindings remain in their existing group; retain its missing
Fault Return there. Tail stays with its existing root exit; do not duplicate it.

## Bounded implementation and acceptance

1. Complete entry group with its direct Fault Return, checked against original
   Invoke landing/frame. Independent follow-up corrects the earlier claim that
   entry was completely owned: only its Invoke/Normal pair was retained.
   Retain body Invoke + its checked Normal projection + matching ReturnFault
   in the existing detached packet. Replace its Invoke-only lifecycle
   projection; keep original Invoke accessor needed by other consumers.
2. Existing zero-argument publication check lends header's exact three
   bindings after checking header and tail. Attach header to that same packet
   before pending validation. Tail stays with its existing root exit.
3. Both pending and retained lifecycle collectors consume the packet's full
   bindings. No independent ledger, second writer or general admission change.
4. Extend existing selected pending tests: normal projection missing/foreign
   ValueId, Fault frame/landing drift, extra lifecycle operation. Check
   retained coverage and no retention before collection. Reuse original
   source/publication negatives and body packet tests.
5. Fresh CLI against unchanged SizeClass source + Main inputs0/9/4194305:
   require native EXE results1/2/73, or classify the exact next first-stop
   without claiming this acceptance. Record source revision/binary/command
   and separate build/link/run timings. Pointer/publication guards and diff
   check required; existing qualified-route stale pin remains baseline debt.

Selected old responsibility: body Invoke-only lifecycle projection and
header check whose verified bindings are discarded. Replace those projections
in the same slice; retain shared artifact validator and other callers.

## Non-claims and return

Main terminal S0, parent Static two-exit S0, JSON transport and final retirement
review remain unpaid. This row closes only its physical ownership handoff and
required positive/negative checks. Resume Main acceptance afterward. Whole
SizeClassBox, mimalloc-lite and MirBuilder completion are not claimed.

## Implementation checkpoint (acceptance pending)

Existing body packet now retains its Invoke/Normal/Fault triple. Its header
retention API borrows the original publication batch and the same detached
function, rechecks batch brand/caller/body site/target and body physical shape,
and checks header/tail before adding the header triple. Raw instruction append
was rejected by independent review and removed; no semantic receipt was added.
Both pending/collected collectors borrow the resulting six bindings; the
original Invoke accessor remains for the existing producer-coordinate owner.

Existing pending mutation test now checks artifact coverage directly through
the original validator: six bindings, missing/foreign Normal/Fault/Invoke and
extra projection are rejected without publishing retention. Positive native
results and retained acceptance remain unverified.

V1 build was interrupted after review found the raw API issue (no test result).
V2 typecheck failed E0603/private module and E0369/brand comparison; use a narrow
existing type reexport and allocation `is_same`. V3 duplicate reexport E0252
was a current-change compile error; build interrupted after diagnosis. All
interrupted cargo/rustc processes were confirmed terminated before Rust edits.
V4 focused output: `/tmp/static-loop-artifact-handoff-focused-v4.log`.
These attempts provide no acceptance evidence.

V4 compiled in8m07s but focused test failed before the new mutation matrix at
publication-zeroarg-physical-drift. Current-change error: header's direct
ReturnFault requirement was accidentally applied to tail, whose existing root
graph owns a cleanup route. Keep tail's original Invoke/Normal check and lend
Fault bindings only for header. Fix applied; V5 also covers foreign Normal
invoke linkage and Fault landing directly. No acceptance is claimed from V4.

V5 interrupted before test execution after finite ownership review confirmed
entry ReturnFault was also unretained. Independent worker corrected its earlier
entry-complete claim. Existing entry packet/group now owns this third binding,
with original landing/frame corroboration; header/body packet stays six bindings.
Touched scope adds the existing entry lexical projection owner because it owns
that missing group projection. No new owner or accepted grammar is selected.

V6 interrupted before test execution to include entry in the same existing
mutation matrix: nine entry/header/body bindings, missing/foreign results,
Fault frames/landings and extra projection. All processes terminated before
Rust edits. V7 output: `/tmp/static-loop-artifact-handoff-focused-v7.log`.
No test result is claimed from either interrupted build.

V7 compiled8m08s, focused test0.02s FAILED at physical-boundary/phi-in-recorded-block
before the mutation matrix. Current-change incomplete ownership/policy join,
not baseline debt. Follow-up static review distinguishes true two-predecessor
header from its single-predecessor Normal landing, which may carry canonical
SSA forwarding PHIs. No broad PHI waiver is authorized. First identify the
actual rejecting block, then borrow its existing checked source-call Normal
role into existing PHI policy with leading-prefix/input/sequence checks kept.
A diagnostic-only change to physical_boundary_phi_policy includes block ID
and instructions in the existing named reject; no acceptance rule changes.
Fresh CLI probe will identify the exact first-stop before selecting this join.
Two accidental recursive rustfmt child edits were canonically equal to HEAD
and restored after confirming Cargo/rustc terminated.

Fresh diagnostic CLI build finished in3m52s. Receipt:
`/tmp/static-loop-artifact-diagnostic-cli-receipt.json` records HEAD+Rust patch,
source/binary/runtime hashes, commands and timings. Trivial EXE returns0; all
three original SizeClass probes stop at bb39, with two leading PHIs from bb36
followed by InvokeNormalResult(bb36) and Compare. Compile0.027/0.027/0.028s;
no selected artifact exists. This is the checked header Normal landing, not
header's true join. Existing source/publication packet must lend this exact
Normal PHI prefix into physical boundary capture; no general single-pred waiver.

Decision after independent read-only review: lend the exact checked header
Normal leading-PHI prefix from the existing packet into the existing prefix
role map (renamed source_phi_roles). Issuance checks original Normal has only
header as predecessor and each prefix input comes from header. Merge rejects
duplicate roles; capture compares the sealed prefix even if an extra edge
would turn it into a generic join. Original full sequence/edge and DAG checks
remain. Existing pending matrix adds missing/foreign/nonleading PHI cases;
existing physical prefix tests add extra-predecessor and finished-drift cases.
No new authority, semantic receipt, writer or generic PHI permission.

V8 build interrupted before test execution during final negative audit:
empty observed PHI prefix must not bypass an existing sealed nonempty role.
Early-return now applies only to blocks without a source-prefix role; add
all-PHIs-missing regression. Cargo/rustc termination confirmed before edits.
V9 focused log: `/tmp/static-loop-artifact-handoff-focused-v9.log`.

V9 interrupted before test execution after follow-up review found that
LoopCond after issuer does not restrict incoming count. Correct the generalized
loan to existing prefix plus optional exact predecessor: after roles keep None
and legitimate multi-input joins; header Normal lends Some(original header),
including empty-prefix cases. Prefix comparison precedes generic-join admission.
Add existing-after two-input positive; fix real-fixture full-prefix deletion case.
No V9 acceptance. V10 log `/tmp/static-loop-artifact-handoff-focused-v10.log`.

V10 found current-change E0412: emission_prepare parent does not import
BTreeMap. Use the owner's existing fully-qualified std::collections form.
Interrupted after explicit diagnostic; cargo/rustc terminal before edits.
V11 log `/tmp/static-loop-artifact-handoff-focused-v11.log`.

Final read-only review accepts the prefix+optional predecessor mapping;
no new specific safety hole found. V11 remains live. Existing wrong_input
negative expects phi-in-recorded-block; current prefix-first order instead
returns loop-after-phi-prefix-drift. After receipt, preserve the original
sole-predecessor/input structural diagnostic before prefix comparison, while
keeping prefix comparison before the generic-join success return. Do not
change Rust while the current Cargo/rustc process is live.

Independent read-only work while V11 builds identifies a candidate for owed
Main sibling negative after returning to Main task4: unchanged SizeClass source
plus instance Heap.lookup(size) calling size_to_bin(size), and Main creates
Heap, invokes lookup(7), then returns size_to_bin(9). The real Main->Heap
incoming should avoid NoIncoming; Heap's real forwarded sibling is not a
static omission. First prove package/cohort positive, then remove only Heap's
actual row and require cohort-unfinished while Main remains finished, restore
and require packet again. Static candidate only: not tested or accepted yet.

V11 finished8m09s; original selected Loop preflight1/1 PASS (0.04s), including
nine binding mutations and Normal PHI negatives. Same binary Main7/7 PASS
(0.04s). Physical boundary real module16/17 (0.01s): sole failure is the
expected diagnostic-order regression at wrong_input, not acceptance. First
wrong filter ran0 tests and was discarded. Restore structural single-pred/input
reject before sealed prefix comparison, preserving prefix-before-join checks.
Remove temporary block/instruction diagnostic now its purpose is complete;
retain the original named reject. Add empty-PHI Normal topology positive/negative.
No Cargo/rustc live before edits. V12 log:
`/tmp/static-loop-artifact-handoff-focused-v12.log`. Native results still owed.

V12 completed8m11s, focused selected preflight1/1 PASS (0.04s). Same binary:
physical_boundary module18/18 (0.01s), Main7/7 (0.04s), canonical root writer3/3
(0.00s), all PASS. Preserved binary `/tmp/static-loop-artifact-v12-test-binary`.
Only the new emission_prepare region is formatted after receipt; rustfmt
canonical before/after equality proves no semantic change. Other old format
drift is preserved. Owner README combines the new ownership/PHI contract in
its existing selected-loop paragraph. Source boundary782 lines receives only
renames; before further growth its existing >760 responsibility needs splitting.
Fresh CLI build log `/tmp/static-loop-artifact-handoff-cli-v12-build.log`;
required native acceptance remains pending.

Read-only closeout review confirms selected delete-set: session replaces the
header-check/discard call with same-packet retention; root_validation replaces
both pending and retained body Invoke-only projections; existing entry group
adds its original Fault Return. Old direct header-check caller now exists only
inside the packet owner, retain_header_bindings has one session caller, and
old body map(original_invoke) projections are absent. Shared Invoke accessors
remain coordinate readers, tail/root/artifact validator remain shared. This is
selected responsibility caller-zero, not whole-owner/file retirement. Review
found no blocking source issue; native receipt and row closeout remain pending.

Fresh CLI V12 finished3m50s. Receipt:
`/tmp/static-loop-artifact-verified-cli-receipt.json` records revision+Rust diff,
source/binary/runtime hashes and original commands. Trivial EXE rc0; selected
inputs0/9/4194305 clear artifact/PHI refusal, then all stop before artifact at
`mir/invoke/ordinary-new/emission/selected-static-packet/canonical-entry-required`.
Compile0.031/0.037/0.033s; no selected EXEs. Required bins1/2/73 are not proven.
This row's implementation/negative/caller-zero checkpoint is verified, but
its native acceptance remains open. Main/parent/full retirement remain owed.

Next bounded design (independent read-only review): final verifier's
original_call_slots visits a finalized lexical packet, then its existing source
accessor delegates SelectedStaticLoop to the generic ordinary actual getter,
which intentionally rejects it. Canonical emitter already uses the selected
packet's checked forwarded actual. Extend only the finalized source accessor
to lend that same packet actual with matching source observation and shared
borrow lifetime. Preserve generic raw-entry refusal, finished producer/cleanup
checks, whole incoming/slot coverage and physical publication checks. No new
receipt, emitter, retry or grammar expansion. This accessor is shared by final
verifier and compiled_entry_contract physical publication. Implementation is
not in this artifact-retention checkpoint; select the next row before editing.
Tests: finalized packet exact actual positive, foreign source/producer refusal,
generic direct getter still canonical-entry-required; unchanged native probes.

V12 binary SHA256: `2aceb78501285662128f87244e20e61c238cbab79fddf78fcaebad0954e620c5`; original source: `ac6513ccd595a664bf7bd4a47402baff1377152a93ef2a059566826b13d642b0`.

## Selected follow-up: finalized actual loan

Current selected row: `MIRBUILDER-STATIC-I64-LOOP-FINALIZED-ACTUAL-LOAN-S0`.
Artifact-retention checkpoint `ea99e9bfbb` is pushed; its native acceptance
remains unpaid and is inherited by this follow-up, not silently closed.

Decision: existing FinalizedRootSourceHandoffV1 lends the typed selected Loop
packet's already checked forwarded actual to final verification and physical
publication. Source authority/canonical issuer remain the original selected
packet, publication and whole incoming witness. Existing finalized visitor
checks cleanup, canonical physical source read and exact finished producer
before lending. No re-emission or new authority.

Selected old responsibility: this accessor's SelectedStaticLoop delegation
to generic borrowed_actuals, which refuses canonical packets. Keep generic
raw-entry refusal and its Instance/Static consumers unchanged. No owner/file
retirement is authorized.

Implementation: match typed SelectedStaticLoop, corroborate original local
observation, use publication_source caller/site to borrow existing checked
forwarded actual, return a one-element slice. Reborrow the exact retained packet from the existing local pool; tie output
to self and reject foreign ledger observations or pointer identity. Preserve original_call_slots incoming/slot checks and compiled
physical publication's producer and snapshot checks.

Acceptance: real finalized selected packet lends exact actual; foreign
source/producer rejects; generic getter remains canonical-entry-required.
Use existing selected Loop tests/fixtures and final module verification.
Record fresh unchanged-target native inputs0/9/4194305 with expected1/2/73.
Main's sibling negative, terminal cutover/final retirement review and parent
JSON acceptance remain separate debts. Source/contract/physical gaps stay
internal design work, not external wait.

Follow-up implementation checkpoint: finalized accessor now distinguishes
typed SelectedStaticLoop, corroborates its observation, borrows the packet's
checked forwarded actual using original publication coordinates, and returns
a one-element slice. V1 attempted a shared self/packet lifetime; the checked
retained-pool reborrow below supersedes it. Generic getter unchanged.
Owner-local child test uses unchanged SizeClass source plus original Main;
requires final visitor's exact witness identity and generic rejection, then
mutates final Invoke actual/target and requires source/producer refusal.
Test compilation first, before heavy codegen; no acceptance yet.

V1 follow-up check failed with current-change E0521 (callback packet borrow
escaped into physical publication's incoming map) and E0277 (test target was
constructed from a string instead of a canonical key). No test acceptance.
Read-only review confirms SelectedStaticLoop is constructed only for the local
entry group; terminal admission still rejects it. Reborrow the exact packet
from this handoff's existing local_calls pool, verify original observation in
this ledger and unique retained pointer identity, and lend its original actual
with self lifetime. No new pool/index, owned witness copy or terminal expansion.
V2 check log: `/tmp/static-loop-finalized-actual-check-v2.log`.

V2 lib+tests check PASS,1m10s (existing warnings retained). The getter now
lends from the handoff's original local pool rather than the temporary visitor
loan. Independent source compile negative exercises cross-handoff refusal;
exact witness identity and final target/argument mutations remain covered.
Unrelated original formatting is restored. Focused V3 log:
`/tmp/static-loop-finalized-actual-focused-v3.log` (not yet accepted).

V3 focused build8m10s, execution0.02s:1 FAILED before the callback at
`mir/invoke/ordinary-new/borrowed-entry/entry-values-missing`. No finalized
loan positive, cross-handoff negative or native acceptance is claimed.
Same binary selected preflight1/1 (0.04s), physical boundary18/18 (0.01s),
Main7/7 (0.04s) PASS. This first-stop changes the next action; it is an
incomplete selected production chain, not a known baseline waiver.
Read-only review finds selected canonical formal adoption does not record
its entry correspondence through ordinary install_entry_values. Existing
source-only owner-view also lacks the finished Main tagged cohort branch;
its generic incoming check must not grant executable status to forwarded
SourceStatic siblings. Resolve the entry source/physical correspondence
mapping before adding a record or relaxing a verifier. Prior rows remain
unaccepted, and no native receipt is claimed from this Rust test run.

## Selected prerequisite: tagged entry correspondence D0

Current selected row: `MIRBUILDER-STATIC-I64-LOOP-TAGGED-ENTRY-CORRESPONDENCE-D0`.

Decision: keep the finalized actual-loan implementation unaccepted. Before
its positive can run, connect the selected canonical formal to the existing
entry owner. Resolve source-only tagged owner-view and entry correspondence
as one bounded responsibility; do not bypass SourceStatic executable refusal.
Source authority + canonical issuer: existing finished StaticTaggedInputFinishV1
and whole checked_tagged_cohort_v1, original incoming inventory/definition,
selected signature and canonical adopt_exact_formal_parameter.
Non-authority: Main name, expected bin, carrier metadata alone, omitted caller
counts, arbitrary ValueId or treating source-only actuals as executable.
Fail-fast boundary: whole incoming identity/coverage; source/formal/ordinal and
unique physical parameter; missing/foreign/unfinished/duplicate record rejects.

Smallest next slice, in dependency order:
1. Existing entry owner borrows a source-row view from the finished tagged
   cohort. Preserve whole incoming and source origin identity. Distinguish
   executable actual-slice loans from canonical actuals still owed at packet
   consumers; never return an empty-success stand-in. Retained forwarded
   siblings remain SourceStatic and are validated by the existing issuer.
2. Entry source/physical correspondence uses that checked view. Final verifier
   and physical publication still demand each actual from its finalized packet
   and prove exact producer plus bidirectional incoming coverage. CurrentOwner
   incoming_targets membership must use the same checked source proof.
3. Canonical formal adoption prevalidates `(0, formal, actual ValueId)` through
   the existing ledger. Register once at the collector commit boundary after
   all fallible checks, before infallible collection; failed prepare/collector
   publishes no entry row. No new semantic receipt or metadata-derived proof.
4. Positive original source and negatives: foreign formal/ordinal, unfinished
   incoming, duplicate record, failure leaves no entry correspondence. Reuse
   the unaccepted finalized actual test for exact witness/cross-invocation/
   producer negatives and the three unchanged native outcomes.
5. Parent entry source770 lines: split source-target projection by its existing
   responsibility before growing it; preserve unrelated contract behavior and
   keep every resulting source under800. Separate behavior-preserving movement
   from the new acceptance branch in review/commit scope.

No native acceptance, terminal retirement, JSON or full goal completion is
claimed. This is internal design work; no external wait is identified. After
this prerequisite, return to the unaccepted finalized loan and Main closeout.

Entry mapping selected for construction:
`MIRBUILDER-STATIC-I64-LOOP-TAGGED-ENTRY-CORRESPONDENCE-S0`.
Use existing entry owner-view's typed distinction: executable actual loans vs
checked tagged source rows whose actuals remain owed at canonical packets.
The ordinary actual getter remains strict. First commit is only movement of
incoming_targets into the existing source owner-view child; no behavior change.
Subsequent behavior change remains dependent on focused positive/negative
and native acceptance. Source targets/mapping above are fixed before coding.

T0 landed/pushed at `8a926571f5`: incoming_targets moved byte-for-byte to the
existing entry owner-view child; parent770->727. Lib+tests check PASS1m10s.
This commit has no new acceptance or behavior branch. Unaccepted accessor
changes remain outside that commit.
S0 implementation now loans whole finished tagged source rows in the existing
owner view, marks canonical actuals explicitly unavailable through incoming(),
and keeps generic SourceStatic execution refusal. Final consumers still demand
original actuals from packets. Canonical formal tuple is prevalidated, then
recorded after fallible seal/batch checks and before infallible collection.
S0 V1 check PASS1m11s. V2 test check has current-change missing ValueId imports,
not a baseline error; import corrected. V3 log:
`/tmp/static-loop-tagged-entry-s0-check-v3.log`. Focused runtime evidence pending.
Tests extend existing pending/collector cases with entry nonpublication and
success correspondence/duplicate checks. Existing Main fixture adds canonical
actual-loan refusal, invalid formal/ordinal, and original retained Heap sibling
positive->missing-row veto->restore (must first prove the positive fixture).

S0 V3 lib+tests check PASS1m11s. Independent review confirms staged source
correspondence and per-packet executable actual checks remain separate.
V4 focused log `/tmp/static-loop-tagged-entry-s0-focused-v4.log` is live;
no runtime acceptance yet. Touched Rust sources remain under800; existing
source_actual_tests773 has only Result handling changes and no line growth.


V4 focused build8m10s, execution0.02s: finalized positive now clears
entry-values-missing and stops at borrowed-source-incoming-coverage before
callback. No native/finalized acceptance. Same binary entry preflight1/1,
physical18/18, existing entry tests18/18 and existing Main7/7 PASS. Initial
filename filter matched0: not acceptance; corrected owner filter above.
New Main sibling test fails during source issue at selected-forwarded-actual-
unavailable: its instance caller cannot satisfy existing Static checked-input
closure. Preserve that fixture as refusal; use a source-declared Static sibling
with the existing compare/forward closure for the required positive->row-removal
negative. Do not extend instance or SourceStatic executable authority.
Read-only review identifies retained loop_body_calls lacking a finalized
visitor consumer, specifically size_to_bin->bin_size. Main terminal enumeration
already exists. Expand only existing coverage failure diagnostics into missing/
extra callee and site differences to identify the actual gap; coverage predicate
unchanged. This diagnostic path is added to scope because combined error alone
cannot discriminate a missing body packet from a missing Main terminal.
V5 rebuild is required for these two changes; all prior non-claims remain.


V5 build488.24s. Finalized positive still fails0.02s, now exact evidence:
missing-callees=[SizeClassBox.bin_size/1], extra-callees=[], missing-sites=[],
extra-sites=[]. This confirms the retained detached LoopBody packet is missing
from finalized borrowed-call enumeration; Main is not the missing callee.
Preflight1/1, entry18/18, physical18/18 and old Main7/7 remain PASS. New Main
sibling test's unsupported-instance refusal passes; supported sibling fixture
fails cardinality1!=2 because unused Heap.lookup is pruned by ClosedApp selection.
Use original Main's reachable Heap.lookup(7) caller; never accept a pruned row
or invent a retained inventory. Positive sibling evidence remains unpaid.

Current selected row: `MIRBUILDER-STATIC-I64-LOOP-FINALIZED-BORROWED-COVERAGE-D0`. Prior entry and
finalized actual-loan S0 remain unaccepted; all native/retirement/JSON debts carry.
Decision: connect existing retained LoopBody packets to the same finalized source
transport enumeration used by verifier and physical publication. Keep each
packet's existing source issuer/actual/finished producer; do not forge Lexical.
Source authority + canonical issuer: original StaticIncomingSourceV1,
VerifiedLoopStaticBodyScalarSourceV1, canonical binding-read receipt, detached
packet retained only after collection, existing finished binding projection.
Non-authority: callee name, carrier metadata, expected bin, arbitrary coordinates,
empty actuals, generic SourceStatic promotion or re-issued Main packets.
Fail-fast boundary: original owner/site/pool pointer identity, scalar/formal/ordinal,
exact final producer, complete incoming/coordinate coverage and cleanup unchanged.
Smallest next slice:
1. Existing finalized-source owner lends a typed Lexical/LoopBody transport view.
   Lexical reuses current visitor; body borrows exact loop_body_calls pool and
   finished_loop_body_call_producer_v1. Reject duplicate sites/coordinates.
2. Actuals reborrow from self-owned original pool, not callback lifetime. Body
   lends original scalar actual and original Static source; no new packet/receipt.
3. Replace both current lexical-only borrowed incoming enumeration callers:
   verification/invoke_borrowed_calls and compiled_entry_contract/
   borrowed_call_incoming. Keep both complete coverage checks; copy-use logic
   accepts only real Copy dependencies, never fake Copy for a canonical read.
4. Existing original-source finalized test proves exact body enumeration and
   mutation/foreign refusal plus both verification and physical publication.
   Finish reachable sibling positive/row-removal veto, then unchanged native
   inputs0/9/4194305. Selected entry and prior finalized acceptance remain owed.
5. After required receipts, selected old responsibility is these two lexical-only
   coverage callsites. Shared lexical visitor and other callers remain supported;
   no whole owner/file deletion permission. Mechanically verify scoped caller-zero.
Non-claims: no native result, Main terminal complete, JSON transport, full
SizeClass/mimalloc/MirBuilder retirement. This gap is internal, not external wait.
Independent read-only design confirms this mapping and warns to keep actual loans
at retained self lifetime and preserve Scalar read/finished Invoke correspondence.


D0 mapping accepted after V5 exact missing-callee evidence and independent owner/
consumer/lifetime review. Current selected construction row:
`MIRBUILDER-STATIC-I64-LOOP-FINALIZED-BORROWED-COVERAGE-S0`.
Only the transport/consumer connection above is selected. New reachable sibling
fixture has been edited but not rebuilt; keep it unpaid. All earlier entry/
finalized positive and native/JSON/retirement conditions carry into this series.


Coverage S0 implementation uses one existing-owner transport enum/visitor:
Lexical reuses the exact prior finalized visitor; LoopBody borrows the original
pool and finished_loop_body_call_producer_v1. Both consumers now call this
visitor and the self-retained actual getter. Original identity, scalar read/
Invoke correspondence, duplicate site/coordinate checks and both full coverage
predicates remain. No packet, source authority or Copy dependency is forged.
Existing finalized test adds exact body witness/foreign invocation/missing,
duplicate and changed producer refusal plus lifecycle compiled-entry publication.
Owner README owns this shared coverage contract. Independent read-only review
found no source/lifetime/coverage blocker; runtime positive still required.
Lib+tests check PASS1m10s, log /tmp/static-loop-finalized-coverage-s0-check-v1.log.
Focused V1 build live, logs/patch/hashes under
/tmp/static-loop-finalized-coverage-s0-v1-*. No runtime acceptance yet.


Coverage S0 V1 build489.267s, receipt/patch/source hashes saved at
/tmp/static-loop-finalized-coverage-s0-v1-receipt.json and -rust.patch.
Final verification now PASSES and callback reaches all lexical/body exact loans,
foreign-invocation refusals, body missing/duplicate/changed producer negatives and
lexical producer/target mutations. Final callback step fails at physical program
construction: published-lifecycle-program/instruction-unsupported (before the
physical incoming checker). Thus the aggregate test1 remains FAILED0.04s; do not
call both-consumer/native/S0 acceptance complete. This is current incomplete
physical publication, not a baseline waiver.
Main8/8 now PASS0.04s: reachable Static sibling positive, missing-row whole cohort
veto, duplicate/formal/ordinal negatives and unsupported instance refusal all run.
Entry18/18 (0.03s), selected preflight1/1 (0.04s), physical18/18 (0.01s) PASS.
Selected caller-zero static inspection: both coverage consumer callsites use the
common typed visitor; neither delegates coverage to lexical-only enumeration.
Shared lexical visitor remains inside that same source owner for other readers.
Physical program allowlist admits Add/Mul but not Sub/Div, both present in original
bin_size source. Need exact real module/instruction boundary and physical contract
review before changing it; broader admission or JSON waiver is not selected.
Fresh unchanged CLI/native first-stop probes selected next; no Rust edit during
build. Prior native/physical/JSON/retirement conditions remain unpaid.


## Fresh native evidence and next owner (2026-10-12)

CLI build completed in3m50s, quick/jobs4/incremental0, at5622cc8159 plus
uncommitted Rust patch abc0b763792c5a17a12bd1f7f429b59fd361514c084e3c1831a849cad544d3a7
and the two untracked source hashes in the V1 receipt above. Executable SHA256:
e7f7c3d7c8773a970f017681bb36920346e07ff5ef1819d985922231e087016d.
Original SizeClass source remains unchanged, SHA256:
ac6513ccd595a664bf7bd4a47402baff1377152a93ef2a059566826b13d642b0.

Two runtime selections are recorded separately, not as compiler regressions:
- target/release/libnyash_kernel.a is stale for selected lifecycle admission:
  all three actual inputs stop at checked Map symbol missing/duplicate,
  nyash.map.checked_array_length_v1. Trivial compatibility EXE exits0 in0.001s,
  compile/link0.731s; it does not exercise selected lifecycle publication.
- Existing target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a has the
  required symbol and descriptor; archive SHA256
  9f6fe986691e9e1b754b41b632d1453fe9bc6c627b21ddbd3728699723bc7dec.
  Actual inputs0/9/4194305 reach published-lifecycle-program/instruction-unsupported
  in1.991/2.000/2.026s. No artifact or native return code was obtained.
  Trivial compatibility EXE requires the generic archive and is not a lifecycle
  positive. Keep both archive results; do not rename an archive or relax inventory.

Commands: target/quick/hakorune --emit-exe /tmp/<case>.exe
--emit-exe-nyrt target/lifecycle-kernel/release /tmp/<case>.hako,
with NYASH_DISABLE_PLUGINS=1, HAKO_BACKEND_COMPILE_RECIPE=pure-first,
HAKO_BACKEND_COMPAT_REPLAY=none. Full receipts:
/tmp/static-loop-finalized-coverage-v1-cli-receipt.json and
/tmp/static-loop-finalized-coverage-v1-lifecycle-runtime-cli-receipt.json.
Canonical ClosedApp dump uses the SAME regular source and fresh binary with
--backend mir --dump-mir --mir-verbose --emit-exe and the runtime selector;
/tmp/static-loop-finalized-coverage-v1-dump.log. Dump is evidence of final MIR,
not physical publication or successful canonical JSON transport.

Dump resolves the finite arithmetic inventory in SizeClassBox.bin_size/1:
- bb15: %45 = %38 Div %40, %40 is original const4.
- bb15: %55 = %44 Sub %54, %54 is original const2.
- bb22: %117 = %109 Div %118, %118 is const4.
- bb22: %108 = %109 Sub %115, %115 is the original Mul result.
Both negative returns are Const(-1). Source has one Div and two Sub sites;
there are two emitted Div occurrences, so a one-source/one-instruction assumption
is invalid. The dump does not itself issue authority for any occurrence.

Decision: select internal scalar-operation correspondence D0 before extending
physical admission. Keep previous finalized coverage S0 UNACCEPTED: verifier and
original-loan negatives pass, but physical incoming admission and native outcomes
remain unpaid. Keep tagged entry, Main production closeout, selected old-edge
retirement and parent canonical MIR JSON transport obligations unchanged.

Source authority + canonical issuer: existing resolver expression_source binary,
operand/literal/local-origin sites and source scalar owner. Existing scalar
preflight admits Integer Sub and local-initializer-root Div with a positive
Integer literal divisor. That divisor excludes zero and MIN/-1; no general Div
Fault policy or exact-width/usize arithmetic is inferred. The preflight currently
returns kind/calls, not a retained operation proof. This missing connection is
internal implementation/design work, not an external dependency.

Non-authority: MirType, final const4, function name, expected native result,
old StaticV2 sdiv support, emitter whitelist, or a second occurrence matching by
shape. Do not infer source authority backward from the dump.

Fail-fast boundary: original owner/source/site/operator and ordered operands;
each original emitted occurrence; exact FinishedBindings projection; operand
class and positive-divisor correspondence; missing/foreign/duplicate/changed
operation rejection. No new semantic receipt, solver, fallback or source rewrite.

Smallest next design/implementation tasks:
1. Trace the actual source lowering paths for both Div occurrences. Confirm how
   the existing source binding origin is carried during local initializer reuse;
   do not assume the ordinary Binary completion hook covers both paths.
2. Lend the existing scalar source proof before ordered operand descent. Retain
   the original exact append with its source site in the existing ledger owner.
   Derived appends must borrow that SAME source proof through their physical
   rematerialization correspondence, never mint a second source materialization.
   Reuse CompletedOrdinaryBinary/FinishedBindings where their actual paths allow.
3. Fix the I64 Sub policy against existing arithmetic contracts before activation;
   do not invent checked overflow for Sub alone. Retain source-accepted total Div
   scope, with original positive divisor guaranteed through publication.
4. After mapping is fixed, connect Rust physical admission/JSON and existing C
   schema/index/type-flow/admission/emitter for those exact operations. Preserve
   Integer vs tagged/Bool distinction and all operand/use checks. Other operators,
   general Div, Float and exact-width extensions are separate capabilities.
5. Verify original source and source/physical mutation negatives, including each
   Div occurrence and changed divisor. Rerun the unaccepted finalized positive,
   then inputs0/9/4194305 must exit1/2/73 using the selected runtime. Only afterward
   close prior production/caller-zero/retirement obligations within their scopes.

Read-only owner review confirmed source preflight's positive literal restriction
and the missing retained correspondence. FinishedBindings maps each ORIGINAL
physical binding; it does not manufacture source1->multiple occurrences. Existing
ordinary Binary completion is a possible connection, currently unarmed. Task1
must resolve the actual route before implementation. This D0 is not S0 permission
and not NoSafeSlice; continue the finite internal mapping above.


Task1 focused static refinement: LocalSSA materialize.rs:451 directly emits
BinOp after recursive arg(lhs)/arg(rhs), and :500 follows Copy sources recursively.
It bypasses ordinary Binary completion. This path can explain the repeated
Div/operand chain in the dump; do not classify the second append as a second
resolver traversal merely from equal operators. The exact source->original
append loan and original->derived append relation still need implementation and
a focused execution test. Existing local_ssa_map is a reuse cache, post_success
is type/newbox metadata (non-Copy rematerialization excluded from exact-fact
publication), and propagate_record_local_value is RecordBox state. None is an
arithmetic lineage proof. BorrowedMulMaterialization supplies an original-source
retention example, but explicitly rejects a second materialization at the same
source site; do not register two source records to evade this boundary.

Task2 physical correspondence design is now constrained: sole LocalSSA owner
records original producer->derived producer and the actual ordered operand
rematerializations ONLY after successful emission. Failure and cache hit record
no new append; a cache use borrows its existing relation. The source proof stays
one per original site. FinishedBindings validates each original/derived binding,
while final admission requires complete operation/operand correspondence and
rejects inserted, missing, foreign or changed derived producers. The retained
source proof loan and Sub policy remain Task2/3 internal design requirements;
physical operation admission is still unauthorized. No external wait is needed.


Source-retention mapping accepted (read-only owner review): the SAME scalar
preflight returns its original Sub/positive-literal-Div initializer evidence only
after both Integer children and the complete field/call batch succeed. Retain
owner/statement/declaration binding, original binary child sites, Integer class
and original positive literal through existing RootHomeFlow->Completion->ledger.
Scan only transports it; no second classifier. Ordinary emission and LocalSSA
will later borrow that original row, not reconstruct it. Select
MIRBUILDER-STATIC-I64-SCALAR-SOURCE-RETENTION-S0 for this necessary connection.
Paths: existing scalar preflight + responsibility child for immutable row;
RootHomeFlow, prefix/branch scanner argument transport, existing source-local
claim tests. Acceptance: original bin_size has exactly2Sub+1Div source rows with
owner/binding/site and literal4; rejected/mixed/zero/negative-divisor roots retain
none; clones retain same Rc. Source retention does not change supported syntax,
physical admission, overflow policy, native or prior S0 acceptance. After this
connection, continue original->derived producer correspondence and full Sub/Div
publication in the same series; all prior debts remain unpaid.


Source-retention construction checkpoint: preflight responsibility extraction is
behavior-preserving commit de4d5f22e1 (original body unchanged); semantic retention
remains uncommitted and unaccepted. Same preflight now retains original local
statement/declaration correspondence only after complete Integer root admission.
Added source-owner tests cover original SizeClass 2Sub/1Div, rejected roots, and
both branch inventories with identical retained Rc loans. These are not physical
producer or native acceptance. Lib+tests V3 check PASS1m11s, log
/tmp/static-scalar-source-retention-check-v3.log. V1 wrong module import and V2
iterator helper compile errors were current-change failures, corrected before V3.
Focused test build V1 is running; no runtime acceptance claimed. Prior finalized
coverage, physical publication, native/JSON, caller-zero and retirement remain owed.

Task2 read-only connection refinement (not activation permission): original append
belongs in existing OrdinaryNewClaimLedgerV1, borrowing the retained source Rc,
expression site and ordered child values with exact block/BinOp. RawInvocation
child port supplies current source site/owner; existing binary completion carries
ordered child tuple rather than inferring source children from final MIR.
LocalSSA success lends the same original record and retains original->derived
producer plus each ordered operand relation. Its child arg() result alone is not
provenance: distinguish existing/cache/new appends and link each to original lhs/
rhs. Existing NormalCallableSemanticLoweringState ledger loan is sufficient.
Required negatives: foreign owner/equal-but-distinct source, changed site/op/
divisor, swapped or unrelated operands, missing original, duplicate row, failed
emission publishing none. Positive: one original source with original+derived
occurrences. Select this only after current retention acceptance; general Div,
new semantic authority, physical activation and ABI changes remain excluded.

Task2 failure refinement: LocalSSA BinOp child arg() uses LegacyFacade and can
recover the raw operand after failed materialization. A selected scalar root and
its recursive operand chain must instead use the existing Checked policy via the
same ensure owner. Require original->returned physical outcomes (identity/cache/
successful append) for both children, including Div's Const; failed child/parent
must not publish cache or derived correspondence. Keep unselected legacy callers
unchanged and do not reuse borrowed Mul authority. Tests must exercise child
emission failure propagation, no failed publication, unchanged cache relation and
ordered operand mismatch. This is a physical failure contract, not a new issuer.


Source-retention S0 acceptance: V1 test binary build8m20s completed. Same binary
scalar owner module15/16 PASS0.048s (new original-source/negative/branch tests3/3
PASS), field-read module8/9 PASS0.019s. Both failing existing tests reproduce at
the SAME named boundary in preserved pre-retention V12 binary: scalar module
12/13 and field module8/9. Failures are known pre-existing debt, not retention
regressions: static_current_owner_eq_home_rejects_incomplete_or_reordered_pair
(home-missing) and local_field_read_rejects_parameter_receiver
(source-actual-unavailable). Neither is ignored/waived or rebaselined. Exact binary
hashes, commands, patch/untracked hashes and logs are in
/tmp/static-scalar-source-retention-v1-receipt.json. Unrelated old rustfmt drift
was restored after testing; no semantic changes. Source files592/228/49/718/751,
test393 satisfy hard stop; pointer/diff guards PASS. Sole original source owner
retains2Sub+1Div and branch Rc identity without physical permission.
Close only MIRBUILDER-STATIC-I64-SCALAR-SOURCE-RETENTION-S0. Prior finalized coverage
S0 remains unaccepted; physical program/incoming consumer, native0/9/4194305,
Main closeout/caller-zero/old-edge and parent canonical JSON obligations remain.
Next selected design: MIRBUILDER-STATIC-I64-SCALAR-PRODUCER-CORRESPONDENCE-D0,
existing original Binary completion/ledger and sole LocalSSA Checked recursive
operand/producer transport, as bounded above. Internal contract work, no external
wait; no physical arithmetic activation yet.


## Task2 current producer correspondence (2026-10-12)

Construction evidence is verified below; S0/native acceptance remains UNACCEPTED. Source retention is accepted at
`1998e02ab6`; prior finalized coverage and original/index S0 remain unclosed.
Superseded Copy-only operand mapping was rejected by read-only review: LocalSSA
can actually re-emit Const/BinOp/Copy before the first original scalar append.
Actual Copy emits a new Copy after materializing its child, not Copy-forward.
No shape/equal-Const search, alternate forward arm or new source issuer is allowed.

Decision: retain the ordinary-new ledger's immutable scalar initializer source Rc
as sole authority. The sole LocalSSA materialization writer records physical
correspondence in its EXISTING function-owned frame state. Record successful
exact original/output producer and ordered child loans. Identity/cache lend the
same index relation; they do not mint append/source evidence. Generic recording
adds no legacy rejection and checks local shape only; selected consumption checks
same-index Rc membership, unique definitions and recursive producer mutation.
Keep index history across block cache clear. FunctionLoweringState, nested
function_state_transaction, idle check and root exit own save/restore/reset.

Original Binary completion receives raw children plus exact ordered operand
relations, validates same source Rc/site/owner/op and positive Divisor, then
returns Rc<OriginalScalarAppendV1>. SAME Binary descent installs that record at
original dst in the SAME frame index. LocalSSA successful derived append lends
that SAME record to returned dst. Duplicate install/target rejects. Failed child
or parent publishes no parent cache/relation/consumer record.

Selected original/derived re-entry uses CheckedScalar recursively. Converting
errors inside materialize is insufficient while ordinary arg() uses unwrap_or.
The existing fallible try_arg dispatch checks selected values; nonselected facade
is retained. Finite source consumers are bit_group -> Mul, shift_count -> Compare,
legacy BinOp children and default Call args. Loop CoreEffect BinOp/Copy/Call args,
selected Static and declared GlobalCall args use the same Result dispatch.
Completion refuses missing callable entry/ordinary-new ledger with installed
scalar originals. No deferred terminal, second reuse box or second semantic issuer.

At existing complete_construction_stores_v1, original inventory must match
expected source rows by count, unique site and exact source/record Rc. Derived
observations must match non-original scalar loan targets, append keys and collected
consumer targets. Validate every source/index membership, exact relation and unique
target before atomically retaining observations in the SAME ordinary-new ledger.
No partial consumer handoff on failure.

The SAME FinishedBindings projects original/derived mandatory operations and
non-Const/Copy relation dependencies at root AND child finishing. Do not compare
draft block IDs directly to final MIR. Exact captured membership and unique final
producer are required; instruction dst cannot collide with a parameter. Parameter
relations retain exact ordinal/ValueId and reject instruction collisions.
Only auxiliary Const/Copy omissions actually accepted by existing validate_complete
are retained by FinishedBindings; absence alone grants no permission. Final
no-definition/no-use includes return_env. No arbitrary pure-op omission becomes
scalar authority. If real source requires a nonselected Add/Mul snapshot omission,
decide that bounded policy before accepting, not via equality repair.

Projection prerequisite T0 extracted FinishedBindings responsibility from the
782-line physical_boundary.rs, preserving local_commit method visibility.
Original/derived source observations stay on the same ledger; final projection
issues no Integer, divisor, overflow, backend or execution permission.

Required acceptance still owed:
- Actual source original3 rows (2Sub+1Div), repeated derived Div, SAME Rc source,
  same finished projection and unchanged source identity.
- Const/BinOp before original; identity/cache/actual Copy append and nested-frame
  lifetime; selected child/re-entry failure before parent publication.
- Foreign source/index, changed op/divisor/ordered operand/producer, missing or
  duplicate original/derived target, consumer coverage and atomic failure.
- Finished exact/mutated/duplicate/parameter-collision producer, missing mandatory
  operation, permitted dead Const/Copy vs unvalidated/foreign/live omission.
- Task3 Sub policy, Task4 physical/C/JSON, Task5 native0/9/4194305, prior finalized
  coverage, production switch/caller-zero/old-edge retirement and parent JSON
  remain separate obligations. Whole SizeClass/mimalloc/MirBuilder not complete.

Earlier checks and integrated runtime: V1/V2 source/index/projection receipts
are retained at /tmp/static-scalar-finished-connection-v{1,2}-receipt.json.
The missing Ring0 initialization, wrong undefined-source emission fixture and
projection visibility errors were current-change setup/scope failures; corrected
code was checked and the actual Copy-emission refusal/source proof passed V2/V3.
They are not baseline debt, ignored tests or unexecuted acceptance. V3 below is
the latest integrated runtime. All physical/native/retirement/JSON debts remain.

Scoped-closeout review: correspondence/index/ledger/projection has no direct Rust
API dependency on preceding entry/coverage changes, but its unchanged real-source
invocation does require those runtime prerequisites (isolated V2 below). Keep
their responsibilities separate in the same series. Integrated real-source PASS
does not prove a Task2-only commit; do not drop the positive or close earlier S0.

Task3 read-only design recommendation (not physical activation): selected source
Integer Sub should share the existing published I64 Add/Mul modulo-2^64 policy,
with fresh Integer/no result lifecycle and no newly introduced overflow Fault.
The operator reference currently preserves the I64 policy without specifying
wrapping explicitly; document that bounded Decision before activation. Exact
numeric checked add/sub/mul in types.md is a separate deferred substrate and
is not authority for this source Integer row. Preserve the existing source
preflight/retained Binary/ordered operands; no new arithmetic receipt or solver.
Div stays original positive Integer-literal scope, excluding zero and MIN/-1;
no general Div or exact-width/usize permission. Task4 must retain I64-only
Sub/Div operands (not Add/Mul's tagged extension), complete source/finished
correspondence, and schema/index/type-flow/admission/emission checks together.
Focused conformance: MIN-1->MAX, MAX-(-1)->MIN, 0-1->-1; changed operator/order,
Bool/tagged/handle, missing operations and Div divisor0/-1/nonconstant reject.
Unchanged source and native0/9/4194305 remain required; Task2 is still selected.

Task4 read-only mapping review (not activation or Task2 acceptance): reuse
FinalizedRootSourceHandoffV1's SAME ledger and FinishedBindings. A read-only
visitor lends actual function, exact final binding and original source loan;
derived rows project the relation's retained target snapshot, not a value/shape
search in final MIR. Reverify the complete original/consumer set before lending.
Rust physical projection admits Sub/Div only at those exact function/coordinates;
missing, foreign, duplicate or changed producers reject. JSON projects the
validated instruction without issuing source authority or adding a receipt.
C schema/index/admission/flow/emission retain unique definitions, availability
and dominance. Both Sub/Div operands must be LV4_I64, excluding Add/Mul's tagged
extension. Div rhs must be an available unique same-function const_i64 in
1..=INT64_MAX; no constant evaluator or general Div. After Task3's policy is
fixed, emit plain sub/sdiv without nsw/nuw/exact or a new overflow Fault.
Before activation, separate behavior-preserving T0: extract operation schema
validation from the 766-line physical schema and teardown helpers from the
758-line emitter into responsibility-owned child includes. No second index,
translation unit, source issuer or old StaticV2 backend authority.
Negatives additionally cover unloaned/extra Sub/Div, foreign coordinates,
Bool/tagged/handle operands, zero/negative/nonconstant/non-dominating/duplicate
divisor and malformed JSON keys. Both original and derived operations must
survive the physical image. Task5 native/JSON and prior cutover/retirement debts
remain owed; this mapping does not change the selected Task2 row.


Test ownership: scalar real-source/finished mutation tests now reside in the
existing owner's private finalized_scalar_source_tests.rs child; pending borrowed
coverage borrows the same unchanged source/Main preparation. Split lib+tests
check PASS1m11s. Reconstruction matches pre-move V2 source SHA exactly:
/tmp/static-scalar-owner-test-move-receipt.json. V3 executes the separated test.
No isolated runtime or earlier coverage completion is inferred from that move.

Completion acceptance: missing original install, changed late derived producer,
no partial consumer publication, successful original-function handoff, duplicate
handoff refusal and preserved original/relation Rc are covered by the new existing
owner test. Correspondence tests moved to a private sibling to preserve800hard.
Two V1 private package/ledger test-scope errors were corrected through the existing
cfg(test) OriginalScalarAppend inventory fixture; production visibility unchanged.
Consumer V2 lib+tests check PASS1m11s. V3 source47-file snapshot did not change
during the original build. Initial accepted-vs-index Rc equality was added after
V3 terminal and still needs the isolated runtime below; no S0 closeout yet.

V3 integrated runtime receipt: no-run terminal PASS8m18s; both focused and
regression drivers terminal. Source47-file snapshot unchanged. Binary SHA256
a36899a53c524bd7d9562114b9a6e96335fbd8fab0526730c9d057a4100d470a, preserved
at /tmp/static-scalar-finished-connection-v3-test-binary. Index7/7 (including
consumer atomic/missing/resend), original ledger1/1, separated actual source/
finished mutations1/1, final projection1/1 PASS. Regression LocalSSA28/28,
physical boundary19/19, transaction3/3 and lowering state1/1 PASS. Required
aggregate remains red at instruction-unsupported; no waiver or physical claim.
Receipts /tmp/static-scalar-finished-connection-v3{,-regression}-receipt.json.

After original Cargo/rustc and both drivers became terminal, strengthened initial
handoff's accepted original/relation Rc equality against index observations.
New selected tree fb80099872fbde33ff5be5f67170d5c247152bba contains only29 Task2
paths; /tmp/static-scalar-task2-selected-snapshot.json records the exact tree.
Isolated no-run/8-filter driver is live at the archive checkout, reusing dependency
artifacts only after original Cargo termination. It will establish whether prior
unaccepted entry/coverage is actually needed by the real-source positive and
execute the exact-Rc success assertion. No isolated PASS inferred yet. Build log
/tmp/static-scalar-task2-isolated-build-v1.log; receipt planned
/tmp/static-scalar-task2-isolated-v1-receipt.json. Real index stays unchanged.

Isolated V1 no-run terminal101: only two current-change reference errors in the
new Rc equality assertion; no missing entry/coverage dependency was reported.
Borrowed both tuple Rc fields explicitly without changing production code. After
original isolated Cargo/driver termination, selected tree is now
24307681b65671788043907e3c767048dc0f84a9; all29 selected archive blobs match.
V2 isolated build/driver terminal: no-run PASS8m16s. Index7/7 (including exact
accepted/index Rc equality), original ledger1/1, final projection1/1 PASS;
regression LocalSSA28/28, physical boundary19/19, transaction3/3 and lowering
state1/1 PASS. Unchanged actual-source positive FAILS before the callback with
ordinary-new/emission/selected-static-packet/canonical-entry-required. This is
a real dependency on preceding unaccepted Static entry work, not baseline debt
or permission to replace/drop the source positive. Integrated V3 source PASS
cannot justify an independently accepted Task2 commit. Preserve exact selected
tree and receipts at /tmp/static-scalar-task2-isolated-v2-receipt.json; identify
the finite prerequisite entry hunks and their acceptance before scoped closeout.
Isolated binary SHA256 is
75577f51d5fb69da3bf22d3fa7f994006b4ff6b0eceff66ab593f0c1b9bbda3d;
preserved at /tmp/static-scalar-task2-isolated-v2-test-binary. All8 filters ran
nonzero tests; the driver exit0 does not turn its failing source filter into PASS.
Physical/native/retirement/JSON and earlier S0 debts remain unchanged.

Finite prerequisite review selected four construction checkpoints: tagged Static
entry correspondence; canonical actual loans plus BOTH finalized coverage
consumers; behavior-preserving FinishedBindings extraction; Task2 provenance and
finished scalar correspondence. Entry registry alone is insufficient: its tuple
validation needs the same checked source-only tagged cohort/owner view. The
entry/coverage/Task2 changes issue no new source authority and remain unaccepted
as native/S0/retirement evidence. Use one exact full-series runtime snapshot and
sequential lib+tests checks for each intermediate tree; do not repeat three heavy
test builds or claim intermediate runtime independence.
The unchanged source fixture now lives in the earlier Static actual test child;
the scalar test borrows it. Exact helper-body SHA256
b78bd3ab295798dab637ecaaf5d296fbaf9c7e8c0e70512a75b51abd77659061
is retained at /tmp/static-scalar-prerequisite-fixture-move-receipt.json.
Pure T0 method movement preserves bodies except owner visibility; receipt
/tmp/static-scalar-finished-bindings-t0-move-receipt.json. Prepared intermediate
trees and path sets are in /tmp/static-scalar-checkpoint-trees.json. Full series
tree1bead9fdf6c698c5acd0a23540312315e95924ef has47 code/README paths;
/tmp/static-scalar-series-selected-snapshot.json pins exact source blobs.
Full-series V1 build/12-filter driver terminal: no-run PASS8m15s, unchanged
source47-path snapshot. Entry preflight1/1, entry18/18, selected Main filter7/7,
index7/7, original ledger1/1, unchanged source/finished mutation positive1/1,
final projection1/1 PASS. Regression LocalSSA28/28, boundary19/19, transaction3/3
and lowering state1/1 PASS. The required aggregate still FAILS only after its
callback assertions at published-lifecycle-program/instruction-unsupported;
record it as unmet physical acceptance, not PASS or baseline debt. Driver exit0
means no unexpected failure, not aggregate success. Receipt
/tmp/static-scalar-series-v1-receipt.json; preserved binary
/tmp/static-scalar-series-v1-test-binary, SHA256
bdc8dd28c1d7d7f90df939a4be1aaa441a54f03f4beaa82ba7379d37d6fda8c0.
Only after both original build and driver terminated did the sequential
intermediate lib+tests checks run; all3 PASS. No S0/native/retirement completion
is inferred. Their exact trees/commands/times are recorded at
/tmp/static-scalar-intermediate-trees-v1-receipt.json.

## Construction checkpoints (2026-10-12)

- Entry correspondence: `49ad735662`; entry-only lib+tests PASS73.83s.
- Canonical actual/coverage: `1fbaa8e461`; coverage-only lib+tests PASS72.62s.
- Pure FinishedBindings extraction: `26cb57c4aa`; lib+tests PASS72.47s.

These save separate responsibilities in the same bounded series. Runtime uses
the exact combined source tree described above, not three independently linked
intermediate binaries. Coverage test EOF was normalized without changing any
non-trailing byte; /tmp/static-scalar-checkpoint-commits.json retains hashes.
All production blobs and the scalar test remain identical to the verified series.
Source/negative/final projection evidence supports the Task2 construction
checkpoint; prior S0, physical/native/JSON/caller-zero/retirement remain open.

## Arithmetic Decision and next construction boundary (2026-10-12)

Task2 construction is saved at `5af70e2b1f`; its unchanged source/negative
proofs are verified in the combined series. Earlier S0/native/retirement and
whole-goal acceptance are still unclosed and carry into this same series.
Task3 Decision: selected source Integer Sub uses current published Add/Mul's
modulo-2^64 policy, fresh Integer/no result lifecycle/no new overflow Fault.
Positive original literal Div truncates toward zero; no general Div, tagged
Sub/Div or exact-width/usize permission. Normative reference:
`docs/reference/language/dynamic-operators.md#published-current-i64-scalar-arithmetic`.
LLVM sub/sdiv semantics were checked against their primary LangRef sections;
plain flags match this policy, but emitter behavior is not source authority.

Current next slice is behavior-preserving C physical T0: extract operation
schema and teardown responsibilities from the 766/758-line files before
activation. Preserve existing translation unit, owner, type/flow/index checks
and all accepted/rejected inputs. Validate the existing C harness and exact
method movement, then connect Task4's source-loaned Rust/C/JSON path above.
No physical activation, old-edge deletion or new semantic receipt in this T0.
Task5 unchanged native0/9/4194305->1/2/73, final JSON/Normal/Fault, prior
production/caller-zero/retirement and whole SizeClass/mimalloc/MirBuilder remain owed.

C physical T0 verified (2026-10-12): operation/instruction schemas moved into
physical_instruction_schema child (parent766->497, child272); marked-field
teardown moved into teardown_emit child (parent758->667, child94). Original
bodies reconstruct byte-for-byte, and cc -E -P output is identical before/after
with HAKO_LLVMC_LIFECYCLE_TEST_SEAM. Existing physical parser, receiver identity
and nested-call regressions all PASS; no Rust/Cargo rebuild was needed for
unchanged C preprocessing. Receipts /tmp/static-scalar-c-t0-{movement,regression}-receipt.json.
No Sub/Div activation, source/native acceptance or retirement is claimed.
Next is Task4's same-source-ledger/FinishedBindings loan into existing physical
projection and matching C/JSON checks, under the arithmetic Decision above.
