# Hako Compiler Libraries

This directory is the ordinary `.hako` library home for compiler-facing helper
modules.

Placement SSOT:
- `docs/development/current/main/phases/phase-296x/296x-1740-HAKO-COMPILER-TEXT-BUILDER-V0-001.md`
- `docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md`
- `docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md`

Scope:
- ordinary Hako library code for compiler meaning, text formatting, JSON
  serialization, and projection value helpers
- initial files belong here:
  - `text_builder.hako`
  - `canonical_json.hako`
  - `projection_value.hako`
- first shadow-projector support library:
  - `return_emission_projector.hako`
- next shadow-projector support library:
  - `function_region_stack_pop_projector.hako`

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
