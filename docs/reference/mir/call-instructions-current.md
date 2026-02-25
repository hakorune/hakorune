# MIR Call 命令の現状（補助資料）

Status: Supplemental (Call design snapshot)
SSOT: `docs/reference/mir/INSTRUCTION_SET.md`

## 現在の方針（RCL-3-min3）

- `MirInstruction::BoxCall` は retired。
- `MirInstruction::ExternCall` は retired。
- callsite は `MirInstruction::Call` に一本化し、呼び先種別は `Callee` で表す。

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
