# Hako Alloc Provider Activation Input Bundle Inventory

Status: accepted
Decision: accepted
Scope: MIMAP-376A provider activation input bundle inventory.

## Purpose

MIMAP-376A fixes the explicit input bundle that any future provider activation
first-pattern row must consume. This is still an inventory row: it records the
candidate, kind, request token, and mode in scalar/model space, while keeping
provider activation and provider calls closed.

## Owner

```text
lang/src/hako_alloc/memory/provider_activation_input_bundle_inventory_box.hako
```

## Input Contract

The bundle consumes:

```text
HakoAllocProviderActivationUnsupportedOutcomeLedgerReport
```

The caller must also pass explicit row-owned scalar inputs:

```text
activation_request_token
activation_mode
```

No hidden environment variable, implicit provider discovery, process-global
activation config, provider callback, or backend matcher may supply these
values.

## Accepted Row

An accepted bundle requires:

```text
unsupported-outcome report is present and accepted
provider candidate token is valid
provider kind is valid
activation request token > 0
activation mode > 0
provider activation remains unsupported and inactive
host replacement, hooks, backend matcher, provider calls, and threads remain inactive
```

## Stop Lines

- No provider activation or provider calls.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```text
validation_profile = scalar-mir
exe = deferred-to-first-pattern
```
