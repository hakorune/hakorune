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

## Current execution brief

`GENERAL-FUNCTION-PLAN0-INSTANCE-INTEGER-LOCAL-RETURN0-S0`
— parent `INSTANCE-LOCAL-BINDING0-D0`, T2

```text
Change:
  add InstanceMethodIntegerLocalReturn0 to the cumulative plan vocabulary:
  no parameters; body = untyped Local x initialized by one ordinary Integer
  literal, then Return of exactly x. Select it in the total source classifier
  before the existing single resolver pass. Old authority: none.

Contract:
  this is a dynamic Integer value Recipe, not exact-i64 inference. Co-seal one
  Receiver, one Local(ordinal 0), the initializer site/payload, one terminal
  resolved Local read, and existing completion. Typed LocalSlotContract,
  parameter-fed initialization, reassignment, Binary, and physical lowering
  remain separate owners.

Done:
  literal-return, i64-parameter-return, and Integer-Local-return coexist with
  exact catalog-key coverage and unchanged Main0 receipt; one unsupported
  method rejects the whole set without retry. Reuse and compact existing tests
  and shared normal-source-plan guard; new test/check files = 0; every touched
  source/check file < 800. Stable gate: run_row_guard normal-source-plan0.

Stop:
  typed-Local meaning, parameter flow, assignment/Binary, a second resolver
  pass, partial-plan publication, Builder/MIR/representation authority, a new
  test/check file, or any source/check file reaching 800 lines is required.
```

## Queue to the north star

```text
M2c  closed exact i64 parameter-return variant
M2c+ current local, then reassign / Binary finite binding slices
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

INSTANCE-I64-PARAMETER-RETURN0-S0 / this implementation commit
  total two-family classifier; exact Receiver + Parameter(0) + Return use
  evidence 76/76; production +464, test +62, check +36; max file 791
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
