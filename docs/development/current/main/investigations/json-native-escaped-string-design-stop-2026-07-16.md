---
Status: design stop; user selection required
Date: 2026-07-16
Parent: json-native-iterative-parser-task-2026-07-16.md
Blocked row: JSON-NATIVE-ITER0-P0
WIP: stash `wip/json-native-iter0-p0 (blocked on escaped-key policy)`
---

# JSON native escaped-string design stop

## Question

`JSON-NATIVE-ITER0-P0` requires decoded Unicode-key parity, including the
strict duplicate pair `"a"` and `"\u0061"`. Current source proves that this
input never reaches either parser:

```text
JsonScanner.read_string_literal
  sees `\`
  -> returns null immediately

JsonTokenizer
  -> ERROR("Unterminated string literal")

old recursive parser:
  raw lexer error, no tree

new iterative parser:
  typed lexer-error, no tree
```

The landed J0 fixture covers literal duplicate keys only. Its current
decoded-key wording is therefore broader than its executable proof.

Should escaped JSON strings become a separate lexical prerequisite before P0,
or should the first strict profile explicitly exclude them?

## Why implementation stops here

P0 is a BoxShape parity proof. Adding escaped-string admission is a BoxCount
language/lexer change. Mixing them would violate the one-row rule and would
make the new parser appear parity-green only because the shared tokenizer
changed underneath both implementations.

The rest of the P0 prototype is green:

```text
test-only iterative pair walker:
  scalar/container/order/exact-value parity

fixture-owned error expectations:
  grammar kind/code/site/priority parity
  lexer raw-token -> typed-error field parity

strict literal duplicate and i64 range parity:
  green

depth 24 / 128 / 129 and ordinary MIR carrier:
  new-engine resource proof green
```

No production parser selector changed. The prototype is stashed rather than
committed with a weakened corpus.

## Candidate A — exact ASCII escape prerequisite (recommended)

Insert one separate code-facing row before P0:

```text
JSON-NATIVE-ESC0-D0
  -> JSON-NATIVE-ESC0-I1
  -> resume JSON-NATIVE-ITER0-P0
```

Selected lexical profile:

```text
simple JSON escapes:
  \" \\ \/ \b \f \n \r \t

Unicode escape first slice:
  exact `\u00XX` ASCII code points

non-ASCII `\uXXXX` and surrogate pairs:
  typed fail-fast until a UTF-8/code-point owner is selected
```

Implementation boundary:

```text
existing JsonScanner:
  iteratively skips and validates escape spans

existing JsonTokenizer:
  remains the sole token owner

existing EscapeUtils:
  materializes admitted decoded values

new tokenizer/parser/tree authorities:
  0
```

Required proof includes `"a"` versus `"\u0061"`, escaped quote/backslash,
invalid/truncated escapes, exact error sites, and unchanged unescaped input.
This restores the P0 requirement without pretending to support unrestricted
Unicode or surrogate composition.

## Candidate B — keep escaped strings outside the first strict profile

Revise P0 and J0 claims:

```text
decoded duplicate key:
  not yet claimed

strict HMI JSON first profile:
  backslash-containing string tokens reject before tree construction

P0:
  proves only the currently admitted unescaped-string domain
```

This is the smallest route to HMI-S0-T0, but it deliberately narrows the
accepted decision text and requires a later escaped-string row before claiming
general MIR JSON V1/JSON conformance.

## Candidate C — full Unicode JSON escapes now

Add complete code-point decoding and surrogate-pair composition before P0.
This is semantically strongest but is not a small prerequisite: current
`EscapeUtils.hex_to_char` is an ASCII-oriented table and maps unsupported code
points to `?`. UTF-8/code-point identity and diagnostics need their own owner.

## Recommendation

Select Candidate A.

It preserves the accepted decoded-duplicate safety property, keeps the first
slice exact instead of silently mapping arbitrary Unicode to `?`, and does not
force the unrelated full-Unicode substrate into the iterative-parser cutover.

## Next code-facing owner if A is selected

```text
owner:
  JsonScanner escaped-string span state

fail-fast boundary:
  unsupported/malformed escape rejects in JsonTokenizer before parser effects

non-authorities:
  parser policy
  HMI field names
  MIR shape
  VM call-depth
  fallback/retry
```

## Stop conditions

Stop the prerequisite if it requires:

1. a second tokenizer or decoded-string tree product;
2. parser/HMI-specific escape recognition;
3. silent replacement of unsupported Unicode with `?` as an exact claim;
4. strict-to-compat or new-to-old retry;
5. changing VM `MAX_CALL_DEPTH`;
6. mixing the P0 parity cutover into the escape-admission commit;
7. a source/check file at or above 800 lines.
