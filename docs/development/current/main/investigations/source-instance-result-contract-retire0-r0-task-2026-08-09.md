---
Status: closed design audit — deletion I0 opened
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

## Landed census — 2026-08-09

The read-only audit is complete. `src/mir/mod.rs:194` is the sole production
reachability edge for this family. `git grep` found no imports or calls to
`source_instance_result_contract::*` outside the module's seven source files
and two test files. The module is therefore caller-zero and may be deleted as
one BoxShape slice; it is not a live compatibility route.

The dead family is exactly:

```text
src/mir/source_instance_result_contract/
  target.rs                 current-owner/name/arity target lookup
  contract.rs               body-inferred ExactI64 result contract
  owned_rebind.rs           retained witness and rebind terminal
  preloop_association.rs    pre-loop result association
  preloop_located_argument.rs
  rejection.rs
  mod.rs / tests / owned_rebind_tests
```

The general `src/mir/source_call_target` source-site primitives are not part
of this retirement. They have unrelated production/test consumers and remain
the future declaration-first call-site substrate. Likewise, the general
`callable_result_representation` owners remain; only the old fixture helpers
that exist solely for this dead family are removed or renamed to a neutral
raw-source-view fixture.

The audit also found that `issue_unannotated_body_proof()` and
`body_proof_issue.rs` have five callers, all inside the retiring module's test
fixtures. They are part of the old body-inferred authority and are deleted in
I0. The static result catalog, `prove_function`, and unrelated static result
rows remain.

The next implementation slice is
`SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-I0`. It removes the module and
`mir/mod.rs` edge, removes the old-only rebind/pre-loop tests and fixture
helpers, and keeps the four raw source-view cursor tests through a neutral
fixture with no target/result contract. No new resolver target is introduced
in that slice.

## Nonclaims

```text
no resolver target implementation
no Recipe/CallSlot
no Builder/MIR/provider/runtime route
no body inference promotion
no fallback from declaration-first to old target
```
