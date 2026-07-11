# P381FE Private Parser Fail-Fast

Date: 2026-05-06
Scope: keep `typed_global_call_parser_program_json` as diagnostics metadata while removing it from Stage0 definition planning and lowering.

## Context

After P381FD, the public BuildBox authority seam already lowers through a MIR-owned
Stage1 runtime route fact. The remaining parser-body proof:

```text
typed_global_call_parser_program_json
```

still existed as metadata, but ny-llvmc could continue to treat it like a normal
uniform same-module definition target if that proof leaked back into Stage0.

That is the wrong ownership boundary for the private parser helper.

## Change

Adjusted Stage0 metadata consumers so `typed_global_call_parser_program_json`
remains observable but is no longer planned or emitted as a same-module function
definition target.

- same-module definition planning now excludes the parser-program proof
- same-module prepass no longer treats the parser-program proof as an emit-ready
  uniform MIR definition
- `emit_global_call_lowering_plan_mir_call(...)` fail-fast rejects that proof
  instead of lowering it

The route metadata still exists for diagnostics and blocker reporting; it is no
longer a Stage0 lowering contract.

## Verification

Commands:

```bash
cargo test -q mir::global_call_route_plan::tests::shape_reasons
cargo test -q runner::mir_json_emit::tests::global_call_routes::parser_program_json
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Stage0 now treats the private parser helper proof as diagnostics-only metadata.
The public BuildBox authority route stays live through the Stage1 runtime helper,
and the next cleanup slice can focus on wrapper routing/runtime ownership instead
of letting parser-body lowering slip back into ny-llvmc.
