# P381FX Defs Method Body Program Name Cleanup

Date: 2026-05-06
Scope: naming cleanup after defs method-body parser seam promotion.

## Result

Renamed the defs scanner method-body parser seam:

```text
FuncScannerBox._parse_method_body_json/1
  -> FuncScannerBox._parse_method_body_program_json/1
```

The old name implied that the helper returned a raw body/block JSON fragment.
After P381FV it returns complete Program(JSON v0), because it delegates to
`ParserBox.parse_program2`. The new name matches the contract.

## Evidence

Probe:

```bash
timeout 240 target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_defs_rename_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed:

```text
FuncScannerBox._parse_method_body_json/1:         absent
FuncScannerBox._parse_method_body_program_json/1: present, bad routes 0
FuncScannerBox._scan_methods_defs_json_array/4:
  -> FuncScannerBox._parse_method_body_program_json/1
     tier: DirectAbi
     proof: typed_global_call_parser_program_json
BuildBox.emit_program_json_v0/2 bad routes:       0
BuildProgramFragmentBox._inject_defs_json/2 bad routes: 0
```

Semantic sanity:

```bash
timeout 240 target/release/hakorune \
  --emit-program-json-v0 /tmp/hakorune_stage1_cli_env_defs_rename_program.json \
  lang/src/runner/stage1_cli_env.hako
```

Compared with the previous dead-builder prune probe:

```text
all_equal true
defs true
imports true
enum_decls true
body true
user_box_decls true
```

## Remaining Blocker

No new blocker was introduced. The remaining concrete owner blocker is still:

```text
BuildBox._parse_program_json/2
  -> ParserBox.parse_program2
```
