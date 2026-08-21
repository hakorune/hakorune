Status: queued correctness/compile-cost follow-up SSOT; current pointer is the canonical-emission strictness design row
Date: 2026-08-22
Parent: `CURRENT_STATE.toml` and the active Script A capability card
---

# MirBuilder post-audit follow-up queue

This queue records the concrete issues confirmed by top-down review and the
2026-08-22 production-reachability audit. The current pointer is now
`MIR-EMIT-CANONICAL-STRICTNESS-D0`; these rows do not authorize an A/C bypass,
a fallback, a production switch, or an optimization selected only from source
appearance.

| Row | Priority | Owner | Selection boundary |
| --- | --- | --- | --- |
| `MIR-RESULT-DISCARD-CENSUS-D0` | Medium-High | `src/mir/builder` policy | read-only census before any lint/guard rollout; [detail](./mirbuilder-result-discard-policy-d0-2026-08-21.md) |
| `MIR-ASSIGNMENT-RELEASE-FAILFAST-I0` | High | `assignment_lowering.rs` | independent correctness cell; [detail](./mirbuilder-assignment-release-failure-atomicity-i0-2026-08-21.md) |
| `MIR-RESULT-DISCARD-GUARD-I0` | Medium-High | `tools/checks` | narrow multiline-aware physical-writer guard; [detail](./mirbuilder-result-discard-guard-i0-2026-08-22.md) |
| `MIR-EMIT-MOVE-COMMIT-R0` | High confidence | `builder_emit.rs` | after current A/C series or an explicit independent selection; do not overlap the strictness row |
| `MIR-LOCAL-SSA-PREPARED-OPERAND-D0` | Medium-High | `builder_emit.rs` + `ssa/local.rs` | design the prepared/legacy boundary and function-owned definition index before implementation |
| `MIR-PHI-ANALYSIS-BATCH-D0` | Medium-High | PHI materialization/finalization | name a mutation-stable analysis batch before caching or deleting a repair pass |
| `MIR-EMIT-CANONICAL-STRICTNESS-D0` | Accepted | canonical Loop physicalization + ordinary writer | C-prime same-block Decision and P0/CONNECT0 order are fixed; [detail](./mirbuilder-emit-canonical-strictness-d0-2026-08-22.md) |
| `MIR-LOOP-OPERATION-EMITTER-SPLIT-S0` | Selected prerequisite | `loop_recipe_physicalizer/operation_emitter.rs` | behavior-neutral pure-operation owner split before semantic growth; [detail](./mirbuilder-loop-operation-emitter-split-s0-2026-08-22.md) |
| `MIR-LOOP-COMPARE-SAME-BLOCK-P0-CONNECT0` | High, ordered | canonical CFG/SSA, Loop ledger, ordinary writer | caller-zero only: session target -> same-block operands -> reservation -> strict writer -> CONNECT0; production I0/R0 remains a later design |
| `MIR-RECIPE-VERIFY-MOVE-R0` | Medium | three production Recipe producers | only after their selected profiles remain production-reachable |
| `MIR-COMPILE-COST-BASELINE-P0` | Parked prerequisite | existing compile timing/scaling tools | selfhost/perf return lane or before claiming a compiler-speed keeper |
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

## `MIR-EMIT-MOVE-COMMIT-R0`

The sole physical writer is production-reachable and currently performs work
on every successful emission:

- unused current-function and region debug clones near function entry;
- MethodCall `Callee`, owner/method strings, and argument-vector cloning while
  rebuilding the receiver-bearing Call;
- another unconditional function-name clone;
- `MirInstruction::clone()` at the final block commit;
- repeated debug/env queries on the per-instruction path.

This row extracts only the post-commit facts needed by metadata/PHI observers,
then moves the instruction into the block once. Invocation debug policy is
snapshotted once or queried only inside a selected debug/error branch. The
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
extend the existing scaling runner with optional peak-RSS capture and a
versioned output schema; keep wall time/stage timing as the portable default.
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
