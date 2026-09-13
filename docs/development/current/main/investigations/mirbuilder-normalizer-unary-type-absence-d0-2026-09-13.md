---
Status: Design stop — A-4 authority/disposition unresolved
Date: 2026-09-13
Decision: MIR-NORMALIZER-UNARY-TYPE-ABSENCE-D0
Parent: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
ProductionCaller: PlanNormalizer::lower_value_input (selected normalizer path)
ReplacementCell: unary minus result-type admission
---

# MIR-NORMALIZER-UNARY-TYPE-ABSENCE-D0

## Six-line brief

Decision: resolve the unary-minus missing-type path before changing its `Integer` fallback.
Source authority + canonical issuer: the unary AST operand and the existing lowering-time operand type issuer; no new issuer is available for an absent type yet.
Non-authority: `type_ctx` `None`/`Unknown`, the integer zero constant, raw Builder unary behavior, runtime values, and VM/backend defaults.
Fail-fast boundary: after child lowering and before zero/destination allocation or `Sub` effect publication; missing or contradictory type evidence must not allocate a guessed result.
Smallest next slice: classify known Integer, known Float, Unknown, missing, and nonnumeric operand states, then choose one source-backed issuer or an explicit typed rejection for each.
Non-claims: no unary language expansion, arithmetic-wide type unification, raw Builder parity, runtime coercion, fallback, retry, or semantic receipt invented from `None`.

## Finite scope

Census boundary: `PlanNormalizer::lower_value_input` unary-minus admission ->
child operand lowering -> operand type lookup -> zero/destination allocation ->
`BinaryOp::Sub`. Includes only `src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs:124-153`
and its existing operand type context. Excludes the accepted CorePlan Add result
matrix, the adjacent non-Add arithmetic helper, `ops/unary.rs`, VM defaults,
other normalizers, and backend parity.

## Current behavior and states

The current arm lowers the child, reads `type_ctx.get_type(rhs)`, substitutes
`MirType::Integer` for `None`, chooses an Integer zero for every non-Float
variant, allocates the zero and destination, and emits `Sub`. `Some(Unknown)`
therefore remains an `Unknown` destination while still receiving an Integer
zero; `None` becomes an Integer destination. The two cases must not be merged
by a new default.

| State | Existing authority/condition | D0 disposition |
| --- | --- | --- |
| `ChildError` | operand lowering returns an error | propagate before zero/destination allocation |
| `KnownInteger` | child issuer publishes `MirType::Integer` | preserve Integer zero and Integer result |
| `KnownFloat` | child issuer publishes `MirType::Float` | preserve Float zero and Float result |
| `KnownNonnumeric` | child issuer publishes another concrete type | decide typed reject versus an existing explicit contract; do not infer Integer |
| `UnknownFact` | child publishes `Some(MirType::Unknown)` | keep distinct from missing; choose an owner or typed reject before allocation |
| `MissingFact` | `get_type(rhs)` returns `None` | current implicit Integer is the A-4 edge; require source issuer or typed reject |

No state in this table is a permission to read a runtime value, source name,
or another backend's result type. If the required issuer cannot be named, this
candidate remains `NoSafeSlice` and no new `Verified*`/`Prepared*` type product
is allowed.

## Evidence

- `helpers_value/lower.rs:124-130` obtains the operand through the existing
  `ExprChildRoleV1::UnaryOperand` path and propagates child errors.
- `helpers_value/lower.rs:131-136` reads the existing type context and applies
  `unwrap_or(MirType::Integer)` when the fact is absent.
- `helpers_value/lower.rs:137-146` chooses the zero and allocates the result
  before the `Sub` effect is appended.
- `helpers_value/lower.rs:147-153` emits the existing `BinaryOp::Sub` plan.
- `ops/unary.rs:99-104` contains a separate raw unary result rule; it is
  evidence only and cannot issue the normalizer's missing type.
- The accepted CorePlan Add owner at
  `hmi-s0-v0-r0-coreplan-string-add-representation-task-2026-07-17.md` pins a
  different String/Float/Other matrix, including its own compatibility
  behavior. It does not authorize unary-minus inference.

### Producer inventory (2026-09-13)

The normalizer's own call arms do not produce `MissingFact`: the method arm
registers either the known env return type or `MirType::Unknown`
(`helpers_value/lower.rs:204-215`), and the function/call arms register
`MirType::Unknown` (`:389-393`, `:487-491`). Untyped formal parameters likewise
enter the type context as `Some(Unknown)` through the function-signature
skeleton (`calls/function_lowering.rs:20-28`, `:53-58`) and parameter
publication (`calls/parameter_setup.rs:253-278`).

There are raw call terminals that allocate a result without publishing a type:

- cataloged raw calls use `next_value_id()` and emit the call at
  `calls/build.rs:349-362`;
- an installed AppMain scalar call does the same at
  `calls/build.rs:156-165`;
- raw env and standard method terminals return their destination without a
  local type publication at `calls/method_call_terminal.rs:414-429` and
  `:444-459`.

When such a result is copied into a local, `variable_stmt.rs:246-256` invokes
`metadata::propagate`; that helper copies a type only when the source already
has one (`metadata/propagate.rs:22-30`). The variable arm of the normalizer
then returns the existing `ValueId` without inventing a type. Thus a shape such
as `local x = f()` followed by a source-aware loop operand `-x` is a concrete
`None` candidate if `f()` is admitted by the installed catalog/AppMain route.

This inventory does not yet prove that the candidate reaches the selected
source-relation → loop-normalizer production path. The preflight route rejects
raw script/legacy compatibility origins (`calls/function_call_preflight_route.rs:369-383`)
and requires an installed caller relation for cataloged calls (`:413-424`), so
old raw-call fixtures are not production evidence. D0 must name one installed
source relation and follow it through local binding into the selected loop
operand, or explicitly prove that all selected producers publish a type.

The static audit found no focused test that directly pins unary-minus with a
missing operand type. Existing integer/float unary positives cover only the
known-type states. Runtime impact for `MissingFact`, `UnknownFact`, and
`KnownNonnumeric` is therefore un-reproduced and must not be claimed.

## Ordered D0 tasks

1. Inventory the lowering-time producers for the exact unary operand and prove
   whether they can issue Integer/Float/nonnumeric type facts before this arm.
2. Define the negative matrix for missing, Unknown, and nonnumeric facts,
   including the first fail-fast point and the no-allocation/no-effect rule.
3. Decide whether the existing normalizer compatibility contract intentionally
   retains `MissingFact -> Integer`, or whether a source-backed issuer and
   typed rejection replace it. Do not borrow `ops/unary.rs` or Add semantics.
4. Only after that Decision, select a one-owner I0 with known Integer/Float
   positive coverage, separate negatives, and the exact deletion or retention
   edge for `unwrap_or(MirType::Integer)`.

Implementation permission is false in this design stop. No code, fixture,
fallback, route switch, or guessed semantic receipt may cross the boundary.

## Reopen / non-claims

Reopen when an existing operand type issuer and a canonical missing-type
terminal are both named, or when a concrete source counterexample demonstrates
that the current fallback changes a selected acceptance result. A test-only
type insertion, runtime inference, or cross-backend parity result is not an
issuer. This card does not claim that every Integer fallback in the repository
is covered; the Add and non-Add siblings remain separately owned.
