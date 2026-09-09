---
Status: Follow `docs/development/current/main/CURRENT_STATE.toml`; this rolling file is not the active pointer
Date: 2026-09-06
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
---

# MirBuilder In-Place Replacement Workstream

## Goal

最終production authorityを一本へ収束させる。

```text
Resolve -> Observe -> Facts -> Recipe -> Verify
        -> Lower -> Seal -> Collect -> Atomic Publish
```

現在のMirBuilderを稼働させたまま、一つのproduction responsibilityとその旧辺を
同じbounded seriesで交換する。第二MirBuilder、consumer-zero route、fallback、
fixture由来のacceptance、新しい文字列authorityは作らない。

## Current restart pointer

`CURRENT_STATE.toml` is the sole mode and row authority. Current birth work is
branch-only WIP, not the closed main checkpoint; StaticBoxMethod, FreeStatic,
FreeFunction, Builtin Print, and one root-lexical SameModuleInstance semantic
vertical are landed and must not be redesigned. The canonical product remains:

```text
source -> exact Facts/Recipe -> typed target before arguments
       -> canonical Call -> Seal -> Atomic Publish
```

The M1 current-HEAD Call producer/consumer census and M2 remaining-family
disposition are closed in `mirbuilder-final-pipeline-ssot.md`. M3-A is now
closed at `474e8518b0`: the UnifiedCallEmitter core has zero environment
reads and zero `emit_legacy_call` calls, while the outer MirBuilder facade
preserves explicit compatibility and required ingress stays fail-closed.
Historical Call test inventory and baseline receipts live in Git; the recorded
133 whole-lib failure set is not a green claim for the current Array row.
M3-B is parked after its finite census: the Birth issuer is unique, but two
unclaimed compatibility writers share the existing outer APIs and have no
exclusive delete-set. M3-C's four JoinIR/JSON ingress censuses are complete
and parked.
R6 Group A closed at `45c6759962` and Group B closed at
`cce62db090`: canonical and legacy instruction shapes are separate, and the
VM reference reader directly consumes canonical Builtin(Print) while wrong
arity and other canonical targets fail closed. The finite post-Group-B census
closed at `bb41e2e880` with
`NoSafeSlice__NoSingleRemainingCanonicalReaderFamily`.

Do not repeat the census. The bounded WASM `LegacyCallV0(Global)` reader stop
landed at `833eb87a80`: the shared preflight now rejects it before shape/WAT/
binary/fallback work, and the name/arity/zero-padding reader is gone. No
canonical WASM reader or general fallback retirement is claimed. The next
exact tuple `MIR-CALL-VM-GLOBAL-CANONICAL-CUTOVER-R0` has now landed at
`111216b539`: the VM canonical Print reader covers same-module
FreeFunction/StaticBoxMethod, Legacy Global is rejected at the shared ingress,
and only that finite VM legacy arm was removed. The stale WSM-G4-min8 success
probe was retargeted to explicit pre-WAT rejection and its old lock retired.
The exact reader stop `MIR-CALL-LEGACY-READER-STOP-VM-EXTERN-R0` landed at
`18f08124f8`: Legacy Extern now rejects before provider dispatch and its VM
legacy arm is gone. The exact Rust WASM reader stop
`MIR-CALL-LEGACY-READER-STOP-WASM-EXTERN-R0` landed at `3c7f5ea5bc`: the
preflight rejects before shape/WAT/binary/fallback, the legacy Extern lowering
arm and reader-only name helpers are gone, and `EXTERN_CALL_MAP` remains the
runtime import contract.
Reopen another family only when source authority, typed issuer, lossless
selected consumer, real production caller, fail-fast terminal, exclusive
old-edge delete-set, outside-reader inventory, and source budget form one exact
tuple.

The finish path is:

```text
one exact R6 family cutover
  -> stop/quarantine every product-reachable LegacyCallV0 reader
  -> R7 caller-zero schema/repair deletion
  -> Builder physical thinning
  -> backend retirement after replacement

post-R6 sibling:
  Hako published-view ingress and backend family migration
```

Backend parity is not an R7 prerequisite. In particular, Rust WASM's legacy
reader must stop or quarantine before R7, while Hako WASM W0 may land later on
the mandatory canonical publication. Closed details remain in Git and the
final-pipeline SSOT; this workstream does not duplicate their task ledger.

Broad Extern, Value/closure, explicit object method, nested/upvar receiver,
ordinary-new no-claim, generic CorePlan GlobalCall, and physical thunk are not
implicit next implementation rows. They reopen only with exactly one existing
source authority, canonical issuer, live caller, lossless selected consumer,
and finite old-edge delete set. Otherwise they stay `ParkedSealed` and no
additional D0/receipt/adapter/fixture/guard is created.

Hako SameModuleInstance remains `ParkedSealed__HakoIngressMissing`: the Rust
published-view owner exists, but Hako has no borrow-only ingress and no real
scalar production caller. This does not block mandatory-Callee MirBuilder core
completion. selected-C arbitrary UserBox remains typed
`UnsupportedBeforeObject`; it is a compatibility/backend fate, not a language
or semantic issuer.

Repository reduction is coupled to the same queue. Every switched family
deletes its selected old edge and exclusive temporary assets before the next
family. R7 owns Call-only legacy tests/guards/docs/adapters after caller-zero;
post-R7 thinning owns dead barrel exports, raw ports, stale wrappers, false
dead-code allowances, and disconnected proof modules. Closed detail belongs
to Git history; tracked archive copies receive zero reduction credit.

The superseded 2026-09-02 family audit, baseline refresh detail, and landed
family table are retained by Git history and the owning manifest. They are not
a second executable queue.

### Execution order authority

The executable order is the compact M0--M9 Call completion program in
`mirbuilder-final-pipeline-ssot.md`. Landed row narratives and exact evidence
remain in Git history and the owning machine-readable manifest; this
workstream does not duplicate them.

### R6/R7 migration program (worker-audited 2026-09-03)

The current `NoSafeSlice` is schema-wide, not repository-wide. `MirCall` is
ready, but `LegacyCallV0` still has shared builder, JoinIR, JSON v0/v1,
serializer, structural-reader, and backend owners. Do not repeat the census,
add `CallV2`, or delete `func`/`Option<Callee>` in isolation. The executable
queue is staged, but remains unopened until its exact boundary is selected:

```text
R6-S0  behavior-neutral preparation:
       split instruction schema/visitor and published-view/transport owners
       before semantic growth; no new authority, receipt, or guard.
R6-S1  one canonical producer cohort:
       existing source authority -> mandatory MirCall -> one typed consumer;
       delete that cohort's old writer/reissuer in the same series.
R6-S2  one backend boundary:
       typed consume or UnsupportedBeforeArtifact before codegen; no JSON,
       name, registry, args[0], fallback, or retry.
R6-S3  outer compatibility quarantine:
       JSON/JoinIR/unified-off remain explicit ingress only and cannot re-enter
       canonical production; shared owners are not split by guesswork.
R7    caller-zero physical retirement:
       delete LegacyCallV0, func, optional callee/receiver, repair, and
       family-only assets only after production writer/reissuer/reader zero.
```

Migration red is test-only and named: each changed test records path, owner,
reason, successor, first-red commit, and expiry at family closeout. Production
build/check failures, unknown red, test deletion/ignore, baseline rewrite, or
fallback/retry aborts the series and returns to design_stop. Existing known-red
failure names remain a separate immutable baseline. A family is eligible only
when one issuer, lossless consumer or typed terminal, caller, and exclusive
old-edge delete-set are all proven; otherwise it stays `ParkedSealed`.

