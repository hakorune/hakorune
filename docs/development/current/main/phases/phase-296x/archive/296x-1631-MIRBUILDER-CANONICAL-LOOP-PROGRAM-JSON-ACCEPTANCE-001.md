# 296x-1631 MIRBUILDER-CANONICAL-LOOP-PROGRAM-JSON-ACCEPTANCE-001

Status: closed
Date: 2026-06-23

## Decision

Accept the Stage-0 Loop Program(JSON v0) contract in the canonical MirBuilder
compat entry:

```text
lang/src/mir/builder/compat/program_json_v0_entry.hako
```

The old compiler-tree Program(JSON) entry is not used as runtime fallback for
this contract.

## Implemented Scope

- Added a canonical lowerer for the phase21 shape:

```text
Local(i = Int)
Loop(i < Int) {
  If(i < Int) { Return Int }
  i = i + Int
}
Return i
```

- Repaired the existing canonical simple loop lowerer for Stage-0 Program(JSON)
field order where `type` appears after payload fields.
- Kept MIR JSON emission in `CompatMirEmitBox` so lowerers do not own giant
string bodies.
- Re-enabled the Loop node case in the Program(JSON) contract pin.
- Redirected the phase21 Loop pin to the canonical compat entry.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase21_loop_if_return_var_min_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
```

## Next Blocker

```text
MIRBUILDER-CANONICAL-COMPAT-AST-PHASE0-DRAIN-001
```
