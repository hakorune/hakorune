Status: parked correctness/compile-cost follow-up SSOT; selection remains owned only by CURRENT_STATE.toml
Date: 2026-08-22
Parent: `CURRENT_STATE.toml` and `perf-owner-first-optimization-ssot.md`
---

# MirBuilder post-audit follow-up queue

This queue records the concrete issues confirmed by top-down review and the
2026-08-22 production-reachability and compile-cost audits. It never selects
the current row: read `CURRENT_STATE.toml` for that. These rows do not
authorize an A/C bypass, a fallback, a production switch, or an optimization
selected only from source appearance.

| Row | Priority | Owner | Selection boundary |
| --- | --- | --- | --- |
| `MIR-RESULT-DISCARD-CENSUS-D0` | Medium-High | `src/mir/builder` policy | read-only census before any lint/guard rollout; [detail](./mirbuilder-result-discard-policy-d0-2026-08-21.md) |
| `MIR-ASSIGNMENT-RELEASE-FAILFAST-I0` | High | `assignment_lowering.rs` | independent correctness cell; [detail](./mirbuilder-assignment-release-failure-atomicity-i0-2026-08-21.md) |
| `MIR-RESULT-DISCARD-GUARD-I0` | Medium-High | `tools/checks` | narrow multiline-aware physical-writer guard; [detail](./mirbuilder-result-discard-guard-i0-2026-08-22.md) |
| `MIR-COMPILE-COST-BASELINE-P0` | Parked prerequisite | existing compile timing/scaling tools | select before claiming any compiler-speed keeper |
| `MIR-EMIT-DEBUG-POLICY-SNAPSHOT-D0` | High confidence | config ingress + Builder session | choose a request/session owner; a process-global `OnceLock` is not accepted by source inspection alone |
| `MIR-EMIT-MOVE-COMMIT-R0` | High confidence | `builder_emit.rs` | after debug-policy ownership is fixed; do not overlap a semantic writer row |
| `MIR-DEBUG-PAYLOAD-LAZY-P0` | High confidence | unified-call observer ingress | gate candidate projection and JSON construction before work, preserving observer output exactly |
| `MIR-LOCAL-SSA-PREPARED-OPERAND-D0` | Medium-High | `builder_emit.rs` + `ssa/local.rs` | design the prepared/legacy boundary and function-owned definition index before implementation |
| `MIR-PHI-ANALYSIS-BATCH-D0` | Medium-High | PHI materialization/finalization | name a mutation-stable analysis batch before caching or deleting a repair pass |
| `MIR-POSTPROCESS-WALK-CENSUS-D0` | Medium | semantic refresh + old/shared finish owners | count actual block/instruction visits and caller classes before one adjacent-wave fusion is considered |
| `NORMAL-ROOT-AST-MOVE-D0` | Medium | normal source package -> root work plan | remove the one production root AST deep clone only after a move/loan boundary is accepted |
| `SCRIPT-NEUTRAL-LOAN-ROWS-D0` | Medium-Low | neutral Script source issuer | co-lend one borrowed Program-row view to existing subissuers; do not merge their authorities |
| `USING-TEXT-MERGE-SOURCE-GRAPH-D0` | Medium | using resolver/text merger | issue one invocation-local path/content/strip graph; no stale global file cache |
| `INCREMENTAL-COMPILE-AUTHORITY-D0` | Future | source/artifact cache authority | open only after a measured whole-compile owner and stable cache identity/invalidation contract exist |
| `MIR-EMIT-CANONICAL-STRICTNESS-D0` | Accepted | canonical Loop physicalization + ordinary writer | C-prime same-block Decision and P0/CONNECT0 order are fixed; [detail](./mirbuilder-emit-canonical-strictness-d0-2026-08-22.md) |
| `MIR-LOOP-OPERATION-EMITTER-SPLIT-S0` | Selected prerequisite | `loop_recipe_physicalizer/operation_emitter.rs` | behavior-neutral pure-operation owner split before semantic growth; [detail](./mirbuilder-loop-operation-emitter-split-s0-2026-08-22.md) |
| `MIR-LOOP-COMPARE-SAME-BLOCK-P0-CONNECT0` | High, ordered | canonical CFG/SSA, Loop ledger, ordinary writer | caller-zero only: session target -> same-block operands -> reservation -> strict writer -> CONNECT0; production I0/R0 remains a later design |
| `MIRBUILDER-STRUCTURE-BASELINE-CENSUS-P0` | Parked BoxShape | module/test ownership and live README | validate the structure review before any move; [detail](./mirbuilder-structure-refactor-queue-d0-2026-08-23.md) |
| `MIR-RECIPE-VERIFY-MOVE-R0` | Medium | three production Recipe producers | only after their selected profiles remain production-reachable |
| `MIRBUILDER-BARREL-RESPONSIBILITY-CLEANUP-D0` | Medium | `builder.rs` | after relevant production callers are caller-zero |
| `MIRBUILDER-INIT-RESPONSIBILITY-CLEANUP-D0` | Medium | `builder_init.rs` | after the barrel/owner census is accepted |
| `MIRBUILDER-MAIN-INTEGRATION-CLOSEOUT` | Operational | branch/pointer SSOT | after the active branch has an explicit integration step |

