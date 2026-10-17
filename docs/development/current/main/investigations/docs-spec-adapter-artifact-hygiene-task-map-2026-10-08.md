Task: DOCS-SPEC-ADAPTER-ARTIFACT-HYGIENE-D0
Status: accepted follow-up queue; implementation is not selected
Date: 2026-10-12
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
- docs/development/current/main/workstreams/language-v1-convergence-current.md
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
| H-2: user guide / specification / implementation status | Navigation correction now; verified guide after language delivery evidence | Separate tested usage, normative meaning and mutable implementation status. Preserve the existing `local x` current/target comparison in its owners. Each usable example must have a fixture, revision/profile and reproducible EXE command. |
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

2026-10-12 user review extends H-2 to the language entrypoints:

- `docs/reference/language/README.md` routes readers to normative topic pages,
  implementation status and the eventual verified guide. `quick-reference.md`
  must honestly identify its current mixed syntax/target coverage until its
  examples have the delivery evidence owned by Language v1 C-1–C-3.
- Then rewrite `quick-reference.md` around commands and tested examples only.
  Put target rationale in its design owner and migration/activation vocabulary
  in the existing status/task owners. Preserve topic contracts and useful links;
  moving prose cannot activate a feature or remove a semantic obligation.
- Complete when every advertised example maps to a runnable fixture/command
  and the named supported profile, with no `production 0`, internal task tokens
  or parser-only success presented as usable behavior. Planned features remain
  discoverable through a separate link; no second support-status ledger.

Navigation is corrected in this documentation slice. The verified guide remains
queued; no fresh EXE support was measured or invented. Language decisions and
delivery acceptance are owned by the
[existing workstream](../workstreams/language-v1-convergence-current.md#language-review-decision-2026-10-12).

H-3 starts with one of the currently oversized owners when its family is selected.
Do not re-split the already corrected instance_construction owner based on the
obsolete count. At 760 lines design the split; do not add to an oversized file or
compress formatting to meet the cap. Size-only splitting and adapter deletion
are separate responsibilities. Unknown or shared adapter consumers remain.

H-4 uses a release/consumer condition rather than an arbitrary expiry date.
Required acceptance evidence and non-reproducible assets remain available.
Missing generator or unresolved consumers means UnknownRetain. The historical
108-file inventory is not a newly measured baseline; recheck the chosen family.

The 2026-10-12 `apps/*-proof` feedback is included in H-4, not a new delete-all
task. For one selected family, distinguish human examples, permanent semantic
tests and disposable migration proofs. Route permanent tests to their test owner
when needed, update all imports/runner/docs consumers, and delete only exclusive
completed proofs whose obligations have a retained replacement. Keep concise
human examples discoverable from `apps`; Git retains completed migration history.
Acceptance is preserved coverage plus resolved consumer links and reproducible
generation where required. Directory suffixes and reported counts do not prove
retirement eligibility; no app was moved or deleted in this registration.

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

## 2026-10-17 non-test growth audit: checked disposition

User requested a response policy; the following is a scoped read-only check at
ac35be25de plus the selected Text-read WIP. No deletion, archive move, history
rewrite or compiler-row change is made by this registration.

| Existing owner | Finding and bounded next action |
|---|---|
| H-4 generated artifacts | `tools/checks/rust_source_topology/target`1.2GiB, `tools/hako_check/tmp`1.7GiB, `__mir__.log`93MiB, `dist`106MiB are untracked/ignored. Target/tmp/log are first regeneration/active-use checks; distribution assets additionally need release/retention checks. Scoped lsof reported no matching open files, with inaccessible unrelated mount warnings, so it is not a whole-host non-use proof. These paths are on workdisk, not the root filesystem. At observation root has6.4GiB free, workdisk about1.1TiB. Deleting them would not recover root space. |
| Existing test/guard retirement family | Measured240 hako-alloc app directories(~2.1MB),377 k2_wide_hako shell guards(~1.6MB), rather than374. `proof_apps.toml` and includes register206apps/public guards; allocator `.steps` directly select additional EXE guards. Absence from `guard_rows.toml` does not prove caller-zero. Even unregistered apps have direct guard consumers. No bulk archive set is established. Select one family, identify successor and all manifest/direct consumers, switch and verify, then retire its exclusive old assets together. |
| H-1 active history | phase-296x exists in both `docs/development/current/main/phases/phase-296x`(~15MiB) and `docs/development/archive/phases/phase-296x`(~856KiB). Keep live contracts/links; compress completed bodies to commit references. Moving tracked text to another directory alone does not reduce repository text or history size. The reported278K lines was not remeasured here. |
| H-3 semantic owners | The reported ncsp seven-day+49K/493files is external audit data, not a new measurement. Continue exclusive old-edge retirement at each owning series closeout, then review responsibility and duplicate owners at that family's checkpoint. Do not turn a directory-size target or broad rewrite into a new compiler-goal prerequisite. |
| Existing test/guard retirement family | Nine `*_min_exe.sh` wrappers exist. Compare their runtime resolution, emit/run, cleanup and route checks for common-lib extraction; preserve source-specific output, expected rejection, profile and skip/fail boundaries. The reported95% similarity and retirement-metadata coverage were not independently counted. Add owner/successor/retire_when to temporary assets when their family is selected, without relabelling permanent safety evidence as temporary. |
| History maintenance | Reported huge blobs/14generations/2.83GB remain unverified audit inputs. Gate-1-after is the proposed review point, not authorization for rewriting shared history. Measure reachability and size, prepare backup and all-ref/remote coordination before a separately approved filter-repo operation. |

Read-only review reused the existing source-review worker. It corroborated
`tools/checks/proof_apps.toml`, `tools/checks/guard_rows.toml`,
`tools/checks/allocator/k2_wide_allocator_gate.steps` and
`tools/checks/allocator/families/hako_alloc_production.steps`.
The guard migration SSOT requires stable entry handling and retains unknown
legacy callers until an explicit successor/caller-zero retirement decision.
`Still Closed` is not evidence that a campaign is completed or unused.
