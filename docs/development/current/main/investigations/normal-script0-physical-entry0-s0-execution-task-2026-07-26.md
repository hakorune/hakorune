---
Status: active
Date: 2026-07-26
Decision: NORMAL-SCRIPT0-PHYSICAL-ENTRY-prime-r1
Row: NORMAL-SCRIPT0-PHYSICAL-ENTRY0-S0
Scope: build one unbranded canonical Script physical main/0 candidate while
sharing one Script result and Return kernel with Raw Script
ceremony_tier: T2 new candidate-only session, completion/result witness, and
physical exit authority
series_mode: BoxShape with one bounded conformance correction; accepted source
and result carrier shapes do not grow
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
proof_inventory_before: shared Script source recipe, Raw Script physical exit,
Main-only canonical dispatch, neutral published-source-entry VM execution
new_proofs: one shared Script terminal/exit proof, one Raw origin parity proof,
one canonical one-row candidate proof
retired_or_merged_proofs: Raw direct Script exit proof is replaced by the
shared-kernel plus Raw-adapter proof; disconnected Script dispatch proof merges
into the canonical-core family proof
net_proof_delta: temporary +1
sunset_id: NORMAL-SCRIPT0-PHYSICAL-PROOF-SUNSET-001
sunset_budget: one disconnected canonical Script physical/candidate proof
retirement_owner: NORMAL-FILE-CANONICAL-CORE0-G0
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: canonical-core production caller equals one, Script uses the sole
family dispatcher/publication terminal, Raw and canonical Script share one
Return writer, and the disconnected Script candidate consumer equals zero
budget_repayment_evidence: canonical-core real-binary parity, route guard,
caller census, and disconnected-consumer zero
Related:
  - normal-script0-physical-entry-d0-design-question-2026-07-26.md
  - normal-file-canonical-core0-dispatch-series-execution-task-2026-07-26.md
  - script-result-tail0-s0-execution-task-2026-07-25.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/tools/check-scripts-index.md
---

# NORMAL-SCRIPT0-PHYSICAL-ENTRY0-S0

## Outcome

This series closes the Script cell of the canonical-core dispatcher without
borrowing Raw invocation lifecycle:

```text
SealedNormalScriptSourceV1
  ↓ one consuming source projection
VerifiedNormalScriptRecipeV1
  - opaque retained Script source owner
  - exact RawScriptBodyRecipeV1 lowering authority
  ↓
OpenScriptPhysicalEntryV1
  - ScriptPhysicalEntrySessionV1
  - exact physical main/0 target
  - no live-Builder commit terminal
  ↓
PreparedScriptPhysicalDraftV1
  - projected final function
  - PreparedScriptBodyCompletionV1
  - VerifiedScriptEntryResultContractV1
  ↓ infallible function-state close
CompletedScriptPhysicalDraftV1
  ↓ one-row canonical transaction
CompletedNormalScriptModuleCandidateV1
  ↓ sole compiler family dispatcher
CompletedCanonicalCoreSourceEntryFamilyV1::Script
```

Publication, VM execution, process projection, and production caller activation
remain later rows.

## Fixed authority split

```text
source result classification:
  RawScriptBodyRecipeV1

physical terminal:
  LoweredScriptTerminalV1

source/physical result relation:
  VerifiedScriptEntryResultContractV1

Script Return/signature writer:
  ScriptPhysicalExitCommitV1

canonical outer lifecycle:
  OpenScriptPhysicalEntryV1
  ScriptPhysicalEntrySessionV1

Raw outer lifecycle:
  existing brand/tracker/witness/ledger owners
  RawScriptPhysicalExitAdapterV1

canonical one-row module transaction:
  NormalScriptModuleTransactionSchemaV1
  PreparedNormalScriptModuleTransactionV1

family dispatch:
  NormalCanonicalCoreSourcePlanCompilerV1
```

Non-authorities:

