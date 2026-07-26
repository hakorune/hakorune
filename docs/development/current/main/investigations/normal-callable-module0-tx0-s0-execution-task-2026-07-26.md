---
Status: closed; Decision B accepted
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Failure decision: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1
Row: NORMAL-CALLABLE-MODULE0-TX0-S0
Scope: prepare and commit one atomic canonical normal module containing source Main, all helper drafts, and one physical main thunk
ceremony_tier: T1 bounded composition and activation of existing function-draft, helper-topology, normal-schema, shell-drain, and thunk owners
series_mode: BoxShape only; accepted source/result shapes do not grow
proof_inventory_before: closed Main-only transaction, closed Acyclic/Recursive normal topology selector, closed passive heterogeneous normal-module schema
new_proofs: one common callable-module transaction and one complete-candidate verification receipt
retired_or_merged_proofs: variant-specific transaction selection is forbidden; Main-only and helper-only preparation become internal reusable steps
net_proof_delta: one temporary activation proof
sunset_budget: repaid when the canonical-core production profile consumes the completed candidate through the existing neutral VM-reference terminal
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: canonical-core caller equals one, fallback equals zero, and no Acyclic/Recursive-specific transaction terminal exists
Related:
  - docs/development/current/main/investigations/normal-callable-module0-r0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-module-tx0-l0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/hakorune-sparse-ownership-surface-task-2026-07-15.md
  - src/mir/compiler/normal_source_plan/normal_acyclic_module_plan.rs
  - src/mir/builder/normal_module_transaction/
---

# NORMAL-CALLABLE-MODULE0-TX0-S0

## Accepted failure boundary

The current lowering terminal consumes
`CanonicalTrivialBindingSsaPlanV1` and discards its unpublished function
session on body-lowering failure. Draft-seal rejection is also flattened and
discarded by the current facade.

The accepted decision does not require rollback of that single-use
operational capability. TX0 retains:

```text
RetainedNormalCallableSourceAuthorityV1
+ exact owned topology receipt
+ exact successfully prepared draft prefix
+ ConsumedNormalLoweringCapabilityReceiptV1
+ typed lowering stage/cause
+ Builder restoration receipt
```

It does not recreate or expose:

```text
consumed CanonicalTrivialBindingSsaPlanV1
resume/retry capability
restorable unpublished Builder session
```

The existing `lower_resolved_trivial_function_draft` remains a
String-compatible facade. A new TX0-private typed terminal records the stage
at the failure site and issues a restoration receipt only after the
unpublished function session has been closed or discarded.

## Outcome

```text
CompletedNormalMainHelperResolutionV1
  ↓ consume into one open transaction
OpenNormalCallableModuleTransactionV1
  ↓ borrow-only topology and lowering preparation
PreparedNormalCallableModuleTransactionV1
  - retained source/catalog authority
  - Acyclic or Recursive topology receipt
  - source Main draft
  - canonical-key ordered helper drafts
  - physical main thunk
  - exact heterogeneous schema
  - recursive capability plan when required
  - projected complete-candidate verification
  - prepared empty-shell drain
  ↓ one infallible commit
CompletedNormalCallableModuleCandidateV1
  ↓ explicit publication
PublishedNormalCallableModuleInvocationV1
```

The sole transaction owns:

```text
FunctionDraftKeyV1::CanonicalResolvedOwner(main_owner)
+ FunctionDraftKeyV1::CanonicalCallable(helper_key)*
+ FunctionDraftKeyV1::Main
```

No second compiler, collector, module transaction, or entry selector is added.

## Critical ownership law

`VerifiedNormalHelperTopologyPlanV1` borrows
`CompletedNormalMainHelperResolutionV1`. It must not be moved beside that owner
inside a self-referential product.

The safe sequence is:

```text
own CompletedNormalMainHelperResolutionV1
-> borrow prepare_topology_plan()
-> lower helper plans to owned unpublished drafts
-> end topology-plan borrow
-> borrow/reissue exact Main lowering view
-> lower source Main to an owned unpublished draft
-> prepare physical thunk
-> prepare schema, marker, correspondence, and full verification
-> issue PreparedNormalCallableModuleTransactionV1
```

