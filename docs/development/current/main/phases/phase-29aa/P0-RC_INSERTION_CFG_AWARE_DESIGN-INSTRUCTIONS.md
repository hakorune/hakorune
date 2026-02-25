# Phase 29aa P0: RC insertion safety expansion — CFG-aware design

**Date**: 2025-12-27  
**Status**: Ready (design-first)  
**Scope**: `src/mir/passes/rc_insertion.rs` を CFG-aware に拡張するための設計。Branch/Jump/PHI/loop/early-exit を安全に扱う前提を固める。  
**SSOT**: `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`

---

## 目的

- Phase 29z の「単一block限定」実装を、誤releaseを起こさない形で次段へ進める。
- “SSA last-use で drop する” を禁止したまま、CFG-aware の設計を確定する。
- 解析→挿入を二段階に分離し、後続の安全条件・Fail-Fast を明文化する。

## 非目的

- 既定挙動変更（feature `rc-insertion-minimal` 以外は no-op 維持）
- SSA last-use ベースの drop
- いきなり全ケース実装

---

## 設計タスク（P0）

### 1) RcPlan のデータ構造案（block/edge）

**目的**: Stage A (分析) が Stage B (挿入) に渡す、最小かつ安全な「release 予定表」を決める。

提案:

```text
RcPlan
  - block_sites: Vec<BlockPlan>
  - edge_sites: Vec<EdgePlan>

BlockPlan
  - block_id: BlockId
  - drops: Vec<DropSite>

EdgePlan
  - from: BlockId
  - to: BlockId
  - drops: Vec<DropSite>

DropSite
  - kind: DropKind  // Overwrite | ExplicitNull | ScopeEnd(ReturnOnly)
  - value_id: ValueId
  - binding_id: BindingId
  - at: DropPoint   // BeforeInstr(index) | BeforeTerminator
  - proof: ProofTag // SafeCurrentValue | ExplicitNull | Overwrite
```

方針:
- **block_sites** は “局所で安全が確定できる release” のみを置く。
- **edge_sites** は “スコープが閉じる edge” の cleanup を載せるが、P1 では Return 専用に限定する。
- Stage B は `DropSite` の `proof` と `kind` を必ず検査し、未対応のものは Fail-Fast する。

### 2) PHI/loop/early-exit の危険パターン（Fail-Fast 方針）

Fail-Fast 原則: **安全条件を証明できない場合は release を挿入しない**。必要なら debug_assert/diagnostic を出す。

危険パターン（例）:
- PHI 入力値を release してしまう（binding の current value と一致しない）
- loop back-edge が絡む binding を loop body 終端で release する
- early-exit（return/break/continue）で閉じるスコープが曖昧なまま cleanup を入れる
- edge ごとにスコープ閉鎖が異なるのに、block 終端で一括 release する

Fail-Fast ポリシー:
- `binding current value` との一致が証明できない DropSite は **計画に載せない**。
- `edge cleanup` は「閉じるスコープが確定した edge」に限定し、未確定なら no-op + 診断（opt-inのみ）。

### 3) 安全に release できる条件（契約）

**契約**: release できるのは “binding が保持している current value” のみ。

許可条件:
- `x = <new>` の直前に、`x` の **旧値** を release（Overwrite）
- `x = null` の直前に、`x` の **旧値** を release（ExplicitNull）
- **Return 終端**で閉じる binding scope の current value を release（ScopeEnd/ReturnOnly）

禁止条件:
- SSA last-use を根拠に release
- PHI の入力値（current value と一致しない限り禁止）
- loop/edge のスコープ閉鎖が確定していない cleanup

### 4) 次P1の最小実装ターゲット（1つだけ）

**P1 でやること**: Return 終端の cleanup を RcPlan 経由で行う（Branch/Jump は引き続き禁止）。

- Stage A: Return 終端の scope end だけを `BlockPlan` に載せる
- Stage B: `DropSite(kind=ScopeEnd(ReturnOnly))` のみ挿入
- Branch/Jump/PHI/loop/early-exit は **no-op + Fail-Fast guard** を維持

---

## ドキュメント更新（必須）

- `docs/development/current/main/phases/phase-29z/README.md` に次フェーズ入口と残課題の分類を追記
- `docs/development/current/main/10-Now.md` の Current Focus を Phase 29aa P0 に更新
- `docs/development/current/main/30-Backlog.md` に Phase 29aa を追加（最小3項目）

---

## 受け入れ基準

- docs-only（コード変更なし）
- quick 154/154 PASS 維持（確認のみ）
- 「安全に release できる条件」が契約として明文化されている
