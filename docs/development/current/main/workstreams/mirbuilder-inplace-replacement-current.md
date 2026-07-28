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
Current: NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0
Ceremony: T2; one atomic I0/R0 commit
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

## Execution brief

Change:

```text
ModuleBuilderInvocationSessionV1 + owned AST
-> one session-consuming root/catalog lifecycle
-> completed session + MirModule

atomically delete:
  ExistingGeneralModuleCompatibilityV1
  selected session.builder_mut().build_module(ast) edge
close:
  NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
```

Contract:

```text
preserve:
  Program root expansion preflight; non-Program current acceptance
  -> prepare_module
  -> one root-level AST clone
  -> callable catalog seal/install
  -> existing port-aware root lower
  -> finalize_module

typed failure order:
  RootExpansion < PrepareModule < CatalogSeal
  < CatalogInstall < RootLower < FinalizeModule

rejection retains session + source and exposes no retry/recovery terminal
existing finish/result/external-commit policy and explicit compatibility lanes stay unchanged
```

Done:

```text
selected-normal build_module reachability = 0
selected lifecycle caller                = 1
compiler-side session.builder_mut        = 0
general/non-Program parity and failure/reuse evidence = green
existing shared guard/manifests updated; new test/check file = 0
module_lifecycle.rs unchanged; every source/check file < 800
```

Stop:

```text
forwarding facade or selected build_module edge remains
failure order changes, second root clone, reparse, or source split
non-Program acceptance narrows
NarrowV1, Stage-B, second session/publication, retry, or fallback is required
finish/result/commit semantics change
new per-row guard or any edit to 799-line module_lifecycle.rs
```

Implementation boundary:

```text
new sibling:
  builder/normal_default_root_catalog_lifecycle.rs
session-consuming API:
  complete_normal_default_root_catalog_lifecycle(self, ast)
durable products:
  completed lifecycle
  rejected lifecycle with typed stage and existing diagnostic parity
non-claim:
  global MirBuilder::build_module callers = 0
```

## Queue

```text
R0  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-D0 closed
R1  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0 closed
R2a NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0 closed
R2b NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0 current
R2c fresh live-edge census after closeout
R2d later named AST-node responsibility cells; each selected old edge becomes zero
R3  eight-pack ledger + final-pipeline completion conformance

after R3 only:
F0  refresh missing-feature / Ownership / View readiness inventory
F1  resume the existing Ownership taskboard from its read-only readiness gate
F2  Unique Box / ScopedAlias -> callable ABI -> Anchored View
F3  select one later unimplemented feature from the language status index
```

`NORMAL-DEFAULT-NONPROGRAM-ROOT-DESCENT0-D0` is the first census candidate,
not selected authority. The exact live graph after R2b decides the next row.

The old M2c-to-M8 complete-program queue is superseded. Passive assets are
reconsidered only when the selected live edge names an exact consumer.
Source-level Ownership/View and other new language semantics do not enter the
MirBuilder replacement train. Analysis-only views used to observe existing
control flow are not source-language View activation.

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
Preloop Stage-B special production activation
source-level Ownership/View and unimplemented language features until R4
.hako selfhost MirBuilder/parser migration
unselected cleanliness work
new language semantics
default Raw/Canonical cutover before M7
```

新しいper-row shell guardは作らない。通常gateと詳しいassertionはactive
source/testおよび既存shared guardが所有する。
