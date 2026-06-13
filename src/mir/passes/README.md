# MIR Passes (`src/mir/passes/`)

This subtree contains MIR transformation passes and their local helpers.

## Status

Docs-first only for now. Do not package this subtree as
`hakorune-mir-passes` yet.

Current blockers:

- `callsite_canonicalize.rs` still couples to `crate::ast::ASTNode` through
  closure-body metadata
- `cse.rs`, `dce.rs`, and `escape.rs` still assume the main `crate::mir::*`
  surface and module layout
- `rc_insertion.rs` / `rc_insertion_helpers.rs` still depend on AST, runtime,
  and config/env seams
- `concat3_canonicalize/` is the only plausible future extraction candidate,
  but it still depends on the same MIR surface for now
- landed internal cleanup: `concat3_canonicalize/analysis/` now splits
  `stringish.rs` and `def_use.rs` behind the current facade

Next review target:

- `concat3_canonicalize/` as the first real substrate-style extraction candidate

## Read First

1. `src/mir/README.md`
2. `src/mir/contracts/README.md`
3. `src/mir/policies/`

## Boundaries

- New acceptance rules belong in `contracts/`, not hidden inside a pass.
- Shared policy belongs in `policies/` and should be reused by consumers.
- A pass should do one job: transform or verify, not both.

## Main Responsibilities

- MIR-wide transformations
- pass-local verification and fail-fast checks
- small helper wiring for optimizer / normalization stages

## Optimizer Schedule Facade

The visible optimizer schedule is documented as seven facade groups. These
groups do not merge behavior-owning passes; they only make the top-level order
readable.

```text
normalize_frontend_surface
placement_effect_pre
canonical_simplification
memory_cleanup_wave
placement_effect_post
late_call_and_inline
optional_and_diagnostics
```

SSOT:

- `docs/development/current/main/design/compiler-pipeline-thinning-ssot.md`
- `src/mir/optimizer/core.rs`

Critical order contracts:

```text
placement_effect_pre
  -> canonical_simplification
  -> memory_cleanup_wave
  -> placement_effect_post
  -> late_call_and_inline
```

Do not physically merge DCE, memory-effect cleanup, or pre/post
placement-effect without a separate optimizer behavior card.

## Reserved / Scaffold Hooks

These entries currently exist to keep the schedule seam stable. Do not read
them as active optimizer wins.

| Hook | Current behavior | Schedule group | Rule |
| --- | --- | --- | --- |
| `optimizer_passes::reorder::reorder_pure_instructions` | no-op scaffold; debug logging only | `late_call_and_inline` | keep until a separate card either implements or retires it |
| `optimizer_passes::intrinsics::optimize_intrinsic_calls` | no-op scaffold; debug logging only | `late_call_and_inline` | keep until a separate card either implements or retires it |
| `passes::type_hints::propagate_param_type_hints` | returns `0` updates | `late_call_and_inline` | keep as reserved hook; do not claim type propagation is active |
| `optimizer_passes::normalize::normalize_ref_field_access` | idempotence marker only unless future rewriting lands | `normalize_frontend_surface` | do not move before proving Core-13 / ref-field timing |

## P5 Crate Split Prep

`src/mir/passes/` is a future `hakorune-mir-passes` candidate. Keep the public seam
small so the eventual split is a packaging step, not a redesign.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- one pass should transform or verify, not both
- helper extraction should keep the pass entry thin
- shared policy still belongs in `src/mir/policies/`
- this subtree is docs-first only until the AST/runtime/config coupling is
  reduced enough to make packaging mechanical