```text
last Builder ValueId
physical Return scan
VMValue kind
module/symbol inventory
Raw invocation brand
source identity string
runner backend spelling
```

## Mandatory implementation refinements

### Retain the complete source owner

The current recipe handoff drops the parsed Script source after projection.
That conflicts with the dispatch failure law.

The new product keeps:

```rust
struct VerifiedNormalScriptRecipeV1 {
    source: RetainedNormalScriptSourceV1,
    recipe: RawScriptBodyRecipeV1,
}
```

`RetainedNormalScriptSourceV1` has no AST accessor, reclassification terminal,
or retry terminal. It exists only for rejection/candidate evidence. The recipe
is the sole lowering input.

### Use a candidate-only physical session

`CanonicalModuleLoweringSessionV1` can replace the live compiler Builder. That
capability is too broad for a candidate-only Script transaction.

```rust
struct ScriptPhysicalEntrySessionV1 {
    candidate: MirBuilder,
}
```

Allowed terminals:

```text
complete one detached Script draft
discard
```

Forbidden terminals:

```text
commit into live Builder
publish module
return bare mutable Builder
```

The session reuses the existing canonical candidate-construction policy; it
does not duplicate compiler configuration policy.

### Project the Script result once

The shared kernel issues:

```rust
enum VerifiedScriptEntryResultContractV1 {
    Unit {
        origin: RawScriptUnitOriginV1,
        physical: VerifiedScriptUnitPhysicalV1,
    },
    Integer,
    Bool,
    Float,
    String,
}
```

Raw and canonical adapters consume this contract. They may attach family
identity, but may not rematch Unit origin or physical scalar type.

### Name the Raw origin correction

Current Raw execution collapses final `Print`, `Local`, `Assignment`, and
`CompoundAssignment` to `EmptyBody` physical evidence. The shared terminal
keeps their exact recipe origin.

Allowed observable delta:

```text
SourceEntryResult::Unit(origin):
  EmptyBody -> exact statement origin for those four terminal forms
```

Unchanged:

```text
stdout
physical Void return
process status 0
entry target
Raw brand / publication identity
Raw App behavior
```

This is a conformance correction, not a behavior-neutral refactor.

### Share row-set validation

`NormalScriptModuleTransactionSchemaV1` is a separate semantic schema:

```text
row count = 1
role      = PhysicalEntry
key       = Main
symbol    = main
arity     = 0
```

It reuses a shared canonical row-set validator for key/symbol/arity/role
uniqueness and shell correspondence. It does not copy Main schema validation,
make SourceMain optional, or invent a dummy SourceMain row.

## Exact terminal and result law

| Source recipe | Lowered terminal | Physical Return | Signature | Result |
| --- | --- | --- | --- | --- |
| empty | Unit(EmptyBody, SyntheticVoid) | one synthetic Void | Void | Unit |
| value expression | Value(exact operand) | exact operand | exact type | Value |
| void/null expression | Unit(VoidExpression, ExistingVoid) | existing Void | Void | Unit |
| Print | Unit(PrintStatement, SyntheticVoid) | one synthetic Void | Void | Unit |
| Local | Unit(LocalStatement, SyntheticVoid) | one synthetic Void | Void | Unit |
| Assignment | Unit(AssignmentStatement, SyntheticVoid) | one synthetic Void | Void | Unit |
| Compound assignment | Unit(CompoundAssignmentStatement, SyntheticVoid) | one synthetic Void | Void | Unit |

Supported value-expression physical types:

```text
Integer
Bool
Float
String
```

Pre-commit typed rejection:

```text
Unknown
Void classified as ValueExpression
Box / Array / Future / WeakRef
other owner-bearing or dynamic type
undefined operand
preterminated block
```

`null` and `void` currently share `RawScriptUnitOriginV1::VoidExpression`.
Publication must not infer `ExplicitNull`; provenance separation remains a
future capability.

## Buildable commit series

