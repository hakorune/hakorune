---
Status: closed through CUT0; HMI-S0-T0 resume next
Date: 2026-07-16
Decision: B-prime one iterative grammar engine and one text-to-tree owner
Parent stop: hmi-s0-t0-json-parser-depth-consultation-question-2026-07-16.md
Resume target: HMI-S0-T0
Scope: BoxShape prerequisite; HMI execution callers remain zero
---

# JSON Native iterative parser task

## Decision lock

`apps/lib/json_native` will keep one tokenizer, one explicit-stack grammar
engine, and one `JsonNode` construction path for compatibility and strict
parsing.

```text
JsonTokenizer
  -> JsonTokenCursorV1
  -> JsonIterativeParserEngineV1
       stack: JsonParseFrameV1[]
       policy: compatibility | StrictJsonPolicyV1
  -> existing JsonNode factories and mutation API
```

CUT0 also removes the independent `JsonNode.parse()` text grammar. `JsonNode`
remains the JSON value/tree owner; text-to-tree grammar belongs only to the
iterative parser.

```text
selected tokenizer count: 1
selected grammar-engine count: 1
selected text-to-JsonNode owner count: 1
intermediate JSON AST count: 0
```

This series is a BoxShape prerequisite. It does not seal MIR JSON, execute an
HMI opcode, change VM call depth, or add a product caller.

## Current-source audit

The selected repair is grounded in the current tree:

```text
full recursive grammar:
  apps/lib/json_native/parser/parser.hako
  parse_value -> parse_object/parse_array -> parse_value

second mini grammar:
  apps/lib/json_native/core/node.hako::JsonNode.parse

JsonNode.parse current repository surface:
  19 call sites
  5 .hako files

VM resource boundary:
  src/backend/mir_interpreter/exec/frame_transaction.rs
  MAX_CALL_DEPTH = 16
```

The tokenizer already emits one flat token sequence. Existing `JsonNode`
factories and object insertion-order arrays remain the value/tree substrate.

The current parser error array primarily exposes rendered strings. L0 adds one
typed parser-error authority and keeps the existing compatibility rendering as
a projection. Existing string messages do not become kind/site authority.

## Authority split

| Concern | Authority | Non-authority |
| --- | --- | --- |
| lexical tokens | existing `JsonTokenizer` | iterative engine policy |
| token position | `JsonTokenCursorV1` | policy callbacks |
| delimiter/state grammar | `JsonIterativeParserEngineV1` | HMI seal |
| open-container state | `JsonParseFrameV1` stack | VM call stack |
| parser resource limit | `JSON_NATIVE_MAX_CONTAINER_DEPTH_V1` | env/backend/policy |
| JSON value/tree | existing `JsonNode` factories/API | temporary parser frames |
| duplicate/i64 admission | selected policy | delimiter engine |
| typed parser failure | `JsonParseErrorV1` | rendered compatibility string |
| MIR semantic admission | later HMI-S0-T0 seal | JSON parser |

Policies may decide only decoded duplicate-key and integer-lexeme admission and
their typed policy failures. They may not consume/rewind tokens, push/pop
frames, choose delimiters, construct containers, attach children, change the
depth limit, recover, retry, or inspect HMI field names.

## Frame and publication law

Each frame owns:

```text
container_kind: array | object
phase:
  array.first-or-end
  array.value
  array.comma-or-end
  object.first-key-or-end
  object.key
  object.colon
  object.value
  object.comma-or-end
node
pending_key
open token site
```

Root state is separate:

```text
root.expect-value
root.expect-eof
```

All completed values pass through one publication operation:

```text
empty stack:
  publish root exactly once

array parent waiting for value:
  array_push(value)

object parent waiting for value:
  object_set(pending_key, value)

any other state:
  internal parser-state invariant failure
```

A child container is frame-local until its closing delimiter is verified. It
is popped before its completed node is attached to its parent. Failed parsing
therefore publishes neither a root nor a partial child into its parent.

```text
token cursor rewind: 0
recursive container-parser calls: 0
root publication on success: exactly 1
root publication on error: 0
```

## Resource law

```text
JSON_NATIVE_MAX_CONTAINER_DEPTH_V1 = 128

top-level scalar depth: 0
top-level array/object depth: 1
new open container attempted depth: frame_stack.length + 1
attempted depth <= 128: accept
attempted depth == 129: reject before node allocation/frame push
```

Stable failure:

```text
code: [json_native/parser/nesting-limit-v1]
kind: nesting-limit
fields:
  max_depth = 128
  attempted_depth = 129
  position
  line
  column
  token = LBRACE | LBRACKET
```

