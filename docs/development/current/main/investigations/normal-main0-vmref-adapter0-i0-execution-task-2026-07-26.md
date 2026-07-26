---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MAIN0-VMREF-ADAPTER0-I0
Scope: publish one completed canonical Main candidate into the shared source-entry VM-reference owner without adding a runner caller
ceremony_tier: T1 bounded owner/adapter implementation
proof_inventory_before: closed Main TX0 candidate plus closed neutral VM executor and Raw production adapter
new_proofs: one canonical publication/adapter correspondence fixture
retired_or_merged_proofs: none
net_proof_delta: one bounded fixture required for a new source family
sunset_budget: no compatibility or fallback owner
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/normal-main0-tx0-i0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/source-entry-vmref-neutral0-l0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/source-entry-vmref-raw-adapter0-i0-execution-task-2026-07-26.md
---

# NORMAL-MAIN0-VMREF-ADAPTER0-I0

## Outcome

Consume the complete unpublished canonical Main candidate through one explicit
publication transition, then issue the same neutral published-entry and
VM-reference products now used by Raw:

```text
CompletedNormalMainModuleCandidateV1
  -> publish once
  -> PublishedNormalMainInvocationV1
  -> canonical Main family adapter
  -> PublishedSourceEntryInvocationV1<PublishedNormalMainInvocationV1>
  -> PreparedVmReferenceSourceEntryInvocationV1<PublishedNormalMainInvocationV1>
```

This row has one disconnected production-shaped fixture and zero runner/CLI
callers. Actual Main execution parity belongs to the immediately following
`NORMAL-MAIN0-VMREF0-P0`.

## Structure first

Add a dedicated module:

```text
src/mir/compiler/source_entry_vm_normal_main_adapter.rs
```

The completed transaction candidate must not gain a bare `MirModule` consuming
escape. Its only publication terminal creates an opaque published owner:

```rust
pub(in crate::mir) struct PublishedNormalMainInvocationV1 {
    // opaque verified module
    // exact source-header/result/entry evidence
    // completed verification receipt
}

impl CompletedNormalMainModuleCandidateV1 {
    pub(in crate::mir) fn publish(
        self,
    ) -> PublishedNormalMainInvocationV1;
}
```

The published owner may lend the immutable module only through the existing
`VmReferenceExecutablePublishedOwnerV1` boundary. It has no compatibility
conversion and no mutable module escape.

## Exact evidence projection

The canonical adapter consumes the published owner and projects only evidence
already sealed by Main planning/TX0:

```text
membership =
  CanonicalNormal {
    source_owner
  }

target =
  VerifiedNormalMainEntryRelationV1
  physical symbol + exact arity 0

result =
  VerifiedNormalMainThunkResultV1
```

The exact result mapping is:

```text
Unit(EmptyBody)          -> UnitOriginV1::EmptyBody
Unit(ImplicitFallthrough)-> UnitOriginV1::ImplicitFallthrough
Unit(ExplicitVoid)       -> UnitOriginV1::ExplicitVoid
Unit(ExplicitNull)       -> UnitOriginV1::ExplicitNull
Unit(BareReturn)         -> UnitOriginV1::BareReturn
Integer                  -> Integer
Bool                     -> Bool
Float                    -> Float
```

This match must be exhaustive. No default arm, string-name conversion, or
origin collapse is allowed.

The physical Main thunk returns exact Void for every Unit disposition, so the
neutral unit contract is:

```text
PublishedUnitPhysicalContractV1::ExactVoid
```

## Correspondence preparation

Preparation validates before issuing the neutral owner:

```text
completed candidate verification count = 2
source owner matches entry relation source owner
physical target is the sealed main/0 relation
module contains exactly the two TX0 schema members
source and physical symbols/arity match retained evidence
result maps losslessly to one neutral result contract
```

The adapter does not scan the module to select an entry. A bounded membership
check against the retained TX0 schema is allowed; symbol discovery is not.

Forbidden:

```text
module.functions entry search
NYASH_ENTRY
"main" string as route authority
VMValue/MirType/Return result inference
AST/source re-observation
module clone
bare module extraction
mutable module access
Raw profile reconstruction
fallback/retry
```

## Failure retention

```rust
pub(in crate::mir) struct RejectedNormalMainPublishedVmAdapterV1 {
    owner: PublishedNormalMainInvocationV1,
    stage: NormalMainPublishedVmAdapterStageV1,
    error: NormalMainPublishedVmAdapterErrorV1,
}
```

