# Loop Prelude Argument Receipt P0

Status: `closed; caller-zero and pre-effect only`
Date: 2026-08-07
Parent: `LOOP-COMMON-PHYSICALIZER-DESIGN0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

Close the one remaining source-to-physical gap before the common Loop
physicalizer canary. The outer callable lowerer must receive exact argument
identity from the resolver; it must not reread the AST, infer by name, or
reconstruct arguments from arity.

## Canonical product

Introduce one move-only, AST-free product in the callable Prelude boundary:

```text
VerifiedCallablePreludeArgumentListV1
  rows: [VerifiedCallablePreludeArgumentV1]

VerifiedCallablePreludeArgumentV1
  ordinal: u32
  site: SourceExprSiteV1
  binding: BindingRefV1
  abi: ExactTrivialReturnAbiV1::I64
```

The issuer uses the existing resolver product at the exact argument site:

```text
VerifiedResolvedFunctionV1.variable_ref(site)
  -> ResolvedLexicalRefV1::Local(binding)
```

The first profile admits only a direct variable argument whose binding belongs
to the caller owner and whose declared representation is exact `i64`.
`Upvar`, literal, nested expression, unknown source site, foreign binding,
arity mismatch, and unsupported ABI return typed `NoSafeSlice`. No new resolver
or semantic owner is introduced.

## Ownership and consumption

```text
resolver / verified function
  -> issues source-site + BindingRef rows
callable Prelude prepared product
  -> owns the argument list exactly once
outer prelude materializer
  -> consumes the list and reads BindingRef values from canonical session identity
private ReadyLoopEntryV1
  -> records that all required entry/result bindings are installed
common Loop physicalizer
  -> never sees the argument list, Tail, ABI, Completion, AST, or input view
```

The product is non-Clone and cannot be rebuilt from `ResolvedFunctionLoweringInputV1`
after preparation. Materialization failure is pre-effect when possible; any
late failure follows the existing fresh-session whole-discard rule.

## Scope and non-goals

```text
included: one resolver-backed static call, variable-only i64 arguments
included: exact ordinal/site/BindingRef owner checks and caller-zero tests
excluded: nested expression argument recipes
excluded: Upvar/capture argument transport
excluded: MethodCall receiver/argument product
excluded: G0, production selector, physicalizer, retry/fallback, legacy deletion
```

Do not add a second generic call argument owner. Future richer arguments must
extend the same product family or remain `NoSafeSlice`.

## Acceptance

- positive static fixture issues one exact argument list for `to_i64(n)`;
- argument order, source site, binding owner, and `i64` ABI are checked;
- missing/foreign/non-variable/arity-mismatched/unsupported rows reject before Builder
  effects;
- product survives source-view drop and cannot be cloned or consumed twice;
- AST reread/name lookup/arity-only reconstruction has no caller;
- focused Rust tests and `current_state_pointer_guard.sh` are green;
- implementation commit updates the design SSOT, this task receipt, the
  relevant reference README, and current mirrors together;
- source files remain below the 800-line cap.

## Implementation receipt

The test-only `callable_single_loop_prelude_arguments.rs` now issues the
resolver-backed argument list for the genuine `FreeStatic` fixture and stores
it in the prepared Prelude capability. The focused prepare suite checks the
single argument's ordinal, exact `i64` ABI, and caller owner. The list and
prepared Prelude are non-Clone; the product is consumed by ownership rather
than rebuilt from a source view. No Builder, MIR, selector, retry, fallback,
or production caller was opened. The compiler README and MIR reference page
were updated with this receipt, and the current mirrors advance to the next
bounded recursive-physicalizer design row.

## Next row

After this row closes, open the bounded common recursive physicalizer row:
`LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`. It must still be caller-zero and
must output only an open Loop After/continuation receipt.
