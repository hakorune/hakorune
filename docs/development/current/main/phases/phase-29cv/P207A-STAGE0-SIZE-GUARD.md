---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207a, Stage0 size guard and body-classifier no-growth boundary
Related:
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md
  - docs/development/current/main/phases/phase-29cv/README.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P207a: Stage0 Size Guard

## Problem

P180-P206 advanced the source-execution route by making missing DirectAbi facts
explicit and by adding narrow body-shape classifiers. That was useful for
removing VM fallback pressure, but the accumulated pattern now has a structural
risk:

```text
selfhost compiler body
  -> route/body classifier understands the body
  -> C shim emits body-specific lowering
```

If continued indefinitely, Stage0 stops being a small MIR-to-LLVM bootstrap
line and becomes a second implementation of the `.hako` compiler's semantics.

Current red flags:

- `generic_string_body.rs` and `generic_i64_body.rs` keep gaining body
  understanding.
- dedicated `GlobalCallTargetShape` variants are growing beyond ABI facts.
- `hako_llvmc_ffi_module_generic_string_function_emit.inc` duplicates local
  value-class and PHI/method flow in C.
- `.hako` MIR(JSON v0) emit helpers can grow by fixed fixture/body shape.

## Decision

Lock a Stage0 size guard before adding more source-execution blockers:

```text
Stage0 does not know the selfhost compiler.
Stage0 knows Canonical MIR, a uniform ABI, explicit route facts, runtime helper
routes, and fail-fast diagnostics.
```

Stage0 may own:

- generic MIR(JSON) schema validation
- object/exe emission and linking
- bootstrap/recovery diagnostics
- uniform MIR function emission
- explicit runtime helper calls from MIR-owned `LoweringPlan` facts

Stage0 must not own:

- ParserBox / BuildBox / JsonFrag / LoopScan / MirBuilder semantic policy
- new body-shape clones of selfhost compiler functions
- C-side rediscovery of body semantics already owned by MIR facts
- fixture-name or source-box-name lowering paths

## Boundary

From this card onward, source-execution blocker work should default to this
order:

1. Prefer source cleanup that makes an existing MIR/route fact explicit.
2. Prefer consuming existing MIR-owned facts such as `value_types`,
   `LoweringPlan`, arity, return shape, and value demand.
3. Prefer adding generic MIR op support to a shared per-function emitter.
4. Avoid new `GlobalCallTargetShape` variants unless the card includes an
   explicit removal path or proves that the shape is only a validator.
5. Do not add new ny-llvmc body-specific emitters for selfhost compiler helpers.

Dedicated body classifiers already present remain temporary compatibility
capsules. They should be quarantined behind ABI/capability facts and eventually
collapsed into a uniform MIR function emitter.

## Current Next Blocker

P206 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=LoopScanBox.find_loop_var_name/2
target_shape_blocker_reason=generic_string_return_not_string
```

P207b must fix that blocker without adding a new body shape.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
