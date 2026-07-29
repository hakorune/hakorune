---
Status: Active workstream
Date: 2026-07-29
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
Task map:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
---

# MirBuilder In-Place Replacement Workstream

## Goal

最終production authorityを次の一本へ収束させる。

```text
Resolve -> Observe -> Facts -> Recipe -> Verify
        -> Lower -> Seal -> Collect -> Atomic Publish
```

現在のMirBuilderを稼働させたまま、競合する責務ownerを一つずつ交換する。
第二MirBuilder、production consumer 0のroute拡張、Legacy fallbackは作らない。
cell数、pack数、LOCは観測値であり、完成条件ではない。

## Current execution

```text
Parent:  MIRBUILDER-LIVE-PRODUCTION-RESET0-D0
Closed:  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0
Closed:  NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0
Closed:  NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0
Closed:  NORMAL-DEFAULT-NONPROGRAM-ROOT-DESCENT0-D0
Closed:  RAW-NONPROGRAM-PORT-NEUTRAL-EXPR-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR0-D0
Closed:  RAW-NONPROGRAM-AWAIT-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR1-D0
Closed:  RAW-NONPROGRAM-CHECK-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY0-D0
Closed:  RAW-NONPROGRAM-PRINT-ROOT-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY1-D0
Closed:  RAW-NONPROGRAM-NOWAIT-ROOT-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY2-D0
Closed:  RAW-NONPROGRAM-ARRAY-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY3-D0
Closed:  RAW-NONPROGRAM-MAP-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-D0
Closed:  RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY4-D0
Closed:  RAW-NONPROGRAM-GROUPED-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY5-D0
Closed:  RAW-NONPROGRAM-INDEX-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY6-D0
Closed:  RAW-NONPROGRAM-EMPTY-BLOCK-EXPR-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY7-D0
Closed:  RAW-NONPROGRAM-ANNOTATION-FREE-LOCAL-ROOT-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY8-D0
Closed:  RAW-NONPROGRAM-ROOT-PARITY-TEST-SEAM1-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY9-D0
Closed:  RAW-NONPROGRAM-BLOCK-EXPR-COMPOSITIONAL-PRELUDE0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY10-D0
Closed:  RAW-NONPROGRAM-TASK-SCOPE-COMPOSITIONAL-DESCENT0-I0-R0
Closed:  RAW-NONPROGRAM-NEXT-RESPONSIBILITY11-D0
Closed:  NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-D0
Closed:  NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-I0-R0
Closed:  PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-RET0
Closed:  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-D0
Closed:  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-I0-R0
Closed:  REPL-TYPED-PROGRAM-INGRESS0-D0
Closed:  REPL-TYPED-PROGRAM-INGRESS0-I0-R0
Closed:  POST-MACRO-PROGRAM-ADMISSION0-D0
Closed:  STAGE1-DIRECT-POST-MACRO-PROGRAM-INGRESS0-I0-R0
Closed:  NORMAL-DEFAULT-VERIFIED-MAIN-LOWERING-HANDOFF0-I0-R0
Closed:  MIRBUILDER-POST-VERIFIED-MAIN-HANDOFF-LIVE-EDGE-CENSUS0-D0
Closed:  INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0
Closed:  MIRBUILDER-POST-INSTANCE-METHOD-BATCH-LIVE-EDGE-CENSUS0-D0
Closed:  INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0
Closed:  MIRBUILDER-POST-INSTANCE-BOX-LIFECYCLE-LIVE-EDGE-CENSUS0-D0
Closed:  CALL-BOX-KIND-POLICY-SSOT0-I0-R0
Closed:  MIRBUILDER-POST-CALL-BOX-KIND-POLICY-LIVE-EDGE-CENSUS0-D0
Closed:  CALL-NAME-CLASSIFICATION-SSOT0-I0-R0
Closed:  MIRBUILDER-POST-CALL-NAME-CLASSIFICATION-LIVE-EDGE-CENSUS0-D0
Closed:  RAW-MATCH-OWNED-INPUT-SINGLE-USE0-I0-R0
Closed:  MIRBUILDER-POST-MATCH-OWNED-INPUT-LIVE-EDGE-CENSUS0-D0
Closed:  RECORD-HELPER-BODY-INVOCATION0-I0-R0
Closed:  MIRBUILDER-POST-RECORD-HELPER-BODY-LIVE-EDGE-CENSUS0-D0
Closed:  RAW-INSTANCE-METHOD-PARAM-NORMALIZATION-ONCE0-I0-R0
Closed:  MIRBUILDER-POST-INSTANCE-PARAM-NORMALIZATION-LIVE-EDGE-CENSUS0-D0
Closed:  RAW-PORT-AWARE-COMPOUND-EXPR-OWNED-INPUT0-I0-R0
Closed:  MIRBUILDER-POST-COMPOUND-EXPR-OWNED-INPUT-LIVE-EDGE-CENSUS0-D0
Current: VERIFIED-MAIN-STATIC-CHILD-LOWERING-HANDOFF0-I0-R0
Mode:    one atomic T1 I0/R0
```

## Current execution brief

`VERIFIED-MAIN-STATIC-CHILD-LOWERING-HANDOFF0-I0-R0` / parent R70 / T1 /
`MODULE-LIFECYCLE0`

```text
Change:
  Extend VerifiedMainStaticChildV1 with borrowed, already-verified function
  parts and one bounded owned lowering projection. The selected Main terminal
  consumes that projection and dispatches the helper exactly once.

Contract:
  Keep VerifiedMainExpansionV1 as the sole issuer before Builder effects.
  Preserve helper lexical order, symbol/arity, port failure order, root and
  callable-main distinct identities, source projection, and raw Main behavior.

Done:
  build_verified_static_main_box_with_port_v1 contains no FunctionDeclaration
  match, no bare-AST helper payload extraction, and no
  main-expansion/static-child-source rejection. Existing expansion, helper
  failure/order, selected-normal parity/reuse, and lane guards are green.

Stop:
  Return to design if raw Main is rerouted, source is cloned/reparsed, verified
  symbol/arity is rederived, failure order/text changes, Main identities merge,
  or fallback/retry/View/Ownership/new grammar appears.
```

## Latest closeout

```text
RAW-PORT-AWARE-COMPOUND-EXPR-OWNED-INPUT0-I0-R0

implementation commit                       = 1e73b9180f
immediate compound-expression deep clones   = 8 -> 0
Call / QMark / Await / Record owner APIs     = unchanged
child order / diagnostics / RootLower delta = 0
focused Call/QMark/Await/Record tests        = green
general module parity / failure / reuse      = green
release build / current pointer / lane guard = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## Latest closeout

```text
RAW-INSTANCE-METHOD-PARAM-NORMALIZATION-ONCE0-I0-R0

