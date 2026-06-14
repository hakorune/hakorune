---
Status: Active
Date: 2026-06-15
Task: CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001
Scope: Design the dominance/visibility contract for residual call-operand
  forwarding.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-690-CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001

## Purpose

296x-690 selected the next family:

```text
selected_policy_family=dominance_required_call_operand_forwarding
selected_policy_candidate_count=14
rejected_policy_family=arg_same_block_root_forwarding
rejected_policy_candidate_count=7
```

This row designs the dominance/visibility contract before any implementation.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-design-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-690
selected_policy_family=dominance_required_call_operand_forwarding
pre_candidate_count=14
safe_dominance_candidate_count=<n>
unsafe_candidate_count=<n>
selected_keeper_shape=<shape>
selected_keeper_candidate_count=<n>
arg_forwarding_enabled=<0|1>
requires_dominance_guard=1
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not implement in this row
do not patch source .hako
do not reopen startup optimization
do not broaden LocalSSA copy coalescing
do not forward unknown-root chains
do not special-case helper names
do not change variable_map semantics
do not change PHI lifecycle
do not claim a winner
```

## Acceptance

```text
call_operand_dominance_required_forwarding_design_active=1
source_evidence=296x-690
design_run=0
selected_keeper_shape=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
