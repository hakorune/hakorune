# RAW-SOURCE0 LOWER ROOT0 — BODY0-S0 execution task

Status: **In progress — disconnected BODY0 owner; production consumers remain zero**  
Date: 2026-07-24  
Decision: **BODY-prime-r1**

`DECLACCESS0-S0` is closed in `e2fb0839f5`. The design consultation is
closed with `BODY-prime-r1`; this card is the first executable Raw root-body
slice. It may create a disconnected owner and exact proof products, but it
must not wire a public ingress or any production executor.

## Progress

S0-A is implemented in `3648bad79f`: the shared neutral recipe contract,
LinearScalar0 conversion, App Main metadata seal, duplicate-site validation,
and Return/ScopeBox source-path repair are green. A false duplicate in
expression-statement provenance was corrected in `ca38ab81b`. The fresh
tracker `begin_root_body -> seal_root_body` typestate is implemented in
`589fa501df`. S0-B now has a disconnected AST-free value lowerer and fixtures
for literal/binary and local/assignment/print recipes; it does not open a
physical owner or publish a draft. S0-C (consuming BODY0 owner) remains
unimplemented and production consumers remain zero.

## Decision lock

```text
Q1  DeclaredRawRootInvocationV1::begin_body(self) is the only BODY0 entry.
    The active owner remains private; the consuming terminal returns success
    or a discard-only rejection.

Q2  RawRootBodyRecipeV1 is exact, non-Clone, and compiler/Builder neutral.
    It is produced once with the manifest and consumed directly by Builder.
    AST lookup, AST reconstruction, classifier rerun, and current_module
    access are forbidden.

Q3  InstalledRawRootEnvironmentV1 owns the paired session/physical terminal.
    RawRootPhysicalStateV1 exposes only private tracker transitions. BODY0
    creates an unpublished main/0 draft and CompletedRootBody witness; it does
    not borrow or mutate the collector/ledger root batch.

Q4  The first grammar is LinearScalar0. If, Loop, LoopRange, Return, Break,
    Continue, ScopeBox, And, and Or reject at recipe/eligibility time.
    App Main requires arity zero, no return type, no uses, empty attrs, and
    empty contracts.

Q5  Fresh tracker -> begin_root_body -> recipe lowering -> cleanup/restore
    -> seal_root_body -> CompletedRootBodyV1 is one-way. Callable-Main
    disposition and receipt are retained but never reselected or recounted.

Q6  Preflight failure retains the unchanged Declared owner. Post-lowering
    failure retains the mutated-but-unpublished paired owner, successful
    prefix, failing site, and exact typed nested cause. Collector admission is
    excluded from BODY0 and remains ROOTBATCH0 responsibility.

Q7  Success returns route-specific RawRootBodyCompleteInvocationV1. Its only
    next terminal is ROOTBATCH0. main/0, condition_fn/1, collector, and root
    ledger remain unpublished/unreserved.
```

## Ownership chain

```text
DeclaredRawRootInvocationV1
  -> begin_body(self)
  -> private ActiveRawRootBodyInvocationV1
       owns exact recipe, paired session/physical owner, and fresh tracker
  -> RawRootBodyCompleteInvocationV1::{Script, App}
       owns unpublished main/0 draft + CompletedRootBodyV1
  -> ROOTBATCH0 later
```

No `ActiveRawRootBodyInvocationV1` is returned to callers. No loose
`(session, shell, collector, ledger, tracker)` tuple is exposed.

## S0-A — recipe boundary

Add a neutral contract under `src/mir/`, for example
`raw_root_body_recipe.rs`. It must own:

```text
Script/App entry contract
LinearScalar0 statements and expressions
literal payloads, variable names, operators
stable source path and span for every node
App Main metadata seal (arity=0, return=None, uses/attrs/contracts empty)
```

The recipe must not contain `ASTNode`, `OwnedRawSourceV1`, compiler objects,
Builder references, `ValueId`, `BasicBlockId`, collector identity, or ledger
identity. `RawRootPostInstallFactsV1` is consumed once to produce this recipe;
BODY0 does not reclassify or rescan source.

Repair source-site construction before claiming exact provenance:

```text
Return.value  = parent path + [0]
ScopeBox.body = parent path + [0] + nested statement index
```

Seal duplicate-path absence, parent-prefix correspondence, and span
correspondence. Add fixtures for nested Return/ScopeBox rejection in S0; they
are not admitted by LinearScalar0.

