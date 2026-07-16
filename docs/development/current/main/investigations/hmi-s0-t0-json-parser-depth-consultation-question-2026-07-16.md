# HMI-S0-T0 strict JSON parser depth consultation

Status: accepted; B-prime taskized, HMI-S0-T0 remains parked

Date: 2026-07-16

Baseline:

```text
HMI-S0-J0 strict json_native policy: 91afe54973
HMI-S0-E0 exact-none edge witness: 9a4e849b7a
next accepted row before probe: HMI-S0-T0
```

## Decision

The selected durable parser/runtime shape is **B-prime**:

```text
existing JsonTokenizer
  -> one iterative JsonParser engine
  -> one JsonNode construction path
       |-> compatibility policy
       `-> StrictJsonPolicyV1
```

Compared with candidate B, CUT0 also physically retires the independent
`JsonNode.parse()` mini grammar. Current-source audit found 19 call sites in
five `.hako` files, so retaining it would violate both the one-grammar and
one-text-to-tree-owner laws.

Accepted implementation card:

```text
docs/development/current/main/investigations/
  json-native-iterative-parser-task-2026-07-16.md
```

## Exact observed blocker

T0 began with the accepted B-prime architecture:

```text
MIR JSON V1 text
  -> apps/lib/json_native JsonParser.parse_with_policy
  -> StrictJsonPolicyV1
  -> one JsonNode tree
  -> future whole-document HMI seal
```

A disconnected T0 fixture used one valid scalar function with four blocks and
the normal nested carrier shape:

```text
root object
  functions array
    function object
      blocks array
        block object
          instructions array
            const object
              value object
```

The document string was built successfully. Execution failed inside
`JsonParser.parse_with_policy`, before it returned a JsonNode and before the HMI
seal allocated or published any view:

```text
[vm/error] Invalid instruction:
vm call stack depth exceeded (max_depth=16, fn=AddOperator.apply/2)
```

A worker audit repeated the probe with operator-box add adoption disabled. It
still failed at depth 16, this time in `JsonNodeInstance.birth/0`. Therefore the
operator route is not the owner; container recursion itself is the blocker.

The parser is recursive descent:

```text
parse_value -> parse_object -> parse_value -> parse_array -> ...
```

Each JSON container therefore consumes multiple interpreted function frames.
Ordinary MIR JSON nesting exceeds the current Rust reference interpreter limit
even though the source program itself contains no recursion.

## Why the VM limit cannot be raised locally

`src/backend/mir_interpreter/exec/frame_transaction.rs` fixes:

```text
MAX_CALL_DEPTH = 16
```

The accepted P0c-MR proof records that 1024, 128, and 32 allowed Rust host-stack
overflow before the typed VM error in focused debug execution. Sixteen is a
host-stack-safe resource boundary, not a language recursion limit. Raising it
inside HMI-S0-T0 would invalidate the existing frame-safety proof and mix a VM
trampoline problem into a JSON/seal row.

## Preserved facts

The following accepted decisions remain valid:

```text
JSON grammar/tree authority:
  apps/lib/json_native

strict-policy facts already closed:
  decoded duplicate rejection
  exact signed-i64 lexemes
  exact whole-input consumption

HMI carrier:
  Rust-emitted MIR JSON V1

forbidden:
  V1 -> V0/compact translation
  raw MirModule/AST ingress
  tolerant fallback
  second MIR instruction/CFG schema
  Rust fallback after HMI failure
```

The blocker is only the physical recursion shape of the shared JSON parser.

## Candidates

### A — HMI-only iterative strict reader

```text
json_native tokenizer
  -> new HMI-specific iterative parser
  -> JsonNode
```

Advantages:

- smallest path to T0 execution;
- compatibility parser is untouched.

Problems:

- creates a second JSON grammar/tree-construction authority;
- contradicts the accepted B-prime decision that selected one json_native
  parser with an opt-in policy;
- compatibility and strict grammar can drift.

Recommendation: reject unless B proves structurally impossible.

### B-prime — one iterative json_native parser substrate (selected)

Refactor `apps/lib/json_native/parser` so both existing compatibility parsing
and `StrictJsonPolicyV1` consume one explicit-stack parser engine.

```text
JsonTokenizer
  -> JsonParseFrameV1 stack
  -> one JsonNode builder
       |-> compatibility policy
       `-> strict policy
```

