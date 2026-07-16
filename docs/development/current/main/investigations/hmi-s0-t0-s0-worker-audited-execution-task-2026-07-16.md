---
Status: S0 closed; P0-EMIT0 is next
Date: 2026-07-16
Decision: finish S0 through BoxShape cleanup, one post-finish publisher, and a conservative value-use law
Parent: hmi-s0-t0-whole-document-seal-hardening-task-2026-07-16.md
Scope: disconnected HMI-S0-T0-S0; production callers and execution remain zero
---

# HMI-S0-T0-S0 worker-audited execution task

## Outcome

Three read-only workers inspected the live S0 worktree from structure,
`.hako` lifetime/API, and fixture/guard perspectives.

```text
focused S0 execution:
  green

current WIP:
  useful
  not landable

next work:
  BoxShape cleanup inside the existing S0 semantic row
```

The public row order remains:

```text
HMI-S0-T0-L0
  -> HMI-S0-T0-S0
  -> HMI-S0-T0-P0
```

The names below are implementation checkpoints only. Do not create a new
current-state row for each checkpoint.

```text
S0-BS0
  -> S0-PUB0
  -> S0-VALUE0
  -> S0-I1
  -> commit and push

P0-EMIT0
  -> P0-MUT0
  -> P0-G0
  -> commit and push
```

## Current blockers

```text
source debug prints:
  3

stale temporary constructor probe:
  present and failing

Verified function-view construction:
  happens before whole-document finish

constructor guard:
  disagrees with constructor locations

instruction_shape.hako:
  646 lines and multiple responsibilities

cross-block non-PHI use:
  dominance is not proved

producer drift guards:
  opcode subset missing
  opaque root-field projection missing
```

Passing the current handwritten fixture does not waive these blockers.

## S0-BS0 — responsibility split

Behavior and accepted grammar delta: zero.

Create or extract these owners:

```text
seal/results.hako
  document/function result products

seal/function_context.hako
  typed function seal context

seal/instruction_facts.hako
  terminator/value-use/value-definition/ownership-site facts

seal/instruction_contract.hako
  opcode and exact field contract

seal/instruction_inventory.hako
  block scan, PHI prefix, terminator final, fact aggregation
```

Rules:

```text
generic MapBox string context growth:
  forbidden

second opcode vocabulary:
  forbidden

instruction_shape facade:
  allowed only as a thin compatibility facade during the split

document coordinator target:
  <= 180 lines

each instruction/context owner target:
  <= 350 lines

source/check hard limit:
  < 800 lines
```

## S0-PUB0 — one publication owner

Treat every `VerifiedHmi*View` construction as publication.

Add:

```text
view/publication.hako
```

It is the only file allowed to construct:

```text
VerifiedHmiDocumentView
VerifiedHmiFunctionView
VerifiedHmiBlockView
VerifiedHmiInstructionView
```

Construction occurs only after:

```text
all function seals
all CFG/value/scalar/PHI/ownership checks
whole-document correspondence finish
```

View files expose storage initialization and exact attachment methods, but
must not construct nested Verified views.

Retain:

```text
no-argument birth()
explicit initialize(...)
view-owned MapBox/ArrayBox storage
root JsonNode anchor held by the document view
direct imports for every named view type
```

Remove:

```text
tools/hako_shared/hmi/tests/view_constructor_probe.hako
all [hmi/s0-debug] source prints
progress-only S0 success prints
```

Required failure fixture:

```text
two functions
first function valid
second function invalid

expected:
  document/function/block/instruction view construction count = 0
```

## S0-VALUE0 — conservative value-use law

Do not add a dominator authority in T0-S0.

Admit:

```text
ordinary non-PHI use:
  function parameter
  OR same-block earlier instruction definition

PHI incoming:
  function parameter
  OR value defined in the named predecessor before its terminator
```

Reject:

```text
ordinary non-PHI use of another block's instruction result
PHI value from a block other than its named predecessor
same-block use before definition
```

Required fixtures:

```text
same-block prior definition:
  pass

same-block use before definition:
  reject

parameter used in later block:
  pass

predecessor-local value used by ordinary instruction:
  reject

predecessor-local value used by matching PHI input:
  pass

PHI value from unrelated block:
  reject
```

General SSA dominance remains a later row and must not be claimed.

## S0-I1 — close and publish milestone

Seal order:

```text
strict parse
  -> root envelope
  -> function envelopes
  -> instruction inventories
  -> CFG
  -> value inventory
  -> scalar/PHI
  -> ownership transport
  -> whole-document finish
  -> one view publisher
```

S0 closeout counters:

```text
unconditional source prints = 0
temporary probes = 0
Verified views before whole finish = 0
Verified view constructor owner files = 1
production/external HMI callers = 0
execution state/handler files = 0
fallback/retry/V0 conversion = 0
source/check files >= 800 lines = 0
```

Additional fixtures:

```text
non-lowest entry block:
  pass

unreachable block with reachable=false:
  pass

transported reachable mismatch:
  reject

branch then == else:
  successor set follows exact producer deduplication law
```

Milestone:

```text
commit:
  feat(hmi): seal bounded MIR JSON V1 profile

push:
  required before P0
```

## P0 packet

### P0-EMIT0

Rust constructs the minimal `MirModule`, calls `build_mir_json_root`, and
compares exact serialized bytes with checked-in fixtures. `.hako` reads those
same bytes.

Required fixtures:

```text
scalar CFG:
  Const i64/Bool, Branch, Jump, Phi, Add, Copy, Return

scalar supplement:
  Sub, Mul, Div, Mod, Bool return, no-value return, multi-function

ownership:
  borrowed WidgetBox parameter
  CopyOwned to owned WidgetBox
  DestroyOwned
  no-value return
```

Handwritten `root()` and `plan_names()` helpers cease to be positive
authority.

### P0-MUT0

Complete the parent card's mutation matrix. Every rejection proves:

```text
accepted = false
document view = null
Verified view constructions = 0
execution/register/heap effects = 0
Rust fallback/retry = 0
```

### P0-G0

Extend the existing manifest-backed `hmi-t0-authority` guard. Do not add a
second shell guard.

It must prove:

```text
strict parser selector = 1
compatibility/fallback/retry/conversion = 0
constructor owner file = view/publication.hako
foreign constructor callers = 0
root_for_seal consumer = document_seal only
opcode subset = HMI inventory projection
21 opaque root arrays = Rust emitter projection
handwritten positive authority = 0
Rust emitter fixture freshness owner = 1
production/external callers = 0
runtime state/handlers = 0
source debug/probes = 0
source/check files < 800
```

Milestone:

```text
commit:
  test(hmi): prove producer-backed T0 seal parity

push:
  required before HMI-S0-V0 selection
```

## Validation order

Always disable the import-insensitive EXE cache for focused `.hako` runs:

```bash
HAKO_EMIT_EXE_CACHE=0 tools/bin/hako --backend mir --verify \
  tools/hako_shared/hmi/tests/l0_contract_test.hako
HAKO_EMIT_EXE_CACHE=0 tools/bin/hako --backend mir --verify \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako

HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/l0_contract_test.hako
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako

HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/l0_contract_test.hako
HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako

bash tools/checks/run_row_guard.sh --only hmi-t0-authority
bash tools/checks/run_row_guard.sh --only json-native-parser-authority
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

P0 additionally runs the Rust emitter fixture equality tests and full mutation
harness before the quick gate.

## Stop conditions

Stop and return to design review if any step requires:

1. A second JSON grammar/tree or decoded MIR instruction schema.
2. Verified view construction before whole-document finish.
3. More than one Verified view constructor owner file.
4. CFG reparsing raw terminators after instruction admission.
5. General cross-block SSA use without a dominator proof.
6. A second handwritten opcode/root-field SSOT.
7. Rust fallback, retry, V0/compact conversion, or runtime discovery.
8. Product callers, execution state, opcode handlers, or BoxRef execution.
9. Float, String, MethodCall, another backend, or broader MIR JSON admission.
10. A source/check file reaching 800 lines.

## Claims

After S0 only:

```text
one disconnected strict whole-document seal
one post-finish bounded-view publisher
exact T0 instruction/CFG/value/scalar/PHI/ownership transport admission
zero execution/product/fallback authority
```

After P0 additionally:

```text
the admitted carrier is reproduced by the current Rust MIR JSON V1 emitter
producer and HMI inventories drift fail-fast
the complete mutation matrix publishes no partial view or execution state
```

Do not claim all MIR JSON V1, general SSA dominance, ownership execution,
interpreter cutover, or backend replacement.

## S0 closeout

S0 landed with the worker-selected structure:

```text
typed function context:
  one exact accessor owner

instruction responsibilities:
  contract / facts / inventory split

Verified view construction:
  view/publication.hako only

publication order:
  all function seals
  -> whole-document finish
  -> one publisher call

ordinary non-PHI cross-block instruction result use:
  rejected

parameter cross-block use:
  admitted

PHI cross-block use:
  exact named-predecessor provenance only
```

Closeout evidence:

```text
focused MIR verify:
  L0 green
  S0 green

release MIR interpreter:
  L0 green
  S0 green

debug MIR interpreter:
  L0 green
  S0 green

S0 fixtures:
  non-lowest entry
  unreachable=false
  duplicate branch target deduplication
  second-function failure with null document
  conservative value-use matrix
  scalar/PHI/ownership transport matrix

hmi-t0-authority:
  green

json-native-parser-authority:
  green

current-state pointer:
  green

quick:
  66/66

largest source/check file:
  ownership.hako = 366
  all files < 800

production callers / state / handlers / fallback:
  0
```

The checked-in positive JSON remains handwritten in S0 and is not producer
parity authority. `P0-EMIT0` is therefore the exact next blocker.