implementation commit                       = 71714556db
instance params normalization calls         = exactly 1
instance param-decls normalization calls    = exactly 1
duplicate capture normalization pair        = 0
normalized-input-only capture terminal      = exactly 1
static capture / route / grammar delta       = 0
focused normalization / constructor tests   = green
depth-three capture / first-failure tests    = green
release build / current pointer / lane guard = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

R1 closeout:

```text
selected normal construction sites = exactly 4
NormalCompileRequestV1 constructors = exactly 4
selected-normal Legacy reachability = 0
candidate / finish / publication    = exactly 1
compatibility build_module edge     = exactly 1
fallback / retry / reselection      = 0
new test/check file                 = 0
```

## Evidence

```text
selected normal constructors
  -> NormalCompileRequestV1
  -> NormalDefaultPublishedPipelineV1
  -> ModuleBuilderInvocationSessionV1
  -> ExistingGeneralModuleCompatibilityV1
  -> MirBuilder::build_module(ASTNode)

selected normal constructors:
  execute_mir_mode
  execute_mir_json_minimal
  LLVM source compiler
  Wasm source compiler

explicit compatibility:
  VM keep/fallback, Stage1, REPL, Program JSON v0, selfhost macro-preexpand
explicit reference:
  VM-Hako and the three VM-reference lanes
definition-only:
  execute_mir_interpreter_mode
```

The shared source-hint wrappers are provenance-blind and remain compatibility
surfaces. NarrowV1 lacks normal imports and general module/callable coverage;
only its source-neutral lifecycle kernels are reusable.

R2b closeout:

```text
selected lifecycle caller            = 1
ExistingGeneralModuleCompatibilityV1 = 0
selected-normal build_module edge    = 0
root-level AST clone                 = 1
typed lifecycle failure evidence     = 3/3
normal parity / failure / reuse      = 4/4
explicit compatibility build_module = 2, unchanged
new source file                      = 1, 292 lines
new test/check file                  = 0
all source/check files               < 800
optional quick gate                  = pre-existing EBNF naming-charter failure
clean efe2c467c2 reproduces the same failure
```

R2d closeout:

```text
shared root classifier                 = exhaustive 57/57
Program owner                          = unchanged
selected recursive-safe kinds          = 5
registered residual kinds              = 51
broad self.build_expression fallback   = 0
selected invocation-port descent       = 1
root-specific raw compatibility edge   = 1
selected failure retry                 = 0
focused tests                          = 7/7
new source file                        = 1, 338 lines
new test/check file                    = 0
largest touched source/check file      = 796
```

R2f closeout:

```text
Await recursive closure                 = selected safe / residual unsafe
selected invocation-port descent        = 1
existing Await completion owner          = 1
selected failure retry                   = 0
selected recursive-safe kinds            = 6
registered residual kinds                = 50
focused tests                            = 8/8
new source/test/check/task file           = 0
largest touched source/check file         = 574
```

R2h closeout:

```text
Check recursive closure                  = selected safe / residual unsafe
same-port eager child order              = sealed
existing Check completion owner          = unchanged
selected failure retry                   = 0
selected recursive-safe kinds            = 7
registered residual kinds                = 49
focused Rust tests                        = 13/13
existing Check surface guard              = green
new source/test/check/task file           = 0
largest touched source/check file         = 582
```

R2j closeout:

```text
safe Print compatibility edge           = 0
selected expression kinds               = 7
selected statement-root kinds           = 1
registered residual kinds               = 48
selected / compatibility terminals      = 1 / 1
direct instruction/span parity           = green, unified on/off
focused tests                            = 6/6
fallback / retry / reselection           = 0
production Rust delta                    = +37
new source/test/check/task file           = 0
largest touched source/check file         = 593
largest relevant source/check file        = 774, unchanged
```

R2l closeout:

```text
safe Nowait compatibility edge             = 0
selected expression kinds                 = 7
selected statement-root kinds             = 2
registered residual kinds                 = 47
selected / compatibility terminals        = 1 / 1
MIR/span/Future/binding/slot parity         = green
focused Rust tests                          = 7/7
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +153
new source/test/check/task file             = 0
largest touched source/check file           = 706
largest relevant source/check file          = 774, unchanged
```

R2n closeout:

```text
safe Array compatibility edge              = 0
selected expression kinds                 = 8
selected statement-root kinds             = 2
registered residual kinds                 = 46
selected / compatibility terminals        = 1 / 1
empty/homogeneous/mixed/nested parity       = green
focused Rust tests                          = 8/8
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +206
new source/test/check/task file             = 0
largest touched source/check file           = 745
largest relevant source/check file          = 774, unchanged
```

R2p closeout:

```text
safe Map compatibility edge                = 0
selected expression kinds                 = 9
selected statement-root kinds             = 2
registered residual kinds                 = 45
selected / compatibility terminals        = 1 / 1
duplicate/nested/unified off-on parity      = green
focused Rust tests                          = 9/9
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +241
new source/test/check/task file             = 0
largest touched source/check file           = 769
largest relevant source/check file          = 774, unchanged
```

## Program-v0 closeout

```text
RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-R0

production / test files              = 317 / 447 lines
existing focused tests               = 4/4 green
selected expression / statement      = 9 / 2 unchanged
registered residual kinds            = 45 unchanged
selected / compatibility terminals   = 1 / 1 unchanged
new test-only Rust file               = 1
production behavior / grammar delta  = 0
shared guard / artifact inventory     = green
fallback / retry / reselection        = 0
```

## Latest closeout

```text
RAW-NONPROGRAM-GROUPED-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0

safe Grouped Assignment compatibility edge = 0
selected expression / statement kinds      = 10 / 2
registered residual kinds                   = 44
selected / compatibility terminals          = 1 / 1
root focused tests                           = 6/6
grouped parity / failure / reuse             = 3/3
existing assignment/raw-port evidence        = green
normal-vs-Legacy non-Program parity          = green
shared guard / artifact inventory            = green
fallback / retry / reselection               = 0
new source/test/check file                    = 0
largest touched source/check file             = 665
```

## Latest closeout

```text
RAW-NONPROGRAM-INDEX-COMPOSITIONAL-DESCENT0-I0-R0

safe Index compatibility edge             = 0
selected expression / statement kinds     = 11 / 2
registered residual kinds                  = 43
selected / compatibility terminals         = 1 / 1
static / generic descent laws               = 0/1 and 1/1 green
StaticDataLoad success-only type evidence   = green
root focused tests                          = 7/7
Array/Map Index full-effect parity          = green
normal-vs-Legacy parity/failure/reuse        = green
shared guard / artifact inventory           = green
fallback / retry / reselection              = 0
new source/test/check file                   = 0
largest touched source/check file            = 675
```

