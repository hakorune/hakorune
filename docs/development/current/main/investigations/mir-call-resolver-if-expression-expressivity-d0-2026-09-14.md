---
Status: closed__design__2026-09-14
Task: MIR-CALL-RESOLVER-IF-EXPRESSION-EXPRESSIVITY-D0
Date: 2026-09-14
Priority: admit the finite source-backed resolver cases that place If in expression Value/Rhs/Initializer positions
Parent: mir-call-static-compatibility-i0-a3-package-admission-boundary-2026-09-14.md
NextCard: mir-call-resolver-if-expression-contract-d0-2026-09-14.md
Implementation permission: false; this audit closes as `NoSafeSlice` until an expression-If contract is designed
---

# Resolver If-expression expressivity design stop

## Six-line brief

```text
Decision: treat the five observed If-expression deferrals as one finite resolver/body expressivity row, but keep the row at `NoSafeSlice` because statement-If recipes do not carry expression result semantics.
Source authority + canonical issuer: `ShadowResolverV0` owns the source/site deferred fact; a future expression-If contract must co-seal condition, both branch values, result class, and the existing source-backed callable body before any Recipe issuer is selected.
Non-authority: AST/name/arity matching, VM compatibility roots, fallback, source-admission witnesses, fixture-only green, and a new parallel expression matcher.
Fail-fast boundary: unsupported If placement remains `SelectedCallableResolverDeferredBatchV1`; no Compatibility downgrade, Hako ternary rewrite, or expected-value relaxation is allowed.
Smallest next slice: design the finite expression-If contract and its String/f64/i64 result ABI, then split only the Pattern i64 subset if it can reuse one co-sealed Recipe/JoinSig.
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

## Worker decision receipt

The read-only audit confirmed that `? :` is parsed as `ASTNode::If` and the
live `ShadowResolverV0::resolve_expr` deliberately defers an `If` in an
expression position. All five rows are source-backed and retain identity
through `SelectedCallableResolverDeferredBatchV1`:

| Row family | Expression result observed |
| --- | --- |
| `PatternUtilBox.find_local_bool_before` | i64 branch result in `Return.Value` |
| `JsonNumberCanonicalBox.canonicalize_f64` | String/f64 normalization value |
| `JsonFragNormalizerBox._normalize_instructions_array` | String initializer/result |
| `JsonFragNormalizerBox._canonicalize_f64_str` | String/f64 normalization value |
| `LowerMethodArrayGetSetBox.try_lower` | String/array access initializer or RHS |

The existing `resolved_region_flow`/`if_control` owner handles statement-If
branch assignment, but its `IfRecipe`/`IfJoinSig` does not issue expression
results, especially String. No lossless existing Recipe/result issuer was
found, so positive resolver acceptance is forbidden in this row. The next
card designs that missing contract; until then all five rows remain
`NoSafeSlice` at the named deferred terminal.
