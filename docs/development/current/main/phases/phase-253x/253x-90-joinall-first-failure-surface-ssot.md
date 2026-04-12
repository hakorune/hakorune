Status: SSOT
Date: 2026-04-13
Scope: explicit `TaskGroupBox.joinAll(timeout_ms)` failure surface only.

# 253x `joinAll()` First-Failure Surface

## Decision

Current `TaskGroupBox.joinAll(timeout_ms)` is pinned as:

1. bounded-join the explicit task-group owner
2. return `ResultBox::Ok(void)` when no first failure is latched
3. return `ResultBox::Err(first_failure_payload)` when a first failure is latched
4. do not introduce a timeout-specific error payload in this cut
5. do not mix aggregate/multi-failure reporting into `joinAll()` in this cut

## Payload rule

- explicit-scope owner state stores the first failure as a box payload
- `joinAll()` and explicit scope exit read that same preserved payload
- current payload preservation is first-failure only in this cut

## Why this cut

- scope exit already surfaces first failure
- `joinAll()` still returned `void`, so explicit owner APIs diverged
- preserving payload boxes now avoids another contract break when aggregate reporting arrives later
