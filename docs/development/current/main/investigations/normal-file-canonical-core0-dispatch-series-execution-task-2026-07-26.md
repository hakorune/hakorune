---
Status: active
Date: 2026-07-26
Decision: NORMAL-FILE-CANONICAL-CORE0-DISPATCH-prime-r1
Row: NORMAL-FILE-CANONICAL-CORE0-DISPATCH0-S0
Scope: connect one sealed canonical-core source plan to exactly one existing family compiler, preserve an unpublished candidate boundary, and prepare one canonical publication path without Raw fallback
ceremony_tier: T2 new consuming dispatch and publication authority
series_mode: BoxShape only; accepted source/result shapes do not grow
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
proof_inventory_before: canonical-core profile proof, canonical Main candidate/VM proof, callable TX0 candidate proof, Raw Script result proof
new_proofs: one bounded dispatch family matrix and one canonical publication correspondence proof
retired_or_merged_proofs: provisional Main-only dispatch proof is merged into the three-family production-shaped proof before caller activation
net_proof_delta: temporary +1
sunset_id: NORMAL-CANONICAL-CORE-DISPATCH-PROOF-SUNSET-001
sunset_budget: one disconnected dispatch proof
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: canonical-core caller equals one, all three family routes use the sole dispatcher, fallback equals zero, and the disconnected Main-only consumer is zero
Related:
  - normal-file-canonical-core0-dispatch-d0-design-question-2026-07-26.md
  - normal-script0-physical-entry-d0-design-question-2026-07-26.md
  - normal-file-canonical-core0-profile0-s0-execution-task-2026-07-26.md
  - normal-callable-module0-tx0-s0-execution-task-2026-07-26.md
  - hakorune-sparse-ownership-surface-task-2026-07-15.md
---

# NORMAL-FILE-CANONICAL-CORE0-DISPATCH0-S0

## Outcome

The series adds one compiler-layer dispatch boundary:

```text
ClassifiedNormalFileSourcePlanV1
  ↓ consuming front-door projection
CanonicalCoreSourcePlanCompileRequestV1
  - SealedNormalSourcePlanV1
  - VerifiedCanonicalCoreSourcePlanAdmissionV1
  - one-read/one-parse receipt
  ↓ one family match in compiler layer
NormalCanonicalCoreSourcePlanCompilerV1
  ↓
CompletedCanonicalCoreSourceEntryCandidateV1
  - Script
  - Main
  - Callable
  ↓ one canonical publication boundary
PublishedSourceEntryInvocationV1
  ↓ existing neutral VM-reference execution
```

The front door decides only that the sealed profile is canonical-core. It
never matches Script/Main/Callable and the compiler never imports runner
types.

## Fixed authority split

```text
source family:
  SealedNormalSourcePlanV1

profile capability:
  VerifiedCanonicalCoreSourcePlanAdmissionV1
  one consumed canonical-core capability witness

sole family dispatcher:
  NormalCanonicalCoreSourcePlanCompilerV1

Script result semantics:
  existing RawScriptBodyRecipeV1

Main semantics/candidate:
  existing F1 + normal Main transaction owners

Callable semantics/candidate:
  existing catalog/topology/TX0 owners

publication:
  one canonical-core publication owner after a complete candidate

execution/process:
  existing neutral VM-reference owner
  existing ProcessExitProjectionV1
```

Non-authorities:

```text
runner backend spelling
profile name
last ValueId
physical Return scan
module inventory
function symbol
NYASH_ENTRY
Raw invocation brand
runtime VMValue kind
```

## Failure and ownership law

```rust
enum CanonicalCoreDispatchStageV1 {
    Handoff,
    FamilyCapability,
    ScriptCandidate,
    MainCandidate,
    CallableCandidate,
    Publication,
}

struct RejectedCanonicalCoreNormalDispatchV1 {
    owner: RetainedCanonicalCoreSourcePlanOwnerV1,
    stage: CanonicalCoreDispatchStageV1,
    cause: CanonicalCoreDispatchErrorV1,
}
```

Every rejection retains the complete classified source owner, canonical-core
profile evidence, and read/parse receipt. Borrowing Main/callable preflight
errors are consumed inside a local scope and projected to typed causes. An
owner is never stored beside a plan borrowing that owner.

Allowed exits:

```text
stage()
cause()
bounded_report()
discard(self)
```

Forbidden:

```text
into_owner
retry / resume
try another family/profile
String-only internal flattening
AST clone/rewrite
Raw fallback
partial publication
```

## Buildable implementation cells

### S0-A — `DISPATCH-INPUT0`

Add a thin consuming projection from the front door to a compiler-neutral
request. Seal `VerifiedCanonicalCoreSourcePlanAdmissionV1` from the exact
canonical-core profile without inspecting the source-family variant.

```text
runner family match             = 0
compiler -> runner import       = 0
bare AST/source accessor        = 0
production caller              = 0
```

