---
Status: accepted; HMI-S0-J0 next
Date: 2026-07-16
Decision: B-prime json_native strict policy plus exact-none CFG witness
Previous row: HMI-P0-G0 closed at `dec4769b18`
Consultation: hmi-s0-strict-reader-interpreter-implementation-consultation-2026-07-16.md
Scope: disconnected HMI-S0; production callers remain zero
---

# HMI-S0 strict ingress and scalar interpreter task

## Decision

HMI-S0 reuses `apps/lib/json_native` as the sole native JSON grammar/tree
authority and adds an opt-in strict policy. It does not create an HMI-specific
recursive JSON parser.

```text
Rust-emitted MIR JSON V1 bytes
  -> json_native lexer/parser with StrictJsonPolicyV1
  -> one scalar-kind-preserving ordered JsonNode tree
  -> HMI whole-document profile seal
  -> bounded views over the same tree
  -> disconnected HMI scalar state machine
```

The first portable arithmetic law is checked signed i64:

```text
Add/Sub/Mul overflow       -> ArithmeticOverflow
Div/Mod by zero            -> DivisionByZero
i64::MIN / -1              -> ArithmeticOverflow
i64::MIN % -1              -> ArithmeticOverflow
normal signed Div          -> truncate toward zero
normal signed Mod          -> dividend-sign remainder
wrapping                   -> not implicit; future explicit vocabulary only
```

Jump and Branch remain the existing V1 payload shape. The Rust producer must
first prove every final control edge uses exact `Option::None` and add one
function-level metadata witness:

```text
control_edge_args_v1:
  mode = exact_none
```

`Some(EdgeArgs { values: [] })` is rejected because it still carries a layout
contract. Missing witness is never interpreted as empty edge arguments.

## Why this is the clean boundary

`tools/hako_shared/json_parser.hako` is a tolerant compatibility parser. It
collapses scalar kinds, permits prefix parsing, and overwrites duplicate keys.
It remains outside HMI.

`apps/lib/json_native` already owns:

```text
JSON tokenization and grammar
scalar-kind-preserving JsonNode
ordered object member vectors
source spans
whole-input EOF checking
```

Its missing strict facts are limited to duplicate-key rejection and integer
range checks before host conversion. Adding an opt-in policy avoids a third
JSON parser authority while preserving compatibility entry behavior.

The generic Rust MIR interpreter is not the overflow authority. Its raw i64
operators panic in checked debug builds and wrap or abort differently in
release. Existing numeric policy already says plain exact operators are
checked and wrapping requires explicit vocabulary. HMI follows that policy.

## Authority split

| Concern | Authority | Non-authority |
| --- | --- | --- |
| JSON lexical grammar | `apps/lib/json_native/lexer` | HMI document seal |
| JSON parse/tree shape | `apps/lib/json_native/parser` and `core/node` | `tools/hako_shared/json_parser.hako` |
| strict JSON admission | `StrictJsonPolicyV1` | MIR field whitelist |
| MIR allowed/required fields | HMI document seal | JSON parser |
| final edge-arg fact | Rust final `MirFunction` verifier | missing JSON fields |
| transported edge witness | function metadata `control_edge_args_v1` | opcode-name inference |
| i64 semantic law | accepted checked numeric policy plus HMI handler | Rust build profile |
| execution state | disconnected HMI state product | JsonNode/VMValue |
| Rust parity | HMI-S0-P0 harness | product execution routing |

## Physical structure

```text
apps/lib/json_native/parser/
  strict_policy.hako       new policy and typed strict errors
  integer_span.hako        exact lexeme/range/conversion helper
  parser.hako              small opt-in hooks only

apps/lib/json_native/core/node.hako
  object_has(key)          behavior-neutral read-only helper

tools/hako_shared/hmi/
  README.md                ownership and forbidden conversions
  strict_ingress.hako      strict json_native entry only
  document_seal.hako       MIR whole-document verifier
  document_view.hako       bounded views over the same JsonNode
  state.hako               disconnected execution state
  i64_arithmetic.hako      checked five-op law
  handlers/
    scalar.hako
    control_flow.hako
```