This is a versioned json_native parser resource limit, not a JSON language
limit, HMI semantic limit, or VM recursion limit. Compatibility and strict
entries use the same limit. No environment/backend override is allowed.

## Normalized parity law

Successful old/new parity uses a test-only iterative event walk:

```text
Scalar(kind, exact value)
ArrayStart(length) / children in order / ArrayEnd
ObjectStart(key_count) / Key(decoded key) in insertion order / value / ObjectEnd
```

It compares exact scalar values, array order, decoded object-key insertion
order, compatibility duplicate final value, root cardinality, and whole-input
consumption. It does not compare Box identity, allocation count, helper call
count, or temporary frames.

`stringify()` is not parity authority: object stringify may reorder keys and a
deep recursive stringify would reintroduce the unrelated VM-depth problem.

Malformed parity compares the first typed error's:

```text
kind/code
offending token type and start site
line/column
expected/actual token class
first-error priority
strict policy kind when applicable
```

English message bytes are presentation only. Old recursive-parser VM-depth
failures are outside semantic parity; they are the resource failure being
replaced.

## Exact task order

### JSON-NATIVE-ITER0-D0 — decision lock

Status: closed by this card. Production behavior delta is zero.

```text
B-prime selected
depth 128 and typed error selected
policy-port limits selected
normalized success/error parity selected
JsonNode.parse physical retirement selected
HMI-S0-T0 remains parked
```

### JSON-NATIVE-ITER0-L0 — passive vocabulary

Status: closed. Production behavior delta is zero.

Add small files under `apps/lib/json_native/parser/`:

```text
token_cursor_v1.hako
frame_v1.hako
error_v1.hako
resource_limits_v1.hako
```

L0 owns monotonic cursor vocabulary, frame phases, typed parser errors, and the
single depth constant. Production parser callers and iterative engine callers
remain zero.

L0 closeout:

```text
cursor:
  JsonTokenCursorV1
  one-shot text initialization through the existing JsonTokenizer
  typed ArrayBox/IntegerBox/BoolBox storage
  current/peek/advance only; rewind/reset = 0

frame:
  JsonParseFrameV1
  exact array/object phase vocabulary
  frame-local node, pending key, and opening token site

error:
  JsonParseErrorV1
  typed kind/code/site/expected/actual owner
  MapBox and rendered text are one-way projections

resource law:
  JsonParserResourceLimitsV1.max_container_depth() = 128
  one physical definition

production JsonParser references to L0 products:
  0

fixture:
  iterative_parser_l0_test -> [json-native/iter0-l0] ok

retained strict/default proof:
  strict_policy_test -> [json-native/strict-policy] ok

quick gate:
  66/66

file sizes:
  all five new source/fixture files below 140 lines
```

The one-shot cursor initializes from source text rather than accepting an
`ArrayBox` parameter. The current reference-VM route drops an untyped
collection when it crosses that method boundary; the cursor therefore owns a
typed `ArrayBox` field and invokes the existing tokenizer directly. Token
grammar and construction remain owned by `JsonTokenizer`.

`apps/lib/json_native/tests/compat_smoke.hako` is not a current green baseline:
it fails at the pre-existing undeclared `doc` assignment before exercising L0.
L0 does not repair or use that unrelated fixture. P0 must inventory the active
compatibility fixture set and either repair this stale fixture in a separate
fixture-only slice or exclude it with a documented retirement decision.

### JSON-NATIVE-ITER0-S0 — disconnected iterative engine

Status: closed. Production selection remains unchanged.

Add `iterative_engine_v1.hako`. Direct fixtures exercise compatibility and
strict policies, ordinary T0 MIR JSON, mixed/empty containers, and deep inputs.

```text
production JsonParser selection: recursive engine remains 1
iterative production selectors: 0
HMI callers: 0
```

S0 closeout:

```text
engine:
  apps/lib/json_native/parser/iterative_engine_v1.hako
  542 lines
  one explicit ArrayBox frame stack
  no recursive container-parser calls

direct fixture:
  scalar kinds and empty/mixed containers
  strict i64 MIN/MAX/range rejection
  strict/compat decoded duplicate split
  duplicate-before-colon error priority
  trailing comma/missing colon/mismatched closer/trailing root
  alternating depth 24 and exact depth 128
  depth 129 typed rejection and parser reuse
  ordinary nested MIR carrier
  all lexer errors preflighted before grammar effects

before-effects law:
  lexer errors come from the existing tokenizer before frames/nodes
  failed/trailing roots are never externally published
  depth 129 rejects before child-node allocation/frame push

representation laws:
  frame close = get(last) -> remove(last) -> publish
  ArrayBox remove/pop return value is not an authority
  EOF is never consumed for diagnostics
  duplicate error site remains the post-key current token

production JsonParser/JsonNode iterative selectors:
  0

release/debug vm-reference fixtures:
  [json-native/iter0-s0] ok

retained fixtures:
  [json-native/iter0-l0] ok
  [json-native/strict-policy] ok

quick gate:
  66/66

VM MAX_CALL_DEPTH:
  unchanged at 16
```

