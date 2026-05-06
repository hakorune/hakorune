# P381GM Stage1 Emit Program Runtime Helper Repair

Date: 2026-05-06
Scope: repair Stage0 lowering for the public Stage1 Program(JSON v0) runtime-helper seam.

## Context

After P381FY, live source-owner Program(JSON v0) calls intentionally route
through the public seam:

```text
BuildBox.emit_program_json_v0(source, null)
  -> route_kind=stage1.emit_program_json_v0
  -> definition_owner=runtime_helper
  -> target_symbol=nyash.stage1.emit_program_json_v0_h
```

The C emit path already knew how to call the runtime helper, but it still
assumed the older raw one-argument helper shape. The same-module prepass also
did not recognize the `runtime_helper` global-call view, so full
`stage1_cli_env.hako` MIR stopped at:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
```

After adding prepass recognition, the next failure exposed the second half of
the same seam:

```text
error: use of undefined value '%r24'
%r23 = call i64 @"nyash.stage1.emit_program_json_v0_h"(i64 %r24)
```

The source argument had been copied through a PHI/copy chain, and the special
runtime-helper call path printed the raw register instead of the resolved copy
source.

## Change

- Same-module global-call prepass now accepts
  `lowering_plan_global_call_view_is_stage1_emit_program_json(...)`, publishes
  the i64 result type, and marks the MIR-owned string result origin.
- The Stage1 emit-program runtime-helper emitter now accepts the public
  two-argument `source, null` seam in addition to the older one-argument raw
  helper shape.
- The optional second argument must resolve to the void/null sentinel value
  `0`; it is validated and not passed to the one-argument runtime helper.
- The source argument is resolved through `resolve_copy_source_reg(...)` before
  printing the LLVM call operand.
- Added a focused MIR fixture for same-module prepass/body emission of:

```text
same-module helper -> BuildBox.emit_program_json_v0(source, null)
```

## Result

The public BuildBox seam and the Stage1 runtime helper now agree at the Stage0
C boundary:

```text
BuildBox.emit_program_json_v0(source, null)
  accepted as stage1.emit_program_json_v0 runtime helper
  emitted as nyash.stage1.emit_program_json_v0_h(source)
```

This is not a parser-body owner promotion and does not accept
`diagnostics_only` parser proofs.

## Validation

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage1_emit_program_json_runtime_helper_guard.sh
target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_stage1_emit_program_json_runtime_helper_same_module_min_v1.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381gm_stage1_runtime_helper.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in /tmp/hakorune_p381gk_stage1_cli_env.mir.json \
  --emit obj \
  --out /tmp/hakorune_repair_stage1.o
NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381gk_stage1_cli_env.mir.json \
  --emit exe \
  -o /tmp/hakorune_repair_stage1.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env /tmp/hakorune_repair_stage1.exe emit-program ...
stage1_contract_validate_emit_output emit-program ...
stage1_contract_run_bin_with_env /tmp/hakorune_repair_stage1.exe emit-mir ...
stage1_contract_validate_emit_output emit-mir ...
```

Observed:

- focused fixture OBJ: rc=0
- full `stage1_cli_env.hako` MIR OBJ: rc=0
- full `stage1_cli_env.hako` MIR EXE: rc=0
- `emit-program` runtime sanity: run rc=0, validate rc=0
- `emit-mir` runtime sanity: run rc=0, validate rc=0

Hygiene:

```bash
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
