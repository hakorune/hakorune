---
Status: active executable row
Date: 2026-07-26
Decision: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1
Row: NORMAL-CALLABLE-MODULE0-TX0-HANDOFF0-S0
Parent: NORMAL-CALLABLE-MODULE0-TX0-S0
Scope: split durable normal callable source authority from single-use lowering capabilities and issue one owned topology receipt plus scoped helper schedule
ceremony_tier: T1 bounded owner/evidence refactor
series_mode: BoxShape only; accepted source/result shapes do not grow
proof_inventory_before: one completed Main/helper semantic owner and one borrowed Acyclic-or-Recursive topology plan
new_proofs: one durable source-authority owner, one owned topology receipt, and one consuming helper-lowering schedule
retired_or_merged_proofs: no independent Acyclic/Recursive transaction handoff
net_proof_delta: one temporary TX0 handoff proof
sunset_id: NORMAL-CALLABLE-MODULE0-TX0-HANDOFF-PROOF-SUNSET-001
sunset_budget: repaid when the canonical-core production profile consumes one completed normal callable candidate
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: canonical-core caller equals one, fallback equals zero, and no proof-only TX0 handoff consumer remains
Related:
  - docs/development/current/main/investigations/normal-callable-module0-tx0-draft-failure-d0-design-question-2026-07-26.md
  - docs/development/current/main/investigations/normal-callable-module0-tx0-s0-execution-task-2026-07-26.md
  - src/mir/compiler/normal_source_plan/normal_acyclic_module_plan.rs
  - src/mir/builder/normal_module_transaction/
---

# NORMAL-CALLABLE-MODULE0-TX0-HANDOFF0-S0

## Outcome

Create the only TX0 handoff:

```text
CompletedNormalMainHelperResolutionV1
  ↓ one consuming split
OpenNormalCallableModuleTransactionV1
  - RetainedNormalCallableSourceAuthorityV1
  - ConsumableNormalMainLoweringProofV1
  ↓ one scoped visitor
PreparedNormalHelperTopologyReceiptV1
+ OwnedNormalHelperLoweringScheduleV1
```

This row is Builder-free, MIR-free, backend-free, and caller-free. It fixes
ownership and schedule authority before draft lowering begins.

## Authority split

### Durable authority

`RetainedNormalCallableSourceAuthorityV1` owns the exact semantic facts that
must survive every later TX0 rejection:

```text
one original Program/catalog source owner
exact Main box and Main.main/0 sites
exact helper declaration sites and complete helper index
same compilation brand and resolver/source identities
sealed Main result/entry facts
```

It may expose bounded inspection needed by TX0 verification. It must not expose:

```text
bare AST or Program
source text
reclassification
catalog rebuild
resolver restart
profile retry
```

The exact split is:

```rust
struct RetainedNormalCallableSourceAuthorityV1 {
    helpers: VerifiedResolvedCallableModuleV1,
    main: RetainedNormalMainSourceAuthorityV1,
}

struct RetainedNormalMainSourceAuthorityV1 {
    identity: NormalSourceIdentityV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    role: VerifiedNormalMainRoleV1,
    owner: FunctionOwnerIdV1,
}

struct ConsumableNormalMainLoweringProofV1 {
    if_control: VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
}
```

`VerifiedResolvedCallableModuleV1` remains whole. It already owns the exact
Program/catalog and canonical-keyed helper forest/projection map. Do not split
or rebuild it.

### Single-use operational capability

`OwnedNormalHelperLoweringScheduleV1` owns the already prepared helper plans in
canonical-key order only for the scoped visitor:

```rust
struct OwnedNormalHelperLoweringScheduleV1<'source> {
    topology: PreparedNormalHelperTopologyReceiptV1,
    plans: BTreeMap<
        CanonicalCallableKeyV1,
        CanonicalTrivialBindingSsaPlanV1<'source>,
    >,
}
```

The transaction must not store this schedule beside the source authority after
the visitor returns. That would create a self-reference consisting of an owned
source authority and plans borrowing it.

The safe implementation shape is:

```text
own completed Main/helper resolution
-> borrow once to prepare topology and helper plans
-> project an owned topology receipt
-> consume or discard every plan in that borrow scope during DRAFTS0
-> close every plan borrow
-> move durable authority into success or rejection owner
```

If Rust cannot express an owned long-lived schedule without self-reference,
keep the schedule as a closure-scoped visitor. Do not clone the Program or
reconstruct plans to force a stored product.

## Topology receipt

```rust
enum PreparedNormalHelperTopologyReceiptV1 {
    Acyclic {
        graph: VerifiedAcyclicCallableGraphV1,
    },
    Recursive {
        partition: VerifiedCallableSccPartitionV1,
        capability: PreparedNormalRecursiveCapabilityV1,
    },
}
```

The receipt owns the full existing verified graph or SCC partition, not only
counts. Counts are projections for diagnostics and cardinality checks. The
laws are fixed:

```text
inventory producer                   = existing one
SCC partition producer               = existing one
Acyclic/Recursive selection           = exactly once
helper order                          = canonical key order
second sort                           = 0
Acyclic-to-Recursive retry            = 0
source/body re-observation            = 0
symbol-derived topology inference     = 0
```

Recursive capability may be prepared here only when it is a pure projection
of the already verified SCC partition and has no Builder/module effects.

## Consuming terminal

Add one narrow terminal on the completed source owner. The exact spelling may
follow module visibility, but its authority is:

```rust
fn prepare_normal_callable_transaction_v1<R>(
    self,
    consume: impl for<'source> FnOnce(
        &'source RetainedNormalCallableSourceAuthorityV1,
        OwnedNormalHelperLoweringScheduleV1<'source>,
    ) -> R,
) -> R;
```

A callback/visitor form is preferred when it prevents a self-referential
owner. A method returning a product is acceptable only if the product is not
self-referential and does not widen AST/catalog access.

The compiler-side scoped API is:

```rust
fn with_helper_plans<R>(
    &self,
    use_once: impl for<'source> FnOnce(
        PreparedNormalHelperTopologyReceiptV1,
        BTreeMap<
            CanonicalCallableKeyV1,
            CanonicalTrivialBindingSsaPlanV1<'source>,
        >,
    ) -> R,
) -> Result<R, NormalAcyclicCallableModuleErrorV1>;
```

Every helper plan is consumed or dropped before the callback returns. Remove
the `owner: &CompletedNormalMainHelperResolutionV1` field from the topology
plan product; the owned receipt and scoped plans are sufficient.

After helper lowering ends, take the Main proof exactly once and bind it to a
fresh borrowed lowering input projected from the retained source:

```rust
struct DetachedTrivialBindingSsaProofV1 {
    if_control: VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
}

impl DetachedTrivialBindingSsaProofV1 {
    fn bind<'source>(
        self,
        input: ResolvedFunctionLoweringInputV1<'source>,
    ) -> CanonicalTrivialBindingSsaPlanV1<'source>;
}
```

This is recomposition of already verified parts. It is not a second preflight,
source reclassification, or profile selection.

## Handoff rejection

Preparation before any lowering may fail at:

```text
TopologyReceipt
HelperSchedule
MainEvidence
AuthorityCorrespondence
```

The rejection owns the durable source authority and typed cause:

```rust
struct RejectedNormalCallableHandoffV1 {
    source: RetainedNormalCallableSourceAuthorityV1,
    stage: NormalCallableHandoffStageV1,
    error: NormalCallableHandoffErrorV1,
}
```

Only:

```text
stage()
error()
discard(self)
```

are allowed. There is no `into_source`, retry, resume, or alternate profile
terminal.

## File layout

Keep source/check files below 800 lines and do not grow the 593-line
Main-only transaction:

```text
src/mir/compiler/normal_source_plan/
  normal_callable_transaction_handoff.rs

src/mir/builder/normal_module_transaction/
  normal_callable_handoff.rs
```

Prefer compiler-side ownership projection in the first file and
Builder-facing opaque handoff vocabulary in the second. Do not move lowering
logic into the compiler classifier.

## Focused fixtures

