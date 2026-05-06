# P381FK BuildBox Emit Wrapper Collapse

Date: 2026-05-06
Scope: collapse `BuildBox._emit_program_json_from_scan_src/1` to the public
`BuildBox.emit_program_json_v0/2` seam so the next unsupported work starts one
step lower in the wrapper chain.

## Context

P381FJ refreshed the live blocker ordering and showed that the next unsupported
Program(JSON) work was not `_parse_program_json/2` directly, but the BuildBox
wrapper chain above it.

Before this slice:

```text
BuildBox._emit_program_json_from_scan_src/1
  -> BuildBox._parse_program_json_from_scan_src/1   Unsupported
  -> BuildProgramFragmentBox.enrich/2               Unsupported
```

That kept one extra wrapper carrying parser/enrich choreography even though the
public authority seam is already `BuildBox.emit_program_json_v0/2`.

## Change

- moved the parse/freeze/enrich body into `BuildBox.emit_program_json_v0/2`
- changed `BuildBox._emit_program_json_from_scan_src/1` into a thin wrapper:

```text
return BuildBox.emit_program_json_v0(scan_src, null)
```

This keeps the public seam as the only BuildBox Stage1 authority boundary while
removing one unsupported wrapper hop from the Stage0-facing inventory.

## Result

After the live Stage1 CLI probe:

```text
BuildBox._emit_program_json_from_scan_src/1
  global_call_routes = []

BuildBox.emit_program_json_v0/2
  -> BuildBox._parse_program_json_from_scan_src/1   Unsupported
  -> BuildProgramFragmentBox.enrich/2               Unsupported
```

So the next cleanup target is now the public helper body and its remaining
private/source-owner children, not the old `_emit_program_json_from_scan_src/1`
wrapper.

## Verification

```bash
cargo test -q mir::global_call_route_plan::tests::shape_reasons
cargo test -q runner::mir_json_emit::tests::global_call_routes::parser_program_json
./target/release/hakorune --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_probe_after_wrapper.mir.json lang/src/runner/stage1_cli_env.hako
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
