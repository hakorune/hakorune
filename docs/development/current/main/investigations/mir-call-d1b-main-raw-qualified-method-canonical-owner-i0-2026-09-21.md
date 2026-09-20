---
Status: selected__2026-09-21__MainQualifiedMethodCanonicalOwner
Task: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-CANONICAL-OWNER-I0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-qualified-method-canonical-owner-d0-2026-09-21.md
Implementation permission: true for the bounded owner tuple below
NextCard: acceptance follow-up after focused evidence
---

# Main qualified MethodCall canonical owner I0

## Six-line brief

```text
Decision: extend the existing Binding-SSA Facts/Recipe and physical owner with
  one qualified StaticBoxMethod expression row for Cataloged App Main.
Source authority + canonical issuer: resolver MethodCall ledger plus the
  retained, branded Main qualified-receiver relation; no AST rescan or lookup.
Non-authority: raw inner.lower_body for admitted rows, Script/VM/
  compatibility, bare FunctionCall loans, and result publication ABI.
Fail-fast boundary: exact site/receiver/declaration/ordered args, selected
  callee ExactI64 header, InlineI64 arguments, one-shot consumption, and empty
  residual relation before canonical session finish.
Smallest slice: issue the row, lower it through CanonicalTrivialSsaLowererV1,
  emit the existing target-only terminal, then switch only the Main caller.
Non-claims: strings/objects, nested or instance methods, me, VM parity,
  broad production cutover, and LegacyCallV0 retirement.
```

## Owned work

1. Add the qualified-method row to the existing trivial canonical product and
   its consumption surface.  The row is keyed by `SourceExprSiteV1` and keeps
   the declaration key, ordered argument sites, and `InlineI64` result
   representation proven by the selected callee physical header.
2. Add the package-to-lowerer recipe port.  It may consume the installed Main
   relation exactly once, but it may not re-resolve a receiver, target, or
   argument site.  Relation errors are typed before Builder effects.
3. Extend the existing analyzer/lowerer pair.  A MethodCall arm claims the
   exact row, recursively lowers its arguments through canonical `lower_expr`,
   requires `InlineI64`, emits `emit_static_global_target_value_terminal_v1`,
   and reports the row's `InlineI64` result.  The existing
   `CanonicalSsaFunctionSessionV2` finish chain remains the only completion.
4. In `lower_app_main_root_body_v1`, select this owner before
   `inner.lower_body` only when the complete bounded recipe is admitted.  A
   rejected or outside row must not be silently reinterpreted as a bare call;
   existing non-selected routes remain outside this slice.

## Required guards and acceptance

Focused evidence must cover:

- direct canonical receiver and branded import alias;
- wrong/foreign receiver, duplicate site, missing row, argument-site reorder,
  non-I64 selected header, and unsupported argument shape;
- one admitted call's physical target, ordered arguments, result type, session
  finish, cleanup, and zero residual relation rows;
- an installed Main caller guard proving the raw body path is not entered for
  an admitted qualified row.

The I0 is complete only when the source resolver row, package Recipe row,
canonical Binding-SSA consumer, existing physical terminal, and relation
cleanup are all observed in one bounded source-to-MIR acceptance. Local helper
tests without the installed Main caller do not close this card.

## Explicit non-claims

This card does not admit String/OwnedText results, Map/Handle arguments,
conditional-value joins, nested owner forests, `me.method`, instance methods,
ScriptRoot, compatibility or VM lanes, publication ABI changes, or old-edge
deletion outside the selected Main qualified family.
