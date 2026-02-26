---
Status: Active
Decision: provisional
Date: 2026-02-17
Scope: app-first 方針で `.hako` アプリを外部AIに実装依頼するための固定指示書。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-runtime-meaning-decision-red-inventory-ssot.md
  - docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md
---

# APP-1 Outsource Instructions (Gate Log Summarizer)

## 0. Goal

- `.hako` だけで動く最小CLIアプリを 1 本追加する。
- 目的は「app-first で開発を進めつつ、no-compat mainline を維持できる」ことを証明すること。

## 1. Task (fixed)

- アプリ名: `gate-log-summarizer`
- 機能:
  - 入力ログ（テキスト）を 1 ファイル受け取る
  - `PASS / FAIL / SKIP` を集計する
  - FAIL 行だけを抽出して末尾に表示する

## 2. Required Deliverables

1. App source:
   - `apps/tools/gate_log_summarizer/main.hako`
2. App README:
   - `apps/tools/gate_log_summarizer/README.md`
3. Fixture logs:
   - `apps/tests/gate_log_summarizer/sample_mixed.log`
4. Smoke:
   - `tools/smokes/v2/profiles/integration/apps/archive/gate_log_summarizer_vm.sh`

## 3. Output Contract (stable)

出力は次の順序・表記を固定すること（余計な行を出さない）。

1. `SUMMARY pass=<n> fail=<n> skip=<n>`
2. `FAIL_LINES <m>`
3. `m` 件の FAIL 行（入力ログの生行を先頭から順に）

例:

```text
SUMMARY pass=5 fail=2 skip=1
FAIL_LINES 2
[FAIL] phase29y_handle_abi_borrowed_owned_vm: rc=1
[FAIL] phase29y_lane_gate_vm: contract mismatch
```

## 4. Rules (must)

- Rust 側コードは変更しない（`src/**` 禁止）。
- fallback を新規追加しない。
- silent pass を作らない（入力ファイル不正時は非0終了）。
- 1タスク1目的。ついでの cleanup を混ぜない。

## 5. Acceptance Commands

実装AIは次を実行して結果を添えること。

1. `bash tools/smokes/v2/profiles/integration/apps/archive/gate_log_summarizer_vm.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`

## 6. Reviewer Checklist

- 追加ファイルが `apps/**` と `tools/smokes/**` に閉じている
- 出力フォーマットが contract と一致する
- FAIL 行抽出の順序が入力順で安定している
- acceptance 3コマンドが PASS

## 7. Copy-Paste Request (for external AI)

```text
Implement APP-1 from:
docs/development/current/main/phases/phase-29y/70-APP-FIRST-OUTSOURCE-INSTRUCTIONS.md

Must follow exactly:
- Do not modify Rust files under src/**
- Add only the required deliverables
- Keep output contract stable
- Run and report acceptance commands (3)
```
