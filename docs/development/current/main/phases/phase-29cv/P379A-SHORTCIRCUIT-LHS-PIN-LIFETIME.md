---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: keep short-circuit LHS pins expression-local
Related:
  - docs/development/current/main/phases/phase-29cv/P378A-STANDARD-ROUTE-COMPOSE-BINDING-BOUNDARY.md
  - docs/development/current/main/design/short-circuit-joins-ssot.md
  - src/mir/builder/ops/logical_shortcircuit.rs
---

# P379A: Short-Circuit LHS Pin Lifetime

## Intent

Continue the direct Stage1 env MIR verify cleanup after P378A reduced the
bucket from 24 to 10 errors.

The remaining errors cluster around nested short-circuit/string-comparison
helpers:

```text
BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1:
  %4058 defined in bb13107, used through a later merge
  %5123 defined in bb13186, used through a later merge

JsonFragBox.read_float_from/2:
  %158 defined in bb4909, used from bb4910/bb4911

JsonFragNormalizerBox._read_num_token/2:
  %518 defined in bb10490, used from bb10485

JsonNumberCanonicalBox.read_num_token/2:
  %524 defined in bb10102, used from bb10097
```

The common source shape is nested `&&` / `||` lowering. The LHS value is pinned
as `__pin$...@sc_lhs` so the already-emitted branch condition can safely reuse
the value. That pin is not a user local and must not participate in branch-entry
materialization or the 3-predecessor short-circuit merge.

## Boundary

This is a BoxShape cleanup in the short-circuit lowering owner.

Do:

- keep the LHS branch-condition pin alive only long enough to emit the branch
- remove the synthetic `@sc_lhs` slot from `variable_map` before branch snapshots
- keep the branch condition SSA value and existing short-circuit CFG unchanged
- add a canary that `@sc_lhs` does not escape into the outer variable map

Do not:

- change verifier-wide dominance semantics
- make `merge_modified_vars_multi` stricter globally
- edit `.hako` source to avoid `&&` / `||`
- add route shapes, Stage0 emitters, or C shim logic

## Acceptance

```bash
cargo test --release shortcircuit_lhs_pin_does_not_escape_variable_map
cargo build --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Remeasure:

```bash
timeout --preserve-status 240s env \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/selfhost_exe_stageb.sh \
  lang/src/runner/stage1_cli_env.hako \
  -o /tmp/p379_stage1_cli_env.exe
```

## Result

Implemented:

- `logical_shortcircuit` records the LHS `@sc_lhs` pin slot after pinning
- the synthetic slot is removed from `variable_map` before branch snapshots
- the same slot is removed again after the 3-predecessor variable merge
- `shortcircuit_lhs_pin_does_not_escape_variable_map` locks the boundary

Verified:

- `cargo test --release shortcircuit_lhs_pin_does_not_escape_variable_map` passed
- `cargo build --release --bin hakorune` passed
- `bash tools/checks/current_state_pointer_guard.sh` passed
- `git diff --check` passed

Remeasure:

- The direct Stage1 env MIR verifier bucket moved past the previous 10
  dominance errors.
- The next stop is backend recipe support:

```text
unsupported pure shape for current backend recipe
```

This confirms P379A fixed the short-circuit PHI/dominance leak without widening
Stage0, C shim, or `.hako` source shapes.
