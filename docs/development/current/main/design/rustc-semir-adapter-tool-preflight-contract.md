# Rustc SemIR Adapter Tool Preflight Contract

Status: Design
Scope: tool boundary before implementing rustc-internal lifecycle facts.

## Purpose

Define where the rustc semantic adapter lives and how its first preflight is
validated without adding rustc internals to product crates.

## Tool Location

The adapter should live outside the root product crate:

```text
tools/rust_lifecycle/rustc_semir_adapter/
  Cargo.toml
  src/main.rs
```

It should be a standalone tool workspace, following the existing
`apps/rust-subset-to-hako/tools/syn_adapter` pattern.

The root `Cargo.toml` must not gain `rustc_private` or rustc-internal
dependencies.

## Dependency Boundary

Allowed only inside the adapter tool:

```text
rustc_driver / rustc_interface
rustc_hir
rustc_middle
rustc_mir_dataflow
rustc_span
```

Forbidden in product crates:

```text
nyash-rust
hakorune_mir_*
hakorune_frontend_*
runtime / backend crates
```

The stable communication surface is JSON only.

## First Preflight

The first preflight guard should check:

```text
adapter tool manifest exists
tool reports rustc version / channel diagnostics
tool can run in no-extraction mode
tool emits no RustLifecycleAdapterFacts
tool emits no HakoLifecyclePlan
tool emits no .hako
product Cargo.toml has no rustc_private dependency
```

The first command shape should be diagnostic-only:

```bash
cargo run --manifest-path tools/rust_lifecycle/rustc_semir_adapter/Cargo.toml -- --preflight
```

If the local toolchain cannot support rustc_private, the preflight must fail
with a clear diagnostic. It must not silently fall back to source-shape probes.

## Generated vs Checked In

Checked in:

```text
design docs
adapter tool source
preflight guard
expected output contract tokens
```

Generated at runtime:

```text
toolchain diagnostics
future RustLifecycleAdapterFacts JSON
```

Do not check in rustc raw dumps.

## Future Implementation Ladder

```text
1. preflight only
2. HIR item/provenance inventory for BindingContext
3. THIR typed method body inventory
4. MIR copy/move/borrow/drop inventory
5. generated RustLifecycleAdapterFacts parity with existing fixture
6. retire source-shape extractor as authority
```

Each step needs its own guard.

## Stop Lines

```text
do_not_add_rustc_private_to_root_Cargo=1
do_not_depend_on_rustc_private_from_product_crates=1
do_not_use_raw_rustc_dump_as_stable_schema=1
do_not_fallback_to_source_shape_on_preflight_failure=1
do_not_emit_facts_before_preflight_is_green=1
```
