---
Status: design consultation stop
Date: 2026-07-17
Baseline: ef4c26c50eebbda3192a5fd97878a2fe2996117d
Parent: callable-result-i64-catalog0-task-2026-07-17.md
Scope: canonical source-call target projection and neutral Core method result kind
---

# Source-call target and Core-result authority design stop

## Current evidence

`R0-CALLABLE-RESULT-I64-CATALOG0-S0a` proves local-body exact-i64 result
requirements without production consumers. Two independent missing facts
prevent full S0:

1. the complete declaration catalog proves candidate existence, not the final
   source-call route selected after builtin/direct-module/current-owner policy;
2. the Core method vocabulary owns canonical identity, arity, and effect, but
   not a representation-neutral result kind for `String.length/0`.

A concrete counterexample is a same-module static declaration named `str/1`.
Builder's builtin `str(...)` route precedes bare-static recovery, so unique
declaration recovery cannot be used as final target authority. The actual
wrapper `ParserStringUtilsBox.skip_ws/2` is therefore unavailable even though
`StringHelpers.skip_ws/2` itself is `ExactI64 {1}`.

The actual `StringHelpers.to_i64/1` first loses the proof at:

```hako
local s = "" + x
local n = s.length()
local zero_i64 = n - n
```

Using Builder `MirType`, runtime tags, or a method-name whitelist would create
a second or downstream authority and is forbidden.

## Decision required

Select the smallest durable architecture answering all of these questions:

1. Which product co-seals the final selected source-call route and canonical
   callable key before Builder effects?
2. Does one product cover bare, qualified static, and current-owner calls, or
   must route-disjoint products be introduced in a fixed order?
3. How does the product prove every higher-priority route declined without
   replaying Builder policy or inspecting a physical MIR symbol?
4. Which neutral owner states that canonical `String.length/0` returns exact
   i64, while remaining reusable by Builder and source proof?
5. Which owner proves `"" + x` has a String receiver representation without
   introducing general non-i64 values into the exact-i64 abstract domain?
6. Which owner canonicalizes `length` / `len` / `size` aliases, if aliases are
   admitted at all?
7. Is task order target projection first, Core result-kind first, or one
   co-sealed product? The answer must allow disconnected tests before any
   production consumer.

## Candidate boundaries

### Candidate A — selected-route witness plus neutral Core result-kind catalog

```text
complete declarations + existing route policies
  -> VerifiedSourceStaticCallTargetV1

CoreMethodContract canonical identity
  -> CoreMethodResultRepresentationV1

both borrowed by callable-result S0b
```

This is preferred if route selection can be centralized once rather than
replayed. The target witness must retain a structured key, never a parsed MIR
symbol. The Core result catalog must not depend on Builder `MirType`.

### Candidate B — one broader callable resolution product

One product seals builtin/Core/user/static call identity and result
representation together. This may reduce joins, but risks becoming a second
whole callable/type authority. Select only if the existing callable registry
can own it without duplicating declaration or Core-method truth.

### Candidate C — keep call results unavailable

This preserves S0a but cannot close the selected wrapper/to_i64 blocker and
does not advance HMI. It is a parking decision, not completion.

## Exact next code-facing owner requirement

The consultation must name exactly one first disconnected owner:

```text
R0-SOURCE-CALL-TARGET0-S0
or
R0-CORE-METHOD-RESULT-KIND0-S0
or one explicitly co-sealed replacement
```

The first row must have:

```text
production consumers = 0
Builder/MIR/runtime behavior delta = 0
name heuristic count = 0
physical-symbol parsing = 0
fallback/retry = 0
source/check files >= 800 lines = 0
```

## Non-authorities

```text
callable declaration existence alone
bare-static recovery decision alone
Builder infer_method_return_type
generic_method_route_plan ScalarI64 rows
MirType or type_ctx
final MirFunction metadata
runtime VMValue/class tags
physical MIR symbol spelling
function/method/HMI-name special cases
callee-first lowering, retry, or re-lowering
```

## Stop conditions

Stop implementation if any proposal requires:

1. replaying route precedence independently in the result analyzer;
2. a second callable declaration/body catalog;
3. Builder or runtime representation facts flowing back into source proof;
4. a method spelling whitelist as result-kind authority;
5. non-i64 values entering the exact-i64 domain without a separate bounded
   receiver view;
6. physical-symbol parsing, declaration order, fallback, or retry;
7. connecting a production consumer before disconnected target/result parity;
8. a source/check file reaching 800 lines.

## Implementation may not yet claim

```text
canonical call-target projection
same-catalog call substitution
String.length/0 exact-i64 result authority
StringHelpers.to_i64/1 exact result
ParserStringUtilsBox.skip_ws/2 exact wrapper
call-result ValueId publication
HMI register execution
```