## Latest closeout

```text
RAW-NONPROGRAM-EMPTY-BLOCK-EXPR-COMPOSITIONAL-DESCENT0-I0-R0

safe empty-prelude BlockExpr edge       = 0
selected expression / statement kinds  = 12 / 2
registered residual kinds               = 42
selected / compatibility terminals      = 1 / 1
statement / tail demands                = 0 / 1
nested selected / unsafe-tail partition = green
raw-port MIR/type parity                 = green
normal-vs-Legacy parity/failure/reuse    = green
shared guard / artifact inventory        = green
fallback / retry / reselection           = 0
new source/test/check file               = 0
largest touched source/check file        = 729
```

## Latest closeout

```text
RAW-NONPROGRAM-ANNOTATION-FREE-LOCAL-ROOT-DESCENT0-I0-R0

safe annotation-free Local edge          = 0
selected expression / statement kinds    = 12 / 3
registered residual kinds                 = 41
selected / compatibility terminals        = 1 / 1
typed-array / record special-hook reach    = 0 / 0
root partition tests                       = 8/8 green
existing Local descent/raw/parity suites   = 8/8, 7/7, 6/6 green
standalone lexical-scope diagnostic parity = green
candidate discard / compiler reuse         = green
shared guard / artifact inventory          = green
fallback / retry / reselection             = 0
new source/test/check file                 = 0
largest touched source/check file          = 782
```

## Latest closeout

```text
RAW-NONPROGRAM-ROOT-PARITY-TEST-SEAM1-R0

production Rust delta                = 0
moved fixture body parity            = 6/6 exact
root partition/parity tests          = 8/8 green
normal integration/failure tests     = 8/8 green
selected expr/stmt/residual          = 12 / 3 / 41
selected/compatibility terminals     = 1 / 1
parent/child/shared guard lines       = 482 / 305 / 718
shared guard / artifact inventory    = green
fallback / retry / reselection       = 0
new test-only Rust file              = 1
new check/guard/task file            = 0
all source/check files               < 800
```

## Latest closeout

```text
RAW-NONPROGRAM-BLOCK-EXPR-COMPOSITIONAL-PRELUDE0-I0-R0

safe non-empty BlockExpr compatibility edge = 0
selected prelude responsibilities             = Expr / Print / Nowait / Local
unsafe prelude or tail                         = whole compatibility
existing raw BlockExpr semantic owner          = unchanged
standalone Local lexical-scope failure parity  = green
root partition/parity tests                    = 10/10 green
normal integration/failure tests               = 8/8 green
selected expr/stmt/residual                     = 12 / 3 / 41
selected/compatibility terminals                = 1 / 1
production/parent/parity/integration/guard LOC  = 386/512/374/623/735
shared guard / artifact inventory               = green
fallback / retry / reselection                  = 0
new source/test/check/task file                  = 0
all source/check files                           < 800
```

## Latest closeout

```text
RAW-NONPROGRAM-TASK-SCOPE-COMPOSITIONAL-DESCENT0-I0-R0

safe TaskScope compatibility edge          = 0
empty/non-empty/nested safe partition      = green
safe TaskScope in BlockExpr prelude        = green
existing early-exit / push-body-pop owner  = unchanged
child failure pop-order parity             = green
root partition/parity tests                = 12/12 green
normal integration/failure tests           = 8/8 green
selected expr/stmt/residual                 = 12 / 4 / 40
selected/compatibility terminals           = 1 / 1
production/parent/parity/integration/guard = 413/577/440/646/751 lines
shared guard / artifact inventory          = green
fallback / retry / reselection             = 0
new source/test/check/task file             = 0
all source/check files                      < 800
```

## Closed execution

`NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-I0-R0` — T2, parent
`NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-D0`.

```text
result:
  four selected normal constructors -> one opaque Program admission
  -> one session-owned Program root/catalog kernel
  selected bare-AST/non-Program/generic-root admission = 0
  fallback / retry / reselection = 0

evidence:
  root/catalog lifecycle 2/2; constructor admission 1/1; normal candidate 8/8
  generic Program parity 1/1; raw non-Program partition 12/12
  shared guards, pointer guard, diff check, release build = green

structure:
  module_lifecycle.rs 796 -> 575
  program_root_lowering.rs = 286
  shared guard = 798
  new source/test/check files = 1/0/0
  every touched source/check file < 800
```

## Closed conformance census

`MIRBUILDER-EIGHT-PACK-FINAL-CONFORMANCE0-C0` — T0 conformance census,
replacement credit 0.

```text
verdict:
  selected-normal production chain = Complete
  repository-wide final pipeline   = Residual
  replacement credit               = 0

selected-normal:
  four typed constructors
  -> one Program admission before token/session
  -> one candidate/session
  -> one root/catalog lifecycle
  -> one collector-backed callable batch
  -> one finish/readiness/external commit

ledger reconciliation:
  14 landed production rows backfilled
  SOURCE-NEUTRAL-CALL-RECEIPT = ReuseNeutral closed
  PRELOOP-STAGEB-SPECIAL-ACTIVATION = Delete closed
  test-only seam rows receive replacement credit = 0
```

Eight-pack verdict:

| Pack | Verdict | Exact residual |
| --- | --- | --- |
| `REPLACEMENT-LEDGER0` | Residual | detached Stage-B asset is deleted; active compatibility sunsets remain |
| `DESCENT-SPINE0` | Complete | fixed selected old-edge inventory is physically zero |
| `FUNCTION-STATE0` | Residual | `function_state` / PHI / `variable_map` authority remains distributed |
| `CALL-OBJECT0` | Residual | MethodCall / Call / New / Field / Index and other header-sensitive compatibility surfaces remain |
| `CONTROL0` | Residual | If / Loop / TryCatch / Throw / QMark / Match and related control authority remains |
| `FUNCTION-LIFECYCLE0` | Residual | selected-normal is complete; raw legacy direct function publication remains |
| `MODULE-LIFECYCLE0` | Residual | selected-normal is complete; two production arbitrary-AST `build_module` surfaces remain |
| `COMPILER-RESIDUE0` | Residual | MirCompiler/runtime arbitrary-AST compatibility remains; Stage-B activation is zero |

## Closed execution

`PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-RET0` — T1 detached-asset
retirement, one atomic commit.

