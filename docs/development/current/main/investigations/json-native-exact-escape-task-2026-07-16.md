---
Status: accepted; D0 closed; S0 next
Date: 2026-07-16
Decision: A1 exact representable JSON escape profile
Parent: json-native-escaped-string-design-stop-2026-07-16.md
Blocked parent row: JSON-NATIVE-ITER0-P0
Resume target: JSON-NATIVE-ITER0-P0
Scope: one lexical BoxCount prerequisite; parser selection and HMI stay unchanged
---

# JSON native exact escape task

## Decision lock

Select the worker-refined Candidate A1.

```text
JSON-NATIVE-ESC0-D0
  -> JSON-NATIVE-ESC0-S0
  -> JSON-NATIVE-ESC0-I1
  -> resume JSON-NATIVE-ITER0-P0
```

The earlier phrase `exact \u00XX ASCII` is rejected as imprecise:

```text
ASCII:
  U+0000..U+007F

\u00XX:
  U+0000..U+00FF

current exact character construction:
  not every U+0000..U+007F value
```

ESC0 admits only values that the current `.hako` string substrate can
materialize exactly. Unsupported values fail before token/tree publication;
they never become `?`.

## Exact first profile

### Simple escapes

All eight JSON simple escapes are admitted in keys and values:

```text
\"  -> quote
\\  -> backslash
\/  -> slash
\b  -> backspace
\f  -> form feed
\n  -> newline
\r  -> carriage return
\t  -> tab
```

### Unicode escapes

Four hex digits are case-insensitive. The admitted decoded set is exactly:

```text
selected controls:
  U+0008 U+0009 U+000A U+000C U+000D

printable ASCII:
  U+0020..U+007E
```

Examples:

```text
\u0061 -> a
\u006A -> j
\u006a -> j
\u0022 -> quote
\u005C -> backslash
```

Explicitly rejected in this row:

```text
U+0000
U+0001..U+0007
U+000B
U+000E..U+001F
U+007F
U+0080 and above
high/low surrogate halves
surrogate pairs
```

NUL and the remaining ASCII controls require an exact code-unit construction
owner. Non-ASCII and surrogate composition belong to the accepted decoded
UTF-8 workstream, not ESC0. This row does not normalize Unicode.

## Current-source evidence

Three independent worker audits and a direct P0 probe established:

```text
JsonScanner.read_string_literal:
  every backslash -> null

JsonTokenizer:
  null scan -> ERROR("Unterminated string literal")

EscapeUtils.unescape_string:
  unknown/truncated escape -> tolerant literal preservation
  unsupported Unicode/surrogate -> ?
  lowercase hex accepted by validation but missed by uppercase mapping rows

parser policy:
  never reached for escaped strings
```

The parser already consumes decoded STRING tokens. Duplicate-key policy is
therefore downstream and must not gain escape grammar.

## Authority split

| Concern | Authority | Non-authority |
| --- | --- | --- |
| quote/body span traversal | `JsonScanner` | escape policy/parser |
| one escape sequence admission/materialization | exact escape decoder V1 | tolerant legacy utility |
| decoded STRING or typed ERROR token | `JsonTokenizer` | parser/HMI |
| lexical error kind/code/site | existing `JsonToken` plus optional error metadata | rendered prose |
| decoded duplicate policy | existing strict/default parser policy | scanner/decoder |
| tree construction | existing `JsonNode` path | escape decoder |

### Scanner law

Scanner owns only string boundary traversal:

```text
opening quote:
  required

ordinary character:
  advance one

backslash:
  shields the next source character from quote termination
  scanner does not decide which escape is legal

raw U+0000..U+001F:
  reject at the raw character site

unescaped closing quote:
  closes the body

EOF before closing quote:
  unterminated-string
```

It returns one typed scan result. Mutable `last_error` state and a second
string scanner are forbidden.

### Decoder law

One exact decoder consumes the scanner-proven raw body and returns:

```text
success:
  decoded StringBox

failure:
  kind
  stable code
  relative escape offset
  expected
  actual
```

The selected tokenizer path must not use tolerant preservation or the legacy
`hex_to_char -> ?` fallback. Lowercase and uppercase hex spellings produce the
same value.

### Tokenizer law

Tokenizer remains the sole token owner:

```text
scan success + decode success:
  one STRING token with decoded payload

scan/decode failure:
  one ERROR token at the exact absolute site

partial STRING token:
  0
```

