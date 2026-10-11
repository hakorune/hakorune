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
