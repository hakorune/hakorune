Task: MIR-CALL-BIRTH-VALUE-RETURN-I0
Parent: mir-call-loop-terminal-return-i0-2026-09-15.md
NextCard: next named merged-route terminal after OrdinaryNew co-seal
Implementation permission: pending six-line brief acceptance
---

# Birth value-return completion I0

## Six-line brief

```text
Decision: the merged route now stops at
OrdinaryNew/BirthCompletionNotUnit (slot 317) because ParserBox.birth()
seals an explicit `return 0` value completion while the OrdinaryNew
co-seal requires `!completion.returns_value()`; census of merged entry
shows 3 birth functions, exactly one with a value return — decide whether
birth `return <value>` is a unit-completion with a discarded value
(language contract) or stays rejected.
Source authority + canonical issuer: verify_function_completion_v1 owns
the sealed completion; ordinary_new_candidate.rs owns the
BirthCompletionNotUnit check; the language reference owns the birth
return contract.
Non-authority: neither issuer may silently drop the value edge, mint a
Unit origin for an explicit value, or relax the check by name; if
discarded-value is accepted it must be an explicit sealed disposition.
Fail-fast boundary: non-Birth callables keep uniform value/unit
classification; foreign-owner or wrong-target returns stay rejected.
Smallest next slice: confirm the language-level birth-return contract in
docs/reference, census value-returning births across the whole corpus,
then admit or reject with a typed row.
Non-claims: birth body lowering, constructor recipe issuance, production
caller switch, legacy retirement.
```

`Census boundary: merged entry program -> birth bodies; includes every
resolved birth; result: 3 births, 1 with a value return
(ParserBox.birth -> return 0 at merged line 5951), 2 void/implicit.`

## Entry contract

The previous card admitted the three completing-terminal shapes
(if/else, `loop(true)` without escaping break, all-Void exits plus
implicit end) and the merged route advanced past
`Dynamic/Completion/NonTerminalReturn`. It now stops at
`OrdinaryNew { _error: BirthCompletionNotUnit { slot 317,
Initializer(0), class "ParserBox" } }` because the published birth row's
sealed completion claims `returns_value()`.

## Acceptance

Focused tests prove the chosen contract on both sides; the merged entry
advances past `OrdinaryNew/BirthCompletionNotUnit` to the next named
terminal; owner README and reference record the birth-return decision.
