# P381CU Module Generic Method Dispatch View Reuse

Date: 2026-05-05
Scope: reuse the generic-method LoweringPlan view in the Stage0 method body dispatch.

## Context

P381CT made the module-generic prepass read each LoweringPlan view once per
call instruction. The body emitter still repeated the older pattern: each
method-specific emitter found the same lowering-plan entry and re-read the same
generic-method view before checking its predicate.

That kept the data ownership split across the dispatch chain even though the
site has only one route contract.

## Change

`module_generic_string_emit_mir_call` now reads the generic-method view once
for method calls and passes the cached view pointer to each method-specific
emitter:

- len
- push
- get
- keys
- set
- substring
- indexOf
- lastIndexOf
- contains

The method emitters still own their route-specific predicates and emission
logic. This is a view reuse cleanup only; it does not add accepted routes.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cu_dispatch_view.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cu_dispatch_view.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cu_dispatch_view.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cu_emit_program.out \
  /tmp/hakorune_p381cu_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cu_emit_program.out \
  /tmp/hakorune_p381cu_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cu_dispatch_view.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cu_emit_mir.out \
  /tmp/hakorune_p381cu_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cu_emit_mir.out \
  /tmp/hakorune_p381cu_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic method dispatch now has one LoweringPlan generic-method view
read per method call site. Individual emitters consume that view instead of
re-opening the same site contract.