### Commit 1 — `SCRIPT-TERMINAL-KERNEL0`

Purpose:

```text
retain the opaque source owner
store exact RawScriptBodyRecipeV1
return exact LoweredScriptTerminalV1
add typed lowering error
```

Types:

```rust
enum LoweredScriptTerminalV1 {
    Value { value: ValueId },
    Unit {
        origin: RawScriptUnitOriginV1,
        payload: LoweredScriptUnitPayloadV1,
    },
}

enum LoweredScriptUnitPayloadV1 {
    ExistingVoid { value: ValueId },
    SyntheticVoid,
}
```

For buildability, Raw may temporarily project the exact terminal into the old
`RootBodyResultV1`. This private adapter writes no Return and is deleted in
Commit 3.

Focused proof:

```bash
cargo test -q --lib script_terminal_kernel_
cargo check --lib
```

Acceptance:

```text
whole RawRootBodyRecipe retained by canonical Script = 0
opaque source retained                               = 1
terminal origin retained                             = 1
Return writer delta                                  = 0
```

Status: closed on 2026-07-26. The retained source owner is opaque, the
normal handoff now owns only `RawScriptBodyRecipeV1`, and the typed terminal
kernel preserves value versus Unit plus exact Unit origin. Raw uses only a
temporary result-shape adapter; it has not gained a second Return writer.

### Commit 2 — `SCRIPT-PHYSICAL-EXIT-KERNEL0`

Add the brand-free preparation, result contract, completion witnesses, and
sole Script Return/signature writer.

```text
PreparedScriptBodyCompletionV1
CompletedScriptBodyCompletionV1
PreparedScriptPhysicalExitCoreV1
CompletedScriptPhysicalExitCoreV1
VerifiedScriptEntryResultContractV1
ScriptPhysicalExitCommitV1
```

In this same commit, every existing Raw Script exit branch delegates to the
shared writer. `AppVoid` remains in the Raw owner. Do not land a commit in
which an old direct Script writer and the new writer coexist.

This shared kernel is deliberately smaller than the later canonical candidate
transaction: it validates the exact existing operand/type relation, reserves a
synthetic Void ID during preparation, and performs no fallible work in commit.
Full type propagation, metadata projection, and `MirVerifier` ownership remain
with the canonical candidate transaction in Commit 4/5; Raw retains its
existing later verification path. The kernel does not duplicate either owner.

Focused proof:

```bash
cargo test -q --lib script_physical_exit_kernel_
cargo check --lib
```

Acceptance:

```text
Script Return writer                    = 1
Raw Script exit branches delegate       = 1
Raw App exit writer delta               = 0
fallible function mutation after close  = 0
```

Status: closed on 2026-07-26. `PreparedScriptBodyCompletionV1` and
`CompletedScriptBodyCompletionV1` seal the source/physical relation without a
brand; `ScriptPhysicalExitCommitV1` is the one Script Return/signature writer.
Raw Script now delegates to it, while Raw App keeps its own fixed-Void writer.
The legacy unclassified Raw recipe bridge is explicitly temporary and carries
no Return authority; Commit 3 removes its old `RootBodyResultV1` projection.

### Commit 3 — `RAW-SCRIPT-EXIT-ADAPTER0`

Connect the Raw BODY transaction directly to the exact terminal/result core:

```text
LoweredScriptTerminalV1
  -> shared preparation/commit
  -> CompletedScriptPhysicalExitCoreV1
  -> RawScriptPhysicalExitAdapterV1
  -> existing brand-bound completion/witness/lifecycle
```

Delete the temporary `RootBodyResultV1` projection. The Raw adapter writes no
Return and owns no source-result policy.

Focused proof:

```bash
cargo test -q --lib raw_script_exit_adapter_
cargo test -q --lib source_entry_vm_raw_adapter
cargo check --lib
```

Acceptance:

```text
Print/Local/Assignment/Compound origin = exact
EmptyBody collapse for those rows       = 0
Raw Script status/module/target delta    = 0
Raw App delta                            = 0
```

