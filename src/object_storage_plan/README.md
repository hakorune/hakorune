# object_storage_plan

Passive vocabulary for exact-AOT local-first object and fastpath planning.

This module does not lower code. It defines the small set of data carriers that
later planners and backends may read.

## Concept Groups

Keep new vocabulary inside one of these groups:

- `storage`
  - `ObjectStoragePlan`, `ObjectPlan`, scalar/flattened field plans
  - owns representation names only
- `publication`
  - publication sites, publication state, publication reasons
  - owns when a local object leaves the unpublished local region
- `alias`
  - local alias observations and source kinds
  - owns conservative value-to-alias-class inputs
- `inventory`
  - report-only rows and shadow rows
  - owns inputs to eligibility, not backend-consumable proof
  - owns `ReasonDomain` report classification; it does not replace owner-specific reason enums
- `decision`
  - `FastPathDecision` and `PlanEpoch`
  - owns `Allow(LocalFastPathFact)` / `Deny(reason)` shape
- `fastpath`
  - `LocalFastPathFact`, fastpath kind, deny reason
  - owns positive backend-consumable permission

## Boundary Rules

```text
backend reads LocalFastPathFact only
fallback evidence is not a fact
deny decisions are not exported to MIR JSON
reachability is owned by hako_check tooling, not object_storage_plan Rust types
```

Do not add near-synonym types for an existing concept group. If a new concept
does not fit one of the groups above, add a design row before adding code.

## Stop Lines

```text
do not put backend emission here
do not infer from source names, helper symbols, or benchmark names
do not add fallback facts
do not make inventory rows backend-readable
do not split PublicationPlan out until ObjectPlan becomes too large
```
