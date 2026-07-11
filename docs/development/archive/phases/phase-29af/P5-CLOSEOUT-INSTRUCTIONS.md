# Phase 29af P5: Closeout — Instructions

Status: Ready for execution  
Scope: Phase 29af の成果（boundary hygiene / regression entrypoint / layout SSOT / consistency contract）を締める（仕様不変）

## Goal

Phase 29af (P0–P4) の SSOT と contract を “入口が迷わない状態” に確定し、JoinIR 側の回帰確認導線を固定する。

## Non-goals

- 挙動変更（release 既定挙動の変更）
- env var の追加
- fixture/smoke の増加（回帰パックは既存を使う）

## Deliverables

1) Phase 29af README を “Complete” にする
- `docs/development/current/main/phases/phase-29af/README.md`
  - Status: P0–P4 complete
  - Next: none（または次フェーズへのリンク）
  - Verification は `phase29ae_regression_pack_vm.sh` 1本に収束

2) Now/Backlog を 29af 完了へ更新
- `docs/development/current/main/10-Now.md`
  - Current Focus: Phase 29af complete
  - JoinIR 回帰確認のSSOT: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `docs/development/current/main/30-Backlog.md`
  - Phase 29af: ✅ COMPLETE
  - 次の候補（例）を 1 行で明記（Phase 29ag など）

3) Command SSOT の二重化を解消
- `docs/development/current/main/phases/phase-29ae/README.md`
  - Commands: entrypoint script 1本のみ

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- JoinIR 回帰確認が “1コマンド” で PASS
- quick が 154/154 PASS（不変）
- 29af の入口ドキュメントだけ見れば “どこを見れば良いか” が分かる
