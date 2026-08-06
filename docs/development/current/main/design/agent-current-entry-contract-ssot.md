---
Status: SSOT
Date: 2026-08-06
Scope: `AGENTS.md` の current-first 読み順と historical section の扱い。
Related:
  - AGENTS.md
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/hako-optimization-toolbox-usability-ssot.md
  - docs/tools/check-scripts-index.md
---

# Agent Current Entry Contract

## Purpose

`AGENTS.md` is local AI/developer instruction material. It is intentionally
ignored by git in this repository, so durable policy must also live in tracked
docs.

This SSOT fixes how agents should read that local file without reviving old
phase-specific guidance.

## Decision

Read current-state documents first:

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `CURRENT_TASK.md`
3. `docs/development/current/main/05-Restart-Quick-Resume.md`
4. `docs/development/current/main/10-Now.md`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
6. `docs/development/current/main/DOCS_LAYOUT.md`
7. `docs/development/current/main/design/agent-current-entry-contract-ssot.md`

Then read `AGENTS.md` for personality, always-on engineering rules, and
stop-the-line policy.

## Document placement contract

`AGENTS.md` is a local router, not a second taskboard or design registry.
Keep each kind of truth in one durable home:

| Truth | Durable home |
| --- | --- |
| current lane, blocker, latest card | `CURRENT_STATE.toml` |
| restart pointer and one-screen status | `CURRENT_TASK.md`, `05-Restart-Quick-Resume.md`, `10-Now.md` |
| active multi-day execution brief | `workstreams/*.md` |
| reusable compiler/language policy | `design/*-ssot.md` |
| source-language contract | `docs/reference/**` (with an explicit `Decision:`) |
| investigation and bounded census | `investigations/` |
| check/guard entrypoints | `docs/tools/check-scripts-index.md` and the owning reusable guard |
| repository commands and procedures | the owning tool/module README, `docs/tools/check-scripts-index.md`, or the active card |
| landed implementation detail | source/tests and git history |
| superseded or historical material | `docs/archive/**` or the owning retirement SSOT |
| local personality and always-on routing rules | `AGENTS.md` only |

When an instruction grows beyond a short routing rule, move its durable
content to the matching tracked home above and leave a pointer in
`AGENTS.md`. Do not copy the same rule into `CURRENT_TASK.md`, a workstream,
and a phase log merely for visibility.

Repository-wide build, smoke, backend, or environment command tables do not
belong in the root instruction router. Keep the executable command at its
own tool README or active card, and keep the check index as the single human
entrypoint. This prevents a historical command from looking current merely
because it was copied into `AGENTS.md`.

When the active lane is MirBuilder in-place replacement, read the
`mirbuilder_north_star` path from `CURRENT_STATE.toml` before selecting a cell.
The replacement method and current row are subordinate to that final
production-authority goal.

## Big-picture-first contract

The local `AGENTS.md` may carry a short `Big-Picture First` reminder, but the
durable rule is here: a passing test or a small accepted row is not the goal by
itself. Before implementation, identify the complete authority chain:

```text
natural source
  -> exact semantic membership
  -> AST-free Facts
  -> portable product / Recipe
  -> Verifier
  -> sole physical owner
  -> one publication/commit boundary
  -> legacy authority retirement
```

If the mapping between two adjacent products is still being discovered in
code or tests, return to a design stop. Do not deepen the task suffix, add a
compatibility adapter, or treat a local green result as a production claim.
The active row must name its finish line, production switch point, deletion or
retirement boundary, and explicit non-claims. Keep the execution sequence a
finite shallow ladder; repeated sub-suffixes are evidence that the premise or
schema needs correction, not a progress metric.

The source-to-Recipe correspondence gate is detailed in
`recipe-first-entry-contract-ssot.md`, and the final authority chain is owned
by `mirbuilder-final-pipeline-ssot.md`. The root instruction file should point
to those documents rather than duplicate their design prose.

For optimization work, the durable toolbox entry is:

```text
docs/development/current/main/design/hako-optimization-toolbox-usability-ssot.md
```

Local `AGENTS.md` may link to that document, but this tracked SSOT is the
durable pointer because root `AGENTS.md` is ignored by git.

If a fixed phase name, old backend preference, or historical runtime line in
`AGENTS.md` conflicts with `CURRENT_STATE.toml`, the current-state SSOT wins.

## Unsupported Pure Shape Triage

When a normal build log reports:

```text
unsupported pure shape for current backend recipe
```

read the inline hint fields on the same error first:

```text
first_block first_inst first_op owner_hint reason callee_symbol next_check_hint
```

If those fields identify the blocker, continue with that owner directly. Rerun
with `NYASH_LLVM_ROUTE_TRACE=1` only when the inline hint is still insufficient
and the detailed `[llvm-pure/unsupported-shape]` inventory is needed.

This diagnostic is a triage boundary inventory. It must not become C-shim shape
policy, route selection, or `.hako` workaround logic. If `callee_symbol`,
`first_op`, or `next_check_hint` are still absent/unknown, the next slice is to
shorten the diagnostic distance before attempting a semantic fix.

## Design Consultation Stop

When `CURRENT_STATE.toml` or the active task-order SSOT marks the current
blocker as a selection, design, consultation, or policy-boundary step, agents
must not silently continue into implementation.

Instead, first produce a compact design-stop brief:

```text
source authority
non-authority
fail-fast boundary
candidate slices
recommended next slice
explicit non-claims
```

Do not promote a lower-level green fact into a higher-level policy claim. For
example, CoreContext generator scalarization does not prove
`MirBuilder::next_value_id` allocation policy; the latter also involves
function-local allocation, reserved ValueId skipping, parameter reservation, and
module-global fallback.