### Family-local scheduler reconciliation (worker-audited 2026-09-03)

The aggregate `NoSafeSlice__NoSingleRemainingCanonicalReaderFamily` is a
schema-wide R6 disposition, not a repository-wide scheduler stop. Existing
M7-S reader stops are already landed for Rust WASM `Global`, `Extern`, and
`Method`, and for the VM `Global`, `Extern`, `Value`, and `Method` cohorts;
do not reopen or duplicate those rows from an older review. The scheduler
selects only an already-inventoried family with one source/issuer (for Promote)
or one compatibility reader and typed terminal (for Stop), one real caller,
an exclusive delete-set, and no unclassified red. A family with zero/multiple
owners remains `ParkedSealed` while other eligible families may proceed.

The 2026-09-04 worker audit decomposed the former shared boundaries at their
outer leaves. Reuse generic `MIR-CALL-LEGACY-READER-STOP-R0`; do not add a new
card, semantic receipt, adapter, fixture file, cohort dispatcher, or guard.
`direct_mir_json_duplicate_reader_delete` landed at `ef3ee28bc5`,
`skip_ws_probe_reader_delete` at `d4ce50b87c`, and
`canonical_value_fallthrough_stop` at `a33987e8e4`, and
`methodize_fallthrough_stop` at `24ece062bb`; the fixed comparator stayed
unchanged. `singleton_name_args0_reissuer_stop` landed at `01a1a6bc83`:
its retired JSON singleton name/args[0] reissuer is stopped before mutation,
the singleton rewrite code is gone, and early-phi/JSON classification remain
unchanged. Stage1's ReturnCall writer/name/arity path was removed at
`99b4446cab`, but the selected defs boundary has no executable acceptance: the
three old smokes neither contain that defs writer nor pass the earlier static
terminal. The semantic cohort is
`ParkedSealed__SelectedBoundaryUnreachableThroughCurrentImportClosure`; do not
restore its writer, add a test-only seam, or call it landed. The three invalid
probe scripts and their exclusive fixtures (6 files/189 lines) were
RetireFromTree'd at `f15098cf0b`; stale quick/checklist paths and default
integration discovery are now zero. The active
`mir_json_v0_call_ingress_stop` leaf landed at `9a40ece824`: both
call spellings stop in the shared Rust MIR JSON-v0 dispatch before publication,
the caller-zero call/catalog parsers and call-only tests are deleted, and 14
focused tests pass. Its closeout compacts the 1,000-line active card toward
900 lines using hash-plus-one-line tombstones. Only a future unchanged route
reaching `FuncLoweringBox` may reopen terminal ownership and predicate naming;
caller-zero deletes that owner instead. Remaining families stay local
`ParkedSealed`; R7 still waits for all legacy callers to reach zero. The
ArrayElementWrite projection candidate is now closed by the typed selected-C
consumer landed at `9cb7a6c71a`: all four operations have one published row
owner, the three selected native artifact callers no longer invoke legacy
projection, and the generic `Insert` gap is covered by a direct runtime alias.
The remaining llvmlite projection is an explicit compatibility lane outside
this closeout.

### S-class gate coverage (navigation only)

The five post-M9 S-class gates are already defined by the final-pipeline SSOT;
they are not additional current execution rows. Their concrete coverage is
kept explicit here so a release claim cannot be mistaken for the current R6
closeout:

```text
S1 Enforce       M4/M6/M7 + M8; private boundary constructors/capabilities
                 and negative guards must be observed before release.
S2 Prove         M5/M6 evidence is necessary but not sufficient. A future
                 MIR-VERIFY-VM-LLVM-DIFFERENTIAL-R0 must compare observable
                 results/failures/effect order, and a compact spec-ID trace
                 matrix must link each rule to positive and reject evidence.
S3 Delete        M3/M4/M7/M9; LegacyCallV0 production writers/reissuers/
                 readers and compatibility repair must reach caller-zero.
S4 Bootstrap     owner: selfhost-bootstrap-route-ssot; stage0->stage1->stage2
                 reproducibility and identity comparison are post-M9 gates.
S5 Release       owner: hakoruneup-release-distribution-ssot; clean checkout,
                 pinned tools, sample ladder, limitations, and regression
                 evidence are post-M9 gates.
```

S2's differential harness, rule traceability, S4 bootstrap proof, and S5
release packaging are planned but unopened. The accepted whole-library receipt
is `7543/7381/133/29`; it is stable known debt, not a green claim or permission
to rebaseline.

### Compact closed tail

- Core/Call retirement tombstones: `598530d23b`, `e5120589dc`, `2b7b3e7489`,
  `44555655ab`, `7a6fb9e2db`, and ordinary-new `4b2db34ee3`.
- Repository/tooling tombstones: legacy-tests `bcc9a6ba65`; entrypoint policy
  `06454bd084`/`c78889dc0b`/`b22a87392d`; baseline/matrix
  `878480e395`/`80dc7102fb`; current accepted receipt is
  `7543/7381/133/29`; the earlier 7555/7393 and 138-name receipts are
  historical.
- focused green is not a whole-repo green claim; the 133-name receipt remains known debt.

Each production family closeout repays its own old implementation, exclusive
tests/proofs, row guards, adapters, and closed docs. Moves or tracked archive
copies earn zero reduction credit. A standalone broad test/guard purge stays
parked unless caller-zero plus an equal-or-stronger successor is proved.

### Audit follow-up boundary

The landed Print and true FreeFunction slices do not claim whole-exit proof or
whole-repository health. The mixed App Main/top-level helper regression is now
covered by the FreeFunction cohort. Before widening the next family, the
remaining named follow-ups are the deterministic known-red/no-new-red gate,
representative runner/C negative evidence, and minimal negative coverage for
the remaining published-view/join variants. Phi ordering, definition
overwrite protection, and non-scalar argument admission remain separate design
work. These items do not reopen either completed cohort and do not authorize a
second semantic pipeline.

## Current architecture decision

Final Call shape:

```rust
enum Callee {
    Global(CanonicalGlobalTargetV1),
    Method { receiver: ValueId, /* existing typed method fields */ },
    Value(ValueId),
    Extern(String),
}

Call {
    dst: Option<ValueId>,
    callee: Callee,
    args: Vec<ValueId>,
    effects: EffectMask,
}
```

```rust
enum CanonicalGlobalTargetV1 {
    Builtin(CanonicalBuiltinGlobalV1),
    SameModule(CanonicalSameModuleGlobalTargetV1),
}

enum CanonicalBuiltinGlobalV1 { Print }

enum CanonicalSameModuleGlobalTargetV1 {
    FreeFunction { name: Box<str>, arity: u32 },
    StaticBoxMethod { owner: Box<str>, method: Box<str>, arity: u32 },
}
```

`Print`はexact `print/1`だけ。bare `panic/1`はCall/Externではなく、共通exit
transactionを通るterminal Faultのaccepted target（production 0）。bare `exit/1`
は未発行の別lane。bare `error`と`now`はrejectし、explicit providerだけがExtern
になる。mathはMethod、GCは未実行Global producerをretireしてrejectする。
import aliasはfinal moduleのexact declarationに着地した時だけSameModuleとし、
distinct Imported/Helper/Generated/Legacy variantは作らない。JoinIRは生成関数
宣言とcallを同じownerでco-sealし、SameModule FreeFunctionを使う。

