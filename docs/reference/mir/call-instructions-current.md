# MIR Call 命令の現状（補助資料）

Status: Supplemental (Call design snapshot)
SSOT: `docs/reference/mir/INSTRUCTION_SET.md`

## 現在の方針（RCL-3-min3）

- `MirInstruction::BoxCall` は retired。
- `MirInstruction::ExternCall` は retired。
- callsite は `MirInstruction::Call` に一本化し、呼び先種別は `Callee` で表す。

## JSON-v1 Constructor ingress boundary

V1 の typed `Constructor` は、valid shape（`name` または `box_type` の一方、
`args` 配列、`dst`）を既存の `NewBox` owner へ渡す。`name` と `box_type` を
同時に指定する場合は同一値でなければならない。args の欠落・非配列・flat と
nested の同時指定、および alias conflict は、MIR block へ `NewBox` を追加する
前に typed reject する。これは wire-shape の fail-fast 契約であり、constructor
宣言の source/arity を解決する仕様ではない。AST `New` の source/arity handoff と
R6 の core field cutover は別の設計停止に属する。

## Canonical 形

```rust
Call {
    dst: Option<ValueId>,
    func: ValueId,           // legacy fallback slot（canonical では INVALID 推奨）
    callee: Option<Callee>,  // Some(...) を canonical とする
    args: Vec<ValueId>,
    effects: EffectMask,
}
```

## Callee

```rust
pub enum Callee {
    Global(String),
    Method {
        box_name: String,
        method: String,
        receiver: Option<ValueId>,
        certainty: TypeCertainty,
        box_kind: CalleeBoxKind,
    },
    Constructor { box_type: String },
    Closure { ... },
    Value(ValueId),
    Extern(String),
}
```

## マッピング

- 旧 `BoxCall { box_val, method, args, dst }`
  - → `Call { callee: Some(Callee::Method { receiver: Some(box_val), method, ... }), args, dst }`
- 旧 `ExternCall { iface_name, method_name, args, dst }`
  - → `Call { callee: Some(Callee::Extern(format!("{iface}.{method}"))), args, dst }`
