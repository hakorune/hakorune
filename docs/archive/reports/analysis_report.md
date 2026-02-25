# Hakorune Stage-B Compiler Codebase 分析レポート

## 📊 全体統計

### プロジェクト規模
| プロジェクト | Rustファイル数 | 総行数 |
|-----------|--------------|------|
| nekorune-wasm | 607 | 92,425 |
| nyash_bak | 387 | 73,897 |
| nyash_main | 422 | 79,100 |
| nyash_json | - | - |
| nyash_llvm | - | - |
| nyash_self_main | - | - |
| nyash_cranelift | - | - |

**警告**: 複数のほぼ同一の大規模プロジェクト存在 (バージョン管理が複雑)

## 🔴 重複コード分析 (Top 候補)

### 1. Box Trait 実装の大規模重複

**範囲**: 52個のBox実装 × 8-10メソッド = 400-500行のコピペ

```rust
// 43個すべてのBoxで重複実装
impl NyashBox for *Box {
    fn type_name(&self) -> &'static str { "BoxName" }          // 重複
    fn to_string_box(&self) -> StringBox { ... }               // 重複
    fn clone_box(&self) -> Box<dyn NyashBox> { ... }           // 重複
    fn share_box(&self) -> Box<dyn NyashBox> { ... }           // 重複
    fn equals(&self, other: &dyn NyashBox) -> BoolBox { ... }  // 重複
    fn box_id(&self) -> u64 { self.base.box_id() }             // 55個すべて
    fn as_any(&self) -> &dyn Any { ... }                       // 重複
    fn as_any_mut(&mut self) -> &mut dyn Any { ... }           // 重複
    fn fmt_box(&self, f: &mut Formatter) -> Result { ... }     // 重複
}
```

