# P381FU Defs Text Fragment Seam

Date: 2026-05-06
Scope: BoxShape cleanup for Program(JSON v0) defs enrichment.

## Result

`BuildProgramFragmentBox._inject_defs_json/2` no longer calls
`FuncScannerBox.scan_all_boxes/1` or consumes its ArrayBox/MapBox result on the
public BuildBox Program(JSON v0) path.

The scanner owner now exposes a text-return seam:

```text
FuncScannerBox.collect_defs_fragment_json(source)
  -> "" | ",\"defs\":[...]"
```

`BuildProgramFragmentBox._inject_defs_json/2` consumes that text fragment and
then calls `inject_json_fragment/2`, matching the imports/enum-decls enrichment
shape.

## Structural Changes

- Added `FuncScannerBox.collect_defs_fragment_json/1` as the public defs
  fragment text seam for BuildBox.
- Added `FuncScannerBox._scan_methods_defs_json_array/4` to build defs JSON text
  without MapBox def records.
- Added text helpers for identifier and params JSON construction:
  `_scan_ident_text/2`, `_params_json_text/2`, `_first_param_name_text/1`.
- Added `FuncScannerHelpersBox._find_header_open_brace/2`, completing the
  existing `_seek_*_body_open_brace` helper seam.
- Isolated method-body parsing behind `FuncScannerBox._parse_method_body_json/1`.

## Evidence

Probe:

```bash
timeout 240 target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_defs_text_probe3.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed route:

```text
BuildBox.emit_program_json_v0/2
  -> BuildProgramFragmentBox._inject_defs_json/2
     blocker: FuncScannerBox._parse_method_body_json/1
  -> BuildProgramFragmentBox._inject_enum_decls_json/2 DirectAbi
  -> BuildProgramFragmentBox._inject_imports_json/2    DirectAbi
```

Semantic sanity:

```bash
timeout 240 target/release/hakorune \
  --emit-program-json-v0 /tmp/hakorune_stage1_cli_env_defs_text_program.json \
  lang/src/runner/stage1_cli_env.hako
```

Compared with the previous Program(JSON v0) probe, `defs`, `imports`, and
`enum_decls` are identical.

## Remaining Blocker

Follow-up: `P381FV-DEFS-METHOD-BODY-PARSE-PROGRAM-SEAM` removes this blocker
from the public BuildBox path by routing method-body parsing through
`ParserBox.parse_program2`.

The defs scanner text path is now blocked at:

```text
FuncScannerBox._parse_method_body_json/1
  reason: generic_string_unsupported_instruction
```

This is the expected next owner seam: method-body parsing still creates a
`ParserBox` and calls `parse_block2`. Keep this as a parser-owner cleanup; do
not reintroduce `scan_all_boxes/1` object results or add broad object-return
DirectAbi acceptance.
