---
Status: SSOT
Date: 2026-08-11
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

## Current Capsule

- **Current decision:** `AGENTS.md` is a compact local router; tracked SSOTs
  own durable procedure and design truth. `CURRENT_STATE.toml.work_mode` is
  the sole Fast/Design-stop/Closeout routing authority; blocker text is not a
  mode classifier.
- **Current implementation status:** the router and this tracked contract are
  synchronized; this policy does not change the active compiler lane.
- **Next ordered task:** resume only the blocker selected by
  `CURRENT_STATE.toml` after the normal pointer guard passes.
- **Production stop line:** no router or guard wording may authorize a source
  shape, production caller, fallback, or publication path.
- **Retirement finish line:** duplicated procedure and historical command
  tables are absent from the local router and current mirrors.

## Purpose

`AGENTS.md` is local AI/developer instruction material. It is intentionally
ignored by git in this repository, so durable policy must also live in tracked
docs.

This SSOT fixes how agents should read that local file without reviving old
phase-specific guidance.

The local router is intentionally compact. It keeps personality, the
current-first read order, short big-picture/design-stop reminders, and links to
owning SSOTs. Long structural rules, command tables, phase roadmaps, and
historical toolchain notes are not copied back into `AGENTS.md`; they remain in
the tracked documents named below.

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

## Daily usability contract

The entry path must be strict without becoming ceremonial. Apply these limits
to new and actively revised material.

### Three work modes

Every turn selects exactly one mode before editing. This is a routing rule,
not another task hierarchy.

The selected mode is the explicit scalar
`CURRENT_STATE.toml.work_mode = "fast" | "design_stop" | "closeout"`.
`current_blocker_token` and free-form `current_design_stop` explain the mode;
they never select it. All restart mirrors and guards must follow this field.

| Mode | Entry condition | Allowed work | Exit evidence |
| --- | --- | --- | --- |
| Fast path | source-to-Recipe mapping, owner, and failure boundary are already closed | one existing owner, one focused gate, no new authority | positive/negative evidence and the row's closeout receipt |
| Design stop | any mapping, selector, authority, failure owner, or canonical issuer is still being discovered | one compact design brief; no code, fixture, fallback, production switch, or guessed `Verified*`/`Prepared*` receipt | accepted Decision plus one bounded next slice |
| Closeout | the selected slice has been edited and tested | classify evidence, update owning docs, commit/push or retain blocker | all Done items observable; otherwise the row remains active |

The readiness sentence is:

```text
This input maps once to this Facts/Recipe, and fails at this boundary.
```

If that sentence is not true, the work is a design stop. A worker report,
local green test, or compatibility fixture never changes the mode by itself.

### 90-second routing card

The mode decision must be cheap enough to use before every edit. Ask these
questions in order:

1. Read `CURRENT_STATE.toml.work_mode`; do not classify it from token text.
   `design_stop` forbids code, fixtures, fallback, and production switches;
   `fast` permits only the selected bounded slice; `closeout` permits evidence,
   owning-doc, commit, and pointer work.
2. Can the source be stated as one deterministic
   `source -> Facts -> Recipe -> fail-fast boundary` sentence? If no, stay in
   **Design stop**. A worker may audit the premise, but a worker report is not
   implementation permission.
3. If the mapping is closed, is the change **BoxCount** (one new accepted
   shape) or **BoxShape** (same behavior, cleaner ownership)? Choose exactly
   one. A behavior-preserving refactor series may have a few commits, but it
   may not add a shape or fixture.

Use this compact brief when stopping:

```text
Decision:
Source authority + canonical issuer:
Non-authority:
Fail-fast boundary:
Smallest next slice:
Non-claims:
```

### Classification-completeness check

Every routing, claim, publication, admission, or lifecycle design card must
also include a finite state table before implementation. The table must name
every outcome, including the state that is neither selected nor rejected (for
example `Unavailable`, `Absent`, `Unresolved`, or `NoCandidate`), and bind
each outcome to its authority, pre-effect behavior, allowed terminal, and
fallback policy. A wildcard arm, `Option::None`, `unwrap_or(default)`, or
generic compatibility label may not silently merge two distinct states.

Use `LoopFamilyRowDispositionV1`'s four-way
`Candidate | Declined | Unresolved | Rejected` matrix as the reference
pattern, but choose the vocabulary owned by the current row. Reviewers must
check that every state is issued by one named owner, every negative witness
maps to exactly one state, and every state transition is exhaustive before a
focused gate is treated as evidence. If the table cannot be made finite and
authority-backed, remain at `NoSafeSlice` rather than inventing a default.

