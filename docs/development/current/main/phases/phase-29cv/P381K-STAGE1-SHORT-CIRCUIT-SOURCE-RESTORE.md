---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381K, restore Stage1 env short-circuit source after P381J
Related:
  - docs/development/current/main/phases/phase-29cv/P381J-BOOLEAN-SHORT-CIRCUIT-PHI-LOWERING.md
  - docs/development/current/main/phases/phase-29cv/P381F-STAGE1-SHORT-CIRCUIT-NESTED-GUARD.md
  - docs/development/current/main/design/short-circuit-joins-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/stage1_cli_env/input_contract.hako
  - lang/src/runner/stage1_cli_env/mode_contract.hako
---

# P381K: Stage1 Short-Circuit Source Restore

## Problem

P381F temporarily rewrote valid Stage1 env short-circuit source into nested
guards to move past a backend `i1` / `i64` PHI mismatch. P381J fixed that
compiler-owned mismatch in generic MIR/LLVM lowering.

Leaving the P381F source workaround in place would make the codebase imply that
`&&` / `||` should be avoided in Stage1 owner source. That is the wrong
contract.

The restore also exercises same-module `generic_pure_string_body` definitions,
not only the pure entry function. That emitter must preserve short-circuit PHIs
whose incoming values are a mix of compare results and canonical `0` / `1`
constants.

## Decision

Restore idiomatic short-circuit source in the Stage1 env input/mode contract
surface:

- `clean_env_value` may use a chained `||` guard for null/void sentinel text
- `_env_flag_enabled` may use `flag != null && ... == "1"`
- mode alias normalization may use `||` in the same-file monolith to match the
  split `mode_contract.hako` owner file

This is a source cleanup that consumes P381J. It must not add backend body
shapes or source-specific C shim semantics.

## Boundary

Do:

- update both the same-file authority cluster and split owner file where the
  same contract exists
- treat `0` / `1` constants as bool-like PHI inputs only inside the existing
  boolean PHI type refinement path
- keep behavior identical
- verify the phase29cg replay remains green

Do not:

- change language support for `&&` / `||`
- add new `GlobalCallTargetShape`
- widen `generic_string_body` / `generic_i64_body`
- add a C shim body-specific emitter
- reintroduce VM fallback

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo build --release --bin hakorune

rm -rf /tmp/p381k_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381k_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- Stage1 env source contains the restored short-circuit forms
- phase29cg remains green
- no new Stage0 body shape or fallback surface is introduced

## Result

Done:

- restored Stage1 env/mode short-circuit source
- kept Stage0 shape inventory unchanged
- repaired same-module generic string PHI refinement so restored source emits
  `phi i1` for short-circuit joins
- phase29cg replay is green with `verify_rc=0`
