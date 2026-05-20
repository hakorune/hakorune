# 293x-993 BOX-INIT-002 Post New-Box Field Initializer Row Selection

Status: current
Date: 2026-05-21

## Purpose

Choose the next narrow row after BOX-INIT-001 landed explicit
`new Box { field: expr }` construction-site field initializers.

## Current State

Accepted:

```hako
local report = new Report {
    accepted: fields.accepted
    reason: fields.reason
}
```

Still closed:

```hako
new Report { fields.accepted }
new Report { fields.* }
new Report(accepted: fields.accepted)
```

## Candidate A: Same-Name Field Copy Shorthand

Accept:

```hako
local report = new Report {
    fields.accepted
    fields.reason
}
```

Only same-name explicit entries are allowed. No wildcard copy.

Validation:

```text
L2 daily: parser + MIR sugar lowering + duplicate/unknown diagnostics
```

## Candidate B: Return To Provider Activation Lane

Resume from MIMAP-375A / provider activation explicit-input follow-up.

Validation follows the phase-293x mimalloc row validation cadence.

## Recommendation

Prefer Candidate A if report-copy boilerplate remains the active pain point.
Otherwise return to MIMAP-375A without reopening provider activation,
host allocator replacement, hooks, or `#[global_allocator]`.