### S0-B — `DISPATCH-MAIN0`

Add the sole compiler match and connect only `ScalarRoot::Main0`:

```text
Main0
  -> existing source relation
  -> existing F1 plan
  -> existing Main candidate transaction
  -> CompletedCanonicalCoreSourceEntryCandidateV1::Main

Script / CallableModule
  -> FamilyCapabilityPending before Builder effects
```

This is the first executable slice. It does not publish or execute.

### S0-C — `SCRIPT0`

Reuse the existing Script terminal/result recipe below source classification,
but prepare a canonical Script candidate.

```text
RawScriptBodyRecipeV1 semantic reuse = 1
normal-only Script tail classifier   = 0
RawPublishedCompileRequestV1         = 0
RawPublishedInvocationV1             = 0
Raw invocation brand                 = 0
```

The Script candidate carries exact entry target, source result, and physical
result evidence before publication.

Implementation is paused at `NORMAL-SCRIPT0-PHYSICAL-ENTRY-D0`: the existing
Raw root body transaction is branded and cannot become this canonical owner.

### S0-D — `CALLABLE0`

Strengthen the completed callable candidate so commit retains:

```text
source Main owner
schema
exact entry relation
source-result contract
topology receipt
complete-candidate verification receipt
module
```

Then add one infallible publication terminal. Target/result evidence must not
be reconstructed from the module or symbol table.

### S0-E — `PUBLICATION-DISPATCH0`

Close all three family variants through one candidate enum and one publication
boundary:

```text
CompletedCanonicalCoreSourceEntryCandidateV1
  -> PreparedCanonicalCorePublicationV1
  -> PublishedSourceEntryInvocationV1
  -> existing neutral VM-reference adapter
```

The dispatcher still matches the sealed source family exactly once.

### S0-F — `FIXTURE-G0`

Close the disconnected family matrix and reuse law:

```text
Script success
-> Main success
-> Callable success

source-plan reject
-> later Main success

Main capability reject
-> later Script success

callable draft/publication reject
-> later Main success

program Fault
-> later success
```

Use the existing lane guard. Do not add a per-cell shell wrapper.

## Near production series

After the dispatch series is green:

```text
NORMAL-FILE-CANONICAL-CORE0-REQUEST0-S0
-> NORMAL-FILE-CANONICAL-CORE0-REPORT0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
-> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
-> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0b
-> NORMAL-FILE-CANONICAL-CORE0-G0
-> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
```

`CALLER0-I0` is the first production consumer. It remains CLI-visible,
default-off, and separate from the frozen narrow profile.

## Alias / View handoff before product promotion

Ownership activation does not block this canonical-core series. It does block
product/default promotion.

```text
MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
-> OWNERSHIP-SPARSE-RESUME-D0
-> Pack A: syntax safety and evidence
-> Pack B: passive ownership grammar / Loan Flow
-> Pack C: first ScopedAlias
-> Pack D: callable ownership ABI
-> Pack E: first Anchored View
-> OWNERSHIP-SPARSE-PRODUCT-READINESS-D0
-> NORMAL-ENTRY-PRODUCT-BACKEND-D0
-> NORMAL-DEFAULT-CALLER-CENSUS0-P0
-> NORMAL-ENTRY-PROMOTION-D3
```

The exact ownership rows remain owned by
`hakorune-sparse-ownership-surface-task-2026-07-15.md`.

## Structural gate

```text
compiler-layer family match                         = 1
runner-layer family match                           = 0
source family classification                        = 1
profile capability consumption                      = 1

complete unpublished candidate boundary             = 1
canonical publication owner                         = 1
neutral VM executor                                 = existing 1
ProcessExitProjectionV1 authority                   = existing 1

canonical-core Raw handoff                          = 0
Raw invocation brand in canonical candidates        = 0
module/symbol entry reconstruction                  = 0
NYASH_ENTRY / execute_module discovery              = 0
fallback/retry/reclassification                     = 0

failure retains complete classified owner           = 1
partial module publication on failure               = 0
production caller before CALLER0-I0                 = 0
default route delta                                 = 0

all modified/new source/check files                 < 800 lines
```

## Verification

Each buildable cell runs:

```bash
cargo check --lib --features vm-reference
cargo test -q --lib normal_source_plan --features vm-reference
cargo test -q --lib normal_module_transaction --features vm-reference
python3 tools/checks/lib/normal_file_vm0_route_guard.py
bash tools/checks/current_state_pointer_guard.sh
```

Focused test filters may be used while a cell is incomplete. The active card
must record exact green commands before advancing.

## Non-claims

```text
new source/result carrier
imports/using
instance methods
Main-box helper methods
ownership alias/view activation
default/product backend selection
compile_with_source change
Legacy caller retirement
LLVM/native/ny_main activation
JSON / REPL / Stage1
executor / selfhost / fastmem
CUT0
```