```text
legacy text
  -> owner-private compatibility resolver exactly once
  -> typed target
  -> canonical Call
```

Canonical Call-corridor MIR JSONはexact `schema_version = "2.0"`。Global targetはfamily付き
object、`args`/`effects`は必須、`flags`/`func`/aliasは不可とする。effectsは
coreのcanonical順で重複・未知語彙なし。exact `1.0`とv0はowner-private
compatibilityであり、明示schemaのparse失敗を別schemaでretryしない。v2のop cohortは
`const/copy/copy_owned/destroy_owned/newbox/field_get/binop/compare/branch/jump/phi/ret/mir_call`
だけで、他opはtyped unsupported。full MIR-v2 vocabularyはこのlaneのclaim外。

## Current finite state

```text
Global B0
  BuiltinPrintReady
  SameModuleFreeFunctionReady
  SameModuleStaticBoxMethodReady
  CompatibilityTextReady
  MissingSourceRelation
  ForeignModule
  DuplicateOrCollision
  AliasUnresolved
  WrongNamespace / WrongArity
  UnsupportedForWireOrCompiledConsumer
  TypedRejectBeforeEffect
  ExternOrMethodOrConstructionOwner

Ingress Wpre
  SharedArtifactProfile
  DirectMirProfile
  CanonicalV2Selected
  CompatibilityV1Selected
  CompatibilityMirV0Selected
  ProgramV0OwnerSelected
  CanonicalV2ParserUnavailableBeforeB1
  FamilyForbiddenAtEntranceRejected
  SelectedDecoderRejected
  MalformedJson / MalformedSchema
  ConflictingMarkers / UnsupportedVersionOrShape
  TypedRejectNoRetry

D1B lifecycle
  ObserverOnlySiteRecorded
  OwnerObservationComplete
  DispositionReady
  TargetReadyForLoan
  KnownNonDirect
  TypedRejectBeforeArguments
  ParkedCompatibility
  PackageAbortBeforeInstall
  RawLoanInstalled / Consumed / Exhausted
  ResolvedFallthroughForbidden
```

Only a typed target crosses into argument descent. `KnownNonDirect` exits through
its typed owner. Reject/park issues no target. Incomplete inventory aborts before
install. Resolved fallthrough and residual loan are guard failures, not terminals.

## Adversarial correction

Ordinary `FunctionCall`のobserver/package contractも受理済み。selected shadow
profileのDeferred edgeは次の一回限りtransitionで置換する。

```text
profile-gate-adjacent observer-only FunctionCall
  -> record existing site/name/arity
  -> observe arguments in the same traversal
  -> issue no target
  -> allow owner observation to complete
  -> require total disposition before package install
```

No second AST walk、package-external scratch、target issuance、semantic profile
widening、BodyEffect inference。Package installはtotal dispositionを要求する。
後続affine loanは既発行Calleeだけをmoveし、同じcellでdirect
`CatalogedTargeted` payloadを削除する。

## Ordered frontier

Execution projection updated 2026-09-05. Global order remains in the
[final-pipeline SSOT](../design/mirbuilder-final-pipeline-ssot.md).
The former B0/Wpre/individual-row navigation list is superseded here;
its exact historical body remains at `bafa076579`. This removes stale
scheduling instructions, not unfinished acceptance obligations.

### Closed now — VM legacy call terminal collapse

M7-S `vm_legacy_call_terminal_collapse_i0` landed at `a74648f0b3`; no new card, semantic
carrier, receipt, adapter, fixture, or guard. ArrayElementWrite write-only
source/MIR/OBJ/EXE/reject evidence is closed; runtime readback remains at its
separate `mir_call_no_route` terminal and is not widened into that row.

1. **One terminal.** Both existing VM instruction paths in
   `src/backend/mir_interpreter/handlers/mod.rs` and `exec/block.rs` call the
   same `reject_legacy_call` helper. Global, Method, Value, Extern,
   missing-callee, and other unsupported legacy shapes keep their existing
   typed rejection contract; canonical `MirCall` paths are untouched.
2. **Old edge delete-set.** Remove `handle_call`, `execute_callee_call`, the
   VM legacy dispatch connections, and the VM-only `HAKO_CABI_TRACE` legacy
   route display. Do not remove the `LegacyCallV0` schema or reopen canonical
   Call/R7 work.
3. **Acceptance (landed).** Existing call-handler tests and
   `cargo check --features vm-reference` must pass. Structural search proves
   the deleted VM symbols have no callers; canonical print/free/static tests
   remain green and legacy Global/Method/Value/missing-callee inputs reject
   before old dispatch. The fixed baseline comparator and existing pointer
   guards remain unchanged.

Non-claims: no VM feature parity, no `SameModuleInstance` execution, no
LegacyCallV0 schema deletion, no Hako/WASM/JSON ingress work, no new backend
consumer, and no whole-repository green claim. After closeout, the next
eligible M7-S owner is selected from the existing finite queue; parked families
remain family-local.

### Scheduler pause after the closed VM cohort

The finite M7-S queue has no safe next owner at this checkpoint. MIR/JSON
`boxcall` is shared by parser, artifact, selfhost, and VM-Hako compatibility
callers; the JSON egress is likewise shared by harness, VM-Hako, selected
dynamic, and CLI export paths. Both lack an exclusive delete-set, so they are
`ParkedSealed__SharedCompatibilityCallersNoExclusiveDeleteSet`. Do not reopen
the VM row, repeat the broad census, or add a new D0/receipt/adapter/fixture/
guard. Resume only when an existing owner-specific decision or caller-zero
asset supplies a finite Stop/Delete boundary.

These are repair/verification tasks, not a request for a new source authority.
An upstream terminal is dependency evidence: name its owner and fix an in-scope
regression or make a bounded disposition decision; do not silently claim the
downstream acceptance passed.

### Dependency-ordered remaining development

