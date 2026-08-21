Status: queued follow-up map; none of these rows is the current pointer
Date: 2026-08-21
Parent: `CURRENT_STATE.toml` and the active Script A capability card
---

# MirBuilder post-audit follow-up queue

This queue records the concrete issues confirmed by the top-down review. The
current pointer remains `SCRIPT-A-CAPABILITY-I0`; these rows do not authorize
an A/C bypass, a fallback, or a production switch.

| Row | Priority | Owner | Selection boundary |
| --- | --- | --- | --- |
| `MIR-ASSIGNMENT-RELEASE-FAILFAST-I0` | High | `assignment_lowering.rs` | independent correctness cell; [detail](./mirbuilder-assignment-release-failure-atomicity-i0-2026-08-21.md) |
| `MIR-EMIT-CANONICAL-STRICTNESS-D0` | Medium-High | `builder_emit.rs` | after assignment fix or an explicit design selection |
| `MIRBUILDER-BARREL-RESPONSIBILITY-CLEANUP-D0` | Medium | `builder.rs` | after relevant production callers are caller-zero |
| `MIRBUILDER-INIT-RESPONSIBILITY-CLEANUP-D0` | Medium | `builder_init.rs` | after the barrel/owner census is accepted |
| `MIRBUILDER-MAIN-INTEGRATION-CLOSEOUT` | Operational | branch/pointer SSOT | after the active branch has an explicit integration step |

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
unconditional fallback, guessed semantic receipt, or a production route that
is not named in the current card. A row that discovers a missing authority
returns to `design_stop` and adds that authority to this queue.
