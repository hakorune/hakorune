---
Status: Active workstream
Date: 2026-07-28
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

## Current

`GENERAL-FUNCTION-PLAN0-INSTANCE-I64-PARAMETER-RETURN0-S0`
（parent `GENERAL-FUNCTION-PLAN0-INSTANCE-SCALAR-BINDING0-D0`, T2）

### Change

```text
VerifiedNormalModuleSourceV1
-> cumulative instance-function plan set
-> existing module function-plan aggregate

add:
  I64ParameterReturn
  exact one parameter declaration spelling "i64"
  body [Return(Variable(the same parameter))]

old authority:
  single-variant-only selection assumption
```

### Contract

各methodを一度だけtotal分類し、選択後にresolver/projectionを一度だけ通す。
receiver `me` は一つのunused lexical binding、parameterはindex 0の一bindingと
一useに固定する。IntegerLiteralReturn、module source/catalog、Main0 receiptを
維持する。Builder/MIR、physical receiver ABI、field/call/new、Ownership、
production callerは増やさない。

### Done

mixed literal/parameter moduleがgreen。non-`i64`、別変数Return、unsupported
methodはmodule全体をtyped rejectionする。新しいtest/check fileは作らず、
既存fixtureとshared guardを統合更新する。

Stable entry:

```bash
bash tools/checks/run_row_guard.sh --only normal-source-plan0
```

### Stop

source clone/reparse、family retry、partial plan、typed return、physical
receiver、field/call/new、またはsource/check file 800行到達が必要ならD0へ戻る。

## Queue to the north star

```text
M2c  current exact i64 parameter-return variant
M2c+ local / reassign / Binary finite binding slices
M2d  field schema
M2e  field read
M2f  field write
M2g  constructorless default construction
M2h  Main-to-instance call
M3   aggregate VerifiedNormalGeneralProgramPlanV1
M4   reuse DraftSeal / Collector / atomic publication
M5   current-normal MirCompileResult parity
M6   Candidate A technical readiness audit
O1-O5 Ownership/View readiness
M7   Candidate A final re-evaluation
M8   atomic normal/default cutover, only when M7 is green
```

M2c+の各行は有限なFacts/Recipe語彙を一つだけ追加する。M3まではproduction
caller 0であり、replacement creditを主張しない。M8だけがselected
normal/default Legacy edgeを切り替える。

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
Ownership/View activation before O1
.hako selfhost MirBuilder/parser migration
unselected cleanliness work
new language semantics
default Raw/Canonical cutover before M7
```

新しいper-row shell guardは作らない。通常gateと詳しいassertionはactive
source/testおよび既存shared guardが所有する。
