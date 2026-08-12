---
Status: active compact card
Date: 2026-08-12
Scope: selected Dynamic callable, canonical session admission, hako.text.scan@1,
  AOT/LLVM production activation
ParentHistory: docs/development/current/main/design/archive/dynamic-fault-exit-transaction-d0-history-2026-08-10.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/ring2-provider-link-abi-lifecycle-ssot.md
---

# Dynamic callable current card

## Current capsule

Current decision: the final `VerifiedDynamicExitTransactionCoSealV1` is the
selected cohort's sole semantic plan. `CanonicalTrivialBindingSsaPlanV1` is a
different family and must not be extended to accept this Loop. The installed
package port remains the exactly-once transport owner; the existing A-prime
demand/emission plan opens the existing canonical CFG/SSA/PHI session inside
that scoped loan.

Current implementation status: exact-I64 semantic recut, exact-two Completion
and DraftSeal machinery, the Rust-VM nonconsumer fence, neutral output wire,
the I8 unpublished canary, and the complete R0 canonical-session projection
series are landed. The selected emitter now consumes the exact package input,
borrows the final Dynamic program only through its private HRTB authority,
snapshots Completion/control expectations, and opens its unpublished canonical
session internally. Production still uses the selected raw AST/JoinIR edge.

Next ordered task: implement the already-accepted AOT physical activation cell
(`DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0`) with the complete provider contract,
admission, strict LLVM leaf, I6/I7 receipts, End/lifecycle, and atomic selected
production switch. No provider/runtime/LLVM implementation is implied by the
completed R0 BoxShape row itself.

Production stop line: provider/AOT/runtime activation and the selected
production switch remain closed until the projection series is green. No
trivial-plan widening, second Completion/If/profile, raw AST repair, arbitrary
session pairing, fallback, retry, or Rust-VM DynamicV2 consumer may cross the
seam.

Retirement finish line: one atomic AOT activation consumes the selected package
loan through exact-two DraftSeal, removes the selected old edge in the same
commit, and leaves provider/selector/registry/image reselection, legacy
fallthrough, fallback, retry, and Rust-VM DynamicV2 callers at zero.

## Accepted design decision

```text
Decision:
  Reuse the final Dynamic program as the sole semantic plan and the existing
  A-prime demand/emission plan as the sole pre-session physical plan. Do not
  admit this cohort into CanonicalTrivialBindingSsaPlanV1.
Source authority + canonical issuer:
  Installed package same-batch loan + VerifiedDynamicExitTransactionCoSealV1;
  issue_selected_a_prime_i64_physical_demand is the existing co-seal issuer.
Non-authority:
  generic trivial analysis, package-local verify_function, canary semantic
  AST/header re-verification, reissued Completion/If, names/ordinals, provider,
  LLVM, VM.
Fail-fast boundary:
  ordinary/foreign/mismatched identity, authority reissue, arbitrary session
  pairing, borrow escape, double consume, or incomplete physical capability
  rejects before Builder effect.
Smallest next slice:
  DYNAMIC-V2-CANONICAL-SESSION-PROJECTION-R0, a behavior-neutral refactor
  series with the existing unpublished canary as its named consumer.
Non-claims:
  no accepted source shape, provider/runtime feature, LLVM hook, VM feature,
  production switch, generic typed-trivial expansion, fallback, or retry.
```

### Why the former trivial-plan premise is rejected

The target is a mixed typed/Dynamic Loop with an inner If Return. The generic
trivial verifier has no Loop arm, rejects Return inside If, and owns its own
trivial profile, If control, and Completion. Widening it would create a second
semantic planner beside the already-complete Dynamic Recipe/JoinSig program.

The correct products already exist:

```text
SelectedCallableLoweringInputRefV1
  -> VerifiedDynamicExitTransactionCoSealV1
       owns source / mixed Recipe / JoinSig / Completion / cleanup / exits
  -> VerifiedAPrimeI64PhysicalDemandV1
  -> PreparedSelectedDynamicV2EmissionPlanV1
```

The missing seam is only the physical session projection:

```text
borrowed sole Completion
+ Dynamic-program-owned Loop control disposition
+ exact same-source resolved input
+ move-only A-prime emission plan
  -> existing canonical CFG/SSA/PHI session
```

`PreparedProgramRootWorkPlanV1` stays a root scheduling owner. It does not gain
a borrowed canonical-plan field, avoiding a self-reference and foreign-plan
pairing surface.

## Final owner graph

```text
InstalledNormalCallableSemanticPackageV1
  owns batch + selected mapping + parameter contracts + final Dynamic program
             |
             | NormalCallableSemanticPackagePortV1
             | exactly-once HRTB selected loan
             v
SelectedCallableLoweringInputRefV1::Dynamic
  same-source ResolvedFunctionLoweringInputV1
  + &VerifiedDynamicExitTransactionCoSealV1
             |
             +-> borrowed canonical-session authority
             |     sole Completion
             |     Dynamic-owned Loop If disposition
             |     common outer If rows = 0
             |
             +-> issue_selected_a_prime_i64_physical_demand
                    |
                    v
             PreparedSelectedDynamicV2EmissionPlanV1
                    |
                    | opens its own scoped session
                    v
             CanonicalSsaFunctionSessionV2
               sole CFG / Binding SSA / PHI owner
                    |
                    v
             site-keyed Completion claims
             -> DraftSeal prepare: Return x 2
             -> DraftSeal commit
             -> Collector / Atomic Publish
```

The scoped loan may yield a private view, not a durable semantic receipt. The
view cannot escape the callback and exposes no raw AST, Recipe, JoinSig,
Completion parts, `ValueId`, or `BasicBlockId`.

## Ordered implementation DAG

### 1. `DYNAMIC-V2-SELECTED-SESSION-ADMISSION-D0` — accepted Decision

The previously listed projection row is now opened by this owner decision.
The target is a Dynamic Loop with an inner If Return, so
`CanonicalTrivialBindingSsaPlanV1`, `CanonicalLoweringPreflightV1`, and the
first-family trivial analyzer are not valid session inputs.  Making them
accept this shape would issue a second semantic plan, Completion, or If
authority.

Decision:
  choose one same-source admission boundary that lends the existing Dynamic
  semantic authority to the existing canonical CFG/SSA/PHI engine.
Source authority + canonical issuer:
  the installed package's exactly-once selected loan and its
  `VerifiedDynamicExitTransactionCoSealV1`; the existing
  `VerifiedAPrimeI64PhysicalDemandV1` ->
  `PreparedSelectedDynamicV2EmissionPlanV1` chain remains the only pre-session
  physical plan.
Non-authority:
  `CanonicalTrivialBindingSsaPlanV1`, generic trivial analysis, package-local
  Completion/If reissuance, canary semantic AST/header re-verification, raw
  AST/JoinIR meaning, names/ordinals, provider/LLVM/runtime/VM, and arbitrary
  external sessions.
Fail-fast boundary:
  if owner/function/forest/projection/source-root identity, Completion,
  Dynamic Loop control disposition, or lifetime cannot be lent exactly once
  without clone/reverification, remain `NoSafeSlice` before Builder effect.
Smallest next slice:
  decide the private HRTB/consuming callback shape and whether the existing
  canonical session can consume the borrowed facts without leaking a
  semantic borrow into DraftSeal.
Non-claims:
  no code, new durable `Verified*`/`Prepared*` semantic receipt, provider or
  LLVM implementation, VM work, production switch, fallback, or retry.

Required acceptance:

```text
selected Dynamic -> CanonicalTrivialBindingSsaPlan consumer = 0
selected Dynamic -> CanonicalLoweringPreflight consumer      = 0
Dynamic Completion semantic issuer                          = 1
Dynamic Loop control issuer                                  = 1
canonical-session admission issuer                          = 1
source/Recipe/Completion/If reissue                         = 0
semantic AST/header re-verification                          = 0
foreign/arbitrary session pairing                            = 0
```

Only after this D0 is accepted may the following parked BoxShape row open.

The accepted private boundary is:

```text
VerifiedDynamicExitTransactionCoSealV1
  -> with_canonical_session_authority(HRTB callback)
       borrows the retained Completion and Dynamic Loop control disposition
       and carries owner/target/source-root identity
  -> existing A-prime demand/emission plan
  -> CanonicalSsaFunctionSessionV2::new_selected_dynamic
       Completion consumer = Owned | Borrowed
       control consumer   = Resolved | DynamicProfileOwned
```

The HRTB callback is the only place where the borrowed authority and the
mutable canonical session meet.  It cannot return the borrow, a session, or a
semantic part.  `finish()` snapshots only the borrow-free physical claims and
return kind needed by DraftSeal.  The Dynamic control disposition is a
private view of the already sealed JoinClosure; it is not an empty
`VerifiedResolvedFunctionIfControlV1`, and it is not reissued from the source.

This closes the D0 design question.  The next row is the parked BoxShape
projection, beginning with the completion consumer's `Owned | Borrowed`
storage and borrow-free ready close.

### 2. `DYNAMIC-V2-CANONICAL-SESSION-PROJECTION-R0` — landed BoxShape

Change:
  make the existing Completion consumption ledger accept owned or borrowed
  Completion authority; add a private HRTB session-authority view on the final
  Dynamic program; make the Dynamic emission plan open the canonical session
  internally instead of accepting an arbitrary session.

Contract:
  semantic facts remain in the final Dynamic program. The physical consumer
  copies only target/result/site expectations into its one-shot claim ledger.
  Dynamic JoinSig remains the Loop-local control owner; common outer If rows
  are exactly zero without constructing another verified empty If product.

Done:
  the existing I8 unpublished canary uses the selected package loan, existing
  A-prime demand, borrowed Completion, and Dynamic-owned control disposition.
  Its calls to `verify_function_completion_v1` and
  `empty_for_owned_loop_profile` are zero. Ordinary/trivial behavior is
  unchanged, no borrow escapes, and the emitter no longer accepts externally
  paired outer/canonical sessions.

Landed evidence (R0-A/R0-B/R0-C):

```text
Completion consumer Owned|Borrowed storage                 = landed
borrow-free Ready close                                    = landed
final-program HRTB authority                               = landed
Dynamic-owned control disposition                          = landed
selected emitter external session arguments                = 0
selected canary semantic re-verification                   = 0
selected canary empty If reissuance                         = 0
preflight ledger Clone / clone-or-split production path     = 0 / 0
focused canary / semantic authority / pointer guards        = green
```

Stop:
  if the sole Completion or control disposition cannot be projected without
  clone, re-verification, raw parts, or a second semantic issuer, return to
  design stop.

Recommended refactor commits:

```text
R0-A  completion consumer Owned|Borrowed internal storage; borrow-free ready close
R0-B  private final-program HRTB session authority and Dynamic control disposition
R0-C  emission plan opens session; canary reissue/external pairing deleted; guard
```

Required structural evidence:

```text
selected Dynamic imports CanonicalTrivialBindingSsaPlanV1       = 0
selected Dynamic calls CanonicalLoweringPreflightV1             = 0
selected canary verify_function_completion_v1 calls             = 0
selected canary empty_for_owned_loop_profile calls              = 0
external canonical-session argument to Dynamic begin            = 0
Dynamic Completion semantic issuer                              = 1
Dynamic canonical-session projection issuer                     = 1
CanonicalSsaFunctionSessionV2 mutable owner                      = 1
provider / LLVM / VM additions                                  = 0 / 0 / 0
fallback / retry                                                = 0 / 0
preflight ledger Clone / clone-or-split production path          = 0 / 0
```

### 3. `DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0` — atomic BoxCount

This is one activation cell built in small owner modules. Intermediate code
does not become an independently selectable provider or production route.

No provider contract, registry, executable branch, wire, LLVM leaf, runtime
lease, or receipt is landed ahead of this cell.  The complete
`hako.text.scan@1` contract, admission, executable branch, strict AOT leaf,
canonical I6/I7 receipts, and lifecycle must land as one activation unit; an
isolated preparatory authority is forbidden.

Change:
  activate the complete `hako.text.scan@1` provider capability, strict AOT/LLVM
  I6/I7 execution, full Dynamic Loop physical session, exact-two DraftSeal,
  and selected package production caller; delete the selected raw AST/JoinIR
  edge in the same activation commit.