## Compile-cost observation SSOT

Do not add a second timing framework or an always-on benchmark gate. The
existing owners are:

```text
src/mir/compile_timing.rs
  -> opt-in stable per-stage timing lines

tools/perf/mir_compile_scaling.py
  -> sole opt-in 50/100/250-method scaling runner
```

The runner currently records end-to-end elapsed time and internal stages. It
does not own a checked-in current baseline, RSS, compiler-process instruction
count, or CI regression threshold. Rust cold-build history is not Hakorune
source compile-time authority.

When a compile-cost row is selected, use a prebuilt release binary and the
existing runner. Five repetitions and median wall time are advisory evidence,
not a noisy CI threshold. Measure peak RSS for one fixed large shape only when
allocation ownership is under review; use hardware instruction counts only
when available and never make them a portable correctness gate.

Structural acceptance remains primary:

```text
duplicate production observer removed
same MIR and same error boundary
new always-on env lookup / allocation / full scan = 0
focused correctness gate green
before/after timing recorded only for the selected row
```

## 2026-08-22 compile-cost audit disposition

The latest review is directionally useful, but source facts and performance
claims must remain separate.

| Review claim | Verified source fact | Queue disposition |
| --- | --- | --- |
| 7-9 env lookups per instruction | with debug flags absent, ordinary `emit_instruction()` reaches 7 process-env reads, Copy reaches 8, and Call reaches 9; short-circuiting changes the count when flags are enabled | snapshot D0 before any cache implementation |
| dead debug clones and instruction clone | `_dbg_fn_name` and `_dbg_region_id` are unused values; function name and final `MirInstruction` are cloned on every successful ordinary emission | move-commit R0 |
| debug payload is prepaid | `resolve.try` builds receiver text, candidate `Vec<String>`, and JSON before the debug hub checks `NYASH_DEBUG_ENABLE`; header lookup visits every symbol | lazy payload P0; possible `calls * symbols` growth is a hypothesis until scaling evidence |
| 50-70 full MIR walks | the semantic refresh stack contains 65 named refresh invocations and 13 post-fixpoint refresh calls; many are separate scans, but not every function is proven to walk every instruction | walk census D0; do not claim 30-40% yet |
| refresh runs twice | full semantic refresh runs once after RC and a second time only when callsite canonicalization changes rows; pre-verify contract refresh is a narrower owner | census caller/transition classes before fusion |
| one root AST deep clone | selected-normal root materialization clones `source_ast` to own `lowering_statements` while the source package remains borrowed | AST move/loan D0 |
| Script source is re-walked | the neutral issuer performs at least three full `loan.statements()` traversals: instance transfer, constructor source, and final window; the bounded composite loan is separate | one borrowed-row projection D0; semantic issuers stay separate |
| prelude read/strip repeats | main using-strip runs twice; a direct dependency prelude can be read and stripped during profile DFS, import DFS, and final merge | invocation-local source-graph D0 |
| no compile-time tooling | false as stated: stable opt-in stage timing and a V0 scaling runner already exist; they lack repeated samples/median, binary identity, RSS, and a portable regression policy | evolve the existing baseline owner; do not create a second framework |
| 2-4x expected speedup | no measurement supports this number | reject as an acceptance claim |

Keeper finding: the audited selected path did not expose a second parse,
resolver rerun, or catalog reconstruction owner. Existing semantic products are
transported by move, borrow, or shared ownership. Do not merge those semantic
issuers merely to save a traversal, and do not add a cache task without a
measured duplicate computation and a named invalidation authority.

This audit does not reopen performance while `CURRENT_STATE.toml` says it is
closed. It only provides bounded rows for a later explicit selection.

