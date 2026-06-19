# 296x-1317 RUST-SUBSET-PATH-NAME-NORMALIZATION-001

Status: closed
Date: 2026-06-20

## Purpose

Close the P0 RustSubset path/name normalization gap before crate-wide
manifest handoff or any `creat`-style pilot.

The row does not add `.hako` syntax. It fixes the transport contract so the
external Rust adapter owns deterministic emitted names and the converter prints
those names without resolving Rust paths.

## Decision

```text
adapter owns:
  structured Rust path provenance
  source_name / emitted_name metadata
  Hako-reserved identifier escaping
  tuple field _0 / _1 normalization
  duplicate emitted-name fail-fast
  simple-pattern-only handoff

converter owns:
  emit name/emitted_name text
  never resolve Rust paths
  never invoke the adapter
  never own crate graph or FileBox routing
```

Rust raw identifiers are normalized to logical names before Hako escaping:

```text
r#type -> source_name=type -> emitted_name=rust_type
r#match -> source_name=match -> emitted_name=rust_match
```

Multi-segment paths remain provenance only:

```text
crate::model::Config -> emitted_name=crate_model_Config
```

## Implementation

```text
apps/rust-subset-to-hako/tools/syn_adapter/src/names.rs
  emitted_ident()
  emitted_path()
  source_path provenance
  duplicate emitted-name fail-fast

apps/rust-subset-to-hako/tools/syn_adapter/src/types.rs
  type/path normalization
  tuple field _0 / _1 normalization
  simple pattern metadata helper

apps/rust-subset-to-hako/tools/syn_adapter/src/items.rs
apps/rust-subset-to-hako/tools/syn_adapter/src/functions.rs
apps/rust-subset-to-hako/tools/syn_adapter/src/stmts.rs
apps/rust-subset-to-hako/tools/syn_adapter/src/exprs.rs
  source_name / emitted_name propagation
  no unsupported_param / unsupported_pattern fake names

apps/rust-subset-to-hako/convert.py
  emitted_name-preferred reference converter

apps/rust-subset-to-hako/converter_core.hako
  emitted_name-preferred converter core
  literal JSON key lookup only; no dynamic schema-key lookup

apps/rust-subset-to-hako/examples/path_name_*
apps/rust-subset-to-hako/convert_path_name_fixture.hako
  adapter / Python / EXE-AOT fixture parity
```

## Stop Line

```text
new_hako_syntax_enabled=0
rust_name_resolution_in_converter=0
crate_manifest_schema_accepted=0
converter_core_filebox_ownership=0
adapter_invocation_in_converter_core=0
unsupported_pattern_fake_binding=0
unsupported_param_fake_binding=0
duplicate_emitted_name_silent_accept=0
```

## Evidence

```bash
python3 apps/rust-subset-to-hako/selftest.py
cargo check --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml
bash apps/rust-subset-to-hako/smoke.sh
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

Final repository-wide gate:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
RUST-SUBSET-CRATE-MANIFEST-V0-001
```

Define the crate handoff manifest contract before implementing multi-module
adapter output:

```text
manifest is transport index, not semantic AST
unit is one selected Cargo target
body_embedding=0
per_module_artifact=RustSubsetModule-v0
absolute paths forbidden
dependency crates excluded
```
