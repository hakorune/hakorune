---
Status: design_stop__scalar_loopcond_mapping_selected_for_design
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-LOOPCOND-CARRIER-RELATION-D2
Parent: mir-call-parser-array-push-b3-branch-continuation-d1-2026-09-23
NextCard: same-card__resolve_scalar_caller_type_and_completion_mapping
Implementation permission: false; design/taskification only. Tasks D1-D3 below must close before selecting implementation. No compiler, fixture, receipt, or production switch is authorized by this record.
---

# LoopCond Recipe carrier relation D2

## Six-line brief

```text
Decision: design scalar LoopCond in existing owners first; B3 String/Array joins follow separately.
Source authority + canonical issuer: same-function resolver ledger and source view; extend compiler projection and existing callable Recipe/semantic-program issuers.
Non-authority: names, AST rescans in physical lowering, MIR-derived types/effects, unrelated GenericLoop receipts, and post-hoc product pairing.
Fail-fast boundary: exact source/type/exit/coverage and co-seal relations must reject before physical allocation; no retry after selected membership.
Smallest next slice: D1 below names the real scalar caller cohort and its old edge; D2/D3 settle typing/effect and completion APIs before implementation.
Non-claims: no landed scalar producer, production cutover, old-edge removal, B3 completion, String/Fault ABI, tail push, or backend parity.
```

## Corrected entry premise (2026-09-23)

