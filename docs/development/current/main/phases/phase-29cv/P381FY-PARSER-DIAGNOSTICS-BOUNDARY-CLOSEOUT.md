# P381FY Parser Diagnostics Boundary Closeout

Date: 2026-05-06
Scope: close the `ParserBox.parse_program2` blocker reading after the source-owner Program(JSON v0) seam cleanup.

## Decision

`ParserBox.parse_program2` is not the next live Stage0 lowering blocker.

It remains an intentional diagnostics-only proof boundary:

- `typed_global_call_parser_program_json`
- `definition_owner=diagnostics_only`
- `return_shape=string_handle`
- no Stage0 definition owner is emitted for C lowering readiness

Live source-owner Program(JSON v0) calls must continue through the public
BuildBox seam and the Stage1 runtime helper:

- `BuildBox.emit_program_json_v0(source, null)`
- `route_kind=stage1.emit_program_json_v0`
- `definition_owner=runtime_helper`
- `target_symbol=nyash.stage1.emit_program_json_v0_h`

## Evidence

Probe:

```bash
target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_parser_boundary_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed source-owner route:

```text
Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
  route_kind=stage1.emit_program_json_v0
  tier=ColdRuntime
  emit_kind=runtime_call
  proof=typed_global_call_stage1_emit_program_json
  definition_owner=runtime_helper
  target_symbol=nyash.stage1.emit_program_json_v0_h

MirBuilderSourceCompatBox._emit_program_json_from_source_raw/2
  route_kind=stage1.emit_program_json_v0
  tier=ColdRuntime
  emit_kind=runtime_call
  proof=typed_global_call_stage1_emit_program_json
  definition_owner=runtime_helper
  target_symbol=nyash.stage1.emit_program_json_v0_h
```

Observed parser proof route:

```text
BuildBox.emit_program_json_v0/2 -> BuildBox._parse_program_json/2
  tier=DirectAbi
  proof=typed_global_call_parser_program_json
  definition_owner=diagnostics_only
  return_shape=string_handle

FuncScannerBox._scan_methods_defs_json_array/4 -> FuncScannerBox._parse_method_body_program_json/1
  tier=DirectAbi
  proof=typed_global_call_parser_program_json
  definition_owner=diagnostics_only
  return_shape=string_handle
```

This is a valid closed state. C lowering readiness does not accept
`diagnostics_only`, so the parser proof cannot silently become a Stage0-owned
lowered body.

## Cleanup Result

Closed as policy/docs cleanup only:

- no Rust lowering change
- no `.hako` workaround
- no parser-body owner promotion
- no proof-name C fallback revival

The next cleanup lane is the remaining T5 owner/body inventory:

- generic string-or-void sentinel plumbing
- `PatternUtil` local-value probe body handling
- `BoxTypeInspector` describe body handling

## References

- `P381FE-PRIVATE-PARSER-FAILFAST.md`
- `P381FH-PARSER-PROOF-DIAGNOSTICS-OWNER.md`
- `P381FI-STAGE0-CLEANUP-REMAINING-INVENTORY.md`
- `P381FN-CONCRETE-BLOCKER-ORDER.md`
