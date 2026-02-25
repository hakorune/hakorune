# 🏆 Nyash Golden Dump Testing System

*ChatGPT5推奨・MIR互換テスト（回帰検出）完全仕様*

## 🎯 目的

**「同じ入力→同じ出力」をinterp/vm/wasm/aot間で保証する自動検証システム**

MIR仕様の揺れ・バックエンド差異・最適化バグを**即座検出**し、Portability Contract v0を技術的に保証。

## 🔧 **Golden Dump方式**

### **基本原理**
```bash
# 1. MIR「黄金標準」生成
nyash --dump-mir program.hako > program.golden.mir

# 2. 実行時MIR比較（回帰検出）
nyash --dump-mir program.hako > program.current.mir
diff program.golden.mir program.current.mir

# 3. 全バックエンド出力比較（互換検証）
nyash --target interp program.hako > interp.out
nyash --target vm program.hako > vm.out
nyash --target wasm program.hako > wasm.out
diff interp.out vm.out && diff vm.out wasm.out
```

### **階層化検証戦略**
| レベル | 検証対象 | 目的 | 頻度 |
|--------|----------|------|------|
| **L1: MIR構造** | AST→MIR変換 | 回帰検出 | 毎commit |
| **L2: 実行結果** | stdout/stderr | 互換性 | 毎PR |
| **L3: 最適化効果** | 性能・メモリ | 最適化回帰 | 毎週 |
| **L4: エラー処理** | 例外・エラー | 堅牢性 | 毎リリース |

## 🧪 **検証テストスイート**

### **1️⃣ MIR Structure Tests (L1)**

#### **基本構造検証**
```rust
// tests/golden_dump/mir_structure_tests.rs
#[test]
fn test_basic_arithmetic_mir_stability() {
    let source = r#"
        static box Main {
            main() {
                local a, b, result
                a = 42
                b = 8
                result = a + b
                print(result)
                return result
            }
        }
    "#;
    
    let golden_mir = load_golden_mir("basic_arithmetic.mir");
    let current_mir = compile_to_mir(source);
    
    assert_eq!(golden_mir, current_mir, "MIR回帰検出");
}

#[test]
fn test_box_operations_mir_stability() {
    let source = r#"
        box DataBox {
            init { value }
            pack(val) { me.value = val }
        }
        
        static box Main {
            main() {
                local obj = new DataBox(100)
                print(obj.value)
            }
        }
    "#;
    
    let golden_mir = load_golden_mir("box_operations.mir");
    let current_mir = compile_to_mir(source);
    
    assert_mir_equivalent(golden_mir, current_mir);
}

#[test]
fn test_weak_reference_mir_stability() {
    let source = r#"
        box Parent { init { child_weak } }
        box Child { init { data } }

        static box Main {
            main() {
                local parent = new Parent()
                local child = new Child(42)
                parent.child_weak = weak(child)

                local c = parent.child_weak.weak_to_strong()
                if c != null {
                    print(c.data)
                }
            }
        }
    "#;

    verify_mir_golden("weak_reference", source);
}
```

#### **MIR比較アルゴリズム**
```rust
// src/testing/mir_comparison.rs
pub fn assert_mir_equivalent(golden: &MirModule, current: &MirModule) {
    // 1. 関数数・名前一致
    assert_eq!(golden.functions.len(), current.functions.len());
    
    for (name, golden_func) in &golden.functions {
        let current_func = current.functions.get(name)
            .expect(&format!("関数{}が見つからない", name));
        
        // 2. 基本ブロック構造一致
        assert_eq!(golden_func.blocks.len(), current_func.blocks.len());
        
        // 3. 命令列意味的等価性（ValueId正規化）
        let golden_normalized = normalize_value_ids(golden_func);
        let current_normalized = normalize_value_ids(current_func);
        assert_eq!(golden_normalized, current_normalized);
    }
}

fn normalize_value_ids(func: &MirFunction) -> MirFunction {
    // ValueIdを連番に正規化（%0, %1, %2...）
    // 意味的に同じ命令列を確実に比較可能にする
}
```

### **2️⃣ Cross-Backend Output Tests (L2)**