The read-only worker `loop_issuer_design` confirmed the implementation gaps
below, but found the previous entry condition circular. The
[entry policy](../design/current-docs-update-policy-ssot.md#implementation-entry-and-retirement-conditions)
allows a **named consumer to implement or connect**. Connection, new tests and
resulting caller-zero are migration outputs. A shared MethodCall writer may
remain while one selected old responsibility/edge is removed. Its whole-file
retirement is not a prerequisite for this design.

This supersedes this card's earlier global frontier-pause recommendation and
its requirement for an already-connected LoopCond issuer. It does not turn
missing implementation into a completed mapping. The earlier premise-reset
count audit remains narrowly valid: B3, S6D and parser forest ancestry were
different responsibilities, not three consecutive identical failures. That
count does not override the entry policy or prevent resolving internal design.

B3 currently has exact source BindingRefs and structural LoopCond/GeneralIf
Recipe items, but lacks the co-issued carrier/join relation needed for safe
branch-state restoration. A detached relation in
`SourceLoopCondPhysicalInputV1` cannot supply that meaning. The separate
portable owner has useful schemas and consumer seams, not a ready B3 contract.

## Selected design order and boundary

Choose the scalar LoopCond profile below for the first design. Names and
literal values are witnesses, not dispatch selectors:

```text
local i = <I64 literal>
loop (i < <I64 literal>) {
    i = i + <I64 literal>
    if (i == <I64 literal>) { break } else { continue }
}
return i
```

One initialized-local carrier, one scalar write, two comparisons, an explicit
Break/Continue branch and an exact return read. Require the existing source-
declared I64 result ABI on the enclosing function (omitted from the snippet);
do not infer a default result ABI. Exclude calls, String/Array,
nested loops, shadowing, GeneralIf and nonlocal exits. Membership comes from
resolver identities and exact sites, never `i` or these textual examples.

The existing strict S6D witness has only an If in the body. With pure predicates
and no write, a taken Continue cannot terminate. The step-bearing profile is
an explicit proposed extension, not an assertion that the old projector already
accepts it. Retain the strict witness's observation contract; do not silently
reclassify it. The design includes terminating Continue and Break evidence
without demanding termination proof for all admitted programs.

M8 S6D remains producer/observation scope. The later callable production
extension below is a separate Promote row, subject to existing M10 semantic-
program and production-entry requirements. S6D producer green is not permission
to select a production route, claim all-family coverage, or skip M8/M9 gates.
If that ordering prevents the exact caller switch, D1 must record the precise
remaining prerequisite; it must not label a producer-only row a cutover.

## Owner mapping to settle

| Stage | Existing owner / extension seam | Required relation |
| --- | --- | --- |
| Source membership | `compiler/loop_cond_break_continue_projection.rs` | same `ResolvedFunctionLoweringInputV1` / `FunctionSourceViewV1` and ledger; declaration, initializer, reads, write, loop/if and resolved exits |
| AST-free Facts | `loop_structural_facts/loop_cond_break_continue_source.rs` | BindingRef, exact sites, source literals/operators, scope/exit targets and full coverage; no Recipe keys or physical IDs |
| Source type/effect | compiler source-semantic validation; exact helper unresolved in D2 | prove I64 initializer/read/add/write closure and Bool comparisons using existing source arithmetic semantics; no default Pure or MIR signature inference |
| Recipe | existing `loop_recipe_contract` producer owner | deterministic role-to-key issuance; Predicate block, ReadBinding/ConstI64/CompareI64/BinaryI64/WriteBinding, If, Break/Continue |
| Co-seal | `compiler/callable_single_loop_recipe_coseal.rs`, `compiler/callable_semantic_program.rs` | one source context, source-bound Core, operation/effect and initialized-local relations, Recipe and that Core's JoinSig continuation |
| Physical consumption | `normal_callable_semantic_loan_port/canonical_route.rs` through prepared operation and `resolved_lowering/loop_recipe_physicalizer/callable_lowerer.rs` | consume the co-sealed relations in the existing SSA session, segment allocator/dispatcher and publication owner; no second physicalizer/PHI solver |

Paths above are relative to `src/mir/`, except physical paths relative to
`src/mir/builder/`. Names identify existing extension seams, not landed APIs
for the new profile. Recipe V1 can express the scalar operations; this alone
does not establish typing/effect, callable admission or completion contracts.

Continue transfers the post-write carrier to the header. Break transfers it
to After. A false header transfers its current carrier to After. The return
reads the same source BindingRef through the After relation. The current
callable wrapper's prelude-call/separate-tail profile must gain a real
initialized-local/After-return boundary variant: no fake prelude or empty
receipt. Physical selection must consume the specified predicate relation,
not find the first `CompareI64`; tail completion must not assume header read.

## Ordered tasks and observable exits

| Task | Work | Done evidence |
| --- | --- | --- |
| D1 — real callers and old edge (next) | Enumerate the finite existing normal-callable entry cohort, exact source witness, canonical selection and current raw destination. Reconcile the separate production row with M10 prerequisites. | Named caller/terminal/delete tuple and remaining prerequisites; no invented production claim from a new fixture alone. |
| D2 — scalar meaning | Name the existing source-semantic owner and helper to reuse, extend, or implement, with the exact typing/arithmetic/effect input/reject contract for initializer, add, comparisons and write. | Source-derived I64/Bool/effect mapping, including overflow/Fault semantics and full source-row coverage; no synthetic Pure/default authority. |
| D3 — callable boundary | Specify initialized-local ingress, source If transfer and exact After-to-return relation in existing co-seal/completion/ABI APIs. | Concrete API/type seams, owner-issued continuation and one-shot residual checks; accepted six-line implementation brief plus module/reference contract. |
| I1 — S6D producer/observation | Only after D1-D3: extend projection/Facts and Recipe/Core/JoinSig co-seal for the selected scalar grammar. | Focused positive/negative source-to-product evidence and existing strict-profile regression; explicitly no production completion. |
| I2 — production Promote | After applicable production prerequisites: extend the existing physical consumer and switch all named cohort callers; retire their old edge in the same bounded series. | Source-to-selected-publication result, no raw retry, selected caller-edge zero and retained legacy inputs unchanged. |
| C — closeout | Record evidence, reusable guard, README/reference and pointer; classify all reds. | Actual selection/cutover/delete receipt, not merely a logical producer or local test. |

D1-D3 are internal design work available now. They are not a wait for another
agent or for missing code to appear. No unresolved API/typing/caller choice is
silently treated as accepted. Implementing I1/I2 is outside this design-only
turn and remains gated by the mapping above.

### Planned retirement, not observed deletion

Candidate old responsibility: for the exact selected callable grammar,
canonical `Outside` routing into compatibility/raw body lowering and its
LoopCond planner/AST carrier collection. D1 must identify the actual caller
branch and selected destination before this becomes a deletion instruction.
Relevant seams include `control_flow/plan/normalizer/mod.rs`,
`control_flow/plan/recipe_tree/loop_cond_composer.rs`, and source child
`raw_loop_child_entry.rs` → `control_flow/plan/features/loop_cond_bc_source.rs`
(all relative to `src/mir/builder/`).

Remove that cohort's old selection/retry edge when its replacement is selected.
Retain shared classifiers and MethodCall implementations still used by other
supported shapes. A source guard and executed negative path must show selected
membership cannot retry through the raw owner. Whole-schema retirement and
repository-wide caller-zero are separate later claims.

### Acceptance matrix to implement

- Initial zero-iteration return; Continue at least once then Break; multiple
  iterations ending via false header. Check the returned carrier value.
- Alpha-renaming preserves behavior; Break and Continue target the same exact
  resolved loop. Exercise the real entry and selected publication terminal.
- Reject missing/duplicate/foreign binding or product, wrong exit target,
  predicate class drift, shadowing, mixed co-seal and residual source/effect
  rows before physical allocation. Grammar exclusion must remain distinct
  from invalid selected membership; the latter is terminal, never fallback.
- Preserve original strict S6D observations and supported legacy inputs.
  A new focused fixture is not a replacement for required existing acceptance.

## B3 follows the scalar design, not bundled into it

`StringHelpers.split_lines` still needs `i` and `last` outer carrier relations,
GeneralIf then-write/implicit-else identity for `last`, and iteration-local
`ch` non-escape. `arr`, `s`, `n` are read-only in the selected loop. Its nested
ArrayPush row and substring operation need their existing source-owned
operation receipts in the same issuance. Scalar success does not prove these.

Preserve D1: branch reset touches only current ledger values and active
origins; reads/assignments/calls/materialized locals/consumed rows/emission
ports/historical origins remain monotonic. Both branches compile once.
B3 implementation remains unauthorized until its own mapping closes. The
post-loop tail push, runtime Text/Fault ownership, serializer and backend
parity remain outside this card. Shared MethodCall whole-writer deletion is
neither claimed nor a prerequisite for selected-edge migration.
