---
Status: Design stop — MIR-CALL-GLOBAL-TARGET-B0-FINITE-IDENTITY-DECISION
Date: 2026-08-26
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
Call owner:
  - docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
Active card:
  - docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml
Task map:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
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

## Current six-line brief

Decision:
  Canonical Callはtyped structural Global targetへ収束する。現在はB0の有限
  family/issuer/wire/projector設計だけを行い、実装許可は開かない。

Source authority + canonical issuer:
  exact source-site/declaration、finite builtin owner、またはowner-private typed
  ingressがtargetを発行する。`MirInstruction::call`は決定済みtargetを格納するだけ。

Non-authority:
  raw name/arity、catalog key単体、`mir_symbol_projection`、physical symbol、
  `ModuleInvocationBrand`、EffectMask、registry、caller=None、methodize、args[0]、
  optimizer/backend repair。

Fail-fast boundary:
  missing/foreign/duplicate/ambiguous/unsupported target、incomplete observation、
  residual loan、missing receiverはarguments、MIR mutation、package install、wire、
  backend effectより前にrejectまたは明示的ParkedSealed。

Smallest next slice:
  `MIR-CALL-GLOBAL-TARGET-B0-FINITE-IDENTITY-DECISION`のread-only finite census。
  builtin/same-module/runtime-helper/compatibilityごとにauthority、issuer、wire、
  every compiled schema consumer、selected parity terminal、old edgeを一行へ閉じる。

Non-claims:
  typed schema code、FunctionCall observer実装、loan、Method/receiver、JSON/backend、
  performance、Loop/M8/M9、warning/dead-code cleanup、broad crate split。

Census boundary:
  production `Callee::Global` issuer -> optimizer/wire/all compiled core-schema
  consumers; includes builtin/static/runtime-helper/compatibility families and
  selected VM/native semantic terminals. Tests and non-selected backends are not
  semantic authority or new parity targets, but their compiled schema consumers
  require B1 adaptation/isolation/retirement disposition. PyVM/reference production
  activation and typed Extern/Method/Value owners are excluded.

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

`CanonicalGlobalTargetV1`はwire-stableなstructural valueで、最低限Builtinと
SameModuleStaticを区別する。追加familyはB0 censusが必要性を証明した時だけ。
`Legacy(String)`、opaque ID、hidden registry、physical symbol authorityは不採用。

```text
legacy text
  -> owner-private compatibility resolver exactly once
  -> typed target
  -> canonical Call
```

## Current finite state

```text
Global B0
  BuiltinReady
  SameModuleStaticReady
  AdditionalFamilyObserved
  CompatibilityTextReady
  MissingSourceRelation
  ForeignModule
  DuplicateOrCollision
  AliasUnresolved
  WrongNamespace / WrongArity
  UnsupportedForWireOrCompiledConsumer
  TypedRejectBeforeEffect
  ParkedSealedOutsideSelectedBoundary

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

Ordinary `FunctionCall` currently defers at the selected shadow profile gate.
Deferred owner trees do not issue the semantic package, so adding an external
scratch or loan alone would create a product with no lawful lifecycle.

After B0, design this exact transition before any semantic implementation:

```text
profile-gate-adjacent observer-only FunctionCall
  -> record existing site/name/arity
  -> observe arguments in the same traversal
  -> issue no target
  -> allow owner observation to complete
  -> require total disposition before package install
```

No second AST walk, post-Deferred recovery, package-external scratch, semantic
profile widening, or BodyEffect-based target inference is allowed. Existing
brand/site/catalog identity is reused unless a concrete undetectable mispair is
shown; receipt proliferation is not a substitute for evidence.

## Ordered frontier

```text
0. MIR-CALL-GLOBAL-TARGET-B0-FINITE-IDENTITY-DECISION        (now, design only)
   finite target families, source issuers, wire owners, selected projectors,
   alias/collision/arity rules, old-edge disposition

1. MIR-CALL-D1B-SELECTED-FUNCTIONCALL-OBSERVATION-COMPLETION-D0
   observer-only state transition, same-traversal argument observation,
   package completion and install-abort contract

2a. MIR-CALL-TOUCHED-OWNER-SHELF-S0
    behavior-neutral split only for a 760+ owner the next semantic row touches

2b. MIR-CALL-MIRCALL-CALLFLAGS-RETIRE-R0
    replace the live one-stage transport before retiring reader-zero flags

2c. MIR-CALL-JOINIR-SCHEMA-CONSUMER-DISPOSITION-D0
    retire or include each isolated compiled consumer in the schema cutover

3. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE
   choose v1/v0 exactly once; invalid explicit v1 -> v0 retry becomes zero

4. MIR-CALL-GLOBAL-TARGET-B1-CUTOVER
   typed target owner + producer/core/wire/optimizer/all compiled consumer
   adaptation; selected terminals own semantic parity, no String wrapper/reparse

5. MIR-CALL-D1B-SELECTED-FUNCTIONCALL-OBSERVATION-COMPLETION-I0
   delete the selected Deferred edge, complete owner/package issuance, abort
   incomplete disposition before install, and issue no target from observation

