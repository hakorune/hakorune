# Phase 97 Integration Guide - 箱化モジュールの統合手順

## 概要

Phase 97で作成した5つのBox/Policyを既存コードに統合する際の詳細手順です。

## 前提条件

- Phase 97で作成した全Boxが正常にビルド完了していること
- 既存テストが全てPASSしていること

## 統合順序（推奨）

依存関係を考慮した統合順序：

1. TypeFactsBox（基盤）
2. PhiSnapshotPolicyBox（PHI処理）
3. PrintArgMarshallerBox（print処理）
4. CallRoutePolicyBox（Call処理）

PluginErrorContextは既に統合済みのため不要。

---

## 統合1: TypeFactsBox

### 目的
型情報伝播ロジックを一箇所に集約し、stringish taggingの一貫性を保証。

### 変更ファイル

#### 1. `src/llvm_py/resolver.py`

**変更箇所**: LINE 98-119 (`mark_string` メソッド)

**変更前**:
```python
def mark_string(self, value_id: int) -> None:
    try:
        vid = int(value_id)
        self.string_ids.add(vid)
        # TypeFacts SSOT: keep value_types in sync
        try:
            if not hasattr(self, 'value_types') or self.value_types is None:
                self.value_types = {}
            cur = self.value_types.get(vid) if isinstance(self.value_types, dict) else None
            is_already_string = False
            if isinstance(cur, dict):
                if cur.get('kind') == 'string':
                    is_already_string = True
                if cur.get('kind') == 'handle' and cur.get('box_type') == 'StringBox':
                    is_already_string = True
            if not is_already_string and isinstance(self.value_types, dict):
                self.value_types[vid] = {'kind': 'handle', 'box_type': 'StringBox'}
        except Exception:
            pass
    except Exception:
        pass
```

**変更後**:
```python
def mark_string(self, value_id: int) -> None:
    # Phase 97: Use TypeFactsBox
    from type_facts import TypeFactsBox
    try:
        vid = int(value_id)
        # Delegate to TypeFactsBox
        if not hasattr(self, '_type_facts'):
            self._type_facts = TypeFactsBox()
        self._type_facts.mark_string(vid, reason="resolver.mark_string")

        # Backward compatibility: keep string_ids in sync
        self.string_ids.add(vid)

        # Keep value_types in sync for downstream code
        try:
            if not hasattr(self, 'value_types') or self.value_types is None:
                self.value_types = {}
            if isinstance(self.value_types, dict):
                self.value_types[vid] = {'kind': 'handle', 'box_type': 'StringBox'}
        except Exception:
            pass
    except Exception:
        pass
```

**変更箇所**: LINE 121-125 (`is_stringish` メソッド)

**変更後**:
```python
def is_stringish(self, value_id: int) -> bool:
    # Phase 97: Use TypeFactsBox
    try:
        if hasattr(self, '_type_facts'):
            return self._type_facts.is_stringish(int(value_id))
        # Fallback to legacy path
        return int(value_id) in self.string_ids
    except Exception:
        return False
```

#### 2. `src/llvm_py/instructions/copy.py`

**変更箇所**: LINE 52-60 (型伝播処理)

**変更前**:
```python
    # TypeFacts propagation (SSOT): preserve "stringish" tagging across Copy.
    try:
        if resolver is not None and hasattr(resolver, "is_stringish") and resolver.is_stringish(src):
            if hasattr(resolver, "mark_string"):
                resolver.mark_string(dst)
    except Exception:
        pass
```

**変更後**:
```python
    # Phase 97: Use TypeFactsBox for propagation
    from type_facts import TypeFactsBox
    try:
        if resolver is not None and hasattr(resolver, '_type_facts'):
            resolver._type_facts.propagate_copy(dst, src)
        # Fallback to legacy path for backward compatibility
        elif resolver is not None and hasattr(resolver, "is_stringish") and resolver.is_stringish(src):
            if hasattr(resolver, "mark_string"):
                resolver.mark_string(dst)
    except Exception:
        pass
```

#### 3. `src/llvm_py/phi_wiring/wiring.py`

**変更箇所**: LINE 270-286 (PHI型伝播)

**変更前**:
```python
            # TypeFacts propagation (SSOT): if any incoming source is stringish, mark dst stringish.
            try:
                if (
                    hasattr(builder, "resolver")
                    and hasattr(builder.resolver, "is_stringish")
                    and hasattr(builder.resolver, "mark_string")
                ):
                    for (_decl_b, v_src) in (incoming or []):
                        try:
                            if builder.resolver.is_stringish(int(v_src)):
                                builder.resolver.mark_string(int(dst_vid))
                                break
                        except Exception:
                            continue
            except Exception:
                pass
```

