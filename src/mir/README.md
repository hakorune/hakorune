# MIR (`src/mir/`)

This directory is the Rust-side MIR workspace. It is intentionally broad, but the
navigation order must stay narrow and explicit.

## Read First

1. [`builder/README.md`](./builder/README.md)
2. [`contracts/README.md`](./contracts/README.md)
3. [`policies/README.md`](./policies/README.md)
4. [`join_ir/README.md`](./join_ir/README.md)
5. [`join_ir_vm_bridge/README.md`](./join_ir_vm_bridge/README.md)
6. [`join_ir_vm_bridge_dispatch/README.md`](./join_ir_vm_bridge_dispatch/README.md)

## Top-Level Map

- `analysis/`: analysis helpers and shared inspection utilities.
- `builder/`: AST -> MIR construction. FlowPlanner / JoinIR glue are
  physically under builder today but conceptually separate from builder core.
- `contracts/`: backend acceptance allowlists and fail-fast instruction tags.
- `control_tree/`: structure-only control-flow SSOT and normalized shadow path.
- `definitions/`: MIR definition data and shared type/shape declarations.
- `instruction/`, `instruction_kinds/`: instruction model and kind definitions.
- `join_ir/`: normalized JoinIR lowering and ownership helpers. Docs-first only for now.
- `join_ir_runner/`: JoinIR execution entry helpers.
- `join_ir_vm_bridge/`: JoinIR -> VM bridge implementation.
- `join_ir_vm_bridge_dispatch/`: bridge routing policy and dispatch tables.
- `loop_canonicalizer/`: loop normalization and route detection.
- `lowerers/`: lowering helpers that are not part of the builder core.
- `numeric_substrate.rs`: fixed-width and pointer-sized numeric type-name
  vocabulary, target pointer-width resolution, and target-resolved numeric kind
  metadata. It does not add exact-width runtime semantics by itself.
- `raw_layout.rs`: fixed raw-layout vocabulary for substrate metadata. It does
  not add `.hako struct` syntax or backend-active native layout by itself.
- `optimizer_passes/`, `passes/`: MIR pass implementations. Docs-first only for now.
- `phi_core/`: PHI / loopform helpers and supporting state.
- `policies/`: shared policy SSOT used by builder/canonicalizer/router.
- `region/`, `ssot/`, `type_propagation/`, `utils/`, `verification/`: supporting helpers.

## Generic Method Route Metadata

- `generic_method_route_plan.rs` owns MIR-derived route carriers only. It may
  attach `CoreMethodOp` metadata after MIR facts prove the surface, but it must
  not choose new semantic behavior for runtime methods.
- `generic_method_route_facts.rs` owns reusable facts such as receiver origin,
  key route, return shape, value demand, and publication policy. Keep backend
  helper names out of this file.
- `core_method_op.rs` owns the MIR carrier vocabulary that mirrors the
  generated `CoreMethodContract` manifest.
- Mutating carriers such as `ArrayPush` / `ArraySet` must not reuse key metadata
  for value operands. Add an explicit operand/value field if a future lowering
  needs one.
- Legacy `.inc` mirror rows may only be pruned after a metadata-absent boundary
  contract exists for the same method family.

## MIR Semantic Plans / Routes Map

This map keeps top-level MIR plan/route/seed surfaces readable without moving
files. Keep the terms distinct:

- `Plan`: compiler-owned candidate, selection, layout, or placement data.
- `Route`: backend/VM-consumable lowering contract.
- `SeedRoute`: temporary exact-shape proof bridge with a retire path.
- `Fact` / `Contract`: verifier, optimizer, or diagnostics input; not a
  backend helper selector by itself.

| Class | Top-level surfaces | Current state |
| --- | --- | --- |
| `LayoutPlans` | `typed_object_plan.rs` + `typed_object_plan/`, `record_layout_plan.rs`, `static_data_plan.rs`, `array_record_storage_plan.rs` | `typed_object_plans` and `static_data_plans` are backend-active. `record_layout_plans` are semantic layout truth. `array_record_storage_plans` remain metadata-only storage candidates. |
| `PlacementPlans` | `array_text_residence_session_plan.rs`, `array_text_state_residence_plan.rs`, `map_lookup_fusion_plan.rs` | residence/session plans are metadata-only unless a route row explicitly promotes them. `array_text_state_residence_route` is the active residence contract. |
| `LoweringRoutes` | `generic_method_route_plan.rs` + dir, `extern_call_route_plan.rs` + dir, `global_call_route_plan.rs` + dir, `user_box_method_route_plan.rs` + dir, `array_rmw_window_plan.rs`, `array_string_len_window_plan.rs`, `array_text_*_plan.rs`, `string_direct_set_window_plan.rs`, `string_kernel_plan.rs`, `exact_seed_backend_route.rs` | backend/VM-facing route carriers. Route rows may be fail-fast or unsupported-route metadata; they do not automatically make an operation lowerable. |
| `ExperimentalSeedRoutes` | `array_getset_micro_seed_plan.rs`, `array_rmw_add1_leaf_seed_plan.rs`, `array_string_store_micro_seed_plan.rs`, `concat_const_suffix_micro_seed_plan.rs`, `substring_views_micro_seed_plan.rs`, `sum_variant_*_seed_plan.rs`, `userbox_*_seed_plan.rs`, `indexof_search_micro_seed_plan.rs` | temporary exact-shape bridges. Keep retire conditions in metadata/seed ledgers before making them long-term route vocabulary. |
| `SemanticFacts/Contracts` | `generic_method_route_facts.rs`, `effect_capability_plan.rs`, `inline_plan.rs`, `rune_plan_refresh.rs`, `aot_plan_import.rs` | facts/contracts/support surfaces. `effect_capability_plan` is verifier-active contract material; `inline_plan` is optimizer-active only for verified rows. |

Do not collapse these categories into one `plans/` bucket without preserving the
plan vs route vs seed distinction. The reference catalog owner remains
`docs/reference/mir/metadata-facts-ssot.md`.

## Boundary Rules

- Add shared policy once under `policies/` and reuse it from the other subtrees.
- Do not hide new acceptance rules inside local helpers when `contracts/` already owns the tag.
- When a subtree grows a new reading order, update this file and the subtree README together.

## P5 Crate Split Prep

This repo is not splitting `src/mir` yet. The prep step is to keep the public
navigation narrow and make the future crate seams explicit first.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Candidate future crates:

- `hakorune-mir-core`: definitions, instruction kinds, shared shape data, value kind substrate
- `hakorune-mir-builder`: AST -> MIR construction; FlowPlanner boundary stays
  documented before any future packaging decision
- `hakorune-mir-joinir`: JoinIR lowering and ownership helpers
- `hakorune-mir-passes`: passes, normalization, and verification helpers

Prep rule:

- do not split until each subtree README names its public surface and rejected boundaries