The repeated 2026-08-22 top-down review adds no new compile-cost row. Its
recommendations map to the existing baseline, session debug-policy, lazy
payload, move-commit, walk-census, source move/loan, source-graph, and future
incremental-authority rows above. In particular, a process-global `OnceLock`,
blanket pass fusion, and the unmeasured `2-4x` speed claim remain rejected.
This keeps the queue thin: measurement selects one owner first, then one
behavior-preserving seam is changed and remeasured.

## Parked compiler-cost order

```text
MIR-COMPILE-COST-BASELINE-P0
  -> select exactly one measured owner from:
       MIR-EMIT-DEBUG-POLICY-SNAPSHOT-D0 / P0
       MIR-EMIT-MOVE-COMMIT-R0
       MIR-DEBUG-PAYLOAD-LAZY-P0
  -> rerun the same immutable observation batch
  -> only then consider MIR-POSTPROCESS-WALK-CENSUS-D0
  -> source reobservation rows remain independently selectable
```

The three small emit candidates may eventually land as separate commits in one
measurement series, but each keeps its own revert boundary. A single source
review does not authorize bundling them or claiming their combined speedup.

## `MIR-EMIT-DEBUG-POLICY-SNAPSHOT-D0`

Do not replace every env helper with a process-global `OnceLock`. Tests and CLI
entrypoints intentionally set and restore process variables inside one process;
global first-read caching can freeze the wrong configuration for later
builders.

The D0 must choose one owner created after CLI/TOML env bootstrap and before
the first MIR instruction, preferably a compile- or Builder-session policy.
The hot writer may borrow booleans from that policy but cannot read process env
or reinterpret strings.

Acceptance for a later P0:

```text
emit_instruction process-env reads in debug-OFF session = 0
two independently created test sessions may observe two explicit env setups
one session's policy cannot change mid-emission
debug/strict/error behavior is byte-for-byte equivalent at selected fixtures
no semantic feature flag is moved into debug policy
```

Non-claims: no repository-wide env cache, no runtime flag ABI change, no
removal of compatibility aliases, and no compile-speed keeper before baseline.

## `MIR-DEBUG-PAYLOAD-LAZY-P0`

The observer gate must run before payload construction. Use a closure or named
prepared observer policy so debug OFF does not build method candidates,
receiver strings, JSON, function-name strings, or region strings. The existing
debug hub remains the sole event writer and emitted JSON stays unchanged when
enabled.

Acceptance:

```text
debug OFF candidate visits = 0
debug OFF serde_json payload constructions = 0
debug ON resolve.try / resolve.choose output parity = exact
method resolution and target selection are unchanged
no observer result becomes semantic authority
```

## `MIR-POSTPROCESS-WALK-CENSUS-D0`

Blanket pass fusion is rejected. Several refresh functions own different
facts, mutation boundaries, or post-fixpoint dependencies. The shared
`run_postprocess_stages()` kernel is a Keeper, while the old
`finish_built_module()` schedule and route-specific physical adapters still
need caller classification.

The D0 adds observation counters, not another validator:

```text
route/family
stage invocation count
function count
block visits
instruction visits
metadata-only / MIR-read / MIR-write class
whether a later stage consumes the intermediate product
```

A later BoxShape may fuse only one adjacent pair with the same input authority
and mutation-stable boundary. A changed callsite still requires its dependent
refresh; an unchanged callsite must not pay the conditional second refresh.

## Source reobservation rows

`NORMAL-ROOT-AST-MOVE-D0` must choose between consuming/splitting the existing
source package and a scoped HRTB work-plan loan. It may not keep an AST
reference in a long-lived Recipe or reconstruct parser identity.

`SCRIPT-NEUTRAL-LOAN-ROWS-D0` may project one borrowed row slice/cursor once
inside the existing HRTB loan and pass it to the composite, instance,
constructor, and window subissuers. Those issuers retain separate meanings and
errors; this is shared traversal input, not merged authority.

`USING-TEXT-MERGE-SOURCE-GRAPH-D0` should make the DFS issuer own canonical
path, file content, stripped text, imports, and child paths for one invocation.
The final merger consumes those rows without rereading files. A process-global
content cache is outside the slice because file invalidation and test mutation
are not yet sealed.

`INCREMENTAL-COMPILE-AUTHORITY-D0` remains future-only. Its prerequisite is a
stable source/config/toolchain identity, dependency graph, invalidation law,
and atomic artifact publication; collector presence alone is not cache
authority.

## `MIR-EMIT-MOVE-COMMIT-R0`

The sole physical writer is production-reachable and currently performs work
on every successful emission:

- unused current-function and region debug clones near function entry;
- MethodCall `Callee`, owner/method strings, and argument-vector cloning while
  rebuilding the receiver-bearing Call;
