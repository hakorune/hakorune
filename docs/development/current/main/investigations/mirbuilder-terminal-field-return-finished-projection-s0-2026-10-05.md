# Terminal field return finished projection S0

Status: Landed / bounded slice complete
Execution row: MIRBUILDER-TERMINAL-FIELD-RETURN-FINISHED-PROJECTION-S0
Scope: preserve the existing source-issued I64Field return through canonical CFG
finishing, using the same captured PhysicalBoundary/FinishedBindings projection.
Related:
- mirbuilder-declared-object-tagged-borrow-physical-s1-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision

Source authority + canonical issuer: existing TerminalRelationV1::I64Field,
its exact source exit/read site and emitted dst/base/field/block. No new source
vocabulary, result claim, borrowed Home or class inference. Existing draft
emission captures the physical boundary; finishing already issues FinishedBindings.

Production consumers: validate_finalized_child_emissions before capture,
validate_root_body for initial/root finishing, and validate_finalized_child_functions
for ordinary-child finishing. The current terminal field validator looks up the
original block even after the canonical sole-predecessor CFG merge moves that
read. Select the existing check_binding mapping; no second scan/registry/remapper.
Keep exact dst/base/field, recorded result and direct Return.value checks, with
existing boundary.validate_complete preserving original CFG/sequence.

Non-authority: any field read with the same result type, diagnostic names,
a guessed surviving block, relaxing final source correspondence, AST rewrites.
Fail-fast boundary: wrong/deleted/duplicate/reordered read, result drift, foreign
field/base/value or return drift remain rejected by the terminal/boundary owners.

Smallest slice: lend the existing physical binding checker to the terminal read
validator at its three callers, keeping draft None and finishing Some(projection).
Read-only worker reviews exact interface/return obligations before implementation.
No additional visibility/authority or unchecked default callback is authorized.
Replacement: original-block-only terminal read check after finishing. Existing
draft checks, field-read coverage, source result contracts and boundary remain.

## Acceptance

- Original guarded direct formal.field return, null/object, both optimization
  settings through source->final MIR->physical JSON; execute both arms.
- Exact moved read allowed only through original boundary projection; reject
  orphan movement without canonical CFG, wrong field/base/dst/Return and extra
  read/redefinition. Focused positive/negative tests use existing owner fixtures.
- Retain S1 ingress/forwarding, physical boundary and source field families.
- Package red classification and selected scope pins; guard root1351 debt is
  HEAD-identical baseline, never full-green evidence. Source files below800.
- Owner README, pointer, whitespace, exact scoped staging; fresh unchanged app
  probe if a relevant production terminal changes. No .hako workaround.

## Evidence / non-claims

S1 landed/pushed fe0c9ad278: source20both-opt/C20Normal-Fault plus15 negatives,
entry21, source25, boundary11, JSON29; package622/exact known3, C3 regressions.
Fresh quick llvm-boundary PASS; unchanged mimalloc-lite rc1 at
ordinary-new/local-commit/artifact-unowned-lifecycle-site, no EXE.
Previous direct-return source witness optfalse published; opttrue failed at
ordinary-terminal-field-return/physical-drift. Read-only worker traced original
bb27 read moving into predecessor bb26; final opttrue coordinates not yet measured.
Protected C/Array/S2 changes remain uncommitted and must be preserved. No Cargo
live at selection. Required call-valued result/Provided and vocabulary obligations,
production switching/retirement and whole finite pipeline remain open.

## Construction

Selection landed/pushed e857f1c68b. Worker confirmed the sole local_commit wrapper
is the safest minimal interface: it always constructs canonical check_binding,
while terminal_field_return receives a mandatory read-check callback and keeps
recorded result/direct Return equality. Three callers switched to draft None,
root's existing projection and child finishing Some(projection). No visibility
expansion or new physical map. Read origin Jump/Return remain captured even
without cleanup, so boundary.validate_complete still checks CFG/sequence.

Implemented that exact mapping. Source97018 terminal0: root and typed borrowed
child direct field result, object/null and both opts, six source-issued JSON
inputs. Source log `/tmp/hako-field-return-projection-S0-source.log`.
Empirical child read moves from bb10 (optfalse) to bb9 (opttrue), retaining
base1/dst11/canonical field0/object0. Root keeps bb9/base5/dst6/field0/object0.
C53854 terminal0: all six inputs execute with root5, object5, null7, plus every
birth-store fault and exact Normal/Fault cleanup counters. C log
`/tmp/hako-field-return-projection-S0-c-execution.log`.

