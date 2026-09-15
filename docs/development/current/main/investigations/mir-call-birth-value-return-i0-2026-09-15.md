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

## Receipt (landed)

**Decision taken: reject stays — the source was malformed.** Worker audit
established that `Unit Birth has no result` is the accepted contract
(`constructor-birth-new-lifecycle-ssot.md`,
`function-exit-and-entry-result.md`), that the legacy route emits the value
return and silently drops it at the call boundary (`dst = None`, VM bridge
returns Void), and that no typed "discarded value" slot exists in any
published artifact. Admitting discard semantics would require a new
`FunctionUnitOriginV1`/disposition plus a `BirthResultAbiV1` variant — a
language decision, not this card.

**Census** (`lang/src/**/*.hako` + `apps/**/*.hako` birth bodies; includes
every `birth(` definition; result: 231 births, 15 value-returning, all
terminal `return 0`): parser_box, mir_builder_box, backend_box,
execution_pipeline_box, debug_box, intarray_core_box, mini_map_box,
mini_collections×2 (lang/src); ssa_static_delegation_{enhanced,via_me,min},
parser_box_minimal, ops_calls, string_cursor (apps).

**Fix**: all 15 sites normalized `return 0` -> `return void` — the
grammar-live explicit-Unit spelling (22 existing corpus uses). Bare
`return` was rejected as the spelling because it is grammar-inactive
(EBNF.md:105-107, function-exit-and-entry-result.md).

**Docs**: `function-exit-and-entry-result.md` now states explicitly that
`return <value>` inside `birth` is malformed source.

**Route evidence**: merged entry (patched equivalently at line 5951)
advances past `OrdinaryNew/BirthCompletionNotUnit` to the install-stage
terminal `[callable-semantic-package/install] MapLifecycleConsumerMissing`
— a different family, recorded as the next card.

Non-claims held: no compiler semantic changed; `BirthCompletionNotUnit`
still rejects genuinely value-completing births; bare `return` grammar
activation stays a separate row.
