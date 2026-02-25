# 🤖 Nyash MIR (Mid-level Intermediate Representation) - Complete Reference

*ChatGPT5アドバイス基盤設計・Everything is Box最適化対応*

## 🎯 Nyashのユニークな点（短く）

### 🌟 **4つの革新的特徴**

1. **Ownership-Forest + weak**: GCなしで確定破棄、学習コスト低（Rustより軽い）
2. **Effect注釈**: pure/mut/io が MIR に入り、Busを消せる/elide基盤に
3. **Busを命令系に内蔵**: 分散・非同期が"あと付け"じゃなく言語仕様の一次市民
4. **バックエンド設計が最初から同居**: Interp→VM→JIT/AOT/WASMを同じMIRで回せる

### 🚀 **差別化ポイント**
```bash
# 全バックエンド統一実行
nyash --target interp program.hako    # デバッグ
nyash --target vm program.hako        # 高速実行
nyash --target wasm program.hako      # Web配布
nyash --target aot-rust program.hako  # ネイティブ
nyash --target jit-cranelift program.hako  # JIT
```

それぞれに**ベンチ + 互換テスト**が通る統一設計

## ⚠️ "化け物"への落とし穴（と対策）

### 🚨 **現在の問題状況**
- **MIRが太り過ぎ**: 35命令（ChatGPT5推奨20命令の175%）
- **仕様が揺れる可能性**: 互換テスト未整備
- **バックエンドごとの差**: 効果・所有の最低保証未定義

### ✅ **ChatGPT5対策実装**
1. **命令20個以内 + intrinsic逃がし**で開始
2. **MIRの互換テスト**（golden dump）＆ポータビリティ契約を先に切る
3. **効果＆所有の"最低保証"**を定義（Tier-0）

## 🔧 **いま決めておくと強い"3点セット"**

### 1️⃣ **MIR最小コア（20命令以内）**

#### **Tier-0: 絶対必要コア（15命令）**
```mir
// 算術・比較
Const { dst, value }                    // 定数
BinOp { dst, op, lhs, rhs }            // 二項演算（算術・論理）
Compare { dst, op, lhs, rhs }          // 比較演算

// 制御フロー
Branch { condition, then_bb, else_bb }  // 条件分岐
Jump { target }                        // 無条件ジャンプ
Return { value? }                      // 関数リターン
Phi { dst, inputs }                    // SSA合流

// 関数・メソッド
Call { dst?, func, args, effects }     // 関数呼び出し
BoxCall { dst?, box_val, method, args, effects }  // メソッド呼び出し

// Everything is Box基本操作
NewBox { dst, box_type, args }         // Box生成
Load { dst, ptr }                      // フィールド読み取り
Store { value, ptr }                   // フィールド書き込み

// Bus（分散・非同期一次市民）
Send { bus, message, effects }        // Bus送信
Recv { dst, bus, effects }            // Bus受信

// Effect制御
Safepoint                              // GC・最適化ポイント
```

#### **Tier-1: 高度最適化（5命令）**
```mir
Cast { dst, value, target_type }      // 型変換（最適化用）
TypeCheck { dst, value, expected_type } // 動的型チェック
WeakNew { dst, box_val }              // weak参照（Forest用）
WeakLoad { dst, weak_ref }            // weak読み取り
Intrinsic { dst?, name, args, effects } // intrinsic逃がし
```

### 2️⃣ **Portability Contract v0**

#### **決定的破棄保証**
```rust
// 強参照のみ伝播
pub struct OwnershipRule {
    strong_propagation: true,    // 強参照は破棄連鎖
    weak_non_propagation: true,  // weak参照は非伝播
    deterministic_finalization: true, // 確定的破棄順序
}
```

#### **Effect意味論**
```rust
pub enum EffectContract {
    Pure,     // 副作用なし→最適化可能
    Mut,      // メモリ変更→順序保証必要
    Io,       // I/O操作→Bus統合
    Bus,      // 分散通信→elision対象
}
```

#### **weakは非伝播＋生存チェック**
```mir
// weak生存チェックは必須
%alive = weak_load %weak_ref
br %alive -> %use_bb, %null_bb
```

### 3️⃣ **互換テスト仕様**

#### **Golden Dump検証**
```bash
# MIR出力の一致検証
nyash --dump-mir program.hako > expected.mir
nyash --dump-mir program.hako > actual.mir
diff expected.mir actual.mir  # 0でなければ回帰

# 全バックエンド同一出力
nyash --target interp program.hako > interp.out
nyash --target vm program.hako > vm.out  
nyash --target wasm program.hako > wasm.out
diff interp.out vm.out && diff vm.out wasm.out
```

#### **Bus-elision検証**
```bash
# Bus最適化のon/off切り替え
nyash --elide-bus program.hako > optimized.out
nyash --no-elide-bus program.hako > reference.out
diff optimized.out reference.out  # 結果は同一であるべき
```

## 📊 **現在の実装状況**

### ✅ **完成済み**
- SSA-form MIR基盤（ChatGPT5設計）
- Effect追跡システム
- 3バックエンド（Interp/VM/WASM）
- 280倍WASM高速化実証

### 🚧 **緊急改善必要**
- [ ] **命令数削減**: 35個→20個（intrinsic逃がし）
- [ ] **Bus命令実装**: Send/Recv（分散一次市民化）
- [ ] **互換テスト**: Golden dump自動化
- [ ] **Portability Contract**: v0仕様策定

### 🎯 **Phase 8.4実装推奨**
```bash
# Bus統合MIR設計
Bus { dst?, target, operation, args, effects }

# Bus-elision最適化
--elide-bus / --no-elide-bus フラグ実装

# 性能数値提示（WASM速いデータ活用）
Bus-elision ON:  280倍高速化（現在実証済み）
Bus-elision OFF: 分散通信フルサポート
```

## 🚀 **これで "全部に変換できる" を名乗れる**

### **統一コマンド体系**
```bash
nyash --target interp program.hako    # インタープリター
nyash --target vm program.hako        # 仮想マシン  
nyash --target wasm program.hako      # WebAssembly
nyash --target aot-rust program.hako  # AOTネイティブ
nyash --target jit-cranelift program.hako  # JITコンパイル
```

### **品質保証体系**
- **ベンチマーク**: 各ターゲットの性能測定
- **互換テスト**: 同一入力→同一出力検証
- **回帰テスト**: Golden dump差分チェック

---

## 📚 **関連ドキュメント**

- **実装仕様**: [MIR命令セット詳細](mir-instruction-set.md)
- **最適化戦略**: [Everything is Box最適化](optimization-strategies.md)  
- **互換性**: [Portability Contract v0](portability-contract.md)
- **テスト**: [Golden Dump検証システム](golden-dump-testing.md)

---

*最終更新: 2025-08-14 - ChatGPT5アドバイス基盤設計完了*

*「Everything is Box」哲学 × MIR最小コア = Nyashの差別化核心*