The durable Program/catalog/source-site authority stays owned by the outer
transaction throughout fallible preparation. Ephemeral lowering plans are
single-use operational capabilities. Their consumption is recorded, not
reversed. The source, catalog, Main identity, selected family, and topology
are not reconstructed.

Forbidden:

```text
store owner and borrowed topology plan together
move owner after issuing a surviving borrowed plan
clone AST or Program
rebuild a helper catalog or call graph
try Acyclic then Recursive
retry another source family/profile
```

The implementation must split durable authority from operational plans through
one consuming TX0 handoff. It must not keep an owner and a plan borrowing that
owner in one self-referential product.

## Reused authorities

```text
source/Main/helper semantics:
  CompletedNormalMainHelperResolutionV1

topology:
  VerifiedNormalHelperTopologyPlanV1

function exit and physical draft seal:
  existing CanonicalTrivialBindingSsaPlanV1
  existing PreparedFunctionDraftSealV1

heterogeneous batch schema:
  NormalModuleTransactionSchemaV1

source Main draft:
  VerifiedNormalMainSourceDraftV1

physical entry thunk:
  VerifiedNormalMainPhysicalThunkDraftV1

candidate shell:
  ModuleLoweringShellV1::from_empty_module
  ModuleLoweringShellDrainInventoryV1
  PreparedModuleLoweringShellDrainV1

final process execution:
  existing PublishedSourceEntryInvocationV1
  existing exact VM-reference execution and ProcessExitProjectionV1
```

Do not call `MirModule::try_add_functions_atomic` directly from the new owner.
Do not reuse a Raw root batch or create a Raw invocation brand.

## Prepared topology receipt

The borrowed topology plan is projected once into an owned transaction receipt:

```rust
enum PreparedNormalHelperTopologyReceiptV1 {
    Acyclic {
        helper_count: usize,
        edge_count: usize,
    },
    Recursive {
        helper_count: usize,
        component_count: usize,
        recursive_component_count: usize,
        capability: PreparedNormalRecursiveCapabilityV1,
    },
}
```

The exact fields may use existing typed receipts. The laws are fixed:

```text
selected once
same helper cardinality as schema and drafts
same deterministic helper order
recursive marker present iff Recursive
no source/body re-observation
no symbol-derived topology inference
```

Recursive capability preparation must be derived from the verified SCC
partition. Direct metadata insertion, a fake invocation brand, or reuse of a
Legacy/Raw capability path is forbidden.

## Draft preparation

### Helpers

Reuse the existing callable draft-lowering implementation through a retaining
normal facade:

```text
topology helper plans in canonical-key order
-> one unpublished verified helper draft per key
-> retained exact prefix on failure
```

The facade must not re-run helper preflight or rebuild an existing historical
`VerifiedAcyclicCallableModulePlanV1` /
`VerifiedRecursiveCallableModulePlanV1`.

### Source Main

Reissue one scoped Main lowering view from the retained semantic evidence.
Reuse:

```text
lower_resolved_trivial_function_draft
-> F1 draft seal
-> VerifiedNormalMainSourceDraftV1
```

Main remains outside the helper catalog and SCC partition.

### Physical thunk

Reuse the existing normal physical thunk writer. Its target, result, and entry
identity come only from sealed Main/header/result evidence.

```text
module inventory scan = 0
NYASH_ENTRY            = 0
symbol inference       = 0
```

## Full candidate preparation

Before commit, verify:

```text
schema rows =
  one source Main
  + every helper exactly once
  + one physical entry

draft order =
  source Main
  + helpers by canonical key
  + physical entry

correspondence =
  exact key
  exact symbol
  exact arity
  exact result type
  exact Main call targets
  exact helper call rows
  exact entry relation

recursive capability =
  present exactly for Recursive

verification =
  every function verified
  complete candidate module verified
```

All fallible work finishes before commit begins.

## Commit

```rust
impl PreparedNormalCallableModuleTransactionV1 {
    fn commit(self) -> CompletedNormalCallableModuleCandidateV1 {
        // no Result, lookup, inference, verification, fallback, or retry
    }
}
```

