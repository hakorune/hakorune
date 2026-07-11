# 2978 MIRBUILDER-PROGRAMJSON-V0-SCANNER-READ-CHAR-AOT-LOWERING-001

Status: Landed  
Date: 2026-07-05  
Scope: ProgramJsonV0ScannerBox AOT lowering unblock for the first scanner helper.

## Decision

Fix the first ProgramJSON scanner AOT blocker before opening the ProgramJSON
snapshot parity runner.

Selected slice:

```text
ProgramJsonV0ScannerBox._read_char/2
```

The fix is intentionally below ProgramJSON traversal parity and below MIR
mutation/lowering migration claims. It only unblocks same-module AOT lowering
for the scanner helper shape.

## Implementation

- Accept `I64`/`Bool` vs `VoidSentinel` `Eq`/`Ne` comparisons in generic string
  body analysis. This covers null-guarded scalar/string helper bodies after
  later comparisons refine an unknown value to scalar.
- Fix the same-module method-view registry rule for `string_substring` so a
  `return_shape = null` lowering-plan row can match the existing direct
  substring emitter.
- Add a Rust route unit test for the `_read_char`-style CFG:
  null receiver/index guards return empty string, valid path returns substring.

## Evidence

```text
cargo test -q refresh_module_semantic_metadata_accepts_read_char_null_guard_string_body --lib
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_v0_scanner_aot_blocker_inventory_guard.sh
```

Inventory result:

```text
probe=read_char mir_verify=green aot_emit=green blocker_symbol=-
probe=seek_after mir_verify=green aot_emit=green blocker_symbol=-
probe=seek_obj_end mir_verify=green aot_emit=blocked reason=module_generic_prepass_failed blocker_symbol=ProgramJsonV0ScannerBox.seek_obj_end_unescaped/2
probe=seek_obj_field_value_start mir_verify=green aot_emit=blocked reason=module_generic_prepass_failed blocker_symbol=ProgramJsonV0ScannerBox.seek_obj_field_value_start/3
probe=seek_obj_field_obj_start mir_verify=green aot_emit=blocked reason=module_generic_prepass_failed blocker_symbol=ProgramJsonV0ScannerBox.seek_obj_field_obj_start/3
```

## Non-Claims

```text
programjson_snapshot_parity_claim = 0
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_migration = 0
full_recipe_matcher_execution = 0
rust_astnode_projector_fully_retired = 0
programjson_full_parser_claim = 0
runtime_fallback = 0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-V0-SCANNER-SEEK-OBJ-END-UNESCAPED-AOT-LOWERING-001
```

Unblock `ProgramJsonV0ScannerBox.seek_obj_end_unescaped/2` as the next
module-generic scanner helper. Do not add another ProgramJSON snapshot facade
before this AOT blocker advances or exposes a concrete missing backend
capability.