## S0-B — disconnected LinearScalar0 lowerer

Implement the recipe expression/statement driver using existing value-based
Builder semantic owners where possible. Do not convert the recipe back to
AST. The admitted forms are:

```text
Literal | Variable
Unary(Minus | Not | BitNot, LinearScalar0)
Binary(ordinary value operators, LinearScalar0, LinearScalar0)
Expr | Print | untyped Local | Assignment | CompoundAssignment
```

All other forms return typed recipe rejection before physical effects. Every
admitted recipe form must have a total lowerer; no `Some`-without-lowerer
contract is allowed.

## S0-C — consuming BODY0 owner

Add one paired terminal on `InstalledRawRootEnvironmentV1`:

```rust
fn drive_root_body(
    self,
    recipe: RawRootBodyRecipeV1,
) -> Result<RawRootBodyCompleteInvocationV1, RejectedRawRootBodyInvocationV1>;
```

Internally it obtains the Builder loan and invokes private physical tracker
transitions. It creates an unpublished physical `main/0` draft, lowers the
recipe in source order, restores function context on success/failure, and
seals `CompletedRootBodyV1`. It does not call collector admission, ledger
reserve/complete/abort, condition creation, or root-batch commit.

The tracker path is fixed:

```text
fresh tracker
-> begin_root_body
-> RootBodyDrive
-> RootBodySeal
-> CompletedRootBodyV1
```

The existing `ModuleLoweringBorrowScheduleV1` is the order authority; do not
invent a second lifecycle schedule.

## Failure owner

```rust
RejectedRawRootBodyInvocationV1 {
    exact current owner,
    stage: Preflight | BeginTracker | Skeleton | Lower |
           FinalizeDraft | FunctionCleanup | SealTracker,
    typed cause,
    successful statement prefix,
    failing source site,
}
```

It exposes inspection and `discard(self)` only. Retry, resume, re-entry,
fallback, `mark_omitted`, `prepare_root_batch`, and `catch_unwind` are not
allowed. Primary/Cleanup/DuringCleanup details from the existing function
session remain nested rather than being flattened into strings.

## Required fixtures

```text
empty Script -> unpublished main/0 + CompletedRootBody(NoValue)
Script print/literal -> exact recipe site/value, one source-order draft
Script local/assignment/compound -> value-based lowering and cleanup
plain App Main.main() -> App metadata seal and unchanged callable evidence
Return / ScopeBox / If / Loop / And / Or -> typed preflight rejection
undefined variable -> failing site and discard-only unpublished owner
Primary/Cleanup/DuringCleanup -> exact nested cause retention
foreign brand/family -> mutation-free rejection
tracker mismatch -> no completion witness
all BODY0 paths -> collector delta=0, root ledger delta=0,
                    shell published-root count=0
```

## Guard contract

```text
begin_body(self) production definition                     = 1
RawRootBodyRecipeV1 producer                              = 1
Builder import of compiler source-facts                   = 0
ASTNode / OwnedRawSourceV1::ast in BODY0 files             = 0
classifier rerun / current_module in BODY0                = 0
legacy lower_root/build_static_main_box_typed/finalize_module = 0
MainPending/MainCaptured/PendingMainDraft in BODY0        = 0
collector/ledger mutation in BODY0                        = 0
condition_fn creation / root publication                  = 0
begin_root_body and seal_root_body producers              = 1 each
bare tracker complete production consumer                 = 0
retry/resume/fallback/catch_unwind                         = 0
ROOTBATCH0/drain/finalizer/postprocess/commit consumers   = 0
public ingress/JSON changes                               = 0
all modified source/check files                           < 800 lines
```

## Implementation boundary

Expected new modules (split before 800 lines):

```text
src/mir/raw_root_body_recipe.rs
src/mir/builder/raw_root_body_terminal.rs
src/mir/compiler/raw_root_body.rs
src/mir/compiler/raw_root_body_p0.rs
tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_body0_s0_guard.py
```

The source-facts provenance repair and App metadata seal are part of S0-A;
they must not be implemented as an untracked side change. No production
consumer is authorized by this card.

## Evidence commands

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_root_body --lib -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_body0_s0_guard.py
```

## Explicit non-claims

This row does not claim ROOTBATCH0, MainHeaderCompletion, collector
admission, condition_fn creation, drain, finalization, postprocess, external
commit, public ingress, JSON parity, legacy retirement, selfhost activation,
or CUT0 activation. Production consumers remain zero.
