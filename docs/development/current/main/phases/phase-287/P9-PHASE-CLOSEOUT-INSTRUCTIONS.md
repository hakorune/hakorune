# Phase 287 P9: Closeout（docs-only, 意味論不変）

**Date**: 2025-12-27  
**Status**: Ready（next）  
**Scope**: Phase 287 の完了状態を docs と index で確定し、次フェーズへ迷わず移れるように入口を締める（コード変更なし）。  
**Non-goals**: コード変更、テスト追加、CI/スモーク構成変更

---

## 目的

- Phase 287 の “Done” を SSOT に反映し、Now/Backlog の参照先を次へ切り替える。
- 入口文書が古い “Next (P4/P5...)” を指さない状態にする。

---

## 手順

1. `docs/development/current/main/phases/phase-287/README.md`
   - `**Status**: Complete`（または `Completed`）へ変更
   - P0–P8 を “Done” として列挙し、Next を撤去（または “Next: none”）
2. `docs/development/current/main/10-Now.md`
   - Current Focus を次フェーズ（または次タスク）へ更新
3. `docs/development/current/main/30-Backlog.md`
   - Phase 287 を “completed” へ移す（active から外す）

---

## 検証

docs-only なので、最低限:

```bash
git status --porcelain=v1
```

（任意）手元の安心:
```bash
cargo check -p nyash-rust --lib
./tools/smokes/v2/run.sh --profile quick
```
