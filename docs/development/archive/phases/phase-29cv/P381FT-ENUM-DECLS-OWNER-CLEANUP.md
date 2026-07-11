# P381FT Enum Decls Owner Cleanup

Date: 2026-05-06
Scope: BoxShape cleanup for Program(JSON v0) enum declaration enrichment.

## Result

`BuildProgramFragmentBox._inject_enum_decls_json/2` is now a DirectAbi
`generic_pure_string_body` child of `BuildBox.emit_program_json_v0/2`.

The active enum blocker moved from parser-private helper ownership to a shared
text helper surface and then closed:

```text
before:
  _inject_enum_decls_json/2
    -> ParserEnumInventoryBox.collect/1
       blocker: ParserStringUtilsBox.is_alpha/1

after:
  _inject_enum_decls_json/2
    -> ParserEnumInventoryBox.collect/1
       shape: generic_pure_string_body
```

## Structural Changes

- `ParserEnumInventoryBox` no longer depends on
  `ParserStringUtilsBox.is_alpha` / `ParserStringUtilsBox.index_of` for the
  Program(JSON v0) enum inventory path.
- `ParserIdentScanBox.scan_ident/2` now uses shared `StringHelpers` instead of
  the parser-private string-utils wrapper.
- `StringHelpers.is_alpha/1` is expressed through the existing
  `StringHelpers.index_of/3` generic-i64 surface, avoiding an extra range-compare
  shape in the helper.
- enum keyword boundary checks are local to the enum collector path, so
  `_kw_boundary_before/2` / `_kw_boundary_after/2` are no longer on the public
  enrichment route.
- `StringHelpers.last_index_of/2` replaces receiver `lastIndexOf` calls in the
  enum scanner path.

## Evidence

Probe:

```bash
timeout 180 target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_enum_owner_probe_final.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed route:

```text
BuildBox.emit_program_json_v0/2
  -> BuildProgramFragmentBox._inject_defs_json/2       Unsupported
     blocker: FuncScannerBox.scan_all_boxes/1
  -> BuildProgramFragmentBox._inject_enum_decls_json/2 DirectAbi
  -> BuildProgramFragmentBox._inject_imports_json/2    DirectAbi
```

## Next

Continue with defs owner cleanup. Do not add ArrayBox/MapBox return acceptance
for `FuncScannerBox.scan_all_boxes/1`; move the public Build fragment path to a
scanner-owned text fragment seam instead.
