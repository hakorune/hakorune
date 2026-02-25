# 🌟 Nyash Override + Delegation 統一構文仕様

バージョン: 2.0  
作成日: 2025年8月11日  
ステータス: 正式決定

## 📋 概要

Nyashプログラミング言語における明示的オーバーライドとデリゲーション構文の完全仕様。世界初の**完全明示デリゲーション言語**としてのNyashの核心機能を定義する。

## 🎯 設計哲学

### 基本原則
1. **完全明示性**: すべての動作を明示的に宣言
2. **曖昧性の完全排除**: 暗黙の動作は一切許可しない
3. **統一構文**: デリゲーションとオーバーライドの完全統合
4. **初学者フレンドリー**: 直感的で理解しやすい構文

### Everything is Box との整合性
- すべてのオブジェクトがBox
- デリゲーション先もBox
- オーバーライドもBoxメソッドの置換

## 🔥 基本構文

### デリゲーション宣言
```nyash
box Child from Parent {
    // 親Boxからの機能デリゲーション
}

// 多重デリゲーション
box Child from Parent1, Parent2 {
    // 複数のBoxからの機能デリゲーション
}
```

### メソッドオーバーライド
```nyash
box Child from Parent {
    // 必須: overrideキーワードによる明示的宣言
    override methodName(params) {
        // オーバーライド実装
        from Parent.methodName(params)  // 親実装呼び出し（任意）
    }
}
```

### コンストラクタデリゲーション
```nyash
box Child from Parent {
    init(params) {  # init構文に統一
        from Parent.init(params)  # 必須: 親コンストラクタ明示呼び出し
        me.childSpecificField = value
    }
}
```

## 📚 詳細仕様

### 1. Override キーワード

#### 必須条件
- 親Box（デリゲーション先）に同名メソッドが存在する場合のみ使用可能
- 同名メソッドが存在しない場合は**コンパイルエラー**

#### 禁止事項
- 同一Box内での同名メソッド重複定義は**すべてエラー**
- 暗黙のオーバーライドは**完全禁止**

#### 構文例
```nyash
box MeshNode from P2PBox {
    // ✅ 正しい使用法
    override send(intent, data, target) {
        me.routing.log(target)
        from P2PBox.send(intent, data, target)
    }
    
    // ❌ エラー: P2PBoxに存在しないメソッド
    override nonExistentMethod() {
        // Error: Method 'nonExistentMethod' does not exist in parent P2PBox
    }
    
    // ❌ エラー: overrideなしで親メソッドと同名
    send(intent, data, target) {
        // Error: Method 'send' overrides parent method. Add 'override' keyword.
    }
}
```

### 2. From デリゲーション構文

#### 基本構文
```nyash
from ParentBox.methodName(args)
from ParentBox.fieldName
```

#### メソッド呼び出し
```nyash
// 親の特定メソッドを明示的に呼び出し
from P2PBox.send(intent, data, target)

// 複数親からの呼び出し
from Logger.log("Starting operation")
from P2PBox.send(intent, data, target)
from Cache.store(data)
```

#### フィールドアクセス
```nyash
// 親のフィールドへのアクセス
local status = from P2PBox.connectionStatus
from Logger.logLevel = "DEBUG"
```

#### コンストラクタ呼び出し
```nyash
init(nodeId, world) {  # init構文に統一
    from P2PBox.init(nodeId, world)  # 完全統一構文
    me.routing = RoutingTable()
}
```

### 3. 多重デリゲーション

#### 基本形式
```nyash
box ComplexNode from P2PBox, Logger, Cache {
    override send(intent, data, target) {
        from Logger.debug("Sending: " + intent)    // Logger親から
        from Cache.store(intent, data)             // Cache親から  
        from P2PBox.send(intent, data, target)     // P2PBox親から
    }
}
```

#### 曖昧性の解消
```nyash
// 複数親に同名メソッドが存在する場合
box ConflictNode from ParentA, ParentB {
    // ❌ エラー: どちらのprocessを置換するか不明
    override process(data) {
        // Error: Method 'process' exists in multiple parents. Use specific parent.
    }
    
    // ✅ 正しい解決法: 親を明示指定
    override ParentA.process(data) {
        // ParentAのprocessをオーバーライド
        from ParentA.process(data)
    }
    
    override ParentB.process(data) {
        // ParentBのprocessをオーバーライド  
        from ParentB.process(data)
    }
}
```

### 4. エラーメッセージ仕様

