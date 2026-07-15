---
Status: Closed; R0a-I1 production activation green
Date: 2026-07-15
Decision: SSA-I1-COMPAT-R0a — static exact-i64 typed return
Next blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-I64-DESIGN-STOP-001
Related:
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - mirbuilder-ssa-i1-compat-static-i64-parameter-selection-2026-07-15.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../../../../reference/language/ownership.md
---

# SSA-I1-COMPAT-R0a Static Exact-i64 Return Selection

## Decision

Select the smallest typed-return row:

```text
owner family:
  non-main static function only

source return spelling:
  exact `i64` only

terminal:
  exactly one final explicit Return
  terminal value representation = InlineI64

runtime exit owner:
  existing ReturnExitContract
  owner tag = ReturnExitContractOwner::FunctionReturnContract

first supported backend:
  Rust MIR interpreter only
```

R0a adds one function-level return ABI witness. It does not add a new terminal
analysis, ValueId, MIR opcode, Ownership SSA operation, source Call, receiver,
or backend ABI.

This card is the one allowed docs-only selection step. The next blocker is the
behavior-neutral R0a-L0 implementation artifact:

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Why R0a is selected

The three required authorities already exist:

```text
source declaration:
  FunctionDeclaration.return_type_name

pre-Builder terminal truth:
  TrivialTerminalProfileV1::ExplicitValue
  + TrivialRepresentationV1::InlineI64

runtime boundary:
  ReturnExitContract
  + return_exit_backend_capability
```

The missing seam is narrow: co-seal exact source spelling `i64` with the
existing explicit InlineI64 terminal, then make resolved lowering consume that
witness instead of rereading the raw return annotation.

Other candidates remain separate:

```text
P0n-fixed:
  requires a fixed-width numeric vocabulary expansion

P0n-usize:
  requires an explicit target-width/unsigned policy

P0b / P0f:
  require final-callee Bool/Float callable-boundary contracts

P0r:
  is a receiver/BoxRef/ownership owner family

P0c-i64:
  requires source Call and callee result ABI consumption
```

## Exact accepted grammar

### Root and signature

```text
AST owner:
  exactly one FunctionDeclaration

required:
  is_static = true
  name != main
  is_override = false
  uses/contracts/attrs = empty
  nested owner/capture = none

parameters:
  zero parameters
  or every parameter is exact source `i64`
  mixed typed/untyped and non-i64 parameters reject

return_type_name:
  exactly Some("i64")
```

No spelling aliases are admitted. `int`, `Integer`, `I64`, `u64`, `usize`,
or whitespace-normalized variants are not R0a.

### Body

The body grammar is the unchanged current SSA-I1-T/P0a grammar. R0a does not
duplicate or widen expression acceptance. In particular, the currently sealed
expression family is:

```text
Literal
Variable
BinaryOp except And/Or
BlockExpr
```

The currently sealed statement/control family remains:

```text
initialized untyped local
binding assignment
fallthrough statement If, including nested If
final explicit Return
```

Still rejected:

```text
Call / MethodCall / NewBox
Loop
early or branch-local Return
typed local
Outbox
Lambda / QMark / Try
String / BorrowedText
ownership operations
```

### Terminal

```text
required:
  exactly one final explicit Return
  return expression representation = InlineI64

rejected:
  implicit completion
  return;
  return void
  return Null / Bool / Float
  non-final or branch-local Return
```

## Sealed product

Do not create another terminal row. Retain:

```rust
TrivialTerminalProfileV1::ExplicitValue {
    statement,
    value,
    representation,
}
```

Add only a function-boundary witness over a shared exact scalar substrate:

```rust
pub enum ExactTrivialScalarAbiV1 {
    I64,
}

pub struct ExactTrivialReturnAbiV1 {
    scalar: ExactTrivialScalarAbiV1,
}

pub struct VerifiedTrivialFunctionReturnV1 {
    abi: ExactTrivialReturnAbiV1,
}
```

`VerifiedTrivialCanonicalOwnerV1` gains:

```rust
function_return: Option<VerifiedTrivialFunctionReturnV1>,
```

The analyzer may construct the witness only when all of these agree:

```text
source return_type_name == Some("i64")
terminal == ExplicitValue
terminal representation == InlineI64
completion is the same final explicit value return
terminal statement/value sites match exact completion/coverage sites
implicit completion == false
```

Authority split:

```text
TrivialTerminalProfileV1:
  what exact source value/site is returned

VerifiedTrivialFunctionReturnV1:
  whether the function boundary may declare exact i64

BindingSsaBuilderV1 / expression Lower:
  reaching ValueId and PHI result

ReturnExitContract:
  final-callee runtime result validation

return_exit_backend_capability:
  backend support/fail-fast
```

The witness must not own a second return value, terminal merge, BindingRef, or
ValueId authority.

## Preflight transition law

The current broad typed-return rejection in capability preflight must not be
deleted without replacement. R0a-L0/S0/I1 must establish this exact order:

