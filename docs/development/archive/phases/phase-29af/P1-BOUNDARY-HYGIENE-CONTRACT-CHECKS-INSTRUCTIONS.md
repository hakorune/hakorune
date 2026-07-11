# Phase 29af P1: Boundary Hygiene Contract Checks — Instructions

Status: Ready for execution  
Scope: JoinIR merge の contract_checks に boundary hygiene を集約（仕様不変）

## Goal

Phase 29af P0 で確定した boundary hygiene（LoopBreak route、historical label 2）を、merge の `contract_checks` 側でも検証できる形に収束する。

- upstream（LoopBreak lowerer 側）だけに Fail-Fast が散らばらないようにする
- future pattern / future refactor で boundary 構築の歪みが再発したときに、merge 入口で検知できるようにする

## Non-goals

- 挙動変更（release ビルド既定挙動の変更）
- env var の追加
- fixture/smoke の増加（必要性が明確になった場合のみ）

## Contract (SSOT)

SSOT: `docs/development/current/main/phases/phase-29af/README.md`

- Exit reconnection (`boundary.exit_bindings`) は LoopState のみ
- Header PHI の対象は `boundary.carrier_info` の carriers（LoopState + ConditionOnly + LoopLocalZero）
- `CarrierInit::FromHost` の `host_id=0` は契約違反（Fail-Fast）
- `exit_bindings.carrier_name` の重複は禁止（Fail-Fast）

## Implementation Steps

1) **contract_checks に boundary hygiene チェックを追加**
   - 追加ファイル案: `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_hygiene.rs`
   - 関数案:
     - `verify_boundary_hygiene(boundary: &JoinInlineBoundary) -> Result<(), String>`
   - 検証項目:
     - `exit_bindings` の `role` が全て LoopState であること
     - `exit_bindings` の `carrier_name` が重複しないこと
     - `carrier_info` がある場合:
       - `CarrierInit::FromHost` の carrier は `host_id != ValueId(0)` であること
       - `exit_bindings` の carrier が `carrier_info` と整合すること（最低限: 同名が存在すること）

2) **merge 入口のチェックに配線**
   - `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_creation.rs`
     - boundary の概要ログの直後（Fail-Fast 位置）で `verify_boundary_hygiene()` を呼ぶ
   - `src/mir/builder/control_flow/joinir/merge/contract_checks/mod.rs`
     - module 宣言と re-export を追加

3) **docs を更新（入口と責務の明文化）**
   - `docs/development/current/main/phases/phase-29af/README.md`
     - P1: contract_checks に集約した旨を追記
   - `docs/development/current/main/10-Now.md`
     - Phase 29af P1 の進捗（Started/Complete）に反映
   - `docs/development/current/main/30-Backlog.md`
     - Phase 29af のステータスを更新

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `bash tools/smokes/v2/profiles/integration/joinir/loop_break_release_adopt_vm.sh`
- `bash tools/smokes/v2/profiles/integration/joinir/nested_loop_minimal_strict_shadow_vm.sh`

## Acceptance Criteria

- 既存 smokes（quick / `loop_break_release_adopt_vm.sh` / `nested_loop_minimal_strict_shadow_vm.sh`）が PASS のまま
- release ビルドでの挙動は不変（Fail-Fast は debug/strict のみ）
- boundary hygiene の契約が “merge 入口の SSOT（contract_checks）” でも検証される