Status: closed on 2026-07-26. Raw no longer projects
`LoweredScriptTerminalV1` into `RootBodyResultV1`. A Raw-only prepared
completion adapter reads the shared prepared completion receipt solely to
satisfy the existing brand-bound tracker before commit; it writes neither a
Return nor a source-result policy. The Raw witness now retains exact synthetic
Unit origins through its decode plan, with focused coverage for EmptyBody,
Print, Local, Assignment, and CompoundAssignment.

### Commit 4 — `NORMAL-SCRIPT0-PHYSICAL0`

Add the compiler-owned outer transaction:

```text
OpenScriptPhysicalEntryV1
ScriptPhysicalEntrySessionV1
PreparedScriptPhysicalDraftV1
CompletedScriptPhysicalDraftV1
RejectedNormalScriptPhysicalEntryV1
```

The candidate-only session lowers the recipe and consumes the shared exit
kernel. It cannot mutate the live compiler Builder and does not build a module
candidate yet.

Focused proof:

```bash
cargo test -q --lib normal_script_physical_
cargo check --lib
```

Acceptance:

```text
canonical Raw brand/tracker/ledger consumer = 0
live Builder mutation on every failure       = 0
detached verified Script draft               = 1
publication                                 = 0
```

Progress: the detached `OpenScriptPhysicalEntrySessionV1` foundation is
landed. It opens only a fresh physical `main/0` with a provisional Unknown
signature and has no live-Builder commit terminal. The remaining C4 work is
to lower the retained recipe through the shared exit core, verify the detached
draft, and retain a typed rejection owner.

### Commit 5 — `NORMAL-SCRIPT0-TX0`

Add:

```text
NormalScriptModuleTransactionSchemaV1
PreparedNormalScriptModuleTransactionV1
CompletedNormalScriptModuleCandidateV1
NormalScriptCandidateVerificationReceiptV1
```

The one-row wrapper reuses shared canonical row-set and shell machinery. All
correspondence and verification completes before the infallible module commit.

Focused proof:

```bash
cargo test -q --lib normal_script_tx_
cargo test -q --lib normal_module_transaction
cargo check --lib
```

Acceptance:

```text
candidate function count = 1
SourceMain row            = 0
Helper row                = 0
PhysicalEntry row         = 1
publication               = 0
```

### Commit 6 — `DISPATCH-SCRIPT0-G0`

Replace only the current Script pending branch:

```text
sealed Script
  -> verified recipe
  -> physical draft
  -> one-row candidate
  -> CompletedCanonicalCoreSourceEntryFamilyV1::Script
```

Callable remains pending. Add Script/Main rejection-reuse coverage and extend
the existing normal-source-plan lane guard through a child helper.

Focused proof:

```bash
cargo test -q --lib canonical_core_dispatch_script_
tools/checks/run_row_guard.sh --only normal-source-plan0
cargo check --lib
```

Acceptance:

```text
Script dispatch success                    = 1
family match remains compiler-owned         = 1
complete source/profile/read-parse retained = 1
production caller                           = 0
publication                                = 0
```

## Failure law

```rust
enum NormalScriptPhysicalEntryStageV1 {
    Recipe,
    Schema,
    FunctionOpen,
    BodyLowering,
    ExitPreparation,
    ProjectedVerification,
    ModuleCorrespondence,
    ModuleShell,
}

enum RetainedNormalScriptPhysicalOwnerV1 {
    Open(OpenScriptPhysicalEntryV1),
    CompletedDraft(CompletedScriptPhysicalDraftV1),
}
```

