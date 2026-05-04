---
Status: Active
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381J, boolean short-circuit PHI lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P381F-STAGE1-SHORT-CIRCUIT-NESTED-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P379A-SHORTCIRCUIT-LHS-PIN-LIFETIME.md
  - docs/development/current/main/design/short-circuit-joins-ssot.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
---

# P381J: Boolean Short-Circuit PHI Lowering

## Problem

P381F moved the Stage1 env cluster forward by rewriting short-circuit source
checks into nested guards. That was a bootstrap cleanup, not a language or
compiler policy change.

The root blocker is still compiler-owned:

```text
opt: /tmp/p381e_bad.ll:257:19: error: '%r8' defined with type 'i1' but expected 'i64'
  %r9 = phi i64 [ %r8, %bb1 ], [ 0, %bb2 ]
                  ^
```

The source shape is valid Hakorune:

```hako
if s == "null" || s == "Null" || s == "void" || s == "Void" {
  return ""
}
```

`&&` and `||` are supported language constructs. Rewriting owner source to
avoid them is acceptable only as a temporary unblocker. The durable fix is to
lower boolean short-circuit PHIs with the correct LLVM type.

## Decision

Handle short-circuit boolean value PHIs in the generic MIR/LLVM lowering path:

- boolean short-circuit PHIs used as branch conditions are `i1`
- boolean constants in those PHIs are emitted as `false` / `true`, not raw `i64`
- conversion to `i64` happens only at scalar ABI / return boundaries that require
  an integer result
- existing MIR builder short-circuit CFG shape remains the SSOT unless a MIR
  fixture proves the CFG itself is wrong

This is generic MIR op support. It must not be solved by adding a source-helper
body shape or by teaching Stage0 the meaning of a specific `.hako` helper.

## Boundary

Do:

- inspect the current MIR for short-circuit `&&` / `||` fixtures before editing
- keep `src/mir/builder/ops/logical_shortcircuit.rs` and
  `docs/development/current/main/design/short-circuit-joins-ssot.md` as the
  short-circuit CFG contract
- fix generic lowering / prescan type ownership for boolean PHIs
- add a focused canary that compiles a short-circuit boolean expression without
  producing `phi i64 [ %i1, ... ]`
- preserve P381F as a temporary source-surface cleanup, not as the final rule

Do not:

- add a new `GlobalCallTargetShape`
- widen `generic_string_body` / `generic_i64_body`
- add a body-specific C shim emitter
- add by-name handling for `Stage1InputContractBox`
- change language support for `&&` / `||`
- restore VM fallback

## Implementation Notes

Likely owners:

- `lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc`
- `lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc`
- `lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc`
- `src/mir/builder/ops/logical_shortcircuit.rs`
- `src/mir/builder/phi.rs`
- `src/runner/mir_json_emit/emitters/basic.rs`
- `src/runner/mir_json_emit/emitters/phi.rs`
- `src/tests/mir_controlflow_extras.rs`

Current inventory:

- expression-level `&&` / `||` routes through
  `src/mir/builder/ops/logical_shortcircuit.rs`
- the normal MIR builder creates a 3-input PHI typed `MirType::Bool`
- MIR JSON emits bool PHIs with `dst_type: "i1"`
- MIR JSON currently emits bool constants as `{"type":"i64","value":0|1}`
- pure-first pre-scan tries to infer bool PHIs from `i1` values and bool-like
  constants
- the fragile boundary is LLVM use-site coercion: PHI, branch, and return paths
  each handle type conversion locally

The expected shape is a boolean PHI in value/condition context. If the PHI result
crosses an `i64` ABI boundary, the boundary emits the `zext`; the PHI itself
should not be widened while retaining `i1` incoming values.

Adjacent gap to keep separate: some normalizer value-context paths lower
`&&` / `||` as eager `And` / `Or`. That is not this card unless a failing
fixture proves it is the active blocker. P381J owns the LLVM type mismatch for
already-lowered boolean PHIs.

## Acceptance

Add one focused short-circuit canary before relying on the Stage1 source replay.
The fixture should cover at least this shape:

```hako
if s == "null" || s == "Null" {
  return 1
}
return 0
```

Required checks:

```bash
cargo test --release shortcircuit_no_inner_join_phi
cargo test --release shortcircuit_lhs_pin_does_not_escape_variable_map
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381j_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381j_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- no LLVM verifier error of the form `phi i64 [ %<i1>, ... ]`
- the focused canary verifies without rewriting `&&` / `||` source
- phase29cg remains green after the backend fix

## Result

Active.
