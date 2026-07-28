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
Current: RAW-NONPROGRAM-NEXT-RESPONSIBILITY3-D0
Pack:    selection pending
Ceremony: design stop; production edit = 0
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

## Current design question

```text
RAW-NONPROGRAM-NEXT-RESPONSIBILITY3-D0

Candidate A:
  MapLiteral([(String key, PortNeutralExprTreeV1)]*)
  CALL-OBJECT0
  preserve allocation -> birth -> registry
  -> source-order key const -> selected value -> fixed runtime set

Separate:
  QMarkPropagate -> physical Return/CFG/runtime-call CONTROL0

Required:
  recursive expression constructor, not root-shape special case
  same-commit safe residual deletion
  whole-container route; no per-entry mixing or retry
  root partition file remains < 800

Stop:
  production edit or automatic candidate selection
```

Compatibility sunset:

```text
sunset_id: RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
retirement owner: MIRBUILDER-INPLACE-REPLACEMENT0
target card: this rolling card, before R3 completion conformance closes
retire when: this owner's construction/definition, registered residual surface,
and root-specific `-> drive_raw_legacy_expression_v1` production edge are all zero
evidence: later named AST-node replacements plus the shared caller guard
surface may only shrink; adding a residual requires a new design stop
```

Closed sunset:

```text
NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
  owner ExistingGeneralModuleCompatibilityV1 = 0
  selected-normal build_module surface       = 0
  global build_module definition/callers     = non-claim
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
R2o RAW-NONPROGRAM-NEXT-RESPONSIBILITY3-D0 current design stop
R3  eight-pack ledger + final-pipeline completion conformance

after R3 only:
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