Every rejection exposes only:

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
try Raw / Main
profile reselection
String-only internal flattening
```

Effects:

| Stage | Allowed effects |
| --- | --- |
| recipe/schema | none |
| function open/body/exit | candidate-only session |
| candidate correspondence/shell | immutable unpublished draft |
| module commit | infallible |
| publication | zero in this series |
| live compiler Builder | unchanged on every failure |

“No fallible work after Return commit” applies to function mutation. Immutable
candidate correspondence and module-shell preparation may still reject while
retaining the completed draft.

## Source layout

Keep orchestration out of files already near the 800-line cap.

```text
src/mir/builder/script_physical_exit/
  mod.rs
  terminal.rs
  completion.rs
  exit_plan.rs
  exit_commit.rs
  error.rs
  tests.rs
  raw_adapter_tests.rs

src/mir/compiler/normal_script_physical_entry/
  mod.rs
  session.rs
  owner.rs
  rejection.rs
  tests.rs

src/mir/builder/normal_script_module_transaction/
  mod.rs
  schema.rs
  transaction.rs
  evidence.rs
  rejection.rs
  tests.rs

src/mir/compiler/
  canonical_core_dispatch_script_tests.rs
```

Existing large files receive only module registration, thin delegation, or
removal:

```text
raw_root_source_facts.rs
raw_root_body_exit.rs
compiler/mod.rs
normal_module_transaction/main_transaction.rs
canonical_core_dispatch.rs
```

No new or modified source/check file may reach 800 lines.

## Guard and fixture plan

Do not add a new shell or manifest row.

```text
existing manifest:
  tools/checks/run_row_guard.sh --only normal-source-plan0

parent guard:
  tools/checks/lib/normal_source_plan0_guard.py

new child helper only:
  tools/checks/lib/normal_source_plan0_script_physical_guard.py
```

The parent is already near 800 lines; it receives one import and one call.
Update the existing Raw BODY guard to require the shared writer plus Raw
adapter instead of pinning `commit_raw_root_exit_v1` as the direct Script
writer.

Fixture prefixes:

```text
script_terminal_kernel_
script_physical_exit_kernel_
raw_script_exit_adapter_
normal_script_physical_
normal_script_tx_
canonical_core_dispatch_script_
```

Origin parity must inspect the published decode plan. Status zero alone cannot
distinguish statement Unit from `EmptyBody`.

## Umbrella verification

```bash
cargo check --lib
cargo check --lib --features vm-reference

cargo test -q --lib script_terminal_kernel_
cargo test -q --lib script_physical_exit_kernel_
cargo test -q --lib raw_script_exit_adapter_
cargo test -q --lib normal_script_physical_
cargo test -q --lib normal_script_tx_
cargo test -q --lib canonical_core_dispatch_script_
cargo test -q --lib source_entry_vm_raw_adapter

tools/checks/run_row_guard.sh --only normal-source-plan0
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_body0_s0_guard.py
python3 tools/checks/lib/entry_result_projection0_s3_execution_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

At the Raw writer cutover milestone, run the existing enabled/disabled
real-binary Raw VM-reference conformance once. Do not add another conformance
runner.

## Stop conditions

Stop and reopen design only if implementation evidence requires any of:

```text
canonical Raw brand/tracker/ledger
live Builder commit from the Script session
two Script result/decode projections
Raw App behavior change
Raw observable delta beyond the four corrected Unit origins
dummy FunctionOwnerId or SourceMain
Script Return emission outside the shared writer
AST rewrite/reclassification
```

Ordinary typed capability rejection is not a design stop.

## Immediate publication follow-up

After `DISPATCH-SCRIPT0-G0`:

```text
NORMAL-SCRIPT0-PUBLICATION0-S0
  -> CANONICAL-PUBLISHED-MODULE0
  -> NORMAL-SCRIPT0-PUBLISHED-OWNER0-S0
  -> NORMAL-SCRIPT0-VMREF0-P0
```

This follow-up adds only:

```text
one canonical-family opaque published module/exact interpreter terminal
PublishedNormalScriptInvocationV1
PublishedSourceEntryMembershipV1::CanonicalScript
VmReferencePublishedOwnerV1::CanonicalScript
one Script adapter to the existing neutral VM owner
```