**変更後**:
```python
            # Phase 97: Use TypeFactsBox for PHI propagation
            from type_facts import TypeFactsBox
            try:
                if hasattr(builder, "resolver") and hasattr(builder.resolver, '_type_facts'):
                    # Extract incoming value IDs
                    incoming_ids = [int(v_src) for (_decl_b, v_src) in (incoming or [])]
                    builder.resolver._type_facts.propagate_phi(int(dst_vid), incoming_ids)
                # Fallback to legacy path
                elif (
                    hasattr(builder, "resolver")
                    and hasattr(builder.resolver, "is_stringish")
                    and hasattr(builder.resolver, "mark_string")
                ):
                    for (_decl_b, v_src) in (incoming or []):
                        try:
                            if builder.resolver.is_stringish(int(v_src)):
                                builder.resolver.mark_string(int(dst_vid))
                                break
                        except Exception:
                            continue
            except Exception:
                pass
```

### テスト

```bash
# 型伝播のテスト
NYASH_LLVM_TRACE_CALLS=1 ./target/release/hakorune --backend llvm apps/tests/string_ops_basic.hako

# PHI型伝播のテスト
./target/release/hakorune --backend llvm apps/tests/loop_with_string_concat.hako
```

---

## 統合2: PhiSnapshotPolicyBox

### 目的
PHI値のSSA有効性契約を明示化し、snapshot miss時の適切な処理を保証。

### 変更ファイル

#### `src/llvm_py/resolver.py`

**新規メソッド追加**: `_value_at_end_i64` の前に以下を追加

```python
def is_phi(self, value_id: int) -> bool:
    """Check if value_id is a PHI value

    Phase 97: Helper for PhiSnapshotPolicyBox
    """
    try:
        # Check if value is in block_phi_incomings
        for block_id, dst_map in (self.block_phi_incomings or {}).items():
            if int(value_id) in dst_map:
                return True
        return False
    except Exception:
        return False

def get_phi_definition(self, value_id: int):
    """Get PHI definition value

    Phase 97: Helper for PhiSnapshotPolicyBox
    """
    try:
        # Try to get from vmap first
        if hasattr(self, 'global_vmap') and self.global_vmap:
            return self.global_vmap.get(int(value_id))
        # Try cache
        for cache in [self.i64_cache, self.ptr_cache, self.f64_cache]:
            for (_, vid), val in cache.items():
                if vid == int(value_id):
                    return val
        return None
    except Exception:
        return None
```

**変更箇所**: `_value_at_end_i64` メソッド（存在する場合）

**変更後**:
```python
def _value_at_end_i64(self, value_id, block_id):
    """Resolve value at end of block

    Phase 97: Use PhiSnapshotPolicyBox for PHI handling
    """
    from phi_snapshot_policy import PhiSnapshotPolicyBox

    snapshot = self.block_end_values.get(block_id, {})

    # Phase 97: Check if this is a PHI value
    if PhiSnapshotPolicyBox.is_phi(value_id, self):
        return PhiSnapshotPolicyBox.resolve_phi_at_snapshot(
            value_id, snapshot, self
        )

    # Regular value resolution
    return snapshot.get(value_id)
```

### テスト

```bash
# PHI処理のテスト
NYASH_PHI_ORDERING_DEBUG=1 ./target/release/hakorune --backend llvm apps/tests/loop_min_while.hako

# PHI snapshotのテスト
./target/release/hakorune --backend llvm apps/tests/if_phi_sum.hako
```

---

## 統合3: PrintArgMarshallerBox

### 目的
print引数のmarshal処理を統一し、FFI境界の契約を明示化。

### 変更ファイル

#### `src/llvm_py/instructions/mir_call/global_call.py`

**変更箇所**: LINE 84-120 (print引数の型変換)

**変更前**:
```python
        # Type conversion for function signature matching
        if i < len(func.args):
            expected_type = func.args[i].type
            if expected_type.is_pointer and isinstance(arg_val.type, ir.IntType):
                # Convert i64 to i8* for C ABI-style functions (print/panic/error).
                # ... (長い処理)
```

**変更後**:
```python
        # Type conversion for function signature matching
        if i < len(func.args):
            expected_type = func.args[i].type
            if expected_type.is_pointer and isinstance(arg_val.type, ir.IntType):
                # Phase 97: Use PrintArgMarshallerBox for print marshal
                if func_name == "print":
                    from instructions.mir_call.print_marshal import PrintArgMarshallerBox
                    try:
                        is_stringish = False
                        if resolver is not None and hasattr(resolver, "is_stringish"):
                            is_stringish = resolver.is_stringish(int(arg_id))

                        type_info = {"stringish": is_stringish}
                        arg_val = PrintArgMarshallerBox.marshal(
                            arg_id, type_info, builder, resolver, module
                        )
                    except Exception as e:
                        # Fallback to legacy path
                        pass
                else:
                    # Non-print functions: legacy path
                    if arg_val.type.width == 64:
                        # ... (既存の処理)
```