#### 重複定義エラー
```
Error: Method 'send' is already defined in this box at line 15.
  --> box.hako:20:5
   |
20 |     send(msg) {
   |     ^^^^ duplicate method definition
   |
Help: Remove duplicate definition or rename method.
```

#### Missing Override エラー
```
Error: Method 'send' overrides a parent method. Add 'override' keyword.
  --> box.hako:18:5
   |
18 |     send(intent, data, target) {
   |     ^^^^ missing 'override' keyword
   |
Help: Change to 'override send(intent, data, target) {'
```

#### Wrong Override エラー  
```
Error: Method 'newMethod' does not exist in any parent. Remove 'override' keyword.
  --> box.hako:22:5
   |
22 |     override newMethod() {
   |     ^^^^^^^^ unnecessary 'override'
   |
Help: Remove 'override' or verify parent method name.
```

#### 曖昧Override エラー
```
Error: Method 'process' exists in multiple parents. Specify which parent to override.
  --> box.hako:25:5
   |
25 |     override process(data) {
   |     ^^^^^^^^ ambiguous override
   |
Help: Use 'override ParentA.process' or 'override ParentB.process'
```

## 🚫 禁止事項

### 1. 暗黙のオーバーライド
```nyash
box Child from Parent {
    send(msg) {  // ❌ エラー: overrideキーワードなし
        print("Child implementation")
    }
}
```

### 2. コンストラクタオーバーロード
```nyash
box Node {
    init(id) {           // 最初の定義
        me.id = id
    }
    
    init(id, name) {     // ❌ エラー: 重複定義
        me.id = id
        me.name = name
    }
}
```

### 3. 同名メソッド重複定義
```nyash
box Example {
    process(data) {             // 最初の定義
        print("Version 1")
    }
    
    process(data) {             // ❌ エラー: 重複定義
        print("Version 2")  
    }
}
```

## ✅ ベストプラクティス

### 1. 明示的な親呼び出し
```nyash
box MeshNode from P2PBox {
    override send(intent, data, target) {
        // 前処理
        me.routing.logOutgoing(target)
        
        // 親実装呼び出し（明示的）
        from P2PBox.send(intent, data, target)
        
        // 後処理
        me.statistics.incrementSentCount()
    }
}
```

### 2. 多重デリゲーションでの順序指定
```nyash
box SmartNode from P2PBox, Logger, Cache {
    override send(intent, data, target) {
        // 1. ログ記録
        from Logger.info("Sending to: " + target)
        
        // 2. キャッシュ保存
        from Cache.store(intent + ":" + target, data)
        
        // 3. 実際の送信
        from P2PBox.send(intent, data, target)
    }
}
```

### 3. コンストラクタチェーン
```nyash
box SecureNode from P2PBox {
    init security = SecurityManager()
    
    init(nodeId, world, keyFile) {  # init構文に統一
        // 1. 親初期化（必須）
        from P2PBox.init(nodeId, world)
        
        // 2. 子固有の初期化
        me.security = SecurityManager()
        me.security.loadKeys(keyFile)
    }
}
```

## 🔮 将来の拡張

### 1. Final メソッド（検討中）
```nyash
box Parent {
    final criticalMethod() {  // オーバーライド禁止
        // 重要な処理
    }
}
```

### 2. Abstract メソッド（検討中）
```nyash  
box AbstractParent {
    abstract process(data)  // 子でのoverride必須
}
```

### 3. Override チェック強化（検討中）
```nyash
override! send(data) {      // 親呼び出し必須チェック
    // from Parent.send(data) がないとエラー
}
```

## 📊 他言語との比較

| 言語 | 継承方式 | オーバーライド | 親呼び出し | 多重継承 |
|------|----------|---------------|-----------|----------|
| **Nyash** | デリゲーション | `override` 必須 | `from Parent.method()` | 明示的解消 |
| Java | クラス継承 | `@Override` 注釈 | `super.method()` | 不可 |
| Python | クラス継承 | 暗黙 | `super().method()` | MRO（複雑） |
| C# | クラス継承 | `override` 必須 | `base.method()` | 不可 |
| TypeScript | プロトタイプ | 暗黙 | `super.method()` | 不可 |

### Nyashの優位性
1. **完全な明示性**: すべての動作が明確
2. **曖昧性の完全排除**: 多重デリゲーションでも安全  
3. **統一構文**: デリゲーションとオーバーライドが統合
4. **初学者フレンドリー**: 分かりやすいエラーメッセージ

---

**この仕様により、Nyashは世界初の「完全明示デリゲーション言語」として、プログラミング言語史に新たな1ページを刻むことになります。** 🌟