### JSON-NATIVE-ITER0-P0 — normalized parity proof

Status: closed. Production behavior remains unchanged.

Prove existing compatibility behavior, J0 strict behavior, valid generated
trees, shallow malformed first-error parity, ordinary MIR JSON, and exact depth
boundaries. Production selection remains unchanged.

The test-only non-recursive pair walker and fixture-owned error parity
prototype is green for the admitted unescaped-string domain, strict literal
duplicates, grammar/lexer errors, and resource fixtures. It is stashed rather
than committed because the required decoded Unicode-key fixture exposes a
pre-existing tokenizer boundary:

```text
JsonScanner.read_string_literal:
  any backslash -> null

JsonTokenizer:
  ERROR("Unterminated string literal")
```

ESC0 is now closed independently. The shared tokenizer decodes the exact
selected escape subset before either parser, preserves typed lexical metadata,
and proves `"a"` versus `"\u0061"` duplicate identity for compatibility and
strict parsing. P0 may now restore only its three test-only parity files and
rebase their lexical expectations onto that final ESC0 contract.

The repository fixture audit also found that only L0, S0, and J0 strict tests
are active green authorities. Seven legacy json_native files fail before their
intended assertions or are print/demo fixtures. They are not counted as P0
proof and must receive a separate fixture repair/retirement decision; P0 does
not patch them opportunistically.

Closeout evidence:

```text
selectively restored test-only files:
  iterative_parser_p0_parity_test.hako
  support/error_parity_v1.hako
  support/tree_pair_walker_v1.hako

production parser selector delta:
  0

normalized tree parity:
  scalar / container / value / key order / compatibility overwrite

normalized first-error parity:
  kind / code / token / position / line / column

decoded key identity:
  a == \\u0061

resource law:
  alternating depth 24 pass
  depth 128 pass
  depth 129 reject before root publication

reuse after grammar/depth failure:
  green

focused and retained release/debug fixtures:
  green

quick gate:
  66/66

recursive fixture walker / deep stringify:
  0

touched source/check files at or above 800 lines:
  0
```

### JSON-NATIVE-ITER0-CUT0 — atomic cutover and retirement

Status: closed.

Both exact MatchReturn compiler rows are green:

```text
MR-SD0-I1: closed
MR-IS0-I1: closed
CUT0 resume: next
```

Task owner:

```text
docs/development/current/main/investigations/
  mirbuilder-match-return-json-literal-profiles-task-2026-07-16.md
```

The CUT0 WIP remains parked at immutable stash commit
`c0cfc7bddda8b4ca3b7bc4bd68a096440fbb9df4`, based on
`a4901f3cc783d4a8172ec1862a3ebdd44e1621a1`. Compiler rows must not touch the
parked parser/JsonNode surface. Both compiler prerequisite commits are now
green; after the MR-IS0-I1 commit is pushed and the worktree is clean, CUT0
applies the stash by immutable hash.

When resumed, one commit must:

```text
JsonParser.parse -> iterative engine only
JsonParser.parse_with_policy -> same iterative engine only
delete recursive parse_value/object/array grammar
delete JsonNode.parse mini grammar
migrate all retained JsonNode.parse callers to one parser facade
migrate old fixtures to normalized expectations
install guards proving selector/retry/old-authority zero
```

No landed production state may probe between engines. Rollback is a Git revert,
not runtime fallback. CUT0 includes closeout guards/docs; no separate G0 row is
added.

CUT0 closeout:

```text
selected tokenizer:
  1

selected grammar/text-to-tree engine:
  JsonIterativeParserEngineV1

JsonParser:
  thin stateful facade
  compatibility and strict share the same engine

recursive parse_value/object/array:
  definitions/callers 0

JsonNode.parse:
  definition/callers 0

typed error authority:
  JsonParseErrorV1
  facade MapBox rows are one-way projections

release/debug:
  P0 / S0 / L0 / strict / ESC0 all green

authority guard:
  json-native-parser-authority green

quick:
  66/66

VM MAX_CALL_DEPTH:
  unchanged at 16

source/check files at or above 800 lines:
  0
```

