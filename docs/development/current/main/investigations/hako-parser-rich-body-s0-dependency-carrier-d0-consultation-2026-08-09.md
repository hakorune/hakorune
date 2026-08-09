---
Status: accepted — general exact call-result publication edge selected
Date: 2026-08-09
Row: `HAKO-PARSER-RICH-BODY-S0-DEPENDENCY-CARRIER-D0`
Blocks: numeric scanner carrier I0 and H2-S2-S0
Mode: BoxShape / first imported dependency blocker
---

# HAKO-PARSER-RICH-BODY-S0-DEPENDENCY-CARRIER-D0

## Reproducer bisection

The same clean `MissingTransientType { init: ValueId(3) }` occurs for all of:

```text
import ParserNumberScanBox and call scan_int
import ParserNumberScanBox without calling it
import ParserCommonUtilsBox and call index_of
import ParserStringUtilsBox and call i2s
import sh_core and call StringHelpers.skip_ws
```

Therefore the current executable blocker precedes `ParserNumberScanBox` and
its attempted `i: i64` annotation. No parameter-carriage conclusion may be
drawn from that failed probe.

## First source candidate

`sh_core` starts with `StringHelpers.int_to_str(n)`. Its first Loop carrier is:

```hako
local v = me.to_i64(n)
loop (v > 0) { ... }
```

The callee `to_i64(x)` has no declared result annotation. This is the first
source-order candidate for the missing carrier representation: a call result
is copied into local `v`, while neither the call terminal nor the local copy
may invent Integer meaning.

That absence is not permission to edit otherwise-valid library source merely
to satisfy a narrow compiler route. The repository already has a bounded
semantic proof and publication family for this exact shape:

```text
VerifiedSameModuleCallableResultCatalogV1
  proves StringHelpers.to_i64/1 = ExactI64 from complete source behavior

VerifiedStaticCallResultPublicationOwnerV1
  binds exact caller + source call site + exact target

PreparedStaticCallResultPublicationV1
  publishes Integer only after one successful physical Call receipt
```

The current question is therefore whether the imported-source lowering path
issues and consumes this existing verified row for
`StringHelpers.int_to_str/1 -> StringHelpers.to_i64/1`. It is not whether the
source can be made more explicit.

The existing focused source proof confirms
`StringHelpers.to_i64/1 = ExactI64 {}`. The normal lifecycle test also proves
that an exact current-owner `int_to_str -> to_i64` row can reach the raw
terminal when a publication handoff exists.

## Sole census

```text
sh_core module method order
  -> first lowered Loop-bearing method
  -> exact loop variable
  -> exact initializer expression
  -> exact source-bound call target and semantic result disposition
  -> emitted call-result ValueId
  -> emitted local-copy ValueId
  -> TypeContext visibility at GenericLoop entry
```

## Exact gap

```text
VerifiedSameModuleCallableResultCatalogV1
  -> exact general call-result row exists

project_static_exact_i64_requirement_v1
  -> GeneralCallResultAlreadyAvailable

VerifiedStaticCallResultPublicationOwnerV1::issue
  -> currently skips that disposition

general-row activation
  -> disconnected from production publication
```

The semantic result, exact source target, and exact call-site relation already
exist. Only the general exact row is not admitted into the sole production
publication owner. `VerifiedFunctionCompletionV1` is not the missing owner;
it owns terminal Value/Unit and cleanup relations, not I64 representation.

## Decision

Open `GENERAL-STATIC-CALL-RESULT-PUBLICATION-I0` as one compiler-side
BoxShape repair. It projects exact general `SameModuleStatic` call-result rows
into the existing move-only `VerifiedStaticCallResultPublicationOwnerV1`,
then reuses the existing physical Call receipt and
`PreparedStaticCallResultPublicationV1::commit`.

No second publication owner, type inference path, Completion meaning, or Loop
rule is introduced. General evidence wins when present; bounded requirement
rows and general rows may not double-publish the same caller/site/target.

The terminal must distinguish an exact selected row that was lost or consumed
twice from a genuinely unselected ordinary call. Selected missing, duplicate,
foreign-brand, or target-mismatch cases freeze; they never return `None` and
fall through to ordinary emission.

## Rejected shortcuts

```text
annotate scan_int while an earlier dependency still freezes
annotate to_i64/int_to_str solely to satisfy the current compiler route
infer Integer from int_to_str/to_i64 names
infer from modulo, division, comparison, or numeric literals
post-hoc set local v or ValueId(3) to Integer
default GenericLoop numeric carriers to Integer
compile dependencies in a different order to hide the failure
source rewrite, unroll, retry, fallback, or JSON rescan
```

## Acceptance

```text
exact first failing function named
exact Loop carrier and initializer named
declared-result annotation presence/absence recorded without making it authority
existing semantic ExactI64 disposition for to_i64 confirmed
general-row exclusion from the production publication owner proven
existing call-result publication owner identified
one bounded compiler-side follow-up I0 selected
end-to-end ValueId correspondence reserved for the I0 canary
scanner I0 and lexical S0 remain parked
temporary bisection fixtures removed
```

## Closed evidence

```text
StringHelpers.to_i64/1 body result = ExactI64
exact current-owner source target = available
exact general call-result row = available
sole physical publication terminal = available
general row -> publication-owner projection = missing
```

The two focused authority tests each passed. Implementation and its exact
end-to-end dependency canary belong to the I0 card; this D0 changes no code or
production route.
