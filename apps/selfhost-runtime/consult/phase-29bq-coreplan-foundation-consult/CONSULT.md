# Phase 29bq consult: CorePlan “foundation blocks” for selfhost canary

## Context

This repo is intentionally **compiler-expressivity-first**: avoid `.hako` workarounds and instead strengthen the Rust-side compiler (Facts/Normalize/CorePlan) while keeping **release defaults unchanged**.

SSOT policy:

- `docs/development/current/main/design/compiler-expressivity-first-policy.md`

## Constraints (non-negotiable)

- **No AST rewrite** (no algebraic transforms like `j+m<=n` → `j<=n-m`; preserve eval order / overflow / side effects).
- New expressivity must be **strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1` only** (release default unchanged).
- No silent fallback: strict/dev should fail-fast when out of scope.
- Lowering must respect EdgeCFG’s **“1 block = 1 terminator”** contract (`src/mir/builder/control_flow/edgecfg/api/emit.rs`).

## Current problem (selfhost canary)

Selfhost entry gate:

- `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`

Current FAIL (stage-b compile):

- Case: `apps/tests/phase118_pattern3_if_sum_min.hako`
- Log: `/tmp/phase29bq_selfhost_phase118_pattern3_if_sum_min.hako.log`
- Tail snapshot: `logs/selfhost_blocker_tail.txt`
- Error (latest): `[plan/freeze:unsupported] generic loop v0.2: control flow after in-body step`

Meaning:

- The loop has a **step (loop_increment) that is not at the end of the loop body**, and there is **control flow after that step** (e.g. `exit-if` such as `if (...) { break }`).
- Current generic loop v0/v1 assumes it can **extract the step and move it into a dedicated `step_bb`** without changing semantics.
- Under “no rewrite”, this is **not safe**, so the planner fails fast in strict/dev.

## What we want (design goal)

Strengthen the compiler’s “block stacking” foundation so these parser-ish loops can be represented without `.hako` rewrites:

- Keep raw AST unchanged.
- Add **small, composable CorePlan building blocks** and keep verification local.

## Proposed missing foundation blocks (candidate list)

The current blocker suggests we need a way to represent **step placement** as-is:

1. **`StepPlacement` as Facts data** (keep it, don’t just validate and discard)
2. A plan/lowerer path for **“step-in-body”** (i.e. step is lowered in the body at its original position), still within strict/dev + planner_required.

Additionally, to keep the overall system composable and avoid pattern explosion, we expect these foundational blocks to be needed soon (if not already present):

- `Block/Seq` (n-ary) as an explicit CorePlan skeleton node (avoid “implicit effect lists”).
- `CleanupWrap` (defer/cleanup hierarchy) as a CorePlan vocabulary node (nested exit semantics).
- `Break/Continue(depth)` (depth=1 default) + LoopFrame stack in lowering (for nested loops without by-name labels).

## Questions for review (what we want feedback on)

1. Is the “step-in-body” issue best solved by:
   - (A) extending GenericLoop facts to retain `StepPlacement` and adding a **new plan variant** for step-in-body, or
   - (B) making a more general LoopFrame v1 pathway that always consumes `StepPlacement` and lowers accordingly?

2. Are there any other *must-have* CorePlan foundation primitives missing besides:
   - `Block/Seq`, `CleanupWrap`, and `StepPlacement/step-in-body`?
   Examples: `BranchN` (switch/match), “cleanup-on-exit” routing rules, etc.

3. For strict/dev only, what is the cleanest local verifier contract for “step-in-body” so we do not accidentally widen semantics?

## Included references (copied into this bundle)

Docs (SSOT):

- `docs/development/current/main/phases/phase-29bq/README.md`
- `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`
- `docs/development/current/main/design/coreloop-loopframe-v1-ssot.md`
- `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- `docs/development/current/main/10-Now.md`
- `CURRENT_TASK.md`

Core code (relevant):

- Generic loop facts/normalizer: `src/mir/builder/control_flow/plan/generic_loop/facts.rs`, `src/mir/builder/control_flow/plan/generic_loop/normalizer.rs`, `src/mir/builder/control_flow/plan/generic_loop/canon.rs`
- LoopTrue facts/normalizer: `src/mir/builder/control_flow/plan/loop_true_break_continue/facts.rs`, `src/mir/builder/control_flow/plan/loop_true_break_continue/normalizer.rs`
- Loop(cond) facts: `src/mir/builder/control_flow/plan/loop_cond_break_continue/facts.rs`
- Planner entry: `src/mir/builder/control_flow/plan/planner/build.rs`
- CorePlan nodes + lowerer/verifier: `src/mir/builder/control_flow/plan/mod.rs`, `src/mir/builder/control_flow/plan/lowerer.rs`, `src/mir/builder/control_flow/plan/verifier.rs`, `src/mir/builder/control_flow/plan/coreloop_body_contract.rs`
- EdgeCFG terminator contract: `src/mir/builder/control_flow/edgecfg/api/emit.rs`

Scripts:

- Selfhost gate: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Selfhost subset TSV: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- 29bq fast iteration gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`

