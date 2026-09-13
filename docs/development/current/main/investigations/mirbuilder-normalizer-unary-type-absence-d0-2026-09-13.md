---
Status: Design stop — unary I0 owner evidence green; production relation mapping open
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

## MIR-NORMALIZER-UNARY-TYPE-ABSENCE-I0 (complete)

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

Implementation permission was enabled by the current pointer's
`work_mode = fast` selection for `MIR-NORMALIZER-UNARY-TYPE-ABSENCE-I0`.

### I0 evidence (2026-09-13)

- `unary_minus_preserves_known_integer_and_float_types` passes for both
  Integer and Float, including the typed zero and `BinaryOp::Sub` plan.
- `unary_minus_rejects_missing_unknown_and_nonnumeric_before_allocation` passes
  for missing, `Unknown`, String, and Bool facts; each rejection occurs before
  the next ValueId changes.
- `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib unary_minus --
  --nocapture` passes (4 tests, 0 failed; the other two matches are existing
  unary coverage).
- `CARGO_BUILD_JOBS=4 cargo check --profile quick` passes. Repository-wide
  `cargo fmt -- --check` remains red on pre-existing unrelated files; the two
  changed Rust files were formatted directly and `git diff --check` is clean.

The module README now records the missing-type rejection boundary. This is
owner-level evidence only: the resolver-to-loop production relation and the
separate provider/CAPI/Windows temporary-input evidence remain outside this
I0 and must not be reported as closed here.

## MIR-NORMALIZER-UNARY-PRODUCTION-RELATION-D1 (design stop)

Decision: keep the unary type-absence I0 closed at its owner boundary and first
settle one exact resolver-to-GenericLoopV1 production relation before adding an
acceptance fixture.
Source authority + canonical issuer: the normal callable resolver's selected
source site, `BindingRef` ledger, existing `CallableGenericLoopSourceFactsIssuerV1`,
and the existing normalizer type-context issuer for the operand value.
Non-authority: a raw `env.get` result with no local type, hand-built AST/source
fixtures, a function-level `Outside` classification, or a test-only type insert.
Fail-fast boundary: selected source admission and `verify_located_generic_loop_v1`
must prove the exact front route with no overlap before the unary missing-type
rejection is claimed as a production terminal.
Smallest next slice: static-audit one source wrapper and caller through canonical
route selection, exact loop site, local `BindingRef` publication, and operand
`read_variable` consumption; only then taskize the smallest real acceptance.
Non-claims: no new source fixture, type inference, route fallback, production
unary semantic change, or whole normal-callable/backend acceptance.

The finite chain is now explicit. `classify_canonical_callable_route` checks
DirectAccum, CallableSingleLoop, GenericG0, then the canonical preflight; only
the final `Outside` branch enters the located raw-loop handoff. That handoff
calls `CallableGenericLoopSourceFactsIssuerV1::issue_once`, which rejects a
non-front or overlapping route at `verify_located_generic_loop_v1` before a
Recipe is issued. The physical adapter then consumes the selected Recipe through
`CallableLoopSourceExpressionPortV1`, whose variable read uses the exact source
site and ledger `read_variable` rather than a spelling lookup.

Two production facts remain unclosed for the candidate `local x = f()` followed
by unary `-x` in the loop body: the selected source wrapper must be shown to
reach the front `GenericLoopV1` branch without another route winning, and that
same source site must publish the local `BindingRef` whose ledger value reaches
the unary child. Existing source-facts tests fabricate owners/bindings and call
`lower_for_test`; the handoff fixture uses a different real loop body. Neither
is evidence for this exact relation. This D1 therefore stops before code,
fixture, or Cargo work until the two source-backed mappings are named.

Static route review narrows the remaining work. DirectAccum requires a two-
statement body whose first local is a pair of zero integer literals, so a
candidate with a call local, a separate carrier local, a Loop, and a final
Return is outside that probe. CallableSingleLoop can observe the call local,
but its step observer requires one assignment whose value is a binary update;
an operand-shaped unary body or a unary assignment value therefore returns a
typed shape-outside result. GenericG0 requires exactly a root Loop plus a final
Return, so the preceding locals keep it from winning. The canonical preflight
then reports an unsupported first-family shape and `classify_canonical_callable_route`
enters the existing `Outside` handoff. This is a static selection argument for
the bounded candidate, not yet a production acceptance receipt.

The BindingRef side is already closed at the owner-chain level: shadow
resolution records the local declaration and every variable source site;
`CallableSemanticLoweringState::from_exact_source` retains those declaration and
variable maps; `record_completed_local` publishes the completed value; and
`CallableLoopSourceExpressionPortV1::exact_source_variable_value` derives the
unary child site and calls the ledger's exact `read_variable`. The unresolved
part is the type issuer for an unannotated call result. The current normalizer
sets `FunctionCall` and ordinary `MethodCall` results to `MirType::Unknown`, so
the unary terminal must reject missing type rather than infer `Integer`; one
real production source case must still demonstrate that this terminal is
reached through the selected raw Loop handoff.

The tempting unary/call/loop sources under `apps/tests/phase29bq_*` do not
close that obligation. The current generic-loop disposition ledger records
these rows as `nonproduction-future-evidence` and `P0-INVENTORY-ONLY`
(`docs/development/current/main/design/fixtures/generic-loop-legacy-disposition-v1.tsv:370-379`),
while the older phase backlog only records them as parser-handoff candidates.
They therefore remain inventory evidence and cannot be promoted as the
production resolver-to-GenericLoopV1 relation for this card.

A bounded read-only production-source audit (2026-09-13) found no eligible
replacement candidate. The tracked `.hako`/`.nyash` corpus was reduced from
3,397 files to 1,587 after excluding tests, fixtures, archives, VM sources,
proof/probe surfaces, and the phase29bq inventory; comment/string-stripped
`-identifier` hits were then checked at function scope. The remaining three
shapes are non-candidates: `apps/lib/std/operators/neg.hako:2` has no Loop and
is not injected by the selected-normal prelude path, while
`examples/wasm/05_mini_pong.hako:85,98` uses CanvasLoopBox callback/field
expressions rather than a language `Loop` with the normal-callable
`BindingRef` relation. This lexical audit is not an AST census, but it closes
the current inventory question: no existing production source can be named
for the required relation without designing a new source-backed acceptance.

## Reopen / non-claims

Reopen when an existing operand type issuer and a canonical missing-type
terminal are both named, or when a concrete source counterexample demonstrates
that the current fallback changes a selected acceptance result. A test-only
type insertion, runtime inference, or cross-backend parity result is not an
issuer. This card does not claim that every Integer fallback in the repository
is covered; the Add and non-Add siblings remain separately owned.
