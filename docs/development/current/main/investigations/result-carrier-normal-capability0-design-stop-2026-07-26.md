---
Status: accepted decision
Date: 2026-07-26
Decision: RESULT-CARRIER-NORMAL-CAPABILITY0-prime-r1
Superseded by execution: RESULT-CARRIER-NORMAL-CAPABILITY0-S0
Related:
  - docs/development/current/main/investigations/normal-file-vm0-frontdoor-forge-task-2026-07-26.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/reference/language/types.md
  - src/mir/compiler/raw_root_source_facts/recipe_projection.rs
  - src/mir/compiler/raw_root_eligibility.rs
---

# Normal-file result carrier and rejection authority

## Why Forge0 must stop here

`FORGE-SEMANTIC0-S0` and S1 prove that the production-caller-zero front door
reuses the existing Raw VM-reference lane without adding a compiler, entry, or
status owner. They also expose a boundary that the front door must not repair:

```text
Script scalar / Unit results
  -> current Raw NarrowV1 / VM decode plan

Main return annotation
  -> Raw eligibility Manifest(AppMainMetadata)

Main explicit return
  -> Raw eligibility Manifest(BodyRecipe(UnsupportedStatement))

ordinary top-level function
  -> Raw eligibility Work(UnsupportedWork)

Array / Map / Record / call result shapes
  -> Raw eligibility Work(UnsupportedWork)

New / FromCall result shapes
  -> Raw eligibility Slots(UnsupportedProcessGlobalSlot)
```

All current failures occur before physical opening and publication. That is a
safe boundary, but it is not a coherent **normal-file result-carrier
taxonomy**. In particular, `MirType` has Box/Array/Future/WeakRef vocabulary,
while the NarrowV1 source route and VM decode plan do not decide whether those
are profile exclusions, future carrier capability exclusions, or source
semantic errors.

The front door must not invent that policy by matching AST shapes or type names
again. Doing so would create a second source-result authority.

## Fixed facts

```text
Canonical source semantics:
  return annotation is a source contract, not a MIR hint
  unannotated explicit return is permitted by the language
  source result and process exit status are distinct

Current NarrowV1:
  physical result decode = Unit / Integer / Bool / Float / String
  Bool / Float / String reach the process projection and produce status-70
    typed faults
  Box / Array / Future / WeakRef do not reach a result decode plan
  Null and Void currently collapse to one Raw VoidExpression origin

Forge0:
  production caller = 0
  may observe or retain existing typed rejection
  may not add a dynamic carrier, source rewrite, fallback, or result repair
```

## Resolved questions

### Q1 — first normal-profile carrier boundary

Chosen: **A — Scalar-and-Unit only.**

```text
A (recommended): Scalar-and-Unit only.

  NormalFileNoImportVmReferenceV1 admits only the already-representable
  Unit / Integer / Bool / Float / String source-result set. Composite and
  owner-bearing results are explicit profile/capability exclusions. No
  dynamic carrier is activated in this family.

  The owning Raw source/profile stage issues the typed rejection; the front
  door transports it unchanged.

B: Add a first object/dynamic result carrier now.

  This requires one source-result carrier, VM decode, process projection, and
  failure/ownership design. It is not a Forge0 extension.

C: Treat composite results as process faults after execution.

  Rejected. It requires executing a source shape that has no existing Raw
  physical/VM representation and moves a capability failure after effects.
```

### Q2 — annotation rejection authority in the narrow profile

Chosen: **A — Keep annotations outside NarrowV1.**

```text
A (recommended): Keep annotations outside NarrowV1.

  Any Main/helper result annotation remains a named Raw eligibility rejection
  before physical opening. The normal front door does not reinterpret `: void`
  or non-Void annotations. Main/ordinary annotation admission belongs to
  FUNCTION-EXIT-F1-NORMAL-CAPABILITY0.

B: Add a front-door annotation classifier.

  Rejected unless a new source-profile authority is deliberately selected.
  It duplicates Raw source facts and would make the front door a second
  semantic classifier.

C: Admit `: void` only in NarrowV1 now.

  Requires a new declared-result contract through Main/body/physical exit and
  is a function-exit capability row, not a Forge0 fixture change.
```

### Q3 — Null and Void evidence

Chosen: **A — Keep both out of normal-admission credit.**

```text
A (recommended): Keep both out of normal-admission credit until a source
   provenance decision. Existing Raw representation may remain shared, but
   the normal matrix cannot claim Null equals Void or that it preserves their
   distinct source origins.

B: Declare Null and Void one canonical Unit origin for normal entry.

C: Preserve separate Unit origins through Script result, entry result, and
   diagnostics while retaining a shared physical representation.
```

`B` changes the language/type SSOT; `C` changes source-result evidence and
possibly the VM decode plan. Neither is a safe front-door-only edit.

## Accepted decision

**Q1-A, Q2-A, and Q3-A are accepted.**

This keeps the first normal profile narrowly honest:

```text
front door              = transports one fixed profile
Raw NarrowV1            = owns existing source/eligibility rejection
FUNCTION-EXIT capability = owns Main/ordinary explicit-return admission
SCRIPT-RESULT capability = owns Null/Void source provenance
RESULT-CARRIER capability = owns any composite/owner-bearing result transport
```

## Execution handoff

```text
RESULT-CARRIER-NORMAL-CAPABILITY0-S0
  preserve already-classified Raw rejection facts and issue typed run evidence

-> FORGE-SEMANTIC0-S2
  observe every annotation/carrier row through the front door

-> FORGE-REUSE0-S0
-> FORGE-REUSE0-S1
-> FORGE-G0
-> NORMAL-ENTRY-CUTOVER-D2
```

The accepted S0 task is
`result-carrier-normal-capability0-s0-execution-task-2026-07-26.md`. Dynamic
carrier, annotation admission, and Null/Void provenance remain separate future
capability rows; Forge0 must not add an ad-hoc adapter.

## Non-claims

```text
normal/default cutover
production caller
Main explicit-return implementation
ordinary callable implementation
dynamic or object result carrier
Null/Void semantic equivalence
fallback / legacy compile re-entry
JSON, LLVM/native, REPL, executor, selfhost, or CUT0 activation
```
