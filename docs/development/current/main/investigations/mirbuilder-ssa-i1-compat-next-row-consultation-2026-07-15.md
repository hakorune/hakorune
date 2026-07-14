---
Status: Consultation packet — answer pending
Date: 2026-07-15
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-NEXT-ROW-SELECTION-DESIGN-STOP-001
Parent taskboard: mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
Previous closed card: mirbuilder-ssa-i1-compat-static-i64-parameter-selection-2026-07-15.md
Public baseline commit: 71f7dfc7d2
Decision requested: select exactly one bounded SSA-I1-COMPAT implementation row
---

# ChatGPT Pro Consultation: SSA-I1-COMPAT Next Row

## Executive question

Hakorune's D-prime canonical Lower now has one production function-owned
Binding SSA for the admitted trivial owner family. Exact Null, explicit Void,
and static exact-`i64` parameter ingress are closed. We must now choose exactly
one next compatibility row before implementation.

Please inspect the current architecture and select the cleanest next row from:

```text
P0n:
  exact-numeric parameter type expansion
  usize / fixed-width integer source spellings

P0b:
  Bool parameter entry and final-callee contract

P0f:
  Float parameter entry and final-callee contract

P0r:
  instance receiver owner-family ingress

P0c:
  exact known-static source Call ingress

R0:
  typed return/result family
```

Choose one row only. Do not combine receiver, Call, typed return, numeric
expansion, or Box ownership merely because P0a is green.

Our working hypothesis is:

> **R0a — static exact-`i64` typed return over the existing
> FunctionReturnContract is probably the narrowest useful next row.**

Please validate or reject this hypothesis. If it is wrong, identify the exact
dependency or authority conflict and select the better row.

## Current closed production boundary

The following is already production green:

```text
root owner:
  one non-main static FunctionDeclaration
  no override / uses / contracts / attrs

parameters:
  zero parameters
  or one-or-more exact source `i64` parameters
  exact name/order/cardinality
  reserved formal ValueIds %0..%N-1

body:
  current SSA-I1-T trivial grammar
  local / assignment / BlockExpr
  fallthrough statement If, including nested If
  final Return or implicit completion

value authority:
  one function-owned BindingSsaBuilderV1
  demand-driven PHIs
  no second BindingRef -> ValueId map

route selection:
  whole source unit before Builder effects
  TrivialBindingSsa or temporary whole-unit A+
  selected-route failure never retries A+
```

P0a specifically proved:

```text
sealed parameter-entry row
  -> exact MirParamDecl
  -> reserved formal ValueId
  -> exact Parameter BindingRef
  -> entry Binding SSA definition

fresh parameter ValueId allocation = 0
legacy name map use = 0
selected-route legacy RC insertion = 0
production Ownership SSA/opcodes = 0
```

Existing final-callee parameter contracts validate exact arity and runtime
`i64` before body effects. Unsupported backends fail through the shared
capability gate.

## Current evidence

The P0a census found:

```text
lang/src typed-parameter declarations:
  static owner lines:                         48
  instance owner lines:                      367
  static i64 occurrences:                      7
  strict all-i64 static P0a candidates:         0

apps:
  all-parameter exact-i64 static lines:      228
  files:                                      53
  strict P0a candidates:                       0
  reason: every candidate also has a typed return

tests / tools/smokes / examples:
  strict P0a candidates:                       0
```

The 228 app rows are only a potential R0 motivation. The existing census does
not yet prove that every return annotation is exact `i64`, nor that every body
stays inside the current trivial grammar. Do not turn 228 into an activation
claim without an exact bounded census.

The repository already contains an exact-numeric return contract owner:

```text
src/mir/type_contracts/return_exit.rs
  FunctionReturnContract
  exact declared numeric source name
  RejectVoid
  runtime_check_required

src/mir/return_exit_backend_capability.rs
  Rust MIR interpreter first-slice support
  unsupported backend fail-fast

src/backend/mir_interpreter/exec/return_contracts/
  final-outcome runtime validation
```

However, current canonical capability rejects every typed return before
Builder effects. Reusing the existing runtime contract is not by itself proof
that R0a is the correct next source/profile owner.

## Candidate comparison to resolve

### R0a — exact `i64` typed return

Potential advantages:

```text
keeps the same static owner family as P0a
keeps InlineI64 and MirType::Integer
may reuse the existing FunctionReturnContract
does not require receiver Box ownership
does not require source Call/MethodCall activation
may unlock real app corpus rows after exact census
```

Questions:

```text
Does typed return require a new sealed profile row, or is the existing exact
terminal value row plus exact declared-return ABI witness sufficient?

Who owns exact source spelling -> MIR return representation?

At what point is declared_return_type_name installed and the existing
return-exit contract refreshed?

Can R0a remain production Ownership SSA = 0 for exact i64?

Can a typed-return function with no source calls run through the current
closed body grammar without widening expression acceptance?
```

### P0n — broader exact-numeric parameters

Please determine whether this is genuinely one semantic row. In particular:

```text
usize host-width policy must not be collapsed to i64 accidentally
fixed-width source names need one source-name/MIR representation SSOT
parameter final-callee checks must already exist or be added explicitly
backend support must remain exact and fail-fast
```

