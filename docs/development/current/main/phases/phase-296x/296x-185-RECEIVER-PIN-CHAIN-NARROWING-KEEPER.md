---
Status: Current
Date: 2026-05-28
Scope: remove the redundant LocalSSA recv copy after receiver pinning.
Blocker: RECEIVER-PIN-CHAIN-NARROWING-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-184-RECEIVER-PIN-CHAIN-POLICY-SELECTION.md
---

# 296x-185 Receiver Pin Chain Narrowing Keeper

## Purpose

Narrow the receiver materialization chain without removing receiver pinning.
`pin_to_slot(receiver, "@recv")` already emits a same-block Copy and registers
the pin slot for PHI/loop tracking. The extra `local::recv(r_pinned)` copy is
redundant for the immediate call receiver.

## Keeper Rule

```text
For method receivers, keep pin_to_slot("@recv") but use the pinned value as the
call receiver directly. Do not emit a second LocalSSA recv copy from the pinned
value.
```

## Non-Goals

```text
- Do not remove receiver pinning.
- Do not skip pinning across blocks.
- Do not change argument materialization.
- Do not add receiver sharing/cache policy.
- Do not add generic copy coalescing.
```

## Acceptance

```text
semantic proof summary=ok
instruction_count=153
copy_count=61
receiver_attributed_copy_count=18
unique_receiver_copy_count=15
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_receiver_pin_chain_narrowing_keeper_guard.sh
```
