---
Status: selected__fast__2026-09-15
Task: MIR-CALL-LOOP-TERMINAL-RETURN-I0
Date: 2026-09-15
Priority: admit completing terminal statements and void-early-exit completion
Parent: mir-call-return-null-value-classification-i0-2026-09-15.md
NextCard: next named merged-route terminal after Dynamic/Completion
Implementation permission: true for the resolved_control_flow owner only
---

# Completing-terminal return completion I0

## Six-line brief

```text
Decision: the merged route stops at Dynamic/Completion/NonTerminalReturn for functions whose exits are sealed but whose root-terminal statement is not a Return; census of 899 bodies shows exactly three violating shapes — if/else-terminal where both branches complete (3 fns), `loop(true)`-terminal with no break (1 fn), and all-Void exits with a fallthrough end (1 fn); admit all three via a recursive terminal-completes proof plus a combined Void-exits+implicit-end sealed form — never synthesize a root return.
Source authority + canonical issuer: verify_function_completion_v1 in src/mir/resolved_control_flow/function_control.rs owns terminal classification; resolver exit rows (origin+transfer+target_loop) and loop/if region bundles stay the sole control-flow authority.
Non-authority: the sealed form must not mint PHI/JoinSig, loop-break semantics, MIR blocks, or physical exit claims; physicalization of the new coverage kinds is a later family slice.
Fail-fast boundary: if-without-else, one-branch fallthrough, non-constant loop conditions, and Break rows targeting the terminal loop keep named rejections; mixed Void/Value sets stay ReturnClassificationInvariant; Value sets with a non-completing terminal stay rejected.
Smallest next slice: hoist per-exit validation, classify the root terminal (Return | completing If | non-fallthrough Loop | Void-set+implicit-end), add ExactIfTerminalReturnSet / ExactLoopTerminalReturnSet / ExactExplicitUnitSetWithImplicitEnd coverage + the combined disposition/variant.
Non-claims: physical exit-set vocabulary (Single|ExactTwo today), resolved_region_flow singleton authorization, production caller switch, PHI/JoinSig physicalization, and legacy retirement.
```

`Census boundary: merged entry program -> function root-terminal statements;
includes every resolved body with >=1 explicit return whose root-terminal
statement is not a Return; excludes string-continuation false positives.
Result: if-terminal 3 (normalize_cmp_limit, normalize_limit_for_lt,
try_lower), loop-terminal 2 (_consume_optional_semicolons loop(true);
_append_defs loop(i<n) + early bare return -> Void+implicit-end).`

## Entry contract

The previous card made `return null` a value return, unblocking
`ReturnClassificationInvariant`. The route now stops at
`Dynamic/Completion/NonTerminalReturn` (batch slot 158,
`_consume_optional_semicolons`: `loop(true) { ...; return next }`). The
current rule demands the root-terminal statement be a Return; the correct
rule is that every path provably produces the declared result.

Worker audit (2026-09-15) confirmed: no existing sealed artifact certifies
"every path through a statement completes"; the proof must be a new
recursive check inside `function_control.rs` built on sealed
inventory — `child_body_from_stmt` (`IfThen`/`IfElse`/`LoopBody`),
`child_expr_from_stmt` (`LoopCondition` → literal `Bool(true)`),
`product.loop_region_bundle(site).loop_pair().region()` plus a
`Break{target_loop}` scan over `product.resolved_exits()`. Downstream:
`explicit_sites()`-set consumers are nested-site-safe; singleton
`explicit_site()` consumers (trivial profile, region flow, g0 completion)
fail closed — keep `ExplicitReturn` only for the true root-terminal return.
`_append_defs` needs a new combined form carrying Void sites + implicit end
(`implicit_body_end()` returns `Some`, `is_implicit_void()` stays false).

## Acceptance

Focused tests prove: if-terminal admits only when both branches complete;
loop-terminal admits only for literal-true condition with no Break row
targeting the terminal loop's region (nested-loop breaks ignored);
all-Void sets with a fallthrough terminal seal
`ExplicitUnitSetWithImplicitEnd`; Value sets with a non-completing
terminal, if-without-else, non-constant conditions, and targeted breaks
keep named rejections. Run one quick-profile lib test process with at most
four build jobs, classify repository warning debt as baseline, and update
the owner README and pointer in the same closeout slice. Route evidence:
the merged entry must advance past `Dynamic/Completion/NonTerminalReturn`
to the next named terminal.

## Receipt (landed)

Bounded slice landed on `codex/birth-definition-publication`:

- `function_control.rs`: `verify_function_completion_v1` now dispatches to a
  recursive completing-terminal proof; new coverage rows
  `ExactIfTerminalReturnSet` / `ExactLoopTerminalReturnSet` /
  `ExactExplicitUnitSetWithImplicitEnd` plus the
  `ExplicitUnitSetWithImplicitEnd` variant and
  `SealedFunctionExitDispositionV1::ExplicitUnitSetWithImplicitEnd`.
- `function_control_terminal.rs` (new sibling module): per-exit validation
  reused from the parent plus the completing-terminal proof —
  `child_body_from_stmt` IfThen/IfElse recursion, `loop(true)` literal
  condition via `child_expr_from_stmt`, non-fallthrough via
  `loop_region_bundle(site).loop_pair().region()` vs a `Break{target_loop}`
  scan over `product.resolved_exits()`, and the all-Void + implicit-end
  combined form.
- `function_control_new_homes.rs`: explicit arm for the combined variant.

Focused gates: `function_control_tests` 28/28 green including nine new
cases — if-terminal both-branch admit, nested if-chain, no-else and
one-branch-fallthrough rejects, `loop(true)` admit, targeted-break reject,
non-constant-condition reject, nested-loop-break tolerance, Void set +
implicit end admit (single and multi), Value + implicit-end reject.

Adjacent sweep (`resolved_` filter, 855 tests): 15 reds classified — 10
resolved_lowering + vocabulary/upvar/brand reproduce at the parent commit
(baseline debt); `nested_predicate_profile` passes in isolation (shared
physical-effect probe ordering flake, not a current change).

Route evidence: merged entry advances past
`Dynamic/Completion/NonTerminalReturn` (former slot 158) to the next named
terminal `OrdinaryNew/BirthCompletionNotUnit` (slot 317, `ParserBox`
initializer).

Non-claims held: physicalization of the new coverage kinds
(`PreparedFunctionExitSetV1` still Single|ExactTwo), PHI/JoinSig issuance,
production caller switch for the newly admitted shapes, and legacy
retirement. Singleton `explicit_site()` consumers fail closed on the new
multi-site forms by construction.
