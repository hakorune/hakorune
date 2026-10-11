# MIRBUILDER-STATIC-I64-LOOP-ARTIFACT-HANDOFF-D0

Status: mapping closed; S0 selected, acceptance pending
Date: 2026-10-11
Scope: existing selected Static LoopV2 entry/header/body physical bindings
Related: `mirbuilder-main-static-terminal-ingress-d0-2026-10-11.md`, `mirbuilder-root-exit-static-two-exit-s0-2026-10-11.md`

Selected row: `MIRBUILDER-STATIC-I64-LOOP-ARTIFACT-HANDOFF-S0`.

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
