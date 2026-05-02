---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P153, StringHelpers starts_with generic-i64 route closure
Related:
  - docs/development/current/main/phases/phase-29cv/P152-PARSER-PROGRAM-JSON-BODY-RECIPE.md
  - docs/development/current/main/phases/phase-29cv/P144-GLOBAL-CALL-STRING-SCAN-I64-BODY.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/src/shared/common/string_helpers.hako
  - src/mir/global_call_route_plan.rs
---

# P153: Generic I64 Debug String Surface

## Problem

P152 moved the source-execution stop-line to:

```text
target_shape_blocker_symbol=StringHelpers.starts_with/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The target is an i64 helper, not a string-return helper. Its normal path is a
string scan that returns `0` or `1`, but its dev-only debug branch contains:

- `env.get("HAKO_STAGEB_DEBUG")`
- string `print(...)`
- `"" + i` style debug concat

The `String + i64` temporary may be recorded in MIR `value_types` as raw ABI
`i64`. That is not a numeric proof; it is a string handle at the ABI boundary.

## Decision

Keep the target in the existing `generic_i64_body` shape and widen only the
debug string surface already used by scanner helpers:

- accept no-result backend `print(value)` inside generic i64 bodies
- allow i64 `0`/`1` values to feed a Bool PHI destination
- treat `String + ...` as semantic proof that the Add result is a string handle,
  even when the pre-existing MIR value metadata says raw `i64`
- keep non-string numeric uses strict: a value refined to String still rejects
  if it is later used as a numeric i64 operand
- normalize the `.hako` debug guard to `dbg == "1"` so the helper does not
  carry mixed numeric/string debug-env comparisons

This does not add a by-name `StringHelpers.starts_with/3` backend rule. The
backend still consumes only the MIR-owned `generic_i64_body` proof.

## Evidence

The MIR JSON route now records the three callsites as direct generic i64 calls:

```text
BuildBox._is_freeze_tag/1 -> StringHelpers.starts_with/3
  tier=DirectAbi
  target_shape=generic_i64_body
  proof=typed_global_call_generic_i64

BuildProgramFragmentBox._is_freeze_tag/1 -> StringHelpers.starts_with/3
  tier=DirectAbi
  target_shape=generic_i64_body
  proof=typed_global_call_generic_i64

StringHelpers.starts_with_kw/3 -> StringHelpers.starts_with/3
  tier=DirectAbi
  target_shape=generic_i64_body
  proof=typed_global_call_generic_i64
```

The top pure-first source-execution stop moved past
`StringHelpers.starts_with/3` to the next owner boundary:

```text
target_shape_blocker_symbol=FuncScannerBox.scan_all_boxes/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q generic_i64 --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p153_stringhelpers_starts_with.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p153_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
