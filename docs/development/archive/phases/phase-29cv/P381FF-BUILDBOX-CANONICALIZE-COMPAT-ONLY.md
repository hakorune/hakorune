# P381FF BuildBox Canonicalize Compat-Only

Date: 2026-05-06
Scope: remove the BuildBox-specific Stage1 Program(JSON) rewrite from `callsite_canonicalize` now that MIR-owned route facts carry the lowering contract.

## Context

Before P381FD/P381FE, `callsite_canonicalize` still rewrote:

- `BuildBox.emit_program_json_v0/2` with null opts
- `BuildBox._emit_program_json_from_scan_src/1`

directly to:

```text
nyash.stage1.emit_program_json_v0_h
```

That kept Stage0 alive, but after the global-call route fact and private-parser
fail-fast work landed, the rewrite was redundant. The lowering contract already
exists in MIR metadata.

## Change

Removed the BuildBox-specific rewrite from `callsite_canonicalize`.

- BuildBox authority calls now stay as global calls in MIR
- Stage1 Program(JSON) helper lowering is driven by `global_call_routes`
  metadata instead of mutation in the canonicalization pass
- compatibility language in the pass docs/tests now reflects that BuildBox
  routing is MIR-owned, not rewrite-owned

## Verification

Commands:

```bash
cargo test -q mir::passes::callsite_canonicalize::tests
cargo test -q runner::mir_json_emit::tests::global_call_routes::parser_program_json
cargo test -q host_providers::mir_builder::tests::test_imported_alias_qualified_call_uses_json_imports
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

`callsite_canonicalize` no longer owns the BuildBox Stage1 Program(JSON) route.
The next cleanup can stay focused on runtime-helper ownership and uniform wrapper
emission instead of carrying a second truth path for the same lowering contract.