| order | existing task / owner | finite output and handoff |
| --- | --- | --- |
| A | `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE` in final-pipeline | Pin the existing real-app EXE owner at `72f2496568fcd555499fdcb26fef2d8f1df03089` / manifest `fb27826bbf81694bb86056a5fbf389b6216b5eea`; run its 11 fixed entries, record hashes/expected result and exclusions, then hand the finite scope to convergence. |
| B | M7-S remaining owner-unit Stop/Promote/Delete | Use the existing finite reader inventory, including MIR-to-JoinIR and remaining external compatibility owners. For each shared owner close outside readers and one finite delete-set together; do not split by callee merely to create more rows. Preserve accepted product scope from A. No backend parity prerequisite for an authorized compatibility Stop. |
| C | Call/R7 `MIR-CALL-COMPATIBILITY-RETIRE-R7` | Production writers/readers/reissuers/re-entry/fallback zero; delete LegacyCallV0 and obsolete Call/Method shapes in one isolated series. Compiler diagnostics cover mechanical uses, supplemented by existing dynamic ingress inventory. No CallV2 or second resolver. |
| D | Loop prerequisite closure in `joinir-loop-selfhost-recipe-pipeline-ssot.md` M10 | Reuse recorded closed proofs; close the first unfinished semantic-program co-seal, JoinSig transfer, common bound segment or S6C package dependency. Each output must name its actual consumer/cutover/delete-set, not another disconnected receipt. |
| E | Loop/M8 S6 and Loop/M9 S7 | Complete the fixed all19 coverage and selfhost Loop parity sets. These are NOT Call/M8 cleanup or Call/M9 backend retirement. Missing source shape is a named bounded design/implementation dependency, not a repository-wide Park. |
| F | `LOOP-PRODUCTION-SELECTION-D0` → Loop/M10b → M11 → M12 | After prerequisite/coverage evidence, decide production selection, switch the portable handoff, then remove raw/family adapters. Use exact existing IDs in the Loop owner SSOT; no repetition of landed G0 proofs. |
| G | `REPO-FINAL-CONVERGENCE-AUDIT0-G0` and physical cleanup | Run scope A and verify actual switches/deletions. Remove caller-zero tests/guards/facades/docs leaf-to-root after equivalent evidence is retained. Call/M8 cleanup can repay already-zero owners earlier, but cannot substitute for Loop completion. |
| H | unified selfhost resume owner | Whole MirBuilder gate → language conformance/rejection matrix → canonical mimalloc gate → `MIRBUILDER-FACT-OWNER-PARITY-TEMPLATE-PILOT-SELECTION-001`. Select actual Facts authority/caller/deletion, not the retired MapStore/classifier queue. |
| I | authority migration and final self-compile task pack | Facts/REGISTRY/commands/executor then parser; pinned compiler closure compiled by Stage1 without Rust frontend delegation, runnable Stage2 compiles and runs acceptance programs. Equality alone is bootstrap evidence. |

### Acceptance-scope closeout (queued existing owner)

This is a closeout task, not a new semantic family. The owner manifest is
`tools/smokes/v2/suites/integration/real-apps-exe-boundary.txt` at manifest
revision `fb27826bbf81694bb86056a5fbf389b6216b5eea`, evaluated at repository
HEAD `72f2496568fcd555499fdcb26fef2d8f1df03089` with:

```text
tools/smokes/v2/run.sh --profile integration --owner-profile integration \
  --suite real-apps-exe-boundary
```

The fixed selection is ten source programs plus one unsupported-boundary probe:

```text
apps/typed-object-newbox-min/main.hako          exit 30
apps/typed-object-untyped-field-min/main.hako  exit 7
apps/typed-object-birth-min/main.hako           exit 30
apps/typed-object-method-min/main.hako           exit 30
apps/typed-object-birth-param-min/main.hako     exit 30
apps/boxtorrent-mini/main.hako                  stdout + Result: 0
apps/binary-trees/main.hako                     stdout + Result: 0
apps/mimalloc-lite/main.hako                    stdout + Result: 0
apps/allocator-stress/main.hako                 stdout + Result: 0
apps/json-stream-aggregator/main.hako           stdout + Result: 0
real_apps_exe_boundary_probe.sh                 exact unsupported boundary
```

The runner, manifest, and each existing smoke own the command, backend,
toolchain, expected output, and evidence. At task execution, record the
source/script SHA-256 values and the observed result without changing fixtures,
except the explicit annotation migration below; retain its pre-migration evidence.
Required exclusions are language-v1 corpus, Loop/M8-M9 parity, non-delegating
selfhost proof, WASM, unselected backend parity, and the whole-library known-red
baseline. This scope must not grow into a second ledger or a whole-repository
green claim. Missing evidence is an open acceptance item; after the fixed list
is recorded, the next handoff is `REPO-FINAL-CONVERGENCE-AUDIT0-G0`.

#### Acceptance incident and bounded repair order (2026-09-05)

