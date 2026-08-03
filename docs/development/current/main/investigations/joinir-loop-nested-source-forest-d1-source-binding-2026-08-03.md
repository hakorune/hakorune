# JOINIR Nested Source Forest D1 — source-binding projection

Date: 2026-08-03
Status: accepted after worker design audit; implementation authorized.
Task: `JOINIR-LOOP-NESTED-SOURCE-FOREST0-D1-SOURCE-BINDING-PROJECTION`
Parent: `JOINIR-LOOP-NESTED-SOURCE-FOREST0-D0`

## Decision

D0 owns the sealed, non-`Clone` `VerifiedResolvedLoopSourceForestV1`. D1 adds
one neutral adapter in `mir/loop_structural_facts` that consumes that witness
and projects portable source paths. It must not hand the forest directly to a
wire DTO: the adapter first retains local parent indices in a non-`Clone`
`VerifiedLoopSourceForestBindingV1`, then accepts a verified recipe only at the
final `into_source_binding` step.

```text
sealed resolved Loop index
  -> VerifiedResolvedLoopSourceForestV1 (D0, consuming)
  -> VerifiedLoopSourceForestBindingV1 (D1, consuming)
  -> into_source_binding(&VerifiedLoopRecipeV1)
  -> LoopRecipeSourceBindingV1 (wire claim)
```

The D1 adapter reuses the existing `portable_function_owner_v1` and
`portable_path_v1` rules. It does not rediscover source sites, inspect AST or
facts, select a route, create a Recipe/JoinSig, invoke Builder, or own PHI/SSA.

## Authority and product

| Owner | Owns | Must not own |
| --- | --- | --- |
| `VerifiedResolvedLoopSourceForestV1` | exact source members and local ancestry | portable DTO or recipe keys |
| `VerifiedLoopSourceForestBindingV1` | projected owner/path rows and local parent indices | source lookup, route, recipe meaning |
| `LoopRecipeV1` | canonical semantic loop keys and parent rows | source lookup or physical IDs |
| `LoopRecipeSourceBindingV1` | serializable source claim | source existence proof |

Required product shape:

```rust
VerifiedLoopSourceForestBindingV1 {
    owner: LoopRecipeSourceOwnerV1,
    members: Box<[VerifiedLoopSourceForestBindingMemberV1]>,
}

VerifiedLoopSourceForestBindingMemberV1 {
    path: LoopSourcePathV1,
    parent_index: Option<u32>,
}
```

The first forest member supplies the owner. Every later member must project to
the same declared-function owner. Forest position is the only temporary key;
the final wire claim maps position `n` to `LoopNodeKeyV1::new(n)` after recipe
coverage and parent correspondence are checked.

## Typed rejection boundary

The adapter wraps existing root/path rejects as
`Source(member_index, LoopRootSourceBindingRejectV1)` and adds only these
projection/recipe checks:

- `SourceForestEmpty`;
- `SourceForestOwnerMismatch { member_index }`;
- `ParentIndexOutOfRange { member_index, parent_index }`;
- `RootParentMismatch`;
- `RecipeLoopCoverageMismatch { expected, found }`;
- `RecipeParentMismatch { member_index, expected, found }`.

All rejects occur before Recipe/Builder/physical effects. `LoopRecipeVerifierV1`
remains the structural verifier; D1 only supplies its source rows.

## Required tests and guard

Builder-free tests must pin:

- nested root plus `Always` child, including `BodyItem`/`LoopBodyItem` paths and
  `[None, Some(0)]` parent rows;
- a child beneath a lexical scope;
- source/path reject propagation from D0 and the existing root adapter;
- empty forest, owner mismatch, coverage mismatch, out-of-range parent, root
  parent, and recipe-parent mismatch;
- successful wire binding followed by existing artifact verification.

Extend `joinir_loop_compile_candidate_scope.sh` to require the D1 type/API,
forbid production callers, and reject AST/MIR/route/Retry imports in the
adapter. Touched Rust/check/docs files remain below 800 lines.

## Explicit non-claims

D1 does not add a Nested producer, canonical family plan, route cutover,
Retry removal, physicalizer, candidate transaction, PHI/SSA writer, or
selfhost execution. The only authorized consumer is caller-zero test code
until a later M7 producer card accepts the same source-bound capability.