Files may be split further before reaching 800 lines. No file may cross that
limit.

## Exact implementation order

### HMI-S0-J0 — opt-in strict json_native policy

```text
production behavior delta: 0
HMI execution callers: 0

add:
  StrictJsonPolicyV1
  typed strict parse errors
  duplicate-before-insert check
  exact number lexeme range validation
  signed-negative i64 MIN conversion
  behavior-neutral JsonNode.object_has

preserve:
  JsonParser.parse default behavior
  JsonParserUtils.parse_json behavior
  JsonNode.object_set overwrite behavior
```

Strict reader responsibilities:

```text
valid JSON syntax
decoded scalar kind
decoded-key duplicate rejection
exact number lexeme
field-neutral i64-safe conversion
whole-input consumption
```

It does not know MIR field names or schemas.

#### J0 compiler prerequisite — receiver-field predicate read

The first J0 execution probe found one existing Facts/Lower contract gap in
`JsonParser.parse_object`. The strict duplicate guard contains:

```hako
if me.policy != null and me.policy.rejects_duplicate_keys() {
    if object_node.object_has(key) {
        me.add_error(...)
        return null
    }
}
```

`me.policy` is parsed as a `FieldAccess` rooted at `Me`. Canonical value
lowering already supports this read, but the shared boolean/value-expression
Facts vocabulary rejects it. That prevents the existing exit-allowed
`loop(true)` recipe from being sealed and incorrectly hands the loop to the
generic induction-variable route, which then reports zero candidates.

The exact prerequisite row is:

```text
HMI-S0-J0-RF0

accept in shared value-expression Facts:
  Me | This value roots
  recursively lowerable FieldAccess over an admitted value root

use:
  comparison operand inside an already-supported boolean condition

materialization:
  existing PlanNormalizer FieldAccess lowering only

new control recipe:
  0

new lowering path:
  0

AST rewrite / source workaround / fallback:
  0
```

RF0 does not admit field assignment, dynamic indexing, ownership/view
semantics, or a second condition classifier. FieldAccess is structural rather
than field-name-specific and is admitted only when its base is already an
admitted value expression. The shared Facts helper remains the only admission
owner and Lower does not reclassify the predicate.

Required proof:

```text
minimal fixture:
  loop(true)
  receiver field != null in an and-condition
  nested conditional return
  continue/break tail

route:
  loop_true_break_continue / exit-allowed recipe

reject regression:
  unsupported projection roots remain unsupported

full consumer:
  json_native strict_policy_test reaches execution
```

This is a compiler-expressivity prerequisite, not an HMI semantic widening.

Closeout evidence:

```text
status:
  closed

shared Facts:
  Me / This roots and recursive FieldAccess admitted

lowering:
  existing PlanNormalizer only

focused unit tests:
  expr_value 5/5
  receiver-field nested-exit recipe 1/1

fixture:
  phase29bq_loop_true_receiver_field_nested_exit_min.hako

fast gate:
  loop_true_receiver_field_nested_exit PASS

consumer proof:
  json_native strict_policy_test -> [json-native/strict-policy] ok
```

J0 remains open for the strict json_native policy itself.

### HMI-S0-E0 — exact-none control-edge witness

```text
production execution delta: 0
JSON artifact metadata delta: one function witness
HMI execution callers: 0

verify before function JSON publication:
  Jump.edge_args == None
  Branch.then_edge_args == None
  Branch.else_edge_args == None

emit:
  function.metadata.control_edge_args_v1.mode = exact_none
```

The same verifier is required before Rust-oracle parity execution. Current
Rust terminator execution ignores edge args and is not a valid oracle for
functions with `Some(...)` edge arguments.

### HMI-S0-T0 — whole-document MIR profile seal

