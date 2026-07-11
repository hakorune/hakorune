# P381FL BuildBox Parse Wrapper Collapse

Date: 2026-05-06
Scope: remove the public `BuildBox.emit_program_json_v0/2 ->
_parse_program_json_from_scan_src/1` hop so the live blocker chain starts at the
private parse owner and enrich path.

## Context

P381FK collapsed `_emit_program_json_from_scan_src/1` to the public
`BuildBox.emit_program_json_v0/2` seam.

The next live probe still showed:

```text
BuildBox.emit_program_json_v0/2
  -> BuildBox._parse_program_json_from_scan_src/1   Unsupported
  -> BuildProgramFragmentBox.enrich/2               Unsupported
```

That left one more wrapper hop above the real parser-private owner.

## Change

`BuildBox.emit_program_json_v0/2` now resolves `parse_src` itself and calls the
private parser owner directly:

```text
parse_src = BuildBox._resolve_parse_src(src)
ast_json = BuildBox._parse_program_json(parse_src, src)
```

`BuildBox._parse_program_json_from_scan_src/1` remains as a helper, but it is no
longer on the public authority path.

## Result

The public BuildBox blocker chain is now:

```text
BuildBox.emit_program_json_v0/2
  -> BuildBox._parse_program_json/2                 Unsupported
  -> BuildProgramFragmentBox.enrich/2               Unsupported
```

This is cleaner because the remaining unsupported work now starts at the actual
private parse owner and enrich owner, not at a wrapper that only re-packed those
calls.

## Verification

```bash
./target/release/hakorune --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_probe_after_parse_wrapper.mir.json lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
