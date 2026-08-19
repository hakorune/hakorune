# FunctionCall Direct versus Value Call Compatibility Census D0

Status: selected design stop
Scope: legacy bare-call target and first-error behavior census
Parent: `../workstreams/mirbuilder-inplace-replacement-current.md`
Row: `FUNCTION-CALL-DIRECT-VS-VALUE-CALL-COMPAT-CENSUS-D0`

## Current execution brief

Decision: Classify every active source/test/caller that relies on post-argument
bare-name resolution before selecting a canonical migration slice.
Source authority + canonical issuer: The accepted FunctionCall evaluation
contract and exact checked-in source occurrences own membership; this census only
classifies compatibility dependence and issues no call meaning.
Non-authority: Regex count alone, method names, `ValueId`, Builder snapshots,
green tests, C, ASM, and perf cannot classify a canonical target.
Fail-fast boundary: Unknown ownership, unobserved diagnostics, dynamic env-only
tail behavior, or a caller lacking an exact source witness remains unclassified.
Smallest next slice: Count and classify local callable, current-static, builtin/
extern, unique-static recovery, tail resolver, parenthesized callee, and failing-
argument cases; then select one bounded migration or prerequisite.
Non-claims: No parser/resolver/Builder change, Script activation, receipt/Recipe,
compat retirement, production switch, fallback, or retry.

## Required census

```text
canonical candidates:
  resolver-backed direct FreeStatic FunctionCall
  explicit special source forms
  general Call(callee, arguments)

legacy-dependent candidates:
  current-static method chosen from a bare FunctionCall
  local variable chosen as Callee::Value
  builtin/extern selected only by Builder name policy
  unique static recovery after initial resolution failure
  current-module/header tail resolver
  post-argument mutation that retargets the call
  argument failure observed before unresolved-target failure
```

For each active source fixture and production caller, record the exact source
shape, current first decision, current first effect/error, intended canonical
shape, and owning migration row. Archive/comment-only matches are separate.

## Acceptance

- Every raw `FunctionCall` resolver arm has a named caller count and fixture.
- `f((f = value))`-class retargeting and unresolved-plus-failing-argument have
  executable or parser-valid witnesses, or are explicitly impossible by grammar.
- Parenthesized/simple variable callee parsing is classified without assuming
  that AST `FunctionCall` versus `Call` already proves language meaning.
- Tail resolution env gates and header/current-module variants are both counted.
- The next migration changes one semantic family only and names its exact old edge.
- Before implementation, the 790-line preflight owner tests are extracted into a
  child module; no source file may cross 760 lines (800 absolute stop).

## Stop condition

If the census cannot distinguish canonical FreeStatic from legacy callable-value
or recovery behavior at exact source sites, select the missing parser/resolver
source issuer D0. Do not infer membership from a successful raw call.
