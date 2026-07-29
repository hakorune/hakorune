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
Current: MIR-INTERPRETER-POST-MACRO-PROGRAM-INGRESS0-D0
Pack:    ROOT-LIFECYCLE0
Ceremony: T1, read-only design audit
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

## Current execution row
```text
Closed: MIR-CLEANUP-EXIT-ADMISSION-SSOT0-I0-R0.
Callers: value Return, void Return, and Throw.
New owner: control_flow::cleanup::ensure_cleanup_exit_allowed_v1(state, kind).
Deleted: ensure_return_allowed and Throw's duplicate cleanup-state predicate.
Preserved: exact diagnostics and admission before Return/Throw observation or effects.
Non-claims: TryCatch transaction, defer completion, Throw completion route, grammar.
Evidence: cleanup matrix, Return parity/order, Throw structural order, exact shared guard, cargo check.
Fallback / retry / reselection: 0.
Current stop: fresh production live-edge census; no next row is preselected.
```
Lambda capture authority and all feature additions remain parked.
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
R33 VM fallback closed; R34 VM keep closed; R35 helper DEL0 closed; R36 public contract D0 closed; R37 public Program admission closed; R38 public root D0 closed; R39 root test evidence closed; R40 host cfgtest AST JSON current

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
