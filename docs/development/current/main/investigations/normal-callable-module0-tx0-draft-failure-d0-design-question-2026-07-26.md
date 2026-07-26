---
Status: active design stop
Date: 2026-07-26
Decision input: NORMAL-SOURCE-PLAN0-prime-r1
Stop: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-D0
Blocks: NORMAL-CALLABLE-MODULE0-TX0-S0
Scope: exact rejection retention boundary after a function lowering plan has been consumed inside an unpublished Builder session
Related:
  - docs/development/current/main/investigations/normal-callable-module0-tx0-s0-execution-task-2026-07-26.md
  - src/mir/builder/resolved_lowering/mod.rs
  - src/mir/builder/resolved_lowering/draft_seal_owner.rs
  - src/mir/compiler/normal_source_plan/normal_acyclic_module_plan.rs
---

# NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-D0

## Why this stop exists

R0 now has the desired source authority:

```text
CompletedNormalMainHelperResolutionV1
  -> borrow one topology plan
  -> Acyclic or Recursive once
```

TX0 must lower:

```text
all helper plans
source Main
physical main thunk
```

before one atomic module commit.

The current function-draft API is:

```rust
fn lower_resolved_trivial_function_draft(
    &mut self,
    plan: CanonicalTrivialBindingSsaPlanV1<'_>,
) -> Result<MirFunction, CanonicalResolvedBuildErrorV1>
```

It consumes the plan by value:

```text
plan.into_parts()
-> input / if-control / completion / profile
-> open unpublished function session
-> body lowering
```

On body-lowering failure it does:

```text
session.discard_unpublished()
-> String-backed CanonicalResolvedBuildErrorV1
```

On draft-seal failure it does:

```text
RejectedFunctionDraftSealV1
-> stage/error formatted
-> rejected.discard()
-> String-backed CanonicalResolvedBuildErrorV1
```

Consequently, after the first consumed helper/Main lowering plan fails, TX0
cannot return the exact original
`CompletedNormalMainHelperResolutionV1`: some of the operational proof values
have already been consumed and intentionally discarded.

This is not a module-publication problem. The module is still unpublished.
The question is what a rejection is required to retain.

## Non-negotiable laws

Every option must preserve:

```text
source AST/Program clone            = 0
source rewrite                      = 0
second catalog/resolver             = 0
second graph/SCC partition          = 0
Acyclic-to-Recursive retry          = 0
profile/source-family retry         = 0
partial module publication          = 0
live Builder replacement on failure = 0
Legacy/Raw fallback                 = 0
resume capability                   = 0
```

Every prepared helper draft before the failing row must be retained as an
exact ordered unpublished prefix until the rejection is inspected/discarded.

## Q1 — rejection retention authority

### A — exact original completed owner

Require every TX0 failure to retain:

```text
the exact original CompletedNormalMainHelperResolutionV1
+ every prepared draft prefix
+ exact typed failure
+ Builder restoration receipt
```

This requires a new owner-preserving function-lowering transaction:

```text
OpenTrivialFunctionDraftLoweringV1
-> borrow-only preparation
-> PreparedTrivialFunctionDraftLoweringV1
-> infallible body/draft commit
```

or a rejected terminal that can reconstruct and return every consumed
preflight proof.

Practical impact:

```text
CanonicalTrivialBindingSsaPlanV1 lifecycle changes
CanonicalTrivialSsaLowererV1 failure lifecycle changes
body-lowering errors stop being String-only
draft-seal rejected owner is no longer discarded by the facade
existing callable-module transactions need adaptation/parity
```

This is the strongest typestate rule, but it is a substantial lowerer-wide
BoxShape series rather than a bounded TX0 composition.

### B — source authority plus consumed-operation receipt (recommended)

Retain the durable semantic authority, not the already-consumed operational
capability object:

```text
RetainedNormalCallableSourceAuthorityV1
  - one owned Program/catalog source owner
  - exact Main site and helper declaration sites
  - exact selected family/topology receipt
  - exact source-entry relation facts

+ RetainedNormalCallableDraftPrefixV1
  - every successfully prepared unpublished draft

+ ConsumedNormalLoweringCapabilityReceiptV1
  - exact key/role that was consumed
  - exact lowering stage
  - whether draft seal was reached

+ NormalCallableBuilderRestorationReceiptV1
+ nested typed/bounded cause
```

Do not retain:

```text
the exact consumed CanonicalTrivialBindingSsaPlanV1
resume/retry capability
reconstructable Builder session
```

