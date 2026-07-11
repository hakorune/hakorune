# P381FJ BuildBox Wrapper Blocker Refresh

Date: 2026-05-06
Scope: refresh the live Program(JSON) blocker after P381FH so the next slice
targets the real unsupported wrapper chain instead of the wrong direct owner.

## Context

P381BS correctly locked that deleting the dedicated parser body emitter was not
safe while the live owner had drifted to:

```text
BuildBox._parse_program_json(parse_src, scan_src)/2
```

After P381FH, the next tempting move would still be "fix `_parse_program_json/2`
first". The live Stage1 CLI probe shows that is one step too low for the next
cleanup slice.

## Probe

Command:

```bash
./target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed same-module blocker chain:

```text
BuildBox._emit_program_json_from_scan_src/1
  -> BuildBox._parse_program_json_from_scan_src/1
     proof=typed_global_call_contract_missing
     tier=Unsupported
     reason=missing_multi_function_emitter
     target_shape_blocker_symbol=ParserBox.parse_program2

BuildBox._emit_program_json_from_scan_src/1
  -> BuildProgramFragmentBox.enrich/2
     proof=typed_global_call_contract_missing
     tier=Unsupported
     reason=missing_multi_function_emitter
     target_shape_blocker_symbol=FuncScannerBox.scan_all_boxes/1

BuildBox._parse_program_json_from_scan_src/1
  -> BuildBox._parse_program_json/2
     proof=typed_global_call_contract_missing
     tier=Unsupported
     reason=missing_multi_function_emitter
     target_shape_blocker_symbol=ParserBox.parse_program2
```

and:

```text
BuildBox._parse_program_json/2
  global_call_routes = []
```

## Decision

Do not treat the next slice as a direct `_parse_program_json/2` contract add.

The real next cleanup target is the unsupported BuildBox wrapper chain:

1. `BuildBox._emit_program_json_from_scan_src/1`
2. `BuildBox._parse_program_json_from_scan_src/1`
3. only then the private owner `_parse_program_json/2`

That keeps the lane on source-owner cleanup / wrapper collapse instead of
teaching Stage0 parser-private meaning too early.

## Result

The remaining parser-adjacent work is now ordered more precisely:

- public Stage1 helper boundary stays the only accepted runtime owner
- wrapper-chain cleanup comes before any new parser-private contract discussion
- `_parse_program_json/2` remains a private root mismatch, not the immediate
  next edit point
