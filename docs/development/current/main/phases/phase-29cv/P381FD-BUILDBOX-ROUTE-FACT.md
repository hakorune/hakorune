# P381FD BuildBox Route Fact

Date: 2026-05-06
Scope: move the Stage1 Program(JSON v0) handoff from callsite-only rewriting into MIR-owned global-call route facts.

## Context

Stage0 already knew how to emit the Stage1 Program(JSON) helper as an extern
route:

```text
nyash.stage1.emit_program_json_v0_h
```

but `BuildBox` calls only reached that helper after MIR callsite rewriting.
`global_call_routes` still described the relevant calls as same-module/direct user
calls, so the lowering plan did not carry an explicit Stage1 runtime boundary for
the BuildBox authority seam.

## Change

Added a dedicated MIR-owned global-call route fact for the Stage1 Program(JSON)
handoff.

- `BuildBox.emit_program_json_v0(src, null)`
- `BuildBox._emit_program_json_from_scan_src(scan_src)`

now publish:

- `proof = typed_global_call_stage1_emit_program_json`
- `route_kind = stage1.emit_program_json_v0`
- `tier = ColdRuntime`
- `emit_kind = runtime_call`
- `target_symbol = nyash.stage1.emit_program_json_v0_h`
- `definition_owner = runtime_helper`

The ny-llvmc `mir_call` shims now consume that global-call route fact directly
for need scanning and runtime helper emission. The existing callsite
canonicalization stays in place as a compatibility path; it is no longer the only
place where Stage1 Program(JSON) intent exists.

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

Stage0 now reads a MIR-owned BuildBox runtime route fact instead of depending
only on callsite rewriting to discover the Stage1 Program(JSON) helper. The next
cleanup slice can focus on private parser helper reachability and delete-last
cleanup, not on reintroducing another backend-local BuildBox branch.
