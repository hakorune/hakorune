---
Status: open__design_stop__2026-09-14
Task: MIR-CALL-RESOLVER-IF-EXPRESSION-EXPRESSIVITY-D0
Date: 2026-09-14
Priority: admit the finite source-backed resolver cases that place If in expression Value/Rhs/Initializer positions
Parent: mir-call-static-compatibility-i0-a3-package-admission-boundary-2026-09-14.md
NextCard: TBD after owner/recipe/terminal decision
Implementation permission: false until the resolver owner and recipe handoff are named
---

# Resolver If-expression expressivity design stop

## Six-line brief

```text
Decision: treat the five observed If-expression deferrals as one finite resolver/body expressivity row, separate from A3 source admission and VM compatibility.
Source authority + canonical issuer: the existing source-backed callable semantic package supplies the body; the resolver shadow/owner path in src/mir/resolved_semantics and its existing result/recipe owners must issue the accepted expression form.
Non-authority: AST/name/arity matching, VM compatibility roots, fallback, source-admission witnesses, fixture-only green, and a new parallel expression matcher.
Fail-fast boundary: unsupported If placement remains ResolverDeferred until one existing resolver owner can lower the condition/value relation and preserve result/branch semantics; no Compatibility downgrade is allowed.
Smallest next slice: census the five callable sites and their exact Value/Rhs/Initializer shapes, select one existing If-expression recipe/owner, and define positive, negative, and terminal guards before implementation.
Non-claims: A3 admission, package publication, caller cutover, VM parity, arbitrary If-expression support, StringBox fixes, and full phase14 completion.
```

## Finite inventory

| Callable | Required observation | Current terminal |
| --- | --- | --- |
| `PatternUtilBox.find_local_bool_before` | `If` in expression Value/Rhs/Initializer position | `SelectedCallableResolverDeferredBatchV1` |
| `JsonNumberCanonicalBox.canonicalize_f64` | same | `SelectedCallableResolverDeferredBatchV1` |
| `JsonFragNormalizerBox._normalize_instructions_array` | same | `SelectedCallableResolverDeferredBatchV1` |
| `JsonFragNormalizerBox._canonicalize_f64_str` | same | `SelectedCallableResolverDeferredBatchV1` |
| `LowerMethodArrayGetSetBox.try_lower` | same | `SelectedCallableResolverDeferredBatchV1` |

The inventory is finite and source-backed. It must be confirmed from the
callable source/body facts before any lowering change. The owner is the
existing resolver/body expressivity path, including the shadow expression
reader and its established recipe/result contracts; A3 must not absorb this
semantic work.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Source/body census | Each callable has one source site, one expression position, one expected value/result relation, and one current deferred receipt. |
| 2 | Existing owner selection | Name the resolver/body owner and existing If-expression recipe/JoinSig/result contract; reject a new parallel matcher. |
| 3 | Shape boundary | Define the exact accepted If form and explicit rejects for condition, branch, type, or result shapes outside the five-row inventory. |
| 4 | Focused guards | Add positive/negative/terminal tests at the selected owner; do not claim package/publication or full-artifact success. |
| 5 | Implementation slice | Implement one owner-local lowering change only after the design stop closes, then re-run the five-row inventory. |
| 6 | A3 handoff | Return the accepted resolver product to the A3 finite target/header/result/publication acceptance matrix. |

## Reopen and stop conditions

Keep this card open if the five rows require different authorities, if the
result relation cannot reuse an existing recipe/JoinSig, or if support would
need AST rewriting, by-name matching, or a fallback. Reopen A3 if the MIR
route falls back to Compatibility after the resolver accepts a body, or if a
new deferred callable appears outside this finite inventory.
