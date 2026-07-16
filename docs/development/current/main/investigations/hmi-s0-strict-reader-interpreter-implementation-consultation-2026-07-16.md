---
Status: resolved by current-source and worker audit
Date: 2026-07-16
Decision: B-prime selected; see accepted task card
Previous row: HMI-P0-G0 closed at `dec4769b18`
Scope: HMI-S0 only; production callers remain zero
---

# HMI-S0 strict reader and disconnected interpreter consultation

## Resolution

This consultation is closed by
`hmi-s0-strict-reader-interpreter-task-2026-07-16.md`.

The initial direct-reader recommendation was corrected after auditing
`apps/lib/json_native`: that existing substrate already preserves scalar kinds,
ordered members, source spans, and exact end-of-input. HMI therefore adds an
opt-in strict policy there instead of creating a third JSON parser authority.
Checked i64 semantics and a function-level exact-none control-edge witness are
also locked in the accepted task card.

## Question

HMI-P0 selected the serialized Rust-produced MIR JSON V1 document as the sole
future `.hako` semantic-reference ingress and closed its machine inventory and
drift guard. What is the smallest durable implementation packet for a direct
strict reader and disconnected scalar interpreter, without creating a second
MIR schema or inheriting Rust-only compatibility behavior?

## Current-source facts

1. `tools/hako_shared/json_parser.hako` parses an object directly into a
   `MapBox` and calls `obj.set(key, value)`. A duplicate key is therefore lost
   before a later verifier can reject it.
2. The same parser returns only the parsed value from `parse()` and does not
   publish an opaque proof that the whole input was consumed exactly once.
3. `tools/hako_shared/mir_analyzer.hako` is a tolerant analysis consumer. It
   checks only a small root surface and cannot be promoted into the HMI ingress
   authority.
4. Rust MIR integer `Add`, `Sub`, and `Mul` use ordinary `i64` operators. Their
   overflow behavior differs between checked debug execution and wrapping
   release execution. This cannot be called a portable semantic law.
5. Rust `Div` and `Mod` already report division by zero, but signed
   `i64::MIN / -1` and `i64::MIN % -1` also require an explicit portable
   overflow decision.
6. The V1 emitter does not transport Jump/Branch edge arguments. HMI-S0 may
   admit only edges whose argument lists are proven empty by the strict
   profile.
7. Bool is reconstructable only by co-validating the integer payload with the
   exact `value_types[dst] = i1` metadata row.
8. `ConstValue::Null` and `ConstValue::Void` collapse to the same wire value.
   HMI-S0 must reject `void` Const and admit no-value only through Return
   without an operand.

## Recommended architecture

### Reader owner

Add one HMI-specific strict document reader in `.hako`. It may reuse lexical
cursor helpers, but it must not call the current `JsonParserBox.parse()` and
then attempt to recover duplicate keys or scalar kinds.

```text
raw MIR JSON V1 text
  -> HmiStrictJsonReader
  -> duplicate-aware exact parse tree
  -> whole-document HMI profile verifier
  -> opaque VerifiedHmiDocumentView
```

The parse tree remains the one JSON carrier consumed by bounded views. Do not
translate it into a second instruction enum, V0 payload, compact payload, or a
second function/block graph.

Recommended physical split:

```text
tools/hako_shared/hmi/
  README.md                 layer boundary and forbidden conversions
  strict_json_reader.hako   JSON syntax, duplicate keys, exact consumption
  document_seal.hako        MIR V1 schema/profile validation
  document_view.hako        bounded read-only views over the sealed tree
  state.hako                disconnected execution state
  handlers/
    scalar.hako
    control_flow.hako
```

The strict JSON syntax helper may later become a shared parser substrate, but
HMI-S0 must not change the behavior of the existing compatibility parser as a
side effect.

### State owner

Use one disconnected state product:

```text
current function key
current block id
predecessor block id or Entry
registers keyed by ValueId
finished outcome: Running | Returned(value) | ReturnedNoValue | Failed(error)
```