```text
one helper:
  Acyclic receipt
  one canonical-key schedule row

finite DAG:
  declaration reorder -> same canonical-key schedule and receipt

recursive SCC plus leaf:
  Recursive receipt
  exact component counts
  one selected recursive capability

profile/topology rejection:
  exact source authority retained
  lowering callback not entered

success -> rejection -> success:
  same semantic classifier remains reusable
```

## Acceptance

```text
durable source authority owner                    = 1
consuming helper schedule                         = 1
owned topology receipt                            = 1

stored owner-plus-borrowed-plan self-reference    = 0
AST/Program clone                                 = 0
source rewrite                                    = 0
second catalog/resolver/inventory/partition       = 0
retry/reclassification/fallback                   = 0

Builder/MIR/module/publication reference           = 0
production/default/CLI caller delta                = 0
existing VM0 route delta                          = 0
all modified/new source/check files               < 800 lines
```

Run:

```bash
cargo check --lib --features vm-reference
cargo test -q --lib normal_source_plan --features vm-reference
python3 tools/checks/lib/normal_source_plan0_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Do not add a new per-row shell guard.

## Fixed continuation

After HANDOFF0 closes, continue without another design stop unless the exact
source authority cannot survive the callback scope:

```text
NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-S0
  typed retaining lowerer terminal
  exact canonical-key helper draft prefix
  source Main and physical thunk preparation

-> NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0
  heterogeneous schema
  recursive capability correspondence
  full candidate verification

-> NORMAL-CALLABLE-MODULE0-TX0-COMMIT0-S0
  one infallible empty-shell drain
  completed candidate

-> NORMAL-CALLABLE-MODULE0-TX0-G0
  failure/reuse/atomicity fixtures
  existing transaction guard extension
  current closeout
```

### DRAFTS0 exact contract

Add:

```rust
enum NormalFunctionDraftLoweringStageV1 {
    SessionOpen,
    BindingInstall,
    Skeleton,
    BodyLowering,
    DraftSeal(FunctionDraftSealStageV1),
    SessionRestore,
}

struct RejectedNormalFunctionDraftLoweringV1 {
    stage: NormalFunctionDraftLoweringStageV1,
    cause: NormalFunctionDraftLoweringCauseV1,
    restoration: NormalFunctionDraftBuilderRestorationReceiptV1,
}
```

The outer stage is issued at the failure site, never parsed from a String.
Bounded legacy detail may remain inside the nested cause. The existing
`lower_resolved_trivial_function_draft` becomes a compatibility facade over
the typed retaining terminal.

Retain:

```text
helper k failure:
  exact drafts [0..k)
  failed key/ordinal/stage
  no later helper/Main/thunk attempt

Main failure:
  every helper draft
  exact Main consumed-operation receipt
  no thunk attempt

later failure:
  helpers + Main + physical thunk when prepared
```

### BATCH0 exact contract

All schema, recursive marker, key/symbol/arity/result/call/entry
correspondence, full-function verification, and shell preparation are
fallible preparation. Every failure retains the exact prepared prefix and
publishes zero functions.

### COMMIT0 exact contract

```rust
impl PreparedNormalCallableModuleTransactionV1 {
    fn commit(self) -> CompletedNormalCallableModuleCandidateV1;
}
```

No `Result`, lookup, inference, verification, allocation decision, fallback,
or retry occurs after commit begins.

### G0 minimum failure/reuse proof

```text
helper failure retains exact canonical prefix
draft-seal failure retains typed stage without String parsing
Main failure retains every helper draft
thunk/batch failure retains exact prepared state
every failure restores Builder and permits later success
partial module publication = 0
```

Extend the existing
`tools/checks/lib/normal_source_plan0_transaction_guard.py`; do not add a new
shell wrapper and do not grow the 767-line parent source-plan guard.

## Non-claims

```text
new source/result capability
String/object/dynamic callable result
Main-box helper methods
instance methods/receiver
nested/multiple/all-path Return
cleanup
imports/using
VM/runner/profile activation
product/default backend selection
Legacy retirement
repository-wide lowerer error rewrite
```