The design stop ends only after the selected slice and its explicit non-claims
are accepted in the owning card/SSOT. If a schema or operation vocabulary is
missing, record `NoSafeSlice` as a development state; do not force it into
`Candidate`/`Declined`/`Unresolved`/`Rejected` and do not publish guessed Recipe
counts.

### Source-backed receipt gate

Before adding any new semantic `Verified*` or `Prepared*` product, the brief
must name both the source authority and its canonical issuer. The issuer must
consume resolver/source capability and issue the receipt from that authority;
lexical receiver presence, names, body inference, physical MIR facts, or an
empty/default enum are not issuers. If any required Home, effect, lifecycle,
receiver-type, or ABI receipt has no issuer, keep the row at `NoSafeSlice` and
do not create a guessed product.

An aggregate may co-seal existing receipts, but may not invent a new semantic
fact while aggregating. Semantic `Pure`, Home, effect, suspension/control, and
ABI receipts must not reuse MIR `EffectMask`/`FunctionSignature` as their
authority. `NoSafeSlice` is a development state; `Candidate`, `Declined`,
`Unresolved`, and `Rejected` remain source dispositions. A parked I0 must say
whether its next slice is (a) the missing issuer design/implementation or (b)
an explicit park; it may not cross the boundary with a test-only constructor.

### Design-stop diagnosis and unblock contract

Give each design stop one primary missing-boundary class. Secondary
dependencies stay in prose rather than becoming deeper task names.

```text
SemanticAuthorityMissing
RepresentationDecisionMissing
MaterializationRelationMissing
BackendCapabilityMissing
NamedConsumerMissing
```

Apply these laws before moving a design stop to implementation:

- Split pre-effect semantic/demand products from session-local realization.
  A product issued before physical allocation owns no `ValueId` or block; a
  session product does not reclassify source meaning.
- Runtime-polymorphic representation provenance is producer-issued and must
  survive copy, rebind, merge, and PHI. Consumers never infer it from raw bits,
  names, receipt-free runtime-table probes, or metadata.
- Classify every required backend/capability cell as exactly `Direct`,
  `Checked`, or `RejectBeforeEffect`. Fallback is not a capability class.
- Backend role and acceptance authority come only from `CURRENT_STATE.toml`
  and its active card. The existence of a VM/AOT/JIT implementation, test, or
  compatibility route is evidence, not permission to add parity, a new
  feature, or a production gate to an unselected backend. A named
  nonconsumer/caller-zero fence remains binding until its owner reopens it.
- Do not implement a new receipt without its named consumer and retirement
  edge in the same bounded series. Caller-zero proof chains remain design.
- Write missing/foreign/duplicate/ambiguous/unsupported cases and, for Fault
  boundaries, primary/suppressed/no-result chronology before the positive
  implementation.
- Keep one serial authority spine in the current pointer. Independent parity,
  performance, and cleanup proofs become explicit DAG siblings after their
  shared production cutover; they are not serialized merely because the UI has
  one current row.

### Compact router

- Keep `AGENTS.md` near one screen of headings and short rules; about 80--120
  lines is the editorial target, not a semantic permission gate.
- Keep only personality, current-first order, always-on stop conditions, and
  links to tracked owners. Move detailed procedures, command tables, fixture
  matrices, and historical rationale to their owning tracked document.
- Do not make the router repeat `CURRENT_STATE.toml`, an active workstream, or
  the check-script index. A pointer is sufficient.
- Keep the router near one screen. Detailed guard procedures, historical
  command tables, and evidence matrices belong in this tracked contract or
  the owning lane SSOT.

### Shallow task names

New task tokens describe the semantic product and its current stage, not the
full ancestry of previous investigations. Prefer:

```text
FAMILY-SLICE-STAGE
```

Do not extend tokens by repeatedly appending `-S*`, `-D*`, or parent task
names. Put parentage and ordering in card metadata or the active queue. If a
new token needs more than one stable family/slice prefix plus one stage suffix,
stop and check whether the work is actually one task or an unsealed design.
Historical tokens are not renamed solely for style.

### Source split trigger

- 760 lines is the design trigger for a source file; 800 lines remains the
  hard boundary.
- At 760 lines, name the responsibility split before adding semantic growth.
  Split by owner or interface, not by arbitrary line ranges.
- Never compress formatting, merge declarations, shorten diagnostics, or
  remove explanatory structure merely to pass the line count.
- Test fixtures and generated files follow their owning policy, but generated
  status must be explicit rather than inferred from size.

### Bounded closeout rule

- A bounded row is closed only when its named positive path, negative matrix,
  line-count guard, focused gate, and same-slice README/reference receipt are
  all green.
- “Partially landed” is an implementation status, not a completion claim.
  Keep the current blocker on the row until every item in its `Done` block is
  observable.
