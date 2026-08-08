Status: closed implementation receipt
Date: 2026-08-09
Row: PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0
Parent: `parser-public-ast-postpass-final-closeout-d0-design-task-2026-08-09.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`

# FINAL-GUARD-CLEANUP-S0

## Purpose

Close the last parser postpass audit gap without changing parser semantics.
The caller-zero `source_gate_prune.rs` owner is retired, but two tracked B2/B3
guards still required that deleted file. This slice updates those guards to
the current shared projection/finalizer owners and fixes successor-state
acceptance in the historical D-I0 guard.

This is a guard/documentation cleanup row only. It does not open a production
switch, resolver/runtime work, grammar redesign, compatibility replacement,
retry, fallback, or a new parser semantic owner.

## Authority boundary

```text
BuildCfg projection:
  src/parser/build_cfg/prune.rs
  project_build_gate_program / BuildGateProjectionSelector

source-session prune:
  src/parser/source_seal.rs
  prepare_prune / commit_prune / source_seal_survives

finalizer alignment:
  src/parser/source_seal.rs
  private FinalizerCoveragePlanV1
  source_seal_finalizer.rs relation coverage
```

The retired `src/parser/source_gate_prune.rs` may remain only as historical
retirement evidence in docs and the dedicated retirement guard. No active B2,
B3, or closeout guard may require or import it.

## Required changes

1. Repoint B2 guard source checks from the deleted caller-zero walker to the
   shared `build_cfg/prune.rs` projection owner and the current finalizer test
   module.
2. Repoint B3 guard checks from `GatePruneOutputV1` to the current
   `OpenParserPostpassProductV1`/`FinalizerCoveragePlanV1` owners and the
   source-seal delegate tests.
3. Extend the historical D-I0 pointer guard to accept explicit postpass
   successors through `FINAL-CLOSEOUT-D0`.
4. Update active/reference wording so landed receipts name current owners;
   historical retirement mentions remain explicitly historical.
5. Add a dedicated closeout guard that checks the active focused suite,
   retired-helper absence, stale B2/B3 guard requirements, source line limits,
   and the known full-parser baseline-red classification.

## Acceptance

```text
b2 guard = green
b3 guard = green
D-I0 successor guard = green
NoElse / final-retire / final-D0 / cutover guards = green
current_state_pointer_guard = green
focused parser suites = green
full parser baseline reds are classified, not attributed to this slice
no active guard requires source_gate_prune.rs
source_seal.rs and all touched parser sources remain below 760/800 limits
docs, task map, README/reference, and check index updated in the same commit
```

The known parent-baseline reds remain parked and are not silently reclassified:

```text
PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0
PARSER-DIRECT-BIRTH-MIGRATION-TRANSPORT-D0
PARSER-LEGACY-WHILE-GRAMMAR-FREEZE-D0
PARSER-LEGACY-FOR-GRAMMAR-FREEZE-D0
PARSER-LEGACY-LOOP-GRAMMAR-FREEZE-D0
```

If a new failure appears outside this list, stop and open a design/diagnostic
row; do not widen a guard or add a fallback to make it green.

## Nonclaims

```text
no parser semantic change
no new source receipt
no SourceBuildGateBranchV1::NoElse path segment
no resolver/Builder/MIR/Recipe/runtime activation
no production postpass selection
no historical guard suite declared green by implication
```

## Closeout receipt (2026-08-09)

The B2/B3 guards now use the shared projection/finalizer owners, D-I0 and all
final successor guards accept this explicit cleanup successor, and the active
guard set no longer requires the retired `source_gate_prune.rs` file. The
cleanup guard itself checks those properties, source line limits, current
state synchronization, and the known parent-baseline red list.

Focused verification:

```text
bash tools/checks/parser_public_ast_postpass_final_guard_cleanup_s0_guard.sh
  = green (B2/B3/D-I0/final/noelse/retire/cutover subguards included)
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust shared_projection_emits_no_else_receipt_without_a_child_path
  = green
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust --test parser_build_cfg_gate
  = 12 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust --lib parser::source_seal
  = 12 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust --lib parser::postpass_envelope::tests
  = 7 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust --lib parser::string_postpass_entry::tests
  = 7 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust --test parser_grammar_profile
  = 17 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust --lib parser_
  = parent baseline: 198 passed, 5 known reds (listed below)
```

The five full-parser baseline reds are unchanged parent debt and are not
attributed to this cleanup row:

```text
ordinary_nested_selected_else_keeps_outer_to_inner_source_path
parser_birth_keeps_parent_constructor_delegation
parser_loopclean_while_stage3_normalizes_to_loop_ast
parser_legacy_for_range_surface_uses_shared_for_range_shape
parser_loop_scan_range_shape_preserves_lte_n_minus_one_ast
```

No parser semantic owner, receipt/path authority, production switch,
resolver/runtime integration, grammar redesign, compatibility replacement,
retry, or fallback was added. The next work must open a new design row.
