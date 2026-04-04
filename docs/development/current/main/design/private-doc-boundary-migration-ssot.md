---
Status: SSOT
Scope: 開発予定ドキュメントの public/private 境界を固定し、段階移行で事故を防ぐ
Updated: 2026-02-25
---

# Private Doc Boundary / Migration SSOT

## Goal

- 「どの文書が private 正本か」を明示して迷子を防ぐ。
- 既存の guard/script を壊さずに段階移行する。
- accidental public push を防ぐ。

## Private Root (canonical)

- canonical private root: `docs/private/development/current/main/`
- public は「最小 stub + machine anchor」を維持する。

## Public Anchor (must remain parseable)

以下はスクリプトが機械読取しているため、public 側に残す（stub化可、削除不可）。

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/design/de-rust-lane-map-ssot.md`
4. `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
5. `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
6. `docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`
7. `docs/development/current/main/phases/phase-29y/README.md`

## Script Dependencies (current)

- `tools/selfhost/sync_lane_a_state.sh`
- `tools/checks/phase29bq_joinir_port_sync_guard.sh`
- `tools/checks/phase29y_derust_blocker_sync_guard.sh`
- `tools/archive/legacy-selfhost/engineering/promote_tier2_case.sh`

## Migration Phases (fixed order)

### Phase P0: Boundary Lock (docs-only)

- このSSOTを更新し、public anchor 一覧を固定する。
- private 正本置き場を上記 canonical root に固定する。
- 受け入れ:
  - `bash tools/checks/phase29bq_joinir_port_sync_guard.sh`
  - `bash tools/checks/phase29y_derust_blocker_sync_guard.sh`

### Phase P1: Non-anchor Move (safe)

- machine parse されない予定文書を先に private へ移す。
- public 側には 3-10 行の pointer stub を残す。
- 1コミット=1文書群（同じ責務）で分割する。
- 受け入れ:
  - `tools/checks/dev_gate.sh quick`
  - 上記 2 guard

### Phase P2: Anchor Stubization (careful)

- Public anchor は削除せず、必要行だけ残す stub に縮退する。
- 詳細本文は private 正本へ移す。
- script/guard が読む行は文言を壊さない。
- 受け入れ:
  - `bash tools/selfhost/sync_lane_a_state.sh`
  - `bash tools/checks/phase29bq_joinir_port_sync_guard.sh`
  - `bash tools/checks/phase29y_derust_blocker_sync_guard.sh`
  - `tools/checks/dev_gate.sh quick`

## Naming Rule (visibility)

- private 正本には先頭に `Status: Private Canonical` を書く。
- public stub には先頭に `Status: Public Stub` と `Private Canonical Path:` を書く。

## Rollback

- P1/P2 の各コミットは独立 revert 可能にする（まとめて大移動しない）。
- guard fail 時は直前コミットを revert して P0 状態へ戻す。
