---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380S, generic body env.get canonical spelling
Related:
  - docs/development/current/main/phases/phase-29cv/P380R-STRUCTURED-CALL-PLAN-DIAGNOSTIC.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/generic_i64_body.rs
---

# P380S: Generic Body `env.get` Canonical Spelling

## Problem

P380R moved phase29cg diagnostics to MIR-owned plan facts. The current target
blocker is:

```text
Stage1ModeContractBox.resolve_mode/0
target_shape_reason=generic_string_unsupported_extern_call
```

The target body contains canonical legacy extern JSON:

```json
{"op":"externcall","func":"env.get","args":[1],"dst":2}
```

Rust-side generic body classifiers accepted only the arity-suffixed spelling
`env.get/1`. The extern route SSOT already treats both `env.get` and
`nyash.env.get` with arity 1 as `EnvGet`.

## Decision

Use the extern route classifier for `env.get` acceptance in generic string and
generic i64 body classifiers.

This is not new extern semantics. It only aligns body classification with the
existing `extern_call_routes` canonical spelling policy.

## Non-Goals

- no C emitter change
- no new target shape
- no generic acceptance of arbitrary extern calls
- no change to `env.set` behavior

## Acceptance

```bash
cargo test --release env_get_canonical_spelling

rm -rf /tmp/p380s_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380s_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: `Stage1ModeContractBox.resolve_mode/0` should no longer be blocked
by `generic_string_unsupported_extern_call` solely because the extern symbol is
spelled `env.get` instead of `env.get/1`.

## Result

Implemented.

Generic string and generic i64 body classifiers now accept `EnvGet` through the
existing extern route classifier. This keeps the spelling policy in
`extern_call_routes` instead of duplicating arity-suffixed name checks.

Validation:

```text
cargo test --release env_get_canonical_spelling
-> PASS: 2 tests

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p380s_phase29cg \
  STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
  NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
-> emit_program_rc=0 emit_mir_rc=0 llvm_rc=4
```

The blocker moved forward. `Stage1ModeContractBox.resolve_mode/0` is now
planned far enough for module generic definition emission, and the next failure
is inside a same-module child definition:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1InputContractBox.clean_env_value/1
```

The next card should make the module generic string prepass/body emitter consume
structured `op:"call"` the same way entry pure lowering does. It must not add a
new body shape.
