---
Status: Design accepted — typed rejection selected; I0 implementation next
Date: 2026-09-13
Decision: MIR-NORMALIZER-UNARY-TYPE-ABSENCE-D0
Parent: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
ProductionCaller: PlanNormalizer::lower_value_input (selected normalizer path)
ReplacementCell: unary minus result-type admission
---

# MIR-NORMALIZER-UNARY-TYPE-ABSENCE-D0

## Six-line brief

Decision: reject missing, Unknown, and nonnumeric unary-minus operands before zero/destination allocation; retain the existing Integer and Float lowering for known typed operands.
Source authority + canonical issuer: ordinary unary syntax plus the existing lowering-time operand type issuer for known Integer/Float; the existing PlanNormalizer owns the typed rejection for states with no usable numeric fact.
Non-authority: `type_ctx` absence/Unknown as an inferred numeric class, the integer zero constant, raw Builder unary behavior, runtime values, and VM/backend defaults.
Fail-fast boundary: after child lowering and before zero/destination allocation or `Sub` effect publication; missing, Unknown, or nonnumeric evidence must not allocate a guessed result.
Smallest next slice: replace the implicit `None -> Integer` edge with the existing normalizer rejection owner, then add known Integer/Float positives and no-allocation negatives for Unknown, missing, and nonnumeric states.
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
| `KnownNonnumeric` | child issuer publishes another concrete type | typed rejection at the normalizer boundary before allocation |
| `UnknownFact` | child publishes `Some(MirType::Unknown)` | typed rejection at the normalizer boundary before allocation |
| `MissingFact` | `get_type(rhs)` returns `None` | typed rejection at the normalizer boundary before allocation; delete the implicit Integer edge |

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

The inventory does not treat old raw-call fixtures as production evidence. The
preflight route rejects raw script/legacy compatibility origins
(`calls/function_call_preflight_route.rs:369-383`) and requires an installed
caller relation for cataloged calls (`:413-424`). The selected typed rejection
does not infer a type from that relation; the production relation remains a
separate acceptance obligation.

### Selected fallback candidate (static route trace, 2026-09-13)

The bounded source shape below is the first concrete candidate for a missing
operand type; it is a route trace, not an acceptance fixture:

```text
local i = 0
local x = env.get("KEY")
loop (i < 1) {
  print(-x)
  i = i + 1
}
return i
```

For a cataloged static method, the normal callable adapter first tries
DirectAccum, `CallableSingleLoop`, Generic G0, and the canonical trivial
preflight (`normal_callable_semantic_loan_port/canonical_route.rs:41-74`).
This body has two loop-body statements, so it is outside the
`CallableSingleLoop` body-arity shape (`callable_single_loop_syntax_facts.rs:432-440`),
and its root has more than the two statements admitted by Generic G0
(`compiler/generic_g0_capability.rs:128-152`). The selected `Outside` branch
therefore keeps the exact source transport and enters
`lower_normal_cataloged_static_box_method_with_source_v1`
(`normal_callable_semantic_loan_port.rs:654-672`), which calls the existing
port-aware raw body driver (`normal_cataloged_box_method_lowering.rs:52-80`;
`port_aware_function_draft_impl.rs:62-79`).

Within that body, the root `env.get` initializer uses the raw env terminal,
which allocates the result without a type publication
(`calls/method_call_terminal.rs:414-429`); local copy/metadata propagation
preserves the absent type (`stmts/variable_stmt.rs:250-256`;
`metadata/propagate.rs:22-30`). The loop entry still has a source-backed
callable handoff because `x` is a body read while `i` has both a condition read
and a body rebind; the handoff projector only parks a binding that is rebound
without a condition read (`normal_callable_loop_handoff.rs:243-358`). The
source-backed GenericLoopV1 adapter then lowers the loop body through the
generic body normalizer (`normal_callable_loop_physical_adapter.rs:31-56`;
`control_flow/plan/features/generic_loop_body/v1.rs:104-138`), where `print(-x)`
reaches `PlanNormalizer::lower_value_input` and observes `type_ctx.get_type(x)
== None`.

The source admission question is no longer needed to choose between a numeric
issuer and a fallback: the dynamic-operator SSOT defines only DynamicAdd and
DynamicLess (`docs/reference/language/dynamic-operators.md:7-30`) and provides
no unary-minus semantic issuer. The selected loop route is nevertheless
traced above so the rejection is attached to the real normalizer boundary,
not to an owner-only fixture. The resolver relation and BindingRef chain still
remain required for the later production acceptance evidence, but they are not
used to invent a type for an absent fact.

The static audit found no focused test that directly pins unary-minus with a
missing operand type. Existing integer/float unary positives cover only the
known-type states. Runtime impact for `MissingFact`, `UnknownFact`, and
`KnownNonnumeric` is therefore un-reproduced and must not be claimed.

## Ordered D0 tasks (completed)

1. Inventory the lowering-time producers for the exact unary operand, separating
   normalizer-owned `Some(Unknown)` from raw terminals with no publication.
2. Define the negative matrix for missing, Unknown, and nonnumeric facts,
   including the first fail-fast point and the no-allocation/no-effect rule.
3. Decide that the existing normalizer has no source-backed unary issuer for
   those states, so typed rejection replaces `MissingFact -> Integer`. Do not
   borrow `ops/unary.rs` or Add semantics.
4. Implement the selected one-owner I0 with known Integer/Float positive
   coverage, separate Unknown/missing/nonnumeric negatives, and deletion of
   the exact `unwrap_or(MirType::Integer)` edge.

D0 exits with the explicit typed rejection owner, the selected loop path
traced end to end, and the state matrix fixing the no-allocation/no-effect
boundary. The production resolver relation remains an acceptance obligation;
it does not reopen the design decision or authorize a guessed type.

## MIR-NORMALIZER-UNARY-TYPE-ABSENCE-I0 (accepted)

Boundary: `PlanNormalizer::lower_value_input` unary Minus after child lowering
and before zero/destination allocation. The existing type context is the only
numeric fact source for this route; no new semantic receipt or source walk is
introduced.

Change the unary-minus match to preserve `MirType::Integer` and
`MirType::Float`, and return one stable typed normalizer error for
`None`, `Some(MirType::Unknown)`, and every known nonnumeric type. The child
value and any effects it legitimately produced remain unchanged; the reject
must occur before `alloc_typed`, zero `Const`, destination allocation, and
`BinaryOp::Sub` publication. Delete only the `unwrap_or(MirType::Integer)`
edge. Raw `ops/unary.rs`, DynamicAdd/Less, VM behavior, and other arithmetic
remain outside this I0.

Acceptance requires focused owner tests for known Integer and Float results,
separate Unknown/missing/String/Bool (or equivalent nonnumeric) rejection with
no post-child allocations/effects, source-size and diff checks, and the
existing pointer/route guards. A later production acceptance pass must also
exercise the selected resolver-to-loop source relation; owner-only tests do not
claim that evidence.

Implementation permission is now enabled only after the current pointer is
changed to `work_mode = fast` with `MIR-NORMALIZER-UNARY-TYPE-ABSENCE-I0`.

## Reopen / non-claims

Reopen when an existing operand type issuer and a canonical missing-type
terminal are both named, or when a concrete source counterexample demonstrates
that the current fallback changes a selected acceptance result. A test-only
type insertion, runtime inference, or cross-backend parity result is not an
issuer. This card does not claim that every Integer fallback in the repository
is covered; the Add and non-Add siblings remain separately owned.