Stages:

```text
Publication
Membership
Target
ResultContract
```

Only:

```text
stage()
error()
discard(self)
```

No owner recovery, repair, alternate profile, Raw retry, or Legacy terminal.

## Implementation order

```text
NM-A PUBLICATION0
  candidate -> opaque published Main owner
  no bare module escape

NM-B RESULT0
  exhaustive FunctionUnitOriginV1 -> UnitOriginV1
  scalar result projection

NM-C ADAPTER0
  exact membership/target/result projection
  one neutral prepare terminal

NM-D EXECUTION-SHAPE0
  implement the exact immutable execution loan trait
  reuse the sole neutral executor

NM-E FIXTURE/G0
  disconnected canonical Main publication/adapter fixture
  production/runner caller zero
```

## Acceptance matrix

Successful adapter projection:

```text
empty body                  -> Unit(EmptyBody)
non-empty fallthrough       -> Unit(ImplicitFallthrough)
return void                 -> Unit(ExplicitVoid)
return null                 -> Unit(ExplicitNull)
bare return evidence        -> Unit(BareReturn)
return Integer              -> Integer
return Bool                 -> Bool
return Float                -> Float
```

Typed rejection fixtures:

```text
source-owner / entry-relation drift
physical target symbol drift
physical target arity drift
schema/member drift
verification-count drift
unsupported or non-lossless result evidence
```

Private forged fixtures may create drift only inside the owning module. They
must not add production constructors or mutable evidence surfaces.

## Structural gate

```text
Completed Main candidate publication terminal          = 1
PublishedNormalMainInvocationV1 producer                = 1
Canonical Main -> neutral published-entry producer      = 1
neutral VM projection canonical Main consumer           = 1 disconnected

FunctionUnitOriginV1 exhaustive mapping                 = 1
Unit origin wildcard/default                            = 0
VMValue/MirType/Return inference in adapter              = 0

module clone / bare consuming module escape              = 0
module inventory entry selection                         = 0
NYASH_ENTRY / execute_module                             = 0
fallback / retry / Raw profile reconstruction            = 0

Raw production behavior delta                            = 0
canonical Main runner/CLI production caller              = 0
default/product route delta                              = 0
all modified/new source/check files                      < 800 lines
```

Extend the existing neutral execution guard where possible. Do not create a
second shell wrapper for one row.

## Immediate continuation

```text
NORMAL-MAIN0-VMREF-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF0-P0
-> NORMAL-CALLABLE-SOURCE0-S0
```

`NORMAL-MAIN0-VMREF0-P0` executes the actual Main matrix through the neutral
executor and proves process status, diagnostics, VM Fault, and compiler reuse.
It does not add a CLI caller.

## Far task order

```text
NORMAL-MAIN0-VMREF0-P0

-> NORMAL-CALLABLE-SOURCE0-S0
-> NORMAL-MAIN-DIRECT-CALL0-S0
-> NORMAL-CALLABLE-MODULE0-A0-S0
-> NORMAL-CALLABLE-MODULE0-R0-S0
-> NORMAL-CALLABLE-MODULE0-TX0-S0

-> NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
-> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
-> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0b
-> NORMAL-FILE-CANONICAL-CORE0-G0

-> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0

-> NORMAL-ENTRY-PRODUCT-BACKEND-D0
-> NORMAL-DEFAULT-CALLER-CENSUS0-P0
-> NORMAL-ENTRY-PROMOTION-D3

-> NORMAL-IMPORT-BUNDLE0
-> MIRBUILDER-LEGACY-FENCE0
-> MIRBUILDER-NORMAL-COMPLETE0
-> MIRBUILDER-COMPLETE0-G0
```

New design consultation is required only if:

```text
the completed candidate cannot publish without a bare module escape
the sealed Main entry relation cannot issue an exact neutral target
the sole neutral executor cannot accept the canonical published owner
```

Unsupported String/object carriers, helpers, imports, nested returns, cleanup,
and product/default routing are typed exclusions, not reasons to reopen this
row.

## Non-claims

```text
canonical Main CLI/runner activation
actual Main execution parity
helper/direct-call support
callable catalog generalization
default/product backend cutover
imports/using
JSON/LLVM/native
String/object/dynamic function result
cleanup
Legacy or Raw retirement
```
