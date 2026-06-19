# 296x-1331 RUST-SUBSET-NEXT-CRATE-PILOT-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next RustSubset crate/module pilot after `hakorune_box_core` root
module MIR acceptance.

This is inventory/selection only. It does not add `.hako` syntax, Rust name
resolution, `use` resolution, crate graph semantics, or converter-core
ownership.

## Candidate Sweep

Command shape:

```bash
cargo run -q --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  --crate-root <crate> --out-dir "$tmp" \
  --crate-name <name> --target-kind lib --target-name <name>
python3 apps/rust-subset-to-hako/tools/crate_inventory.py \
  --manifest "$tmp/crate-manifest.json"
```

Summary:

```text
hakorune_backend_aot     modules=4   items=21  unsupported=42
hakorune_frontend_ast    modules=12  items=82  unsupported=69
hakorune_frontend_parser modules=8   items=40  unsupported=34
hakorune_mir_builder     modules=7   items=41  unsupported=33
hakorune_mir_core        modules=8   items=70  unsupported=54
hakorune_mir_defs        modules=2   items=11  unsupported=12
hakorune_mir_joinir      modules=2   items=10  unsupported=10
nyash_c_core             modules=1   items=8   unsupported=8
nyash-next               modules=1   items=1   unsupported=0
nyash_kernel_min_c       modules=1   items=0   unsupported=0
```

Graph blockers:

```text
hakorune_mir_json_emit  inline module tests out of crate manifest v0 scope
hakorune_mir_plans      lib.rs references missing src/map_repr_plan.rs
nyash-llvm-compiler     no src/lib.rs for crate-mode adapter
nyash_kernel            lib.rs references missing src/exports.rs
nyash_tlv               inline module tests out of crate manifest v0 scope
```

## Selected Slice

Select a focused two-module slice from `hakorune_mir_core`:

```text
selected_crate=hakorune_mir_core
selected_modules=crate::control_ids,crate::types
selected_module_count=2
```

Evidence:

```text
crate::control_ids:
  items=3
  unsupported=0
  content=tuple newtype structs LoopId/ExitEdgeId/ContinueEdgeId

crate::types:
  items=10
  unsupported=2
  unsupported_rust_kind.Use=1
  unsupported_rust_kind.<missing>=1
  content=core enum/type surface plus Display impl with unsupported match expr
```

Rationale:

```text
adds_real_compiler_surface=1
keeps_slice_small=1
module_count_between_2_and_3=1
one_clean_module=1
next_blocker_is_precise=unsupported_expression_in_generated_function_body
```

The single-module zero-unsupported candidates (`nyash-next`,
`nyash_kernel_min_c`) are intentionally not selected because they are too small
to advance the crate/module pilot lane.

## Acceptance

```bash
tmp=$(mktemp -d /tmp/hakorune_mir_core_inventory.XXXXXX)
cargo run -q --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  --crate-root crates/hakorune_mir_core --out-dir "$tmp" \
  --crate-name hakorune_mir_core --target-kind lib --target-name hakorune_mir_core
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
rust_name_resolution_enabled=0
use_resolution_enabled=0
partial_crate_adapter_implemented=0
generated_program_exe_aot_claim=0
```

## Next

Continue:

```text
HAKORUNE-MIR-CORE-RUSTSUBSET-PILOT-001
```

Materialize the selected `hakorune_mir_core` module slice as fixtures and run
it through the existing RustSubset skeleton pipeline. The expected next
technical blocker is unsupported expression transport inside generated function
bodies, not crate graph discovery.
