# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PHYSICAL-INPUT-I0

Status: fast implementation row. The parent D0 fixed the source authority,
scalar operand cohort, detached input, and existing session/exit owner. This
row implements only that contract; it does not switch production callers.

Parent: `script-direct-static-call-canonical-physical-input-d0.md`

## Current six-line brief

Decision: implement one AST-free canonical physical input and one direct-static
kernel for the exact integer scalar cohort. Keep the existing generic Script
Recipe, selected-normal claim ledger, unified Call receipt, ExactI64
publication owner, and Script exit session as the sole authorities.

Source authority + canonical issuer: the existing Join row plus the dedicated
scalar operand Recipe are co-sealed by one physical-input producer keyed by the
existing `ScriptDirectStaticRecipeKeyV1`. The direct-static kernel consumes
that input and delegates Call/publication/exit effects to existing owners.

Non-authority: AST, names/ordinals, `RawScriptBodyRecipeV1`, callable-key
re-resolution, `ValueId`/`MirType` inference, detached defaults, duplicate
candidate sessions, and fallback/retry routes.

Fail-fast boundary: validate source identity/owner/key, target,
`ExactI64`, terminal, receiver, contiguous argument sites, and every scalar
tree before the first operand or Call effect. Any drift or failure discards
the candidate; no partial publication or legacy retry is allowed.

Smallest next slice: add the source view/operand Recipe, physical-input
composer, direct-static kernel, and the one narrow finalization seam in the
existing `OpenScriptPhysicalEntrySessionV1`; add focused positive/negative
tests and a structural guard.

Non-claims: source admission, generic Script Recipe widening, canonical
production cutover, raw/compatibility/Deferred retirement, `MirInstruction::Call`
cleanup, ABI/backend change, and performance/C-parity evidence.

## Exact implementation owners

```text
src/mir/builder/normal_script_direct_static_join_handoff/scalar_operand_recipe.rs
  - Script resolver source view and recursive integer tree producer

src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs
  - Join + operand Recipe identity/cardinality co-seal

src/mir/builder/script_physical_exit/direct_static_entry_kernel.rs
  - operand materialization, existing generic Call receipt, and existing
    Script ExactI64 publication; never opens or finishes a session

src/mir/builder/script_physical_exit/entry_session.rs
  - shared `complete_lowered_terminal_v1` seam only; the existing open/finish
    owner remains sole session lifecycle owner
```

No listed source or guard file may cross the 760-line split trigger or the
800-line hard stop. Split before growth; do not hide authority in a facade.

## Required behavior

Accepted input is limited to resolver-issued `Integer(i64)` leaves, unary
`Minus | BitNot`, and binary `Add | Subtract | Multiply | BitAnd | BitOr |
BitXor`. Operands are materialized left-to-right. The kernel emits exactly one
existing unified Call receipt and one Script ExactI64 publication, then maps
the verified Join terminal to `LoweredScriptTerminalV1::Value { value }` and
calls the shared session finalization seam.

`OpenScriptPhysicalEntrySessionV1` remains the only candidate open/finish
owner. The helper must not create a second `MirBuilder`, Return writer,
signature writer, publication owner, or Call emitter.

## Acceptance

- Positive literal and recursive integer unary/binary inputs produce one
  complete candidate with ordered operand effects, one Call receipt, one
  ExactI64 publication, and one shared Exit commit.
- Join/Recipe source identity, owner, key, terminal, target, representation,
  receiver, argument ordinals, and every tree site match exactly.
- Missing/foreign/duplicate/reordered sites, unsupported operators, typed or
  unknown payloads, variables, nested calls, fields, indexes, controls,
  `ValueId`/type drift, duplicate claim, Call/publication failure, and Exit
  preparation failure produce no fallback and no partial candidate result.
- The existing `lower_and_complete` path remains behaviorally unchanged and
  uses the same finalization helper.
- Focused tests, structural guard, `git diff --check`, pointer guard, and a
  single quick cargo check are green.

## Implementation evidence

The detached physical-input suite is green: scalar Recipe tests cover a
recursive integer tree and typed-integer rejection; physical-input tests cover
successful key/site preservation plus source-identity and argument-site drift;
and the detached kernel test observes one generic `Call`, an
`ExactI64`/`MirType::Integer` result, and one shared Script exit commit. The
reusable guard passes, `git diff --check` passes, and the current-state pointer
guard passes. The quick library test profile reports only the repository's
pre-existing warning census; no new compile or test failure remains.

## Non-claims and next order

This row proves a bounded detached contract only. It does not enable a caller,
alter source admission, or retire the existing raw/static edge. A separate
canonical consumer I0 may use this input after its own D0/selection; only then
may a production cutover and old-edge retirement be considered.
