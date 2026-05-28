---
Status: Landed
Date: 2026-05-28
Scope: design a narrow MIR builder keeper for same-block field_get direct-consumer forwarding.
Blocker: FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-181-FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE.md
---

# 296x-182 Field Get Direct Consumer Forwarding Keeper Design

## Purpose

Open a narrow keeper after row181 found 11 concrete `field_get` expression copy
chains with real consumers. The keeper must not become broad LocalSSA copy
coalescing.

## Keeper Rule

```text
If LocalSSA is asked to materialize an Arg or CompareOperand and the source
ValueId is defined by FieldGet in the current basic block, return the FieldGet
result directly and cache that direct value for the LocalSSA key.
```

## Non-Goals

```text
- Do not forward across blocks.
- Do not forward Phi, Call, BinOp, Compare, or arbitrary Copy roots.
- Do not change receiver/field-base materialization.
- Do not add a source-level .hako rewrite.
- Do not add generic MIR CSE.
```

## Acceptance

```text
semantic proof summary=ok
instruction_count=162
copy_count=70
local_ssa_copy_count=20
expression_materialization_copy_count=9
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_field_get_direct_consumer_forwarding_keeper_guard.sh
```