The `json_pp_vm_llvm` consumer now passes the former MatchReturn compiler
boundary and stops on independent backend availability: the explicit VM-Hako
lane reports unsupported `array_element_write`, while the current binary lacks
the LLVM feature. The two historical AST error smokes similarly select
VM-Hako/LLVM-specific incomplete routes and print `[FAIL]` while returning
zero. CUT0 does not rewrite their expected output or treat that false-green as
acceptance evidence.

### Resume — HMI-S0-T0

CUT0 is green. Resume HMI-S0-T0 from immutable WIP stash commit
`66725ad4ddd5d52a50acc03dc7c5a0e470d8bcc0`, not from a mutable stash ordinal.
The stash is not authority and must not be restored wholesale. Reapply only
pieces compatible with the final iterative parser and typed-error contracts.

## Required fixtures

Pass:

```text
all existing json_native compatibility fixtures
J0 exact i64 MIN/MAX
J0 decoded duplicate split:
  strict reject / compatibility first-position plus last-value
trailing whitespace
ordinary T0 MIR JSON carrier without VM-depth failure
top-level scalar depth 0
empty object/array
mixed object/array
alternating 24 containers
exactly 128 containers
decoded Unicode keys
parse failure followed by valid parse on same parser
parser reuse after depth error
```

Reject:

```text
129 containers -> nesting-limit-v1
strict duplicate -> duplicate-key
strict out-of-range integer -> integer-range
second root -> trailing-root
missing object colon
trailing object comma
trailing array comma
mismatched closing delimiter
unexpected EOF
invalid lexer token
```

Deep fixture builders and inspectors must be loop-based:

```text
deep fixture JsonNode.stringify calls = 0
deep fixture recursive tree walkers = 0
```

## Counters and guards

```text
selected JSON tokenizers = 1
selected JSON grammar engines after CUT0 = 1
selected text-to-JsonNode engines after CUT0 = 1
selected JsonNode factory modules = 1

recursive parse_value/object/array definitions after CUT0 = 0
recursive parser production callers after CUT0 = 0
JsonNode.parse definitions/callers after CUT0 = 0

iterative production selectors after CUT0 = 1
JSON_NATIVE_MAX_CONTAINER_DEPTH_V1 definitions = 1
policy cursor/frame/container operations = 0

strict-to-compat retry = 0
iterative-to-recursive retry = 0
parser-selection env toggles/payload probes = 0
HMI field names in parser = 0
intermediate JSON AST products = 0

VM MAX_CALL_DEPTH delta = 0
HMI production execution callers = 0
HMI seal/opcode activation during ITER0 = 0
V1-to-v0/compact translation = 0
HMI-to-Rust fallback = 0

error kind/site parity failures = 0
frame stack after successful parse = 0
root publication on failed parse = 0
source/check files at or above 800 lines = 0
```

## Implementation may claim after CUT0

```text
one iterative json_native JSON grammar engine
one shared compatibility/strict tokenizer, grammar, tree builder, limit, and site law
policy differences limited to bounded duplicate/integer decisions
ordinary MIR JSON parses without recursive container calls
container depths 0..128 obey the parser resource law
attempted depth 129 fails before child allocation
normalized compatibility and strict parity on the admitted domain
recursive full parser and JsonNode.parse mini grammar are physically retired
VM MAX_CALL_DEPTH remains 16
HMI production callers/fallback remain zero
```

## Implementation must not claim

```text
unlimited JSON nesting or a language-level depth limit
general stack-safe .hako recursion or VM trampoline support
deep stringify/all-JsonNode-operation stack safety
HMI whole-document seal/opcode/cutover completion
performance improvement without measurement
byte-for-byte diagnostic prose parity
source compatibility for removed JsonNode.parse
V1/v0/compact authority
```

## Stop conditions

Stop if implementation requires:

1. a second tokenizer, token vocabulary, JSON tree product, or grammar owner;
2. retaining the `JsonNode.parse()` mini grammar after CUT0;
3. HMI field names or MIR shapes in parser state;
4. policy control over tokens, frames, delimiters, node construction, limits,
   recovery, or retry;
5. an intermediate JSON AST or incomplete-child publication;
6. strict-to-compat or iterative-to-recursive retry/probing/toggle selection;
7. changing VM `MAX_CALL_DEPTH` or making a VM trampoline prerequisite;
8. recursive stringify/tree walking in deep fixtures;
9. HMI seal/opcode/product activation in ITER0;
10. keeping old recursive grammar as a hidden test fallback after CUT0;
11. deleting `JsonNode.parse` after discovering a versioned external contract;
12. a source/check file at or above 800 lines.