```text
production behavior delta: 0
HMI execution callers: 0

add:
  strict json_native ingress facade
  root/function/CFG/block/instruction/type/PHI/ownership validation
  exact allowed/required field sets
  function/CFG name bijection and CFG-owned entry block
  exact-none edge witness requirement
  opaque VerifiedHmiDocumentView
```

No function/block/instruction graph or enum is decoded into a second schema.
Views borrow or index the same sealed JsonNode tree.

### HMI-S0-V0 — disconnected scalar state machine

```text
production behavior delta: 0
HMI production callers: 0

state:
  selected function key
  current block
  predecessor = Entry | Block(id)
  typed scalar registers
  Running | Returned(value) | ReturnedNoValue | Failed(error)
  harness-only step bound
```

The state owns no MIR semantic rows and uses no Rust `VMValue`.

### HMI-S0-I0 — exact portable handlers

Executable disconnected subset:

```text
Const i64
Const Bool reconstructed from payload plus value_types=i1
Copy
BinOp Add/Sub/Mul/Div/Mod with checked law
Jump
Branch with exact i1 condition
Phi i64/i1 with exact predecessor input
Return i64/Bool/no-value
```

Transport/seal contract only:

```text
CopyOwned
DestroyOwned
```

Their Box execution remains blocked on SSA-I1-O1. `ReleaseStrong` is rejected.

### HMI-S0-P0 — Rust-oracle parity

```text
production behavior delta: 0
HMI production callers: 0

compare:
  non-overflowing result domain
  division/modulo by zero typed failure
  exact predecessor-sensitive Phi result
  selected final register/state observations

HMI-only portable proofs:
  Add/Sub/Mul overflow
  MIN/-1 Div/Mod overflow
  debug/release identical HMI failures
```

Rust raw overflow is not used as an oracle. The old Rust interpreter remains
available only as the bounded common-domain reference until later cutover.

## Strict JSON fixtures

Pass:

```text
exact root plus trailing whitespace
distinct int/bool/null/string JsonNode kinds
nested ordered object/array
i64 0/MAX/MIN
escaped string only inside the explicitly supported lexical profile
default parser compatibility fixtures unchanged
```

Reject:

```text
duplicate decoded key before object_set
root/nested/array-object duplicate
trailing second value or garbage
i64 MAX+1 and MIN-1
leading zero, plus sign, incomplete fraction/exponent
unsupported escape or Unicode profile
missing key confused with explicit null
strict failure followed by tolerant fallback
```

Compatibility fixture:

```text
default parser duplicate input remains last-wins
default parse and JsonParserUtils API/return/error behavior unchanged
```

## Edge witness fixtures

Pass:

```text
Branch and Jump with all edge Option fields None
function witness emitted exactly once
strict ingress accepts witness
```

Reject before JSON publication:

```text
Jump Some(nonempty)
Jump Some(empty)
Branch one Some arm
Branch both Some arms
```

Reject at ingress:

```text
Branch/Jump function with missing witness
wrong witness mode
duplicate witness row
```

## Arithmetic fixtures

Run the HMI handler fixtures through debug and release execution:

```text
normal Add/Sub/Mul/Div/Mod
-7 / 3 = -2
-7 % 3 = -1
MAX + 1 -> ArithmeticOverflow
MIN - 1 -> ArithmeticOverflow
MAX * 2 -> ArithmeticOverflow
division by zero -> DivisionByZero
modulo by zero -> DivisionByZero
MIN / -1 -> ArithmeticOverflow
MIN % -1 -> ArithmeticOverflow
```

No Rust panic or process abort is an accepted result.

## Whole-document fixtures

Pass:

```text
entry block not lowest block id
multiple functions with exact cfg/function bijection
PHI prefix followed by ordinary instructions
two-input and multi-input Phi
Return without value
CopyOwned/DestroyOwned with exact ownership witness, execution blocked
```

Reject before state allocation:

```text
unknown bounded-row field
missing/extra CFG function
entry outside function
duplicate function/block/value-type identity
PHI outside prefix or predecessor mismatch
missing/multiple/non-final terminator
Bool metadata/payload mismatch
void Const or Unknown value type
unsupported value class/opcode/operator
CopyOwned/DestroyOwned ownership mismatch
ReleaseStrong
```

