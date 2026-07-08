# 3345 - MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001

## Token

```text
MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001
```

## Purpose

Install a light active guard resolver for the rust_lifecycle ecosystem.

The resolver reads `CURRENT_STATE.toml`, resolves only the latest card and
current blocker guards, and refuses to run the historical rust_lifecycle guard
set by default.

## Entry

```text
tools/checks/current_active_rust_lifecycle_guard_resolver.sh
```

## Result

```text
current_active_guard_resolver = 1
latest_guard_resolved = 1
current_blocker_guard_resolved = 0
current_blocker_guard_pending = 1
runnable_guard_count = 1
max_default_guard_count = 3
run_all_rust_lifecycle_guards_by_default = 0
source_selfhost_claim = 0
```

The current blocker is a design-stop card that has not been materialized yet, so
the resolver records it as pending instead of inventing a guard.

## Decision

```text
decision:
  SelectCompareProofBridgeParkOrConnectDesignStop

reason_token:
  ActiveGuardResolverInstalledBeforeShadowConsume

selected_next_card:
  MIRBUILDER-COMPARE-PROOF-BRIDGE-PARK-OR-CONNECT-DESIGN-STOP-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_current_active_rust_lifecycle_guard_resolver_guard.sh
```

## Non-Claims

```text
all_rust_lifecycle_guards_in_ci = 0
all_rust_lifecycle_guards_in_dev_gate = 0
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
source_selfhost_claim = 0
```
