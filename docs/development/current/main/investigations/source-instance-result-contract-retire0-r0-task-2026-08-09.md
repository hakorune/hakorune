---
Status: parked design task; implementation not opened
Date: 2026-08-09
Decision: retire or quarantine the old body-inferred instance-result target before declaration-first target I0
Parent: `docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`
---

# SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0

## Purpose

Remove the risk of two instance-call target authorities. The old
`source_instance_result_contract` family infers result meaning from an
unannotated body and resolves a call-site receiver through a Builder-side
catalog. The accepted declaration-first design instead issues a reusable
resolver target from a parser-sealed declaration and verifies the body
against that declared contract.

## Required census before implementation

```text
old target types and constructors
old body-result inference callers
old rebind/preloop association callers
test-only witnesses that can migrate to the new target negative matrix
```

The census must classify every use as `retire`, `quarantine`, or
`migrate-to-declaration-first`. It must not leave the old target alive beside
the new resolver target without an explicit compatibility boundary.

## Retire boundary

```text
old body inference -> no new semantic result contract
old call-site name/catalog lookup -> no new reusable target
old target -> no Recipe/CallSlot/physical consumer
```

Any remaining compatibility test must be isolated behind an explicit test or
bootstrap adapter, with a removal condition in the closeout receipt. The new
declaration-first target remains unopened until this row is closed.

## Acceptance gates

```text
all old callers are enumerated
no production caller issues the old target
replacement negative tests cover missing/foreign/duplicate/ambiguous target
resolver target I0 has one declaration-owned authority
source_instance_result_contract module is retired or quarantined explicitly
same-commit README/reference/task-map update
```

## Nonclaims

```text
no resolver target implementation
no Recipe/CallSlot
no Builder/MIR/provider/runtime route
no body inference promotion
no fallback from declaration-first to old target
```