- another unconditional function-name clone;
- `MirInstruction::clone()` at the final block commit;
- repeated debug/env queries on the per-instruction path (removed by the
  preceding policy-snapshot row, not by an ad hoc `OnceLock` here).

This row extracts only the post-commit facts needed by metadata/PHI observers,
then moves the instruction into the block once. The accepted debug policy is
borrowed or debug/error work is constructed only inside its selected branch. The
sole writer, validation order, receiver/PHI behavior, predecessor updates,
metadata, and emitted MIR remain unchanged.

Acceptance:

- successful non-PHI emission commits one moved instruction with no full
  instruction clone;
- debug OFF has zero unconditional function/region string clones and no
  per-instruction environment lookup introduced by this row;
- Const, MethodCall, Phi, Branch, and error-boundary focused outputs match;
- the existing 250-method probe median does not regress, but no CI timing
  threshold is added;
- `builder_emit.rs` is split before 760 and hard-stops at 800 rather than being
  compressed.

Non-claims: no canonical/legacy strictness split, LocalSSA/PHI redesign,
assignment semantics, A/C, Recipe, or physical Call meaning.

## `MIR-LOCAL-SSA-PREPARED-OPERAND-D0`

The common writer may rematerialize every MethodCall receiver. On a LocalSSA
cache miss, the current path scans the variable map, then all blocks and
instructions for definitions; alias traversal can repeat that scan. Verified
control-flow lowering can also prepare operands before the common writer does
the work again. Different receivers can therefore approach quadratic compile
work in function instruction count.

Decision required before I0:

```text
canonical prepared Call
  -> named prepared receiver/operand receipt
  -> sole emission commit, no repair

legacy Call
  -> existing repair owner
  -> sole emission commit

successful sole emission commit
  -> function-owned ValueId -> definition index update
```

The index must not publish a failed instruction and must have an explicit
invalidation/mutation boundary. D0 acceptance is zero canonical receiver
rematerialization, zero whole-function definition scan in the proposed fast
path, and preserved legacy behavior. N/2N probes count scans; time remains
advisory.

Non-claims: no variable-name interning, A/C, Recipe, target selection, or Call
semantics.

## `MIR-PHI-ANALYSIS-BATCH-D0`

Current PHI input materialization can construct CFG, definition-map, and
dominator analysis once per predecessor input. Finalization can then repair
the main function individually and again in the all-function pass. This is a
production-reachable superlinear candidate, but caching without a mutation
boundary could make stale analysis authoritative.

D0 must name one immutable PHI-batch analysis and the exact mutations that
end its validity. Only a successful rematerialization commit may update the
function. Acceptance is at most one analysis build per mutation-stable PHI
batch and zero duplicate main-function repair in the same phase. Do not add a
timing gate.

Non-claims: no PHI semantic change, missing-input policy change, or Binding
SSA accepted-shape expansion.

## `MIR-RECIPE-VERIFY-MOVE-R0`

A repository-wide `Verified*`/`Prepared* : Clone` prohibition is rejected.
The audit found 87 non-Copy Clone derives under `src/mir`, but a derive is not
runtime work. Three confirmed production producers clone a Recipe for source
verification and then retain the original for artifact construction:

- direct accumulator;
- nested predicate;
- Generic G0.

This bounded row changes only those three producers to a consuming verifier or
linear verified-artifact handoff. Caller-zero/test-only Clone surfaces remain
untouched. Acceptance is one Recipe owner, no deep Recipe clone at the three
callpoints, and focused product parity.

## `MIR-COMPILE-COST-BASELINE-P0`

This is parked measurement infrastructure, not a permanent gate. If selected,
evolve the existing V0 scaling runner with one warmup, five retained
repetitions, median output, explicit prebuilt release-binary identity, and
optional peak-RSS capture; keep wall time/stage timing as the portable default.
The current runner defaults to a debug binary and one sample, so its V0 output
is observation scaffolding rather than a keeper budget.
Do not store a single-machine number as semantic truth and do not require
`perf` in CI. The row exists so selfhost can select a reproducible compiler
cost observation before compiler latency becomes user-visible debt.

## Parked triggers; no task inflation

- `variable_map` is a deterministic `BTreeMap<String, ValueId>`, not a
  `HashMap`; there is no current string-hash/interner task. First measure whole
  map snapshots in nested If/scope helpers. Open
  `MIR-VARMAP-SNAPSHOT-SCALING-D0` only when live-variable count and nesting
  make that stage dominant.
