# Rust Subset Converter Probes

Status: EXE/AOT app-front probes and historical investigations.

These small `.hako` programs preserve the current blocker state for the
RustSubset JSON -> `.hako` converter.

## Historical VM Findings

- `tokenizer_probe.hako`: JSON tokenization works for a small object.
- `json_probe.hako`: full JSON parsing currently returns `null`.
- `array_node_probe.hako` / `map_node_probe.hako`: `ArrayBox` and `MapBox`
  do not currently round-trip user `JsonNodeInstance` values through the VM
  route used by this app.
- `node_probe.hako`: `JsonNode` factory methods work, but object storage fails
  once user nodes are stored in a collection.
- `map_probe.hako`: primitive MapBox values still round-trip, so the issue is
  not generic MapBox key lookup.

## Design Consequence

The converter can compile and reach runtime, but product-level JSON tree
execution on the Rust VM is not a good next investment. The active design row
should move app/selfhost validation toward EXE/AOT and freeze VM work to a
small semantic-reference subset.

## Current EXE/AOT Finding

VM product-route app validation is retired. The current route is EXE/AOT.

The current green probes are:

```text
json_probe.hako:
  parses {"kind":"Program","items":[]}
  verifies object field lookup and array length

convert.hako:
  embeds simple_subset.json
  emits simple_expected.hako through EXE/AOT
```

Use `apps/rust-subset-to-hako/smoke.sh` to reproduce the accepted state.

Many other files in this directory are diagnostic probes kept to preserve the
JSON bring-up trail. They are not all part of the stable acceptance gate.
