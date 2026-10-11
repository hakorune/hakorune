# MIRBUILDER-ROOT-EXIT-STATIC-TWO-EXIT-S0

Status: selected execution slice
Date: 2026-10-11
Scope: selected Static V2 early/tail root exit cutover for unchanged `SizeClassBox.size_to_bin/1`
Related: `mirbuilder-root-exit-canonical-writer-d0-2026-10-11.md`, `mirbuilder-root-exit-canonical-writer-r1-2026-10-11.md`

## Authority and boundary

Consume the two existing Home-issued explicit exit rows and original selected
Static call packets through the one root graph algorithm in
`ordinary_new_admission/selected.rs`. The early return is Plain with the
loop's current I64 value; the tail return has the sole `me.huge_bin()` Call
ingress. Both exits share the prior `me.normalize_size(size)` lexical group
according to source `covered_calls`. The `me.bin_size(bin)` body packet stays
a separate selected body retention. Do not infer an extra local group from
either body or tail packet, and do not rewrite `.hako`.

The canonical writer from R1 registers and emits each root graph under the
selected Static `CanonicalSsaFunctionSessionV2`. Keep both clean targets open
until DraftSeal projects their Returns. Replace the fixed 14-block finish
assumption with exact graph-receipt checks, then seal the completed CFG and
Binding SSA identities in dependency order. Do not adopt a raw graph or
repair predecessor caches.

Prepare one two-exit root batch with a read-only pending view. Validate both
projected Returns, root bindings, Home Normal/Fault order, source sites, and
physical child emissions before collector admission. Commit the selected
ledger changes only after successful `.collect()`, with no fallible lookup
afterward. A rejected duplicate collector must leave every selected row
unpublished. `record_root_home_exit` cannot run in the detached callback.

## S0 Decision: atomic pending view (2026-10-11)

Read-only audit of the selected source found that a pending lexical group
alone is insufficient. `claim_static_terminal_root_route_v1` removes the
`huge_bin` site from `lifecycle_local_call_sites`, and exit-group selection
reads that same route set. The detached child validator reads committed root
exits in three places: `validate_new_emissions`,
`validate_root_home_exit`, and `validate_root_cleanup_shape`; it also stores
`child_physical_validation`. Calling the existing mutable claim or root
recorders before collector admission would violate rollback on duplicate
collector rejection. Omitting the route removal would incorrectly demand a
lexical group for the tail terminal ingress.

Decision: prepare an exact selected batch whose one read-only validation view
contains (1) the shared `normalize_size` lexical group, (2) exclusion of the
one original `huge_bin` terminal route, and (3) both pending root exit rows.
All three physical child validation readers must consume that same view.
The view is ephemeral and does not create a second semantic authority. Check
the tail packet's original `Rc` identity and route membership without
mutation; derive both exit orders and fault suffixes from the already
verified Home preparation rules. Reuse the root graph algorithm with the canonical writer;
do not add another cleanup emitter. Hold the physical validation receipt
pending as well. Preflight every exact ledger target and commit order, then
after successful `.collect()` consume the route, lexical group, body
retention, two root exits, and physical receipt infallibly. The ordinary raw
caller retains its current immediate record path.

The selected tail also needs the original publication handoff. The collector's
existing four-site `take_selected_batch_for_source` returns owned handoffs in
source order, but `consume_selected_static_result_batch` currently drops them.
Preflight the tail packet with the borrowed handoff before admission and carry
its original source and Invoke/Normal projection through the pending root
view. In the same successful collector transaction, retain the fourth owned
handoff, assemble the checked tail packet, then commit the root entry after
`.collect()`. Do not take a publication row early or issue a duplicate
handoff; the batch's exact site order is the correspondence authority.

S0 correction from focused source probe: the selected path bypasses raw
`prepare_root_home_exit`, so its `root_exits` slots are not yet `Prepared`.
The read-only preview must apply the same construction-ready, root-flow,
explicit-site, Normal projection, Home end-plan, and direct-object rules as
raw preparation. Both selected I64 exits require available orders; a missing
or unavailable order is an explicit refusal, while a valid empty order is
allowed. Reject a direct-object return in this I64 profile. Carry the two
owned orders through graph emission and pending validation, then install
their checked progress only after collector success. The legacy raw caller
continues to commit its preparation at its existing dispatch point.

Evidence: at `325cf48950`, the preexisting whole-app focused test fails before
selected lowering at `ordinary-new/static-mixed/source-actual-drift` (baseline).
With unchanged `size_class_box.hako` plus trivial Main, the same selected test
first reached `ordinary-new/local-commit/root-exit-not-prepared`. The shared
read-only Home preview moved that first stop to `root-exit-unconsumed`; the
pending validation view and post-collect commit remain unimplemented.

