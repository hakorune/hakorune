---
Status: Active implementation task
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted inventory-only continuation
---

# 3508 - LANGV1-FAILURE-OUTCOME-S3-CONTROL-FLOW-INVENTORY-001

## Objective

Record control-flow evidence that can affect Failure/Outcome relations without
changing parser, MIR, runtime, VM, cleanup, or backend behavior. This is the
next inventory-only slice after the accepted S2 runtime/provider inventory.

## Scope

The manifest must contain deterministic evidence rows for these closed
families:

```text
local_default
return_unit
fault_and_throw
cleanup_precedence
catchability
top_level_normalization
```

Each row preserves source path, line, token, evidence kind, and evidence text.
An existing semantic site may be referenced only when the operation identity is
explicit; otherwise the row remains pending with a closed reason.

## Guard Contract

```text
duplicate control-flow evidence id -> reject
unknown family -> reject
missing source location/text -> reject
pending row with site reference -> reject
unknown pending reason -> reject
semantic activation != 0 -> reject
runtime/parser/MIR/backend behavior change -> out of scope
```

## Acceptance

```text
all six control-flow families have deterministic evidence rows
rows retain stable source location and evidence kind
unresolved relations remain pending rather than heuristically classified
cleanup precedence and catchability are evidence, not activation
S1/S2 manifests remain green
semantic activation = 0
```

## Commands

```bash
python3 tools/docs/failure_outcome_site_inventory.py --check --strict
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
python3 tools/docs/failure_outcome_projection_binding.py --check
python3 tools/docs/failure_outcome_runtime_provider_inventory.py --check
python3 tools/docs/failure_outcome_control_flow_inventory.py --check
python3 -m unittest tools/docs/test_failure_outcome_control_flow_inventory.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Boundary

If a control-flow row requires choosing whether an existing `Void` carrier is
`Unit`, absence, `Err`, or `Fault`, stop at a focused design consultation.
Evidence location, current carrier, and green execution are not semantic
authority.
