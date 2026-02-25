# 🔄 Nyash デリゲーションシステム

## 📋 概要

Nyashは継承の代わりに「完全明示デリゲーション」を採用しています。
これは「Everything is Box」哲学に基づく、より安全で明確な設計アプローチです。

注: `init { ... }` は legacy のフィールド宣言（slot）です（互換のために残っています）。新しい宣言モデル（Unified Members）は `docs/reference/language/EBNF.md` を参照してください。

## 🎯 なぜデリゲーションか

### 継承の問題点
1. **暗黙的な結合**: 親クラスの変更が子クラスに予期せぬ影響
2. **多重継承の複雑性**: ダイヤモンド問題
3. **実装の隠蔽**: 何がどこから来ているか不明確

### デリゲーションの利点
1. **明示的**: すべての委譲が明確に記述される
2. **柔軟**: 複数のBoxから選択的に機能を組み合わせ
3. **安全**: 予期せぬ副作用を防ぐ

## 🏗️ 基本構文

### from構文によるデリゲーション宣言

```nyash
// ParentBoxにデリゲート
box Child from Parent {
    init { childField }
    
    // 親のコンストラクタを呼ぶ
    pack(name, age, childData) {
        from Parent.pack(name, age)
        me.childField = childData
    }
}
```

### override必須

```nyash
box Child from Parent {
    // ❌ エラー: overrideキーワードが必要
    toString() {
        return "Child"
    }
    
    // ✅ 正しい: 明示的override
    override toString() {
        return "Child: " + from Parent.toString()
    }
}
```

## 📖 デリゲーションパターン

### 1. 基本的なデリゲーション

```nyash
box Animal {
    init { name, species }
    
    pack(animalName, animalSpecies) {
        me.name = animalName
        me.species = animalSpecies
    }
    
    speak() {
        return me.name + " makes a sound"
    }
    
    toString() {
        return me.species + " named " + me.name
    }
}

box Dog from Animal {
    init { breed }
    
    pack(dogName, dogBreed) {
        from Animal.pack(dogName, "Dog")
        me.breed = dogBreed
    }
    
    override speak() {
        return me.name + " barks!"
    }
    
    // toStringは親のものをそのまま使用
}
```

### 2. 選択的メソッドオーバーライド

```nyash
box EnhancedDog from Dog {
    init { tricks }
    
    pack(name, breed) {
        from Dog.pack(name, breed)
        me.tricks = new ArrayBox()
    }
    
    // speakは親のまま使用
    
    // toStringだけオーバーライド
    override toString() {
        local base = from Animal.toString()  // 祖父母から直接
        return base + " (Enhanced)"
    }
    
    // 新しいメソッド追加
    addTrick(trick) {
        me.tricks.push(trick)
    }
}
```

### 3. 複数Box組み合わせ（予定機能）

```nyash
// 将来的な複数デリゲーション構想
box MultiChild from ParentA, ParentB {
    pack() {
        from ParentA.pack()
        from ParentB.pack()
    }
    
    methodA() {
        return from ParentA.method()
    }
    
    methodB() {
        return from ParentB.method()
    }
}
```

## 🌟 birth構文 - Box哲学の具現化

### コンストラクタの優先順位

```nyash
box User {
    init { name, email }
    
    // 優先度1: birth（推奨）
    birth(userName, userEmail) {
        me.name = userName
        me.email = userEmail
        print("🌟 " + userName + " が誕生しました！")
    }
    
    // 優先度2: init（birthがない場合）
    init(name, email) {
        me.name = name
        me.email = email
    }
    
    // 優先度3: Box名（互換性のため）
    User(name, email) {
        me.name = name
        me.email = email
    }
}

// birthが優先的に使用される
local user = new User("Alice", "alice@example.com")
```

### birth構文とデリゲーション

```nyash
box Product {
    init { name, price }
    
    birth(productName, productPrice) {
        me.name = productName
        me.price = productPrice
        print("📦 Product created: " + productName)
    }
}

box DiscountedProduct from Product {
    init { discount }
    
    birth(name, originalPrice, discountPercent) {
        local discountedPrice = originalPrice * (1 - discountPercent / 100)
        from Product.birth(name, discountedPrice)  # 親のbirthを呼ぶ
        me.discount = discountPercent
    }
    
    originalPrice() {
        return me.price / (1 - me.discount / 100)
    }
}
```

