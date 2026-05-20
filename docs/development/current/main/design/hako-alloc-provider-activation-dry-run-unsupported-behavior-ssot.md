# Hako Alloc Provider Activation Dry-Run Unsupported Behavior

Status: accepted
Decision: accepted
Scope: MIMAP-378A provider activation dry-run unsupported behavior.

## Purpose

MIMAP-378A consumes the explicit MIMAP-376A provider activation input bundle
and records a dry-run unsupported activation outcome. This row proves the
behavior boundary after input bundling, but it still does not activate a
provider or call provider APIs.

## Owner

```text
lang/src/hako_alloc/memory/provider_activation_dry_run_unsupported_behavior_box.hako
```

## Input Contract

The dry-run owner consumes:

```text
HakoAllocProviderActivationInputBundleInventoryReport
```

The input bundle must be present, accepted, explicitly supplied, and still mark
provider activation as unsupported/inactive.

## Accepted Row

An accepted dry-run outcome requires:

```text
input bundle is present and accepted
activation request token is valid
activation mode is valid
provider activation remains unsupported
provider activation remains inactive
provider calls, host replacement, hooks, backend matcher, and threads remain inactive
```

The accepted row may set `dry_run_attempted = 1` and
`unsupported_outcome_present = 1`. It must keep every `would_*` execution flag at
zero.

## Stop Lines

- No provider activation or provider calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```text
validation_profile = scalar-mir
exe = deferred-to-closeout
```
