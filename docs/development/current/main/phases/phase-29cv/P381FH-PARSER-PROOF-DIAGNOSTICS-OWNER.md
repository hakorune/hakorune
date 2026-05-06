# P381FH Parser Proof Diagnostics Owner

Date: 2026-05-06
Scope: move `typed_global_call_parser_program_json` from ad-hoc Stage0 proof guards to MIR-owned `definition_owner=diagnostics_only`.

## Context

P381FE already made the private parser helper fail-fast in Stage0, but the C
shims still needed a parser-proof-specific exclusion:

```text
typed_global_call_parser_program_json
  -> parsed as a direct global-call contract
  -> then manually excluded from uniform definition planning/lowering
```

That left one proof-name special case in Stage0.

## Change

The parser Program(JSON) proof still exists for diagnostics, but it no longer
pretends to be a same-module definition owner.

- `GlobalCallRoute` now publishes
  `definition_owner=diagnostics_only` for
  `typed_global_call_parser_program_json`
- the trace consumer also comes from MIR metadata:
  `mir_call_global_diagnostics_only_emit`
- the diagnostics recipe recognizes the live
  `_parse_program_json(parse_src, scan_src)/2` body, including
  `set_enum_inventory_from_source(scan_src)`, instead of only the older
  one-argument parser helper shape
- ny-llvmc no longer needs parser-proof-specific guards in
  `hako_llvmc_ffi_lowering_plan_metadata.inc` or
  `hako_llvmc_ffi_mir_call_shell.inc`

## Verification

Commands:

```bash
cargo test -q mir::global_call_route_plan::tests::shape_reasons
cargo test -q runner::mir_json_emit::tests::global_call_routes::parser_program_json
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Stage0 now reads the private parser helper status entirely from MIR-owned owner
metadata instead of a proof-name denylist. The proof remains available for
diagnostics and blocker reporting, while the actual lowering owner stays the
public Stage1 runtime helper route.