### 🚨 pack構文 - ビルトインBox継承専用

**重要**: `pack`構文は**ビルトインBox継承専用**です。ユーザー定義Boxでは使用しません。

```nyash
# ✅ 正しい使い方（ビルトインBox継承のみ）
box EnhancedP2P from P2PBox {
    init { extraFeatures }
    
    pack(nodeId, transport) {
        from P2PBox.pack(nodeId, transport)  # ビルトインBoxの初期化
        me.extraFeatures = new ArrayBox()
    }
}

box CustomMath from MathBox {
    init { history }
    
    pack() {
        from MathBox.pack()  # ビルトインBoxの初期化
        me.history = new ArrayBox()
    }
}

# ❌ 間違い（ユーザー定義Boxでpack使用）
box RegularUser {
    pack(name) {  # これは間違い！birth()を使う
        me.name = name
    }
}
```

## 🔍 from構文の詳細

### メソッド内でのfrom使用

```nyash
box Child from Parent {
    override process(data) {
        // 前処理
        local prepared = me.prepare(data)
        
        // 親の処理を呼ぶ
        local result = from Parent.process(prepared)
        
        // 後処理
        return me.postProcess(result)
    }
}
```

### 条件付きデリゲーション

```nyash
box SmartChild from Parent {
    override calculate(value) {
        if value > 100 {
            // 大きな値は親に任せる
            return from Parent.calculate(value)
        } else {
            // 小さな値は自分で処理
            return value * 2
        }
    }
}
```

## ⚡ ベストプラクティス

### 1. 明示的なoverride

```nyash
// ✅ 良い: 意図が明確
override toString() {
    return "Custom: " + from Parent.toString()
}

// ❌ 悪い: overrideなしはエラー
toString() {
    return "Custom"
}
```

### 2. 適切なデリゲーション粒度

```nyash
// ✅ 良い: 必要な部分だけオーバーライド
box CustomList from ArrayBox {
    override push(item) {
        console.log("Adding: " + item)
        from ArrayBox.push(item)
    }
    // 他のメソッドはそのまま使用
}

// ❌ 悪い: すべてをオーバーライド
box BadList from ArrayBox {
    override push(item) { /* ... */ }
    override pop() { /* ... */ }
    override get(i) { /* ... */ }
    override set(i, v) { /* ... */ }
    // すべて再実装は無駄
}
```

### 3. デリゲーションチェーンの管理

```nyash
// ✅ 良い: 明確な責任分離
box A {
    process() { return "A" }
}

box B from A {
    override process() {
        return from A.process() + "->B"
    }
}

box C from B {
    override process() {
        return from B.process() + "->C"
    }
}

// 結果: "A->B->C"
```

## 🚨 注意点

### 1. 循環デリゲーション禁止

```nyash
// ❌ エラー: 循環デリゲーション
box A from B { }
box B from A { }  // コンパイルエラー
```

### 2. 多段デリゲーション制限

```nyash
// 現在の制限
box A { }
box B from A { }
box C from B {
    method() {
        // ❌ エラー: CはAにデリゲートしていない
        from A.method()
        
        // ✅ OK: 直接の親
        from B.method()
    }
}
```

### 3. 動的デリゲーション不可

```nyash
// ❌ エラー: デリゲーション先は静的に決定
box Dynamic from (condition ? A : B) { }
```

## 🔮 将来の拡張構想

### 1. 複数デリゲーション
```nyash
box Multi from Network, Storage {
    save(data) {
        from Storage.save(data)
        from Network.sync(data)
    }
}
```

### 2. 条件付きデリゲーション
```nyash
box Smart from Parent when Parent.version >= 2.0 {
    // バージョン対応
}
```

### 3. デリゲーションプロキシ
```nyash
box Proxy delegates * to target {
    // すべてのメソッドを自動委譲
}
```

---

関連ドキュメント：
- [Everything is Box](everything-is-box.md)
- [override/from構文詳細](../override-delegation-syntax.md)
- [言語リファレンス](../language-reference.md)