It does not add an executor, decode policy, or process policy. The existing
generic chain remains:

```text
PublishedSourceEntryInvocationV1<O>
  -> PreparedVmReferenceSourceEntryInvocationV1<O>
  -> exact VM execution
  -> ProcessExitProjectionV1
```

## Task order through canonical-core completion

```text
NORMAL-SCRIPT0-PHYSICAL-ENTRY0-S0
  SCRIPT-TERMINAL-KERNEL0
  -> SCRIPT-PHYSICAL-EXIT-KERNEL0
  -> RAW-SCRIPT-EXIT-ADAPTER0
  -> NORMAL-SCRIPT0-PHYSICAL0
  -> NORMAL-SCRIPT0-TX0
  -> DISPATCH-SCRIPT0-G0

-> NORMAL-SCRIPT0-PUBLICATION0-S0

-> NORMAL-CALLABLE0-PUBLICATION-EVIDENCE0-S0
-> NORMAL-CALLABLE0-PUBLICATION0-S0

-> CANONICAL-CORE-PUBLICATION-DISPATCH0-S0
-> CANONICAL-CORE-FIXTURE-G0

-> NORMAL-FILE-CANONICAL-CORE0-REQUEST0-S0
-> NORMAL-FILE-CANONICAL-CORE0-REPORT0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
-> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
-> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0b
-> NORMAL-FILE-CANONICAL-CORE0-G0

-> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
```

Then resume the already accepted ownership lane before product/default
promotion:

```text
OWNERSHIP-SPARSE-RESUME-D0
-> Pack A: syntax safety/evidence
-> Pack B: passive ownership grammar/Loan Flow
-> Pack C: ScopedAlias
-> Pack D: callable ownership ABI
-> Pack E: Anchored View
-> OWNERSHIP-SPARSE-PRODUCT-READINESS-D0

-> NORMAL-ENTRY-PRODUCT-BACKEND-D0
-> NORMAL-DEFAULT-CALLER-CENSUS0-P0
-> NORMAL-ENTRY-PROMOTION-D3
```

## Structural gate

```text
RawScriptBodyRecipeV1 result authority               = 1
normal-only Script tail classifier                   = 0
opaque retained Script source owner                  = 1

LoweredScriptTerminalV1 producer                     = 1
VerifiedScriptEntryResultContractV1 producer         = 1
Prepared/Completed Script completion producer        = 1 each

ScriptPhysicalExitCommitV1 Return writer             = 1
Raw Script direct Return writer                      = 0
canonical Script direct Return writer                = 0
Raw Script shared-kernel consumer                    = 1
canonical Script shared-kernel consumer              = 1

canonical RootBodyResult/tracker/completion consumer = 0
canonical ModuleInvocationBrand consumer             = 0
canonical Raw witness/physical-state consumer        = 0
canonical Raw publication/request consumer           = 0

Script schema PhysicalEntry row                      = 1
Script schema SourceMain/Helper row                  = 0
dummy FunctionOwnerId                                = 0
Main-to-main thunk                                   = 0

candidate function count                             = 1
candidate verification before module commit          = 1
partial publication                                  = 0
production caller                                    = 0

AST clone/rewrite/reclassification                   = 0
module/symbol result inference                       = 0
fallback/retry/profile reselection                   = 0

new VM executor/decode/process owner                 = 0
default route delta                                  = 0
all modified/new source/check files                  < 800 lines
```

## Non-claims

```text
canonical-core production CLI caller
default backend or compile_with_source cutover
Callable dispatch/publication implementation in this row

new Script grammar
nested control / Return / cleanup at Script root
catch / RecoverableFailure

Null versus Void provenance split
dynamic/owner-bearing result carrier

Raw App redesign
Raw publication or old-chain retirement

LLVM/native/ny_main
JSON/REPL/Stage1
executor/selfhost/fastmem
CUT0
```