6. MIR-CALL-D1B-CATALOGED-SOURCE-RELATION-AND-AFFINE-LOAN-I0
   exact site/owner/catalog co-seal -> non-empty stack-owned loan -> take_once
   -> arguments once -> Call once -> residual zero; direct CatalogedTargeted
   payload deleted in the same cell

7. MIR-CALL-EFFECT-AUTHORITY-E0
   name source-owned effects for every promoted Global family; READ/IO conflicts
   stay CutoverBlockerOpen and cannot be resolved by target transport

8. MIR-CALL-D1B-ALL-LINEAGE-PRE-EFFECT-RETIRE-R0
   six lineages + Unlocated/Relationless become exact target / KnownNonDirect /
   typed reject / ParkedSealed; then caller=None, Resolved, unique/tail,
   target Const, and legacy publication retire

9. MIR-CALL-METHOD-CORRIDOR-R0
   receiver lives only in Callee; args are source args; consume the already
   selected effect authority; delete receiver prepend/strip/autoscan/args[0], Method(None),
   methodize, guard repair, UnknownBox, optimizer and VM recovery

10. MIR-CALL-WIRE-CONSTRUCTION-TERMINAL-R0
    close remaining retained-variant parity, ignored flags/effect defaults,
    Constructor -> NewBox, Closure -> NewClosure/Value, and selected fallback zero

11. MIR-CALL-R6-CURRENT-HEAD-RECENSUS-C0
    writers, func readers, optional Callee/receiver, construction variants,
    sentinels, wire/backend retry, and guards recounted at current HEAD

12. MIR-CALL-CORE-SCHEMA-CUTOVER-R6
    atomically delete func, Option<Callee>, optional receiver, INVALID/0 target

13. MIR-CALL-LEGACY-GUARD-CLOSEOUT-R7
    legacy fixtures move to compatibility ingress; impossible-state guards,
    stale comments, README/reference/current history close

14. MIRBUILDER-POST-CALL-INTEGRATION-R0
    recovery context deletion -> root/recursion state localization -> finite
    CompilationContext/metadata/raw-port/adapter/barrel owner cleanup

15. remaining selected pipeline rows -> final repository convergence audit
```

No later row can be pulled before an earlier authority boundary. Local green,
worker review, textual caller-zero, or schema compile errors are not permission.

## Source and ownership budget

Do not append semantic code to these owners:

```text
src/mir/builder/raw_invocation_source_transport.rs      778
src/mir/builder.rs                                      741
src/mir/builder/normal_callable_semantic_loan_port.rs   710
src/mir/builder/raw_expression_dispatch/mod.rs          706
src/mir/builder/calls/unified_emitter.rs                 711
```

The 778-line transport requires a behavior-neutral owner split before touch.
`builder.rs` and `unified_emitter.rs` are deletion/delegation-only. Target,
inventory, handoff, loan, and recursive capability code goes into small
owner-specific siblings. Every touched/new source stays `<760`; `>=800` stops.

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

## Reduction forecast

Finite read-only census estimates the Call corridor can remove roughly
1,200–1,700 gross source lines after the authority cutover. A separately
verified caller-zero `externals.rs` retirement raises the gross range to about
1,600–2,100. These are forecasts, not acceptance evidence; typed identity and
loan preparation may temporarily add code before old-edge deletion.

The following are not current deletion claims:

- `json_v0_bridge` still has multiple live caller families;
- `variable_accum` and `source_coverage.rs` are production-live;
- JoinIR merge code cannot be retired from text references alone;
- `_p0` proof files require evidence migration, not suffix-based deletion.

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
- performance, mimalloc, llvmlite, Hako converter, Loop/M8/M9, and physical-type
  follow-ups until `CURRENT_STATE.toml` reselects them.
- broad Context/metadata/port/barrel cleanup until Call R7.

Reopen only on a selected current row, a new production caller, or an accepted
owner-specific Decision. Parked code/tests never grant implementation permission.

## Short closed tail

- normal-root T0/C0/R0 and atomic source-backed cutover are closed.
- JSON-v0 Call target resolution and Program late target rewrite are closed.
- Callee operand/use/escape/ownership/query projection rows are closed.
- selected optimizer/Rust VM/printer/JSON/native Call terminal prerequisites are
  closed at their owning manifests; PyVM remains outside.
- canonical Call writers through D1X, late callsite target rewrite retirement,
  duplicate projection validation, exact-target child, recursive compatibility
  shelf, and package-only BridgeReady are closed.
- FunctionMetadata owner split is closed at 718 lines; the 127-row consumer
  manifest remains the future sub-owner census authority.

Exact commits, test receipts, and per-row counts live in Git and the linked
investigation/archive owners, not in this rolling card.

## Reusable checks

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/run_row_guard.sh --only mir-call-d1b-targeted-variant-split
bash tools/checks/run_row_guard.sh --only mir-call-d1b-cataloged-affine-loan-lifecycle
bash tools/checks/mir_call_d1b_selected_normal_duplicate_projection_guard.sh
git diff --check
```

Cargo gates are run only by an accepted fast/closeout row. This design-stop
card does not turn a green guard into implementation permission.
