# 296x-1504 RUSTC-SEMIR-ADAPTER-TOOLCHAIN-SETUP-DESIGN-001

Status: open
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
implementation_started=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_extract_HIR_THIR_MIR_in_design=1
do_not_add_rustc_private_dependency_in_product_crates=1
do_not_generate_lifecycle_facts_in_design=1
do_not_emit_HakoLifecyclePlan_in_design=1
do_not_change_backend=1
```
