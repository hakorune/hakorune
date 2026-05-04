---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380W, BuildBox emit Program(JSON) canonical imported static call
Related:
  - docs/development/current/main/phases/phase-29cv/P380V-STRING-OR-VOID-DIRECT-CHILD-EVIDENCE.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/runner/json_v0_bridge/lowering/expr/call_ops.rs
  - src/mir/global_call_route_plan/program_json_emit_body.rs
---

# P380W: BuildBox Emit Program(JSON) Canonical Call

## Problem

P380V moved phase29cg past
`Stage1InputContractBox.resolve_emit_program_source_text/0`. The next blocker
is:

```text
reason=missing_multi_function_emitter
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The raw stage1 source owner body is intentionally small:

```hako
return BuildBox.emit_program_json_v0(source_text, null)
```

After Program(JSON v0) bridge lowering this becomes a runtime `boxcall` on the
import receiver string `lang.compiler.build.build_box`. That prevents the
existing `ProgramJsonEmitBody` classifier from seeing the canonical static
global call it already owns:

```text
BuildBox.emit_program_json_v0/2
```

If this is solved by adding method-call or Map/Box semantics to generic string
classification, Stage0 grows in the wrong place. The fix belongs at the
Program(JSON) bridge owner seam: imported static calls that are already known
compiler-entry contracts should lower to their canonical static call form.

## Decision

Do not add a new `GlobalCallTargetShape`, C emitter, or generic method-call
acceptance.

Add a narrow Program(JSON v0) bridge static-call contract for the existing
source-to-Program entry:

```text
using lang.compiler.build.build_box as BuildBox
BuildBox.emit_program_json_v0(src, opts)
  -> Global("BuildBox.emit_program_json_v0/2")
```

This is not a general `MapBox`/`BoxCall` fallback. It is the canonical
source-owner route that lets the existing `ProgramJsonEmitBody` capsule consume
the raw wrapper without rediscovering body semantics in Stage0.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new C emitter
- no generic acceptance of runtime `boxcall`
- no generic acceptance of arbitrary imported alias method calls
- no widening of `generic_string_body` or `generic_i64_body`
- no VM/compat fallback

## Acceptance

```bash
cargo test --release imported_alias_qualified_call -- --nocapture
cargo test --release stage1_raw_program_json -- --nocapture

bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p380w_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380w_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond
`Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1`, and the
raw wrapper gets `target_shape=program_json_emit_body` through the existing
route fact.

## Result

Accepted.

- Added a narrow Program(JSON v0) bridge canonicalization for imported
  `lang.compiler.build.build_box.emit_program_json_v0/2` calls so the MIR body
  now contains `BuildBox.emit_program_json_v0/2` as a static global call instead
  of a runtime `boxcall`.
- Updated the imported-alias bridge smoke to reject `boxcall`/method-call
  residue for this route.
- The raw stage1 wrapper now moves past the prior
  `generic_string_unsupported_method_call` blocker. The next blocker is the
  missing canonical target definition/fact for the static BuildBox entry:

```text
target_shape_blocker_symbol=BuildBox.emit_program_json_v0/2
target_shape_blocker_reason=generic_string_global_target_missing
```

Validation:

```text
cargo test --release imported_alias_qualified_call -- --nocapture
cargo test --release stage1_raw_program_json -- --nocapture
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

`phase29cg_stage2_bootstrap_phi_verify.sh` is still red, but the blocker moved
from `Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1`
method-call shape to the next canonical target-definition seam above.