```text
Decision:
  Delete the complete caller-zero Stage-B whole-source selector -> activation
  -> carrier -> function-session -> physical-publication closure.

Atomic delete roots:
  compiler:
    legacy_source_selection
    legacy_static_import_snapshot
    legacy_whole_source_request
    legacy_module_activation/**
  mir:
    preloop_stageb_candidate_shell
    preloop_stageb_carrier/**
  builder:
    preloop_stageb_context_install
    preloop_stageb_function_activation
    calls/preloop_stageb_instance_function_session/**
    calls/preloop_located_argument_*
    calls/preloop_located_outer_completion
    calls/preloop_nested_result_*
    calls/preloop_outer_carrier_*
    all dedicated cfg(test) support for those physical owners
  wiring:
    module declarations/reexports
    Stage-B-only module-lifecycle/readiness/install helpers
    Stage-B-only compilation-context helpers
  proof:
    two Stage-B child guards and their aggregate positive assertions

Keep:
  source_call_target/**
  source_instance_result_contract/**
  callable result/catalog and generic method-call receipts
  unified emitter, recursive child ports, generic function session
  ModuleDraftCollectorV1

Measured law:
  production caller                         = 0 -> 0
  detached production-capable asset family  = 1 -> 0
  replacement cell / credit                 = 0
  replacement owner / fallback              = 0

Ledger:
  PRELOOP-STAGEB-SPECIAL-ACTIVATION
  Delete pending -> Delete closed

Evidence:
  exact special-root repository-zero census
  retained source-neutral receipt tests = green
  normal candidate/session focused tests = 8/8 green
  retained method-call focused tests = green
  cargo check --tests = green
  cargo test --lib attempted; pre-existing baseline failures remain
  (one exact edgecfg failure reproduced at clean pre-RET0 HEAD)
  existing aggregate/replacement/pointer guards
  git diff --check
  all source/check files < 800

Hard stop:
  any non-test caller outside this closure appears
  selected-normal or explicit build_module behavior needs an edit
  a retained source-neutral receipt/catalog semantic must change
  a tombstone, alias, forwarding facade, fallback, or new guard is needed
  JoinIR/runtime Stage-B would be touched
```

## Latest closeout

`PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-I0-R0`

```text
production caller moved                    = 1
typed Program admission/lifecycle          = 1
ProgramV0Compatibility origin/constructor  = 0
Program-v0 raw-binding tombstone            = 0
loader compile_legacy edge                  = 0
direct ProgramV0-to-MIR JSON bridge delta   = 0
source hint / Builder imports               = exact / empty
module, metadata, verification, diagnostics = parity green
failure / compiler reuse                    = green
fallback / retry / reselection              = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## REPL closeout

`REPL-TYPED-PROGRAM-INGRESS0-I0-R0`

```text
production caller moved                  = 1
typed Program constructor/caller         = 1 / 1
REPL compile_legacy edge                 = 0
ReplCompatibility Rust symbols           = 0
source hint / Builder imports             = <repl> / empty
repl/quiet/plugin/ContinueLive config     = parity green
MIR/verification/failure/reuse            = parity green
vm-reference build and REPL execution     = green
VM/session/rewrite/auto-display delta     = 0
fallback / retry / reselection            = 0
direct production build_module edges      = 2, unchanged
new source/test/check/task file            = 0
largest touched source/check file          = 799
```

## Latest TryCatch closeout
```text
Decision: Candidate A
Row: RAW-TRYCATCH-FUNCTION-STATE-TRANSACTION0-I0-R0
Ceremony: T2, one atomic implementation/retirement commit
Pack: FUNCTION-STATE0 + CONTROL0

Caller:
  statement_surface ASTNode::TryCatch

New owners:
  PreparedRawTryCatchV1
    -> DisabledCompatibility(try body only)
    -> Enabled(owned try / catches / finally)
  ActiveRawTryCatchFunctionStateV1
    -> exact seven-field success-only transaction

Exact state:
  return_defer_active / slot / target / emitted
  in_cleanup_block / cleanup_allow_return / cleanup_allow_throw

Contract:
  disable/enabled route sampled pre-effect exactly once
  same child port used exactly once
  first catch only, current try/catch/finally order unchanged
  catch body clone = 0
  success restores exact seven fields
  every typed failure restores 0 and preserves current dirty state
  primary String error unchanged
  CFG / ID / type / binding rollback = 0
  fallback / retry / reselection = 0
  grammar / MIR success / result / publication delta = 0
```

Success-only restoration is intentional. Restoring on failure would be a
separate behavior change; the outer candidate session already owns live-Builder
isolation.

### Atomic delete and sunset

```text
delete:
  statement_surface -> cf_try_catch_with_port_v1(raw fields)
  old terminal definition/export
  lower-side disable-route read
  seven saved_* locals and manual restore assignments
  catch body clone
  caller-zero cf_try_catch / MirBuilder facade / fresh Legacy port facade

sunset:
  id: RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-SUNSET-001
  owner: PreparedDisabledRawTryCatchV1
  surface: NYASH_BUILDER_DISABLE_TRYCATCH=1 -> try body only
  retire_when: env definition/read/documented consumers and fixture are zero
  growth: forbidden
```

### Evidence and hard stops

Use module-local tests plus existing integration/shared guards. New test,
check, task, and per-row guard files are zero.

```text
evidence:
  enabled success restores seeded seven fields and preserves MIR
  try/catch failure leaves current inner defer state and stops later bodies
  finally failure leaves current cleanup state and stops exit
  nested success restores outer state, then caller state
  disabled route lowers try only with no transaction/block/catch/finally
  first catch executes once; later catches execute zero
  failed candidate leaves live Builder unchanged; fresh request succeeds

hard stop:
  error-path restore or Drop/RAII restore
  broad FunctionOwnedStateTransactionV1 or non-seven-field capture
  MIR/CFG/ID/type/binding rollback
  cleanup-policy sampling before finally entry
  changed debug timing, catch semantics, or primary error
  clone/reparse, second port, fallback, retry, or old wrapper
  Return/Throw/QMark/If/Loop or Ownership/View/feature work
  compatibility growth/missing sunset
  any touched source/check file >= 800
```

Closeout evidence:

```text
prepared production caller                    = exactly 1
old cf_try_catch terminals/facades             = 0
manual seven-field snapshot/restore authority  = 0
catch-body clone                               = 0
success exact restore                          = green
try/catch/finally failure-state parity         = green
disabled prepare route                         = green
candidate isolation / fresh reuse              = green
fallback / retry / reselection                 = 0
release build                                  = green
quick gate                                     = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
largest touched source/check file              = 774
```

## Latest static Box closeout

```text
Decision: non-Main static Box state authority
Row: RAW-NONMAIN-STATIC-BOX-COMPILATION-STATE-TRANSACTION0-I0-R0
Ceremony: T2, one atomic implementation/retirement commit
Pack: FUNCTION-STATE0

Named caller:
  raw_expression_dispatch
  -> BoxDeclaration
  -> is_static && name != Main && !root_is_app_mode

New owner:
  ActiveRawStaticBoxCompilationStateV1