#### **標準出力一致検証**
```rust
// tests/golden_dump/output_compatibility_tests.rs
#[test]
fn test_cross_backend_arithmetic_output() {
    let program = "arithmetic_test.hako";
    
    let interp_output = run_backend("interp", program);
    let vm_output = run_backend("vm", program);
    let wasm_output = run_backend("wasm", program);
    
    assert_eq!(interp_output.stdout, vm_output.stdout);
    assert_eq!(vm_output.stdout, wasm_output.stdout);
    assert_eq!(interp_output.exit_code, vm_output.exit_code);
    assert_eq!(vm_output.exit_code, wasm_output.exit_code);
}

#[test]
fn test_cross_backend_object_lifecycle() {
    let program = "object_lifecycle_test.hako";
    
    let results = run_all_backends(program);
    
    // fini()順序・タイミングが全バックエンドで同一
    let finalization_orders: Vec<_> = results.iter()
        .map(|r| &r.finalization_order)
        .collect();
    
    assert!(finalization_orders.windows(2).all(|w| w[0] == w[1]));
}

#[test]
fn test_cross_backend_weak_reference_behavior() {
    let program = "weak_reference_test.hako";
    
    let results = run_all_backends(program);
    
    // weak参照の生存チェック・null化が同一タイミング
    let weak_behaviors: Vec<_> = results.iter()
        .map(|r| &r.weak_reference_timeline)
        .collect();
    
    assert_all_equivalent(weak_behaviors);
}
```

#### **エラー処理一致検証**
```rust
#[test]
fn test_cross_backend_error_handling() {
    let error_programs = [
        "null_dereference.hako",
        "division_by_zero.hako", 
        "weak_reference_after_fini.hako",
        "infinite_recursion.hako"
    ];
    
    for program in &error_programs {
        let results = run_all_backends(program);
        
        // エラー種別・メッセージが全バックエンドで同一
        let error_types: Vec<_> = results.iter()
            .map(|r| &r.error_type)
            .collect();
        assert_all_equivalent(error_types);
    }
}
```

### **3️⃣ Optimization Effect Tests (L3)**

#### **Bus-elision検証**
```rust
// tests/golden_dump/optimization_tests.rs
#[test]
fn test_bus_elision_output_equivalence() {
    let program = "bus_communication_test.hako";
    
    let elision_on = run_with_flag(program, "--elide-bus");
    let elision_off = run_with_flag(program, "--no-elide-bus");
    
    // 出力は同一・性能は差がある
    assert_eq!(elision_on.stdout, elision_off.stdout);
    assert!(elision_on.execution_time < elision_off.execution_time);
}

#[test]
fn test_pure_function_optimization_equivalence() {
    let program = "pure_function_optimization.hako";
    
    let optimized = run_with_flag(program, "--optimize");
    let reference = run_with_flag(program, "--no-optimize");
    
    // 最適化ON/OFFで結果同一
    assert_eq!(optimized.output, reference.output);
    
    // PURE関数の呼び出し回数が最適化で削減
    assert!(optimized.pure_function_calls <= reference.pure_function_calls);
}

#[test]
fn test_memory_layout_compatibility() {
    let program = "memory_intensive_test.hako";
    
    let results = run_all_backends(program);
    
    // Box構造・フィールドアクセスが全バックエンドで同一結果
    let memory_access_patterns: Vec<_> = results.iter()
        .map(|r| &r.memory_access_log)
        .collect();
    
    assert_memory_semantics_equivalent(memory_access_patterns);
}
```

#### **性能回帰検証**
```rust
#[test]
fn test_performance_regression() {
    let benchmarks = [
        "arithmetic_heavy.hako",
        "object_creation_heavy.hako", 
        "weak_reference_heavy.hako"
    ];
    
    for benchmark in &benchmarks {
        let golden_perf = load_golden_performance(benchmark);
        let current_perf = measure_current_performance(benchmark);
        
        // 性能が大幅に劣化していないことを確認
        let regression_threshold = 1.2; // 20%まで許容
        assert!(current_perf.execution_time <= golden_perf.execution_time * regression_threshold);
        assert!(current_perf.memory_usage <= golden_perf.memory_usage * regression_threshold);
    }
}
```

