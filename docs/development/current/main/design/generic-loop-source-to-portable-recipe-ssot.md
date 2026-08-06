# Generic loop source → portable Recipe SSOT

Status: `accepted design stop` (`GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0`)

This document closes the source-to-Recipe contract before any Generic selector,
demand, producer, Builder, MIR, or production caller is added.  It is a design
contract, not an implementation claim.  Tests added after this stop are
witnesses for the rows below; they must not be used to discover the language
meaning.

## Decision

The first bounded Generic profile is `GenericSourceProfileG0`.  G0 is the
existing resolver lease's exact nested two-loop forest, not a parallel one-loop
profile:

```hako
function f(i: i64, j: i64) {
  loop(i < 3) {
    loop(j < 3) {
      j = j + 1
    }
    i = i + 1
  }
  return j
}
```

The profile is provisional until the resolver supplies the complete product
described here.  The current cfg(test)-only lease proves only a bounded nested
carrier relationship; it does not yet prove the outer condition/step, complete
body coverage, typed literal context, or the function tail.  Therefore
`selection_open = false` until S0 closes those gaps.

“Positive Int64 constant” is a G0 admission rule, not a claim about the current
numeric-policy witness.  S0 must issue the corresponding typed range/progression
receipt; an untyped or non-positive bound is `Unresolved`/`Rejected` before a
candidate exists.  The broader policy substrate's operator support does not
silently widen G0.

The portable loop schema remains a loop-fragment algebra.  The post-loop
`return j` is not a `LoopExit::Return`: that exit denotes an exit from inside a
loop.  G0 therefore uses a separate verified completion envelope:

```text
VerifiedGenericRecipeProductG0 {
  verified_loop_recipe: VerifiedLoopRecipeV1
  join_signature: LoopJoinSigV1
  after_tail: VerifiedGenericAfterEffectG0
  carrier_effects: exact BindingRef/key relations
}

VerifiedGenericAfterEffectG0 {
  source_tail: Return(binding = j)
  after_carrier: j
  recipe_key_relation: exact source BindingRef -> recipe key
}
```

Extending `LoopRecipeV1` with a function tail is a separate common-schema
decision.  G0 must not silently omit the tail or widen that schema while
implementing its first producer.

## Two Recipe layers

These names must not be conflated:

| Layer | Owner | Role | Portable input? |
| --- | --- | --- | --- |
| `RecipeBody` / `RecipeBlock` | legacy Builder compatibility | AST-bearing lowering oracle and old route transport | No |
| `LoopRecipeV1` / `VerifiedLoopRecipeV1` | portable recipe contract | AST-free recursive loop semantics | Yes |

`GenericLoopV0Facts`, `GenericLoopV1Facts`, and their `RecipeBody` values are
legacy evidence only.  The Generic producer must not reconstruct AST, copy a
Builder recipe, or infer a portable operation from a legacy route.

## Canonical source profile

G0 accepts exactly one function with one outer loop `L0` and one nested loop
`L1`, in preorder.  The declared parameters are `i: i64` and `j: i64`; the
loop-carried bindings are those exact source `BindingRef`s, not names copied
into a later layer.

The complete source shape is:

```text
function body
  L0 condition: i < positive Int64 constant
  L0 body, in source order:
    L1
    i = i + positive Int64 constant
  L1 condition: j < positive Int64 constant
  L1 body, in source order:
    j = j + positive Int64 constant
  after-loop tail:
    return j
```

The following are outside G0 and produce typed rejection or unresolved status,
never a guessed candidate:

```text
>=, >, ==, !=, <= normalization not explicitly admitted by this table
symbolic/non-constant delta or zero/overflowing delta
multiply, divide, remainder, calls, method calls, new, print, fields, maps,
arrays, captures/upvars, shadowing, if, break, continue, extra or duplicate
loops, multiple tails, missing tail, opaque statements, foreign frame/site,
or incomplete source coverage.
```

G0 admits only `Less` and positive `Add` over exact `i64`.  `LoopRecipeV1`
supports `Less`, `LessEqual`, and `Equal`, and `Add`/`Sub`; support in the
portable schema is not an admission of every operator in G0.  Any future
normalization such as `i > 0` → `0 < i` requires a separate policy decision and
source receipt; it is not implicit.

## Source → Facts → Recipe mapping

The resolver owns the source lease, exact sites, forest/frame, scopes, and
`BindingRef` identity.  The shape issuer owns typed condition, step, body
effect, and coverage/exit proofs.  Policy owns numeric type/range/overflow and
progression admission.  The selector receives only an opaque candidate; it
does not see AST, names, receipts, or source leases.

