# P381FW BuildFragment Dead Defs Builders Prune

Date: 2026-05-06
Scope: BoxShape cleanup after defs text-fragment promotion.

## Result

Removed the unused object-defs builder helpers from
`BuildProgramFragmentBox`:

```text
BuildProgramFragmentBox._build_defs_json_with_source/1
BuildProgramFragmentBox._build_defs_json/1
BuildProgramFragmentBox._build_params_json/1
```

These helpers belonged to the old `FuncScannerBox.scan_all_boxes/1`
ArrayBox/MapBox result path. The public BuildBox Program(JSON v0) path now
uses only:

```text
BuildProgramFragmentBox._inject_defs_json/2
  -> FuncScannerBox.collect_defs_fragment_json/1
```

## Evidence

Probe:

```bash
timeout 240 target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_defs_dead_prune_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed:

```text
BuildProgramFragmentBox._build_defs_json_with_source/1: absent
BuildProgramFragmentBox._build_defs_json/1:             absent
BuildProgramFragmentBox._build_params_json/1:           absent
BuildBox.emit_program_json_v0/2 bad routes:             0
BuildProgramFragmentBox._inject_defs_json/2 bad routes: 0
```

Semantic sanity:

```bash
timeout 240 target/release/hakorune \
  --emit-program-json-v0 /tmp/hakorune_stage1_cli_env_defs_dead_prune_program.json \
  lang/src/runner/stage1_cli_env.hako
```

Compared with the previous parse-program defs probe:

```text
all_equal true
defs true
imports true
enum_decls true
body true
user_box_decls true
```

## Remaining Blocker

The BuildBox public Program(JSON v0) enrichment path remains direct. The
remaining concrete owner blocker is still the parser-private seam:

```text
BuildBox._parse_program_json/2
  -> ParserBox.parse_program2
```