```text
1. detect a typed-return candidate before Builder effects
2. let the resolved trivial analyzer attempt exact R0a admission
3. exact `i64` + sealed InlineI64 terminal -> BindingSsa route
4. any other typed return -> typed capability error
5. typed-return NotAdmitted never reaches temporary A+
6. selected R0a failure never retries another route
```

The broad compatibility mapper `source_type_name_to_mir` is not admission
authority. `MirType::Integer` alone is insufficient to prove exact `i64`.

## Lowering seam

The current resolved lowering derives parameter metadata from the sealed
profile but still passes raw `return_type_name` into generic builder metadata.
R0a must close that asymmetry.

Place the new installer in the route-local facade:

```text
src/mir/builder/resolved_lowering/trivial_ssa/callable_abi.rs
```

Conceptual API:

```rust
fn install_trivial_callable_abi_v1(
    builder: &mut MirBuilder,
    params: &[VerifiedTrivialParameterEntryV1],
    result: Option<&VerifiedTrivialFunctionReturnV1>,
);
```

Rules:

```text
resolved trivial route:
  consumes sealed parameter/return witnesses

generic MirBuilder metadata:
  receives completed declarations only

Lower raw return_type_name read:
  0

broad source-type reclassification in Lower:
  0
```

R0a allocates no return-specific value. A returned local/parameter/PHI uses the
current Binding SSA value; a literal or binary expression uses its existing
expression ValueId.

## Contract refresh and publication law

Required order:

```text
1. whole-unit exact preflight
2. open unpublished function session
3. create function skeleton
4. install sealed callable ABI before body effects
5. seed parameter Binding SSA
6. lower body and exact terminal
7. finish CFG / Binding SSA / coverage
8. finalize unpublished MirFunction draft
9. refresh parameter and return boundary contracts
10. MirVerifier
11. function-session cleanup
12. publish function to module
13. backend capability gate before backend effects
```

Carrier/metadata drift must reject before function/module publication.

A wrong runtime value is different: the function has already been compiled
and published, but the Rust MIR interpreter must reject the final callee
outcome before the caller observes the result. Do not claim runtime mismatch
rejection before module publication.

## Implementation series

### R0a-L0 — behavior-neutral ABI facade

```text
add ExactTrivialScalarAbiV1::I64
make exact i64 parameter ABI delegate to the shared scalar substrate
add route-local sealed callable-ABI installer
add one boundary-contract refresh facade for parameter then return

production grammar delta = 0
route delta = 0
```

Acceptance:

```text
existing P0a behavior and fixtures remain green
typed return remains production-inactive
raw return annotation has one bounded route-local transition seam
```

R0a-L0 is closed. Landed evidence:

```text
site-neutral scalar SSOT:
  ExactTrivialScalarAbiV1::I64

parameter ABI:
  delegates exact spelling / MirType projection to the scalar SSOT

route-local facade:
  trivial_ssa/callable_abi.rs
  installs sealed parameter declarations with result = None
  refreshes parameter then return boundary carriers on the unpublished draft

resolved trivial raw return_type_name reads:
  0

production typed-return activation:
  0

grammar / route delta:
  0
```

Verification:

```text
exact scalar/parameter focused tests: green
resolved-lowering parameter fixtures: 5/5
resolved-lowering full fixtures: 94/94
resolved-value-profile fixtures: 18/18
resolved-region-flow authority guard: green
release build: green
quick gate: 66/66
all touched source/check files: < 800 lines
```

### R0a-S0 — disconnected sealed return witness

```text
add ExactTrivialReturnAbiV1
add VerifiedTrivialFunctionReturnV1
co-seal exact :i64 with final InlineI64 terminal and completion
add exact coverage/foreign-key checks

Builder connection = 0
production activation = 0
```

Run a read-only strict corpus census here. Candidate counts are evidence, not
activation authority.

R0a-S0 is closed. Landed evidence:

```text
return source ABI:
  ExactTrivialReturnAbiV1
  exact spelling = i64 only
  physical projection delegates to ExactTrivialScalarAbiV1

function witness:
  VerifiedTrivialFunctionReturnV1
  co-sealed only after existing terminal/completion verification
  exact ExplicitValueTerminal coverage foreign-key count = 1

terminal/value authority added:
  0

external function_return consumers:
  0

Builder / production activation:
  0 / 0
```

Read-only parser-evidence census over `lang/src`, `apps`, `tests`,
`tools/smokes`, and `examples`:

```text
files containing an exact-i64 return with zero/all-exact-i64 parameters: 10
matching function signatures:                                           34
instance-owner signatures:                                              34
static-owner R0a signature candidates:                                   0
parse failures:                                                          0
```

The body grammar did not need further classification because the exact root
owner-family boundary already reduces the strict candidate set to zero. This
is evidence only and does not change admission authority.

Verification:

```text
exact return ABI fixtures: 2/2
resolved-value-profile fixtures: 22/22
resolved-region-flow authority guard: green
all touched source/check files: < 800 lines
```

