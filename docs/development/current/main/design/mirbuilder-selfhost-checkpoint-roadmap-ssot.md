---
Status: SSOT
Date: 2026-06-25
Scope: MirBuilder Rust-to-Hako selfhost checkpoint roadmap.
Related:
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
---

# MirBuilder Selfhost Checkpoint Roadmap

This file is a small roadmap. It is not a landed-history ledger and does not
replace the task-order SSOT.

## Three Endpoints

```text
1. Artifact selfhost
   Generated .hako artifacts run together without Rust adapter fallback.

2. Mainline selfhost
   Some normal compiler routes explicitly select generated/native .hako.

3. Source selfhost
   Compiler meaning and edit authority move from Rust to native .hako.
```

Current work is still in the Artifact selfhost lane. Single-family artifacts
are strong enough to expose integration edges, but the composed execution graph
is not closed yet.

## Current Transition

The work is moving from:

```text
Rust family
  -> generated .hako
  -> standalone AOT green
```

to:

```text
multiple generated families
  -> same module
  -> direct calls
  -> ArrayBox / MapBox / typed object / enum handle returns
  -> downstream generated consumers
```

This is why the current blockers are execution-graph edges, not only source
coverage rows.

## Near-Term Route

```text
1. Same-module ArrayBox return contract
   Close MultiCarrierExitPhi ArrayBox return and restore full matrix green.

2. Newtype ID generator scalarization
   ValueIdGenerator / BasicBlockIdGenerator next and peek.

3. MirBuilder derived context bundle v1
   Compose accepted context families without mainline selection.

4. Minimal MirBuilder execution path
   ValueId creation, BlockId creation, basic block creation,
   Const / Copy / Return emission, and module finalization.

5. DerivedMainline pilot
   Explicitly select generated .hako for a bounded normal route.

6. HakoAdopted decision
   Promote only families whose native .hako source becomes edit authority.
```

## Consultation-Gated Inventory

The following boundary questions should be answered in a design consultation
before any new compiler-library ABI or syntax growth is started:

```text
TypeBox ABI exposure for compiler libraries
host ABI facade for JSON/Text/projector semantics
promotion from lang/src/compiler/lib to lang/src/shared/**
promotion from library helper to language syntax/spec
distribution/package ABI for compiler libraries
hako.buf-backed TextBuilder implementation
```

These are not active implementation tasks yet. They are the points where the
converter bridge would stop being a narrow bridge and start becoming a new
public surface, so they stay consultation-gated.

## Taskization

Treat this roadmap as checkpoint planning, not as the active implementation
queue. The active queue stays in the task-order SSOT.

```text
ARTIFACT-SELFHOST-CHECKPOINT-001
  Complete when generated artifacts can be composed in one execution graph
  without Rust adapter fallback.

MAINLINE-SELFHOST-PILOT-001
  Complete when a bounded normal compiler route explicitly selects
  generated/native .hako.

SOURCE-SELFHOST-ADOPTION-PLAN-001
  Complete when selected families have native .hako as edit authority and
  Rust is frozen as compatibility/reference source for those families.
```

Do not select these checkpoint tasks directly while a concrete semantic red
edge exists. Select the next red edge that moves the nearest checkpoint.

## Progress Signals

The percentages below are planning heuristics, not acceptance criteria:

```text
converter family coverage:
  substantial, but not sufficient for selfhost

artifact selfhost:
  entered; composed execution edges are the active work

mainline selfhost:
  not selected yet

source selfhost:
  future phase
```

Do not use broad coverage count as proof of selfhost. Use explicit composed
execution gates.

Planning horizon:

```text
artifact selfhost visible checkpoint:
  several semantic slices after same-module ArrayBox return

mainline pilot:
  separate follow-up lane after artifact checkpoint

source selfhost:
  later adoption phase, not a current implementation claim
```

## Required Discipline

```text
Do not:
  add callee-name backend branches
  add C shim special cases to hide converter type errors
  use Rust fallback as success
  treat generated artifacts as semantic/edit authority
  promote bundle size as coverage proof

Do:
  close one execution edge per slice
  keep source facts complete
  require body-wide return-contract agreement
  keep task-order below 800 lines
  stop for design selection before opening a new owner
```

## Next Checkpoint

The next meaningful checkpoint is:

```text
full converter matrix green
same-module scalar helper green
same-module ArrayBox helper green
ID generators green
derived context bundle v1 green
```

After that, the project can select the first minimal MirBuilder execution path.
