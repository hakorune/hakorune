# Exit PHI Design - Phase 132 Architecture

## Overview

Phase 132 で完成した Exit PHI アーキテクチャの責務分離設計。

## Three-Layer Responsibility Model

### Layer 1: JoinIR (Frontend - データ生成層)

**責務**: ループ脱出時の変数バインディング情報を生成

**実装箇所**: `src/mir/join_ir/lowering/inline_boundary/`

**主要コンポーネント**:
- `LoopExitBinding`: ループ脱出時の変数バインディング構造
  ```rust
  pub struct LoopExitBinding {
      pub carrier_name: String,       // 変数名 (e.g., "i")
      pub join_exit_value: ValueId,   // JoinIR k_exit 関数の引数
      pub host_slot: ValueId,         // Host MIR の PHI dst
  }
  ```

**データフロー**:
```
Pattern 1 Minimal:
  loop_step → Jump(k_exit, [i_param]) → exit_bindings = [LoopExitBinding { "i", i_param, host_slot }]
```

**Phase 132 貢献**:
- `exit_bindings` フィールド追加（`JoinInlineBoundary` 構造体）
- Pattern 1-5 各パターンで exit binding 生成ロジック実装

---

### Layer 2: Boundary (Middleware - 接続実行層)

**責務**: JoinIR の exit_bindings を使って Host MIR に Exit PHI を接続

**実装箇所**: `src/mir/builder/control_flow/joinir/merge/`

**主要コンポーネント**:
- `ExitLineReconnector`: Exit PHI 接続 Box (Phase 33-10 で箱化)
  ```rust
  impl ExitLineReconnector {
      fn connect_exit_line(
          &self,
          boundary: &JoinInlineBoundary,
          exit_block_id: BasicBlockId,
          exit_predecessor: BasicBlockId,
          builder: &mut Builder,
      ) -> Result<(), String>
  }
  ```

**処理フロー**:
1. `exit_bindings` をイテレート
2. 各 binding について:
   - `host_slot` (PHI dst) に対して
   - `(exit_predecessor, join_exit_value)` を incoming として追加

**Phase 132 貢献**:
- `exit_bindings` を読み取る発火ロジック実装
- Phase 131 の metadata 参照ロジックを完全削除（SSOT化）

---

### Layer 3: LLVM Backend (Execution - PHI保護層)

**責務**: builder.vmap 内の PHI を SSOT として保護・管理

**実装箇所**: `src/llvm_py/`

**主要コンポーネント** (Phase 132-Post):
- `PhiManager` Box: PHI ライフサイクル管理
  ```python
  class PhiManager:
      def register_phi(bid: int, vid: int, phi_value)
      def is_phi_owned(bid: int, vid: int) -> bool
      def filter_vmap_preserve_phis(vmap: dict, target_bid: int) -> dict
      def sync_protect_phis(target_vmap: dict, source_vmap: dict)
  ```

**SSOT Principle**:
- `builder.vmap` の PHI は **Single Source of Truth**
- PHI は絶対に上書きしない
- ブロック間で PHI 所有権を明確に管理

**Phase 132 貢献**:
- `predeclared_ret_phis` dict による PHI ownership tracking
- vmap filtering: ブロック外の PHI を除外
- sync protection: 既存 PHI を上書きしない

**Phase 132-Post 貢献** (Box-First Refactoring):
- `PhiManager` Box 化で PHI 管理ロジック集約
- `filter_vmap_preserve_phis()`: PHI フィルタリングをカプセル化
- `sync_protect_phis()`: PHI 保護ロジックをカプセル化

---

## Data Flow Example (Pattern 1 Minimal)

```
【JoinIR Layer】
  loop_step 関数:
    Jump(k_exit, [i_param], cond=exit_cond)
  ↓
  Pattern 1 lowering:
    exit_bindings = [
      LoopExitBinding {
        carrier_name: "i",
        join_exit_value: ValueId(1003),  // JoinIR i_param
        host_slot: ValueId(3),           // Host MIR PHI dst
      }
    ]

【Boundary Layer】
  ExitLineReconnector::connect_exit_line():
    for binding in exit_bindings:
      builder.add_phi_incoming(
        block: exit_block_id,
        dst: ValueId(3),                  // host_slot
        incoming: (bb6, ValueId(3))       // (exit_pred, remapped join_exit_value)
      )
  ↓
  Host MIR:
    bb3: ValueId(3) = phi [(bb6, ValueId(3))]

【LLVM Layer】
  PhiManager::register_phi(3, 3, phi_value)  // PHI を登録
  ↓
  block_lower.py Pass A (非終端命令処理):
    vmap_cur = PhiManager.filter_vmap_preserve_phis(builder.vmap, 3)
    # → bb3 所有の PHI(3) のみ保持、他ブロックの PHI は除外
  ↓
  Pass A sync:
    PhiManager.sync_protect_phis(builder.vmap, vmap_cur)
    # → builder.vmap の PHI を上書きしない
  ↓
  Pass B (PHI finalization):
    phi_3.add_incoming(val_3, bb6)
  ↓
  Pass C (終端命令処理):
    ret phi_3
```

---

## Design Principles

### 1. **Separation of Concerns**
- **JoinIR**: データ生成のみ（実行なし）
- **Boundary**: 接続実行のみ（データ保護なし）
- **LLVM**: PHI保護・管理のみ（生成なし）

### 2. **Box-First Architecture** (Phase 132-Post)
- ロジックを Box (クラス/メソッド) にカプセル化
- `ExitLineReconnector` Box (Boundary)
- `PhiManager` Box (LLVM)

### 3. **SSOT (Single Source of Truth)**
- `exit_bindings` が変数バインディングの唯一の真実
- `builder.vmap` の PHI が SSA 値の唯一の真実
- metadata 参照の完全排除

### 4. **Fail-Fast**
- エラーは早期に明示的に失敗
- フォールバック処理は禁止
- PHI 上書きは panic

---

## Migration from Phase 131

### Phase 131 (Before)
- ❌ Jump metadata (`jump_args`) から exit PHI を復元
- ❌ Block.metadata 参照の散在
- ❌ PHI 管理ロジックの分散

### Phase 132 (After)
- ✅ `exit_bindings` で明示的データフロー
- ✅ Boundary 層での一元的 PHI 接続
- ✅ metadata 完全削除（Block からも削除）

### Phase 132-Post (Box-First Refactoring)
- ✅ `PhiManager` Box による PHI 管理ロジック集約
- ✅ `filter_vmap_preserve_phis()` でフィルタリング簡潔化
- ✅ `sync_protect_phis()` で保護ロジック再利用可能

---

## Future Work

### Pattern 2-5 への拡張
- Pattern 2 (If-in-Loop): 複数変数 exit bindings
- Pattern 3 (Loop-with-If): exit line 分岐処理
- Pattern 4-5: 複雑な exit 条件

### PhiManager 拡張候補
- PHI タイプヒント管理
- PHI incoming 検証
- PHI 最適化ヒント

---

## References

- Phase 132 実装ログ: `docs/development/current/main/phases/phase-132/`
- Boundary アーキテクチャ: `docs/development/architecture/phase-33-modularization.md`
- JoinIR 全体設計: `docs/development/current/main/joinir-architecture-overview.md`

---

Last Updated: 2025-12-15 (Phase 132-Post Box-First Refactoring)
