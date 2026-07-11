# 296x-1328 CREAT-SUBSET-PILOT-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the first real crate-style RustSubset pilot after crate handoff,
constructor lifecycle, imported field-initializer routing, and unknown global
callee diagnostics were closed.

This row is selection/tooling only. It does not add `.hako` syntax, Rust
semantics, converter-core ownership, or generated-program execution claims.

## Tooling

Add a focused inventory tool:

```text
apps/rust-subset-to-hako/tools/crate_inventory.py
```

Input:

```text
RustSubsetCrateManifest-v0 bundle
```

Output:

```text
output_contract=rust-subset-crate-inventory-v0
module_count=<n>
module_i_item_total=<n>
module_i_unsupported_total=<n>
total_kind.<kind>=<n>
total_unsupported_rust_kind.<rust_kind>=<n>
unsupported_total=<n>
selection_ready=<0|1>
summary=ok
```

The tool is deliberately outside converter core ownership:

```text
rust_parser_invoked=0
adapter_invoked=0
name_resolution=0
converter_core_changed=0
```

It reads only an already-produced manifest bundle and fails fast for unsafe
artifact paths, duplicate module ids, duplicate artifact paths, missing
artifacts, invalid module roots, and manifest/module id mismatch.

## Candidate Inventory

Focused commands:

```bash
tmp=$(mktemp -d /tmp/hakorune_rs_inventory_boxcore.XXXXXX)
cargo run -q --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  --crate-root crates/hakorune_box_core --out-dir "$tmp" \
  --crate-name hakorune_box_core --target-kind lib --target-name hakorune_box_core
python3 apps/rust-subset-to-hako/tools/crate_inventory.py \
  --manifest "$tmp/crate-manifest.json"
```

Result:

```text
crate=hakorune_box_core
module_count=3
total_item_total=7
unsupported_total=2
total_unsupported_rust_kind.Use=2

module=crate
item_total=4
unsupported_total=2

module=crate::plugin
item_total=1
unsupported_total=0

module=crate::policy
item_total=2
unsupported_total=0
```

Comparison sweep:

```text
hakorune_box_core      module_count=3   item_total=7   unsupported_total=2
hakorune_mir_core      module_count=8   item_total=70  unsupported_total=54
hakorune_mir_defs      module_count=2   item_total=11  unsupported_total=12
hakorune_frontend_ast  module_count=12  item_total=82  unsupported_total=69
hakorune_mir_builder   module_count=7   item_total=41  unsupported_total=33
```

## Decision

Select:

```text
selected_pilot_crate=hakorune_box_core
selected_module_count=3
selected_modules=crate,crate::plugin,crate::policy
known_unsupported_family=Use
known_unsupported_count=2
```

Rationale:

```text
smallest_real_crate_candidate=1
two_clean_leaf_modules=1
root_module_has_known_unsupported_use_only=1
new_hako_syntax_required=0
rust_name_resolution_required=0
trait_generic_macro_semantics_required=0
```

The `Use` items remain explicit Unsupported handoff nodes. They are not a
request to implement Rust name resolution in the converter.

## Acceptance

```bash
python3 apps/rust-subset-to-hako/tools/crate_inventory.py \
  --manifest apps/rust-subset-to-hako/examples/mini_crate_expected/crate-manifest.json

tmp=$(mktemp -d /tmp/hakorune_rs_inventory_boxcore.XXXXXX)
cargo run -q --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  --crate-root crates/hakorune_box_core --out-dir "$tmp" \
  --crate-name hakorune_box_core --target-kind lib --target-name hakorune_box_core
python3 apps/rust-subset-to-hako/tools/crate_inventory.py \
  --manifest "$tmp/crate-manifest.json"

cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
new_hako_syntax_added=0
converter_core_changed=0
rust_parser_owned_by_hako=0
crate_graph_discovery_owned_by_hako=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
generated_program_exe_aot_claim=0
```

## Next

Continue:

```text
HAKORUNE-BOX-CORE-RUSTSUBSET-PILOT-001
```

Run the selected `hakorune_box_core` crate bundle through the existing crate
handoff path, keep Unsupported `Use` explicit, and accept generated skeletons
only through parse / MIR emit.
