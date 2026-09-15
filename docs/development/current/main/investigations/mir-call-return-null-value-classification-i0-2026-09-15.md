---
Status: landed__null_is_value__2026-09-15
Task: MIR-CALL-RETURN-NULL-VALUE-CLASSIFICATION-I0
Date: 2026-09-15
Priority: resolve the return-null vs return-value classification split in function completion
Parent: mir-call-parameter-contract-declared-box-i0-2026-09-15.md
NextCard: next named merged-route terminal after Dynamic/Completion
Implementation permission: true for the resolved_control_flow owner only
---

# Return-null value classification I0

## Six-line brief

```text
Decision: the merged-route lane stops at Dynamic/Completion/ReturnClassificationInvariant because classify_return_value maps `return null` to Void(ExplicitNull) while sibling exits return values; decide whether source `null` is a value-returning expression or a unit origin, and make the completion invariant match that one meaning.
Source authority + canonical issuer: classify_return_value and verify_explicit_return_set in src/mir/resolved_control_flow/function_control.rs own terminal-return classification; DeclaredFunctionResultContractV1 remains the declared-result authority.
Non-authority: `null` classification must not mint a MIR ValueId, a nullable-slot representation, or a result ABI; declared result annotations stay transport data only.
Fail-fast boundary: mixed `return <value>`/`return` (bare) and declared-result mismatches keep named rejections; no fallback classification and no per-site tolerance.
Smallest next slice: census the merged cohort's mixed null/value return sets, fix the single classification meaning, and re-run the merged route to the next named terminal.
Non-claims: Dynamic admission widening, PHI/JoinSig physicalization, production caller switch, VM parity, and legacy retirement.
```

`Census boundary: merged entry program -> return statements per function;
includes every `return`, `return null`, `return void`, and `return <expr>`
inside each resolved declaration; excludes loop bodies that never return and
nested box field initializers.`

## Entry contract

The previous card admitted declared `ArrayBox`/`MapBox` parameters as
`DeclaredHandle`, so the merged route now reaches the dynamic-admission
completion check. Batch slot 108 fails
`FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant`: the
same function returns both a value (`return <expr>`) and `null`
(`return null`, classified `Void`/`ExplicitNull`). The representative cohort
is `variant_payload_type`-shaped bodies (`return null` on miss paths,
`return <expr>` on hit paths).

First census how many merged functions mix `return null`/`return` with
`return <expr>`; then decide the single meaning of source `null` in terminal
position — a value (nullable result) or a unit origin — and align
`classify_return_value`, `verify_explicit_return_set`, and
`verify_declared_return_value` so one meaning holds.

## Acceptance

Focused tests prove the chosen classification: mixed `return null`/`return
<expr>` accepted-or-rejected exactly once per the decided meaning, bare
`return` vs `return null` distinguished if the design keeps both, and
declared-result mismatches still named rejections. Run one quick-profile lib
test process with at most four build jobs, classify repository warning debt
as baseline, and update the owner README and pointer in the same closeout
slice. Route evidence: the merged entry must advance past
`Dynamic/Completion/ReturnClassificationInvariant` to the next named
terminal.

## Receipt — landed 2026-09-15

**Census** (`merged entry program -> return statements per function; includes
every `return`, `return null`, `return void`, `return <expr>` inside each of
the 899 resolved function bodies after stripping comments/string literals;
excludes field initializers and non-function blocks`): 725 value-only, **165
mixed `null`+`value`**, 2 null-only, 1 bare-only, 6 no returns. The `T|Null`
miss/hit idiom is pervasive; `return null` is a value return in this
language. Birth bodies: 3 in cohort, none contain `return null`. Only one
corpus file has `main` + `return null`
(`apps/tests/test_string_concat_phi.hako`, a mixed-return main that was
already rejected under the old classification).

**Decision taken** (worker-audited, `subagent` read-only pass over all
`FunctionUnitOriginV1`/`TerminalReturnValueV1`/ExplicitUnit consumers):
`classify_return_value` maps `LiteralValue::Null` to
`(TerminalReturnValueV1::Value, None, exact_non_unit_literal=false)`. `null`
is a value; `Void` is reserved for bare `return` and `return void`. The
`exact_non_unit_literal=false` flag keeps the `: void` + `return null`
admission (types.md null≡void wire alias) intact. `FunctionUnitOriginV1::
ExplicitNull` stays live: the Main thunk seals an `ExplicitValue` completion
whose terminal representation is `NullSentinel` back to the published
`Unit{ExplicitNull}` result — the wire value is the void representation.

**Implementation files**: `src/mir/resolved_control_flow/function_control.rs`
(Null arm -> Value), `src/mir/resolved_value_profile/analyzer.rs`
(NormalMain0 admits `NullSentinel` as an `ExplicitValue` terminal instead of
`ExplicitNoValue`; non-Main0 keeps `NullRepresentationUnavailable`),
`src/mir/compiler/normal_source_plan/main_thunk_plan.rs` (`NullSentinel` ->
`Unit{ExplicitNull}` arm; `ExplicitVoidValue` still rejects),
`src/mir/resolved_control_flow/function_control_tests.rs` (+2 tests:
null-is-value, mixed null/value set -> `ExplicitValueSet`),
`src/mir/compiler/normal_source_plan/main_function_plan_tests.rs` (Null row
moved out of the unit-origins loop into a new ExplicitValue assertion),
`src/mir/resolved_control_flow/README.md` + 
`docs/reference/language/function-exit-and-entry-result.md` (normative
`return null` text updated to the value-return meaning).

**Focused evidence**: `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
function_control` -> 18 passed / 0 failed; `resolved_value_profile` 63/0,
`if_control` 18/0, `normal_callable_semantic_package` 185/0,
`callable_parameter_contract` 10/0, `source_result_tests` 16/0. Baseline
debt verified by rerun at parent commit `d043965a9d`: `resolved_lowering`
has the same 10 pre-existing failures and `normal_source_plan` the same 2 —
both unchanged by this slice (direct-call index + `ReturnValueTypeMissing`
fixtures, not return classification).

**Route evidence**: `./target/quick/hakorune --backend mir
/tmp/merged_entry.hako` advances past
`Dynamic/Completion/ReturnClassificationInvariant` (slot 108) and stops at
the next named terminal `Dynamic { _batch_slot: 158, _issue: Completion {
_error: NonTerminalReturn { actual: [Body(0), LoopBody(2)], expected:
[Body(0)] } } }` — a function whose only explicit return is inside a Loop
body with no root terminal return. This is the next bounded slice, not part
of this card.

**Non-claims kept**: no Dynamic admission widening beyond classification,
no PHI/JoinSig physicalization, no production caller switch, no legacy
retirement.