Source authority + canonical issuer: existing Home plans, original Static
packet identities, root flow `covered_calls`, and canonical CFG/DraftSeal.
The pending view only carries their prevalidated projection.

Non-authority: MIR rediscovery, global all-caller inference, a temporary
ledger insert/rollback, or a second physical cleanup graph.

Fail-fast boundary: reject absent/foreign route, packet identity drift,
missing or extra lexical group, unprojected Return, wrong Home or binding
order, stale CFG predecessor, and duplicate collector without a partial
ledger publication.

Smallest next slice: this selected two-exit S0. Its physical cutover remains
the completion condition; the pending view is a prerequisite inside S0, not
a separate accepted production result.

Non-claims: this decision does not make the selected source pass yet and does
not authorize a broader Static Loop grammar.

## Acceptance

- Focused positive: unchanged original `size_class_box.hako` plus trivial
  Main advances past `root-exit-unconsumed`; both source exit rows have
  physically validated Home progress, exact Return projections, and the
  sole tail Call ingress.
- Focused negative: foreign/missing projected Return, wrong Home or binding
  order, source packet or tail-route drift, stale CFG predecessors, and
  duplicate collector fail without publishing partial root, route, lexical,
  body, or physical-validation rows.
- Source-level expected lexical sites remain derived from `covered_calls`;
  one focused check shows both exits claim the same prior entry group.
- Reuse existing canonical CFG, DraftSeal, root cleanup, publication, and
  physical-boundary tests. Verify the selected JSON/EXE route required by the
  D0 decision on the unchanged source; classify backend-only limits precisely.
- Remove only the selected old early/tail termination edge after the caller
  switch and caller-zero evidence. Keep shared raw Plain/Call consumers.

## Non-claims

No whole mimalloc-lite EXE success, wider Static Loop grammar, general
multiple-return inference, or final MirBuilder completion follows merely
from this selected two-exit cutover.

## Current status (2026-10-11)

`f9cb8906be` remains an explicit unfinished checkpoint, not S0 acceptance.
The archived unrelated work stays on local `archive/main-dirty-20261011`
(`6e5c7d98d3`) and was not merged into this lane.

Implementation now carries the selected two-exit batch through one
`PendingSelectedStaticLoopViewV1`: the shared `normalize_size` lexical group,
the three non-lexical selected routes, both pending root exits, the body
retention packet, and a pending child physical-validation receipt. The tail
packet is preflighted with the borrowed fourth publication handoff; its owned
handoff is consumed only after collector success. `unpublished.collect()`
then commits the three non-lexical routes, lexical group, early/tail exits,
body packet, and physical receipt. Phi-bearing root source blocks enter the
canonical writer through one isolated clean trampoline; non-Phi source blocks
keep the existing writer shape.

Current evidence:

- `cargo check --profile quick -p nyash-rust --lib` passes.
- `cargo test --profile quick -p nyash-rust --lib
  original_static_loop_collects_only_after_packet_preflight -- --nocapture`
  passes 1/1 on the unchanged source plus trivial Main. The positive path
  commits one body packet, one lexical group, two shared-group root exits,
  and one physical receipt while retaining only the `normalize_size` route.
  The missing-route case rejects with `static-packet-route-missing` and no
  selected ledger publication; the duplicate-collector case rejects before
  root, route, lexical, body, or physical rows are committed.
- Existing focused filters pass: `local_commit` 82/82,
  `root_graph_writer::tests` 3/3, `canonical_cfg` 33/33,
  `module_draft_collector` 39/39, `completion_draft_seal` 12/12, and
  `static_result_publication` 31/31.
- `cargo build --profile quick -p nyash-rust --bin hakorune` passes.
- Physical CLI probe uses the unchanged `size_class_box.hako` contents plus
  trivial Main under `/tmp` with `NYASH_DISABLE_PLUGINS=1`,
  `HAKO_BACKEND_COMPILE_RECIPE=pure-first`, and
  `HAKO_BACKEND_COMPAT_REPLAY=none`. `--emit-mir-json` reaches egress and
  returns rc1 at the existing JSON allowlist: `unsupported terminator Invoke`
  (no JSON artifact). `--emit-exe` with
  `--emit-exe-nyrt target/lifecycle-kernel/release` returns rc1 earlier at
  `ordinary-new/static-scalar/incoming-veto` `NoIncoming(slot 7)`; no EXE is
  produced. These are recorded limits, not EXE/JSON acceptance.
- `git diff --check` passes; source files stay below the 800-line hard stop.

This is not accepted S0 or a production cutover receipt. The selected direct
tail `MirCall::global` construction was removed from `static_loop_tail`, but
the final caller-zero review, card-required closeout evidence, and any later
old-edge retirement still need explicit verification before the slice is
closed. No whole `SizeClassBox`, mimalloc-lite, or MirBuilder EXE pass is
claimed.

