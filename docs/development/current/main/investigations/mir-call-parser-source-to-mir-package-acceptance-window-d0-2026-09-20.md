---
Status: design_stop__2026-09-21__ParserLoopBreakCompositeDependencyParked
Task: MIR-CALL-PARSER-SOURCE-TO-MIR-PACKAGE-ACCEPTANCE-WINDOW-D0
Current execution row: MIR-CALL-PARSER-SOURCE-TO-MIR-PACKAGE-ACCEPTANCE-WINDOW-D0
Date: 2026-09-21
Parent: mir-call-parser-loopcond-source-handoff-i0-2026-09-19.md
Implementation permission: false; package acceptance remains parked at the named composite LoopBreak dependency terminal
NextCard: none__design_stop
---

# Parser source-to-MIR package acceptance window D0

## Six-line brief

```text
Decision: keep the selected parser tuple at a design stop until its evidence
  can be observed through the installed normal semantic package without
  skipping or reclassifying another callable in that same package.
Source authority + canonical issuer: the existing
  VerifiedNormalCallableSemanticPackageV1, its installed
  NormalCallableSemanticPackagePortAdapterV1, and the resolver-issued
  `(ParserProgramBox.parse/2, source-site, ParserStringUtilsBox.starts_with/3)`
  relation from the same invocation.
Non-authority: a target-only lowerer, method-name/ordinal filtering, a cloned
  package, a subset work plan, LoopBreak fallback, GenericLoop retry, AST/MIR
  rescans, or a synthetic Cataloged/Selected receipt.
Fail-fast boundary: before catalog installation or Builder effects, the
  acceptance observation must co-seal caller, exact source site, target,
  signature/result/effect relation, and the explicit dependency terminal; any
  missing or foreign row rejects the package window.
Smallest next slice: design one existing-owner, source-keyed acceptance
  window that observes the selected tuple and records the
  `LoopBreakRecipe -> GenericLoopV1NotSelected` dependency terminal without
  lowering only a selected method. If no such owner exists, name the missing
  LoopBreak source consumer as a separate bounded design row.
Non-claims: no parser source-to-MIR success, publication, old-edge deletion,
  production switch, fallback, VM/AOT parity, or warning cleanup.
```

## Exact bounded tuple

```text
caller       = ParserProgramBox.parse/2
target       = ParserStringUtilsBox.starts_with/3
target site  = resolver-issued source site for parser_program_box.hako:102
route        = LoopCondBreakContinue, with the selected LoopTrue child handoff
result       = ExactI64, required ordinal `[1]`
dependency   = the same merged package's raw `[LoopBreakRecipe]` terminal
```

The line number is only a reviewed source witness. The acceptance window must
carry the resolver's exact source site, caller/target key, signature, result,
effect, and package brand from one invocation. It must not rediscover any of
those relations from Hako text, method names, arity, AST, MIR, or `ValueId`.

## Evidence for the current stop

The selected LoopCond and LoopTrue source handoffs now cross their focused
physical edges. The merged parser lifecycle still stops before catalog
installation at:

```text
[freeze:contract][callable-loop/route-not-front-selected]
GenericLoopV1NotSelected
raw front = [LoopBreakRecipe]
```

This is dependency evidence, not parser acceptance. A read-only owner audit
confirmed that `lower_program_root_work_plan_with_callable_port_v1` lowers all
immediate and deferred work, and that
`PreparedNonMainStaticBoxMethodBatchV1::lower_root_with_port_v1` iterates every
method in each static box. The installed package's selected-call APIs are
scoped loans; they do not complete one callable while preserving package
coverage.

## Unselected dependency row: LoopBreak source consumer

The package terminal has one concrete missing owner, kept outside this D0's
selected tuple:

```text
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-CONSUMER-D0
Status: unselected__dependency_only__2026-09-20
Source authority: same-invocation resolver forest/exit ledger plus the
  existing `LoopBreakFacts`/Recipe outcome for the finite dependency methods.
Required consumer: an existing LoopBreak physical owner extended with one
  source-aware, move-only input; it must reuse the existing cleanup/verifier
  and never construct `LoopRouteContext` on the source path.
Current gap: the source projection and Facts/package co-seal now exist, but
  no source-lineage physical consumer is connected. `loop_break_composer` is
  mutation-first, and `route_loop_break_recipe` consumes the legacy
  `LoopRouteContext`/`MirBuilder` route. The remaining gap is the physical
  source-port adapter and its exclusive production delete tuple, not a missing
  source issuer.
Reopen condition: finite dependency caller/shape inventory, one issuer and
  one physical consumer, named pre-effect reject, and an exclusive old-edge
  delete-set. Until then, preserve `GenericLoopV1NotSelected` and do not
  promote this row from dependency-only.
```

## Owner and rejection matrix

| Concern | Existing authority | Required result |
| --- | --- | --- |
| source target/call relation | resolver-issued source target catalog | exact caller/site/target/signature/effect co-seal |
| package transport | `VerifiedNormalCallableSemanticPackageV1` and installed lowering port | one package-scoped observation; no clone or second ledger |
| root lowering | existing full work plan and method batches | preserve all-method coverage; no target-only filter |
| dependency terminal | existing LoopBreak/GenericLoop named reject | record explicitly; never reinterpret as success or fallback |
| missing/foreign selected row | existing source/package rejection boundary | reject before catalog/Builder effects |

The following are rejected in this D0:

- adding a target-only lowerer or filtering methods by name, ordinal, or arity;
- skipping the `LoopBreakRecipe` method to make the selected tuple appear green;
- cloning or partially installing the semantic package;
- converting the dependency terminal into `Ok(None)`, GenericLoop retry, VM
  compatibility, or another fallback;
- issuing a synthetic `Cataloged`/`Selected` receipt before the package owner
  and consumer are named.

## Handoff decision

The owner audit did not find an existing package-scoped acceptance observer
that can consume the selected tuple while preserving whole-package coverage.
The selected-call APIs are scoped loans, and `complete()` requires every
selected method; a target-only lowerer, skipped `LoopBreakRecipe`, or partial
package would violate the package authority. This is therefore a confirmed
`NoSafeSlice`, not an acceptance failure to hide.

The missing owner is now a separate bounded design row:
`MIR-CALL-PARSER-LOOPBREAK-SOURCE-CONSUMER-D0` in
`mir-call-parser-loopbreak-source-consumer-d0-2026-09-20.md`. That row owns
the finite dependency inventory, source-issuer decision, existing physical
consumer choice, pre-effect reject, and exclusive old-edge delete-set.

The parent LoopCond handoff remains implemented through D2, but this card
claims no parser source-to-MIR acceptance, publication, old-edge deletion, or
production switching. The named `GenericLoopV1NotSelected` terminal remains
unchanged until the next design row closes its owner boundary.


## Static dependency shape audit — 2026-09-21

The package stop was checked against the actual parser source rather than only
against the terminal text. The selected callable is `ParserProgramBox.parse/2`
and the selected target is the `starts_with/3` call at
`lang/src/compiler/parser/program/parser_program_box.hako:102`. That call is
inside the outer `loop(cont_prog == 1)` at `:81`, under the declaration branch.
The same loop has the finite exit/transfer rows already recorded by the parent
handoff: returns at `:104`, `:109`, `:127`, `:142`, `:161`, `:165`, and `:194`,
the outer `break` rows at `:85` and `:95`, the semicolon `continue` at `:186`,
and the final `break` at `:188`.

The current `LoopBreakFacts` owner can issue only loop condition, one break
condition, carrier/step expressions, and an optional three-statement
`source_topology` (`break_if`, `carrier_update`, `step`). The selected body also
contains whitespace/progress assignments, nested conditionals, declaration and
static-row branches, nested method observations, and several return exits. Its
resolver forest/exits therefore cannot be consumed by the direct LoopBreak
source input without losing a body-item or nested-exit relation. The existing
package observer cannot legally skip this method or install a target-only
subset: `complete()` requires every selected method.

This confirms the child composite row's `NoSafeSlice` from source structure and
owner fields. `MIR-CALL-PARSER-LOOPBREAK-SOURCE-COMPOSITE-D0` is now closed as
`ParkedSealed__NoSafeSlice`, and this package acceptance window remains at
`design_stop` until a same-owner composite LoopBreak body/item/exit product or
an explicit typed pre-effect retirement boundary is accepted. No fallback,
LoopCond reclassification, or old-edge deletion is authorized before that
decision. The deletion order remains source acceptance, production cutover,
caller-zero proof, then removal of only the selected parser edge.

## Composite source-product contract — design-only proposal — 2026-09-21

The missing product can be designed without adding a second route authority.
The canonical issuer remains `CallableGenericLoopSourceFactsIssuerV1`, in the
same resolver/planner invocation that already issues the LoopBreak Facts and
package row. Its source product must be a source relation product only; it must
not carry `RecipeItem`/`StmtRef` keys, `BodyId`, BasicBlock/Value IDs, or a
physical route selector.

The co-sealed product has five relation groups:

1. **Root identity** — callable owner/origin/source kind, root loop site `:81`,
   condition site, and the existing resolver forest/frame key.
2. **Body roles** — an ordered source-site row for every root body statement,
   branch body, and nested child body. Each row keeps its parent body/member
   relation and exact source statement or expression site; no AST or line
   reconstruction is permitted.
3. **Exit ledger** — every `break`, `continue`, and return transfer observed
   under the root, paired with its resolver `ResolvedExitRecordV1`. Missing,
   foreign, duplicate, or target-mismatched exits reject before Builder effects.
4. **Child route relations** — `:131` and `:182` retain the planner-issued
   child disposition. `:182` remains the existing literal-true LoopTrue route;
   the composite product only nests that relation and never reclassifies it as
   LoopBreak. Unsupported children produce typed absence, not an empty option.
5. **Call/target relations** — resolver-issued method-call bindings for the
   `starts_with/3` sites plus the selected publication/CoreMethod relation from
   the same invocation. Dropped required sites, multiple selected targets, and
   requirement mismatches remain named package rejects.

The Recipe producer consumes this source product once and creates the existing
Recipe vocabulary: `RecipeItem::Stmt`, `IfV2`, `Exit`, and nested `LoopV0` with
`RecipeBlock` bodies. It is the only layer allowed to mint `StmtRef`/`BodyId`
and `CondBlockView` relations. The associated-source physical provider then
rechecks each Recipe item against the co-sealed source port, including nested
body length and statement syntax, before `lower_loop_v0_core` allocates a
frame. The generic located representation remains outside this bridge because
it rejects nested `LoopV0`.

The implementation row may open only when this product is issued as one
owner-scoped candidate-or-typed-absence, the Recipe mapping consumes it once,
and the physical input carries the same forest, exit, child, carrier, and target
relations without a second scan. Until then the current named terminal and
`NoSafeSlice` remain authoritative; this section grants no code, fixture,
fallback, package-success, production-switch, or deletion permission.
