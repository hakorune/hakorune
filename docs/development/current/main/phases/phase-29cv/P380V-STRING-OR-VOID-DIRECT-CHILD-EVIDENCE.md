---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380V, string-or-void direct child evidence
Related:
  - docs/development/current/main/phases/phase-29cv/P380U-MODULE-GENERIC-LEGACY-EXTERNCALL-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P136-GLOBAL-CALL-STRING-VOID-SENTINEL-BODY.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/string_return_profile.rs
---

# P380V: String-Or-Void Direct Child Evidence

## Problem

P380U moved phase29cg past module generic legacy `externcall` emission. The next
blocker is:

```text
reason=missing_multi_function_emitter
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`Stage1InputContractBox.resolve_emit_program_source_text/0` returns either:

- the result of `Stage1InputContractBox.resolve_source_text/0`, which already
  has `target_shape=generic_pure_string_body`
- a missing-input fail-fast branch

The first hypothesis was that this should use the existing P136
`generic_string_or_void_sentinel_body` contract. The direct-child fixtures proved
that classifier path is already covered. The real Program(JSON) bridge blocker
was narrower: the source owner still emitted a debug flag comparison
`_stage1_debug_on() == 1`. In the Program(JSON) bridge the helper return type is
`?`, so the string classifier saw `ScalarOrVoid == I64` and reported the
void-sentinel reason.

## Decision

Keep the existing `generic_string_or_void_sentinel_body` shape and add a narrow
fixture for a direct generic-string child plus void sentinel return.

If the fixture exposes a classifier gap, fix the string return profile so a
direct child with `target_shape=generic_pure_string_body` counts as concrete
string evidence for the wrapper.

If the fixture is already green, do not widen the classifier. Instead, clean the
stage1 input owner contract so missing source text uses the existing empty
string sentinel and callers fail-fast on `""`. Also keep the debug flag as a
direct 0/1 condition rather than comparing an unknown-return helper against an
integer literal. This removes the unnecessary void/scalar-or-void edge from the
source owner without teaching Stage0 another body shape.

The Program(JSON)->MIR bridge must also keep backend-shape normalization before
semantic metadata refresh. Rewriting console-print calls after metadata emission
creates stale route facts; the module body and LoweringPlan must be refreshed
from the same MIR shape.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new C emitter
- no special case for `Stage1InputContractBox`
- no source rewrite outside the stage1 input/emit owner contract
- no acceptance of unknown child global targets
- no post-metadata JSON rewrite that can stale LoweringPlan facts

## Acceptance

```bash
cargo test --release direct_child_string_or_void -- --nocapture
cargo test --release debug_print_direct_child_string_guard -- --nocapture

bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380v_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380v_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker should move beyond
`Stage1InputContractBox.resolve_emit_program_source_text/0` without adding a
new body shape.

## Result

Accepted.

- Added direct-child fixtures for string-or-void and debug-print string guard
  shapes. They pass without classifier widening.
- Normalized `Stage1InputContractBox.resolve_emit_program_source_text/0` to use
  `""` as the missing-source sentinel and changed the debug flag check from
  `_stage1_debug_on() == 1` to the direct 0/1 condition.
- Moved Program(JSON) bridge backend-shape normalization before semantic
  metadata refresh so the body and route facts are derived from the same MIR
  shape.
- `phase29cg_stage2_bootstrap_phi_verify.sh` now moves past
  `Stage1InputContractBox.resolve_emit_program_source_text/0`; the next blocker
  is:

```text
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```
