# 🚀 Getting Started with Nyash - Practical Guide

**最終更新: 2025年8月8日**

## 🎯 5分でNyashを理解する

Nyashは「Everything is Box」哲学に基づく、シンプルで強力なプログラミング言語です。
このガイドでは、実際にコードを書きながらNyashの機能を学んでいきます。

## ⚡ クイックスタート

### **1. 環境構築**
```bash
# リポジトリのクローン
git clone [repository-url]
cd nyash/nyash-rust

# ビルド
cargo build

# 実行
./target/debug/nyash your_program.hako
```

### **2. はじめてのNyashプログラム**
`hello.hako`を作成：
```nyash
print("Hello, Nyash World!")
print("Everything is Box! 🎉")
```

実行：
```bash
./target/debug/nyash hello.hako
```

出力：
```
Hello, Nyash World!
Everything is Box! 🎉
```

## 📚 基本構文チュートリアル

### **Step 1: 変数と初期化**
```nyash
# 🎯 新機能：初期化付き変数宣言
local name = "Alice"
local age = 25
local height = 165.5
local isStudent = true

print("Name: " + name)
print("Age: " + age)
print("Height: " + height)
print("Student: " + isStudent)

# 複数変数の同時宣言・初期化
local x = 10, y = 20, z = 30
print("Sum: " + (x + y + z))  # 60

# 混合宣言（初期化あり・なし）
local initialized = 42, uninitialized, another = "test"
uninitialized = "assigned later"
print("Values: " + initialized + ", " + uninitialized + ", " + another)
```

### **Step 2: 演算子の使用**
```nyash
local a = 10
local b = 3

# 算術演算子
print("Addition: " + (a + b))       # 13
print("Subtraction: " + (a - b))    # 7
print("Multiplication: " + (a * b)) # 30
print("Division: " + (a / b))       # 3.3333333333333335

# 論理演算子（自然言語ライク）
local hasPermission = true
local isLoggedIn = true
local canAccess = hasPermission and isLoggedIn
print("Can access: " + canAccess)   # true

local isDenied = not canAccess
print("Is denied: " + isDenied)     # false

# 比較演算子
print("a > b: " + (a > b))          # true
print("a == b: " + (a == b))        # false
```

### **Step 3: 制御構造**
```nyash
function testControlFlow() {
    local score = 85
    
    # if文
    if score >= 90 {
        print("Grade: A")
    } else if score >= 80 {
        print("Grade: B")  # これが実行される
    } else {
        print("Grade: C or below")
    }
    
    # ループ（統一構文）
    local count = 0
    loop(count < 3) {
        print("Count: " + count)
        count = count + 1
        if count == 2 {
            print("Breaking at 2")
            break
        }
    }
}

testControlFlow()
```

### **Step 4: Box（クラス）の定義**
```nyash
box Person {
    init { name, age, email }  # フィールド定義（カンマ必須！）
    
    # コンストラクタ（引数サポート）
    Person(n, a, e) {
        me.name = n
        me.age = a  
        me.email = e
        print("Person created: " + me.name)
    }
    
    # メソッド
    introduce() {
        print("Hi, I'm " + me.name + ", age " + me.age)
    }
    
    getInfo() {
        return me.name + " (" + me.age + ") - " + me.email
    }
    
    # デストラクタ
    fini() {
        print("Person destroyed: " + me.name)
    }
}

# 使用例
person = new Person("Bob", 30, "bob@example.com")
person.introduce()
print("Info: " + person.getInfo())
```

## 🏭 実践例：Calculator アプリ

完全なCalculatorアプリを実装：

