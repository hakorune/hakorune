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

## Current design stop

```text
Parent:  MIRBUILDER-LIVE-PRODUCTION-RESET0-D0
Closed:  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0
Current: NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0
Ceremony: short D0; production edits parked
```

External consultation packet:
[NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0 question](../investigations/normal-default-root-catalog-preflight0-consultation-question-2026-07-29.md)

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

## D0 brief

Change:

```text
decide one neutral typed handoff for the selected normal root/module lifecycle
production/source mutation = 0 during D0
target old edge:
  ExistingGeneralModuleCompatibilityV1
  -> session.builder_mut().build_module(ast)
```

Contract:

```text
preserve exact order:
  root expansion preflight
  -> prepare_module
  -> callable catalog seal/install
  -> existing port-aware root lower
  -> finalize_module

reuse:
  VerifiedRawRootExpansionV1
  VerifiedSameModuleCallableDeclarationCatalogV1::seal_root
  lower_root_after_callable_catalog_install_v1

retain source on failure; no NarrowV1 or Stage-B authority reuse
```

Done:

```text
one source/root/catalog owner graph
one sibling lifecycle API; module_lifecycle.rs remains below 800
exact failure ordering and selected old-edge delete set
one bounded implementation row selected
```

Stop:

```text
moving catalog seal before prepare_module changes failure precedence
AST clone/reparse or source authority split
Stage-B context or NarrowV1 grammar becomes required
facade-only rename that leaves selected build_module reachability
new per-row guard or editing 799-line module_lifecycle.rs in place
```

Sunset:

```text
sunset_id: NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
owner: ExistingGeneralModuleCompatibilityV1
surface: selected-normal raw root/module -> MirBuilder::build_module(ASTNode)
baseline callers: 1
sunset_row: NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-RETIRE0-I0-R0
retire_when: selected pipeline compatibility/build_module reachability = 0
evidence: caller manifest + MirBuilder lane guard + normal parity/reuse tests
non-claim: global build_module callers = 0
```

## Queue

```text
R0  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-D0 closed
R1  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0 closed
R2a NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0 current
R2b accepted root/catalog handoff deletes the selected build_module edge
R2c later named AST-node responsibility cells; each selected old edge becomes zero
R3  NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-RETIRE0-I0-R0
R4  eight-pack ledger + final-pipeline completion conformance

after R4 only:
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
