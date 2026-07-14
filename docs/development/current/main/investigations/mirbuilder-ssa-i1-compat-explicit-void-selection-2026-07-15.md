---
Status: Closed
Date: 2026-07-15
Decision: SSA-I1-COMPAT-V0a — exact ExplicitVoidValue disposition row
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-V0A-EXPLICIT-VOID-VALUE-IMPLEMENTATION-001
Related:
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - mirbuilder-ssa-i1-compat-null-sentinel-implementation-2026-07-15.md
  - mirbuilder-ssa-i1-compat-explicit-void-implementation-2026-07-15.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../../../../reference/language/types.md
---

# SSA-I1-COMPAT-V0a Explicit Void Selection

## Decision

Select exactly one compatibility row:

```text
source authority: LiteralValue::Void
profile authority: ExplicitVoidValue
MIR constant: ConstValue::Void
MIR type: MirType::Void
runtime representation: existing VM/reference no-value
ownership: None
```

The profile name is `ExplicitVoidValue`, not `VoidSentinel`. It must remain
distinguishable from source `null`, `return;`, and implicit fallthrough even
though all four currently materialize through the runtime no-value lane.

No new opcode, runtime tag, ownership operation, call ABI, or backend
vocabulary is authorized.

## Why this row is next

Three independent read-only audits and a `lang/src` census select the order:

```text
V0a explicit Void
  -> P0 exact typed parameter ABI
  -> receiver as a separate owner-family decision
```

The reasons are structural:

1. V0a closes the exact source-value disposition required before Outbox can
   be modeled without confusing its identity with a synthetic Void seed.
2. Existing constant emission, MIR typing, interpreter execution, terminal
   coverage, Binding SSA, and If materialization already provide the complete
   runtime substrate. Only the exact profile admission is missing.
3. P0 is a distinct ABI slice. It must seed reserved parameter ValueIds,
   preserve declared parameter metadata, reuse or extend function-entry
   contracts, and retain receiver/untyped/Box rejection.
4. The current corpus contains explicit `return void` sites. Exact scalar
   parameter signatures are overwhelmingly instance methods, so a static-only
   P0 does not yet unlock the receiver-blocked family.
5. BorrowedText is not a representation row yet. It needs a separate anchor,
   lifetime, Phi/Return, ownership, equality, and backend-capability decision.

This is not an Outbox activation and not a shortcut around the later ABI work.

## Exact terminal law

Existing products already distinguish the three terminal dispositions:

```text
return void:
  TrivialTerminalProfileV1::ExplicitValue {
    representation: ExplicitVoidValue
  }

return;:
  TrivialTerminalProfileV1::ExplicitNoValue

implicit fallthrough:
  TrivialTerminalProfileV1::ImplicitNoValue
```

All currently execute as a MIR Return carrying a Void ValueId. V0a seals the
source/profile distinction; it does not claim a new runtime behavior.

## First implementation grammar

Admit only closed owners whose new requirement is exact explicit Void:

```text
Void literal
local declaration initialized by Void
BindingRef read and assignment preserving ExplicitVoidValue
BlockExpr tail preserving ExplicitVoidValue
homogeneous one-sided, two-sided, and nested If
Void == Void and Void != Void producing InlineBool
explicit return void
Void expression statement followed by existing implicit completion
```

Representative runtime fixture:

```hako
local x = void
if cond {
    x = void
} else {
    x = void
}
return x
```

## Explicit fail-fast boundary

The following remain whole-unit non-admitted profiles selected before Builder
effects:

```text
Void as an If condition
Void arithmetic, ordering, or logical operation
Null/Void binary operation
Null/Void mixed definition or Phi
missing local initializer
Outbox
BorrowedText
parameter, receiver, or call-result ABI
```

Once V0a is selected, missing or mismatched sealed profile coverage is a typed
error. It never retries the temporary A+ route.

## Authority split

```text
resolved value profile:
  exact Void source/value/definition/merge/terminal coverage

function completion:
  explicit-value vs explicit-no-value vs implicit-no-value disposition

Binding SSA:
  sole BindingRef -> ValueId and demand-driven Phi authority

MIR/runtime:
  existing ConstValue::Void + MirType::Void + VMValue::Void

non-authority:
  runtime tags, names, Outbox identity, StorageClass, legacy retry
```

## Acceptance

Required positive evidence:

```text
explicit Void literal profile = green
local/read/assignment/BlockExpr forwarding = green
homogeneous one-sided/two-sided/nested If = green
Void == Void -> true
Void != Void -> false
return void has ExplicitValue terminal profile
return; keeps ExplicitNoValue terminal profile
implicit fallthrough keeps ImplicitNoValue terminal profile
all explicit Void values and Phis retain MirType::Void
selected-route ownership operations = 0
selected-route legacy RC insertion = 0
canonical retry/fallback = 0
```

Required negative evidence:

```text
Outbox remains A+
return null remains A+
Null/Void mixed merge remains A+
Void condition/arithmetic remains A+
BorrowedText/parameter/receiver remain unchanged
all modified source/check files < 800 lines
```

This implementation card must satisfy:

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Implementation boundary

Expected production delta is limited to:

```text
resolved_value_profile:
  ExplicitVoidValue vocabulary and exact admission

trivial_ssa/operation:
  ExplicitVoidValue -> MirType::Void
```

The production lowerer, completion owner, If-control product, constant emitter,
runtime interpreter, opcodes, and backend allowlists should remain unchanged.
If implementation requires widening any of those owners, stop and return to
design review.

## Nonclaims

Completion of V0a must not claim:

```text
Outbox support
Null return ABI
Null/Void mixed representation merge
missing-local compatibility
BorrowedText
parameter/receiver ABI
Ownership SSA activation
Loop activation
SSA-I1-FULL
```

## Closeout

V0a is closed by
`mirbuilder-ssa-i1-compat-explicit-void-implementation-2026-07-15.md`.
The next frontier is the P0 exact typed parameter ABI design-selection stop;
receiver remains a separate owner-family decision.