Function/block/instruction identity stays in the verified document views.
State must not copy semantic rows or discover handlers at runtime.

### Handler owner

One closed compile-time dispatch table handles only:

```text
Const
Copy
BinOp
Jump
Branch
Phi
Return
```

`CopyOwned` and `DestroyOwned` receive transport/seal fixtures and disconnected
handler contracts, but executable Box admission remains blocked on
SSA-I1-O1. `ReleaseStrong` is rejected.

## Exact questions to lock

### Q1 — strict parser boundary

Recommended: a new HMI-specific direct reader that owns duplicate detection,
unknown-field rejection, scalar-kind preservation, integer range checks, and
full-input consumption. Reusing only `StringCursor`-level helpers is allowed.

Alternative: first refactor the general `JsonParserBox` into a lossless token
reader and tolerant/strict policy consumers. This is cleaner long-term but is
a BoxShape series larger than HMI-S0-T0.

Decision requested: select the direct HMI reader now, or require the general
JSON parser refactor first.

### Q2 — i64 overflow law

Recommended portable law:

```text
Add/Sub/Mul overflow:
  typed ArithmeticOverflow failure

Div/Mod by zero:
  typed DivisionByZero failure

i64::MIN / -1 and i64::MIN % -1:
  typed ArithmeticOverflow failure
```

This intentionally does not copy Rust's debug/release-dependent behavior.
Rust-oracle parity fixtures must stay inside the non-overflowing common domain;
portable overflow fixtures compare the declared HMI error contract, not an
accidental Rust build-mode result.

Alternative: define two's-complement wrapping for Add/Sub/Mul. If selected,
Div/Mod overflow still needs an explicit rule and `.hako` must implement the
same result independent of its host backend.

Decision requested: checked typed failure or wrapping arithmetic.

### Q3 — exact BinOp matrix

Recommended first executable matrix:

```text
i64 Add/Sub/Mul/Div/Mod only
BitAnd/BitOr/BitXor deferred
Shl/Shr deferred
Bool And/Or deferred
Float/String/operator-box routes deferred
```

This follows the selected HMI-P0 scalar profile. No method/operator fallback is
allowed.

### Q4 — CFG edge arguments

Recommended: Jump/Branch are admitted only when the source MIR edge argument
lists were empty and the emitted strict document carries a capability or
metadata proof of that fact. If current V1 cannot prove emptiness from the same
document, stop and repair the emitter before S0-T0. Never infer empty arguments
from their absence.

Decision requested: confirm that an emitter-side exact empty-edge witness may
be added to existing V1 metadata without creating a new schema.

### Q5 — first function-call boundary

Recommended: S0 implements one selected function execution with parameters
provided by the disconnected harness, but does not execute `Call`. Multiple
functions may be sealed for CFG/name consistency. Cross-function execution is
HMI-S1.

## Task order after the decision

### HMI-S0-T0 — direct reader and whole-document seal

```text
behavior delta: 0
production callers: 0

add:
  HMI-specific strict JSON reader
  duplicate-key and exact-consumption proof
  root/function/CFG/block/instruction/type/PHI/ownership verifier
  opaque VerifiedHmiDocumentView

forbid:
  current tolerant JsonParserBox as ingress
  second MIR instruction schema
  V1 -> V0/compact translation
  partial seal publication
```

### HMI-S0-V0 — disconnected state machine

```text
behavior delta: 0
production callers: 0

add:
  exact register state
  current/predecessor block state
  entry/return/failure outcomes
  step bound for harness resource safety

forbid:
  product VMValue
  Rust MirModule
  heap/Box admission
  runtime handler discovery
```

### HMI-S0-I0 — exact scalar handlers

```text
behavior delta: disconnected fixture execution only
production callers: 0

activate in the disconnected HMI harness:
  Const i64/Bool
  Copy
  exact five-op i64 BinOp matrix
  Jump/Branch/Phi
  Return value/no-value

contract only:
  CopyOwned/DestroyOwned
```