## Independent closeout audit (2026-10-11)

Read-only worker and main-agent source review at `47b5886077` confirm the
production selection in `normal_callable_semantic_loan_port.rs` dispatches
`collect_if_selected` and returns on success; selected errors propagate with
`?`, without retry to the later canonical route. Source selection is unchanged.

Selected old responsibilities, bounded to this Static V2 two-exit caller:

- Early: `f9cb8906be` replaced completion at `body_blocks.then_block` with
  completion at the sole root graph's `clean` block. Current body-exit emitter
  passes the Home plan to `emit_selected_root_graph_v1`; no selected direct
  early-return completion remains at the body block.
- Tail: `47b5886077` removed `MirCall::global(None, target, vec![])` from
  `static_loop_tail.rs`. The current file contains no `MirCall::global` or
  separate `emit_i64_invoke`; original packet/publication loan materializes
  the Call used by the sole root graph. Shared raw Plain/Call owners remain.

This is static before/after and caller-zero evidence for those exact old
responsibilities, not whole-owner retirement or completed S0 acceptance.
No additional production deletion is authorized by this audit.

All Result-returning admission/publication operations precede `.collect()`;
the following commits publish the prepared route/group/root/body/physical
rows. No post-collect transaction defect was found in this read-only audit.
The missing acceptance is concrete: pending-view rejection of foreign/missing
Return, wrong Home/binding order, tail packet/route drift and stale predecessor
must demonstrate no selected ledger publication. Generic owner tests alone
are insufficient. The independently mutated final-MIR Return checks in the
existing source test address finalization only, not this precollect obligation.
JSON/EXE limits above remain outstanding physical acceptance; do not narrow
S0's completion condition or silently retire its unfinished requirements.

Next bounded test work stays in S0: an owner-local cfg(test) child of
`static_loop_entry_session` can destructure the prepared batch and borrow the
uncollected function with `pending.complete_before_restore`. Only the body
retention packet needs an owner-local cfg(test) loan. Reuse the single pending
validator for Return/tail packet/excluded-route mutations and assert all
selected publication counts remain zero; do not add a second production
validation, environment injection hook or semantic receipt. Nonempty Home
order mutations require an existing nonempty Home fixture; this source's
empty order cannot prove ordering. Canonical finish/DraftSeal owns stale CFG
predecessor rejection, not a new pending-view CFG authority.

Focused finished-MIR evidence (2026-10-11, code based on `47b5886077`):
`original_static_loop_collects_only_after_packet_preflight` passes 1/1 with
six independent mutations: each of the two original Return terminators is
removed, changed to no-value, or given the other exit's value. Each mutated
module refuses final child validation; the unchanged module then succeeds.
The existing missing-route and duplicate-collector unpublished-state checks
also pass in the same test. This does not supply the missing precollect
Return/packet/Home-order evidence. First compile failed on a test-helper
Rc/reference mismatch; borrowing the ledger fixed it. Final quick build/link
8m14s, test execution0.03s; no current-change red remains in this focused run.
Pointer guard, qualified-route scope guard, touched-file rustfmt and diff check
pass. No full-suite or fresh CLI acceptance was run for this test-only change.

Pending-negative implementation Decision: separate the existing Static entry's
preparation from its immediate collect terminal, preserving source selection
and production call order. The existing source fixture can then select and
prepare the same product, borrow the uncollected function, validate the one
pending view, mutate it, and discard without collector admission. No new
production validation, environment hook or semantic issuer is introduced.
The body retention owner lends its packet only under cfg(test). Tail mutation
uses publication batch ordinal3's original target; zero-argument matching is
not authority because the header call is also zero-argument.

Required size prerequisite: `resolved_lowering/mod.rs` was already803 lines.
Its canonical function draft-entry `impl MirBuilder` is moved intact to
`function_draft_entry.rs` (parent408/child400), with parent private helpers
and original method visibility retained. Record this as a separate BoxShape
commit from the pending-negative validation change; no grammar is widened.

Size prerequisite verification: normalized Rust syntax of the extracted
`impl MirBuilder` equals the pre-split parent implementation. Existing
`normal_function_draft_lowering_tests` pass3/3 using the same built binary
(restoration, typed BodyLowering refusal, typed DraftSeal refusal). No method
contract or authority changes in this extraction.

