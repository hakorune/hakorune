---
Status: Accepted — implementation next
Date: 2026-07-15
Decision: SSA-I1-COMPAT-N0a — exact NullSentinel local-flow row
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-N0A-NULL-SENTINEL-IMPLEMENTATION-001
Related:
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - mirbuilder-ssa-i1-t-trivial-binding-ssa-cutover-2026-07-15.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../../../../reference/language/types.md
---

# SSA-I1-COMPAT-N0a NullSentinel Selection

## Decision

Select exactly one compatibility row:

```text
source authority: LiteralValue::Null
profile authority: NullSentinel
MIR constant: ConstValue::Null
MIR type: MirType::Void
runtime representation: existing no-value representation
ownership: None
```

The language SSOT defines `null` as the source spelling of the same runtime
no-value concept as `void`. This row preserves that law while keeping the two
source sites distinct in exact profile coverage.

No runtime ownership inference, call ABI, receiver family, new opcode, or
backend representation is required.

## Candidate audit

The four open candidates are not one compatibility family.

| Candidate | Decision | Reason |
| --- | --- | --- |
| exact typed parameters | later independent row | can reuse function-entry ABI, but untyped parameters remain `MirType::Unknown` |
| receiver | separate owner-family row | instance owner, receiver slot, borrowed root, and ownership ABI are currently rejected before Builder |
| Void value | later disposition row | source Void values are distinct from completion/no-result control disposition |
| Outbox | after Void disposition | Outbox identity and transfer metadata are not the current synthetic Void seed |
| BorrowedText | later lifetime/ABI decision | literal anchor, copy/PHI/escape/return/destruction, and backend parity are not sealed |
| Null | selected | exact existing source/MIR/runtime no-value path; no RC or call ABI |

Parameter and receiver must not be bundled. Void and Outbox must not be
bundled. BorrowedText must not be admitted from inventory-only
`StorageClass::BorrowedText` evidence.

## First implementation grammar

Admit only closed owners whose new compatibility requirement is the exact
Null row:

```text
Null literal
local declaration initialized by Null
BindingRef read and assignment preserving NullSentinel
BlockExpr tail preserving NullSentinel
homogeneous Null/Null If merge
Null == Null and Null != Null producing InlineBool
final result is InlineBool or the already-supported no-value completion
```

One representative runtime fixture is:

```hako
local x = null
if cond {
    x = null
} else {
    x = null
}
return x == null
```

## Explicit fail-fast boundary

The following remain whole-unit non-admitted profiles selected before Builder
effects:

```text
Void literal
Outbox
Null returned as the terminal value
mixed Null/non-Null definitions or PHIs
String/Null operations
parameter or call-result Null ABI
receiver or instance owners
BorrowedText
```

Once the Null Binding-SSA route is selected, a missing or mismatched sealed
Null profile is a typed error. It never retries the temporary A+ route.

## Authority split

```text
resolved value profile:
  exact Null source/value/definition/merge coverage

Binding SSA:
  BindingRef -> ValueId reaching values and demand-driven PHIs

MIR constant/type:
  existing ConstValue::Null + MirType::Void materialization

runtime/backend:
  existing no-value execution only

non-authority:
  names, runtime tags, StorageClass inference, legacy A+ retry
```

`NullSentinel` is not a new runtime type and does not make Null an owned
value. It is an exact source/profile name that prevents the compatibility row
from being confused with explicit Void source sites or control completion.

## Acceptance

Required evidence:

```text
exact Null literal profile = green
local/read/assignment/BlockExpr forwarding = green
homogeneous one-sided/two-sided/nested If = green
Null == Null / Null != Null -> InlineBool = green
VM/reference result equality = green
selected values retain MirType::Void = green
selected-route ReleaseStrong = 0
selected-route CopyOwned / DestroyOwned = 0
selected-route legacy RC insertion = 0
canonical retry/fallback = 0
Void/Outbox/Null-return/mixed merge remain preflight rejects
all modified source/check files < 800 lines
```

This implementation card must satisfy:

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Nonclaims

Completion of N0a must not claim:

```text
general Void value support
Outbox support
Null return ABI
Null/Box nullable PHIs
parameter/receiver ABI
BorrowedText
Ownership SSA activation
CopyOwned/DestroyOwned production callers
Loop activation
SSA-I1-FULL
```

## Next action

Implement N0a inside the existing resolved-value-profile and trivial SSA
route, keeping the row below the same whole-owner preflight and publication
barriers. Do not start another compatibility row until N0a's claim and
negative boundaries are green.