If `usize` and fixed-width integers require different ABI laws, split them and
do not select the umbrella P0n name.

### P0b / P0f — Bool or Float parameters

The value profile already supports `InlineBool` and `InlineF64`, but that does
not automatically create a callable ABI. Please decide whether either row is
smaller than R0a after including:

```text
exact source declaration authority
reserved formal ValueId entry seeding
final-callee runtime type validation
MIR JSON transport
backend capability rejection
```

### P0r — instance receiver

This has the largest visible `lang/src` count, but it may be a different owner
family rather than a parameter spelling extension. Please reject P0r as the
next row if any of these are unavoidable:

```text
implicit receiver BoxRef ownership
CopyOwned / DestroyOwned production activation
method Call ingress
capture/upvar or field/place ownership
partial canonical/legacy mixing inside one source unit
```

Do not model `me` as an ordinary exact-`i64` formal parameter.

### P0c — exact known-static source Call

Please make dependencies explicit. A source Call may require:

```text
callee signature lookup
argument ownership ABI
call-result ownership/representation ABI
typed return carrier
backend call support
```

If P0c depends on R0 or future Ownership V2 call ABI, it must not be selected
first merely to increase corpus coverage.

## Architecture invariants

Every acceptable answer must preserve:

```text
pre-Builder:
  exact source identity and coverage
  exact representation/ABI profile
  unsupported-family rejection

Binding SSA:
  the only BindingRef -> reaching ValueId / PHI authority

FunctionEntryContractOwner / FunctionReturnContract:
  runtime callable-boundary validation
  never BindingRef identity authority

Lower:
  consumes sealed rows
  does not reread names/types to rediscover policy

route:
  selected once for the whole source unit before Builder effects
  no parent canonical / child legacy mixing
  no failure retry or silent fallback
```

The next row must also keep these at zero unless explicitly selected by a
separate later decision:

```text
source Call / MethodCall activation
receiver activation
Box/String/BorrowedText ownership
CopyOwned / DestroyOwned production callers
legacy ReleaseStrong on the selected route
Loop production activation
default source-route cutover
ProgramV0 authority
Hako Lower parity claim
```

## Ownership V2 boundary

The accepted future Ownership V2 documents define ScopedBoxAlias and Anchored
View Return ABI. They are not part of this compatibility-row selection.

```text
do not add take/share/clone/view syntax here
do not infer call-result ownership from method names or runtime tags
do not use R0a to decide future Owned/View/Shared source ABI
do not use P0r/P0c to activate Box ownership without an exact BoxRef witness
```

An exact numeric R0a may reuse the existing Language-v1 return contract while
remaining independent of future Box return ownership semantics.

## Required answer

Please return all of the following.

### 1. Decision

```text
selected row:
  exactly one of P0n / P0b / P0f / P0r / P0c / R0

selected first slice:
  a narrower versioned row name such as R0a if needed

why:
  source authority
  runtime/backend contract availability
  dependency order
  real corpus value
```

### 2. Exact accepted grammar

Specify root, signature, body, terminal, parameter, and backend restrictions.
Do not use “current supported grammar” without stating the relevant boundary.

### 3. Authority split

Name the exact owners for:

```text
source declaration spelling
sealed pre-Builder profile
BindingRef identity
reserved/new ValueIds
MIR signature/metadata
runtime entry or exit validation
backend capability
function publication
```

Also list all non-authorities.

### 4. Implementation order

Prefer a bounded sequence such as:

```text
L0:
  behavior-neutral facade/SSOT cleanup

S0:
  disconnected sealed product and fixtures

I1:
  atomic production activation
```

If fewer steps are cleaner, explain why. Only the final activation step may
change production routing.

### 5. Fixtures and gates

Include focused pass/reject fixtures, runtime checks, backend fail-fast,
coverage/consumption verification, and authority counters.

At minimum preserve:

```text
fresh parameter ValueId allocation = 0
second BindingRef -> ValueId map = 0
unsupported typed A+ retry = 0
selected-route legacy RC insertion = 0
production ownership operations = 0 for a trivial row
partial function publication = 0
```

### 6. Claims and non-claims

State exactly what may be claimed after the slice and what remains inactive.

### 7. Stop conditions

List implementation/publication stop conditions. Include any condition that
would force selection of a different row.

### 8. Follow-on order

After the selected row closes, order the remaining candidates without
pretending they are activated.

## Selection discipline

If current evidence is insufficient to choose exactly one row, do not invent
an implementation owner. Instead return:

```text
decision:
  remain at design stop

missing evidence:
  one exact question only

bounded evidence task:
  one read-only census or contract inventory

selection rule:
  deterministic rule that chooses exactly one row from that result
```

Do not propose a second open-ended design consultation. The next result must
either select one code-facing row or name one bounded evidence artifact whose
result mechanically selects it.

## Central question

> **After exact static `i64` parameter ingress is production green, is an
> exact static `i64` typed-return row the cleanest next compatibility proof,
> or does another candidate have a stricter dependency claim? Choose one row
> using source authority, executable ABI evidence, whole-owner atomicity, and
> corpus impact—not apparent implementation convenience.**