Exact state:
  variable_map
  TypeContext snapshot
  current_slot_registry
  BoxCompilationContext option
```

The transaction begins after `register_user_box`, captures/installs the four
states in their current order, and restores them only after every sorted
method succeeds. A method failure consumes a typed rejection without restoring,
preserving the current dirty candidate state and primary String error. The
outer invocation session remains the sole unpublished-candidate discard owner.

### Atomic delete

```text
dispatcher saved_var_map / saved_type_ctx       = 0
dispatcher saved_slot_registry / saved_comp_ctx = 0
dispatcher direct BoxCompilationContext install = 0
dispatcher four manual success restores         = 0
transaction begin / complete / reject            = exactly 1
```

The existing registration point, sorted method iteration, FunctionDeclaration
filter, same child port, per-method lowering, draft behavior, and
restore-before-Void order remain unchanged.

### Evidence and hard stops

Use module-local transaction/route tests plus existing candidate reuse and
static-Box parity evidence. New test/check/task files and per-row guards are
zero.

```text
evidence:
  seeded four-state success restores exactly before Void
  method N failure stops N+1 and leaves the current inner state
  zero methods begin/restore once and emit Void
  nested success restores outer transaction then caller
  Main / App-mode / instance / Program-root static routes do not participate
  late failure publishes no candidate and a fresh compiler request succeeds

hard stop:
  restore-on-error or Drop/RAII restore
  whole FunctionLoweringState / whole MirBuilder capture
  reuse of per-function FunctionOwnedStateTransactionV1
  register_user_box movement or rollback
  method order/filter/port/draft/publication change
  Main, App-mode, instance Box, or Program-root static integration
  Match clone cleanup in the same commit
  fallback, retry, grammar, View/Ownership, or feature work
  new per-row guard/test file or any source/check file >= 800
```

Match owned-input clone retirement remains a fresh-census candidate; it is not
bundled with this state-authority replacement.

Closeout evidence:

```text
named production branch                      = exactly 1
four-state begin / complete / reject          = exactly 1
dispatcher saved_* / direct context authority = 0
success exact restore before Void             = green
method-N failure / later-method stop          = green
failure non-restore / primary String parity   = green
static + instance invocation collection       = green
general-module MIR/result parity              = green
outer candidate discard / compiler reuse      = green
fallback / retry / reselection                = 0
release build                                 = green
quick gate                                    = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
shared spine guard                            = binary-only green
  full mode has pre-existing selected_static_sites drift
largest touched source/check file             = 781
```

## Latest deferred static Box closeout

```text
PROGRAM-ROOT-DEFERRED-STATIC-BOX-LIFECYCLE0-I0-R0
Pack: FUNCTION-LIFECYCLE0
Ceremony: T1
```

```text
direct Program-root context and method-lifecycle authority = 0
new consuming owner production calls                       = exactly 1
sorted demand / success clear / method-N dirty failure      = green
general Program parity / candidate reuse                    = green
fallback / retry / reselection                             = 0
new source/test/check file                                  = 0
largest touched source/check file                           = 784
release build                                               = green
quick gate                                                  = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

The owner preserves the existing success-only lifecycle: method failure leaves
the dirty candidate context, skips later methods, and retains the primary
String. Outer candidate discard remains the sole isolation owner.

## Latest static method-batch closeout

```text
NONMAIN-STATIC-BOX-METHOD-BATCH-SSOT0-I0-R0
Pack: FUNCTION-LIFECYCLE0
Ceremony: T0

prepared batch production issuers                   = exactly 2
caller-local non-Main static method dispatch copies = 0
sorted/non-Function/main-name/symbol/arity contract  = green
Program success-clear / method-N dirty failure       = green
raw four-state success / failure                     = green
general Program parity / candidate reuse             = green
fallback / retry / reselection                       = 0
new source file                                      = 1, 89 lines
new test/check file                                  = 0
largest touched source/check file                    = 776
release build                                        = green
quick gate                                           = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Current design stop

`MIRBUILDER-POST-CALL-NAME-CLASSIFICATION-LIVE-EDGE-CENSUS0-D0`.

```text
Read only:
  recount the production graph after call-name policy cutover
  select at most one behavior-neutral responsibility with a named caller
  require same-commit deletion of its competing old edge

Do not:
  infer the next row from deferred Main-helper or record-helper candidates
  add caller-zero routes, compatibility growth, View, Ownership, or features
```

## Latest call-name policy closeout

```text
CALL-NAME-CLASSIFICATION-SSOT0-I0-R0 / T1 / CALL-OBJECT0

decision owner / production consumers        = 1 / 3
Raw admission / Callee class facts           = independent, one total match
old predicate definitions / call edges       = 0 / 0
resolution priority and cross-surface matrix = exact
stable guard transports / Hako parity        = green
fallback / retry / compatibility growth      = 0
route/MIR/result/View/Ownership delta         = 0
new source / test / check files               = 1 / 0 / 0
new policy / largest touched source-check     = 98 / 460 lines
largest relevant source-check                 = 799, unchanged
release build                                 = green
quick gate                                    = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Latest call policy closeout

```text
CALL-BOX-KIND-POLICY-SSOT0-I0-R0
Pack: CALL-OBJECT0
Ceremony: T1

decision owner                              = 1
production consumers                       = 6
resolver-extended / general contexts        = 2 / 4
old classifier definitions and calls        = 0
analyzer compatibility growth               = forbidden
policy/resolver/call-route/parity/reuse      = green
fallback / retry / reselection               = 0
route/MIR/result/View/Ownership delta         = 0
new source file                              = 1, 114 lines
new test/check file                          = 0
largest source/check file                    = 799
release build                                = green
quick gate                                   = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Latest closeout

`INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0` / T1 /
`FUNCTION-LIFECYCLE0` is closed.

```text
authority:
  one PreparedInstanceBoxDeclarationLifecycleV1
  one effectful common prefix
  distinct consuming root/raw method terminals

production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

deleted from both callers:
  register_user_box_declared_fields
  build_box_declaration
  constructor-batch issue/lower
  instance-method-batch issue/lower

preserved:
  field -> metadata -> every constructor -> every method
  metadata/constructor/method first-error dirty prefix
  root exact catalog key and missing-row diagnostic
  raw lookup-free method demand and trailing Void placement
  compatibility owner/sunset = 0
  fallback/retry/reselection = 0
  grammar/result/publication/View/Ownership delta = 0

evidence:
  lifecycle capture tests 14/14
  depth-three constructor/method capture, general Program parity, reuse = green
  route inventory/root/binary guards and release build = green
  quick gate = unrelated pre-existing EBNF compatibility-alias failure

structure:
  new source file = 1, 98 lines
  new test/check file = 0
  largest source/check file = 799
