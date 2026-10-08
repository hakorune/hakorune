Task: DOCS-SPEC-ADAPTER-ARTIFACT-HYGIENE-D0
Status: accepted follow-up queue; implementation is not selected
Date: 2026-10-08
Scope: four independent hygiene tasks plus the existing borrowed-residence safety task
Exception: explicit user-requested task registration, separate from the compiler slice
ParentCurrentCard: docs/development/current/main/investigations/repo-hygiene-audit-punchlist-2026-09-15.md
Related:
- docs/development/RULES.md
- docs/development/current/main/CURRENT_STATE.toml
- docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
- docs/development/current/main/design/current-docs-archive-policy-ssot.md
- docs/reference/language/variables-and-scope.md
- docs/reference/language/failure-outcome-relations.md
- docs/development/current/main/investigations/test-guard-responsibility-retirement-task-2026-10-08.md

# Hygiene follow-up task map

This map turns the user's feedback into bounded tasks under existing owners.
The active compiler selection remains unchanged; parked lanes remain parked.
These improvements are not new mandatory conditions for the finite compiler goal.
Registration is complete; implementation, deletion and performance gains are not.

## Verified current evidence

Read-only evidence: HEAD `e16549a8bd` plus the retained Root-index WIP, 2026-10-08.
CURRENT_STATE selects `mirbuilder-result-new-site-provenance-d0-2026-10-06.md`,
971 lines at observation. The quoted string-indexof card is not that selected
card; it and the workstream are each 1,000 lines. Closed history must not select
future work or override CURRENT_STATE.

`instance_construction.rs` is now 743 lines after the provider admission split
at `c36b9095d3`; the quoted 970-line state is obsolete. Among tracked Rust files
under `src/` and `tests/`, the current files at or above 800 lines are:

| File | Lines |
|---|---:|
| `src/mir/resolved_value_profile/analyzer.rs` | 842 |
| `src/mir/compiler/capability.rs` | 819 |
| `src/mir/normal_callable_semantic_package/instance_constructor_semantic/tests.rs` | 800 |

This is a scoped census, not a whole-repository count or a delete set.
`variables-and-scope.md` describes current `local x` defaulting to null;
`failure-outcome-relations.md` explicitly labels UninitializedSlot as a target
and retains `uninitialized_local_activation = 0`. They need clearer navigation
between current and target contracts, not an accidental runtime behavior change.

## Independent tasks and timing

| Task | Select when | Responsibility and completion |
|---|---|---|
| H-1: active history thinning | Before the next append reaches the card cap; workstream before its next append | Keep current contract, unresolved decisions, required acceptance, known reds and parked boundaries. Replace completed execution prose with exact commit references. Preserve incoming links and shrink selected tracked text. |
| H-2: current/target specification alignment | A separate documentation slice at a compiler slice boundary | Give `local x` one current/target/activation/owner comparison in its existing reference owners. Both references lead to the same activation decision; current null behavior and target activation 0 remain explicit. |
| H-3: source responsibility split and adapter retirement | At the owning family's next implementation boundary | Split one oversized owner by responsibility without semantic changes; preserve signatures/callers. Separately switch any superseded adapter's callers, verify its obligations, prove exact caller-zero and physically delete only that exclusive edge. |
| H-4: generated/proof/receipt retention | After one artifact family's generator and live consumers are identified | Use existing GENERATED-ARTIFACT-RETENTION-D0 and lifecycle inventory. Record canonical input, generator, reproduction, consumer, release need and `retire_when`; delete only reproducible, unused copies after dependency closure. |

H-1 uses the existing history/archive policy and DOCS-HISTORY-RETIRE family.
Its first target is the selected card's completed history; only touch the old
string-indexof card or workstream when that document's owner needs an update.
Already-landed DOCS-HISTORY-RETIRE-R0/R1 batches stay closed. Moving full bodies
to another tracked archive is not text reduction. Current semantic decisions
must remain readable without reconstructing them from commit history.

H-2 is documentation only. If a new activation is desired, it needs a separately
selected semantic slice and its own source/VM/backend acceptance. Do not infer
activation from example wording or conflate null, Unit and UninitializedSlot.

H-3 starts with one of the currently oversized owners when its family is selected.
Do not re-split the already corrected instance_construction owner based on the
obsolete count. At 760 lines design the split; do not add to an oversized file or
compress formatting to meet the cap. Size-only splitting and adapter deletion
are separate responsibilities. Unknown or shared adapter consumers remain.

H-4 uses a release/consumer condition rather than an arbitrary expiry date.
Required acceptance evidence and non-reproducible assets remain available.
Missing generator or unresolved consumers means UnknownRetain. The historical
108-file inventory is not a newly measured baseline; recheck the chosen family.

Each implementation slice fixes its authority, named consumer, replacement or
delete set and acceptance before construction. Later deletion requires caller
switch/stop, required checks and exact caller-zero. Reuse existing checks; do not
create a new guard, whole-repository ledger or parallel receipt family for this map.

## Separate safety task: existing punchlist P2 row 10

Owner: `src/boxes/map_array_residence.rs`; sole production constructor caller:
`crates/nyash_kernel/src/exports/fault_checked_map.rs::install_borrowed_array`.
Read-only audit_contract review confirms the public safe raw-pointer constructor
checks empty/null only; safe `read_map` dereferences before checking liveness.
The returned CheckedMapReadView also contains a lifetime-free borrowed pointer
and is Clone/Send/Sync. A comment about synchronous use does not enforce lifetime.
This is a confirmed safe-API soundness gap; production UAF/crash was not reproduced.

Decision for selection: isolate raw intake behind an explicit unsafe boundary
and fix the residence plus all issued view/clone lifetime and root contracts.
Do not treat an unsafe-constructor-only patch as full closure. Do not substitute
OwnedMapArrayResidence blindly: it ends children, unlike the caller-only End law.
This safety repair has its own slice; it need not wait for all four cleanup tasks.

Acceptance: borrowed install -> array index -> text read; duplicate roots preserve
caller-only End; empty/null and ended-but-still-allocated root reject; the selected
API prevents safe raw construction or owner-outliving view escape. Retain existing
kernel tests and add only the missing lifetime/escape evidence required by the
chosen contract. Invalid-pointer dereference is not a necessary test technique.
Review Clone/Send/Sync promises under that same lifetime contract before closure.

## Registration closeout

Registration owns this map and its hygiene-punchlist navigation/safety row only.
Check metadata/links, pointer guard, diff and protected WIP; compiler acceptance
and other authors' reference changes are outside this documentation registration.
Pointer guard, metadata/links and diff checks PASS; all 21 pre-existing WIP file
hashes remain unchanged. No new test/guard or compiler acceptance was added.
