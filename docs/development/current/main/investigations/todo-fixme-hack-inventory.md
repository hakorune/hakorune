# TODO/FIXME/HACK Inventory (Phase 29bq)

Purpose: reduce drift by deciding whether each item should be **kept as TODO**, **moved to SSOT docs**, or **tracked as an issue**.

## Snapshot (2026‑02‑04)

- Search command:
  - `rg -n "TODO|FIXME|HACK" src -S`
- Total hits: **136**

### Top files (by count)

1. `src/mir/join_ir/lowering/loop_patterns/mod.rs` (21)
2. `src/mir/join_ir/lowering/loop_patterns/nested_minimal.rs` (11)
3. `src/mir/loop_pattern_detection/legacy/mod.rs` (6)
4. `src/instance_v2.rs` (5)
5. `src/bid-codegen-from-copilot/codegen/targets/vm.rs` (4)
6. `src/runner/modes/common_util/resolve/using_resolution.rs` (3)
7. `src/mir/loop_pattern_detection/features.rs` (3)
8. `src/mir/join_ir_vm_bridge/meta.rs` (3)
9. `src/tests/vm_functionbox_call.rs` (2)
10. `src/tests/vm_compare_box.rs` (2)
11. `src/runner/modes/common_util/resolve/prelude_manager.rs` (2)
12. `src/mir/join_ir_vm_bridge/joinir_block_converter/handlers.rs` (2)
13. `src/mir/join_ir_vm_bridge/handlers/new_box.rs` (2)
14. `src/mir/join_ir_vm_bridge/handlers/method_call.rs` (2)
15. `src/mir/join_ir/lowering/simple_while.rs` (2)
16. `src/mir/join_ir/lowering/if_phi_spec.rs` (2)
17. `src/mir/join_ir/lowering/bool_expr_lowerer.rs` (2)
18. `src/mir/join_ir/json.rs` (2)
19. `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` (2)
20. `src/boxes/json/mod.rs` (2)

## Triage decisions (Top 20)

| File | Count | Decision | Next step |
| --- | --- | --- | --- |
| `src/mir/join_ir/lowering/loop_patterns/mod.rs` | 21 | Issue/SSOT | Move TODOs to design SSOT, then delete |
| `src/mir/join_ir/lowering/loop_patterns/nested_minimal.rs` | 11 | Issue/SSOT | Same as loop_patterns/mod.rs |
| `src/mir/loop_pattern_detection/legacy/mod.rs` | 6 | Issue (legacy) | Isolate legacy detection + README, then delete TODOs |
| `src/instance_v2.rs` | 5 | Issue | Decide v2 cleanup scope and move TODOs to decision doc |
| `src/bid-codegen-from-copilot/codegen/targets/vm.rs` | 4 | Issue | Decide archive/remove; move TODOs to decision doc |
| `src/runner/modes/common_util/resolve/using_resolution.rs` | 3 | Keep (local) | Leave TODOs with short‑term owner notes |
| `src/mir/loop_pattern_detection/features.rs` | 3 | Issue/SSOT | Move to loop-pattern detection SSOT |
| `src/mir/join_ir_vm_bridge/meta.rs` | 3 | Keep (local) | Leave TODOs for near‑term bridge cleanup |
| `src/tests/vm_functionbox_call.rs` | 2 | Keep (tests) | Leave TODOs, add test owner if missing |
| `src/tests/vm_compare_box.rs` | 2 | Keep (tests) | Leave TODOs, add test owner if missing |
| `src/runner/modes/common_util/resolve/prelude_manager.rs` | 2 | Keep (local) | Leave TODOs for resolver cleanup pass |
| `src/mir/join_ir_vm_bridge/joinir_block_converter/handlers.rs` | 2 | Issue | Move to joinir-bridge handler SSOT |
| `src/mir/join_ir_vm_bridge/handlers/new_box.rs` | 2 | Issue | Move to joinir-bridge handler SSOT |
| `src/mir/join_ir_vm_bridge/handlers/method_call.rs` | 2 | Issue | Move to joinir-bridge handler SSOT |
| `src/mir/join_ir/lowering/simple_while.rs` | 2 | Issue/SSOT | Move to joinir lowering SSOT |
| `src/mir/join_ir/lowering/if_phi_spec.rs` | 2 | Issue/SSOT | Move to joinir lowering SSOT |
| `src/mir/join_ir/lowering/bool_expr_lowerer.rs` | 2 | Issue/SSOT | Move to joinir lowering SSOT |
| `src/mir/join_ir/json.rs` | 2 | Issue/SSOT | Move to joinir JSON contract SSOT |
| `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` | 2 | Issue | Move to joinir merge SSOT |
| `src/boxes/json/mod.rs` | 2 | Issue | Decide JSON Box cleanup/retire, move TODOs |

## Decision rules (apply to each TODO)

- **Keep as TODO**: if it is a short‑term, local fix with a clear owner and does not change contracts.
- **Move to SSOT doc**: if it changes design/contract/acceptance criteria or spans layers.
- **Create issue**: if it’s cross‑cutting, large, or blocked by other phases (include scope + exit criteria).

## Next steps

- Triage top 3 files first (loop pattern lowering/detection).
- For each TODO, record decision in `docs/development/current/main/20-Decisions.md` (SSOT) or open a tracked issue.
- Remove stale TODOs once migrated.
