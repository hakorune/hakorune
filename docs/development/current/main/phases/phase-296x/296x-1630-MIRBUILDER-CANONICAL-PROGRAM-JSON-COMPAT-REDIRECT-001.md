# 296x-1630 MIRBUILDER-CANONICAL-PROGRAM-JSON-COMPAT-REDIRECT-001

Status: closed
Date: 2026-06-22

## Decision

Move the active Program(JSON v0) compat callers to the canonical MirBuilder
home:

```text
lang/src/mir/builder/compat/program_json_v0_entry.hako
```

The old compiler-tree Program(JSON) entry remains in the tree for later
freeze -> redirect -> drain -> delete retirement. It is not used as runtime
fallback in this slice.

## Implemented Scope

- Stage-A compat bridge points at the canonical compat entry.
- Active JoinIR MirBuilder Program(JSON) smoke scripts point at the canonical
  compat entry.
- Canonical `MirBuilderBox` accepts the Program(JSON v0) contract for:
  - hand-crafted `Print(Var)` after local initialization,
  - Stage-0 `Expr(Call env.console.log(Var))`,
  - Stage-0 fallthrough `If(Var == Int) { Return Int } ; Return Int`.
- `JsonFrag` instruction-array normalization no longer uses an in-body update
  immediately before loop control-flow.

## Deferred

`Loop(Compare <)` Stage-0 Program(JSON) remains blocked. The phase21 Loop
Program(JSON) pin stays on the old entry until the canonical acceptance blocker
is fixed; it is not treated as a runtime fallback for the redirected non-loop
contracts.

The AST JSON phase0 pin remains a separate old-entry drain target; this slice
only redirects Program(JSON v0) callers.

Attempts to add a small canonical loop lowerer exposed a selfhost compile
contract failure:

```text
[freeze:contract][ssa/phi_input/without_def]
fn=BuildProgramFragmentBox.inject_json_fragment/2
context=finalize_module_all_functions
```

That is an acceptance/compiler-shape blocker, not a reason to route back to
`lang/src/compiler/mirbuilder/`.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase1_min_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase10_min_vm.sh
cargo test -q mir_builder_program_path_is_stable
```

## Next Blocker

```text
MIRBUILDER-CANONICAL-COMPAT-LOOP-CONTRACT-ACCEPTANCE-001
```
