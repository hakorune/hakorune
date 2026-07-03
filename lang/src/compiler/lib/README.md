# Hako Compiler Libraries

This directory is the ordinary `.hako` library home for compiler-facing helper
modules.

Placement SSOT:
- `docs/development/current/main/phases/phase-296x/296x-1740-HAKO-COMPILER-TEXT-BUILDER-V0-001.md`
- `docs/development/current/main/phases/phase-296x/296x-1765-HAKO-COMPILER-CANONICAL-JSON-VALUE-WRITER-001.md`
- `docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md`
- `docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md`

Scope:
- ordinary Hako library code for compiler meaning, text formatting, JSON
  serialization, and projection value helpers
- initial files belong here:
  - `text_builder.hako`
  - `canonical_json.hako`
  - `projection_value.hako`
  - `projector_support.hako`
- first Rust-oracle parity pilot library:
  - `storage_class_classifier.hako`
- second Rust-oracle parity pilot library:
  - `placement_effect_tag_formatter.hako`
- third Rust-oracle parity pilot library:
  - `static_scalar_fact_classifier.hako`
- first shadow-projector support library:
  - `return_emission_projector.hako`
- first native source owner candidate:
  - `next_value_id_prepared_state_kernel.hako`
- first native source seed:
  - `return_emission_native_seed.hako`
- second native source seed:
  - `function_region_stack_pop_native_seed.hako`
- third native source seed:
  - `slot_registry_release_native_seed.hako`
- fourth native source seed:
  - `carrier_merge_assignment_native_seed.hako`
- fifth native source seed:
  - `box_field_initialization_native_seed.hako`
- second shadow-projector support library:
  - `function_region_stack_pop_projector.hako`
- third shadow-projector support library:
  - `slot_registry_release_projector.hako`
- fourth shadow-projector support library:
  - `module_metadata_publication_projector.hako`
- fifth shadow-projector support library:
  - `record_packed_layout_refresh_projector.hako`
- sixth shadow-projector support library:
  - `typed_object_plan_refresh_projector.hako`
- seventh shadow-projector support library:
  - `direct_state_plan_refresh_projector.hako`
- eighth shadow-projector support library:
  - `all_functions_phi_materialization_projector.hako`
- ninth shadow-projector support library:
  - `carrier_merge_assignment_projector.hako`
- tenth shadow-projector support library:
  - `box_field_initialization_projector.hako`
- executable placement guard:
  - `tools/checks/rust_lifecycle_hako_compiler_canonical_json_value_writer_guard.sh`
- executable inventory guard:
  - `tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh`
- first shadow parity guard:
  - `tools/checks/rust_lifecycle_mirbuilder_return_emission_hako_shadow_parity_guard.sh`
- first Rust-oracle parity pilot gate:
  - `tools/checks/rust_lifecycle_mirbuilder_storage_class_classifier_parity_gate.sh`

Allowed:
- reuse of existing `StringBox`, `ArrayBox`, and `OrderedMapBox`
- library-level encapsulation and deterministic formatting

Forbidden:
- TypeBox ABI exposure for compiler-library semantics
- host ABI facades for JSON/Text/projector behavior
- distribution/package ABI for the library surface
- new language syntax or spec promotion
- `hako.buf` backing in v0

Non-claims:
- no ABI surface is added here
- no source selfhost claim is made here
- no HakoAdopted decision is made here

The directory exists to keep compiler-facing Hako code close to the compiler
ownership map without re-opening the ABI discussion for the first placement.