Contract:
  `TextSliceRange` and `TextFindNeedle` are the complete two-role capability.
  The ProviderSlot role contract is the sole I6/I7 result/lifecycle authority.
  The global provider spine is reused; runtime consumes a presealed executable
  branch and never searches a registry or reselects provider/image/selector.
  The LLVM formal lane is exact and role-bound: `src=0`, `pos=1`, `end=2`,
  `pred_chars=3`; swapped or shifted receipt rows reject before effect.

Activation preflight invariants (P0):

```text
physical symbol/header source       = existing catalog/source identity projection
raw method name -> physical symbol  = 0
raw body return scan in canonical    = 0
canonical skeleton input             = exact physical header + Completion contract
authority validation before Builder mutation = 1
legacy raw skeleton/body inference   = selected AOT path only, 0
semantic block count chosen by emitter = 0
DynamicProfileOwned owner validation  = exact or unit disposition
```

The selected physical symbol must come from the existing cataloged method
admission (`ParserScanLoopBox.skip_while/4` for this cohort), never from
`format!("{name}/{}", params.len())`. The canonical skeleton may allocate
function storage and entry blocks only after the selected package loan,
physical header projection, and Dynamic Completion/control expectations have
validated. It must not call `contains_value_return` or otherwise rescan raw
AST to infer a return shape; the existing legacy skeleton remains a
compatibility route only. These checks are part of the same activation cell,
not a new semantic authority.

Acceptance criteria:
  one consuming ProviderAdmissionSeal, immutable deterministic admitted
  registry, receiver-identity RuntimeExecutablePlan, strict CodePoint AOT leaf,
  I6 V10 value+one-shot lease/End, I7 ImmediateI64/no lease, complete operation
  and control schedule, two Completion claims, two physical Returns, one new
  selected production caller, and zero selected old callers are green. The
  schedule is mechanically derived from verified Recipe order/placement and
  JoinSig control (`Prelude`, `ThenTerminal`, `Continuation`); source-role names
  are diagnostic cross-checks only. The preflight ledger is move-only and has
  no Clone, clone, or split emitter path.

Stop:
  missing/foreign/duplicate contract or image, alias ambiguity, incomplete
  slot coverage, missing capability, lifecycle drift, synthetic return join or
  PHI, generic-method fallthrough, fallback, retry, or VM dependence rejects
  before activation.

Internal implementation order, without creating separate authorities:

```text
hako.text.scan@1 normalized contract + A-prime role requirement co-seal
  -> BoxCallableRegistryDraft
  -> consuming ProviderAdmissionSeal
  -> immutable admitted registry
  -> MethodCallRoutePlan / RuntimeExecutablePlan
  -> neutral call-in admission wire + PlanStamp
  -> canonical-session I6/I7 receipts
  -> strict LLVM early hook / CodePoint leaf
  -> V10 lease and exact End
  -> full unpublished physical session
  -> exact-two DraftSeal
  -> atomic selected production switch + old-edge deletion
```

Required activation counts:

```text
complete TextScan roles / same provider-profile                 = 2 / 1
ProviderAdmissionSeal / immutable admitted registry             = 1 / 1
mutable admitted insert / duplicate overwrite                   = 0 / 0
String|StringBox canonical branch                               = 1
RuntimeExecutablePlan with receiver/generation/image/PlanStamp  = 1
LLVM selected early consumer / strict leaf                      = 1 / 1
I6 receipt / lease issuer / End consumer                        = 1 / 1 / 1
I7 receipt / lease / End                                        = 1 / 0 / 0
Completion expected / claimed / physical Return                 = 2 / 2 / 2
synthetic return join / return PHI                              = 0 / 0
new selected production caller / selected old edge              = 1 / 0
runtime registry/selector/provider/image lookup                  = 0
selected legacy finalizer / name-type repair                     = 0 / 0
Rust VM DynamicV2 production consumer                           = 0
fallback / retry / sentinel-zero repair                         = 0 / 0 / 0
```

### 4. `DYNAMIC-V2-SELECTED-LEGACY-RETIREMENT-R0` — after cutover

