# DYNAMIC-FAULT-CUTPOINT-CATALOG-I0

Status: ready implementation; BoxShape only
Date: 2026-08-10
Decision:
`dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md`

## Goal

Derive and retain one private complete catalog of all six fault-authorized
operations from the already-atomic
`VerifiedDynamicFullLoopSemanticProgramV2`.  This closes the incomplete I6/I7
view without opening Home, cleanup, Completion consumption, or physical work.

## Structure

Add a private child module:

```text
dynamic_full_body_recipe/coseal/semantic_program/
  mod.rs
  fault_cut_points.rs
  tests.rs
```

Conceptual private rows:

```text
DynamicFullLoopFaultFamilyV2:
  DynamicLess
  DynamicAdd
  DynamicInvocation

DynamicFullLoopFaultCutPointV2:
  item
  family
  normal_result
```

The semantic-program issuer derives the catalog internally from:

```text
verified Recipe DynamicAdd/DynamicLess rows
+ exact I6/I7 Dynamic call relations already retained by the envelope
```

No caller supplies a Recipe, call item, expected array, source owner, or
Fault family.  The catalog is a private field of the non-Clone semantic
program.  Consumers receive only a borrow-scoped read view; there is no
standalone public product or `into_parts`.

## Exact golden

```text
I1  DynamicLess       -> V5
I5  DynamicAdd        -> V9
I6  DynamicInvocation -> V10
I7  DynamicInvocation -> V11
I9  DynamicLess       -> V13
I15 DynamicAdd        -> V17
```

The order is verified Recipe item order, not enum order, source-role order,
name sorting, or an independently supplied schedule.

## Acceptance

- exactly six rows and exactly the golden item/family/result mapping;
- two `DynamicLess`, two `DynamicAdd`, and two exact Dynamic invocation rows;
- every row names an existing verified Recipe operation and normal result;
- I6/I7 come only from the exact call-relation seal;
- missing, duplicate, foreign, wrong-family, wrong-result, and unexpected
  fault-capable rows reject before any Builder/session effect;
- non-faultable Read/Const/Write/If/Exit rows never enter the catalog;
- the semantic program remains non-Clone and non-splittable;
- structural guards prove no public constructor/`into_parts`/Fault edge;
- module README and `docs/reference/mir/loop-recipe-contract.md` record the
  landed six-site receipt in the same commit;
- focused Dynamic full-body and V2 schema/JoinSig regressions stay green;
- every touched Rust file remains below 800 lines.

## Nonclaims

```text
concrete runtime Outcome/FaultRecord
V10/ch Home classification or installation
cleanup obligations or C-prime DropPlan
Completion consumption / FunctionExit merge
physical layout / Builder / MIR / CFG / PHI
provider/runtime dispatch
production selection / retry / fallback
```

## Focused verification

```text
cargo test -q --lib dynamic_full_body_recipe
cargo test -q --lib typed_schema_v2
cargo test -q --lib join_sig
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
