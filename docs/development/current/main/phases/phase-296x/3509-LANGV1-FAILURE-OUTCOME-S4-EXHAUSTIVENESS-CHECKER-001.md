---
Status: Active implementation task
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted inventory-only continuation
---

# 3509 - LANGV1-FAILURE-OUTCOME-S4-EXHAUSTIVENESS-CHECKER-001

## Objective

Add one deterministic guard over the Failure/Outcome inventory manifests. It
must enforce closed vocabulary and identity contracts without classifying
pending evidence or activating any runtime behavior.

## Required Guards

```text
duplicate semantic site or evidence id -> reject
missing owner/class/target on classified site -> reject
unknown semantic class -> reject
implicit conversion or projection chain -> reject
Unit/absence conflation -> reject
missing foreign-null policy -> reject
invalid four-segment site_id -> reject
compatibility_only without profile -> reject
increasing missing_argument_zero pending count -> reject
semantic activation != 0 -> reject
```

The checker must consume the S1 semantic-site graph and S2/S3 evidence
manifests. It must not infer meaning from a carrier, route, zero, or source
location.

## Acceptance

```text
clean current manifests pass
each required violation has a focused failing test
missing_argument_zero baseline is explicit and non-increasing
runtime/parser/MIR/backend behavior changed = 0
semantic activation = 0
```

## Commands

```bash
python3 tools/docs/failure_outcome_exhaustiveness.py --check
python3 -m unittest tools/docs/test_failure_outcome_exhaustiveness.py
python3 tools/docs/failure_outcome_control_flow_inventory.py --check
python3 tools/docs/failure_outcome_runtime_provider_inventory.py --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Boundary

If a guard would need to choose a semantic class or owner for a pending row,
stop at a focused design consultation. The checker may reject or preserve
pending; it may not guess.
