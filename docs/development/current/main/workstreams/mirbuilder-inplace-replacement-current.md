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
Current: NORMAL-DEFAULT-PUBLISHED-PIPELINE0-D0
```

Decision:

```text
cancel:
  GENERAL-FUNCTION-PLAN0-INSTANCE-PARAMETER-FED-LOCAL0-S0
  # semantic implementation was not landed

reason:
  Parameter -> Local -> Return composes existing responsibilities
  complete-program shape is not a replacement unit

freeze:
  normal_source_plan variants = 3
  production callers = 0
  new accepted variants / replacement credit = 0

retain:
  8859caecba behavior-neutral proof compaction
```

## Evidence

```text
compile_with_source*
  -> compile_legacy_request
  -> compile_legacy_candidate
  -> ModuleBuilderInvocationSessionV1
  -> MirBuilder::build_module(ASTNode)

selected normal constructors:
  execute_mir_mode
  execute_mir_json_minimal
  LLVM source compiler
  Wasm source compiler
```

The shared source-hint wrappers also serve compatibility/reference callers, so
their bodies cannot be switched globally. `compile_raw_published_v1` has a
useful one-shot lifecycle, but `NarrowV1` has no normal caller and lacks normal
imports, callable-Main coverage, accepted-corpus coverage, and result parity.

## D0 brief

Change:

```text
read-only census and owner selection
production/source mutation = 0
next implementation token = unset
```

Contract:

```text
one typed normal request owns source identity/imports/config/admission/result
four selected constructors enter one pipeline exactly once
compatibility/reference constructors stay separate
each residual input selects migrated or compatibility owner exactly once before its effect
compatibility owner has no independent ingress/candidate/publication
creation commit registers exact residual surface, caller baseline, and sunset row
registered compatibility surface/ingress authority does not widen
verified/canonical rejection -> compatibility = 0
compatibility rejection -> verified/canonical retry = 0
one candidate/session/finish/publication; retry/reselection = 0
selected normal -> generic Legacy reachability becomes 0 atomically
REPL / Program JSON / VM compatibility/reference behavior does not move
```

Done:

```text
exact caller/provenance matrix
selected owner plus exact compatibility sunset ledger
sunset_id / sunset_row / retire_when / retirement evidence
atomic old-edge delete set
success, exact transport, late failure/reuse, result-parity gates
existing shared guards to extend; new per-row guard = 0
one bounded code-facing row selected
```

Stop:

```text
unknown fifth normal constructor or shared-wrapper global switch
NarrowV1 renamed normal without capability/parity evidence
fallback after a verifier rejection
Program clone/reparse or second compiler execution
production connection before corpus and late-failure parity
compatibility owner without a creation-time exact sunset row
compatibility surface or ingress authority widening after registration
unexplained caller-count increase without exact mapping and D0 approval
new compatibility debt discovered at R3 without ledger correction and D0 return
```

## Queue

```text
R0  current D0: exact live pipeline decision
R1  atomic selected-normal switch + real old-authority deletion + sunset registration
    adapter rename or forwarding facade alone receives no replacement credit
R2  named AST-node responsibility cells on the live edge
    each closes its selected old edge and shrinks the registered residual surface
R3  close the creation-time sunset ledger; compatibility body/raw-AST caller = 0
    newly discovered debt blocks close, corrects the ledger, and returns to D0
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
