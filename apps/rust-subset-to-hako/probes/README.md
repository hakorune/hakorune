# Rust Subset Converter VM Probes

Status: investigation fixtures.

These small `.hako` programs preserve the current blocker state for the
RustSubset JSON -> `.hako` converter.

## What They Prove

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