Optional lexical error metadata is added to the existing `JsonToken`; a new
error-token vocabulary is not introduced. The iterative parser projects that
metadata into `JsonParseErrorV1`. The recursive parser may retain its raw-token
compatibility surface.

## Stable error vocabulary

```text
[json_native/lexer/unterminated-string-v1]
[json_native/lexer/raw-control-v1]
[json_native/lexer/trailing-escape-v1]
[json_native/lexer/invalid-simple-escape-v1]
[json_native/lexer/incomplete-unicode-escape-v1]
[json_native/lexer/invalid-unicode-hex-v1]
[json_native/lexer/unsupported-ascii-control-v1]
[json_native/lexer/unsupported-unicode-v1]
[json_native/lexer/unsupported-surrogate-v1]
```

Site law:

```text
unterminated string:
  opening quote; EOF is detail

raw control:
  offending raw character

escape grammar/admission failure:
  introducing backslash
```

English messages are presentation only. Kind/code/position/line/column and
expected/actual are contract fields.

## Physical structure

Preferred files:

```text
apps/lib/json_native/lexer/string_scan_result_v1.hako
  typed span/boundary result; target <120 lines

apps/lib/json_native/lexer/string_escape_decoder_v1.hako
  sole exact escape grammar/materializer; target <250 lines

apps/lib/json_native/lexer/scanner.hako
  scan_string_literal_v1 + legacy thin facade

apps/lib/json_native/lexer/token.hako
  optional typed lexical-error metadata

apps/lib/json_native/lexer/tokenizer.hako
  scan/decode/site projection only

apps/lib/json_native/parser/iterative_engine_v1.hako
  preserve lexical metadata when projecting errors

apps/lib/json_native/tests/escaped_string_esc0_test.hako
  focused executable contract; target <300 lines
```

`EscapeUtils.unescape_string` remains a legacy/tolerant utility but has zero
selected tokenizer callers after I1. It is not strict admission authority.
Do not grow its current 430-line mixed serializer/parser helper into the new
owner.

## Exact task order

### JSON-NATIVE-ESC0-D0 — decision lock

Status: closed by this card. Behavior delta is zero.

```text
A1 exact set selected
raw span / exact decode / token publication owners separated
typed errors and sites selected
full ASCII/full Unicode/NUL claims rejected
P0 stash remains parked
```

### JSON-NATIVE-ESC0-S0 — disconnected exact products

Next code-facing row. Production behavior delta is zero.

Add the typed scan result and exact decoder. Add a disconnected scanner entry
and direct fixtures. Existing `JsonTokenizer.tokenize_string` remains selected
until I1.

S0 must prove:

```text
all eight simple escape decodes
selected Unicode set and case-insensitive hex
every rejected class returns typed kind/code/relative site
escaped quote does not terminate the scanner
scanner publishes no partial body on failure
selected production tokenizer callers = 0
```

### JSON-NATIVE-ESC0-I1 — atomic lexical activation

One BoxCount behavior commit:

```text
JsonTokenizer selects scan_string_literal_v1
JsonTokenizer selects exact decoder V1
legacy read_string_literal delegates to the same scan owner
iterative lexer-error projection preserves typed metadata
default and strict entries share the identical token stream
tolerant EscapeUtils selected-tokenizer callers = 0
```

I1 does not switch recursive/iterative parser selection, perform CUT0, or
activate HMI.

### Resume JSON-NATIVE-ITER0-P0

Only after I1 is committed/pushed and the worktree is clean. Do not apply the
entire old stash. Recover and re-review only:

```text
apps/lib/json_native/tests/iterative_parser_p0_parity_test.hako
apps/lib/json_native/tests/support/error_parity_v1.hako
apps/lib/json_native/tests/support/tree_pair_walker_v1.hako
```

Restore the real `"a"` versus `"\u0061"` fixtures and align their lexer-error
projection with final ESC0 metadata.

## Pass fixtures

### Scanner/decoder

```text
all eight simple escapes in a value
all eight simple escapes in a key
ordinary prefix/suffix around escapes
escaped quote shielding
escaped backslash followed by closing quote
multiple escapes in one body
multiline document site accounting
U+0008/0009/000A/000C/000D boundaries
U+0020 and U+007E boundaries
\u0041 / \u0061 / \u006A / \u006a
```

### Parser-visible behavior

