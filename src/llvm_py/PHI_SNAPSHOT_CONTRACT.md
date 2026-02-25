# PHI Snapshot Contract - PHI値のSSA有効性契約

## 根本原則

**「PHIはSSA値として他blockでも有効」**

この原則は LLVM の SSA (Static Single Assignment) 形式の根幹であり、
破ってはならない契約です。

## 契約の詳細

### PHI値の有効性

PHI値は以下の条件で有効です：

1. **Defining Block**: PHI値が定義されたblock
2. **Dominated Blocks**: PHI値のdefining blockがdominateする全てのblock

### Snapshot上のPHI

Block終端のsnapshot上でPHI値を参照する際：

- ✅ **正しい**: snapshot miss時もPHI定義値を返す
- ❌ **誤り**: snapshot miss時にPHI値を「未定義」扱い

### 過去の破綻事例

この契約の破綻により以下の問題が発生しました：

#### 事例1: PHI値の消失
```python
# 誤った実装
def _value_at_end_i64(self, value_id, block_id):
    snapshot = self.snapshots.get(block_id, {})
    if value_id not in snapshot:
        return None  # ❌ PHI値もNone扱い
```

**問題**: PHI値がsnapshot missで消失

**修正**:
```python
def _value_at_end_i64(self, value_id, block_id):
    snapshot = self.snapshots.get(block_id, {})
    if value_id not in snapshot:
        # ✅ PHI値はmiss扱いしない
        if self.is_phi(value_id):
            return self.get_phi_definition(value_id)
        return None
```

#### 事例2: SSA不変条件の破綻

PHI値を「未定義」扱いすることで、SSA形式の基本原則が破綻：

- **SSA不変条件**: 値は一度定義されたら変更されない
- **破綻現象**: PHI値が「定義済み」から「未定義」に変化

## 使用方法

### PhiSnapshotPolicyBox の使用

```python
from phi_snapshot_policy import PhiSnapshotPolicyBox

# PHI値の有効性判定
is_valid = PhiSnapshotPolicyBox.is_phi_valid_at(
    phi_id, block_id, dominator_info
)

# Snapshot上でのPHI解決
phi_value = PhiSnapshotPolicyBox.resolve_phi_at_snapshot(
    phi_id, snapshot, resolver
)
```

### Resolver での統合

```python
class Resolver:
    def _value_at_end_i64(self, value_id, block_id):
        snapshot = self.snapshots.get(block_id, {})

        # PhiSnapshotPolicyBox を使用してPHI値を正しく解決
        if PhiSnapshotPolicyBox.is_phi(value_id, self):
            return PhiSnapshotPolicyBox.resolve_phi_at_snapshot(
                value_id, snapshot, self
            )

        # 通常の値の解決
        return snapshot.get(value_id)
```

## Fail-Fast

契約違反は即座にエラー：

- `AssertionError`: PHI値を「未定義」扱い
- `AssertionError`: snapshot miss時にPHI値を無視

これにより、契約違反を早期に検出し、バグの伝播を防ぎます。

## デバッグ

### PHI値の追跡

環境変数 `NYASH_PHI_ORDERING_DEBUG=1` でPHI値の処理を追跡：

```bash
NYASH_PHI_ORDERING_DEBUG=1 ./target/release/hakorune --backend llvm program.hako
```

出力例：
```
[phi_wiring/create] v42 PHI created: phi.basic_block=bb3 expected=bb3
[phi_wiring] WARNING: Attempting to create PHI in bb5 after terminator already exists!
```

## 参考

- **LLVM SSA Form**: https://llvm.org/docs/LangRef.html#ssa-form
- **Dominator Tree**: https://llvm.org/docs/ProgrammersManual.html#dominators
- **Phase 97 Refactoring**: この契約のSSoT化

## 設計原則

### Monotonic Property

PHI値の状態は単調増加（monotonic）：

- 「未定義」→「定義済み」: ✅ 許可
- 「定義済み」→「未定義」: ❌ 禁止

### SSA Invariant

SSA形式の不変条件：

1. **Single Assignment**: 各値は一度だけ定義される
2. **Dominance**: 値の使用はdefining blockにdominateされる
3. **PHI Merge**: PHI命令は複数の定義をmergeする唯一の方法

この契約はSSA Invariantの維持に不可欠です。