```

`INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0` / T1 /
`FUNCTION-LIFECYCLE0` is closed.

```text
authority:
  PreparedInstanceBoxMethodBatchV1 prepares each lexically sorted
  non-static instance FunctionDeclaration once

production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

durable terminals:
  root -> exact catalog key -> lower_root_instance_method
  raw  -> no catalog lookup -> lower_instance_box_method

deleted:
  both caller-local sorted_method_entries loops
  duplicated filtering, symbol construction, payload cloning, dispatch

preserved:
  build_box_declaration -> constructor batch -> method batch
  lexical order, static/non-Function skip, first-error prefix
  root missing-key diagnostic and raw lookup-free behavior
  grammar/result/publication/View/Ownership delta = 0
  fallback/retry/reselection = 0

evidence:
  exact canonical namespace/symbol handoff, skip matrix, prefix failure
  nested constructor/method order, general Program parity, compiler reuse
  route inventory/root/binary guards and release build = green
  quick gate = unrelated pre-existing EBNF compatibility-alias failure

structure:
  new source file = 1
  new test/check file = 0
  largest source/check file = 799
```

`NORMAL-DEFAULT-VERIFIED-MAIN-LOWERING-HANDOFF0-I0-R0` / T1 /
`MODULE-LIFECYCLE0` is closed.

```text
authority:
  selected Program App terminal consumes VerifiedMainExpansionV1 directly
  for exact Main root source, sorted static children, and callable-main symbol

deleted from selected Program:
  raw Main method-map accumulator and methods.clone()
  second App/Main re-selection and impossible Script fallback
  build_static_main_box_with_port_v1 compatibility-facade edge
  helper re-sort/filter/symbol re-projection

preserved:
  RootExpansion validation precedence
  helper order, first failure, stop-before-later-helper/Main body
  callable-main Omitted/Required policy and Main body exactly once
  explicit raw Main compatibility via one shared body/state kernel
  Main args/state restoration, general MIR/result/publication behavior
  fallback / retry / reselection = 0

evidence:
  verified helper order and helper-N failure stop        = green
  Required selected/compat helper+Main order parity      = green
  general Program MIR/result parity                      = green
  late failure candidate isolation/compiler reuse        = green
  shared root guard / binary-only lane guard              = green
  release build                                           = green
  quick gate                                              = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source/test/check file                              = 0
  largest touched source/check file                       = 799
```

## Previous closeout

`NORMAL-DEFAULT-ROOT-EXPANSION-ROUTE-HANDOFF0-I0-R0` / T1 /
`MODULE-LIFECYCLE0` is closed.

```text
authority:
  VerifiedRawRootExpansionV1 is issued once before prepare_module and borrowed
  through the selected normal Program kernel; is_app_mode is consumed once.

deleted:
  declaration_indexer::has_main_static definition and sole caller
  root_is_app_mode.unwrap_or_else ambient fallback
  second source AST Script/App classification

preserved:
  invalid/duplicate Main RootExpansion precedence
  CatalogSeal/CatalogInstall/RootLower/Finalize ordering
  root_is_app_mode publication for the existing raw observer
  Main/helper/body, catalog/index/static-data, collector/publication behavior
  explicit compatibility, fallback/retry/reselection = 0

evidence:
  Script/App disposition handoff fixture             = green
  invalid Main precedence                            = green
  general Program MIR/result parity                  = green
  late failure candidate isolation/compiler reuse    = green
  shared root guard / binary-only lane guard          = green
  release build                                      = green
  quick gate                                         = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source/test/check file                         = 0
  largest touched source/check file                  = 792
```

## Latest closeout

`INSTANCE-BOX-CONSTRUCTOR-BATCH-SSOT0-I0-R0` / T0 /
`FUNCTION-LIFECYCLE0` is closed.

```text
new owner:
  PreparedInstanceBoxConstructorBatchV1

named production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

deleted:
  both caller-local constructor sort/projection/symbol/clone/dispatch loops
  sorted_constructor_entries helper and its caller-zero test

preserved:
  field registration and build_box_declaration ordering
  ordinary instance-method routes
  Main/static behavior
  stop-on-constructor-N failure and partial candidate state
  grammar/result/publication behavior
  fallback/retry/reselection = 0

evidence:
  lexical order/non-Function skip/first-failure stop = green
  nested constructor and depth-three capture paths   = green
  general Program parity and compiler reuse          = green
  focused shared guard and binary-only lane guard    = green
  full legacy module-draft/headerport guard           = pre-existing stale
    port_aware_function_draft.rs path failure before selected assertions
  release build                                      = green
  quick gate                                         = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source file                                    = 1, 91 lines
  new test/check file                                = 0
  largest touched source/check file                  = 799
```

Lambda capture collector SSOT is a pre-designed candidate only. Main helper
batching and Match hygiene also remain unselected. Feature additions remain
parked until a fresh live-edge census selects one bounded replacement.
Breaking series selected by `MIRBUILDER-PUBLIC-ROOT-API0-D0`:
```text
1 MIRBUILDER-ROOT-TEST-EVIDENCE0-R0 closed (direct callers 15 -> 5)
2 HOST-PROVIDER-CFGTEST-AST-JSON-COMPAT0-RET0 closed (5 -> 4)
3 MIRBUILDER-RAW-OWNER-TEST-EVIDENCE0-R0 closed (4 -> 1)
4 MIRBUILDER-MINIMAL-LIFECYCLE-SMOKE0-R0 closed (1 -> 0)
5 MIRBUILDER-PUBLIC-ROOT-API0-RET0 closed (definition/wrappers -> 0)
```
External consumers are unknown; migration is `MirCompiler::compile*` for Program.

Compatibility sunset:
```text
sunset_id: RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
state: closed
owner / residual surface / root-specific raw edge / execution callers: 0
retired by: RAW-NONPROGRAM-ROOT-COMPAT-RET0-R0

sunset_id: STAGE1-DIRECT-POST-MACRO-NONPROGRAM-COMPAT-SUNSET-001
state: closed
owner and NonProgram Legacy edge: 0
retired by: STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0
```

Closed sunset:

```text
NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
  owner ExistingGeneralModuleCompatibilityV1 = 0
  selected-normal build_module surface       = 0
  global build_module definition/callers     = non-claim
```

Compatibility sunsets:

```text
CALL-BOX-KIND-ANALYZER-COMPAT-SUNSET-001
  state: active
  owner: CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler
  surface: BreakFinderBox / PhiInjectorBox / LoopSSA
  growth: forbidden
  retire_when: analyzer production routes are zero, or one-profile
    classification parity is proven and all callers migrate atomically

RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-SUNSET-001
  state: active
  owner: PreparedDisabledRawTryCatchV1
  surface: NYASH_BUILDER_DISABLE_TRYCATCH=1 -> try body only
  retire_when: environment definition/read/documented consumers and fixture
    are zero, and enabled physical TryCatch is the sole route

RAW-THROW-DEBUG-TRACE-COMPAT-SUNSET-001
  state: active
  owner: PreparedRawThrowV1 completion route
  surface: NYASH_BUILDER_DISABLE_THROW=1 -> env.debug.trace
  retire_when: diagnostic consumers are zero and Throw always uses physical completion

MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001
  state: closed
  production build_module edge: 0
  retired by: MIRCOMPILER-PUBLIC-PROGRAM-ADMISSION0-I0-R0

RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001
  state: closed
  measured build_module edge: src/runtime/mirbuilder_emit.rs = 0
  env.mirbuilder.emit contract: Program(JSON v0) only
  AST JSON rejection: before Builder, no retry
```

## Queue

```text
R0  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-D0 closed
R1  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0 closed
R2a NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0 closed
R2b NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0 closed
R2c NORMAL-DEFAULT-NONPROGRAM-ROOT-DESCENT0-D0 closed
R2d RAW-NONPROGRAM-PORT-NEUTRAL-EXPR-DESCENT0-I0-R0 closed
R2e RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR0-D0 closed
R2f RAW-NONPROGRAM-AWAIT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2g RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR1-D0 closed
R2h RAW-NONPROGRAM-CHECK-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2i RAW-NONPROGRAM-NEXT-RESPONSIBILITY0-D0 closed
R2j RAW-NONPROGRAM-PRINT-ROOT-DESCENT0-I0-R0 closed
R2k RAW-NONPROGRAM-NEXT-RESPONSIBILITY1-D0 closed
R2l RAW-NONPROGRAM-NOWAIT-ROOT-DESCENT0-I0-R0 closed
R2m RAW-NONPROGRAM-NEXT-RESPONSIBILITY2-D0 closed
R2n RAW-NONPROGRAM-ARRAY-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2o RAW-NONPROGRAM-NEXT-RESPONSIBILITY3-D0 closed
R2p RAW-NONPROGRAM-MAP-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2q RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-D0 closed
R2r RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-R0 closed
R2s RAW-NONPROGRAM-NEXT-RESPONSIBILITY4-D0 closed
R2t RAW-NONPROGRAM-GROUPED-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2u RAW-NONPROGRAM-NEXT-RESPONSIBILITY5-D0 closed
R2v RAW-NONPROGRAM-INDEX-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2w RAW-NONPROGRAM-NEXT-RESPONSIBILITY6-D0 closed
R2x RAW-NONPROGRAM-EMPTY-BLOCK-EXPR-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2y RAW-NONPROGRAM-NEXT-RESPONSIBILITY7-D0 closed
R2z RAW-NONPROGRAM-ANNOTATION-FREE-LOCAL-ROOT-DESCENT0-I0-R0 closed
R2aa RAW-NONPROGRAM-NEXT-RESPONSIBILITY8-D0 closed
R2ab RAW-NONPROGRAM-ROOT-PARITY-TEST-SEAM1-R0 closed
R2ac RAW-NONPROGRAM-NEXT-RESPONSIBILITY9-D0 closed
R2ad RAW-NONPROGRAM-BLOCK-EXPR-COMPOSITIONAL-PRELUDE0-I0-R0 closed
R2ae RAW-NONPROGRAM-NEXT-RESPONSIBILITY10-D0 closed
R2af RAW-NONPROGRAM-TASK-SCOPE-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2ag RAW-NONPROGRAM-NEXT-RESPONSIBILITY11-D0 closed
R2ah NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-D0 closed
R2ai NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-I0-R0 closed
R3  MIRBUILDER-EIGHT-PACK-FINAL-CONFORMANCE0-C0 closed: Residual
R4  PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-D0 closed
R5  PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-RET0 closed
R6  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-D0 closed
R7  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R8  REPL-TYPED-PROGRAM-INGRESS0-D0 closed
R9  REPL-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R10 POST-MACRO-PROGRAM-ADMISSION0-D0 closed
R11 STAGE1-DIRECT-POST-MACRO-PROGRAM-INGRESS0-I0-R0 closed
R12 MIR-INTERPRETER-POST-MACRO-PROGRAM-INGRESS0-D0 closed: NoProductionCaller
R13 MIR-INTERPRETER-DETACHED-ASSET-RETIRE0-RET0 closed
R14 BENCH-DETACHED-ASSET-RETIRE0-RET0 closed
R15 INTERPRETER-LEGACY-FEATURE-CLOSURE0-D0 closed: Retire
R16 INTERPRETER-LEGACY-FEATURE-RETIRE0-RET0 closed
R17 MIR-CONTROL-FLOW-DETACHED-HELPERS0-RET0 closed
R18 RAW-NONPROGRAM-VARIABLE-ASSIGNMENT-COMPOSITIONAL-DESCENT0-D0 closed
R19 RAW-NONPROGRAM-VARIABLE-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R20 RAW-NONPROGRAM-VARIABLE-COMPOUND-ASSIGNMENT-COMPOSITIONAL-DESCENT0-D0 closed
R21 RAW-NONPROGRAM-VARIABLE-COMPOUND-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R22 RAW-NONPROGRAM-SAFE-RETURN-ROOT-DESCENT0-D0 closed: Accept
R23 RAW-NONPROGRAM-SAFE-RETURN-ROOT-DESCENT0-I0-R0 closed
R24 RAW-NONPROGRAM-PLAIN-SCOPEBOX-COMPOSITIONAL-DESCENT0-D0 closed: Accept
R25 RAW-NONPROGRAM-PLAIN-SCOPEBOX-COMPOSITIONAL-DESCENT0-I0-R0 closed
R26 RAW-NONPROGRAM-SAFE-THROW-ROOT-DESCENT0-D0 closed: NoProductionConstructor
R27 RAW-NONPROGRAM-ROOT-INGRESS-POLICY0-D0 closed: IndependentSunsets
R28 RUNTIME-MIRBUILDER-AST-JSON-COMPAT-RETIRE0-I0-R0 closed
R29 POST-MACRO-ROOT-CONTRACT0-D0 closed: WholeFileProgram
R30 STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0 closed
R31 SELFHOST-MACRO-PREEXPAND-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R32 VM-HAKO-POST-MACRO-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R33 VM fallback closed; R34 VM keep closed; R35 helper DEL0 closed
R36 public contract D0 closed; R37 public Program admission closed
R38 public root D0 closed; R39 root test evidence closed
R40 host cfgtest AST JSON closed
R41 RAW-TRYCATCH-FUNCTION-STATE-TRANSACTION0-I0-R0 closed
R42 MIRBUILDER-POST-TRYCATCH-LIVE-EDGE-CENSUS0-D0 closed: static Box selected
R43 RAW-NONMAIN-STATIC-BOX-COMPILATION-STATE-TRANSACTION0-I0-R0 closed
R44 MIRBUILDER-POST-STATIC-BOX-LIVE-EDGE-CENSUS0-D0 closed: deferred Program static Box selected
R45 PROGRAM-ROOT-DEFERRED-STATIC-BOX-LIFECYCLE0-I0-R0 closed
R46 MIRBUILDER-POST-DEFERRED-STATIC-BOX-LIVE-EDGE-CENSUS0-D0 closed: method-batch SSOT selected
R47 NONMAIN-STATIC-BOX-METHOD-BATCH-SSOT0-I0-R0 closed
R48 MIRBUILDER-POST-STATIC-METHOD-BATCH-LIVE-EDGE-CENSUS0-D0 closed: constructor batch selected
R49 INSTANCE-BOX-CONSTRUCTOR-BATCH-SSOT0-I0-R0 closed
R50 MIRBUILDER-POST-CONSTRUCTOR-BATCH-LIVE-EDGE-CENSUS0-D0 closed: root expansion handoff selected
R51 NORMAL-DEFAULT-ROOT-EXPANSION-ROUTE-HANDOFF0-I0-R0 closed
R52 MIRBUILDER-POST-ROOT-EXPANSION-HANDOFF-LIVE-EDGE-CENSUS0-D0 closed: verified Main handoff selected
R53 NORMAL-DEFAULT-VERIFIED-MAIN-LOWERING-HANDOFF0-I0-R0 closed
R54 MIRBUILDER-POST-VERIFIED-MAIN-HANDOFF-LIVE-EDGE-CENSUS0-D0 closed
R55 INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0 closed
R56 MIRBUILDER-POST-INSTANCE-METHOD-BATCH-LIVE-EDGE-CENSUS0-D0 closed
R57 INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0 closed
R58 MIRBUILDER-POST-INSTANCE-BOX-LIFECYCLE-LIVE-EDGE-CENSUS0-D0 closed
R59 CALL-BOX-KIND-POLICY-SSOT0-I0-R0 closed
R60 MIRBUILDER-POST-CALL-BOX-KIND-POLICY-LIVE-EDGE-CENSUS0-D0 closed
R61 CALL-NAME-CLASSIFICATION-SSOT0-I0-R0 closed
R62 MIRBUILDER-POST-CALL-NAME-CLASSIFICATION-LIVE-EDGE-CENSUS0-D0 closed: Match owned-input selected
R63 RAW-MATCH-OWNED-INPUT-SINGLE-USE0-I0-R0 closed
R64 MIRBUILDER-POST-MATCH-OWNED-INPUT-LIVE-EDGE-CENSUS0-D0 closed: record-helper body selected
R65 RECORD-HELPER-BODY-INVOCATION0-I0-R0 closed
R66 MIRBUILDER-POST-RECORD-HELPER-BODY-LIVE-EDGE-CENSUS0-D0 closed: instance normalization selected
R67 RAW-INSTANCE-METHOD-PARAM-NORMALIZATION-ONCE0-I0-R0 closed
R68 MIRBUILDER-POST-INSTANCE-PARAM-NORMALIZATION-LIVE-EDGE-CENSUS0-D0 closed
R69 RAW-PORT-AWARE-COMPOUND-EXPR-OWNED-INPUT0-I0-R0 closed
R70 MIRBUILDER-POST-COMPOUND-EXPR-OWNED-INPUT-LIVE-EDGE-CENSUS0-D0 closed
R71 VERIFIED-MAIN-STATIC-CHILD-LOWERING-HANDOFF0-I0-R0 current