| Source fact | Neutral fact / proof | Portable result |
| --- | --- | --- |
| `i` and `j` declarations | exact typed parameter and `BindingRef` receipt | recipe `inputs` value keys plus carrier relation; no name lookup |
| `L0` then `L1` | preorder forest and frame | recipe loop keys `L0`, `L1` |
| `i < c0` | condition proof: lhs=`i`, op=`Less`, rhs=`ConstI64(c0)` | condition block: `ReadBinding(i)`, `ConstI64(c0)`, `CompareI64(Less)` |
| `j < c1` | condition proof: lhs=`j`, op=`Less`, rhs=`ConstI64(c1)` | nested condition block with the same mapping for `j` |
| `j = j + d1` | ordered body effect, target=`j`, positive `Add` delta | body: `ReadBinding(j)`, `ConstI64(d1)`, `BinaryI64(Add)`, `WriteBinding(j)` |
| `i = i + d0` | ordered body effect, target=`i`, positive `Add` delta | outer body after `L1`: `ReadBinding(i)`, `ConstI64(d0)`, `BinaryI64(Add)`, `WriteBinding(i)` |
| `return j` after the forest | complete function-tail proof | `VerifiedGenericAfterEffectG0`, never an inner loop exit |

Every body assignment emits its own lhs `ReadBinding`; it must not reuse a
condition's value identifier or rely on an undocumented dominance shortcut.
Recipe keys are allocated by the Generic producer in preorder only after all
source sites and policy receipts are sealed.  `BindingRef` identity is copied
as provenance, while physical `ValueId`/PHI remains the sole responsibility of
function-owned Binding SSA.

## Carriers, coverage, and effects

G0 has carriers `i` and `j`.  `j` is both the nested loop carrier and the
after-tail read.  The product must prove:

```text
all declared source sites belong to one owner/origin/source-kind
all loop members belong to one forest/frame
each condition/step/body/tail site is inventoried exactly once
each emitted Read/Write has one exact BindingRef relation
the ordered body is [L1, update(i)]
the after-tail reads the post-forest j carrier
no opaque/foreign/duplicate/uncovered source site remains
```

The effect product is explicit and typed.  It contains at least the ordered
body effects, carrier reads/writes, and the after-tail return.  It does not
contain AST nodes, `RecipeBody`, route IDs, MIR `ValueId`s, synthetic names,
or retry/fallback instructions.

## Outcome algebra and rejection boundary

The source-to-policy chain has four distinct outcomes:

```text
Ready       = every required fact, type, range, and progression proof sealed
Unresolved  = a required observation/type context is absent or opaque
Rejected    = a known source fact violates G0 policy or ownership/provenance
NoCandidate = the selector received no sealed candidate; this outcome is owned
  by the whole-unit no-loop/zero-candidate boundary, not by a partial G0
  observation and not by a producer retry
```

`Unresolved` and `Rejected` are not converted into a guessed candidate.
`NoCandidate` is a selector result, not a producer retry signal.  A selector
must have exactly one sealed winner; zero, multiple, or incomplete candidates
are typed terminal outcomes.  There is no re-resolution by name, legacy route
alias, retry, fallback, or post-effect continuation.

## Selection-open gate

The next implementation row may open only when all of these are true:

1. G0 source grammar and positive/negative matrix are fixed.
2. Every source node category has exactly one typed fact mapping or typed
   reject/unresolved row.
3. `LoopRecipeV1` coverage and the separate after-tail envelope are sealed.
4. Numeric type, literal, operator, range, overflow, and progression receipts
   are co-sealed with the source `BindingRef`s.
5. The candidate envelope carries carrier, condition, step, body-effect,
   coverage, exit, and exact lease provenance without exposing internals.
6. The selector outcome algebra and exactly-one winner rule are fixed.
7. No legacy AST/Builder fallback or retry remains in the planned path.

Only then may S0 implement the resolver extension.  Until the full ladder is
complete, Generic production selection, demand, Recipe/key issuance,
Builder/MIR callers, runtime activation, and reference-language claims remain
zero.

## Ordered implementation ladder

```text
GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0
  this document: source grammar, mapping, tail/effect envelope, rejects

GENERIC-SOURCE-TO-PORTABLE-RECIPE-S0
  resolver lease extension: outer/inner condition+step, body effects,
  coverage/exit, typed literal context; cfg(test)-only witness

GENERIC-SOURCE-TO-PORTABLE-RECIPE-S1
  typed Generic candidate envelope; opaque, move-only, no selector caller

GENERIC-SOURCE-TO-PORTABLE-RECIPE-S2
  pure exactly-one selector; zero/multiple/incomplete typed outcomes

GENERIC-SOURCE-TO-PORTABLE-RECIPE-S3
  Generic demand and key/effect relation; no physical IDs

GENERIC-SOURCE-TO-PORTABLE-RECIPE-S4
  producer, verifier, JoinSig, and after-tail product; caller remains gated

GENERIC-PHYSICAL-CUTOVER-M10
  one named production caller, old-edge retirement, no retry/fallback
```

Each implementation commit must update `CURRENT_STATE.toml`, the active
workstream, this pipeline SSOT, and the support README in the same commit.
The exact `docs/reference/**` language/reference documents are updated only at
the corresponding production activation/closeout, and every implementation
commit must carry the planned reference-update row forward.  No implementation
is authorized by this D0 document alone.