- If a closeout reveals a missing source authority or a new source-to-Recipe
  correspondence, return to design; do not grow a deeper task suffix or add a
  repair adapter.
- A source-validation split is a structural refactor only when it preserves
  the single Facts owner and keeps Recipe keys, selectors, and physical IDs
  out of the observation layer.

### One-pass development workflow

Every bounded slice follows one finite loop:

```text
current pointer + clean tree
  -> classify BoxCount / BoxShape / Design Stop
  -> seal the smallest design brief when needed
  -> implement one responsibility and one production edge
  -> run focused positive/negative evidence and the reusable guard
  -> update the owning README/reference, then commit/push
  -> update current pointers only when the active blocker or lane changes
```

If source membership, source-to-Recipe correspondence, authority, or failure
ownership is still being discovered, the slice is a design stop rather than an
implementation row. Do not add fixtures, fallback, or a compatibility adapter
to make an unresolved mapping look green. A row that cannot satisfy its Done
block keeps its blocker and next action; it is not closed as “partially landed.”

### Worker consultation contract

Workers are a bounded review resource, not a second implementation stream.

- Keep exactly one serial authority/current-execution spine. Up to two
  workers may inspect independent premises concurrently, but under the current
  policy they remain read-only and the primary agent integrates one Decision.
- Do not infer ceremony from a task suffix. T0 is a fully censused private
  mechanical move with unchanged authority/identity/failure/ABI/route; T1 adds
  one bounded witness to an existing owner; every unknown or changed truth,
  failure, publication, lifetime, rollback, or schema boundary is T2.
- An audit finding interrupts the unlocked product row only when a named live
  reproducer reaches effect/publication, the exact selected cutover reaches
  the owner and its unchanged gate fails, or that path has reachable UB,
  corruption, or irreversible-effect risk. Otherwise park it and return to
  the product row after the declared closeout.
- A second code-writing lane would change the durable worker and integration
  policy. It is not authorized by independent T0 work and must not be
  improvised in a shared worktree.

- Use a worker for a genuinely difficult design/authority audit or an
  independent premise review; mechanical T0 work does not need one.
- The worker receives a read-only question covering source authority,
  non-authority, candidate boundary, fail-fast owner, explicit non-claims, and
  acceptance evidence. It must not edit the same files as the primary agent.
- For a new semantic receipt, the worker must also audit issuer availability
  and distinguish semantic facts from physical MIR projections. A worker may
  report that an issuer is missing, but that report never authorizes a guessed
  receipt or implementation.
- The primary agent distills the report into one Decision in the existing card
  or owning SSOT. Do not copy the full report into a new task. Conflicting
  reports keep the design stop active until one premise audit resolves them.
- After a design Decision closes, implement the smallest row, run the focused
  gate, and commit/push before opening another design question unless the user
  explicitly requests a parked consultation.
- Before requesting a worker, record one sentence in the active card explaining
  why the slice is not Fast path. The worker supplies premise evidence only;
  the primary agent owns the Decision, task boundary, implementation, and
  closeout.

### Guard result classes

Use the three result classes owned by
`current-docs-update-policy-ssot.md`: current-change failure, known baseline
debt, and informational census. An unclassified failure is blocking, and an
agent may not invent a waiver or turn census output into completion evidence.

### Local Cargo resource-safety contract

One repository checkout and shared `target/` directory may have at most one
agent-started top-level Cargo build, check, or test command in flight.  Before
starting Cargo, inspect the agent's background terminals; wait for the active
Cargo command or stop a redundant one.  Do not use background terminals to
race equivalent focused gates.

Agent-driven development commands use the quick profile, one library target
when applicable, and an explicit four-job ceiling:

```bash
CARGO_BUILD_JOBS=4 cargo check
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib <filter>
```

Use `--exact` only with the complete test path.  Omit `--nocapture` unless the
test output is required evidence.  Do not change `RUSTFLAGS` while another
Cargo command is active: a different flag set creates a separate artifact
graph and may start a full rebuild beside the first one.  In particular, do
not launch an `RUSTFLAGS=-Awarnings` retry merely to hide a large warning
transcript; wait for the current build and keep the existing flag set.

The repository's configured parallelism and release profile remain available
for deliberate standalone use.  `--release` is reserved for an active card's
explicit final evidence, not ordinary iteration.  If a required gate itself
spawns Cargo, it is the sole top-level Cargo owner until it exits.  Stop and
report resource pressure instead of opening another build when aggregate
host RSS (the agent plus Cargo/rustc children) approaches 8 GiB, swap grows
continuously, or the terminal has stopped producing progress.  This is a
hard safety boundary: the 2026-08-18 incident was confirmed by the kernel as
`global_oom` killing the Codex process while overlapping Cargo/rustc workers
were resident, not as a Rust panic.