### R0a-I1 — atomic production activation

```text
admit only exact R0a
install callable metadata from sealed witnesses
reuse existing Binding SSA and terminal emission
refresh parameter + return contracts before MirVerifier/publication
Rust MIR interpreter only
unsupported backend fail-fast
fallback/retry = 0
```

Only I1 may change production routing.

R0a-I1 is closed. Landed evidence:

```text
preflight:
  exact sealed R0a enters TrivialBindingSsa
  every other typed return fails as typed_return_profile_not_activated
  typed-return A+ retry = 0

callable ABI:
  parameter and result declarations come only from the sealed profile
  raw return_type_name reads in resolved Lower = 0

value authority:
  returned parameter/local/If PHI uses the existing Binding SSA ValueId
  fresh return ValueId = 0
  CopyOwned / DestroyOwned / ReleaseStrong = 0

publication/runtime:
  parameter and return boundary contracts refresh on the unpublished draft
  MirVerifier runs before module publication
  Rust MIR interpreter validates the final result before caller observation
  unsupported backends fail through the existing capability gate
```

Verification:

```text
R0a focused runtime/transport fixtures: 5/5
P0a parameter fixtures: 5/5
resolved-value-profile fixtures: 22/22
resolved Binding SSA guard: green
resolved-region-flow authority guard: green
release build: green
quick gate: green
current-state pointer guard: green
all touched source/check files: < 800 lines
```

## Required fixtures

Positive:

```hako
static answer(): i64 {
    return 42
}
```

```hako
static identity(x: i64): i64 {
    return x
}
```

```hako
static choose(x: i64, cond: i64): i64 {
    if cond == 0 {
        x = 10
    } else {
        x = 20
    }
    return x
}
```

Positive assertions:

```text
exact source spelling transported
signature result == MirType::Integer
declared return spelling == exact i64
terminal representation == InlineI64
exactly one ReturnExitContract
owner tag == FunctionReturnContract
void policy == RejectVoid
runtime check required
post-If return uses existing Binding SSA PHI
fresh return ValueId == 0
```

Negative:

```text
:int / :Integer / :u64 / :usize / :bool / :f64 / :String / Box
return; / return void / implicit completion
return Null / Bool / Float
mixed typed/untyped parameters
Call / MethodCall / NewBox / Loop / early Return / receiver / main
```

Transport/runtime:

```text
missing/extra/drifted ReturnExitContract rejects before publication
Void or wrong final runtime result rejects before caller observation
non-VM backend rejects before backend effects
```

## Authority counters

```text
fresh return ValueId allocation = 0
second BindingRef -> ValueId map = 0
raw source return-type read in Lower = 0
broad source_type_name_to_mir as admission authority = 0
typed-return A+ retry = 0
CopyOwned / DestroyOwned / ReleaseStrong selected-route callers = 0
source Call / MethodCall / receiver activation = 0
ReturnExitContract rows per R0a function = 1
partial function publication = 0
```

## Implementation may claim after R0a-I1

```text
static exact-i64 typed returns are production-supported for the closed owner
exact annotation and terminal are co-sealed before Builder effects
existing Binding SSA / expression Lower supplies the returned ValueId
existing final-callee i64 return validation is preserved
unsupported typed returns never retry A+
unsupported backends fail before backend effects
Ownership SSA/opcode activation remains zero
```

## Implementation must not claim

```text
all numeric returns or usize
Bool/Float return ABI
typed locals
source Call/MethodCall
receiver or Box/View/Shared return ABI
Ownership V2 return semantics
all backend support
selfhost corpus coverage increase
Loop support
```

## Stop conditions

Stop R0a if any implementation requires:

1. source Call, MethodCall, receiver, or instance-owner activation;
2. `MirType::Integer` or the broad compatibility mapper as exact admission;
3. Lower rereading raw `return_type_name`;
4. a second return-value/terminal/BindingRef/PHI authority;
5. runtime-only admission of implicit completion or Void;
6. contract refresh after function publication;
7. runtime result publication to the caller before final-callee validation;
8. unsupported-backend execution, silent VM fallback, or A+ retry;
9. CopyOwned, DestroyOwned, ReleaseStrong, or return-specific ValueId emission;
10. parameter and result halves using different canonical value authorities.

## Follow-on order

After R0a closes:

```text
P0c-i64
  exact known-static source Call: exact i64 args -> exact i64 result

P0n-fixed
  target-independent fixed-width parameter/result ABI

P0b
  Bool final-callee parameter/result ABI

P0f
  Float final-callee parameter/result ABI

P0n-usize
  target-width policy as a separate row

P0r
  receiver / BoxRef / method routing / ownership witness
```

R0a is closed. The next task is the **P0c-i64 source Call design stop**. It must
name the exact known-static callee-signature authority, call-site argument and
result ABI carrier, backend fail-fast boundary, and no-fallback rule before any
Call grammar or Lowering activation. P0c must reuse P0a + R0a rather than add a
second parameter/result/value authority.