- semantic refresh clones `ModuleMetadata`, but the current Selected Dynamic
  lane returns before that schedule. Open a disjoint-borrow R0 only when a
  selected production lane reaches it.
- CSE string keys and string-corridor forward/backward scans are measured only
  if `optimize` becomes dominant or scaling becomes superlinear. No typed-key
  or corridor-index task is selected now.
- DCE uses a bitset/worklist and route convergence is capped at four rounds.
  Do not create a global pass-fusion or generic O(n-squared) cleanup row.
  A bounded adjacent-pass fusion remains valid when measurement names both
  passes as dominant, they share one input authority and mutation boundary,
  and no external consumer depends on the intermediate product.

## `MIR-EMIT-CANONICAL-STRICTNESS-D0`

Audit `emit_instruction` as the single physical writer. Separate the contract
for canonical verified placement from legacy materialization/repair without
creating a second writer. The first deliverable is a design card naming the
prepared emission receipt or strict/legacy API boundary; implementation is
blocked until the owner and fail-fast behavior are accepted.

Acceptance:

- canonical missing-block/operand/PHI conditions fail fast;
- receiver materialization and PHI normalization are named inputs, not hidden
  repair;
- one final physical writer remains;
- legacy compatibility behavior is unchanged and explicitly scoped;
- focused positive/negative evidence and a source-size guard exist.

Non-claims: no assignment semantics, Script A/C capability, Recipe/Join, or
backend performance work.

## `MIR-LOOP-COMPARE-SAME-BLOCK-P0-CONNECT0`

The accepted C-prime series is caller-zero and must not be labelled I0. Its
single order SSOT is the strictness D0 card:

```text
MIR-LOOP-OPERATION-EMITTER-SPLIT-S0
-> MIR-LOOP-COMPARE-SESSION-TARGET-P0
-> MIR-LOOP-COMPARE-SAME-BLOCK-OPERANDS-P0
-> MIR-LOOP-COMPARE-LEDGER-RESERVATION-P0
-> MIR-LOOP-COMPARE-STRICT-WRITER-P0
-> MIR-LOOP-COMPARE-SAME-BLOCK-CONNECT0
```

The physicalizer and its whole-session facade remain under `#[cfg(test)]`.
Only the segment dispatcher, which owns its value ledger by move, may receive
the strict Compare connection. The older caller-owned ledger dispatcher,
generic Compare helper, cross-block operands, and production activation remain
outside the series.

A future `MIR-LOOP-COMPARE-I0-R0` requires a newly selected non-test caller,
its exact same-block cohort census, and atomic old-production-edge retirement.
If that caller requires cross-block, parameter, or inherited operands, return
to design instead of weakening C-prime.

## `MIRBUILDER-BARREL-RESPONSIBILITY-CLEANUP-D0`

Build a read-only module census for `builder.rs`: production, compatibility,
migration, and test-only exports. Propose a thin `production`/
`compatibility`/`migration` barrel boundary only where current callers prove
the ownership. Do not move files or change behavior during the census.

Acceptance:

- every retained re-export has a named owner and caller class;
- cross-import rules are explicit;
- caller-zero/retirement conditions are recorded before any move;
- no `#[allow(dead_code)]` cleanup is treated as semantic retirement.

## `MIRBUILDER-INIT-RESPONSIBILITY-CLEANUP-D0`

Audit `builder_init.rs` and name the current owners of queries, ID allocation,
CFG mutation, metadata origin, scope state, and binding allocation. Resolve the
ambient choice in `next_function_value_id_or_core()` as an authority question
before proposing physical file splits.

Acceptance:

- function-local versus module/core ID authority is explicit;
- query and mutation owners are distinct in the design map;
- no allocator or metadata behavior changes in the census row;
- implementation is a later BoxShape refactor with focused parity evidence.

## `MIRBUILDER-MAIN-INTEGRATION-CLOSEOUT`

The active branch is not `main`; `main` remains the canonical integration
target. This row is operational only and must be selected when the branch has
finished its semantic cells.

Acceptance:

- explicit merge/rebase policy and clean-tree check;
- pushed branch commits and integration result recorded;
- `CURRENT_STATE.toml` pointer, active card, and branch status agree;
- no claim that `main` is closed before the integration actually lands.

## Global stop rules

No row may introduce a second physical writer, AST/name/ordinal pairing,
unconditional fallback, guessed semantic receipt, a production route that is
not named in the current card, a repository-wide Clone ban, or an always-on
wall-time/RSS/instruction gate. A row that discovers a missing authority
returns to `design_stop` and updates this queue instead of creating another
current SSOT.