The checked-in `tools/checks/dev_gate.sh` applies the same four-job ceiling to
all Cargo steps it launches (while respecting a smaller caller value), so the
single-entry gate cannot silently restore host-wide parallelism.

#### Forced-termination recovery

`Waiting for background terminal` is an orchestration state, not a successful
Cargo result.  If the terminal reports multiple background terminals, or a
focused test reports `0 passed`/`0 tests`, treat the run as invalid evidence:
do not start a second Cargo command and do not infer that the test is green.
First inspect the process table and stop or wait for every redundant
`cargo`/`rustc` child, then rerun one complete test path serially with the
existing flag set.  A `--nocapture` transcript full of warnings is not a
reason to launch an `RUSTFLAGS=-Awarnings` retry.  After an interruption or
forced termination, the restart order is:

```bash
git status -sb
ps -eo pid,ppid,stat,etime,pcpu,pmem,args | rg '[c]argo|[r]ustc|[s]ccache|[r]ustdoc' || true
bash tools/checks/current_state_pointer_guard.sh
```

Only after the process check is empty may the single quick Cargo gate resume.
Record the command and its nonzero executed-test count as the evidence; a
zero-match filter is a command-selection error, not a passing test.

When the cause of an unexpected termination is unclear, inspect the kernel
record before retrying so an OOM kill is not mistaken for a test failure:

```bash
dmesg -T 2>/dev/null | rg -i 'oom|out of memory|killed process' | tail -40 || true
journalctl -k -b 2>/dev/null | rg -i 'oom|out of memory|killed process' | tail -40 || true
```

### Active SSOT current capsule

Use the five-field Current Capsule defined by
`current-docs-update-policy-ssot.md` for every new or materially revised active
SSOT. It summarizes only that document's authority and never copies global row
tokens or landed history.

### Closeout minimum

A row is not complete merely because its code compiles. The smallest closeout
receipt names the positive path, the negative or rejection boundary, the
reusable guard, the README/reference update when a contract changed, the
commit, and the next blocker. Missing evidence keeps the row active; it does
not justify a deeper task suffix.

### Baseline-red classification

When a focused or package gate is red during a behavior-preserving refactor,
rerun the same command at the parent commit before changing the scope. A
failure that reproduces at the parent is **known baseline debt** and must be
recorded in the active card with the parent commit and exact test; it is not a
permission to repair unrelated semantics in the refactor series. A failure
that does not reproduce at the parent is a current-change failure and blocks
closeout. An uncategorized red is always blocking. Never hide a red by weakening
the test or by calling it an expected baseline without this comparison.

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
| superseded or historical material | `docs/development/current/main/design/archive/**`, `docs/development/archive/phases/**`, `docs/archive/**`, or the owning retirement SSOT |
| local personality and always-on routing rules | `AGENTS.md` only |

When implementation changes a language, ABI, lifecycle, diagnostic, or public
compiler contract, update the owning `docs/reference/**` page and affected
module README in the same implementation slice. Do not defer that work to an
unnamed later documentation task.

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
Do not copy the current backend priority into the root router; keep only the
durable rule that the current pointer and active card select backend roles.

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

The design brief is the only permission boundary. A worker's green probe,
existing lowerer, or compatibility fixture cannot authorize implementation
until the brief names the source authority, non-authority, fail-fast boundary,
recommended slice, and non-claims.

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

### Semantic-program boundary gate

Before opening a physical or production row, the semantic program must have
one issuance point. That issuance point co-seals the resolver-issued source
context, the source-bound Core product, all input/item/carrier relations, and
the Loop continuation authorized by that Core's own JoinSig. A caller may not
construct these verified products independently and pair them later by matching
owner or Loop keys. If the co-seal is missing, the work is a design stop even
when each lower-level product is individually green.

The authority split is fixed:

```text
Facts/observation: source roles, BindingRef, exact source sites, coverage
Recipe producer: Recipe/JoinSig keys and deterministic role -> key mapping
JoinSig: logical transfer/edge authorization
Physical layout: segment placement only; never edge or merge inference
```

Facts must not carry Recipe keys, selector cursors, BasicBlock/Value IDs, or
physical route identity. Layout may consume JoinSig transfer receipts, but it
must not rediscover predicate/exit/backedge meaning from Recipe text. A new
boundary owner or a violation of this split requires a design brief before code
or fixtures are added.

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

Archive destination correction:

- New historical phase moves use `docs/development/archive/phases/<phase>/`.
- `docs/development/current/main/phases/archive/**` is a transitional
  compatibility root only; do not add new archive content there.
- Historical design material may use
  `docs/development/current/main/design/archive/**` until a narrower retirement
  owner supersedes it.

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
