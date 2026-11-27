# MIR Passes (`src/mir/passes/`)

This subtree contains MIR transformation passes and their local helpers.

## Status

Docs-first only for now. Do not package this subtree as
`hakorune-mir-passes` yet.

Current blockers:

- `callsite_canonicalize.rs` still couples to `crate::ast::ASTNode` through
  closure-body metadata
- `cse.rs` and `dce.rs` still assume the main `crate::mir::*`
  surface and module layout
- `rc_insertion.rs` / `rc_insertion_helpers.rs` still depend on AST, runtime,
  and config/env seams
The retired optional concat3 rewrite and VM barrier-elision passes are no longer extraction candidates.

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

The measured usage gate for retired scaffolds is recorded under
[OPT-THIN-002](../../../docs/development/current/main/design/compiler-pipeline-thinning-ssot.md#opt-thin-002).

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

### SimplifyCFG ownership

`simplify_cfg/flow.rs` owns branch threading, jump merging and PHI predecessor
decisions. Its private `value_uses.rs` substitutes value operands only after the
flow owner selects a rewrite; it does not infer control flow or select targets.
`value_uses_tests.rs` retains operand/target preservation coverage.