Delete only after caller-zero evidence:

```text
selected source-seed-only route
selected raw JoinIR edge and legacy finalizer edge
test-side Completion/If reissuance helpers
superseded I8-only canary shell
diagnostic-only raw role/fingerprint authority uses
selected old topology callers
```

Global fixed-topology deletion waits for all remaining callers to reach zero.
H2/H3/H5 parity and the AOT mimalloc gate then run as independent siblings;
both must be green before Hako producer activation.

### 5. `MIRBUILDER-MODULE-DRAIN-CONVERGENCE-D0 -> I0` — after selected cutover

This is a post-cutover publication cleanup, not a second module authority.
First census every production lowering route, then converge the routes onto the
existing one-shot `ModuleLoweringInvocationDrainOwnerV1` and post-drain
finalization owner. The disconnected `module_invocation_cut0_p0` candidate is
not production truth and must not be activated as a parallel drain.

Done requires:

```text
production route census                                      = complete
one production drain owner                                   = 1
one production post-drain finalizer                          = 1
duplicate drain/finalizer callers                            = 0
legacy finalize_function_draft production callers             = 0
candidate-only drain path promoted                           = 0
one-shot drain / atomic publish                              = green
```

The row opens only after the selected AOT caller switch and old-edge
retirement. It does not change semantic source, Recipe, JoinSig, Completion,
provider, or VM ownership.

### 6. `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0` — after legacy retirement

After `DYNAMIC-V2-SELECTED-LEGACY-RETIREMENT-R0`, perform a full production
and test caller census for the fixed-role topology and its old issue APIs.
Only when every caller is zero may the fixed-role receipt, legacy boundary
receipt, and old `issue(...)` compatibility path be hard-deleted. A segment
route is not considered complete merely because a new caller exists; the
retirement row requires proof that no remaining route depends on the old
topology.

Done requires:

```text
fixed-role production callers                             = 0
fixed-role test/guard callers                              = 0
old issue(...) callers                                     = 0
segment route completeness                                 = green
fixed-role receipt / boundary types                        = deleted
compatibility fallback                                     = 0
```

This is a post-cutover BoxShape/retirement row. It cannot be used to bypass
the AOT capability gate or to delete an old path while it still owns a live
production edge.

## hako.text.scan@1 semantic contract

```text
profile: utf8-codepoint-clamped-v1
receiver: canonical Text
aliases: String | StringBox, canonicalized before admission

TextSliceRange / substring/2
  CanonicalText + ImmediateI64 + ImmediateI64 -> CanonicalText
  CP half-open range, endpoint clamp, synchronous
  Normal result = one EndAuthorized lease

TextFindNeedle / indexOf/1
  CanonicalText + CanonicalText -> exact ImmediateI64
  first CP index, empty needle = 0, miss = -1
  Normal result lease = 0, End = 0
```

Selector and diagnostic strings only cross-check dispatch keys. They do not
decide result class, representation, lifecycle, provider, or executable entry.
The strict leaf does not call the environment-selected/generic String surface,
string-to-i64 compatibility parsing, or sentinel-zero helpers.

## Mandatory cleanup and line-budget gates

These are BoxShape siblings, not semantic progress and not substitutes for the
current session projection or production cutover.