```nyash
# 📱 Calculator App - Nyash版

box Calculator {
    init { history }
    
    Calculator() {
        me.history = new ArrayBox()
        print("🧮 Calculator initialized!")
    }
    
    add(a, b) {
        local result = a + b
        me.addToHistory("ADD", a, b, result)
        return result
    }
    
    subtract(a, b) {
        local result = a - b
        me.addToHistory("SUB", a, b, result)
        return result
    }
    
    multiply(a, b) {
        local result = a * b
        me.addToHistory("MUL", a, b, result)
        return result
    }
    
    divide(a, b) {
        if b == 0 {
            print("❌ Error: Division by zero!")
            return 0
        }
        local result = a / b
        me.addToHistory("DIV", a, b, result)
        return result
    }
    
    addToHistory(op, a, b, result) {
        local record = op + ": " + a + " " + op + " " + b + " = " + result
        me.history.push(record)
    }
    
    showHistory() {
        print("📊 Calculation History:")
        local size = me.history.size()
        local i = 0
        loop(i < size) {
            print("  " + (i + 1) + ". " + me.history.get(i))
            i = i + 1
        }
    }
    
    clear() {
        me.history = new ArrayBox()
        print("🧹 History cleared!")
    }
}

# ✨ Calculator使用例
calc = new Calculator()

print("=== Basic Operations ===")
print("10 + 5 = " + calc.add(10, 5))
print("10 - 3 = " + calc.subtract(10, 3))
print("4 * 7 = " + calc.multiply(4, 7))
print("15 / 3 = " + calc.divide(15, 3))
print("10 / 0 = " + calc.divide(10, 0))  # ゼロ除算エラーテスト

print("")
calc.showHistory()

print("")
print("=== Complex Calculations ===")
local complex1 = calc.add(calc.multiply(3, 4), calc.divide(20, 4))
print("(3 * 4) + (20 / 4) = " + complex1)

calc.showHistory()
```

## 📊 VM性能計測（オプション）

VMバックエンドの命令統計を有効化すると、性能分析に役立つ集計が得られます。

```bash
# 人間向け表示
nyash --backend vm --vm-stats hello.hako

# JSON出力（ツール連携向け）
nyash --backend vm --vm-stats --vm-stats-json hello.hako
```

環境変数での制御も可能です（`NYASH_VM_STATS`, `NYASH_VM_STATS_JSON`）。

## ⚡ 並行処理の実践

```nyash
# 🚀 Parallel Processing Example

function heavyComputation(iterations) {
    print("⚙️  Starting computation with " + iterations + " iterations...")
    
    local sum = 0
    local i = 0
    loop(i < iterations) {
        sum = sum + (i * i)
        i = i + 1
        
        # 進捗表示（1000回毎）
        if (i % 1000) == 0 {
            print("  Progress: " + i + "/" + iterations)
        }
    }
    
    print("✅ Computation completed: " + sum)
    return sum
}

function parallelDemo() {
    print("🚀 Starting parallel computations...")
    
    # 3つのタスクを並行実行
    future1 = nowait heavyComputation(5000)
    future2 = nowait heavyComputation(3000) 
    future3 = nowait heavyComputation(4000)
    
    print("⏳ All tasks started. Waiting for results...")
    
    # 結果を待機して取得
    result1 = await future1
    result2 = await future2
    result3 = await future3
    
    local total = result1 + result2 + result3
    print("🎉 All tasks completed!")
    print("Total sum: " + total)
    
    return total
}

# 実行
parallelDemo()
```

## 🏗️ Static Box（名前空間）の活用

```nyash
# 🏗️ Utility Classes with Static Boxes

static box MathUtils {
    init { PI, E }
    
    static {
        me.PI = 3.14159265359
        me.E = 2.71828182846
    }
    
    square(x) {
        return x * x
    }
    
    circleArea(radius) {
        return me.PI * me.square(radius)
    }
    
    power(base, exp) {
        local result = 1
        local i = 0
        loop(i < exp) {
            result = result * base
            i = i + 1
        }
        return result
    }
}

static box StringUtils {
    init { EMPTY }
    
    static {
        me.EMPTY = ""
    }
    
    reverse(str) {
        # 簡易的な実装例
        return "REVERSED:" + str
    }
    
    isEmpty(str) {
        return str == me.EMPTY
    }
}

# 使用例
print("π = " + MathUtils.PI)
print("Circle area (r=5): " + MathUtils.circleArea(5))
print("2^8 = " + MathUtils.power(2, 8))

print("Empty check: " + StringUtils.isEmpty(""))
print("Reverse: " + StringUtils.reverse("Hello"))
```

## 🐛 デバッグ機能の活用