```text
strict:
  {"a":1,"\u0061":2} -> duplicate-key
  {"\u0061":1,"a":2} -> duplicate-key
  duplicate wins before colon validation
  site remains post-key current token
  root publication = 0

compatibility:
  same two inputs
  first key position retained
  second value wins
  key count = 1

old recursive and disconnected iterative parser:
  same decoded tree/error site
```

### Reuse

```text
escaped success followed by ordinary success
escape failure followed by ordinary success
strict failure followed by compatibility parse only when explicitly invoked;
automatic retry remains zero
```

## Reject fixtures

```text
\x
\q
trailing backslash at EOF
backslash before missing closing quote
\u
\u0
\u00
\u000
\u00G1
\u0G00
\u0000
\u0001
\u000B
\u001F
\u007F
\u0080
\u00FF
high surrogate
low surrogate
surrogate pair
raw tab/newline/carriage return/control
valid escape followed by missing closing quote
```

Every reject must fail during lexer preflight before parser frame/tree effects.
Malformed escape has priority over duplicate-key policy because no STRING token
is published.

## Active fixture and guard policy

Authoritative retained fixtures:

```text
escaped_string_esc0_test
iterative_parser_l0_test
iterative_parser_s0_test
strict_policy_test
```

The following remain stale and are not repaired inside ESC0:

```text
compat_smoke
unit/core_test
unit/utils_test
yyjson_replacement_test
phase2_accuracy_test
final_integration_test
integration/full_test
```

They require a separate fixture repair/retirement slice. Print/demo success is
not semantic proof.

Required gates:

```text
release and debug VM escaped_string_esc0_test
release and debug VM iterative_parser_s0_test
iterative_parser_l0_test
strict_policy_test
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Do not add a one-off shell guard. If source-structure guarding is needed, add
one reusable json_native parser-family manifest row and document its stable
entry in `docs/tools/check-scripts-index.md`.

## Authority counters

```text
selected JsonScanner = 1
selected JsonTokenizer = 1
selected exact token decoder = 1
new tokenizer/token vocabulary = 0
new parser/tree authority = 0

unconditional any-backslash rejection after I1 = 0
tolerant EscapeUtils selected-tokenizer callers after I1 = 0
unsupported Unicode -> ? admitted paths = 0
lowercase/uppercase hex semantic drift = 0

strict/default tokenizer route split = 0
parser-side escape grammar = 0
HMI-field-specific escape grammar = 0
automatic retry/fallback = 0

recursive/iterative production selector delta = 0
VM MAX_CALL_DEPTH delta = 0
HMI production callers = 0
P0/CUT0 production delta during ESC0 = 0

touched source/check files at or above 800 lines = 0
```

## Implementation may claim after I1

```text
one exact selected escaped-string token path
all eight simple JSON escapes
case-insensitive selected Unicode escape subset
decoded duplicate identity for a and \u0061
typed fail-fast for every explicitly rejected escape class
shared compatibility/strict lexical behavior
no silent ? replacement on the selected path
```

## Implementation must not claim

```text
all ASCII escapes
all \u00XX
U+0000 support
DEL/remaining C0 support
non-ASCII Unicode escapes
surrogate composition
Unicode normalization
general JSON conformance
decoded UTF-8 byte-budget completion
iterative parser production cutover
HMI seal/opcode/cutover
performance improvement
source compatibility of stale demos
```

## Stop conditions

Stop if implementation requires:

1. scanner and decoder both deciding escape grammar;
2. a second tokenizer, token vocabulary, parser, or JSON tree;
3. decoded output inspection to infer whether the raw source was valid;
4. tolerant unknown/truncated preservation on the selected token path;
5. `?` as an admitted unsupported-Unicode result;
6. claiming all ASCII without an exact code-unit constructor;
7. parser policy, HMI field names, file names, or payload shape in lexing;
8. strict-to-compat or exact-to-legacy retry/probing/toggles;
9. repairing stale fixtures in the ESC0 behavior commit;
10. applying the full P0 stash before ESC0 closes;
11. mixing ITER0-P0, CUT0, HMI, or VM-depth changes into ESC0;
12. a source/check file at or above 800 lines;
13. a new CorePlan/JoinIR acceptance shape hidden as a `.hako` workaround.

If the exact scanner/decoder exposes a compiler expressivity gap, stop and
task that single shape with its own fixture/gate before resuming ESC0.
