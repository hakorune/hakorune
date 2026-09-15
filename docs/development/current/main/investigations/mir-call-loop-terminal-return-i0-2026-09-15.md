---
Status: selected__fast__2026-09-15
Task: MIR-CALL-LOOP-TERMINAL-RETURN-I0
Date: 2026-09-15
Priority: admit a provably non-falling-through loop as a function terminal
Parent: mir-call-return-null-value-classification-i0-2026-09-15.md
NextCard: next named merged-route terminal after Dynamic/Completion
Implementation permission: true for the resolved_control_flow owner only
---

# Loop-terminal return completion I0

## Six-line brief

```text
Decision: the merged route stops at Dynamic/Completion/NonTerminalReturn for functions whose body is a single `loop(true)` whose exits are `return <expr>` (e.g. `_consume_optional_semicolons`); decide whether a loop that provably never falls through may serve as the function terminal carrying its inner return set, or keep the named rejection — never synthesize a root return.
Source authority + canonical issuer: verify_explicit_return_set and the exit inventory in src/mir/resolved_control_flow/function_control.rs own terminal classification; the resolver's exit/region rows stay the sole return-site authority.
Non-authority: a loop-terminal form must not mint PHI/JoinSig, loop-break semantics, or MIR blocks; `loop(true)` non-fallthrough is a source fact, not a control-flow guess.
Fail-fast boundary: loops that can fall through (break, non-constant condition) keep the named NonTerminalReturn/TerminalSiteIsNotReturn rejection; no reachability inference from runtime behavior.
Smallest next slice: census merged functions whose terminal statement is a loop containing all explicit returns, decide the non-fallthrough predicate (constant-true condition + no break), and seal the terminal form.
Non-claims: Dynamic admission widening, loop-carried values, PHI/JoinSig physicalization, production caller switch, and legacy retirement.
```

`Census boundary: merged entry program -> function terminal statements;
includes every function whose root-level last statement is a Loop and every
explicit return inside that loop; excludes loops nested under If/other
statements and functions that also carry a root-level return.`

## Entry contract

The previous card made `return null` a value return, unblocking
`ReturnClassificationInvariant`. The route now stops at
`Dynamic/Completion/NonTerminalReturn` (batch slot 158): the representative
body is `_consume_optional_semicolons(src, j, ctx)` = `loop(true) { ...;
return next }` — the sole explicit return lives at `[Body(0), LoopBody(2)]`
while `verify_explicit_return_set` requires the root terminal statement to be
a `Return`. A `loop(true)` containing only `continue`/`return` provably never
falls through, so the function's terminal is the loop itself.

First census how many merged functions have this shape (loop-as-terminal with
all returns inside), then decide the exact non-fallthrough predicate —
constant-true condition plus absence of `break` — and whether the completion
product gains a loop-terminal disposition or the return set admits the
loop-contained sites with the loop proven as the terminal.

## Acceptance

Focused tests prove the decided terminal form: loop-terminal functions with
uniform inner returns seal (or reject) exactly once, falling-through loops
keep named rejections, and mixed loop-return + root-return bodies stay
coherent. Run one quick-profile lib test process with at most four build
jobs, classify repository warning debt as baseline, and update the owner
README and pointer in the same closeout slice. Route evidence: the merged
entry must advance past `Dynamic/Completion/NonTerminalReturn` to the next
named terminal.
