# Hako Alloc Optional Process Allocator Replacement Proposal

Status: accepted
Decision: accepted
Scope: MIMAP-425A optional process allocator replacement proposal.
Related:
- docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md

## Purpose

MIMAP-425A records the optional process allocator replacement boundary without
executing it. The current goal remains a `.hako` / `hako_alloc` allocator whose
performance and memory usage can be compared against C mimalloc. It is not a
default process allocator replacement row.

The future replacement path is optional, explicit, and parked until a separate
execution row is accepted.

Future DLL/shared-library generation is also parked. The package/ABI design is
owned by
`docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md`;
that SSOT defines provider descriptor, manifest, loader preflight, and
function-table boundaries without reopening process allocator replacement.

## Proposal Boundary

Any future process allocator replacement row must require:

```text
explicit_process_replacement_request_present
host_replacement_preflight_closeout_present
hook_install_preflight_present
backend_matcher_no_growth_closeout_present
rollback_plan_present
comparison_baseline_present
```

The row must fail fast if any activation input is implicit:

```text
hidden env
implicit discovery
process-global activation config
backend owner-name matcher
app-name matcher
```

## Current Decision

```text
replacement_execution = closed
hook_installation = closed
backend_matcher_additions = closed
global_allocator_install = closed
```

## Comparison Goal

Before any optional replacement execution can be reopened, the allocator lane
must retain comparison evidence against C mimalloc:

```text
throughput baseline
memory usage baseline
failure / rollback contract
no-growth backend matcher contract
provider package descriptor / manifest preflight contract, if a provider package
  is involved
```

## Still Closed

```text
hook installation
backend matcher additions
process allocator replacement
#[global_allocator]
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Validation

```text
validation_profile = planning
exe = not-applicable
```
