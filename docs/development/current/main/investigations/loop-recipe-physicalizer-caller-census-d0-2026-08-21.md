# Loop recipe physicalizer caller-zero / line-budget census D0

Status: `Parked design stop — caller-zero evidence exists; no production
consumer or retirement edge is named`

Date: 2026-08-21

Parent: `mirbuilder-cleanup-retirement0-d0-task-map-2026-08-04.md`

Current lane: parked behind `SCRIPT-DIRECT-STATIC-A-CONSUMER-CLOSURE-D0`.
This card does not retarget `CURRENT_STATE.toml` and does not authorize Loop
production work.

Decision: `LOOP-RECIPE-PHYSICALIZER-CALLER-CENSUS-D0`

Classification: `BoxShape` cleanup only for the future split; physicalization,
selection, and retirement are `NoSafeSlice` until a named production consumer
and an exact old-edge removal contract exist.

## Six-line brief

Decision: Keep the V1 Loop physicalizer and `loop_physical_prepare` in a parked caller-zero census; do not activate or delete them.

Source authority + canonical issuer: Existing resolver, Recipe, Completion, and DraftSeal products issue canary evidence; this cleanup card issues no semantic product.

Non-authority: test canaries, `ReadyLoopEntryV1`, `PreparedCallableLoopPhysicalizationV1`, line counts, names, G0 harnesses, and local green tests cannot authorize production selection or legacy retirement.

Fail-fast boundary: Before canary reduction, file splitting, or deletion, prove the complete caller/owner/dependency census and keep every unknown caller, missing consumer, or ownership drift at `NoSafeSlice`.

Smallest next slice: Record the exact caller-zero result for `issue_callable_loop_physicalization_v1`, classify the shared Callable/G0 canary dependency, and freeze a behavior-neutral split plan for the two near-limit files.

Non-claims: No Loop source-shape admission, Recipe/Join change, Builder/MIR/CFG/SSA/PHI effect, production switch, fallback, retry, backend, performance, or old-route retirement.

## Evidence at the census point

The first requested production-caller check is closed as caller-zero:

| Observation | Evidence | Result |
| --- | --- | --- |
| `issue_callable_loop_physicalization_v1` definition | `src/mir/compiler/loop_physical_prepare.rs:430` | one test-only issuer |
| direct call sites | `src/mir/compiler/loop_physical_prepare.rs:724,747`; `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/callable_canary.rs:419` | three test calls, zero production calls |
| compiler visibility | `src/mir/compiler/module_registry.in.rs:108-110` plus `loop_physical_prepare.rs:8` | `#[cfg(test)]` parent and module |
| physicalizer visibility | `src/mir/builder/resolved_lowering/mod.rs:142` | `#[cfg(test)]` parent |
| Generic G0 shared dependency | `src/mir/builder/resolved_lowering/mod.rs:37` and `generic_g0_physical_emitter_session.rs:10-14` | test-only harness depends on shared physicalizer pieces |
| current old production edge | `src/mir/control_flow/plan/lowerer/loop_lowering.rs::lower_loop_generalized` | separate route; not proven replaceable by this canary |

The directory plus `loop_physical_prepare.rs` is 7,372 lines by the current
filesystem census, not the earlier 6,577 estimate. The total is informational;
the actionable limits are the two files at the hard-stop boundary:

| File | Lines | Required treatment |
| --- | ---: | --- |
| `src/mir/compiler/loop_physical_prepare.rs` | 795 | no growth; split by owner before any new semantic work |
| `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_emitter.rs` | 794 | no growth; split by operation-emission responsibility before edits |
| `operation_dispatcher.rs` | 636 | watch the 760-line design trigger |
| `topology.rs` | 575 | no growth unless its owner boundary is explicit |

The four `manifest`/`loan` mentions reported in the adjacent README are
current canary-contract wording, not stale physicalizer authorities. The
physicalizer subtree and `loop_physical_prepare.rs` contain no literal
manifest/loan authority. The broader typed-error debt in
`normal_callable_semantic_loan_port.rs` and the constructor demand loan files
is a separate P1 row and must not be mixed into this cleanup.

`src/mir/README.md` is 106 lines and does not currently provide a navigation
entry for the test-only Loop physicalizer or its caller-zero status. A small
README navigation receipt belongs in the first cleanup implementation commit;
this D0 does not edit it.

## Exhaustive disposition table

Every census outcome has one owner and one allowed terminal. No `None`, empty
module, or compatibility label may merge these states.

