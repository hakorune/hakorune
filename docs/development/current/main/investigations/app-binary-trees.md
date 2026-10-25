# APP-binary-trees
Status: selected app; WIP organization first; EXE not reached. Date: 2026-10-25
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [fixed smoke](../../../../../tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh).

## Current stop
- Fresh binary-trees smoke now stops earlier at `lexical-instance-call/source-pending`, recursive `BinaryTreeBuilder.make` (owner slot5 Body(4)/Initializer(0)), before EXE. The two final `object-field-read-definition-invalid` errors are historical best evidence, not the current stop after stricter source proof. Checked stored-child reads remain owed later.
- Fresh full-tree CLI build PASS (51.78s); explicit quick CLI/runtime fixed suite 6/11 in39.924s, five compile FAIL (json, boxtorrent, binary-trees, mimalloc, allocator-stress). No new acceptance PASS; the previous 9/11 result belongs to an earlier state.
- Mimalloc's best intermediate stop was `published-lifecycle/borrowed-use/guarded-call-correspondence`. After stricter scalar proof, latest reported stop is `ordinary-new/borrowed-object/forward-source-identity` (slot37 Body(5)); original entry-formal read proof remains owed.
- JSON last source stop: `borrowed-entry/source-use-completion-pending` (owner slot22). Restore shared app acceptance before extending object-field operations.

## Scope and retained WIP
- Keep all `.hako` inputs and existing acceptance/negative boundaries unchanged; no fallback, retry or app-specific route.
- Source/result work retains computed Return source demands, original source SUM recomputation/Core proof identity, exact actual plans and parameter domains.
- Object Call selection uses original owned-object input before Dynamic selection. Loop result Homes, Normal/Fault cleanup, exact prefix liveness and independent final Call coverage remain connected in WIP.
- Other WIP includes stored receiver parent lifetime, Return-owned Call/field inventories, structural Jump contraction and nested Loop copy dependencies.
- Scalar source loans must distinguish reaching-point evidence from formal-wide class evidence. The candidate binding is independently matched to the immutable original argument by `bind_original_argument_plans_v1`; this check must remain on both SourceObject and SourceInstance completion paths.
- New Scalar no-read handling remains unlanded with the source group; it does not itself grant a receiver, Home or execution permission.

## Verified WIP organization
- Final borrowed-call consumers now record the same caller-entry-rooted Scalar forward in ordinary and guarded scans; correspondence, guard dominance and instruction order remain checked. Isolated HEAD plus only those three files: existing borrowed-use family 27 PASS; committed.
- Constructor plans now finish after their original lexical child Call slots. Isolated pre-change constructor Unit publication FAIL; with only the order repair and method moves, the existing discard family is 4 PASS; order repair committed separately.
- Pure method moves retain HEAD bodies: root print/terminal/census helpers, entry receiver-class lookup, and lexical copy-dependency helpers move to existing print/validation/entry-values/normal-projection owners. Return filtering, domain widening and Loop tree traversal stay outside this refactor; no new module.
- Isolated move/order tree: borrowed-entry family 21 PASS; three print cases PASS. The fourth print case fails the same `outer-args` assertion before and after (None versus empty carriers); this existing red remains open. Terminal argument focused result is recorded with the refactor commit.
- Broader lexical-family comparison: isolated committed source base 296 PASS/60 FAIL; initial WIP299/57 had20 new failures and23 fixes. Separating computed result facts from typed-only input admission restores12 healthy strict/nested cases: current WIP311/45,8 new failures versus base. A lower total FAIL count is not acceptance. Source coupling remains uncommitted.
- Of the8 new failures, A/B conflict stores only refused Err rows (no successful result authority); several others pin newly expanded source shapes or earlier refusal diagnostics. One actual unfinished boundary is the captured-local static point-premise fixture: input-use captures prevent completion. Do not weaken its cohort check or ignore new reds.
- No new tests, fixtures, files, hooks or checkers. Temporary diagnostic edits were removed; shared target/quick was reused sequentially and the isolated review worktree was removed. Isolated and full-WIP binary identities differ.

## Open verification and next work
1. Continue organizing the coupled source/result/Loop WIP; close the8 new lexical failures, distinguishing safe source-only refusals/stale pins from the static capture completion boundary. Only validated independent changes may be committed.
2. Close source-backed entry-formal read evidence and restore mimalloc/allocator/JSON; then continue binary-trees checked stored-child reads through the sole physical owner and final consumer.
3. Existing broader reds remain open: scalar-expression malformed-source fixture, initializer alias/source provenance, stored-child result diagnostic, physical empty-carrier mutation, and previous full-lib failures/stack abort. Attribute against an isolated baseline before changing assertions.
4. Use explicit quick CLI/runtime paths for app evidence; rerun the fixed suite after source/physical shared changes. Do not count a marker or SKIP as EXE success.
