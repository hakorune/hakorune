# 296x-1505 RUSTC-SEMIR-ADAPTER-PINNED-NIGHTLY-PREFLIGHT-001

Status: open
Date: 2026-06-20

## Purpose

Implement the pinned-date-nightly readiness preflight for the standalone rustc
semantic adapter tool.

This row proves the adapter toolchain route without extracting HIR / THIR /
MIR and without generating lifecycle facts.

## Selected By

```text
296x-1504-RUSTC-SEMIR-ADAPTER-TOOLCHAIN-SETUP-DESIGN-001
```

## Scope

Allowed:

```text
adapter-local rust-toolchain.toml
canonical adapter-directory launcher or guard invocation
rustc -Vv fingerprint reporting
rustc-dev component / rustc_private compile-link-run probe
toolchain route guard
```

Forbidden:

```text
HIR / THIR / MIR extraction
RustLifecycleAdapterFacts generation
HakoLifecyclePlan-v0 output
.hako source output
backend behavior change
root/product toolchain change
CI workflow change
```

## Acceptance

```text
pinned_nightly_route_documented=1
adapter_local_toolchain_file=1
moving_nightly_alias_used=0

pinned_toolchain_active=1
rustc_release_reported=1
rustc_commit_hash_reported=1
rustc_sysroot_reported=1

rustc_dev_component_installed=1
rustc_private_probe_compiled=1
rustc_private_probe_linked=1
rustc_private_probe_executed=1
rustc_private_readiness=verified

canonical_bootstrap_override=0
bootstrap_diagnostic_route_reported_if_used=1
bootstrap_facts_accepted=0

product_crates_rustc_private_dependency=0
root_product_toolchain_changed=0

facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
summary=ok
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_pinned_nightly_preflight_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_extract_HIR_THIR_MIR_in_this_row=1
do_not_generate_lifecycle_facts_in_this_row=1
do_not_emit_HakoLifecyclePlan_in_this_row=1
do_not_emit_Hako_source_in_this_row=1
do_not_add_rustc_private_dependency_in_product_crates=1
do_not_change_root_product_toolchain=1
do_not_add_CI_workflow_in_this_row=1
do_not_remove_Rust_bootstrap=1
```
