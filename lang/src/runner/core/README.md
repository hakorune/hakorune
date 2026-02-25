# Runner Core (Plan Builder)

Scope
- Decision-only layer to build a `RunnerPlan` from CLI/ENV inputs.
- No I/O or process execution here. Rust (Floor) applies the plan.

Status
- Phase 20.26‑C scaffold. Opt‑in via `HAKO_RUNNER_PLAN=1` once wired.

Contract (MVP)
- Provide `RunnerPlanBuilder.build(args)` that returns a small JSON object with a minimal set of fields:
  - `{"action":"ExecuteCore|ExecuteVM|ExecuteNyLlvmc|Skip|Error", "gate_c":bool, "engine":"core|vm|llvm", "plugins":bool, "quiet":bool}`
- Fail‑Fast on ambiguity; no silent fallbacks.