### HMI-S0-P0 — Rust-oracle parity

```text
behavior delta: 0 product routes
production callers: 0

compare:
  result
  typed failure in the common declared domain
  predecessor-sensitive Phi result
  final register/state snapshot selected by the fixture contract

do not compare:
  Rust-only dynamic truthiness
  operator-box/string/float behavior
  Null/Void identity
  Rust build-mode overflow accidents
```

## Required fixtures

Pass:

```text
whole input consumed exactly
entry block not lowest block id
multiple sealed functions, one selected execution function
i64 Const -> Copy -> Return
Bool payload plus exact i1 metadata
all five admitted i64 BinOps in the non-overflowing domain
Jump chain
exact i1 Branch
two-input and multi-input predecessor-sensitive Phi
Return without value
```

Reject before state allocation:

```text
duplicate JSON object key at every bounded level
trailing JSON value or garbage
unknown bounded-row field
missing/extra CFG function
entry outside function
duplicate function/block/value-type key
edge without exact empty-argument proof
Phi outside prefix or predecessor mismatch
missing/multiple/non-final terminator
Bool metadata/payload mismatch
void Const, Unknown, unsupported value class/opcode/operator
CopyOwned/DestroyOwned witness mismatch
ReleaseStrong
```

Runtime typed failures after a successful seal:

```text
division by zero
modulo by zero
arithmetic overflow, if checked law is selected
undefined register read
Phi without exact predecessor input
step-bound exhaustion
```

## Guards and counters

```text
HMI-S0 production callers = 0
Rust fallback = 0
route retry = 0
V1-to-v0 conversion = 0
compact payload normalization = 0
second MIR instruction schema = 0
partial document execution = 0
effects before document seal = 0
runtime handler discovery = 0
ReleaseStrong admission = 0
BoxRef execution before O1 = 0
source/check files >= 800 lines = 0
```

## Implementation may claim after P0

```text
one direct `.hako` strict reader seals the selected MIR JSON V1 profile
one disconnected scalar state machine executes the exact admitted subset
unsupported and lossy documents fail before execution state exists
Rust-oracle parity is green inside the explicitly shared portable domain
production execution ownership remains unchanged
```

## Implementation must not claim

```text
general JSON parser replacement
all MIR JSON V1 support
Call/MethodCall execution
BoxRef or ownership execution
Null/Void identity parity
dynamic truthiness
Float/String/operator-box parity
product VM replacement
Rust handler retirement
parser/MirBuilder or Ownership V2 progress
```

## Stop conditions

Stop before implementation if any of these is required:

1. Duplicate keys must be detected after insertion into `MapBox`.
2. A second MIR instruction or function/block graph schema is introduced.
3. The document must pass through V0 or compact normalization.
4. Missing CFG, type, PHI, ownership, or edge-argument facts are inferred.
5. Rust debug/release overflow behavior is called a portable law.
6. A failed strict seal allocates registers or executes an instruction.
7. Unsupported input falls back to Rust or another route.
8. CopyOwned/DestroyOwned requires Rust `Arc`/`VMValue` layout.
9. Product callers are connected before HMI-P1/X0.
10. One source/check file reaches 800 lines.

## Proposed decision lock

> HMI-S0 uses one HMI-specific `.hako` strict JSON reader over the selected
> Rust-emitted MIR JSON V1 bytes. The reader preserves scalar kinds, rejects
> duplicate keys and trailing input during parsing, and publishes no semantic
> view until the entire document, CFG, types, PHI topology, terminators,
> ownership witnesses, and exact empty-edge condition are sealed. Bounded views
> read the same tree; no second MIR schema or compatibility translation is
> created. A disconnected state machine then executes exact i64/Bool scalar
> Const, Copy, five arithmetic BinOps, Jump, Branch, Phi, and Return, while
> CopyOwned/DestroyOwned remain O1-blocked contracts. Production callers and
> fallback remain zero. The remaining vote is the portable i64 overflow law
> and whether empty edge arguments can be proven by metadata in existing V1.
