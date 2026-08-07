# Callable single-loop Recipe co-seal I0/R0

Status: `Decision: bounded caller-zero implementation task; production selection and physicalization are not authorized`

Design authority:
`docs/development/current/main/investigations/generic-callable-loop-recipe-coseal-d0-task-2026-08-07.md`

## One atomic claim

Consume the closed `VerifiedCallableSingleLoopSourceMapV1` exactly once and
publish one caller-zero, move-only common co-seal for the selected profile:

```text
StringHelpers.int_to_str/1
  prefix: local value = helper.to_i64(n)
  loop:   i < 1 { i = i + 1 }
  tail:   return value
```

The implementation may add one profile-neutral source-relation module under
`src/mir/loop_recipe_contract/` and one thin `cfg(test)` orchestration/test
module. Keep each touched Rust source file below 800 lines. Do not modify the
Builder, MIR, canonical CFG/SSA, physical input, production selector, or
legacy route.

Count/shape invariant:

```text
19 legacy route labels = ingress coverage only
selected callable row  = one instance of the existing recursive LoopRecipeV1
new Recipe/Loop kind   = 0
new verifier/physicalizer branch = 0
```

Do not encode `LoopSimpleWhile`, this callable profile, or any completed source
shape as a portable Recipe variant. The implementation maps the admitted
source roles to the existing `LoopNode + Operation/If/Loop/Exit` algebra.

Preferred physical split (only if the existing facades cannot stay below the
line cap):

```text
loop_recipe_contract/source_roles.rs  <120 lines
loop_recipe_contract/source_co_seal.rs <400 lines
callable recipe/producer              <350 lines each
focused tests                          <600 lines
```

The existing `CallableSourceMapRoleV1` vocabulary must be moved or
re-exported through one common owner if it is needed by the producer. Do not
invent a second role enum or map and reconcile them later.

## Input and output

Input is move-only and fresh:

```text
VerifiedCallableSingleLoopSourceMapV1
  + verified common LoopRecipe/JoinSig/Core inputs
  + typed source relations
```

Output is one `VerifiedLoopRecipeCoSealV1` (or the exact accepted common name)
containing:

```text
existing VerifiedLoopCoreProductV1
VerifiedLoopSemanticContextV1(owner, origin, source_kind, loop, frame, scope/region)
VerifiedLoopOperationSourceRelationV1[]
VerifiedLoopInputRelationV1[]
VerifiedLoopAfterTailEnvelopeV1
```

The new wrapper must not duplicate Core, JoinSig, BindingSSA, PHI, or
completion authority. Existing Recipe verification and JoinSig elaboration
are called, not reimplemented.

## Exact mapping

```text
InitialCarrier   -> carrier entry + explicit preheader InputRelation (i = 0)
ConditionRead    -> ReadBinding
ConditionBound   -> ConstI64(1)
ConditionOperator-> CompareI64(Less)
StepRead         -> ReadBinding
StepDelta        -> ConstI64(1)
StepOperator     -> BinaryI64(Add)
StepWrite        -> one WriteBinding
PrefixBoundary   -> outer callable-prelude envelope; no Recipe Call
TailReturnRead   -> terminal After/Tail envelope; not loop-carrier After
```

Each relation retains exact Recipe item/value keys, typed source site, role,
target kind, and optional source BindingRef. Coverage is consumed once by
`(typed source site, role, target kind)`; no name/path/ordinal/AST rematch.

## Reject before publication

```text
missing/duplicate/foreign/unconsumed source row
owner/origin/source-kind/frame/Scope/Region mismatch
binding or assignment-target mismatch
unsupported literal/operator/type
implicit local-literal-to-input conversion
prefix MethodCall fabricated as a Recipe Call
tail joined to loop-carrier After
missing terminal return/completion ABI
Recipe/JoinSig/relation cross-product mismatch
second Recipe/SSA/PHI/After/completion owner
```

All failures are terminal `NoSafeSlice`/typed reject. No retry, fallback,
reselection, or Builder effect is allowed.

## Acceptance

- positive source-map fixture publishes the co-seal and survives source-view
  drop;
- focused negatives cover every reject row above, including the initial
  `InputRelation` and separate prefix/tail envelope;
- common Recipe verifier, JoinSig, and source-bound Core remain the only
  logical owners;
- caller count remains zero and no `ValueId`, CFG, PHI, Builder, or physical
  route is produced;
- `cargo test --lib callable_single_loop --no-fail-fast`, `cargo check --lib`,
  pointer guard, line guard, and `git diff --check` are green;
- the same implementation commit updates
  `docs/reference/mir/loop-recipe-contract.md`,
  `docs/reference/mir/generic-loop-stage-matrix.md`, the current task/card,
  and any immutable fixture/guard receipt required by the row. The reference
  closeout must record the landed relation schema and tests while preserving
  the invariant that 19 means legacy-ingress coverage, not Recipe kinds.

After this row closes, stop before physicalization. The successor common
physical-demand/fresh-session/failure-discard/Completion-DraftSeal boundary is
already accepted in
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`,
but `LOOP-COMMON-PHYSICAL-DEMAND-I0-R0` does not open until the current pointer
and user authorization advance. Recipe completion is not physical completion,
production activation, or legacy retirement.