### テスト

```bash
# print処理のテスト
./target/release/hakorune --backend llvm apps/tests/peek_expr_block.hako

# 型変換のテスト
./target/release/hakorune --backend llvm apps/tests/print_integer.hako
```

---

## 統合4: CallRoutePolicyBox

### 目的
Call種別判定を統一し、ルーティングロジックを一箇所に集約。

### 変更ファイル

#### `src/llvm_py/instructions/mir_call/__init__.py`

**変更箇所**: LINE 115-134 (Method call routing)

**変更前**:
```python
    elif callee_type == "Method":
        # Box method call
        method = callee.get("name")
        box_name = callee.get("box_name")
        receiver = callee.get("receiver")
        certainty = callee.get("certainty")

        # SSOT: Method calls split into two routes:
        # - Static method (receiver=null, certainty=Known): lower as direct function call
        # - Instance method (receiver omitted in v1 JSON): receiver is implicit as first arg
        if receiver is None:
            if certainty == "Known" and box_name and method:
                func_name = f"{box_name}.{method}/{len(args)}"
                lower_global_call(builder, owner.module, func_name, args, dst_vid, vmap, resolver, owner)
                return
            if args:
                receiver = args[0]
                args = args[1:]  # Remove receiver from args
```

**変更後**:
```python
    elif callee_type == "Method":
        # Phase 97: Use CallRoutePolicyBox for routing
        from instructions.mir_call.route_policy import CallRoutePolicyBox, CallKind

        method = callee.get("name")
        box_name = callee.get("box_name")
        receiver = callee.get("receiver")
        certainty = callee.get("certainty")

        # Construct callee string for routing decision
        if receiver is None and certainty == "Known" and box_name and method:
            callee_str = f"{box_name}.{method}"
            ctx = {"builtin_boxes": ["StringBox", "IntegerBox", "BoolBox", "ArrayBox", "MapBox"]}

            try:
                decision = CallRoutePolicyBox.decide(callee_str, ctx)

                if decision.kind == CallKind.STATIC_METHOD and decision.is_direct_call:
                    # Direct static method call
                    func_name = f"{box_name}.{method}/{len(args)}"
                    lower_global_call(builder, owner.module, func_name, args, dst_vid, vmap, resolver, owner)
                    return
            except ValueError:
                # Fallback to instance method
                pass

        # Instance method path
        if receiver is None and args:
            receiver = args[0]
            args = args[1:]  # Remove receiver from args
```

### テスト

```bash
# Call routingのテスト
NYASH_LLVM_TRACE_CALLS=1 ./target/release/hakorune --backend llvm apps/tests/string_ops_basic.hako

# Static method callのテスト
./target/release/hakorune --backend llvm apps/tests/static_method_call.hako
```

---

## 回帰テスト

### 必須テスト

すべての統合後、以下のテストを実施：

```bash
# 1. Python module compilation
python3 -m py_compile src/llvm_py/**/*.py

# 2. Rust build
cargo build --release

# 3. Smoke tests
tools/smokes/v2/run.sh --profile integration

# 4. 個別機能テスト
./target/release/hakorune --backend llvm apps/tests/string_ops_basic.hako
./target/release/hakorune --backend llvm apps/tests/loop_min_while.hako
./target/release/hakorune --backend llvm apps/tests/if_phi_sum.hako
./target/release/hakorune --backend llvm apps/tests/peek_expr_block.hako

# 5. Phase 97 specific tests
NYASH_LLVM_TRACE_CALLS=1 ./target/release/hakorune --backend llvm apps/tests/call_routing_test.hako
NYASH_PHI_ORDERING_DEBUG=1 ./target/release/hakorune --backend llvm apps/tests/phi_snapshot_test.hako
```

---

## トラブルシューティング

### 問題: import error

**症状**: `ImportError: No module named 'type_facts'`

**解決**:
```python
# 相対importに変更
from ..type_facts import TypeFactsBox
```

### 問題: PHI値が未定義

**症状**: `AssertionError: Cannot resolve PHI value`

**解決**:
- `is_phi()` と `get_phi_definition()` の実装を確認
- PHI値が正しく `block_phi_incomings` に登録されているか確認

### 問題: 型伝播が動作しない

**症状**: stringish tagが伝播しない

**解決**:
- `_type_facts` が正しく初期化されているか確認
- `propagate_copy()` / `propagate_phi()` が呼ばれているか確認
- デバッグログで追跡: `reason` フィールドを確認

---

## まとめ

Phase 97の統合は、以下の手順で段階的に実施します：

1. TypeFactsBox統合（基盤）
2. PhiSnapshotPolicyBox統合（PHI処理）
3. PrintArgMarshallerBox統合（print処理）
4. CallRoutePolicyBox統合（Call処理）

各統合後は必ず回帰テストを実施し、挙動不変を確認してから次の統合に進んでください。
