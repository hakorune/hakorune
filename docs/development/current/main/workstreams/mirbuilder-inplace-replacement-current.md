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

## Current design stop

`GENERAL-FUNCTION-PLAN0-INSTANCE-PARAMETER-FED-LOCAL0-D0`

```text
Question:
  should the next finite family be exactly one i64 parameter, one untyped
  Local initialized from that resolved parameter, and one terminal Return of
  the resolved Local?

Reuse:
  existing exact i64 parameter receipt, dynamic Local binding facts, total
  source classifier, one resolver pass, source projection, and completion.

Must decide:
  exact declaration/use cardinality; initializer Parameter-use and final
  Local-use pairing; whether the Recipe stores both BindingRefs and sites;
  whole-module rejection and fresh-source reuse evidence.

Non-claims:
  no exact Local ABI inference, typed LocalSlotContract/LocalContractWrite,
  reassignment, Binary, physical lowering, production caller, fallback/retry,
  Ownership/View, or tenth replacement row.
```

## Queue to the north star

```text
M2c  closed exact i64 parameter-return variant
M2c+ closed integer-literal Local; current parameter-fed Local
M2c+ then reassignment / Binary finite binding slices
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

INSTANCE-I64-PARAMETER-RETURN0-S0 / bdd0812c26
  total two-family classifier; exact Receiver + Parameter(0) + Return use
  evidence 76/76; production +464, test +62, check +36; max file 791

INSTANCE-INTEGER-LOCAL-RETURN0-S0 / this implementation commit
  third cumulative variant; exact Receiver + Local(0) + Integer initializer
  + terminal Local read; evidence 74/74; production +391, test +62, check +8;
  one new source file, no new test/check file, max source/check 799
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
