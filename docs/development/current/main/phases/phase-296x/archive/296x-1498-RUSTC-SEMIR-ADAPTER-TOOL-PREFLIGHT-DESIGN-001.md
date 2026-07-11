# 296x-1498 RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-DESIGN-001

Status: closed
Date: 2026-06-20

## Purpose

Design the rustc semantic adapter tool boundary and preflight contract before
any rustc-internal adapter implementation.

This row is docs/design only.

## Selected By

```text
296x-1497-POST-RUSTC-SEMIR-INTERNAL-ADAPTER-BOUNDARY-DESIGN-OWNER-SELECTION-001
```

## Scope

```text
tool_location=undecided
rustc_private_owner=adapter tool only
stable_output=repo-owned RustLifecycleAdapterFacts JSON
first_subject=BindingContext identity/provenance or equivalent narrow probe
```

Questions to decide:

```text
where the adapter crate/tool lives
how rustc_private is isolated from product crates
how toolchain version is detected and reported
which command is the first preflight guard
which output files are generated vs checked in
```

Forbidden:

```text
rustc_internal_adapter_implementation
new RustLifecycleAdapterFacts generation
product compiler rustc_private dependency
backend behavior change
```

## Acceptance

```text
adapter_tool_boundary_documented=1
rustc_private_isolated_from_product=1
first_preflight_contract_documented=1
implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Design

```text
design_ssot=docs/development/current/main/design/rustc-semir-adapter-tool-preflight-contract.md
```

Decision:

```text
adapter_tool_location=tools/rust_lifecycle/rustc_semir_adapter/
adapter_tool_workspace=standalone
root_Cargo_rustc_private_dependency=forbidden
stable_surface=JSON only
first_preflight=diagnostic_only_no_extraction
```

The first preflight command shape is:

```bash
cargo run --manifest-path tools/rust_lifecycle/rustc_semir_adapter/Cargo.toml -- --preflight
```

## Closeout

```text
adapter_tool_boundary_documented=1
rustc_private_isolated_from_product=1
first_preflight_contract_documented=1
implementation_started=0
backend_behavior_changed=0
```

Next:

```text
POST-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-DESIGN-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_add_rustc_private_dependency_in_design=1
do_not_generate_facts_in_design=1
do_not_change_product_compiler=1
do_not_change_backend=1
```
