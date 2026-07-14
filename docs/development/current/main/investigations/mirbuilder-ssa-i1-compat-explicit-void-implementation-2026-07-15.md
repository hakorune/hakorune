---
Status: Closed
Date: 2026-07-15
Decision: SSA-I1-COMPAT-V0a — exact ExplicitVoidValue production row
Selection:
  - mirbuilder-ssa-i1-compat-explicit-void-selection-2026-07-15.md
Next blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0-EXACT-TYPED-PARAMETER-SELECTION-DESIGN-STOP-001
---

# SSA-I1-COMPAT-V0a Explicit Void Implementation

## Result

One exact source-value row is now admitted by the existing production Binding
SSA route:

```text
LiteralValue::Void
  -> TrivialRepresentationV1::ExplicitVoidValue
  -> ConstValue::Void
  -> MirType::Void
  -> existing runtime no-value representation
```

This row is non-owned. It adds no ownership instruction, backend opcode,
runtime tag, or call ABI.

The production lowerer, function-completion owner, If-control product,
constant emitter, runtime interpreter, opcode vocabulary, and backend
allowlists are unchanged. The only production materialization delta is the
exact `ExplicitVoidValue -> MirType::Void` profile mapping.

## Closed grammar

The following exact shapes now stay on the one function-owned Binding SSA:

```text
explicit Void literal
local/read/assignment forwarding
BlockExpr tail forwarding
homogeneous one-sided, two-sided, and nested If
Void equality/inequality producing Bool
explicit return void
Void expression statement followed by implicit completion
```

The three terminal source dispositions remain distinct:

```text
return void       -> ExplicitValue(ExplicitVoidValue)
return;           -> ExplicitNoValue
implicit return   -> ImplicitNoValue
```

They currently converge only at the existing MIR/runtime no-value
representation boundary.

## Negative boundary

The following remain whole-unit non-admitted profiles selected before Builder
effects:

```text
Outbox
return null
Null/Void mixed definitions and merge
Void condition/arithmetic/ordering/logical use
BorrowedText
parameter, receiver, and call-result ABI
```

There is no body-level A+/Binding-SSA mixing and no canonical retry.

## Evidence

```text
resolved value profile:             13/13
focused Void VM/reference:           3/3
full resolved lowering:             89/89
canonical capability:                5/5
finish schedule checks:              green
production ownership inventory:     18/18
  exact trivial rows:                 5
legacy lifecycle ledger:        125 rows / 276 occurrences
resolved authority guard:            green
dev gate quick:                      66/66
release build:                       green
fmt / diff check:                    green
```

All modified source/check files remain below 800 lines. The largest are:

```text
src/mir/resolved_value_profile/analyzer.rs                  603
tools/checks/lib/resolved_trivial_owner_profile.py          476
src/mir/resolved_value_profile/tests.rs                     450
src/mir/builder/resolved_lowering/void_tests.rs             277
```

## Claims

V0a may claim:

```text
exact explicit Void local-flow profile
homogeneous Void PHIs with MirType::Void
terminal source/profile distinction
selected-route ownership operations = 0
selected-route legacy RC insertion = 0
```

V0a must not claim:

```text
Outbox support
Null return ABI
Null/Void mixed representation merge
BorrowedText
parameter or receiver ABI
Ownership SSA activation
Loop activation
SSA-I1-FULL
```

## Next

Return to a design-selection stop for **P0 exact typed parameter ABI**.
Receiver remains a separate owner-family decision. P0 must lock entry ValueId
seeding, declared parameter metadata, exact accepted types, and unsupported
owner-family preflight before implementation begins.
