---
Status: Landed
Date: 2026-06-28
Card: MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001
---

# MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001

## Summary

Repair the stale current-state exact pins in the composed-prefix guard family
so the guard suite no longer false-reds when `CURRENT_STATE.toml` advances
past the historical design-stop row. This is a guard/fixture repair, not a
new semantic owner. It preserves the explicit design-stop contract and keeps
the current-state pointer as provenance only.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Authority

Current-state pointer and existing evidence:

```text
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-continuation-v2.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-execution-path-frontier-resolution-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-readiness-resolution-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-hako-adoption-decision-recheck-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-hako-adoption-decision-recheck-v1.json
tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution_continuation.py
tools/rust_lifecycle/mirbuilder_minimal_path_composed_prefix_advance.py
tools/rust_lifecycle/mirbuilder_minimal_execution_path_frontier_resolution.py
tools/rust_lifecycle/mirbuilder_minimal_path_mainline_readiness_resolver.py
tools/checks/rust_lifecycle_mirbuilder_allocation_policy_hako_adoption_decision_recheck_guard.sh
tools/checks/rust_lifecycle_mirbuilder_allocation_policy_hako_adoption_decision_recheck_002_guard.sh
tools/checks/current_state_pointer_guard.sh
```

## Required Delta

At least one code-facing delta must land:

```text
python guard repair for composed continuation/prefix/frontier/readiness
adoption recheck guard repair (stale current-state exact pins)
fixture regeneration for the affected JSON outputs
```

## Acceptance

```text
no exact current_blocker_token pin remains in the composed-prefix / readiness
/ frontier / continuation guard family
current_state in the stable outputs is pointer-backed only, not a stale
design-stop assertion
python3 tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution_continuation.py --check = green
python3 tools/rust_lifecycle/mirbuilder_minimal_path_composed_prefix_advance.py --check = green
python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_frontier_resolution.py --check = green
python3 tools/rust_lifecycle/mirbuilder_minimal_path_mainline_readiness_resolver.py --check = green
adoption recheck guards = green
current_state_pointer_guard = green
no new semantic owner
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
manual_next_owner_selection = 0
next concrete implementation lane after repair is the already-ready minimal-path
mainline pilot, not another docs-only loop
```

## Non-Claims

```text
no new semantic owner
no Source Selfhost
no HakoAdopted change
no runtime fallback
no new backend route
no new ABI
no manual next-owner selection
no full minimal-path mainline claim
```

## Next

After the guard family is green, resume with the existing minimal-path
mainline pilot route if the readiness resolver stays Ready.

## Closeout

```text
output_contract=rust-lifecycle-mirbuilder-composed-prefix-guard-drift-repair-v0
no_exact_current_blocker_token_pin=1
current_state_pointer_guard=green
adoption_recheck_guards=green
frontier_resolution_guard=green
readiness_resolver_guard=green
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
manual_next_owner_selection=0
summary=ok
```