Required laws:

```text
JSON grammar owner count = 1
JsonNode construction owner count = 1
recursive parse_value/object/array production path = 0
default parse normalized parity = exact
strict J0 fixtures = green
maximum JSON nesting = explicit parser resource limit
VM MAX_CALL_DEPTH change = 0
HMI production callers = 0
```

This is a BoxShape prerequisite series, not part of the T0 semantic row.

Suggested order:

```text
JSON-NATIVE-ITER0-D0
  iterative parser frame/limit/parity contract

JSON-NATIVE-ITER0-L0
  behavior-neutral frame vocabulary and token cursor facade

JSON-NATIVE-ITER0-S0
  disconnected iterative engine with compatibility/strict policies

JSON-NATIVE-ITER0-P0
  normalized old/new compatibility parity plus strict J0 parity

JSON-NATIVE-ITER0-CUT0
  atomic parser cutover and recursive engine retirement

then resume:
  HMI-S0-T0
```

### C — iterative Rust MIR-interpreter call frames/trampoline first

Advantages:

- removes the general shallow interpreted-call limit;
- benefits deep user recursion and other `.hako` libraries.

Problems:

- much larger VM execution-owner change;
- mixes frame scheduling, diagnostics, ownership sessions, and restoration into
  a JSON ingress prerequisite;
- JSON parsing still needs its own explicit nesting resource boundary.

Recommendation: park as an independent later row, not the T0 prerequisite.

### D — raise MAX_CALL_DEPTH or special-case MIR JSON

Recommendation: reject.

This either contradicts the proven host-stack boundary or introduces
schema/name-specific parser behavior and a hidden safety regression.

## Resolved questions

1. B-prime is selected; A and D are rejected and C is parked.
2. Compatibility and strict entries share one iterative grammar/tree engine;
   only bounded policy decisions may differ.
3. The V1 parser resource limit is 128 open containers. Attempt 129 fails
   before node allocation with `[json_native/parser/nesting-limit-v1]`.
4. Normalized parity includes successful tree kind/value/order and typed first
   error kind/site/priority. English message bytes and allocation identity are
   not parity authorities.
5. CUT0 atomically selects the iterative engine and deletes both recursive
   full-parser grammar and `JsonNode.parse()` mini grammar. Probe/retry is zero.
6. T0 seal work remains parked and production callers stay zero until
   JSON-NATIVE-ITER0-CUT0 is green.

## Required fixtures for recommended B

Pass:

```text
all existing json_native compatibility fixtures
J0 strict MIN/MAX and duplicate split
ordinary MIR JSON nesting used by T0
alternating object/array nesting of at least 24 containers
empty object/array
mixed object/array nesting
decoded Unicode duplicate keys
nesting exactly at the selected limit
```

Reject:

```text
nesting limit + 1 with stable typed error
duplicate key in strict mode
same duplicate remains last-write-wins in compatibility mode
trailing second root
out-of-range integer in strict mode
malformed delimiter/state transition
```

Counters:

```text
selected JSON grammar engines = 1
selected JsonNode builders = 1
recursive parser production callers after CUT0 = 0
strict-to-compat retry = 0
HMI-to-Rust fallback = 0
VM MAX_CALL_DEPTH delta = 0
HMI execution callers before T0/V0/I0 = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop the selected prerequisite if it requires:

1. a second tokenizer or JSON token vocabulary;
2. HMI field names inside the JSON parser;
3. JSON AST conversion before JsonNode publication;
4. strict failure followed by compatibility retry;
5. VM MAX_CALL_DEPTH widening without a separate trampoline proof;
6. V1/V0/compact MIR translation;
7. parser selection by file name, method name, or runtime payload probing;
8. HMI seal or opcode activation in the parser-refactor series;
9. a source/check file at or above 800 lines.

## WIP disposition

The disconnected T0 seal prototype was not committed. It is preserved only as:

```text
stash@{0}: wip/hmi-s0-t0 (blocked by json parser call depth)
```

It must not be restored until the selected parser prerequisite is green. Its
structure may be reused or discarded after review; the stash is not authority.
