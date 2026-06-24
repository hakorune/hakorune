---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Close the first rust-subset-to-hako EXE/AOT app-front slice.
Related:
  - apps/rust-subset-to-hako/README.md
  - apps/rust-subset-to-hako/STATUS.md
  - apps/rust-subset-to-hako/smoke.sh
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md
---

# RUST-SUBSET-TO-HAKO-EMBEDDED-AOT-PARITY-001

## Result

The first `.hako` rust-subset-to-hako converter slice now compiles and runs
through the primary EXE/AOT app validation route.

Verified:

```text
python_reference_selftest=ok
json_native_probe_exe=ok
hako_converter_mir_json_emit=ok
hako_converter_exe=ok
hako_converter_parity=ok
vm_product_route=retired
primary_app_validation_route=EXE/AOT
```

The converter currently embeds the `simple_subset.json` fixture in
`convert.hako`. This deliberately keeps the first acceptance row focused on:

```text
RustSubset JSON text
  -> json_native parser
  -> JsonNode traversal
  -> .hako skeleton emission
  -> EXE/AOT parity with simple_expected.hako
```

## Fixes Landed

```text
JsonParser.parse/1 EXE/AOT blocker=closed
JsonNodeInstance.get_node object lookup=closed for schema fixture
RustSubsetConverter helper calls=instance methods
convert loop accumulator=single emit_item path
FileBox route=out_of_scope_for_this_slice
```

The JSON tokenizer has a temporary RustSubset schema-key interning bridge. It
is scoped as an app-front compatibility bridge, not final JSON library
semantics.

Removal condition:

```text
scanner_derived_dynamic_string_map_keys_stable_on_exe_aot=1
```

## Reproduction

```bash
bash apps/rust-subset-to-hako/smoke.sh
```

Expected:

```text
summary=ok
```

## Next

```text
selected_next_task=RUST-SUBSET-TO-HAKO-INPUT-ROUTE-SELECTION-001
implementation_allowed=1
```

Goal:

```text
Choose the next input route without reopening VM product execution:

1. EXE/AOT FileBox acceptance for local files
2. host-generated embedded JSON fixture handoff
3. external adapter writes RustSubset JSON and invokes a stable app boundary
```

Stop lines:

```text
do not re-enable VM as the product app route
do not replace json_native with a JSON DLL in this row
do not claim full Rust transpiler semantics
do not treat schema-key interning as final JSON library semantics
```

## Contract

```text
output_contract=rust-subset-to-hako-embedded-aot-parity-v0

python_reference_selftest=ok
json_native_probe_exe=ok
hako_converter_mir_json_emit=ok
hako_converter_exe=ok
hako_converter_parity=ok
file_input_enabled=0
schema_key_interning_bridge=temporary
next_task=RUST-SUBSET-TO-HAKO-INPUT-ROUTE-SELECTION-001

summary=ok
```
