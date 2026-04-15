# LoopBreak - JoinIR

## Scope / Criteria
- `loop(...) { ... break ... }` (break present, no continue/return)
- break condition is normalized to "break when <cond> is true"
- loop variable comes from header condition or loop(true) counter extraction
  - loop(true): `i = i + 1` + `substring(i, i + 1)` or `i = j + K` with `j = indexOf(..., i)` + `substring(i, ...)`

## LoopBodyLocal promotion
- SSOT entry: `loop_break::api::try_promote`
- Facts namespace entry: `loop_break::facts`
- Foundation type source: `loop_break::facts::types::LoopBreakFacts`
- Foundation helpers: `loop_break::facts::helpers_{common,break_if}`
- Body-local subset: `loop_break::facts::body_local_subset`
- Condition helpers: `loop_break::facts::helpers_condition`
- Local helpers: `loop_break::facts::helpers_local`
- Realworld helpers: `loop_break::facts::helpers_realworld`
- Body-local facts: `loop_break::facts::body_local_facts`
- Core dispatcher: `loop_break::facts::core`
- Parse-integer subset: `loop_break::facts::parse_integer`
- Read-digits subset: `loop_break::facts::read_digits`
- Realworld subset: `loop_break::facts::realworld`
- Step-first subset: `loop_break::facts::step_before_break`
- Trim cluster: `loop_break::facts::trim_whitespace{,_helpers}`
- Supported: A-3 Trim / A-4 DigitPos (promote LoopBodyLocal to carrier)
- ConditionOnly carriers are recalculated per iteration (no host binding)

## Trim (seg) minimal shape
- Example shape (A-3): `local seg = s.substring(i, i + 1)`
- Break guard: `if seg == " " || seg == "\\t" { break }`
- seg is read-only (no reassignment in the loop body)

## Derived slot minimal shape (seg)
- Example shape (Derived): `local seg = ""` then `if cond { seg = expr1 } else { seg = expr2 }`
- Break guard: `if seg == "" { break }` (seg used in break condition)
- seg is recomputed per-iteration (Select), no promotion
- Contract SSOT: `cleanup/policies/body_local_derived_slot.rs`
- compat path: `plan/loop_break/contracts/derived_slot.rs`

## Carrier binding rules (LoopBreak)
- `CarrierInit::FromHost` -> host binding required
- `CarrierInit::BoolConst(_)` / `CarrierInit::LoopLocalZero` -> host binding is skipped
- ConditionOnly carriers must not use `FromHost`

## Boundary hygiene (Phase 29af)
- Header PHI 対象: `carrier_info` の carriers（LoopState + ConditionOnly + LoopLocalZero）
- Exit reconnection 対象: LoopState のみ（ConditionOnly は exit_bindings に入れない）
- Host binding 対象: `CarrierInit::FromHost` のみ（BoolConst / LoopLocalZero は host slot 不要）
- Fail-Fast: exit_bindings の `carrier_name` 重複は禁止（debug_assert）
- Fail-Fast: `CarrierInit::FromHost` が `host_id=0` の場合は Err
- SSOT: `docs/development/current/main/phases/phase-29af/README.md`

## Out of scope
- multiple breaks / continue / return in the loop body
- reassigned LoopBodyLocal outside the derived-slot shape
- break conditions with unsupported AST shapes
- non-substring init for Trim promotion (e.g., `seg = other_call()`)

## Fail-Fast policy
- `PromoteDecision::Freeze` -> Err (missing implementation or contract violation)
- JoinIR lowering/merge contract violations -> Err

## `Ok(None)` meaning
- not LoopBreak (extractor returns None)
- promotion NotApplicable (continue LoopBreak without promotion)