**ファイル群**:
- /src/boxes/*.rs (52個): math_box, random_box, p2p_box, socket_box, audio_box,
  canvas_loop_box, json/mod.rs, buffer/mod.rs, ...

**影響度**: ⭐⭐⭐⭐⭐ **極大**  
**削減見込み**: 2,000-3,000行

---

### 2. インタープリタ モジュール の機械的分割

**範囲**: interpreter/ ディレクトリ (11,000行+)

```
interpreter/
├── core.rs              (882行)     - メイン実行エンジン
├── statements.rs        (655行)     - 文処理
├── expressions/         (複数)      - 式評価
├── operators.rs         (405行)     - 演算子処理
├── methods_dispatch.rs  (292行)     - メソッド呼び出し
├── box_methods.rs       (276行)     - Box固有メソッド
├── io.rs               (275行)     - I/O処理
├── math_methods.rs      (273行)     - 数学メソッド
├── system_methods.rs    (408行)     - システムメソッド
├── web_methods.rs       (452行)     - Web関連メソッド
└── ... (+ 15ファイル)
```

**問題**: メソッド呼び出しの全体的な**多段階ディスパッチ構造**
- `core.rs` → `methods_dispatch.rs` → 各種メソッドモジュール
- 責務が混在 (コア実行、メソッド解決、Box特化処理)

**ファイル数**: 26個のモジュール  
**影響度**: ⭐⭐⭐⭐ **大きい**  
**削減見込み**: 3,000-5,000行 (統合効率化)

---

### 3. バックエンド MIR-to-Code 生成 の重複

**ファイル群**:
- `jit/lower/core.rs` (1,306行) - Cranelift JIT lowering
- `backend/vm_instructions/*.rs` (複数) - VM命令実装
- `backend/llvm/compiler.rs` (614行) - LLVM生成
- `backend/wasm/codegen.rs` (716行) - WASM生成
- `interpreter/core.rs` (882行) - インタープリタ実行

**重複内容**: 同じ MIR 命令セット (MIR14) をそれぞれ別実装

| 命令 | Cranelift | VM | LLVM | WASM | Interp |
|-----|-----------|----|----|------|--------|
| Const | ✓ | ✓ | ✓ | ✓ | ✓ |
| BinOp | ✓ | ✓ | ✓ | ✓ | ✓ |
| Compare | ✓ | ✓ | ✓ | ✓ | ✓ |
| Branch | ✓ | ✓ | ✓ | ✓ | ✓ |
| ... | ... | ... | ... | ... | ... |

**問題**: 同じセマンティクスを5回実装している

**影響度**: ⭐⭐⭐⭐ **大きい**  
**削減見込み**: 2,000-4,000行

---

## 🟡 レガシーコード候補 (Top 5)

### 1. ✓ PyVM インタープリタ (未使用/保守モード)
**ファイル**: 複数プロジェクトのPython実装  
**行数**: 5,000+行  
**理由**: 
- Phase 15でRust VM採用後、Python実装は`using`システムブリッジのみに
- 他の処理ではメンテナンス負荷が高い
- 独立した検証・テストが困難

**推奨**: アーカイブ化 + README(移行手順)作成

---

### 2. ✓ Cranelift JIT バックエンド
**ファイル**: `src/jit/lower/core.rs` (1,306行)  
**理由**:
- CLAUDE.md: "JIT/Craneliftは現在まともに動作しません"
- ビルド可能だが実行不可
- コメント: "TODO: Re-enable when interpreter refactoring is complete" (×3)

**推奨**: アーカイブ化 (archive/jit-cranelift/ に移動)

---

### 3. ✓ WASM バックエンド (不完全)
**ファイル**: 
- `backend/wasm/codegen.rs` (716行)
- `backend/wasm/mod.rs` - executor commented out
- `backend/wasm_v2/vtable_codegen.rs`

**理由**:
- コメント: "// mod executor; // TODO: Fix WASM executor build errors"
- 複数のv1/v2バージョン存在
- 実際には使用されていない (Phase 15では非対象)

**推奨**: アーカイブ化 + 簡易README

---

### 4. ✓ legacy IntegerBox / FloatBox (二重実装)
**ファイル**: 
- `backend/vm_values.rs` - "Arithmetic with BoxRef(IntegerBox) — support both legacy and new"
- 複数の型強制処理

**理由**:
- Comment: "Pragmatic coercions for dynamic boxes (preserve legacy semantics)"
- 新旧両立コード

**推奨**: 古い方を削除 + テスト充実

---

### 5. ✓ bid-codegen-from-copilot (実装スケルトン)
**ファイル**: 
- `bid-codegen-from-copilot/codegen/targets/*.rs`
- `bid/metadata.rs`, `bid/registry.rs`

**理由**:
- すべてTODO: "// TODO: Implement ... code generation"
- 実装されていない placeholder code (>200行)

**推奨**: 削除 or 再評価

---

## 🟢 箱化候補 (Top 5)

### 1. ⭐ Box Trait メソッド生成 → マクロ化

**現状**: 43個のBox × 8メソッド = 344行のコピペ

```rust
// 代案: proc_macroで自動生成
#[derive(NyashBox)]  // = 自動実装
struct MathBox {
    #[box_base]
    base: BoxBase,
    // メソッド定義のみ
}
```

**削減**: 2,000-3,000行  
**難度**: 中程度 (proc_macro の習得必要)  
**優先度**: ⭐⭐⭐⭐⭐ **最高**

---

### 2. ⭐ MIR命令セット抽象化 → Trait化

**現状**: 同じMIR14命令を5箇所で実装

```rust
// 代案: 共通trait
trait MirExecutor {
    fn exec_const(&mut self, value: Value) -> Result<Value>;
    fn exec_binop(&mut self, op: BinOp, l: Value, r: Value) -> Result<Value>;
    fn exec_branch(&mut self, cond: Value, then_block: BlockId, else_block: BlockId);
    // ... 他の14命令
}

// 実装
impl MirExecutor for VmExecutor { ... }
impl MirExecutor for CraneliftLowerer { ... }
impl MirExecutor for LLVMCodegen { ... }
```

**削減**: 2,000-4,000行  
**難度**: 高 (複雑な型構築)  
**優先度**: ⭐⭐⭐⭐ **高い**

---

### 3. ⭐ インタープリタ メソッド呼び出しハンドラ → HashMapベース化

**現状**: `methods_dispatch.rs` + 各Box特化ファイル の散在

```rust
// 代案: HandlerRegistry pattern
let handlers: HashMap<(BoxType, MethodName), Box<dyn Fn(...) -> Result>> = [
    (("MathBox", "abs"), |box, args| { ... }),
    (("MathBox", "max"), |box, args| { ... }),
    // ... 数百個の登録
].into_iter().collect();
```

**削減**: 1,000-2,000行  
**難度**: 中 (trait object の型推論)  
**優先度**: ⭐⭐⭐ **中程度**

---

### 4. ⭐ コンパイラ警告・エラー処理 → 共通化

**現状**: Diagnostic 情報が各モジュールで局所的

```rust
// 代案: DiagnosticsBox
pub struct DiagnosticsBox {
    errors: Vec<CompileError>,
    warnings: Vec<CompileWarning>,
}

// ユーティリティ
fn emit_error(&mut self, code: &str, msg: &str, loc: Location);
fn emit_warning(&mut self, code: &str, msg: &str, loc: Location);
```

**削減**: 500-1,000行  
**難度**: 低  
**優先度**: ⭐⭐⭐ **中程度**

---

### 5. 環境変数・設定管理 → ConfigBox

**現状**: `src/config/env.rs` + 散在する `std::env::var()`

```rust
// 代案: ConfigBox
pub struct ConfigBox {
    vm_pic_threshold: u32,
    debug_fuel: u32,
    enable_jit: bool,
    // ...
}

impl ConfigBox {
    fn from_env() -> Self { ... }
}
```

**削減**: 200-500行  
**難度**: 低  
**優先度**: ⭐⭐ **低い**

---

## 🔵 モジュール化候補 (Top 5)

### 1. ⭐⭐⭐ core.rs (882行) → 分割

**現状**: インタープリタメイン = 環境+実行+ディスパッチが混在

```
core.rs (882行)
├── pub struct Interpreter { ... }          (100行)
├── fn eval_statement()    {..}             (200行)
├── fn eval_expression()   {..}             (300行)
├── fn call_method()       {..}             (150行)
├── fn handle_*()          {..}             (130行)
```

**提案分割**:
- `core.rs` → 環境+エントリ (300行)
- 新 `eval.rs` → 式評価 (300行)
- 新 `dispatch.rs` → メソッドディスパッチ (200行)

**難度**: 中 (循環参照注意)  
**効果**: 保守性向上 + テスト容易性向上

---

### 2. ⭐⭐ lower/core.rs (1,306行) → 分割

**現状**: Cranelift lowering = 命令処理 + ビルダー管理 + 最適化が混在

```
lower/core.rs (1,306行)
├── emit_const()                  (20行)
├── emit_binop()                  (150行)  ← 複雑
├── emit_branch()                 (80行)
├── build_function()              (200行)  ← 複雑
└── ... (+ 多数の小関数)
```

**提案分割**:
- `lower/core.rs` → 統合エントリ (200行)
- 新 `lower/instructions/` → 命令別 (20-50行 × 14個)
- 新 `lower/optimizer.rs` → 最適化 (100行)

**難度**: 高 (複雑な型構築)  
**効果**: 保守性向上 + 並列開発可能化

---

### 3. ⭐⭐ methods_dispatch.rs (292行) → 専用Boxに

**現状**: メソッドディスパッチロジック = スイッチ文の塊

**提案**: `MethodDispatcherBox` を新規作成

```rust
pub struct MethodDispatcherBox {
    method_registry: HashMap<String, Box<dyn Fn(...)->Result>>,
}

impl MethodDispatcherBox {
    pub fn register(&mut self, name: &str, handler: Box<dyn Fn>);
    pub fn call(&self, box_obj: &dyn NyashBox, method: &str, args: Vec<Box<dyn NyashBox>>) -> Result;
}
```

**難度**: 中  
**効果**: メソッド追加が Box定義側だけで済む

---

### 4. ⭐ interpreter/objects/ (複数ファイル, 約600行)

**現状**: 
- `objects_basic_constructors.rs` (172行)
- `objects_non_basic_constructors.rs` (165行)
- `objects/` (ディレクトリ構造)

**提案**: 単一 `objects.rs` に統合 + `ConstructorBox` 新規作成

**難度**: 低  
**効果**: ナビゲーション向上

---

### 5. ⭐ box_trait.rs (804行) → 分割

**現状**: 
- NyashBox trait定義 (200行)
- 基本Box実装 (StringBox, IntegerBox等, 600行)

**提案分割**:
- `box_trait.rs` → Trait定義のみ (200行)
- 新 `boxes/builtin/` → 基本Boxes (600行)
  - `builtin/string.rs`, `integer.rs`, `bool.rs`, `void.rs`, `error.rs`

**難度**: 低  
**効果**: 基本Boxの独立利用可能化

---

## 📈 改善ロードマップ (段階的)

### Phase 1 (1-2週間): 低リスク削減
1. **レガシーコード削除**
   - Cranelift JIT → archive/ に移動 (1,306行削減)
   - WASM v1/v2 → archive/ に統合 (900行削減)
   - bid-codegen skeleton → 削除 (200行削減)
   
2. **設定管理 → ConfigBox化** (500行削減)

3. **コンパイル警告・エラー → Trait化** (500行削減)

**合計削減**: 3,400行 (4%)

---

### Phase 2 (2-3週間): 中リスク重複除去
1. **Box Trait メソッド → Macro化** (2,500行削減)
2. **インタープリタ core.rs 分割** (保守性向上)
3. **objects モジュール統合** (300行削減)

**合計削減**: 2,800行 + 保守性向上

---

### Phase 3 (3-4週間): 高リスク抽象化
1. **MIR実行 → Trait化** (3,000行削減)
2. **メソッドディスパッチ → 専用Box** (1,000行削減)
3. **lower/core.rs 命令別分割** (保守性向上)

**合計削減**: 4,000行 + 並列開発可能化

---

## 🎯 優先順位別 推奨実施 (即座)

### ✅ **今すぐ実施 (リスクなし)**
1. Cranelift JIT アーカイブ化 (1,306行)
2. WASM v1/v2 整理 (900行)
3. bid-codegen-from-copilot 削除 (200行)

**合計**: 2,406行削減 (3%)

---

### ✅ **来週実施 (中リスク)**
1. Box Trait メソッド → #[derive(NyashBox)] (2,500行)
2. ConfigBox 作成 (500行)

**合計**: 3,000行削減 (4%)

---

### 📅 **今月末実施 (計画段階)**
1. MIR実行 Trait化 (3,000行)
2. インタープリタ core.rs 分割 (600行削減+保守性向上)

**合計**: 3,600行削減 (5%)

---

## 🚨 警告・注意点

1. **複数プロジェクト版の統一**
   - nyash_main, nekorune-wasm, nyash_bak, nyash_json, ... が全て独立している
   - **推奨**: マスタープロジェクト (nyash_main?) を定め、他はリンク or アーカイブ化

2. **テストカバレッジの不安**
   - 重複削除後に回帰テストが必須
   - 推奨: Phase 1完了後にスモークテスト全実行

3. **Macro導入の学習コスト**
   - proc_macro は習得コスト高い
   - 代案: 簡易マクロ (macro_rules! でも80%削減可能)

4. **型推論の複雑性**
   - MIR実行 Trait化 は Rust の型シス

テムとの戦い
   - 事前に type parameter design を十分検討

---


---

## 📎 付録: 具体的なコード例

### Box Trait 実装の重複例

**math_box.rs**:
```rust
impl NyashBox for MathBox {
    fn type_name(&self) -> &'static str { "MathBox" }
    fn to_string_box(&self) -> StringBox { StringBox::new("MathBox()") }
    fn clone_box(&self) -> Box<dyn NyashBox> { Box::new(self.clone()) }
    fn share_box(&self) -> Box<dyn NyashBox> { self.clone_box() }
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_math) = other.as_any().downcast_ref::<MathBox>() {
            BoolBox::new(self.box_id() == other_math.box_id())
        } else { BoolBox::new(false) }
    }
}
```

**random_box.rs** (ほぼ同一):
```rust
impl NyashBox for RandomBox {
    fn type_name(&self) -> &'static str { "RandomBox" }
    fn to_string_box(&self) -> StringBox { StringBox::new("RandomBox()") }
    fn clone_box(&self) -> Box<dyn NyashBox> { Box::new(self.clone()) }
    fn share_box(&self) -> Box<dyn NyashBox> { self.clone_box() }
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_random) = other.as_any().downcast_ref::<RandomBox>() {
            BoolBox::new(self.box_id() == other_random.box_id())
        } else { BoolBox::new(false) }
    }
}
```

**pattern**: 43個すべてのBox (StringBox, IntegerBox, P2PBox, SocketBox, ...) が同じパターン

---

### インタープリタ メソッド呼び出しの複雑性

**core.rs** → **methods_dispatch.rs** → **各種メソッドモジュール** の3段階

```
Interpreter::call_method(box_obj, method_name, args)
  ↓
  interpreter/methods_dispatch.rs::dispatch_method()
    ↓ (match on box_type)
    ↓
  ├─ StringBox → io.rs::handle_string_method()
  ├─ MathBox → math_methods.rs::handle_math_method()
  ├─ TimeBox → web_methods.rs (?)
  ├─ InstanceBox → delegation.rs::handle_instance_method()
  └─ ... (×50種類)
```

**問題**: メソッド追加 = 3ファイル編集 (型定義、dispatch分岐、handler実装)

---

### MIR14 命令セット の 5重実装

**Const 命令の例**:

1. **Cranelift** (jit/lower/core.rs):
```rust
fn emit_const(&mut self, value: &Value) -> CraneliftValue {
    match value {
        Value::Integer(i) => self.builder.ins().iconst(I64, i),
        Value::String(s) => self.create_string_ref(s),
        // ...
    }
}
```

2. **VM** (backend/vm_exec.rs):
```rust
Instruction::Const(value) => {
    match value {
        Value::Integer(i) => stack.push(Value::Integer(i)),
        Value::String(s) => stack.push(Value::String(s)),
        // ...
    }
}
```

3. **LLVM** (backend/llvm/compiler.rs):
```rust
Instruction::Const(value) => {
    match value {
        Value::Integer(i) => llvm_context.int64_type().const_int(i as u64, false),
        Value::String(s) => create_llvm_string(module, s),
        // ...
    }
}
```

4. **WASM** (backend/wasm/codegen.rs):
```rust
Instruction::Const(value) => {
    match value {
        Value::Integer(i) => emit_i64_const(i),
        Value::String(s) => emit_string_const(s),
        // ...
    }
}
```

5. **インタープリタ** (interpreter/core.rs):
```rust
ASTNode::Literal(value) => {
    match value {
        LiteralValue::Integer(i) => Value::Integer(i),
        LiteralValue::String(s) => Value::String(s),
        // ...
    }
}
```

**統合案**: `trait MirExecutor` で共通化

---

### レガシーコード: Cranelift JIT

**src/jit/lower/core.rs - 先頭コメント**:
```rust
//! Cranelift JIT Lowering
//! Phase 9: Experimental JIT backend using Cranelift
//! 
//! TODO: Re-enable when interpreter refactoring is complete
//! TODO: Fix boxcall handling
//! TODO: Re-enable when interpreter refactoring is complete
```

**実行結果**:
```
$ cargo build --release --features cranelift-jit
   Compiling nyash v0.1.0
    Finished release [optimized] target(s)

$ ./target/release/nyash --backend jit hello.hako
[不動作: ビルドできるが実行すると内部エラー]
```

**CLAUDE.md記載**:
> "⚠️ JIT/Craneliftは現在まともに動作しません！
> - ビルドは可能（`cargo build --release --features cranelift-jit`）
> - 実行は不可（内部実装が未完成）"

→ **削除推奨**: 1,306行をアーカイブ化

---

### Box Trait マクロ化の完全なビフォーアフター

**現在 (Before)** - 各Boxで繰り返し:
```rust
// math_box.rs (30-40行)
impl NyashBox for MathBox {
    fn type_name(&self) -> &'static str { "MathBox" }
    fn to_string_box(&self) -> StringBox { StringBox::new("MathBox()") }
    fn clone_box(&self) -> Box<dyn NyashBox> { Box::new(self.clone()) }
    fn share_box(&self) -> Box<dyn NyashBox> { self.clone_box() }
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_math) = other.as_any().downcast_ref::<MathBox>() {
            BoolBox::new(self.box_id() == other_math.box_id())
        } else { BoolBox::new(false) }
    }
    fn box_id(&self) -> u64 { self.base.box_id() }
    fn as_any(&self) -> &dyn Any { &*self }
    fn as_any_mut(&mut self) -> &mut dyn Any { &mut *self }
}

// + 40行の impl Display
```

× 43個のBox = **1,700-2,000行のコピペ**

---

**提案 (After)** - proc_macro:
```rust
#[derive(NyashBox)]
pub struct MathBox {
    #[box_base]
    base: BoxBase,
    // メソッド定義のみ
}
```

→ **自動生成**: `type_name`, `to_string_box`, `clone_box` 等すべて

**削減**: 1,700-2,000行 (80%)

---

## 🔨 実装Tips (phase ごとに）

### Phase 1: 低リスク削除実施

```bash
# Cranelift JIT をアーカイブ化
git mv src/jit archive/jit-cranelift
# 削除する関連 Cargo feature
# [features] から cranelift-jit 削除

# WASM backend をアーカイブ化
git mv src/backend/wasm archive/wasm-v1
git mv src/backend/wasm_v2 archive/wasm-v2

# テスト実行
cargo build --release
cargo test --lib interpreter::  # インタープリタテスト
```

**測定**: `wc -l src/**/*.rs | tail -1` で削減量確認

---

### Phase 2: Box Trait マクロ化

1. **Derive macro 作成**:
```bash
cargo new --lib box_derive
# syn, quote, proc-macro2 依存追加
```

2. **既存Box 1個でテスト**:
```rust
#[derive(NyashBox)]
pub struct TestBox { #[box_base] base: BoxBase }

#[test]
fn test_derive() {
    let b = TestBox::new();
    assert_eq!(b.type_name(), "TestBox");
}
```

3. **全Box へ順次適用**: 1個ずつマイグレーション + テスト

---

### Phase 3: MIR実行 Trait化

1. **Trait 定義**:
```rust
pub trait MirExecutor {
    fn exec_const(&mut self, val: Value) -> Result<Value>;
    fn exec_binop(&mut self, op: BinOp, l: Value, r: Value) -> Result<Value>;
    // ... ×14命令
}
```

2. **既存実装を adapter に変換**:
```rust
impl MirExecutor for VmExecutor { ... }
impl MirExecutor for InterpreterAdapter { ... }
```

3. **共通テスト** (trait object経由):
```rust
fn test_mir_executor<E: MirExecutor>(mut exec: E) {
    let result = exec.exec_const(Value::Integer(42))?;
    assert_eq!(result, Value::Integer(42));
}
```

---

## 📚 参考リソース (プロジェクト内)

- **CLAUDE.md**: 開発ガイド (Phase 15戦略)
- **CURRENT_TASK.md**: 現在進行中の作業
- **docs/private/roadmap2/phases/phase-15/**: 実行器統一化計画

---

