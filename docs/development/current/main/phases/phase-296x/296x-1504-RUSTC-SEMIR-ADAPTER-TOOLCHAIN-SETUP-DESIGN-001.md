# 296x-1504 RUSTC-SEMIR-ADAPTER-TOOLCHAIN-SETUP-DESIGN-001

Status: closed
Date: 2026-06-20

## Purpose

Define the toolchain setup route for the standalone rustc semantic adapter
before any rustc-internal HIR / THIR / MIR extraction row starts.

The previous diagnostic row reports:

```text
rustc_channel=stable_or_release
rustc_private_readiness=requires_nightly_or_bootstrap
```

This row is design-only. It chooses how a developer or CI environment should
make rustc internals available to the standalone adapter tool without adding
`rustc_private` dependencies to product crates.

## Selected By

```text
296x-1503-POST-RUSTC-SEMIR-ADAPTER-TOOLCHAIN-COMPAT-PREFLIGHT-OWNER-SELECTION-001
```

## Scope

Decide:

```text
supported_toolchain_route
developer_setup_command
CI_setup_policy
fail_fast_message_for_missing_rustc_private
standalone_tool_boundary
Rust bootstrap preservation boundary
```

Candidates:

```text
A. pinned nightly toolchain
   value: clean rustc_private route with explicit toolchain version
   risk: requires rustup toolchain availability

B. RUSTC_BOOTSTRAP diagnostic/developer override
   value: low setup friction for local experiments
   risk: must not become product default or CI silent dependency

C. keep adapter source-shape only until rustc toolchain is available
   value: no toolchain setup work now
   risk: delays rustc semantic replacement and preserves old probe debt
```

## Acceptance

```text
toolchain_route_selected=1
developer_setup_documented=1
ci_policy_documented=1
fail_fast_contract_documented=1
product_crates_rustc_private_dependency=0
rust_bootstrap_preserved=1
rust_compat_route_preserved=1
rust_oracle_vector_preserved=1
implementation_started=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

## Bootstrap Preservation Boundary

This toolchain design must not be read as removing Rust boot.

```text
Hako authority promotion:
  allowed family by family after verified parity

Rust owner demotion:
  allowed only as semantic authority demotion for that family

Rust bootstrap removal:
  forbidden
```

The Rust path remains required for:

```text
bootstrap
oracle comparison
compatibility route
iOS / Windows / new-host bring-up
emergency recovery
```

Future task names should prefer:

```text
MIRBUILDER-<FAMILY>-HAKO-AUTHORITY-PROMOTION-001
MIRBUILDER-<FAMILY>-RUST-OWNER-DEMOTION-001
```

and avoid names that imply deleting Rust boot.

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Decision

```text
supported_toolchain_route=pinned_date_nightly
required_component=rustc-dev
toolchain_file_owner=adapter_subworkspace
formal_readiness=compile_link_run_probe

bootstrap_override=diagnostic_untrusted_only
bootstrap_form=crate_scoped_preferred
bootstrap_CI_allowed=0
bootstrap_facts_accepted=0

product_toolchain=stable
adapter_CI_policy=optional_after_local_preflight
```

The machine-readable pin belongs to:

```text
tools/rust_lifecycle/rustc_semir_adapter/rust-toolchain.toml
```

The canonical invocation must run from the adapter directory so rustup sees
the nested toolchain file:

```text
cd tools/rust_lifecycle/rustc_semir_adapter
cargo run --quiet -- --toolchain-preflight
```

`RUSTC_BOOTSTRAP` is not a formal readiness route. If it is exposed later, it
must report:

```text
bootstrap_override=1
trust_class=diagnostic_untrusted
accepted_facts_generated=0
```

## Closeout

```text
toolchain_route_selected=1
developer_setup_documented=1
ci_policy_documented=1
fail_fast_contract_documented=1
product_crates_rustc_private_dependency=0
rust_bootstrap_preserved=1
rust_compat_route_preserved=1
rust_oracle_vector_preserved=1
implementation_started=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Next:

```text
RUSTC-SEMIR-ADAPTER-PINNED-NIGHTLY-PREFLIGHT-001
```

## Stop Line

```text
do_not_extract_HIR_THIR_MIR_in_design=1
do_not_add_rustc_private_dependency_in_product_crates=1
do_not_generate_lifecycle_facts_in_design=1
do_not_emit_HakoLifecyclePlan_in_design=1
do_not_change_backend=1
do_not_remove_Rust_bootstrap=1
do_not_pin_moving_nightly_alias=1
do_not_treat_version_string_as_rustc_private_readiness=1
do_not_close_without_compile_link_run_probe=1
do_not_auto_enable_RUSTC_BOOTSTRAP_after_nightly_failure=1
```
