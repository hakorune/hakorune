---
Status: parked — no active-path loss proven; dependency blocker comes first
Date: 2026-08-09
Row: `HAKO-PARSER-NUMERIC-SCAN-PARAMETER-CARRIAGE-D0`
Blocks: `HAKO-PARSER-NUMERIC-SCAN-CARRIER-PARAMETER-I0`
Mode: BoxShape / exact declaration-carriage census
---

# HAKO-PARSER-NUMERIC-SCAN-PARAMETER-CARRIAGE-D0

## Proven premise

The source-declared `scan_int(src, i: i64)` probe reported
`MissingTransientType { init: ValueId(3) }`. The local carrier chain is
already known:

```text
formal i
  -> Variable(i)
  -> fresh local Copy j
  -> existing metadata propagation
  -> variable_map[j]
  -> GenericLoop
```

However, import bisection proved the same freeze with `sh_core` alone. The
probe therefore did not reach a point where loss of `scan_int`'s parameter
declaration can be asserted. This D0 is parked, not rejected. It does not
resume merely because the dependency blocker is removed; a compiler-side
canary must first prove an actual parameter-carriage loss on the unmodified
valid source path.

## Sole census

Trace the exact active direct-call compilation path for the imported static
Hako method:

```text
source `i: i64`
  -> Rust parser ParamDecl
  -> module/import carrier
  -> callable header representation
  -> selected function draft entry
  -> set_current_function_declared_signature or canonical sibling
  -> setup_function_params
  -> TypeContext formal fact
```

For every arrow, record the concrete product/type, owner, and whether the
declared type is present, dropped, reconstructed, or never requested.

## Questions to close

1. Does the Rust parser retain the exact `ParamDecl` for imported static Box
   methods in this route?
2. Does an AST/ProgramJSON/module compatibility projection drop it?
3. Which callable-lowering entry is actually used by the VM fixture?
4. Does that entry receive `param_decls`, or only parameter names?
5. Is declared signature projection invoked before parameter identity commit?
6. If the declaration reaches the publisher, why is no TypeContext fact
   visible to the local Copy and GenericLoop?
7. Is `NYASH_USE_TYPE_REGISTRY` creating a second type store on this route?

## Candidate repairs

The Decision may select exactly one real loss boundary:

```text
parser/import carrier loss
  -> preserve existing ParamDecl through that carrier

callable header handoff loss
  -> add exact declared parameter rows to the existing header product

entry ordering loss
  -> ensure existing signature projection precedes parameter commit

dual type-store loss
  -> converge the selected publication/observation path without guessing
```

Do not open a generic header/session redesign unless the census proves the
bounded route cannot reuse an existing canonical owner.

## Rejected shortcuts

```text
publish Integer from scanner/local/loop names
post-hoc set j to Integer
infer from `j = i`, `j + 1`, or call arguments
special-case ParserNumberScanBox
default GenericLoop numeric carriers to Integer
duplicate TypeContext and TypeRegistry publication
source rewrite, retry, fallback, or JSON rescan
```

## Acceptance

```text
one exact active compilation route identified
first declaration-type loss boundary identified
one canonical owner selected for repair
all other candidate boundaries rejected by code evidence
one bounded follow-up I0 named
existing GenericLoop and local-copy fail-fast unchanged
both implementation stashes remain parked
```
