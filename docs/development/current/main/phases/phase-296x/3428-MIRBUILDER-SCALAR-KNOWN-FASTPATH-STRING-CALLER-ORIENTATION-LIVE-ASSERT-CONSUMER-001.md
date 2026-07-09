# 3428 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```

## Purpose

Extend the compiler-side assertion-only caller consumer to the exact three
String read policy rows. The existing String route authority and Rust oracle
remain unchanged.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Required Delta

1. Add a String row-ID assertion over the generated three-row contract.
2. Call it after each existing String policy lookup.
3. Keep the assertion input as `policy_row_id: &str` and return `()`.
4. Add exact three-row, unknown-ID, metadata-drift, and live-call guards.

## Non-Claims

```text
caller_orientation_runtime_path = 0
caller_runtime_dispatch_authority = 0
route_selection_authority_switch = 0
hako_runtime_route_authority = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_scalar_known_fastpath_string_caller_orientation_live_assert_consumer_guard.sh
```

## Result

```text
status = landed
string_caller_orientation_live_assert_consumer = 1
string_three_row_exact = 1
assertion_only = 1
tests = 6 passed
caller_orientation_runtime_path = 0
route_selection_authority_switch = 0
source_selfhost_claim = 0
selected_next_card =
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```
