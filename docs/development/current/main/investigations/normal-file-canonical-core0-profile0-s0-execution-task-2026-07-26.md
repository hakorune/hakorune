---
Status: closed
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0
Scope: issue the separate, frozen-name canonical-core normal-file profile without connecting a production caller
ceremony_tier: T1 bounded profile-owner extension
proof_inventory_before: frozen NormalFileNoImportVmReferenceV1, closed source-plan/callable-TX0 chain, and shared neutral VM-reference terminal
new_proofs: one profile identity/admission fixture family
retired_or_merged_proofs: none
net_proof_delta: +1 bounded profile fixture
sunset_id: NORMAL-CANONICAL-CORE-PROFILE-PROOF-SUNSET-001
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: canonical-core route guard owns the durable profile assertions and disconnected profile-only consumers are zero
budget_repayment_evidence: canonical-core profile/caller G0 absorbs the fixed profile identity and frozen-narrow assertions
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-callable-module0-tx0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/source-entry-vmref-neutral0-l0-execution-task-2026-07-26.md
---

# NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0

## Outcome

Create one new sealed profile identity:

```text
NormalFileNoImportVmReferenceV1
  = frozen Raw scalar/reference lane

NormalFileCanonicalCoreVmReferenceV1
  = separate canonical-core lane
```

This row creates no CLI selector, runner caller, compiler fallback, or default
route. It only gives later canonical-core compilation one typed source/profile
authority instead of widening the existing Raw profile in place.

## Owner boundary

The profile owner is the existing NormalFile front-door profile boundary.
Refactor its private representation if needed so each variant carries exactly
one downstream capability family:

```text
FrozenRawReference
  -> existing Raw VM-reference support profile

CanonicalCoreReference
  -> canonical source-plan/callable candidate capability
  -> shared neutral VM-reference terminal (later consumer only)
```

The new variant must not construct a Raw compile request, inspect a final AST
tail, infer a function result, or select an entry. Those remain respectively
the Raw source owner, Script result owner, F1/Main owner, and sealed source
plan owner.

## Fixed canonical-core axes

```text
source origin/read/parse = one UTF-8 file / one read / Canonical parse once
imports and using         = reject
source rewrite            = zero
optimization              = CanonicalDefaultOptimizedV1 only
execution                 = fresh VM-reference interpreter, later only
entry/result/status       = existing sealed owners, later only
fallback/retry            = zero
```

The canonical-core profile admits only the source families already sealed by
the current plan chain:

```text
Script
Main.main/0
Main.main/0 + top-level callable module
```

It does not yet make all of those families executable from the front door.

## Implementation cells

```text
P0-A PROFILE-VOCAB0
  split the private profile representation so narrow Raw and canonical-core
  are distinct, sealed identities

P0-B REQUEST0
  add one crate-private canonical-core file request factory; preserve exactly
  one path and one sealed profile, with no caller-selected booleans

P0-C SOURCE-PLAN0
  carry the canonical-core profile through the existing one-read/one-parse
  source-plan request without a Raw handoff or route selection

P0-D FIXTURE/G0
  prove frozen narrow behavior, canonical-core identity preservation, and
  profile/source rejection before compiler effects using the existing normal
  front-door guard (no new shell wrapper)
```

## Acceptance

```text
existing normal-file-vm-reference profile spelling/behavior = unchanged
canonical-core profile identity                             = exactly one
new request owns exactly one NormalFileRequestV1-equivalent = green
one file read / one parse                                   = green
imports/using reject before compiler handoff                = green
Raw compile handoff from canonical-core profile             = 0
source plan / AST / entry reclassification                  = 0
CLI/default/runner/backend caller delta                     = 0
fallback/retry                                              = 0
all modified/new source/check files                         < 800 lines
```

## Immediate continuation

```text
NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
-> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
-> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
```

`PARITY0-P0a` may connect only the already-sealed source-plan families to
their existing candidate/publication/neutral-entry owners. A missing source
family capability is a typed rejection, never a profile fallback.

## Non-claims

```text
new production CLI caller
default route cutover
compile_with_source change
second compiler or VM executor
Raw profile widening
imports/using support
dynamic/object result carrier
Legacy retirement
```

## Closeout

```text
cargo check --lib --features vm-reference                  = green
normal_file_vm_frontdoor focused tests (21)                = green
normal_file_vm focused tests (26)                          = green
normal_file_vm0_frontdoor_forge_guard.py                   = green
current_state_pointer_guard.sh                             = green
```

Landed boundary:

```text
frozen Raw profile
  -> existing Raw handoff only

canonical-core profile
  -> one-read/one-parse/source-plan boundary
  -> Raw handoff typed reject with the full source owner retained
```

The next connection requires a new sole source-plan dispatch owner. It is not
a parity-only extension, so implementation stops at
`NORMAL-FILE-CANONICAL-CORE0-DISPATCH-D0`.
