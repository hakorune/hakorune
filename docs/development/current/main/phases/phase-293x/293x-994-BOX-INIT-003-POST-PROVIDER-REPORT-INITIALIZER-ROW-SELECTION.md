# 293x-994 BOX-INIT-003 Post Provider Report Initializer Row Selection

Status: current
Date: 2026-05-21

## Purpose

Choose the next row after provider report-copy helpers adopted explicit
`new Box { field: expr }` construction-site initializers.

## Candidate A: Same-Name Initializer Shorthand

Accept only explicit same-name copy entries:

```hako
local result = new Report {
    fields.accepted
    fields.reason
}
```

Still closed:

```hako
new Report { fields.* }
new Report { ...fields }
new Report(accepted: fields.accepted)
```

Validation:

```text
L2 daily: parser + MIR sugar lowering + duplicate/unknown diagnostics
```

## Candidate B: Continue Provider Activation Follow-Up

Return to parked MIMAP-375A provider activation row selection.

Provider activation, provider calls, host allocator replacement, hooks,
`#[global_allocator]`, backend matcher additions, worker/TLS behavior, and
source concurrency remain closed unless an explicit later row opens them.

## Recommendation

Prefer Candidate B unless report-copy boilerplate remains the immediate
development bottleneck.
