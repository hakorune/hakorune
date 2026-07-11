# P381FV Defs Method Body Parse Program Seam

Date: 2026-05-06
Scope: BoxShape cleanup for Program(JSON v0) defs enrichment.

## Result

`FuncScannerBox._parse_method_body_program_json/1` no longer calls
`ParserBox.parse_block2/2` or strips the `@pos` suffix from a block parse
result.

The public defs text path now uses the same parser-owned Program(JSON v0) seam
as the main BuildBox parse path:

```text
FuncScannerBox._parse_method_body_program_json(method_body)
  -> ParserBox.parse_program2(method_body)
```

`FuncScannerBox._scan_methods_defs_json_array/4` now treats the returned value
as complete Program(JSON v0), so it injects:

```json
"body": {"version":0,"kind":"Program","body":[...]}
```

directly, without wrapping a block array locally.

## Structural Changes

- Moved the empty-method-body skip to the scanner caller.
- Reduced `FuncScannerBox._parse_method_body_program_json/1` to the parser seam only:
  construct `ParserBox`, enable stage3, and call `parse_program2`.
- Removed the method-body `parse_block2` dependency from the public defs
  enrichment route.
- Kept legacy `scan_all_boxes/1` object APIs unchanged; they are not part of
  the public BuildBox Program(JSON v0) path.

## Evidence

Probe:

```bash
timeout 240 target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_program_defs_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed route:

```text
BuildBox.emit_program_json_v0/2
  -> BuildProgramFragmentBox._inject_defs_json/2 DirectAbi
  -> BuildProgramFragmentBox._inject_enum_decls_json/2 DirectAbi
  -> BuildProgramFragmentBox._inject_imports_json/2 DirectAbi

BuildProgramFragmentBox._inject_defs_json/2
  -> FuncScannerBox.collect_defs_fragment_json/1 DirectAbi

FuncScannerBox._scan_methods_defs_json_array/4
  -> FuncScannerBox._parse_method_body_program_json/1
     proof: typed_global_call_parser_program_json
     tier: DirectAbi
     owner: diagnostics_only
```

Semantic sanity:

```bash
timeout 240 target/release/hakorune \
  --emit-program-json-v0 /tmp/hakorune_stage1_cli_env_parse_program_defs_program.json \
  lang/src/runner/stage1_cli_env.hako
```

Compared with the previous defs text-fragment probe, the full Program(JSON v0)
is identical:

```text
defs true
imports true
enum_decls true
body true
user_box_decls true
all_equal true
```

## Remaining Blocker

The BuildBox public Program(JSON v0) enrichment path is now direct through
defs/imports/enum_decls.

The remaining concrete parser-owned seam is:

```text
BuildBox._parse_program_json/2
  -> ParserBox.parse_program2
```

Keep this separate from enrichment cleanup. Do not reintroduce block parsing or
object-return scanner APIs into the public Build path.