Negative owner clarification (read-only worker reviewed): full child finishing
owns original read position/dst/base/field, duplicates and recorded exits.
Null-guard arm meaning belongs to the existing final borrowed-use publication
verifier, which anchors the nonnull successor and checks read dominance.
PhysicalBoundary ingress compares the edge set, so swapping then/else alone
is not its authority. No expansion of its capture or second CFG verifier.
Cargo17614 terminal101: six finishing mutations rejected; seventh guard swap
was accepted, exposing the test-owner mismatch. This is a selected test failure,
not baseline and not acceptance. Keep the six finishing negatives and test guard
swap through final ABI issuance with the same original source handoff, healthy
publication control and both optimization settings. No AlwaysTrue callback,
duplicate-validation failure or serializer-only negative counts as this proof.
Earlier Cargo47663 setup compile errors were fixed; they are not PASS evidence.

Cargo4609 terminal101: source positive and two existing field-return regressions
PASS, guard negative stops at not-final-lifecycle-view because the test omitted
formal lifecycle admission. This is test setup failure, not guard-swap evidence.
Corrected the test to use existing admit_lifecycle with the original borrowed
profile/handoff, including an unchanged reconstructed healthy control.
Six finishing mutations now PASS (`/tmp/hako-field-return-projection-S0-six-negative.log`).
Cargo90507 then tested `field_return`; log
`/tmp/hako-field-return-projection-S0-admitted-focused.log`.
This handle is terminal0; no Cargo remains live.
Selected scope guard reaches the HEAD-identical root1351 debt (whole guard red).
Scoped commit/push is complete; acceptance below is recorded.
Pre-edit selected mixed files captured in `/tmp/hako-field-return-projection-S0-before/`;
prior C/Array/S2 differences remain protected. No implementation commit existed at that checkpoint.

## Closeout evidence

Cargo90507 terminal0: field_return 4/4, including guarded publication rejection
at borrowed-use/undominated-view for both opts with an unchanged reconstructed
control. Child finishing six mutations PASS separately and in package run.
Fresh source27/27 (six direct-result inputs regenerated), boundary11/11,
borrowed-use17/17, full JSON29/29, entry21/21 PASS. Initial filters using filename
rather than module names matched zero tests; they are not PASS evidence. Corrected
filters are recorded in the `*-actual.log`, `json-full.log`, `entry-full.log` files.
All logs use `/tmp/hako-field-return-projection-S0-` prefix.
C64986 terminal0 on regenerated inputs: six Normal/Fault programs PASS;
`final-c-execution.log` confirms results5/5/7 and exact cleanup counters.
Package623 PASS/exact known3 failures, each same original boundary as S1:
ReceiverNonEscape Capture, IncompleteOrdinaryNewCoverage, BorrowedEntryEscape.
No new or unclassified red. Whole package is not green.

Selected scope pins pass; whole guard remains red at HEAD-identical root test
1351 lines (sha256 5fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467).
Rustfmt selected source/tests, Python syntax, pointer and whitespace PASS.
Selected Rust files below800 (terminal190, root_validation470, emission412,
source-parent628, private-negative130, source-child99). README selected row
explains the original/draft versus FinishedBindings owner; no new semantics.

Three production validation callers now use the existing canonical checker;
original-block-only field read lookup is removed (caller-zero). Existing
source/result/ownership contracts and independent final publication stay intact.
No C implementation, .hako app, source authority, extra map or fallback added.
Scoped mixed-file patch is derived from the pre-slice snapshots and applies to
HEAD/index; older C/Array/S2 WIP is excluded. App stops earlier at unowned
lifecycle site, so this repair does not change its observed frontier; fresh CLI
build is not claimed here. Full finite pipeline, later result/Provided and
pending C/Array/S2 landing remain open.

Scoped code/tests/owner-contract closeout landed/pushed `a47039d3fe`; remote hash
verified equal. Next required borrowed I64 call-result source D0 is selected in
CURRENT_STATE; stored-child receiver prerequisite remains separate. Goal active.