Commit:

```text
1. apply the already-prepared recursive marker plan when present
2. move source Main, ordered helpers, and physical thunk into one Vec
3. commit_preflighted into the prepared empty shell
4. issue CompletedNormalCallableModuleCandidateV1
```

Module publication before step 3 is zero.

## Failure retention

```rust
struct RejectedNormalCallableModuleTransactionV1 {
    source: RetainedNormalCallableSourceAuthorityV1,
    stage: NormalCallableModuleTransactionStageV1,
    error: NormalCallableModuleTransactionErrorV1,
    prepared: RetainedNormalCallableDraftPrefixV1,
    consumed: Option<ConsumedNormalLoweringCapabilityReceiptV1>,
    restoration: NormalCallableBuilderRestorationReceiptV1,
}
```

Stages:

```text
TopologyReceipt
HelperDrafts
MainDraft
PhysicalThunk
Schema
RecursiveCapability
Correspondence
CandidateVerification
Shell
```

The retained prefix may contain:

```text
no drafts
ordered helper drafts
helpers plus source Main
helpers plus source Main plus physical thunk
```

The exact Program/catalog/source identity and selected topology authority
remain present on every failure. A consumed lowering plan is represented only
by a non-resumable receipt. Inspection plus `discard(self)` are the only
exits.

Forbidden:

```text
into_owner
resume
retry
Legacy fallback
partial module publication
drop prepared helper prefix before reporting
String-only internal error flattening
consumed lowering plan reconstruction
```

## Buildable commit series

```text
TX0-A HANDOFF0
  split durable source authority from consumable operational proofs
  OpenNormalCallableModuleTransactionV1
  one owned topology receipt and canonical-key helper schedule
  no Builder behavior delta

TX0-B DRAFTS0
  typed retaining function-draft lowering terminal
  existing String-compatible facade unchanged
  retaining common helper draft facade
  source Main draft
  physical thunk
  exact prepared-prefix rejection

TX0-C BATCH0
  heterogeneous schema
  recursive marker preparation
  key/symbol/arity/result/call/entry correspondence
  projected full-candidate verification

TX0-D COMMIT0
  one infallible shell drain
  CompletedNormalCallableModuleCandidateV1
  PublishedNormalCallableModuleInvocationV1

TX0-E G0
  focused success/failure/reorder/reuse fixtures
  existing normal-source-plan transaction guard extension
  docs/current closeout
```

Each commit must build. Do not add a new per-row shell guard.

## File layout

Do not grow `main_transaction.rs` beyond its current Main-only role.

```text
src/mir/builder/normal_module_transaction/
  normal_callable_handoff.rs       # about 200 lines
  normal_callable_drafts.rs        # about 350 lines
  normal_callable_transaction.rs   # about 450 lines
  normal_callable_rejection.rs     # about 220 lines
  normal_callable_success_tests.rs # below 500 lines
  normal_callable_failure_tests.rs # below 500 lines
```

All source/check files remain below 800 lines.

## Fixture matrix

Success:

```text
Main + one call-free helper
Main + independent helpers
Main + finite helper DAG
Main -> helper
Main -> recursive helper
self-recursive helper
mutual-recursive helpers
recursive SCC + independent leaf
declaration reorder -> same module keys/symbols/entry/result
```

Failure:

```text
late helper draft failure
source Main draft failure
physical thunk failure
duplicate helper key/symbol
schema cardinality drift
call-target correspondence drift
recursive marker absent/unexpected
candidate verification failure
shell preparation failure
```

Reuse:

```text
acyclic success -> recursive success
success -> late helper failure -> success
success -> Main failure -> success
success -> thunk failure -> success
```

Every failure proves:

```text
live Builder replacement   = 0
partial module publication = 0
owner loss                 = 0
fallback/retry             = 0
```

## Structural gate