after every bounded retirement:
  fresh-census then select one named production edge or detached Delete asset

after final-pipeline Complete only:
F0  refresh missing-feature / Ownership / View readiness inventory
F1  resume the existing Ownership taskboard from its read-only readiness gate
F2  Unique Box / ScopedAlias -> callable ABI -> Anchored View
F3  select one later unimplemented feature from the language status index
```

The old M2c-to-M8 complete-program queue is superseded. Passive assets are
reconsidered only when the selected live edge names an exact consumer.
Source-level Ownership/View and other new language semantics do not enter the
MirBuilder replacement train. Analysis-only views used to observe existing
control flow are not source-language View activation.

R23 removed the root-only safe Return compatibility edge without body widening.
## Closed tail

```text
MODULE-SOURCE0-S0 / e6baf9b4
  exact Main0 + plain instance Boxes + callable catalog co-seal

INSTANCE-INTEGER-RETURN0-S0 / 34ea62cfea
  every instance method -> exact integer-literal Return plan

MAIN0-BRIDGE0-S0 / 7aed7848e6
  retained instance owner + existing Main0 semantic receipts

INSTANCE-CUMULATIVE0-S0 / 7e3144da62
  one source-owning cumulative set; exact ordered key coverage

INSTANCE-I64-PARAMETER-RETURN0-S0 / bdd0812c26
  total two-family classifier; exact Receiver + Parameter(0) + Return use
  evidence 76/76; production +464, test +62, check +36; max file 791

INSTANCE-INTEGER-LOCAL-RETURN0-S0 / adbb737f8a
  third cumulative variant; exact Receiver + Local(0) + Integer initializer
  + terminal Local read; evidence 74/74; production +391, test +62, check +8;
  one new source file, no new test/check file, max source/check 799

NORMAL-SOURCE-PLAN0-PROOF-COMPACTION / 8859caecba
  behavior/grammar delta 0; tests 701 lines, callable guard 755 lines
```

Detailed landed diffs and older cell measurements belong to git history and
the linked task map. They are not copied into this rolling card.

## Fixed packs

```text
REPLACEMENT-LEDGER0  production owner / detached asset accountability
DESCENT-SPINE0       body / statement / expression / argument descent
FUNCTION-STATE0      function facts / PHI / finalization state
CALL-OBJECT0         calls / new / fields / index / collections / lambda
CONTROL0             If / Loop / Match / QMark / cleanup / async
FUNCTION-LIFECYCLE0  draft / collector / function finalize
MODULE-LIFECYCLE0    declaration / catalog / module transaction
COMPILER-RESIDUE0    compiler ingress / old selectors / proof routes
```

新しい発見はこの8 packのいずれかへ入れる。新packは増やさない。

## Parked

```text
source-level Ownership/View and unimplemented language features until the
repository-wide final pipeline is Complete
.hako selfhost MirBuilder/parser migration
unselected cleanliness work
new language semantics
default Raw/Canonical cutover before M7
```

新しいper-row shell guardは作らない。通常gateと詳しいassertionはactive
source/testおよび既存shared guardが所有する。
