# 3472 - LANGV1-GRAMMAR-MIR-CORRECTNESS-COMPILE-COST-CORRECTIVE-001

## Status

Active corrective card. Repair the correctness and compile-cost prerequisites
found after 3470 before resuming the parked 3471 transport consultation.

Progress: Slice 1 complete. Slice 2 correctness WIP is parked under the named
stash `wip/3472-slice2 match strictness blocked by MIR compile scaling` because
its strict Match control-flow delta makes release MIR compilation exceed 60
seconds and debug compilation exceed 120 seconds. Slice 4 has identified and
removed the per-function full-module clone, added compiler-owned timing, and
rejected an isolated dynamic-loop pathology. The dominant remaining cost is
semantic route convergence on the merged parser module. Resume Slice 2 after
the Slice 4 keeper commit reapplies cleanly.

## Problem Statement

```text
Hako expression freeze propagation is statement-context dependent.
Hako Match treats required delimiters as optional.
The profile facade seeds an Option-only enum inventory.
ProgramJSON Match acceptance is wider than Rust MIR lowering.
The parser adapter compiles 259 functions for every fixture.
Debug AST-to-MIR compilation costs about 74 seconds; VM execution costs 0.07s.
Per-function finalization clones the accumulated MirModule.
The workstream contains a stale duplicated current pointer.
ParserBox is 951 lines.
```

## Structural Rules

1. Keep grammar authority in `grammar/unified-grammar.toml` and the shared
   corpus. Shell scripts and adapter facades must not own spelling policy.
2. Introduce one expression-result contract boundary. Statement parsers must
   not each invent freeze propagation logic.
3. Match required-token validation belongs to `ParserMatchBox`; MIR support
   policy belongs to the MIR Match owner. Do not compensate across layers.
4. Enum inventory comes from the parser invocation context/source inventory,
   not a hard-coded `Option` row in the grammar-profile facade.
5. BoxCount and BoxShape stay in separate commits. Correctness slices do not
   include MIR performance refactors.
6. New or split source files stay below 800 lines. `ParserBox` must finish this
   card below 800 lines.
7. No source-text rewrite, implicit Compat retry, runtime fallback, warn-only
   mismatch, by-name route shortcut, or fixture-specific success branch.
8. Keep 3471 parked. This card does not choose its transport A/B decision.

## Ordered Slices

### Slice 1 - Parser contract-result boundary

Create one parser-owned contract-result helper and route local, return, print,
assignment, expression statement, condition, loop, and throw expression results
through it. A freeze result must reach `parse_program2` unchanged before JSON
publication.

Acceptance:

```text
expression_contract_result_owner_count = 1
statement_context_specific_freeze_embedding = 0
malformed_program_json_on_contract_error = 0
```

Implementation: `ParserBox.freeze_contract` owns the first expression contract
error for a parse invocation. `parse_program2` consumes that latch before any
statement JSON publication. The formerly broken `return peek` case now returns
`parser/hako_peek_canonical_rejected` as a structured adapter rejection.

### Slice 2 - Match grammar strictness and inventory

Require opening brace, arrows, payload close, arm progress, arm limit, and final
brace. Replace the facade's Option-only inventory with the existing invocation
inventory owner. Add positive and negative shared-corpus fixtures for Option and
one user enum.

Acceptance:

```text
match_required_token_fail_fast = 1
match_cursor_progress_guard = 1
profile_facade_option_inventory_hardcode = 0
source_enum_inventory_owner_count = 1
```

### Slice 3 - ProgramJSON to MIR support matrix

Enumerate the canonical Match shapes accepted by ProgramJSON and Rust MIR.
Either lower a contract-required shape through the existing EnumMatch owner or
reject it before publication with a stable unsupported tag. Do not add fallback.

Acceptance:

```text
programjson_mir_match_support_matrix = 1
parser_accept_mir_silent_gap = 0
runtime_fallback = 0
```

### Slice 4 - MIR compile-cost BoxShape

Add compiler-owned timing for prepare/lower/finalize and post-build stages.
Fix the smallest proven scaling owner, starting with the per-function full
module clone. Lock 50/100/250-method scaling and literal/dynamic loop-bound
fixtures before changing additional analyses.

Acceptance:

```text
mir_compile_stage_timing = 1
per_function_full_module_clone = 0
compile_scaling_fixture = 1
dynamic_loop_bound_pathology_reproduced_or_rejected = 1
```

Evidence:

```text
adapter merged source = 166895 bytes / 27 static boxes / 259 functions
debug AST-to-MIR before clone removal = 78.40 seconds
debug AST-to-MIR after clone removal and convergence no-change stop = 70.58 seconds
VM execution = about 0.07 seconds

50/100/250 synthetic method fixture = 67/79/126 milliseconds
literal/dynamic isolated loop fixture = 54/55 milliseconds
isolated dynamic-loop pathology = rejected

dominant merged-module stage = semantic route convergence
first semantic refresh outer iterations = 4
post-canonicalization refresh outer iterations = 2 after no-change stop
```

The convergence stop consumes the existing changed facts from the global-call
and user-box route families. It does not add a route heuristic or a
fixture-specific shortcut. Five directly affected route tests are already red
with the same values at the clean pre-Slice-4 HEAD, so they are recorded as
baseline debt rather than treated as evidence for this change. The broad
`cargo test --lib` suite also has process-global test-state contamination; its
parallel and serial results are not a clean Slice 4 oracle.

### Slice 5 - Corpus-driven batched conformance

Make the shared corpus the fixture SSOT. Compile the Hako adapter once per
matrix run and execute multiple source/profile observations in process. Keep
quick structure, process-unit, and full conformance as separate entrypoints.

Acceptance:

```text
grammar_fixture_ssot_count = 1
hako_adapter_compile_per_fixture = 0
quick_guard_runs_full_matrix = 0
```

### Slice 6 - Structural cleanup

Split `ParserBox` below 800 lines without changing acceptance, remove the stale
workstream current token, and give `ParserPeekBox` an explicit retirement gate:
zero live imports, replacement corpus green, and canonical Match parity green.

## Verification

Each slice lands with its focused unit/fixture tests and `git diff --check`.
Run the reusable quick grammar guard after parser slices, compiler scaling tests
after Slice 4, the full corpus runner after Slice 5, and pointer guard after
every docs pointer change.

## Non-Claims

```text
hako_from_migrated = 0
hako_from_transport_implemented = 0
language_v1_grammar_closeout = 0
selfhost_claim = 0
new_route_authority = 0
runtime_backend_fallback = 0
full_lib_test_isolation_closeout = 0
semantic_route_convergence_closeout = 0
```

## Next

After all six slices are green, return to 3471 with the corrected conformance
scope and decide the Hako compatibility-transport A/B boundary.
