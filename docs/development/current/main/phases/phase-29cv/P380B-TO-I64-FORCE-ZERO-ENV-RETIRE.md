---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380B, retire active NYASH_TO_I64_FORCE_ZERO source residue
Related:
  - docs/development/current/main/phases/phase-29cv/P380A-STRINGHELPERS-TO-I64-DEBUG-HOOK-RETIRE.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/src/shared/common/string_helpers.hako
  - lang/src/runner/stage1_cli/config.hako
---

# P380B: to_i64 Force-Zero Env Retire

## Problem

After P380A, `StringHelpers.to_i64/1` no longer contains the diagnostic MIR
`Debug` hook, but it still begins with an active env-controlled workaround:

```hako
if env.get("NYASH_TO_I64_FORCE_ZERO") == "1" { return 0 }
```

This flag is not part of the current environment-variable reference. It is a
Stage1 bring-up workaround that makes every numeric conversion return zero when
enabled, which is too broad for a common helper used by compiler and runtime
support code.

Keeping it in `StringHelpers.to_i64/1` also keeps an env branch in the hottest
numeric scanner body. Stage0 should not need to preserve a historical
force-zero shortcut to classify or emit the scanner.

## Decision

Remove the active `NYASH_TO_I64_FORCE_ZERO` path from current source and tests:

- `StringHelpers.to_i64/1` keeps its local `null -> 0` guard and numeric scan.
- `Stage1CliConfigBox` no longer captures `to_i64_force_zero`.
- the static box naming unit test no longer sets the retired env var.

Historical archived scripts/docs may still mention the flag as old engineering
evidence. They are not active contracts.

## Non-Goals

- Do not add Stage0 route acceptance.
- Do not change `StringHelpers.to_i64/1` numeric parsing semantics beyond
  removing the env-wide zero override.
- Do not edit archived legacy scripts.
- Do not make the Stage1 env EXE route green in this card.

## Acceptance

```bash
cargo test --release generic_i64 --lib
cargo build --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The old `mir_static_main_box_executes_void_path_with_guard` assertion now lives
behind the `legacy-tests` feature. The default command finds no active test, and
the feature-enabled run is currently blocked before this case by an unrelated
pre-existing compile error in `src/tests/mir/../mir_controlflow_extras.rs`.
P380B therefore verifies the active default path with the generic i64 tests,
release build, current-state guard, diff hygiene, and MIR/route probes below.

Advance-to-next-blocker probe:

```bash
timeout --preserve-status 240s env \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/selfhost_exe_stageb.sh \
  lang/src/runner/stage1_cli_env.hako \
  -o /tmp/p380b_stage1_cli_env.exe
```

Expected reading: active source no longer contains `NYASH_TO_I64_FORCE_ZERO`.
Any remaining pure-first stop is a real route classification or uniform
multi-function emitter issue, not the legacy force-zero env branch.

## Result

Active references were retired from current source:

- `StringHelpers.to_i64/1` no longer reads `NYASH_TO_I64_FORCE_ZERO`.
- `Stage1CliConfigBox` no longer records `to_i64_force_zero`.
- the legacy static box naming fixture no longer sets the retired env var.

The generated Stage1 CLI env MIR confirms that `StringHelpers.to_i64/1` has no
active diagnostic/env residue:

```bash
jq -r '.functions[] | select(.name=="StringHelpers.to_i64/1") |
  [.blocks[].instructions[]?, .blocks[].terminator?] |
  map(select(.op=="debug" or (.mir_call.callee.name?=="env.get/1"))) |
  length' /tmp/p380b_stage1_cli_env.mir.json
# => 0
```

The pure-first EXE route is still expected to stop, but the stop is now a
classification boundary rather than the retired env workaround:

```text
[llvm-pure/unsupported-shape] recipe=pure-first first_block=14023 first_inst=5 first_op=mir_call owner_hint=backend_lowering reason=missing_multi_function_emitter target_return_type=? target_shape_reason=generic_string_global_target_shape_unknown target_shape_blocker_symbol=StringHelpers.to_i64/1 target_shape_blocker_reason=generic_string_return_not_string
```