| State | Issuer / evidence | Pre-effect behavior | Allowed terminal | Fallback policy |
| --- | --- | --- | --- | --- |
| `CallerZeroCanaryOnly` | caller guard plus both `cfg(test)` parents | no production effect | retain canary and parked task | no production selection |
| `SharedCanaryDependency` | Callable/G0 import/dependency census | preserve shared modules | profile-specific shrink plan | do not delete shared owner |
| `NamedProductionCaller` | exact non-test caller and owner contract | stop before switch until full handoff exists | open a separate production D0 | no canary substitution |
| `CutoverReady` | named consumer, fresh session, Completion/DraftSeal, and old-edge map | permit only the selected future cutover card | one production switch | no parallel route |
| `Retired` | caller-zero guard after switch plus old symbol/edge zero | no canary or old facade remains reachable | close the cleanup row | no resurrection |
| `UnresolvedCallerOrOwner` | incomplete grep, foreign import, or ambiguous shared dependency | freeze before reduction/split/deletion | return to census design | no guessed caller-zero |
| `NoSafeSlice` | canary treated as production, missing consumer, or semantic split | publish nothing and keep current route | remain parked | no fallback/retry |

Current state is `CallerZeroCanaryOnly + SharedCanaryDependency`, not
`CutoverReady` or `Retired`.

## Ordered cleanup ladder

This is a conditional ladder, not permission to start the next row now.

1. **D0 — caller census (this card).** Freeze exact non-test/test caller
   counts, parent `cfg(test)` scope, shared Callable/G0 dependencies, and the
   794/795 line budget.
2. **P0 — canary/profile shrink (future BoxShape).** Move duplicate evidence
   into existing focused owners and retain one positive/negative/fresh-session
   witness per Callable/G0 profile. Do not change operation meaning or create a
   selector.
3. **R0 — near-limit responsibility split (future BoxShape).** Split
   `loop_physical_prepare.rs` and `operation_emitter.rs` into child owners
   before either file grows. Preserve visibility, test names, and all existing
   guards; a line-count reduction alone is not completion.
4. **I0 — named production consumer (separate design stop).** Only if a real
   production caller is found, name the source-backed issuer, fresh session,
   Completion/DraftSeal/publication owner, selected route, and exact old-edge
   retirement map. The current census has no such caller.
5. **R1 — retirement (future BoxShape).** After a real switch, require caller
   zero, old symbol/edge zero, focused parity, and README/reference receipt
   before deleting the canary facade or shared shell.

Callable-only shrink must not delete shared G0 dependencies. Conversely, a G0
canary passing 15-row parity does not prove a Callable production consumer.

## Acceptance for the next implementation card

- [ ] A reusable guard proves exactly three test call sites and zero non-test
      call sites for `issue_callable_loop_physicalization_v1`.
- [ ] The guard proves both parent `cfg(test)` boundaries and records the
      Generic G0 shared dependency without treating it as production.
- [ ] Callable seven-row, Generic G0 fifteen-row, duplicate/late-failure, and
      fresh-session discard evidence remains green.
- [ ] The split plan assigns each moved function to one responsibility and
      keeps every touched source/check file below 800 lines; no compression
      or diagnostic shortening is used to cross the boundary.
- [ ] The first implementation commit adds the missing `src/mir/README.md`
      navigation receipt and updates the owning module/reference receipt.
- [ ] Any newly discovered production caller, old edge, or semantic owner
      mismatch stops the row and opens a separate design card.

## Explicit stop conditions

Stop and remain `NoSafeSlice` if any of the following is observed:

- a non-test caller reaches the issuer without a named source-backed owner;
- the current `lower_loop_generalized` route is claimed as replaced without a
  production selection and retirement edge;
- a canary, line count, `ReadyLoopEntryV1`, or G0 harness is used as a
  production semantic/physical authority;
- splitting either near-limit file requires a public API, semantic reorder,
  second dispatcher, fallback, retry, or new Receipt;
- shared Callable/G0 dependencies cannot be separated without changing the
  accepted operation/physical contract;
- a cleanup change would mix the adjacent manifest/loan string debt or the
  global `MirInstruction::Call`/metadata cleanup into this row.

## References

- `docs/development/current/main/investigations/mirbuilder-cleanup-retirement0-d0-task-map-2026-08-04.md`
- `docs/development/current/main/investigations/loop-recipe-physicalizer-module-split-r0-task-2026-08-07.md`
- `docs/development/current/main/investigations/loop-physical-prepare-design-correction-r0-task-2026-08-07.md`
- `docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md`
- `src/mir/builder/resolved_lowering/README.md`
- `src/mir/compiler/README.md`
- `docs/reference/mir/loop-recipe-contract.md`
- `tools/checks/loop_physical_transfer_authority_guard.sh`