Current consultation (2026-09-06, worker-audited `bd517a7bd5`):
`MaterializationRelationMissing`; design/taskization only. The
[constructor lifecycle Decision](../design/constructor-birth-new-lifecycle-ssot.md#typed-c-program-handoff-decision)
owns the authority inventory and supersedes the speculative fixed-Pair emitter.
No implementation permission follows from the worker report or prior local green.

The constructor owner retains the Pair witness and annotation-migration lineage.
Historical V2 Stop/Delete validation is superseded by V4; closed implementation
and test details live in Git, not a second active execution contract here.

**Constructor execution order**

The [constructor plan](../design/constructor-birth-new-lifecycle-ssot.md#accepted-physical-consumer-plan-2026-09-06)
and [lowering card](../design/constructor-lifecycle-llvm-lowering-ssot.md#feedback-reconciliation-queue-2026-09-07)
own execution evidence, acceptance incidents and retirement sets. Pair V4 host
cutover supersedes the old V3/V2 pending-path snapshot; uncovered acceptance
remains with that owner. Continue Call/R7 -> Loop -> language/selfhost.
VM/WASM parity and unrelated performance are outside this task.

### Backend/runtime feedback and task order (2026-09-06)

The [constructor card](../design/constructor-lifecycle-llvm-lowering-ssot.md)
records Pair V4 EXE/OBJ30, Fault cleanup, V2/V3 retirement and view/C/docs cleanup.
Uncovered source representations and full Call R7 remain open.

Order: canonical/compatibility separation -> compile-call state/options -> kernel
hook single storage -> later runtime crate dependency reduction. Each selected
series retires its replaced edge. A demonstrated shared-state failure on the
selected path reopens that dependency immediately; concurrency is not assumed.
The [C boundary owner](../../../../../lang/c-abi/README.md)
retains task2–4 responsibilities and acceptance.

#### MIR-CALL-PUBLISHED-GLOBAL-ROW-REQUIRED-I0

Landed at183aede6e6: required Global rows stop before legacy interpretation.
Landed at7891dbf747 (`MIR-CALL-PUBLISHED-EXTERN-STOP-I0`): published Extern stops
before legacy lookup; generic compatibility remains. C/view/host evidence and
parent ba62871459 stale README guard diagnosis/fix live in Git. No R7-wide claim.

#### MIR-CALL-PUBLISHED-METHOD-CONSTRUCTOR-DISPOSITION-D0

Accepted at 83d7553ceb: intrinsic literals; named new retains provider semantics.
The [language reference](../../../../reference/language/block-expressions-and-map-literals.md#4-collection-literal-construction-identity)
owns the Decision. Runtime named helpers remain CompatibilityOuterIngress;
general Method/Math and Call-carried Constructor keep their existing dispositions.

#### MIR-COLLECTION-LITERAL-CONSTRUCTION-D1

Accepted [construction design](../design/collection-literal-construction-ssot.md) owns the finite inventory and task order; source Facts/Recipe and runtime proof remain open.

#### Collection allocation checkpoints

Closed: named target44c10a5cec/intrinsic Array9b5ef392cc; owner SSOT/Git retain evidence.

#### MIR-SCRIPT-INSTANCE-RUNTIME-COMPLETION-I0

Closed at `278b519d1f`: intrinsic Array identity and single publication; Array/
Pair EXE+OBJ30 green. [Owner SSOT](../design/collection-literal-construction-ssot.md) and Git retain the receipt.

Known baseline debt reproduced at parent `95f400280d` with identical quick profile:
`program_root_work_plan` parent6pass6fail, current6pass4fail; two retired second-demand
fixtures are deleted. Remaining failures (all `cohort-missing`):
`app_partition_preserves_source_order_and_runtime_retention`,
`script_partition_keeps_static_boxes_out_of_deferred_work`,
`selected_constructor_sources_keep_parser_key_order_and_skip_nonfunctions`,
`selected_script_window_keeps_callable_transfer_at_its_original_ordinal`.
`normal_script_runtime_work` parent/current3pass4fail at retired raw ingress:
`selected_generic_script_box_keeps_full_legacy_callable_parity`,
`selected_generic_static_script_box_keeps_legacy_callable_parity`,
`selected_script_box_methods_match_legacy_without_duplicate_functions`,
`selected_script_catalog_failure_discards_candidate_and_reuses_compiler`.
No source gate was relaxed and no baseline test ignored. The prior current-change
shadow DuplicateKey and static-negative syntax error are fixed and rerun green.

#### Script typed Array source checkpoints

Source retention landed b129f066c0/efbc22b14b; V4 feature repair27a4e1ed78.
Nonplugins errors13->11 are not whole-build green. Git and the
[feature owner](../design/constructor-lifecycle-llvm-lowering-ssot.md#feedback-reconciliation-follow-ups-2026-09-08) retain exact baseline inventory/evidence.

#### MIR-SCRIPT-ARRAY-ROOT-TERMINAL-COSEAL-I0

Closed at87a689b013: Return/Home co-seal and Local progress; owner SSOT/Git retain evidence.

#### MIR-SCRIPT-ARRAY-ROOT-VALIDATION-HANDOFF-I0

Landed at c1ab5fd057: exact bindings survive both finishing consumers/optimizer.
Git and the construction owner retain evidence; typed C remains stopped.
Known baseline debt reproduced at parent87a689b013, identical locked quick build
and `normal_default_root_catalog_lifecycle_tests::artifact_validation_` filter:
parent/c991da509a2pass2fail. Exact failing tests are
`artifact_validation_rejects_exact_read_drift_and_birth_reentry` (finished-node)
and `artifact_validation_rejects_terminal_add_operand_drift` (expected token).
Their source and assertions are unchanged; no whole-lib green claim.

#### MIR-SCRIPT-ARRAY-FINAL-ARTIFACT-HANDOFF-I0

Closed at `05945284bb`: retained ScriptArray handoff; selected ownership/mixing
edges retired. The [artifact owner](../design/collection-literal-construction-ssot.md#root-neutral-finalized-artifact-retention-slice) and Git retain evidence.

#### MIR-SCRIPT-ARRAY-LIFECYCLE-PHYSICAL-LOWERING-I0

Closed at `7be46e37e4`: Recipe -> Invoke/frame/cleanup/Return -> typed Stop;
old standalone emission/binding/direct Return retired. Focused41 and Array/Pair
EXE/OBJ pass; runtime/C successor remains open. Owner SSOT/Git retain evidence.

#### MIR-ARRAY-PRIMITIVE-RESULT-PRESERVATION-I0

Closed at `4cc9f2b697`: Result store/raw bool projection; Array51/hostStop7 green.

#### MIR-ARRAY-FALLIBLE-SHARED-OWNERSHIP-D0

User accepted stable Rust/fatal-OOM separation; mandatory fallible Arc withdrawn.
Returned-Fault cleanup remains required; [ordered tasks](../design/collection-literal-construction-ssot.md#accepted-runtime-to-c-task-order) own the successor series.

Array dependencies closed: atomic append `23757dfeb1`, checked native ABI `32ed589930`, final input/session `c14cc1e140`; collection owner/Git retain focused evidence. Parent kernel cfg failure repaired at `7fffbd8a23`. Final C execution below supersedes their earlier typed Stops.

#### MIR-SCRIPT-ARRAY-C-EXECUTION-I0 — closed

`feaaa5d5e8` / `b61aef93ec`: real OBJ/EXE switch and finite retirement verified; host39, allocation Fault8, malformed29, ordered range/Fault cleanup/report/dispose, parent/pointer guards pass. Pair/untyped and CLI regressions remain green. No fatal OOM or whole-MirBuilder claim.
Boundary: selected normal preparation -> retained Script -> bound final input -> V4 OBJ/EXE terminal; both real callers included. Selected stripping/observer/projection/Stop/pending edges retired; shared generic/module-only/compatibility and other families excluded and still live. Inventory Exhausted, open/reopened blocker0. Git owns full audit/evidence.
Known baseline debt at parent24cfe2cf7c: old inplace/raw-Loop, ingress-schema/stale-card and cataloged-affine-loan/unsupported-row guards. No waiver for current changes; use selected-normal parent guard.

#### CONSTRUCTOR-TERMINAL-RELATION-BOXSHAPE

Closed at `a1d86db262`: one terminal relation through source/Completion/ledger/final handoff; derived result storage/view synchronization retired. Handoff5/control33/Pair EXE+OBJ30 and Bool Fault70 pass; guards pass, source max730/net-32. No source widening; baseline inventory follows.
Baseline comparison at parent `b61aef93ec`, identical Cargo.lock/profile quick: package filter `mir::normal_callable_semantic_package::` is116pass/2fail both before/after; pipeline filter `mir::compiler::normal_default_pipeline::tests::` is21pass/2fail both. Four reds are known baseline debt, not waived current failures; no whole-package green claim.
Package failures: `ordinary_new_coseal::field_reads::tests::terminal_read_rows_retain_alias_sites_and_commit_only_complete_expression` (field+Bool retains1read, expected0) and `brand_catalog_tests::normal_home_completion_observes_suffix_and_does_not_reuse_last_new_prefix` (same mixed suffix cleanup). Owner: `home_new_prefix::return_scalar`; reopen with scalar-source classification work, without widening terminal acceptance here.
Pipeline failures: `published_consumer_runs_once_and_propagates_failure_without_retry` (expected callback failure absent) and `published_consumer_does_not_consume_explicit_compatibility` (`artifact-root-completion-unavailable`). Owner: normal-default callable/compatibility finalization and its test ingress; reopen when that ingress is selected, not by bypassing the artifact gate.


Docs reconciliation closed at `980c99ff2c`: constructor2594->440, collection1155->400 lines; README synchronized, links/guards pass. Contracts, open families and baseline evidence retained.

Storage wire-tag BoxShape closed at `dee0d91fd3`: value1 retained; Rust physical9, C parser storage0/2 rejection, V4 Pair30/Fault70 and14 malformed cases pass; source max676, guards pass.

#### MIR-CALL-PUBLISHED-CANONICAL-COMPATIBILITY-RESIDUAL-D0 — decision closed

At `dee0d91fd3`, two read-only audits plus parent source/host checks confirmed the next existing Map obligation; no code/build/runtime evidence was produced. No blanket Method/Constructor Stop or legacy-plan promotion is authorized.
Boundary: published static view/frame -> entry and same-module C walkers -> typed/compatibility terminal; includes Method/Call-Constructor/Named/intrinsic allocation. Excludes closed Global/Extern inventories, V4, general source census and other backends. Rowless Method and Named allocation retain compatibility; explicit intrinsic Array row absence rejects. Call-Constructor/Math keeps its prior disposition.
Map literal remains CutoverBlockerOpen: raw and Core emit Named(MapBox)+birth; canonical set has no legacy-only write plan, so allocation-only extension cannot prove populated Map execution. The [Map owner task](../design/collection-literal-construction-ssot.md#map-literal-selected-construction-design) contains exact issuers, physical gap, inventory, deletion and acceptance. No Exhausted/R7 claim.

#### MAP-LITERAL-COMPILER-CUTOVER-I0 — active implementation series

Decision: implement the accepted [v2 frame](../design/collection-literal-construction-ssot.md#versioned-compiler-frame-decision) in three steps: MIR/Core plus frame contract; both C consumers; atomic source/host switch with v1 and six literal-edge retirement. Private V2 rows/query and value consumers, including i1 PHI normalization, are implemented; Map slot destination/transfer is the next design boundary before runtime retention. Literal issuers stay unchanged until consumers are ready.
Source authority + canonical issuer: existing literal/child and canonical key/ordinal owners. Existing published frame owns pre-JSON representation and ABI projection; body remains the sole operand/CFG graph. No new semantic receipt or second instruction graph.
Non-authority: old i64/origin, plan presence and incidental linkage cannot supply kinds or admission. Role uses existing program/definition/leaf/launch owners, not a Rust name classifier. Accepted parse-once session order and alias-demand obligations are in the [Map owner](../design/collection-literal-construction-ssot.md#allocation-preflight-ownership-and-ordered-implementation).
Fail-fast boundary: missing/malformed/dangling projection, changed original inputs, stale compatibility ingress or residual rows reject before artifact. OriginalValue itself demands an original producer; Map-only Float gets no fabricated legacy lane.
Smallest next slice: existing V2 static-host same-invocation query/frame/compile connection per collection SSOT; retain config/input and close lifetime, no V1 retry. Generic copy/boxed Map does not require checked Home issuance. Local queues11/14 closed (lookup2/parity6/source13/EXE26/completion12). Raw/Script/Core/static-host/V1/six-edge and alias scans remain open; no R7 claim.
Use jobs4 locked quick focused Rust, selected C build/proofs when changed, existing pointer/corridor guards; sources stay below800.
Checkpoint: Map Completion retains direct prior-Home transfer, replacement/Fault order and complete/unavailable demand; common prepare_install stops before catalog/body effects. Package126/Completion33 pass; root11 pass/six parent3661a9a907 baseline reds with matching causes. New Map root test passes. Exact tests/commands are in OWN-FIELD-CONTAINER-DEST-D0. Prefix early-exit bypasses are removed; source max737. No physical Map execution/intake/cutover claim.
Baseline tests (parent a3ee23938d, identical command): program_json_v0_loader::typed_program_v0_import_compile_uses_published_program_pipeline; hmi_t0_fixtures::{ownership_transport,scalar_suite}_matches_checked_in_hmi_t0_fixture; metadata_and_annotations::source_to_program_json_v0_transports_local_type_annotation_metadata. Exact names/command/logs are retained in OWN-FIELD-CONTAINER-DEST-D0; no fixture refresh or semantic waiver.
Private invocation/query consolidation is implemented. Conditional representation is distinct from execution coverage; pattern success/partial skips remain real output concerns. Complete frame/C consumption and public session/host/source cutover remain open.
Baseline observation: initial Print-based physical input hit missing @.fmt_i64 on parent d28498d591 and current alike (named_allocation_emission_test.py before anchor-call revision; logs /tmp/hakorune-named-parent-print.log). Retained tests use a typed FreeFunction anchor to reach allocation; no Print fix/waiver. Invalid type_id=0 plans likewise stop at existing whole-program layout registration; selector tests cover lazy shadowed-plan reads.
Non-claims: existing runtime ABI evidence covers ABI8/String2/codec38/legacy Map11 only; object destination/lifetime dependency remains open. No compiler/source cutover has landed yet. Physical Map execution is bounded fixture evidence; source-level Map OBJ/EXE and R7 remain open. No blanket non-Map tagged rewrite, private-symbol wrapper or retained production v1 fallback.

## Source and ownership budget
Measured 2026-09-09; flow split updated below, others at 7cc63ab9ea; remeasure before edit.

```text
src/mir/builder/ssa/local.rs                           774
src/mir/passes/simplify_cfg/flow.rs                     414
src/mir/builder/raw_expression_dispatch/mod.rs          754
src/mir/builder.rs                                     752
src/mir/string_dead_text_region_plan.rs                750
src/mir/instruction.rs                                744
src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs 736
```

The deleted raw_invocation_source_transport.rs is not a live budget owner.
Flow split is closed (16 tests): value_uses.rs237, tests116; CFG/PHI stays in flow.
Bound-frame analysis is consolidated (27 tests). Keep registered local.rs's
618-line materialize_local_v1 for behavior-neutral phase extraction, preserving
cache/forwarding/failure order. Neither task creates a new semantic authority.
Instruction/raw-dispatch growth requires deletion/delegation or a prior split.
Quality task order and acceptance live in the ownership task's Map quality queue.
Every touched/new source stays `<760`; `>=800` stops. This is a bounded inventory.
Until Call closure, keep these integration owners because they are current
production seams, not cleanup mistakes:

```text
RawExpressionDispatchPortV1                    sole AST matcher
RawInvocationChildPortV1                       recursive capability root
NormalCallableSemanticPackagePortAdapterV1     package/raw integration seam
CompilationContext                             facade pending finite owner split
FunctionMetadata                               127-row consumer manifest owner
normal_default_root_catalog_lifecycle          install/source orchestrator
```

After Call R7, shrink them in this order:

```text
delete method-tail recovery context
-> localize root_is_app_mode
-> unify recursion-depth scope after old emitter retirement
-> classify raw root capability vs recursive frame
-> split CompilationContext one closed cohort at a time
-> split FunctionMetadata only from its 127-row manifest
-> caller census -> retire -> test home -> compatibility shelf -> barrel shelf
```

## Reduction boundary

Forecasts are not deletion evidence. `json_v0_bridge`, `variable_accum` and source coverage have live families; JoinIR merge and `_p0` proofs require owner/caller and evidence migration before retirement. Git retains earlier LOC estimates.

## Production invariants

```text
named production caller required       = yes
same-series selected old-edge deletion = yes
target decision before arguments       = yes
route selection per request            = exactly 1
RootLower execution per request        = exactly 1
canonical rejection -> retry/fallback  = 0
partial product publication            = 0
source AST clone/reparse                = 0
new semantic target/route acceptance   = 0
source/check file line limit            < 800
```

## Compatibility anchors

These IDs remain here because reusable guards consume them. They are boundaries,
not a copied landed ledger.

```text
CALL-BOX-KIND-ANALYZER-COMPAT-SUNSET-001
  state: active
  owner: CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler
  growth: forbidden

NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001
  state: closed

NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
  state: closed

MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001
  state: closed

RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001
  state: closed

SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001
  state: Parked; Compatibility origin has no canonical replacement owner

STAGE1-DIRECT-POST-MACRO-NONPROGRAM-COMPAT-SUNSET-001
  state: closed
  retired by: STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0

RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
  state: closed
```

## Parked boundaries

- PyVM/reference/Python production activation and non-selected backend
  activation/parity. Compiled Rust core-schema consumers, including WASM and
  non-selected consumers, still require B0/B1 adaptation/isolation/retirement.
- RawCompatibility, bounded GC, Main exact-i64/FullFunction, Brand/special,
  ExplicitExtern, ASTNode::Call, and ArrayElementWrite until their owners select.
- warning/dead-code/chronic measurement work in the cleanup map.
  Its fixed order is `CHRONIC-MEASUREMENT-REFRESH-D0` ->
  `PANIC-SURFACE-CENSUS-D0`/`DEAD-CODE-REMEASURE-D0` ->
  `CHRONIC-MEASUREMENT-EXPECTATION-I0` -> `ASTCLEAN-STALE-GUARD-SUPERSEDE-R0`:
  reproduce/classify first, emit one expectation TSV/refresh guard second, then
  tombstone stale per-script thresholds. Exact 334 is observation, never a
  production ceiling; A④ sentinel/non-growth and D ledger/index rows remain
  parked until their owners/evidence are registered.
- performance, mimalloc, llvmlite, Hako converter, Loop/M8/M9, and physical-type
  follow-ups until `CURRENT_STATE.toml` reselects them.
- broad Context/metadata/port/barrel cleanup until Call R7.

Reopen only on a selected current row, a new production caller, or an accepted
owner-specific Decision. Parked code/tests never grant implementation permission.

## Detached territory audit queue (2026-08-27)

The 16-territory read-only audit produced the following named follow-ups. They
are taskized here without changing the selected reference-child I0; no row below
authorizes code until `CURRENT_STATE.toml` selects it.

1. `MIR-CSE-SAME-BLOCK-STATS-DETERMINISM-R0` — landed at `25ab8fb58`.
   `src/mir/passes/cse.rs` now keeps expression maps per basic block, so a
   sibling block cannot reuse a non-dominating value, and its elimination
   counter increments only for an actual `Copy` rewrite. The same-block
   positive case, sibling-block negative case, and non-numeric no-rewrite
   statistics case are retained by the existing tests. This is outside the
   reference-child transport I0; do not reopen it without a new owner or
   counterexample.

2. `CONC-ENV-TASK-SPAWN-OWNER-D0` — compatibility design stop.
   `env.task.spawn` is publicly reachable but currently echoes a clone or returns
   `Ok(None)` without issuing a Future/scheduler task. Its authority is the
   existing `nowait`/Future/TaskGroup contract, not the C-ABI Future route.
   First close it as explicit typed unsupported/arity failure with zero side
   effects; a real spawn implementation needs a separate issuer/ABI decision.

3. `MIR-CALL-V1-FUNC-SENTINEL-R6` — Call schema blocker, not current I0.
   v1 explicit-callee ingress still writes `func=ValueId::new(0)`, which pollutes
   `used_values`/JoinIR remap. The R6 cutover must use the canonical constructor,
   remove the dummy field atomically, and prove Global/Method/Extern/Value inputs
   do not create a false operand. Do not patch parser/schema files during the
   private transport row.

4. `MIR-REFERENCE-LENGTH-MISSING-METADATA-D0` — reference-only semantics choice.
   Hako reference handlers still turn missing/unsupported `length` metadata into
   zero. Decide strict typed reject versus an intentionally retained stub, then
   add valid-size and missing/unsupported negative evidence. This is not selected
   Rust VM behavior and cannot be used to reopen VM production work.

5. `MIR-CALL-RECEIVER-ABI-ROUNDTRIP-R0` — later receiver retirement family.
   The emitter prepends a Method receiver while VM/Hako handlers strip, autoscan,
   or use `args[0]`. After receiver ABI authority is fixed, delete the prepend
   and the six stripping/recovery sites together; preserve the receiver in
   `Callee::Method` and source-only arguments. No partial removal is safe.

6. `PARSER-LOGICAL-OP-LEXICAL-BOUNDARY-D0` — language design consultation.
   `normalize_logical_ops` rewrites source before tokenization, misses single-
   quoted literals, and shifts spans. The tokenizer already owns `&&`/`||` and
   accepts bare `and`/`or`; the open decision is whether word aliases remain
   canonical, compatibility-only, or rejected, plus quote/span guarantees.
   Design-only matrix first; no implementation permission. This is the one item
   that may need an explicit user/Pro language decision.

7. `MIR-COMPILE-TIME-PERF-BASELINE-P0` ->
   `MIR-SOURCE-OBSERVATION-SINGLE-PASS-D0` — measurement before cleanup.
   AST deep clones and repeated environment reads are code-backed candidates but
   hotness/frequency are not proven. Establish a reproducible baseline and a
   finite clone/env census first; only then consider a session snapshot or lazy
   clone removal. Keep this separate from semantic Call and reference transport.

## Guard contract and retirement queue (2026-08-27)

This is the sole successor queue for the existing
`GUARD-SURFACE-CONSOLIDATION-D0`; do not create one card or shell guard per
finding. It does not preempt the selected force-hv1 design stop.

The 2026-08-27 audit's three hygiene findings and the phase2160 accounting
correction are taskized in
`docs/development/current/main/investigations/guard-execution-index-force-hv1-nongrowth-closeout-d0-2026-08-27.toml`.
The RawLegacy C I0 itself is landed. The selected bounded order is
contract-graph freeze -> existing inventory-row ledger check -> navigation
tombstones -> chronic scope -> Rust token scanner (landed)
-> scope reconciliation D0 -> tracked observation receipt -> site-owner map D0 -> one scope-labeled expectation TSV -> closeout
guard co-registration -> phase2160 legacy/support LOC attribution. The
force-hv1 leaf observer stays shell-body-only, no new force-specific TSV or
God guard is introduced, and none of these rows authorizes compiler, VM, or
backend behavior changes.

The landed RawLegacy nested Main C I0 has one queued same-family closeout
hygiene follow-up in
`docs/development/current/main/investigations/runtime-box-rawlegacy-nested-main-retire-i0-2026-08-27.toml`:
the query-before-clone shape is already confirmed; after the guard frontier,
sync the dispatcher/RawInvocation comments, record the no-more-fate-methods
stop line, and perform the finite `lower_static_main_box` caller census. This
follow-up cannot reopen the retired edge or broaden RawLegacy scope.

```text
Decision: keep execution authority in guard_rows.toml plus typed specs, keep
  reverse inventory as a non-authority projection, stop new unregistered public
  guards first, and retire old guards only after successor coverage and
  caller-zero are both observed.
Source authority + canonical issuer: the git-tracked eligible surface, flattened
  guard manifest/spec graph, and owner-reviewed disposition.
Non-authority: raw LOC, filename prefixes, profile names, the hand-written index,
  generated reverse output, historical thresholds, or executable mode alone.
Fail-fast boundary: a new unregistered public guard, duplicate/dangling graph
  edge, stale expectation, missing execution caller, or unproved retirement
  stays red/retained.
Smallest next slice: CHRONIC-MEASUREMENT-SITE-OWNER-MAP-D0;
  the 185-row evidence matrix is complete (A=79, B=46, C=60), A1 pinned-reference resolution is landed, and A2 must classify 697 owner/evidence references and normalize roles/successors before map authoring.
  Keep the tracked map, expectation pinning, stale-guard supersession, and source deletion as separate later stages.
Non-claims: no full registry migration, quick-static activation, bulk chmod,
  compiler behavior change, or grep/count-authorized deletion.
```

Current source-backed observation at `b4edff4c78` is 3,754 tracked check paths;
the finite registry graph has 112 flattened rows. The ratchet-eligible
immediate-child public surface is 2,740 tracked `*_guard.sh` paths, with 19
direct command-target edges and 74 typed wrapper-alias edges (93 mapped when
the edge kinds are kept separate); 2,647 eligible paths are currently
unmapped. `quick-static` has 24 declared rows and no profile caller. These are
different denominators and must not be reported as one coverage percentage;
the profile name is not execution evidence. The 93/2,647 values are the D0
baseline. `GUARD-REGISTRY-RATCHET-I0` landed at `9b49907937`: the current
tuple is now 20 direct targets, 74 typed aliases, 94 mapped paths, and 2,646
unmapped; the PR-only ratchet reports zero new unmapped paths and zero mapping
loss against an explicit base. The four navigation dangling names remain a
separate ledger row.

Required order:

1. `GUARD-CONTRACT-GRAPH-D0` — accepted: the two-plane model, finite public
   eligibility set, direct-command versus typed-alias edge kinds, explicit
   PR-base/absolute-measurement behavior, contract-v1 metadata, and the
   existing six inventory dispositions are frozen. Generated reverse output
   never becomes authority.
2. `GUARD-REGISTRY-RATCHET-I0` — landed at `9b49907937`: the existing
   inventory owner now registers the already-CI-reachable
   `ci_feedback_tier_policy_guard.sh`, rejects new unmapped public guards
   against an explicit PR base, and wires only this structure check to required
   PR CI. It does not run registry member commands or infer `HEAD^`.
3. `GUARD-NAVIGATION-TOMBSTONE-I0` — landed at `595882c065`: the compatibility
   block stayed byte-stable, four owner-reviewed tombstones resolve the former
   dangling names, and the inventory owner rejects missing/duplicate/conflicting
   navigation names without entering `guard_rows.toml` or executing members.
4. `CHRONIC-MEASUREMENT-SCOPE-D0` — accepted: finite panic/dead_code
   boundaries, independent compile-domain/role axes, and the shared
   expectation TSV contract are frozen. It does not authorize deletion.
5. `CHRONIC-RUST-TOKEN-SCANNER-I0` — landed at `8d2bcf0398`: the standalone
   `rust_source_topology` check crate emits a syn/span-based read-only report;
   malformed/unsupported/unknown ranges fail closed. Its counts are not a
   production baseline and no deletion is implied.
6. `CHRONIC-MEASUREMENT-RECONCILIATION-D0` ->
   `CHRONIC-MEASUREMENT-SITE-OWNER-MAP-D0` -> `CHRONIC-MEASUREMENT-EXPECTATION-I0`
   — scope split measured; track the 185-row observation receipt, then freeze
   canonical range/attribute key, provenance, tracked refs, and classification-only
   states before assigning site owners. No TSV, stale-threshold, or source deletion.
7. `ASTCLEAN-STALE-GUARD-SUPERSEDE-R0` — use one token-aware scanner and one
   per-file expectation TSV. Exact-form 334/111 is diagnostic; inclusive
   attribute grammar currently observes 351/126 and is the required D0 scope.
   Remove the 13 obsolete source-wide numeric clauses rather than relaxing them;
   retain living leaf checks, and explicitly supersede ASTCLEAN-007 by
   ASTCLEAN-013 before deleting 007.
5. `GUARD-MANIFEST-MODEL-R0` -> `GUARD-REGISTRY-HEALTH-R0` — BoxShape-consolidate
   manifest loading, define argv-derived executable semantics, and classify the
   44 non-manifest hako-alloc closeout wrappers as register, consolidate, retire,
   or retain. No mass chmod follows from the current 0644 baseline.
6. `GUARD-REVERSE-INDEX-I0` — extend
   `tools/docs/guard_surface_inventory.py` with guard/invariant/path queries and
   forward/reverse edge checks. Migrate two existing rows first. Keep
   `check-scripts-index.md` human-facing and keep its legacy compatibility block
   byte-stable until its callers reach zero.
7. `SOURCE-LINE-BUDGET-CENSUS-D0` -> `SOURCE-LINE-BUDGET-SPEC-I0` — rederive
   target, threshold, kind, and focused caller group; do not freeze the stale
   estimate of 77 guards. Move one MIRBuilder family to the existing typed-spec
   runner, then make old focused guards delegate exactly once before removing
   their inline `wc`/threshold authority.
8. `GUARD-FAMILY-RETIREMENT-R0` — process bounded families only. A guard may be
   removed when its invariant is owned by a named successor or the guarded route
   is physically impossible, all CI/manifest/parent/docs callers are zero, and
   the supersede/retirement edge is recorded. Otherwise it remains
   `unknown_retain`; inactivity and non-registration are not deletion evidence.
9. `QUICK-STATIC-QUALIFY-D0/I0` — finite-classify all 19 rows for side effects,
   latency, and current green status. Only after qualification may the existing
   anti-wiring contract close and the whole profile become a CI entry.

Acceptance for the series is monotonic: new public unregistered guards = 0;
unknown inventory does not grow silently; each selected family loses its local
duplicate expectation in the same slice that gains central coverage; deleted
guards have named successors and caller-zero; generated forward/reverse edges
round-trip; and no family migration changes compiler or test semantics.

The 2026-08-29 Call proof audit is queued without preempting the landed
RawScriptRoot old-edge deletion. The next production decision is the bounded
RawRootMain origin only; run one bounded
`MIR-CALL-GUARD-ACTIVE-SURFACE-PRUNE-R0` series: retire landed lifecycle-phase
handlers to explicit `superseded_by` tombstones instead of making old lanes
replayable at HEAD, keep only the active origin-retirement family, machine-check
that every changed test is covered by a nonzero focused filter, and add the two
missing `SiteCoverageMismatch` negatives. Then return immediately to the next
origin retirement. The disabled `legacy-tests` family is retired at
`bcc9a6ba65`; Git history and the current Method manifest own its evidence.

## Short closed tail

- normal-root identity/forest gates and the installed App Main FreeStatic
  source-keyed affine handoff are closed; this does not generalize the loan to
  the four named compatibility descendants.
- JSON-v0 Call target resolution and Program late target rewrite are closed.
- Callee operand/use/escape/ownership/query projection rows are closed.
- selected optimizer/Rust VM/printer/JSON/native Call terminal prerequisites are
  closed at their owning manifests; PyVM remains outside.
- canonical Call writers through D1X, late callsite target rewrite retirement,
  duplicate projection validation, exact-target child, and the App Main
  package/loan/raw-port handoff are closed. Rust Builder `Method(None)` and the
  shared `Resolved` corridor are retired; Stage1/backend compatibility and the
  final Call schema remain separate open lanes.
- FunctionMetadata owner split is closed at 718 lines; the 127-row consumer
  manifest remains the future sub-owner census authority.
- The published-C dual-consumer BoxShape row is closed in the existing
  published-row `.inc`: exact-site take and Global/dst/arity/numeric-shape
  admission are shared by the two named C consumers, while LLVM emission and
  generic compatibility remain local. The C shim build, 18 focused view tests,
  non-entry typed ingress probe, negative shape/residual/duplicate probes,
  known-red baseline, and active/pointer guards pass. The producer-side Print
  assertion is now folded into the existing lifecycle test (renamed rather
  than added), proving one source-backed builtin row without increasing the
  test count. No new semantic family or standalone guard was added.

Exact commits, test receipts, and per-row counts live in Git and the linked
investigation/archive owners, not in this rolling card.

## Reusable checks

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/run_row_guard.sh --only mir-call-d1b-targeted-variant-split
# manifest row: mir-call-global-target-b0-machine-census
bash tools/checks/mir_call_global_target_b0_machine_census_guard.sh
# manifest: tools/checks/manifests/mir_call_global_target_b0_machine_census.toml
bash tools/checks/mir_call_d1b_selected_normal_duplicate_projection_guard.sh
git diff --check
```

Cargo gates are run only by an accepted fast/closeout row. This guard-only fast
pointer does not turn a green guard into semantic implementation permission.
