# FunctionCall Callee Binding and Evaluation Order D0

Status: accepted design
Scope: bare `FunctionCall` callee binding, argument evaluation, and first error  
Parent: `../workstreams/mirbuilder-inplace-replacement-current.md`  
Row: `FUNCTION-CALL-CALLEE-BINDING-AND-EVALUATION-ORDER-D0`

## Current execution brief

Decision: `FunctionCall` denotes direct FreeStatic or explicit special syntax;
`Call` denotes a callee-value invocation. The callee is fixed before arguments,
which are evaluated exactly once left-to-right.
Source authority + canonical issuer: The exact source call occurrence, lexical
binding environment, callable index, special-call vocabulary, and source language
evaluation-order decision must co-seal one target/evaluation contract.
Non-authority: Builder `variable_map`, current module/static box snapshots,
name/arity, catalog/header alone, MIR order, tail lookup, tests, C, and ASM.
Fail-fast boundary: No lowering change until callee binding, argument effects,
target precedence, and first-error behavior have one source-level meaning.
Smallest next slice: `FUNCTION-CALL-DIRECT-VS-VALUE-CALL-COMPAT-CENSUS-D0`
classifies every caller that depends on local/current-static/recovery/tail behavior.
Non-claims: No Script activation, new receipt/Recipe, raw route change, diagnostic
change, tail retirement, production switch, fallback, or retry.

## Required decision matrix

The D0 must define one order for each current family without using physical state:

```text
weak / externcall / Brand / TypeOp / Math / FastMem
builtin global / current-static method / lexical callable value / extern
exact free-static callable-index row / ambiguous or missing target
tail compatibility candidate
```

For ordinary calls it must answer:

1. Is the callee bound before arguments, after arguments, or by a separately
   specified source evaluation rule?
2. Can an argument assignment change the callee selected by the same call?
3. Does an invalid target reject before or after argument effects/errors?
4. Which source product owns lexical callable-value identity without `ValueId`?
5. Does tail compatibility remain language meaning, or is it explicitly legacy?
6. How is the existing callable index reused without copying a second header?

## Counterexamples and acceptance

- Fix a source-level `f((f = value))`-class counterexample and its expected callee.
- Fix an unknown/ambiguous target with an effectful or failing argument and its
  expected first observable error/effect.
- Specify the complete current target precedence and whether it is preserved or
  intentionally migrated; do not call a semantic change BoxShape.
- Name one source-backed issuer for lexical callable values and exact static rows.
- Require future Lower to consume the issued target without `resolve_call_target`,
  bare recovery, header search, tail retry, or Builder snapshot inference.
- Keep Script admission and its selected one-shape BoxCount separate.

## Implementation boundary after acceptance

Before semantic implementation, extract the inline tests from the 790-line
`function_call_preflight_route.rs` into a child test module as a behavior-neutral
BoxShape. The semantic migration then uses separate bounded commits and keeps all
source files below 760 lines, with 800 as an absolute stop.

## Accepted authority

For direct FreeStatic calls, the existing `VerifiedCallableIndexV1` issues the
only target through the exact source-site `ResolvedDirectCallTargetV1`. Builder
state, post-argument lookup, bare recovery, and tail resolution are legacy
compatibility mechanisms and may not issue canonical meaning. Value calls first
evaluate their callee expression and then evaluate arguments left-to-right.

The durable language Decision is
`docs/reference/language/function-call-evaluation.md`. No implementation route is
opened by this D0; compatibility callers must be counted before migration.

## Stop condition

If lexical callable identity or first-error order has no source authority, remain
`NoSafeSlice`. Do not freeze accidental Builder mutation order into a new receipt.
