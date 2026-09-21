---
Status: design_stop__2026-09-21__NoSafeSlice_CompositeOwnerMissing
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I1
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I1
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-source-product-i0-2026-09-21.md
Implementation permission: false; select the existing owner that will retain the composite inventory before any code change
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I1
---

# Parser composite LoopBreak source-product consumer I1

## Six-line brief

```text
Decision: choose one existing source/Facts owner to retain the composite
  LoopBreak body inventory and its resolver exit ledger.
Source authority + canonical issuer: the same resolved parser function input,
  resolver loop forest, and resolved exit ledger used by I0.
Non-authority: AST/name/line rescans, LoopRouteContext inference, generic
  located-body retries, new Recipe keys, physical IDs, fallback, or VM routes.
Fail-fast boundary: missing consumer, foreign root, dropped nested body, or
  exit/site cardinality mismatch is a NoSafeSlice rather than a new receipt.
Smallest next slice: audit the existing LoopBreak Facts/Recipe owner and the
  source physical adapter for a single exact composite inventory handoff.
Non-claims: no composite Recipe, package admission, production switch, or
  legacy-edge deletion until the owner and full source-to-Recipe mapping close.
```

I0 proved the source-only inventory guard on the existing direct projection.
The parser fixture's root `:81` and nested loops `:131`/`:182` still have no
selected consumer that can retain the ordered body roles and resolver exits
through Facts into the existing Recipe/physical owner. This is a design
boundary, not a permission to add a parallel product or to revive LoopCond
fallback.

The next worker audit must name the canonical consumer, its retained relation,
and the exact source-to-Recipe correspondence before implementation resumes.

## I1 existing-owner audit — 2026-09-21

The audit keeps this row at `NoSafeSlice`; it did not find a safe existing
consumer for the parser composite root.

| owner | observed boundary | consequence |
| --- | --- | --- |
| `normal_callable_loop_source_facts/loop_break.rs:50-60` | candidate owns only the direct projection, planner outcome, and direct terminality | the composite body cannot enter this candidate without dropping roles |
| `normal_callable_loop_source_facts/loop_break.rs:322-369` | physical input requires `facts.loop_condition`, `body.len() == 3`, and a one-statement break-if | the parser root's nested bodies and multiple exits have no retained input slot |
| `normal_callable_loop_source_facts/loop_break.rs:379-416` and `control_flow/plan/recipe_tree/loop_break_builder.rs:207-226` | source Recipe requires the three-statement body and direct break-if | no source-to-Recipe mapping exists for the composite body |
| `control_flow/plan/parts/associated_source/callable_loop_source_lowering.rs:7-10,157-163` | associated-source lowering can recurse only after a Recipe; an opaque `Loop` is a named reject | this is a physical consumer seam, not a source-product issuer |
| `control_flow/joinir/route_entry/registry/handlers/routes.rs:26-54` | legacy route consumes `LoopRouteContext` and `MirBuilder` | it cannot be reused as the source-backed consumer |
| `compiler/dynamic_full_body_source.rs:187-230` and `dynamic_full_body_recipe/mod.rs:177-210` | dynamic owner hard-codes a different three-statement root/body profile | it is not the parser composite owner |
| `compiler/loop_cond_break_continue_projection.rs:145-178` and `normal_callable_loop_source_facts/generic/issuer.rs:39-96` | LoopCond source admission requires one root-body branch with explicit else, then/else exit arity 1 | parser root `:81` has multiple body statements and nested loops, so LoopCond is not an existing consumer |

The existing associated-source Parts spine is therefore reusable only after a
new composite Recipe is issued. It cannot be selected as the missing issuer,
and the direct LoopBreak Facts/Recipe owner cannot retain the parser inventory
without changing its accepted shape. No code, fallback, target-only filter,
new semantic receipt, or production switch is authorized by I1.

The generic issuer's LoopCond branch does not provide a hidden escape hatch:
it is entered only after the located LoopCond projection succeeds. The
projection's one-body/explicit-else contract rejects this parser root before
the LoopCond physical input is constructed. Reclassifying the root merely to
reach that consumer would change route authority and is outside this row.

### Bounded next decision

The next design decision must choose between a same-owner extension of the
LoopBreak Facts/Recipe product and a separately named existing source owner
that already has a complete body/exit vocabulary. The decision must include
the parser root `:81`, nested loops `:131`/`:182`, all resolver exit rows, and
the exact `RecipeItem` mapping before implementation permission can reopen.
