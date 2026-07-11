---
Status: Read-only consultation input
Date: 2026-07-12
Decision: pending activation design stop
Owner: LANGV1-FAILURE-OUTCOME-ACTIVATION-DESIGN-STOP-001
---

# Failure/Outcome Activation Candidate Inventory

## Scope

This document is a worker-style, read-only inventory for selecting the first
semantic activation boundary. It does not choose a semantic owner, change a
carrier, enable a provider, or alter a backend route.

The current inventory has four candidate families:

```text
hako_mem_free.success
provider missing-result fallback
Weak upgrade failure
FFI/provider nullable boundary
```

## Candidate Matrix

| Candidate | Authority evidence | Current carrier / projection | Collision or ambiguity | Current rank |
| --- | --- | --- | --- | --- |
| `hako_mem_free.success` | `substrate-capabilities.md:143` declares `-> void`; `:157` says NULL is a no-op; `route_spec.rs:427-434` names the route and sentinel; `mem.rs:85-87` is the producer | Unit candidate; `void_sentinel_i64_zero`; route test `hako_mem.rs:45-56` | route metadata still says `value_demand=scalar_i64`; consumer discard must be confirmed | 1 |
| provider missing-result | `externals.rs:54-58`, `:218-222`, `:226-243`, `:305-309` use `unwrap_or(Ok(VMValue::Void))` | shared `VMValue::Void`; six pending observations | provider absence, successful no-result, not-found, and domain error are collapsed; API contracts are missing | 2 |
| Weak upgrade failure | `types.md:293`; builder `method_call_handlers.rs:250-253`; runtime host bridge maps failed upgrade to `VoidBox` at `host_box_ops.rs:21-26` | target is an `Option::None` candidate, current result is Void/Null-like | language target and runtime carrier disagree; changing it is a semantic migration, not a projection-only fix | 3 |
| FFI/provider nullable boundary | `nyrt_weak_to_strong` at `crates/nyash_kernel/src/ffi/weak.rs:58`; plugin ABI status/null examples in `bid-ffi-v1-actual-specification.md:23-32,159-160` | foreign null/status observations; adapter not activated | many ABI contracts and directions; no single operation owner or adapter corridor | 4 |

## Evidence Interpretation

### `hako_mem_free.success`

This is the strongest candidate because the public API explicitly declares a
void outcome and the route has a named projection encoding. The evidence is
still not sufficient to activate it automatically: the route carries a scalar
i64 demand and the first consultation must confirm that the returned zero is
ABI-only and never observable as an integer value by the language caller.

Required consultation facts:

```text
source authority = hako_mem_free public API contract
semantic owner = one explicit API/operation owner
target = Unit
payload policy = NoPayload
zero collision = NotAValueLane
consumer observability = discarded at the declared boundary
unsupported consumer = fail-fast before effects
```

### Provider missing-result

The six fallback rows are evidence of a missing provider lane, not evidence of
Unit. They must remain pending until each operation contract distinguishes
provider unavailable, not-found, recoverable domain error, and successful
no-result. The activation default for a required provider can be discussed as
Fault, but it must not be implemented from the `unwrap_or` syntax alone.

### Weak upgrade failure

The language reference describes dead/freed upgrade as absence, while the
current runtime publishes a shared Void-like carrier. This is a genuine
semantic migration candidate and therefore needs an owner, fixture matrix, and
VM/EXE product decision before activation. It is not a safe first projection
slice.

### FFI/provider nullable boundary

The ABI evidence demonstrates several nullable/status conventions, but it does
not identify one operation-level adapter with a complete consumer contract.
`ForeignNull` must remain boundary-only until one explicit adapter corridor is
selected.

## Proposed Consultation Questions

```text
Q1. Does hako_mem_free's scalar_i64 route result have an ABI-only discard
    contract, or can the caller observe Integer(0)?

Q2. If the result is ABI-only, accept exactly one first slice:
    hako_mem_free.success -> Unit -> VoidSentinelI64Zero?

Q3. What exact unsupported-consumer behavior is required before effects?

Q4. Are provider missing-result rows kept pending until per-operation API
    contracts are written, with required-provider absence defaulting to Fault?

Q5. Which single FFI/provider operation, if any, has enough boundary evidence
    to become a later ForeignNull adapter candidate?
```

## Non-Claims

```text
candidate rank is not semantic approval
hako_mem_free Unit activation = 0
provider fallback correction = 0
Weak upgrade behavior change = 0
ForeignNull adapter activation = 0
runtime/backend behavior change = 0
```

The next action is external design consultation. No semantic implementation is
authorized by this inventory.