## 🤖 **自動化CI/CD統合**

### **GitHub Actions設定**
```yaml
# .github/workflows/golden_dump_testing.yml
name: Golden Dump Testing

on: [push, pull_request]

jobs:
  mir-stability:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Run MIR Structure Tests (L1)
        run: |
          cargo test --test mir_structure_tests
          
      - name: Verify MIR Golden Dumps
        run: |
          ./scripts/verify_mir_golden_dumps.sh
          
  cross-backend-compatibility:
    runs-on: ubuntu-latest
    needs: mir-stability
    steps:
      - name: Run Cross-Backend Tests (L2)
        run: |
          cargo test --test output_compatibility_tests
          
      - name: Verify All Backend Output Equality
        run: |
          ./scripts/verify_backend_compatibility.sh
          
  optimization-regression:
    runs-on: ubuntu-latest
    needs: cross-backend-compatibility
    steps:
      - name: Run Optimization Tests (L3)
        run: |
          cargo test --test optimization_tests
          
      - name: Performance Regression Check
        run: |
          ./scripts/check_performance_regression.sh
```

### **自動Golden Dump更新**
```bash
#!/bin/bash
# scripts/update_golden_dumps.sh

echo "🏆 Golden Dump更新中..."

# 1. 現在のMIRを新しい黄金標準として設定
for test_file in tests/golden_dump/programs/*.hako; do
    program_name=$(basename "$test_file" .hako)
    echo "更新中: $program_name"
    
    # MIR golden dump更新
    ./target/release/nyash --dump-mir "$test_file" > "tests/golden_dump/mir/${program_name}.golden.mir"
    
    # 出力 golden dump更新  
    ./target/release/nyash --target interp "$test_file" > "tests/golden_dump/output/${program_name}.golden.out"
done

echo "✅ Golden Dump更新完了"

# 2. 更新を確認するためのテスト実行
cargo test --test golden_dump_tests

if [ $? -eq 0 ]; then
    echo "🎉 新しいGolden Dumpでテスト成功"
else
    echo "❌ 新しいGolden Dumpでテスト失敗"
    exit 1
fi
```

## 📊 **実装優先順位**

### **Phase 8.4（緊急）**
- [ ] **L1実装**: MIR構造検証・基本golden dump
- [ ] **基本自動化**: CI/CDでのMIR回帰検出
- [ ] **Bus命令テスト**: elision ON/OFF検証基盤

### **Phase 8.5（短期）** 
- [ ] **L2実装**: 全バックエンド出力一致検証
- [ ] **エラー処理**: 例外・エラーケース検証
- [ ] **性能基準**: ベンチマーク回帰検出

### **Phase 9+（中長期）**
- [ ] **L3-L4実装**: 最適化・堅牢性検証
- [ ] **高度自動化**: 自動修復・性能トレンド分析
- [ ] **形式検証**: 数学的正当性証明

## 🎯 **期待効果**

### **品質保証**
- **回帰即座検出**: MIR仕様変更のバグを即座発見
- **バックエンド信頼性**: 全実行環境で同一動作保証
- **最適化安全性**: 高速化による動作変更防止

### **開発効率**
- **自動品質確認**: 手動テスト不要・CI/CDで自動化
- **リファクタリング安全性**: 大規模変更の影響範囲特定
- **新機能信頼性**: 追加機能が既存動作に影響しない保証

### **Nyash言語価値**
- **エンタープライズ品質**: 厳密な品質保証プロセス
- **技術的差別化**: 「全バックエンド互換保証」の実証
- **拡張性基盤**: 新バックエンド追加時の品質維持

---

## 📚 **関連ドキュメント**

- **MIRリファレンス**: [mir-reference.md](mir-reference.md)
- **互換性契約**: [portability-contract.md](portability-contract.md)
- **ベンチマークシステム**: [../../../benchmarks/README.md](../../../benchmarks/README.md)
- **CI/CD設定**: [../../../.github/workflows/](../../../.github/workflows/)

---

*最終更新: 2025-08-14 - ChatGPT5推奨3点セット完成*

*Golden Dump Testing = Nyash品質保証の技術的基盤*