```text
MIRBUILDER-LINE-BUDGET-R0
  split module_draft_collector.rs (801; tests start near 433)
  split completion_tests.rs (894)
  split src/mir/resolved_value_profile/analyzer.rs (769) at its policy/
  verification seam; keep one analyzer authority and move only private
  helpers/tests. Freeze src/mir/builder.rs (787): no additions before its
  module-registry classification row below.
  treat this as a pre-cutover hard gate for the 801/894-line files: no new
  production authority or physical activation code may be added to them;
  analyzer.rs is either split at the same private seam or frozen unchanged.

CURRENT-STATE-LIVE-SCHEMA-I0
  CURRENT_STATE.toml -> live pointer/blocker/next/parked + bounded landed tail
  historical key registry -> generated/archive index

MIRBUILDER-WORKSTREAM-ARCHIVE-R0
  rolling workstream current brief below 800 lines
  closed chronology -> archive/git history

MIRBUILDER-EMIT-INSTRUCTION-PHASE-SPLIT-R0
  keep one public emit_instruction writer; split private prepare/validate/
  physical-commit/post-metadata phases

MIRBUILDER-BUILDER-BUILD-SPLIT-R0
  keep the existing MirBuilder methods, visibility, callers, and emission order
  builder_build.rs becomes a thin facade over:
    literal_lowering.rs
      literal dispatch + exact-numeric constant metadata
    variable_read.rs
      variable lookup + undefined-variable diagnostics
    assignment_lowering.rs
      assignment + local/typed-array contract publication
      + previous strong-reference release
    new_expression.rs
      PreparedRawNewExpression + raw new route + legacy child descent
  move file-local tests to one sibling test module
  do not add a new lowering entry, receipt, fallback, env policy, or authority

MIRBUILDER-MODULE-REGISTRY-CLASSIFY-R0
  run after selected production cutover and a caller/cfg census
  keep builder.rs as the sole MirBuilder facade and preserve module paths,
  re-exports, visibility, and cfg gates
  reorder its declarations into:
    state / session
    source admission
    semantic plans
    physical lowering
    draft collection / publication
    legacy compatibility
    tests / migration-only
  move inline binding tests to a sibling test module
  move historical phase/migration prose to archive or owning README
  remove #[allow(dead_code)] or disconnected modules only after caller-zero;
  classification alone never retires them

MIRBUILDER-COMPLETION-COMMENT-CLEANUP-R0
  update the stale completion-consumption comment that still describes a
  single explicit claim; exact-two site-keyed consumption is already landed
  and must remain the only physical claim model
```

Both rows are behavior-neutral refactor series of two to five commits. Their
Done boundary is unchanged public callers and diagnostics, focused parity and
failure tests, no new production edge, `git diff --check`, and every touched
Rust file below 760 lines. If moving a method changes metadata/publication/
release ordering or a module move changes its Rust path or cfg reachability,
the series stops and returns to its parent commit.

The current active card is intentionally compact. Its multi-thousand-line
predecessor is retained only under `design/archive/` as historical evidence.
It is not a current pointer or implementation authority.

## Common negative matrix

```text
Ordinary or foreign selected loan                         -> reject/not selected
owner/function/forest/projection/root drift              -> reject
missing/duplicate/extra Completion site                  -> reject
wrong function target or return operand                  -> reject
Loop/local If/inner Return shape drift                   -> reject
second Completion/If/profile issuance                    -> guard failure
raw AST/Recipe/JoinSig/ValueId handoff                    -> guard failure
arbitrary canonical-session pairing                      -> API/guard failure
borrow escaping package callback                         -> compile failure
provider/LLVM/End incomplete at activation               -> RejectBeforeEffect
selected legacy/generic fallthrough                      -> guard failure
name/ordinal/selector repair, fallback, retry             -> guard failure
Rust VM DynamicV2 provider/receipt/session                -> guard failure
```

## Focused gates

```bash
cargo test -q --lib normal_callable_semantic_package
cargo test -q --lib dynamic_full_body_recipe
cargo test -q --lib selected_dynamic_physical_emitter
cargo test -q --lib completion
cargo check -q --lib

bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dynamic_v2_physical_input_authority_guard.sh
bash tools/checks/dynamic_v2_callslot_wire_authority_guard.sh
bash tools/checks/dynamic_v2_vm_nonconsumer_fence_guard.sh
bash tools/checks/loop_precutover_authority_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

## Non-claims

```text
CanonicalTrivialBindingSsaPlanV1 Dynamic expansion
generic all-V2 Loop admission
full String surface or I6-only provider slot
Dynamic-specific registry
runtime provider/selector/image lookup
generic String compatibility route
Rust VM provider/receipt/session
new Recipe/JoinSig/CFG/SSA/PHI/Completion authority
production cutover before the complete atomic activation cell
fallback / retry / legacy dual-production
```

## History

Detailed landed chronology lives in git history and the historical archive
named in `ParentHistory`. This card owns only the live Decision, next slice,
activation boundary, retirement conditions, and cleanup queue.