Rationale:

```text
Program/catalog/source sites are semantic authority
preflight plan is a single-use operational capability
rejection is inspection + discard only
no caller may resume or retry it
```

This keeps the source truth and all materialized unpublished work without
making one-shot lowering reversible.

Required bounded refactor:

```text
1. split CompletedNormalMainHelperResolutionV1 only inside TX0:
     durable source authority
     consumable operational proofs

2. add a normal retaining helper-draft loop:
     preserve canonical-key ordered successful prefix

3. add a typed lowering-failure adapter:
     stage + bounded cause + restoration receipt
     existing general public facade may remain String-compatible

4. discard only through RejectedNormalCallableModuleTransactionV1
```

The exact source authority must never expose AST/source accessors that allow
reclassification.

### C — current String failure and discarded owner

Keep current lowering APIs unchanged and retain only:

```text
error String
possibly a count
```

This conflicts with the accepted TX0 failure law and is rejected.

## Recommendation

Choose **B**.

The important distinction is:

```text
source authority retention
  !=
operational capability rollback
```

TX0 has no resume/retry terminal. Requiring a consumed preflight capability to
be recreated solely so it can later be discarded increases temporal coupling
without protecting source truth or atomic publication.

Option B still retains:

```text
the exact original Program/catalog owner
the exact source identities/sites
the selected topology as an owned receipt
the complete successfully prepared draft prefix
the exact failure stage
the Builder restoration proof
```

It does not weaken:

```text
source semantics
function-exit semantics
topology selection
module atomicity
fallback prohibition
```

## Q2 — typed function-lowering failure

Recommended:

```text
new TX0-private typed adapter =
  required

existing compatibility facade =
  unchanged
```

Conceptual vocabulary:

```rust
enum NormalFunctionDraftLoweringStageV1 {
    SessionOpen,
    BindingInstall,
    Skeleton,
    BodyLowering,
    DraftSeal,
    SessionRestore,
}

enum NormalFunctionDraftLoweringCauseV1 {
    BuilderContract(Box<str>),
    DraftSeal {
        stage: FunctionDraftSealStageV1,
        detail: Box<str>,
    },
}
```

The nested cause may initially retain a bounded detail for legacy lowerer
errors. The outer stage must not be inferred by parsing the error string.

Do not widen this row into a repository-wide error taxonomy rewrite.

## Q3 — helper prefix

Decision proposed as mandatory:

```text
prepared helper prefix retention = exact
```

On helper `k` failure:

```text
helpers before k:
  retained as key + verified unpublished MirFunction

k:
  exact consumed key and failure stage retained

helpers after k:
  not attempted

source Main / physical thunk:
  not attempted
```

On Main failure:

```text
all helper drafts retained
Main failure retained
physical thunk not attempted
```

On thunk/correspondence/verification failure:

```text
all helper drafts
+ source Main draft
+ physical thunk when already prepared
```

No prefix enters a module before complete preparation succeeds.

## Q4 — successful terminal

Both A and B keep the same success chain:

```text
all fallible preparation
-> PreparedNormalCallableModuleTransactionV1
-> infallible commit
-> CompletedNormalCallableModuleCandidateV1
```

No `Result` operation is added after commit begins.

## Executable order after acceptance

If B is accepted:

```text
NORMAL-CALLABLE-MODULE0-TX0-HANDOFF0-S0
  durable source authority + consumable operational proof split

-> NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-S0
  exact helper-prefix retention
  typed lowering failure adapter
  source Main + physical thunk preparation

-> NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0
  schema/capability/correspondence/full verification

-> NORMAL-CALLABLE-MODULE0-TX0-COMMIT0-S0
  one infallible shell drain

-> NORMAL-CALLABLE-MODULE0-TX0-G0
  failure/reuse/atomicity/guard closeout
```

If A is accepted:

```text
FUNCTION-DRAFT-LOWERING-PREPARE-COMMIT-D0
-> lowerer-wide BoxShape series
-> TX0 resumes only after that owner is closed
```

## Required answer

```text
Decision:
  NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1

Q1:
  A exact original completed owner
  or
  B source authority + consumed-operation receipt

Q2:
  typed TX0-private lowering failure adapter?

Q3:
  exact prepared helper-prefix retention?

Q4:
  commit remains infallible after all preparation?
```

## Non-claims

```text
new language/result capability
new parser/AST behavior
module publication
VM/runner/profile activation
default route
imports/using
Legacy retirement
repository-wide lowerer error rewrite
```
