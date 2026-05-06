# P381FS Imports Owner Cleanup

Date: 2026-05-06
Scope: finish the `_inject_imports_json/2` source-owner cleanup without adding
new Stage0 lowering semantics.

## Context

After P381FR, bundle and non-bundle Program(JSON v0) paths both entered the
public `BuildBox.emit_program_json_v0(..., null)` seam. The remaining imports
side stop was:

```text
BuildProgramFragmentBox._inject_imports_json/2
  -> UsingCollectorBox.collect/1
     blocker: ParserCommonUtilsBox.esc_json/1
```

That was an owner problem, not a reason to teach Stage0 another parser-private
helper.

## Change

- `UsingCollectorBox.collect/1` now uses shared `StringHelpers` instead of
  parser-private `ParserCommonUtilsBox` / `ParserScanLoopBox`.
- JSON string emission moved from `ParserCommonUtilsBox.esc_json` plus manual
  quotes to `StringHelpers.json_quote`.
- `UsingCollectorBox.collect/1` and
  `BuildProgramFragmentBox.convert_usings_to_imports/1` no longer use local
  `null` sentinels for optional alias/path state; they use explicit `has_*`
  flags with empty-string payload slots.
- The parser Program(JSON) diagnostics recipe was completed for the live
  `_parse_program_json(parse_src, scan_src)/2` body with
  `set_enum_inventory_from_source(scan_src)`. This remains
  `definition_owner=diagnostics_only`; no parser lowering owner was added.

## Result

In the Stage1 CLI MIR probe, `_inject_imports_json/2` now has only direct
children:

```text
BuildProgramFragmentBox._inject_imports_json/2
  -> UsingCollectorBox.collect/1                  DirectAbi generic_pure_string_body
  -> BuildProgramFragmentBox.convert_usings_to_imports/1
                                                   DirectAbi generic_pure_string_body
  -> BuildProgramFragmentBox.inject_json_fragment/2
                                                   DirectAbi generic_pure_string_body
```

The concrete owner blocker list advances to:

```text
BuildBox.emit_program_json_v0/2
  -> BuildBox._parse_program_json/2
     blocker: ParserBox.parse_program2

  -> BuildProgramFragmentBox._inject_defs_json/2
     blocker: FuncScannerBox.scan_all_boxes/1

  -> BuildProgramFragmentBox._inject_enum_decls_json/2
     blocker: ParserStringUtilsBox.is_alpha/1
```

Preferred next slice:

```text
_inject_enum_decls_json/2 -> _inject_defs_json/2 -> _parse_program_json/2
```

## Verification

```bash
target/release/hakorune --emit-mir-json /tmp/hakorune_stage1_cli_env_imports_owner_probe_after3.mir.json lang/src/runner/stage1_cli_env.hako
cargo test -q mir::global_call_route_plan::tests::shape_reasons
cargo test -q runner::mir_json_emit::tests::global_call_routes::parser_program_json
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