If a user-scoped Codex goal explicitly says to stop at design consultation, the
goal should be considered complete at this stop point after the brief is ready
and the worktree is clean.

### Source-to-Recipe implementation gate

For any row crossing source, Facts, Recipe, verifier, or physical completion,
the design-stop brief must also satisfy the semantic mapping completion gate in
`recipe-first-entry-contract-ssot.md`. In particular, name every product layer
that is called “Recipe”, prove that the final portable schema can represent the
exact carrier/merge/tail semantics, and identify the sole selector, key issuer,
physical identity owner, and commit owner.

If that correspondence is still being discovered through tests or code, the
row is not an implementation row. Return to BoxShape/design, keep production
callers at zero, and do not deepen the task suffix. AST retagging, synthetic
source evidence, passing compatibility fixtures, or an old Builder recipe do
not substitute for a natural-source-to-portable mapping. A planned legacy
cutover must also classify every currently accepted input before deleting the
old authority.

### Premise-reset circuit breaker

Three consecutive `NoSafeSlice` / `NoStandaloneRow` outcomes for the same
responsibility are not permission to run a fourth edge census. They mean the
closed question may be valid under a wrong premise.

Stop the selector and write one premise audit inside the existing active card:

```text
semantic unit definition
exact body/window membership
all authoritative classifier/partition arms
transferred and opaque subtrees
what the types structurally require (not what their names suggest)
one counterexample fixture
```

Read the complete producer/classifier match before drawing the boundary.
Historical docs, type names, and a partial `rg` result are not substitutes.
Before resuming the same scope, obtain one independent open-question review
when another worker/reviewer is available.

Resume only when the definition maps to every classifier arm, the
counterexample is fixed, and a named production consumer plus old edge are
known.

Repository-wide census must be resource-bounded. Prefer static search; do not
reuse benchmark, allocator, or proof harnesses for syntax inventory. External
process scans default to serial and may use at most two workers. Run one item,
then a small sample, then print the target count before a full scan. Stop
immediately if child processes exceed four or aggregate RSS exceeds 8 GiB.

## Ceremony Tier Selection

Before opening a new design consultation, classify the route/stage cell using
the ceremony tiers, batch-proof trigger, and sunset requirements in
`current-docs-update-policy-ssot.md`. In particular:

1. reuse of an already-proven owner-chain pattern is mechanical fast-path work;
2. a new source authority, identity issuer, physical owner, publication
   terminal, failure owner, or policy boundary is a design-stop consultation;
3. write the active card's proof-budget fields before adding scaffolding;
4. every fast-path proof still needs a focused fixture, an existing batch/lane
   guard assertion, and a sunset reference.

After selection, write only the four-block Minimal Execution Brief from the
current-docs policy. Do not turn a worker report or consultation answer into
the execution card. Detailed types, fixture matrices, guard strings, LOC
forecasts, and rejected alternatives remain in code, tests, the reusable
guard, or a genuinely durable design SSOT.

## Historical Sections

Sections about these topics in `AGENTS.md` are historical unless the active
card explicitly reopens them:

- Phase-15 / PyVM development flow
- Cranelift/JIT branch purpose
- old feature-addition pause until VM bootstrap
- old fixed selfhost gate examples
- old PyVM dev helper environment setup

Do not retain or re-add their command tables, fixed priorities, environment
recipes, or short-term roadmaps in the local current-entry file. Keep one
compact historical pointer to the tracked archive/retirement/reference docs;
history belongs there, not beside always-on instructions.

The root file should remain a compact policy router. When an old operational
section is removed, do not copy its prose into another current mirror merely
for traceability.

The following local sections are specifically retired from current guidance:

- fixed Phase-21.5 perf command ladders;
- Phase-26-H JoinIR roadmap prose;
- generic Codex async/concurrency recipes;
- per-card feature-addition or box-count essays that duplicate the current
  docs-update, recipe-first, or compiler-expressivity SSOTs.

Their replacement pointers are the current perf-owner SSOT, JoinIR
architecture/recipe SSOT, the active card's requested tools, and the
docs-layout/update policy respectively. A command may remain in a tool's own
README or an active card when it is still an executable acceptance criterion;
it must not be copied back into the root instruction router.

## Current Guard/Proof Entry

Current guard/proof entrypoints are listed in:

```text
docs/tools/check-scripts-index.md
```

Manifest runner pilots keep stable shell entrypoints:

```text
tools/checks/run_row_guard.sh
tools/checks/run_proof_app.sh
```

Their shared implementation is:

```text
tools/checks/lib/manifest_runner.py
```

These pilots are local-run/index-listed unless a later card explicitly promotes
them into `dev_gate.sh` or allocator-wide.

Test-only Rust authority witnesses must have a physical test boundary such as
`*_tests.rs` (or an equivalent dedicated test directory). Do not rely only on
an enclosing `#[cfg(test)]` when file-level authority guards classify source
producers and callers. Keep the logical module path stable with `#[path]` when
needed, and update current/reference path claims in the same change.

## Update Policy

Do not update `AGENTS.md`, `CURRENT_TASK.md`, `10-Now.md`, restart mirrors,
phase README, or taskboards for every landed card.

Update `AGENTS.md` only when root AI/developer instruction policy changes.
When that happens, update this tracked SSOT and the current docs layout/update
policy docs in the same slice.

## Non-Goals

- no physical archive/move of local `AGENTS.md`
- no attempt to make ignored root instruction files versioned
- no per-card landed history in `AGENTS.md`
- no new guard wiring