## Stable failure families

Implementation may register these only when an emitting row lands:

```text
[freeze:contract][hmi/json/duplicate-key]
[freeze:contract][hmi/json/integer-range]
[freeze:contract][hmi/mir_json_v1/document]
[freeze:contract][hmi/mir_json_v1/cfg]
[freeze:contract][hmi/mir_json_v1/value_type]
[freeze:contract][hmi/mir_json_v1/ownership]
[freeze:contract][hmi/i64/arithmetic-overflow]
[freeze:contract][hmi/i64/division-by-zero]
```

Rust producer rejection uses a separate source-side family:

```text
[mir_json_v1/control_edge_args_not_exact_none]
```

## Counters and guards

```text
native JSON grammar authorities added = 0
HMI custom recursive JSON parsers = 0
HMI plain/tolerant parser calls = 0
HMI V1-to-v0 conversions = 0
HMI compact normalization = 0
second MIR instruction schemas = 0
effects before document seal = 0
partial document execution = 0
Rust fallback = 0
route retry = 0
runtime handler discovery = 0
HMI production callers = 0
ReleaseStrong admission = 0
BoxRef execution before O1 = 0
source/check files >= 800 lines = 0
```

## Implementation may claim after S0-P0

```text
one opt-in strict policy extends the existing native JSON authority
one whole-document HMI profile seals the selected MIR JSON V1 carrier
one disconnected state machine executes the exact scalar subset
plain i64 arithmetic has checked build-independent semantics
CFG edge arguments are exact-none rather than silently dropped
Rust parity is green inside the declared common domain
production execution ownership remains unchanged
```

## Implementation must not claim

```text
general JSON parser replacement
all JSON escapes/Unicode unless separately proven
all MIR JSON V1 support
Call/MethodCall execution
BoxRef or ownership execution
Null/Void identity parity
dynamic truthiness
Float/String/operator-box behavior
product VM replacement
Rust handler retirement
parser/MirBuilder or Ownership V2 progress
```

## Stop conditions

1. A third native JSON grammar/parser authority is introduced.
2. Duplicate detection happens after `object_set`.
3. Strict integer range proof happens after unchecked host conversion.
4. Default json_native parse or object overwrite behavior changes.
5. MIR field vocabulary enters the generic JSON policy.
6. HMI uses the tolerant tools/hako_shared parser.
7. Missing/null is decided without `object_has` plus kind checks.
8. Edge args are inferred empty from absent payload fields.
9. `Some(empty)` is accepted as exact `None`.
10. Rust build-profile overflow becomes semantic authority.
11. A second MIR graph/instruction schema is decoded.
12. Failed seal allocates state or executes an instruction.
13. Unsupported input falls back to Rust/V0/compact routes.
14. Product callers connect before the later cutover row.
15. Any source/check file reaches 800 lines.

## Final decision lock

> HMI-S0 selects B-prime. The existing `apps/lib/json_native` lexer, parser,
> and ordered scalar-kind-preserving JsonNode remain the single native JSON
> authority. HMI-S0-J0 adds only an opt-in strict policy for duplicate-before-
> insert and exact integer-range admission while default compatibility behavior
> stays unchanged. HMI-S0-E0 proves every final Jump/Branch edge argument is
> exact `None`, emits one function metadata witness, and rejects `Some(empty)`;
> no instruction payload dialect is added. HMI-S0-T0 seals the entire selected
> MIR JSON V1 tree and exposes bounded views over that same tree. V0/I0 then add
> a disconnected typed scalar state machine and checked i64/Bool handlers for
> Const, Copy, Add/Sub/Mul/Div/Mod, Jump, Branch, Phi, and Return. Plain
> arithmetic is checked; wrapping requires future explicit vocabulary. P0
> proves Rust parity only in the declared common domain. Production callers,
> fallback, BoxRef execution, and execution-owner change remain zero.