Pending-batch evidence (2026-10-11): the existing original source test passes
1/1 including its new no-collect branch. One successful pending-view check
is followed by nine independent refusals: both projected Returns missing or
using the other exit's value (4), original tail target/result drift (2), and
each of three route exclusions omitted (3). Every case keeps lexical-group,
root-exit, body-retention and physical-validation counts zero. At callback
completion collector symbols remain zero and all four original routes remain
unconsumed. Original normal, missing-route and duplicate-collector branches
and six independently changed final Return checks still pass.
The first pending-test compile was intentionally stopped after read-only
review found ambiguous zeroarg tail selection; this has been replaced by
exact publication ordinal3 source target. Final quick build/link8m14s,
test0.04s. Canonical CFG regression33/33 and typed function draft regression3/3
reuse that binary, each0.00s; pointer/scope guard, format and diff check pass.
No current-change failures remain in these focused checks, no new test names,
no failure baseline update and no fresh CLI/all-suite acceptance claim.

S0 remains open: nonempty Home/origin/binding-order negative evidence and
physical JSON/EXE requirements are still owed. The nine pending refusals
close only their enumerated Return/tail/route unpublished-state obligations.


Nonempty Home pending-order Decision (2026-10-11): reuse the existing
`ordinary_new_home_prefix_retains_order_and_requires_prior_installation`
source-issued three-Home fixture. Obtain the existing read-only graph plan
before raw preparation and retain its Normal order (third, second, first).
An owner-local cfg(test) child calls the same pending root preparation and
cleanup shape validator before any root publication. Check origin reversal
and omission, wrong entry binding, and physical Normal/Fault path changes;
keep recorded instructions consistent with changed physical nodes so path
refusal is not merely a snapshot mismatch. Drop every pending row and assert
root progress remains absent/Unprepared, then run the original raw
prepare/begin/record/finalization checks. This tests the shared nonempty Home
owner, not a newly source-selected nonempty Static Loop profile. It does not
replace the original selected two-exit transaction test or physical CLI
acceptance. Focused verification is recorded below; S0 remains unaccepted.


Physical acceptance follow-up audit (read-only, 2026-10-11): canonical MIR
JSON document egress and lifecycle EXE physical publication are different
consumers. `backend_core_ops/allowlists.rs`, `mir_json_emit` control-flow
writer, and the instruction reference keep generic Invoke/Normal/Fault JSON
admission closed. Do not add only an allowlist exception or substitute the
lifecycle ABI JSON for the canonical document. The document egress owner
needs a lossless schema/writer/reader contract preserving the published
call/carrier, Normal projection, Fault frame/edges, cleanup order and source
metadata. The S0 JSON requirement remains outstanding.

EXE `NoIncoming(slot 7)` originates in the borrowed incoming inventory query,
not runtime reachability. The same cohort query has both package selection
and final source/entry consumers, so the token alone does not prove which
consumer rejected the original probe. Next physical diagnostic: identify
slot7's exact source callable and first demanding consumer using existing
declaration identities. Then decide the borrowed incoming/entry boundary
between an unselected source-only candidate and a published function needing
entry authorization. Retain selected whole-cohort veto; do not suppress all
NoIncoming errors, infer Integer from an opaque formal, delete uncalled
functions without a selected deletion contract, or change the original Main
to manufacture incoming evidence. This audit selects no new production
implementation and does not waive either physical acceptance condition.


Guard classification during pending-order verification: pointer guard PASS.
`mirbuilder_qualified_route_scope_guard.sh` currently exits1 at line446:
`lower_with_source_publication` count3 versus required >=4. Both guard and
`raw_loop_child_entry.rs` are unchanged from HEAD `9c8d4a59a9`; that HEAD blob
also counts3. This is known pre-change guard debt, not a pending-order test
regression. Earlier recorded scope-guard PASS is historical evidence, not a
PASS for this checkout run. Do not weaken the count as part of this slice.


Nonempty pending-order evidence (2026-10-11, based on `9c8d4a59a9`):
`ordinary_new_home_prefix_retains_order_and_requires_prior_installation`
passes1/1, including the original raw installation/completion/finalization
checks and the new pending positive plus five refusals described above.
Every discarded pending result leaves the source root absent/Unprepared;
read-only preview operations equal the subsequent raw begin result.
Quick build/link8m15s, test0.01s. The same built executable runs
`original_static_loop_collects_only_after_packet_preflight`1/1 in0.04s,
retaining selected transaction and Return/tail/route coverage. No new test
names or baseline changes. Touched Rust format and diff checks PASS.
Pointer guard PASS; scope guard is the classified pre-change count debt above.
Read-only worker confirmed the physical mutations test missing Normal
obligations and Fault successor semantics, not only recorded snapshots.

This closes the shared nonempty Home/origin/entry-binding/Normal/Fault order
negative obligation. It does not claim a new nonempty selected Static source
profile or whole-owner retirement. S0 remains open for the exact physical
JSON/EXE acceptance and final caller-zero/deletion closeout at that revision.