```text
normal callable transaction terminal                 = 1
Acyclic-specific transaction terminal                 = 0
Recursive-specific transaction terminal               = 0

original Program owner                                = 1
helper catalog/index                                  = 1
graph inventory                                      = 1
SCC partition                                        = 1
source Main draft                                    = 1
physical main thunk                                  = 1

normal heterogeneous schema                           = existing 1
module shell drain authority                          = existing 1
infallible commit                                     = 1
Result after commit begins                            = 0

Raw batch/brand                                       = 0
NYASH_ENTRY/module scan                               = 0
AST/source rewrite                                    = 0
retry/fallback                                        = 0
partial publication                                   = 0

VM executor/process projection duplication            = 0
normal/default/CLI caller delta                        = 0
all modified/new source/check files                    < 800 lines
```

## Immediate continuation

TX0 closes the canonical candidate. The VM lane is shared, not forked:

```text
NORMAL-CALLABLE-MODULE0-TX0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
-> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
-> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0b
-> NORMAL-FILE-CANONICAL-CORE0-G0
-> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
```

The new profile consumes:

```text
PublishedNormalCallableModuleInvocationV1
-> existing neutral PublishedSourceEntryInvocationV1
-> exact fresh MirInterpreter execution
-> SourceEntryResultV1
-> ProcessExitProjectionV1
```

No second VM executor, result decoder, diagnostic adapter, or process-status
owner is allowed.

## Product/default and retirement order

The VM-reference lane remains the semantic reference lane until a separate
product-backend decision. Green reference parity does not silently make it the
default product backend.

```text
MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
-> OWNERSHIP-SPARSE-RESUME-D0
-> OWN-GRAM-REJECT0
-> ownership evidence / passive grammar / Loan Flow
-> UBOX-I0
-> ALIAS-I0 / ALIAS-CFG0
-> ABI0 / UCALL-B0
-> VIEW0 / PROJ-S0..PROJ-I0
-> OWNERSHIP-SPARSE-PRODUCT-READINESS-D0

-> NORMAL-ENTRY-PRODUCT-BACKEND-D0
   recommended first candidate: the maintained MIR interpreter product lane
   required evidence: source/result/status/diagnostic parity and performance
-> NORMAL-DEFAULT-CALLER-CENSUS0-P0
-> NORMAL-ENTRY-PROMOTION-D3
-> exactly one named default caller cutover
-> exact replaced caller retirement

-> NORMAL-IMPORT-BUNDLE-D0
-> NORMAL-IMPORT-BUNDLE0-S0
-> NORMAL-FILE-IMPORT0-CUTOVER0

-> MIRBUILDER-LEGACY-FENCE0-S0
-> MIRBUILDER-NORMAL-CALLER-CENSUS0-P0
-> MIRBUILDER-NORMAL-COMPLETE0-P0
-> MIRBUILDER-COMPLETE0-G0
```

Retirement law:

```text
reference lane:
  retained while it is the semantic oracle/conformance lane
  removed only by a named retirement decision with consumer zero

selected old default caller:
  retired immediately after its exact replacement is green

compile_with_source/direct build_module:
  fenced only after every remaining caller is typed canonical or named Legacy

fallback:
  always zero
```

The ownership sequence is not part of TX0 and does not block
`MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0`. It is the selected bridge between
canonical-core completion and product/default promotion. Its detailed
authority is:

```text
docs/development/current/main/investigations/
  hakorune-sparse-ownership-surface-task-2026-07-15.md
```

JSON, REPL, Stage1, WASM, LLVM/AOT, executor, selfhost, and fastmem remain
post-core integration migrations. They do not block the canonical MirBuilder
core completion declaration.

## Acceptance

```bash
cargo check --lib --features vm-reference
cargo test -q --lib mir::builder::normal_module_transaction --features vm-reference
cargo test -q --lib normal_source_plan --features vm-reference
python3 tools/checks/lib/normal_source_plan0_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-claims

```text
new source/result shape
String/Box/object/dynamic callable result
Main-box helper methods
instance methods/receiver
nested/multiple/all-path Return
cleanup activation
imports/using
production/default caller
product backend selection
Legacy caller retirement
JSON/REPL/Stage1/WASM
LLVM/native/ny_main
executor/selfhost/fastmem
CUT0
```
