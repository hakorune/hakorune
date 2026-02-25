# Phase 29y P0: Docs finalize（docs-first, 大きく進める）

**Date**: 2025-12-27  
**Status**: Ready（next）  
**Scope**: Phase 29y の SSOT（ABI/RC insertion/observability）を “次フェーズへ切れる形” で締める。実装の追加はしない（Phase 29y は docs-first）。  
**Non-goals**: MIR 命令追加、GC/finalizer 新規実装、NyRT の .hako 化、既定挙動変更

---

## 目的（SSOT）

Phase 29y を “Draft のメモ” ではなく、後続の実装フェーズへ迷わず移れる SSOT にする。

- ABI SSOT: `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`
- RC insertion SSOT: `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`
- Observability SSOT: `docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md`

---

## 手順（docs-first）

### Step 1: README を “実体に同期” する

`docs/development/current/main/phases/phase-29y/README.md` を更新:
- Status を `Draft` → `In progress`（or `Ready`）へ
- 3つのSSOT（10/20/30）の役割を “1段落ずつ” 明文化
- Phase 29y.1（pilot 実装）が既にあることを “実ファイルパス” で列挙

### Step 2: 10/20/30 の cross-link を揃える

各SSOT文書で、最低限これを満たす:
- “用語” の定義（borrowed/owned, retain/release, weak identity）
- “契約” が 1箇所に書かれている（分散しない）
- 参照先のコード/スモークが実在する（リンク切れ無し）

### Step 3: Pilot の入口を固定（実装は触らない）

README に以下を追記して、後続が迷わないようにする:
- ABI shim: `crates/nyash_kernel/src/ffi/lifecycle.rs`
- RC insertion skeleton: `src/mir/passes/rc_insertion.rs`
- Leak report: `src/runtime/leak_tracker.rs`
- Integration smokes: `tools/smokes/v2/profiles/integration/apps/phase29y_handle_abi_{vm,llvm}.sh`
- Fixture: `apps/tests/phase29y_handle_abi.hako`

### Step 4: “次に何を実装するか” を 3つまでに絞る

Phase 29y は docs-first のため、次フェーズ（Phase 29x/29z など）へ切るための “実装タスク” を最大3つに絞って README に書く。

例:
1. RC insertion pass を no-op から最小動作へ（保持・解放の1ケースだけ）
2. ABI borrowed/owned の conformance smoke を 1本追加
3. Observability の root categories を 1段追加（handles 以外の最小）

---

## 検証

docs 更新後に最低限:

```bash
git status --porcelain=v1
```

任意（安心）:
```bash
cargo check -p nyash-rust --lib
./tools/smokes/v2/run.sh --profile quick
```