```nyash
# 🐛 Debug Features Showcase

box DebugExample {
    init { data, counter }
    
    DebugExample() {
        me.data = "example"
        me.counter = 0
    }
    
    process() {
        me.counter = me.counter + 1
        return "Processed #" + me.counter
    }
}

function debuggingDemo() {
    # DebugBoxでトラッキング開始
    DEBUG = new DebugBox()
    DEBUG.startTracking()
    
    print("🔍 Creating objects for debugging...")
    
    # オブジェクトを作成してトラッキング
    obj1 = new DebugExample()
    obj2 = new DebugExample()
    
    DEBUG.trackBox(obj1, "Primary Object")
    DEBUG.trackBox(obj2, "Secondary Object")
    
    # 処理実行
    result1 = obj1.process()
    result2 = obj2.process()
    result3 = obj1.process()
    
    print("Results: " + result1 + ", " + result2 + ", " + result3)
    
    # デバッグレポート表示
    print("")
    print("=== Memory Report ===")
    print(DEBUG.memoryReport())
    
    print("")
    print("=== Full Debug Dump ===")
    print(DEBUG.dumpAll())
    
    # デバッグ情報をファイルに保存
    DEBUG.saveToFile("debug_output.txt")
    print("🎉 Debug information saved to debug_output.txt")
}

debuggingDemo()
```

## 📦 ファイル組織とモジュール

### **プロジェクト構造**
```
my_nyash_project/
├── main.hako          # メインプログラム
├── utils/
│   ├── math.hako      # 数学ユーティリティ
│   ├── string.hako    # 文字列ユーティリティ
│   └── debug.hako     # デバッグ関数
└── models/
    ├── person.hako    # Personクラス
    └── calculator.hako # Calculatorクラス
```

### **main.hako**
```nyash
# 📦 Module System Example

include "utils/math.hako"
include "utils/string.hako"
include "models/person.hako"
include "models/calculator.hako"

function main() {
    print("🚀 Multi-module Nyash Application")
    
    # 各モジュールの機能を使用
    person = new Person("Alice", 25, "alice@example.com")
    person.introduce()
    
    calc = new Calculator()
    result = calc.add(10, 20)
    print("Calculation result: " + result)
}

main()
```

## 🎯 ベストプラクティス

### **1. 変数命名**
```nyash
# ✅ Good
local userName = "alice"
local totalAmount = 1000
local isComplete = true

# ❌ Avoid
local x = "alice"
local amt = 1000
local flag = true
```

### **2. Box設計**
```nyash
# ✅ Good: 明確な責任分離
box UserAccount {
    init { username, email, balance }
    
    UserAccount(u, e) {
        me.username = u
        me.email = e
        me.balance = 0
    }
    
    deposit(amount) {
        me.balance = me.balance + amount
    }
}

# ❌ Avoid: 責任の混在
box EverythingBox {
    # 多すぎる責任を持たせない
}
```

### **3. エラーハンドリング**
```nyash
function safeOperation(a, b) {
    if b == 0 {
        print("❌ Error: Division by zero")
        return 0
    }
    return a / b
}
```

### **4. パフォーマンス考慮**
```nyash
# ✅ 効率的：static box使用
result = MathUtils.calculate(data)

# ✅ 効率的：初期化付き宣言
local result = heavyCalculation(), cache = new MapBox()

# ⚠️ 注意：不要なオブジェクト生成を避ける
loop(i < 1000) {
    # 毎回new しない設計を心がける
}
```

## 🚀 次のステップ

### **学習順序**
1. ✅ **基本構文** - このガイドで完了
2. **並行処理** - `test_async_*.hako`を参考に
3. **Static Box応用** - ユーティリティクラス作成
4. **デバッグ技法** - DebugBox完全活用
5. **アプリケーション開発** - 実践的なプロジェクト

### **サンプルプログラム**
```bash
# 実装済みサンプル
./target/debug/nyash test_local_init.hako      # 初期化付き変数
./target/debug/nyash app_dice_rpg.hako         # RPGバトルゲーム
./target/debug/nyash app_statistics.hako       # 統計計算
./target/debug/nyash test_async_parallel.hako  # 並行処理
```

### **リファレンス**
- `docs/LANGUAGE_OVERVIEW_2025.md` - 言語全体概要
- `docs/TECHNICAL_ARCHITECTURE_2025.md` - 技術仕様
- `CLAUDE.md` - 開発者向け詳細情報

## 🎉 おめでとうございます！

このガイドでNyashの主要機能を学習しました！

**習得内容:**
- ✅ 基本構文（変数・演算子・制御構造）
- ✅ Box（クラス）定義とオブジェクト指向
- ✅ 並行処理・非同期プログラミング
- ✅ Static Box・名前空間システム
- ✅ デバッグ機能・開発支援ツール
- ✅ 実践的なアプリケーション開発

**Nyashでプログラミングの新しい可能性を探究してください！** 🚀

---
*Getting Started Guide v1.0*  
*Everything is Box - Start Simple, Think Big*
