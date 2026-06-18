# rust-subset-to-hako Status

Status: v0 embedded-fixture `.hako` converter passes EXE/AOT parity.

## Current State

The reference converter is complete enough for v0 fixtures:

```bash
python3 apps/rust-subset-to-hako/selftest.py
```

The native `.hako` converter exists:

```text
apps/rust-subset-to-hako/convert.hako
```

It uses the `.hako` JSON library:

```text
apps/lib/json_native/parser/parser.hako
```

## Verified Working

```text
python_reference_selftest=ok
json_native_type_reserved_word_blocker=fixed
hako_json_probe_mir_json_emit=ok
hako_converter_mir_json_emit=ok
json_native_probe_exe=ok
hako_converter_exe=ok
hako_converter_parity=ok
```

## Current Scope Boundary

The current accepted slice embeds `examples/simple_subset.json` inside
`convert.hako` and verifies the generated output against
`examples/simple_expected.hako`.

```text
route=EXE/AOT
vm_product_route=retired
file_input_enabled=0
stdin_input_enabled=0
external_rust_parser_adapter_enabled=0
```

File/stdin input is intentionally outside this row. The current acceptance
proves the app-front core path:

```text
embedded RustSubset JSON -> json_native -> JsonNode traversal -> .hako skeleton
```

`json_native` currently uses temporary RustSubset schema-key interning in the
tokenizer. This is not the final JSON library semantics; remove it after
scanner-derived dynamic strings can be used as stable `MapBox` keys on EXE/AOT.

## Next Task

```text
RUST-SUBSET-TO-HAKO-INPUT-ROUTE-SELECTION-001
```

Goal:

```text
Choose the next input route for the converter without reopening VM product
execution:

1. EXE/AOT FileBox acceptance for local files
2. host-provided embedded JSON fixture generation
3. external adapter writes RustSubset JSON and invokes converter with a stable
   app boundary
```

Do not replace `json_native` with a host JSON DLL in this row.
Do not re-enable VM as the product app route.
Do not special-case RustSubset converter names.
```

## Reproduction

```bash
bash apps/rust-subset-to-hako/smoke.sh
```
