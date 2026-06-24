# 296x-1629 MIRBUILDER-CANONICAL-COMPAT-ENTRY-COVERAGE-STOP-001

Status: closed
Date: 2026-06-22

## Purpose

Add the canonical Program(JSON v0) compat executable entry under the MirBuilder
home, then stop before redirecting callers because the old compiler-tree entry
still accepts Program(JSON) shapes that `MirBuilderBox` does not.

## Implemented

New canonical entry:

```text
lang/src/mir/builder/compat/program_json_v0_entry.hako
```

Contract:

```text
HAKO_PROGRAM_JSON or HAKO_PROGRAM_JSON_FILE
  -> MirBuilderBox.emit_from_program_json_v0(program_json, null)
  -> one MIR(JSON v0) line on stdout
```

The entry is intentionally thin. It does not import
`lang/src/compiler/mirbuilder/**`.

## Evidence

With a `vm-reference` release binary, the old live caller path is still green:

```bash
cargo build -q --release --bin hakorune --features vm-reference
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase1_min_vm.sh
```

```text
[PASS] hako_mirbuilder phase1 pin: PASS
```

The canonical entry also accepts the same phase1 Program(JSON) payload when
called directly:

```text
canonical_entry_probe=green
```

## Stop Point

The broader Program(JSON) contract pin failed after temporarily redirecting it
to the canonical entry:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
```

Failure:

```text
[FAIL] print_node: emit route failed (rc=2)
[mirbuilder/internal/unsupported] supported: Return(Int|Binary(Int,Int)|Call), Expr(Call env.console.log Int)+Return(Int), If-nested, and probe nested-ternary
[builder/selfhost-first:unsupported:no_match]
[freeze:contract][hako_mirbuilder] MirBuilderBox.emit_from_program_json_v0 returned null/empty
```

Interpretation:

```text
old compiler-tree Program(JSON) entry accepts hand-crafted Print node contract
canonical MirBuilderBox does not yet accept that same shape
```

This is a behavior coverage gap, not a path wiring problem.

## Decision

Do not redirect live callers yet.

The following remain on the old compiler-tree entry until the coverage gap is
closed or explicitly accepted as a compat split:

```text
src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs
tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*.sh
tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
```

## Stop Lines

```text
do_not_repoint_stage_a_bridge=1
do_not_repoint_contract_pin=1
do_not_delete_compiler_mirbuilder=1
do_not_add_fallback_to_canonical_entry=1
```

## Next

```text
next_blocker=MIRBUILDER-CANONICAL-COMPAT-PRINT-CONTRACT-DECISION-001
```

Design question:

```text
Should canonical MirBuilderBox learn the old Program(JSON) Print node contract,
or should the old compiler-tree entry remain the explicit Program(JSON)
contract-pin owner until a broader compat drain?
```
