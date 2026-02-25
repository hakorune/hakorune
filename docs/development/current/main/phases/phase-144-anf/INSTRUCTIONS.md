# Phase 144-anf: ANF (A-Normal Form) Evaluation Order Specification

**Status**: Design SSOT (docs-only, no implementation)
**Date**: 2025-12-19
**Scope**: Normalized JoinIR における impure 式の評価順序固定
**Purpose**: Call/MethodCall 導入（Phase 141 P2+）前に、ANF による順序固定の方針を確立

---

## Table of Contents

0. [Executive Summary](#0-executive-summary)
1. [Purpose - Why ANF?](#1-purpose---why-anf)
2. [ANF Definition (SSOT)](#2-anf-definition-ssot)
3. [Scope and Non-Goals](#3-scope-and-non-goals)
4. [Problem Scenarios](#4-problem-scenarios)
5. [ANF Contract for Normalized JoinIR](#5-anf-contract-for-normalized-joinir)
6. [Diagnostic Strategy (Strict Mode)](#6-diagnostic-strategy-strict-mode)
7. [Implementation Roadmap](#7-implementation-roadmap)
8. [Acceptance Criteria](#8-acceptance-criteria)
9. [Out-of-Scope Handling](#9-out-of-scope-handling)
10. [References](#10-references)
11. [Revision History](#11-revision-history)

---

## 0. Executive Summary

**問題（Problem）**:
Phase 141 P2+ で Call/MethodCall を一般化する際、副作用を持つ式（impure expression）の評価順序が未定義のままだと、実行結果が非決定的になり、バグの温床となる。

**例（Bad Code）**:
```hako
// ❌ 評価順序未定義！counter.inc() と counter.get() どちらが先？
result = counter.inc() + counter.get()
// 結果: 実装依存で 1+0=1 または 1+1=2 になる可能性
```

**解決策（Solution）**:
**ANF（A-Normal Form）** を導入し、impure 式を必ず先に評価して temporary 変数に束縛する。

**例（Good Code after ANF transformation）**:
```hako
// ✅ ANF 変換後: 評価順序明確（左→右）
local _t1 = counter.inc()  // 先に inc() 評価
local _t2 = counter.get()  // 次に get() 評価
result = _t1 + _t2         // pure 式のみ（+）
```

**Phase 144-anf の責務**:
- ANF の定義・契約・診断タグを **SSOT（docs-only）** として確立
- 実装コードは書かない（Phase 145-147 で段階投入）
- Fail-Fast 診断戦略を設計（JoinIR 不変条件 #6 に準拠）

---

## 1. Purpose - Why ANF?

### 1.1 背景（Background）

Phase 131-143 で Normalized JoinIR の制御フロー（loop/if/break/continue）を段階的に構築してきた。
これまでの式（expression）は **pure のみ**（副作用なし）：

- 変数参照（`x`, `flag`）
- リテラル（`1`, `"abc"`, `true`）
- 算術演算（`x + 2`, `count * 10`）
- 比較演算（`flag == 1`, `i < len`）

Phase 140 で `NormalizedExprLowererBox` を導入し、pure 式の一般化を達成した。

### 1.2 次の課題（Next Challenge）

Phase 141 P2+ で **impure 式**（副作用あり）を導入する：

- **Call**: `f(x, y)` - 関数呼び出し（副作用の可能性）
- **MethodCall**: `obj.method(arg)` - メソッド呼び出し（状態変更の可能性）

**問題点（Problem）**:

**評価順序が未定義のまま impure 式を許すと、実行結果が非決定的になる。**

```hako
// ❌ どちらが先に評価される？
x = f() + g()

// ケース 1: f() → g() の順序
// f() が副作用で counter を 1 増やす
// g() が counter を読む → counter=1
// 結果: x = (何か) + 1

// ケース 2: g() → f() の順序
// g() が counter を読む → counter=0
// f() が副作用で counter を 1 増やす
// 結果: x = (何か) + 0

// 🚨 実装依存で結果が変わる！
```

### 1.3 ANF による解決（Solution）

**ANF（A-Normal Form）** を導入し、**評価順序を明示的に固定**する。

**原則**:
1. **Impure 式は immediate position に出現しない**（必ず先に評価）
2. **評価順序は left-to-right, depth-first**（左から右、深さ優先）
3. **Hoist strategy**: impure 式の結果を temporary に束縛してから使用

**変換例（Transformation）**:

```hako
// Before (評価順序未定義):
x = f() + g()

// After ANF (評価順序明確):
local _t1 = f()   // Step 1: 左の impure 式を先に評価
local _t2 = g()   // Step 2: 右の impure 式を次に評価
x = _t1 + _t2     // Step 3: pure 式のみ（+）
```

**利点（Benefits）**:
- ✅ **決定的実行**（Deterministic Execution）: 評価順序が仕様で固定
- ✅ **デバッグ容易性**（Debuggability）: temporary 変数で中間値を観測可能
- ✅ **最適化の基盤**（Optimization Foundation）: pure/impure 境界が明確
- ✅ **並行安全性**（Concurrency Safety）: 副作用の発生箇所が明示的

---

## 2. ANF Definition (SSOT)

### 2.1 A-Normal Form とは

**定義（Definition）**:

A-Normal Form（ANF）は、プログラムの中間表現形式の一種で、以下の性質を持つ：

1. **すべての impure 式が immediate position に出現しない**
   - impure 式の結果は必ず変数に束縛される

2. **すべての式の評価順序が明示的**
   - let-binding の順序で評価順が決まる

**起源（Origin）**:

ANF は Flanagan et al. (1993) "The Essence of Compiling with Continuations" で提案された。
関数型言語のコンパイラ（SML, Haskell, OCaml 等）で広く採用されている。

### 2.2 Pure vs Impure の分類

**Pure Expression（純粋式）**:

副作用がなく、同じ入力に対して常に同じ出力を返す式。

- 変数参照: `x`, `flag`, `counter`
- リテラル: `1`, `"abc"`, `true`, `false`
- 算術演算: `x + 2`, `a * b`, `count - 1`
- 比較演算: `x == 1`, `i < len`, `flag != 0`
- 論理演算: `not cond`, `a and b`, `a or b`
- **注意**: 変数参照自体は pure だが、変数が指す値が mutable な場合は注意が必要

**Impure Expression（非純粋式）**:

副作用がある、または評価ごとに異なる結果を返す可能性がある式。

- **Call**: `f(x, y)` - 関数呼び出し
  - 副作用の例: グローバル状態変更、I/O、例外送出

- **MethodCall**: `obj.method(arg)` - メソッド呼び出し
  - 副作用の例: オブジェクト状態変更、リソース獲得/解放

- **NewBox**: `new SomeBox()` - オブジェクト生成
  - 副作用: メモリ割り当て、コンストラクタ実行

- **ExternCall**: `print(msg)`, `exit(code)` - 外部関数呼び出し
  - 副作用: I/O、プログラム終了

**Phase 144-anf の Scope**:

Phase 144 では **Call/MethodCall のみ** を対象とする（NewBox/ExternCall は Phase 147+ で対応）。

### 2.3 ANF の形式的定義

**BNF（Backus-Naur Form）**:

```bnf
<anf-program> ::= <anf-stmt>*

<anf-stmt> ::= <var> = <pure-expr>
             | <var> = <impure-expr>
             | if <var> { <anf-stmt>* } else { <anf-stmt>* }
             | loop(<var>) { <anf-stmt>* }
             | break | continue | return <var>

<pure-expr> ::= <var>
              | <literal>
              | <var> <binop> <var>
              | <unop> <var>
              | <var> <cmpop> <var>

<impure-expr> ::= <var>(<var>,*)        // Call
                | <var>.<method>(<var>,*) // MethodCall

<var> ::= identifier
<literal> ::= integer | string | bool
```

**重要な制約（Constraints）**:

1. **Immediate positions に impure 式は出現しない**:
   - ❌ `x = f() + g()` （二項演算の引数に impure 式）
   - ✅ `t1 = f(); t2 = g(); x = t1 + t2` （hoist してから使用）

2. **すべての式の引数は変数またはリテラル**:
   - ❌ `x = f(g())` （引数に impure 式）
   - ✅ `t = g(); x = f(t)` （hoist してから渡す）

3. **制御フロー式の条件も変数**:
   - ❌ `if f() { ... }` （条件に impure 式）
   - ✅ `cond = f(); if cond { ... }` （hoist してから条件判定）

### 2.4 Normalized JoinIR 文脈での ANF

**Normalized JoinIR の特徴**:

- **SSA form（Static Single Assignment）**: 各変数は 1 回だけ代入
- **PHI-free**: PHI 命令を含まない（continuation passing で状態を渡す）
- **Continuation-based**: 関数呼び出しは tail call で実現

**ANF との統合**:

Normalized JoinIR では、ANF 変換を **ExprLowerer 層** で実施する：

1. **ExprLowererBox** が impure 式を検出
2. **Hoist strategy** で temporary JoinInst を生成
3. **Pure-only scope** で immediate position を検証

**例（JoinIR への lowering）**:

```hako
// Source code:
result = x + f(y)

// ANF transformation (conceptual):
local _t1 = f(y)
result = x + _t1

// Normalized JoinIR (lowered):
JoinInst::Call {
    func: f_id,
    args: [y_vid],
    k_next: Some(k_cont),
    dst: Some(t1_vid),
}
// k_cont function:
JoinInst::Compute(MirLikeInst::BinOp {
    dst: result_vid,
    op: BinaryOp::Add,
    lhs: x_vid,
    rhs: t1_vid,  // ANF temporary
})
```

### 2.5 Left-to-Right, Depth-First 評価順序

**原則（Principle）**:

Impure 式の評価順序は **left-to-right, depth-first** を保証する。

**例 1（Binary operation）**:

```hako
// Source:
x = f() + g()

// Evaluation order: f() → g() → +
// ANF:
local _t1 = f()   // Step 1 (left)
local _t2 = g()   // Step 2 (right)
x = _t1 + _t2     // Step 3 (pure)
```

**例 2（Nested calls）**:

```hako
// Source:
x = f(g(), h())

// Evaluation order: g() → h() → f()
// ANF:
local _t1 = g()      // Step 1 (first arg, depth-first)
local _t2 = h()      // Step 2 (second arg)
x = f(_t1, _t2)      // Step 3 (outer call)
```

**例 3（Method chain）**:

```hako
// Source:
x = obj.method1().method2()

// Evaluation order: obj.method1() → result.method2()
// ANF:
local _t1 = obj.method1()   // Step 1
x = _t1.method2()           // Step 2
```

**例 4（Loop condition with impure）**:

```hako
// Source:
loop(hasNext()) {
    process()
}

// Evaluation order: hasNext() → (loop decision) → process() → (repeat)
// ANF (hoist to loop preheader):
local _cond = hasNext()
loop(_cond) {
    process()
    _cond = hasNext()  // Re-evaluate at end of loop body
}
```

---

## 3. Scope and Non-Goals

### 3.1 In Scope (Phase 144-anf)

**このドキュメントで扱う範囲**:

1. **ANF の定義と契約（SSOT）**
   - Pure vs Impure の分類
   - 評価順序の規則（left-to-right, depth-first）
   - Hoist strategy の基本原則

2. **問題シナリオの明確化**
   - Observable side effects（副作用の順序）
   - Exception ordering（例外の発生順序）
   - Resource acquisition（リソース獲得の順序）

3. **診断タグの設計**
   - `[joinir/anf/order_violation]`: 非 ANF 式検出
   - `[joinir/anf/pure_required]`: impure in pure-only scope
   - `[joinir/anf/hoist_failed]`: Loop condition hoist 失敗

4. **実装ロードマップ（Phase 145-147）**
   - Phase 145: ANF transformation core（BinaryOp with impure operands）
   - Phase 146: Loop condition hoisting
   - Phase 147: If condition ANF

### 3.2 Out of Scope (Phase 144-anf)

**このドキュメントで扱わない範囲**:

1. **実装コード**
   - Phase 144 は **docs-only SSOT** であり、Rust コードは書かない
   - 実装は Phase 145+ で段階投入

2. **Call/MethodCall の一般化**
   - Call/MethodCall の lowering は Phase 141 P2+ で実施
   - Phase 144 は ANF の「契約」のみを固める

3. **型推論・エフェクトシステム**
   - Pure/Impure の自動判定は扱わない（手動分類またはアノテーション前提）
   - Effect system（副作用の型レベル追跡）は Phase 150+ で検討

4. **最適化（Optimization）**
   - ANF 後の最適化（dead code elimination, common subexpression elimination）は扱わない
   - Phase 144 は「順序固定」のみに集中

5. **NewBox/ExternCall の ANF 対応**
   - NewBox（`new SomeBox()`）の ANF 対応は Phase 147+ で検討
   - ExternCall（`print(msg)`, `exit(code)`）は Phase 148+ で検討

### 3.3 Graceful Fallback Strategy

**原則（Principle）**:

ANF 変換が失敗した場合、**`Ok(None)` で out-of-scope** として扱い、既定挙動不変を維持する。

**フォールバックの種類**:

1. **Soft fallback（許容）**: ANF 変換失敗 → legacy lowering へ
   - 例: 複雑な nested call を ANF 化できない → legacy path
   - 条件: `Ok(None)` を返し、debug log を出力

2. **Prohibited fallback（禁止）**: サイレント退避、契約違反の握りつぶし
   - 例: ANF 化失敗を隠して不正な JoinIR を生成
   - 対策: Strict mode（`HAKO_ANF_STRICT=1`）で fail-fast

**実装例（Conceptual）**:

```rust
pub fn try_lower_with_anf(
    expr: &ASTNode,
    scope: ExprLoweringScope,
    // ...
) -> Result<Option<ValueId>, String> {
    match scope {
        ExprLoweringScope::PureOnly => {
            // Pure-only scope: impure 式を検出したら fail-fast
            if is_impure_expr(expr) {
                if crate::config::env::anf_strict_enabled() {
                    return Err(error_tags::anf_pure_required(
                        &expr.to_string(),
                        "impure expression in pure-only scope"
                    ));
                } else {
                    // Graceful fallback: out-of-scope
                    return Ok(None);
                }
            }
        }
        ExprLoweringScope::AllowImpure => {
            // Impure 許容: ANF 変換を試みる
            if is_impure_expr(expr) {
                match try_anf_transform(expr) {
                    Ok(vid) => return Ok(Some(vid)),
                    Err(e) => {
                        if crate::config::env::anf_strict_enabled() {
                            return Err(e);
                        } else {
                            // Graceful fallback: legacy lowering
                            eprintln!("[anf/debug] Fallback to legacy: {}", e);
                            return Ok(None);
                        }
                    }
                }
            }
        }
    }
    // ... Pure 式の lowering ...
}
```

---

## 4. Problem Scenarios

### 4.1 Observable Side Effects（副作用の順序）

**Scenario 1: Counter increment**

```hako
// Source code:
static box Counter {
    value: IntegerBox

    birth() {
        me.value = 0
    }

    inc() {
        me.value = me.value + 1
        return me.value
    }

    get() {
        return me.value
    }
}

static box Main {
    main() {
        local counter = new Counter()

        // ❌ 評価順序未定義！
        local result = counter.inc() + counter.inc()

        // ケース 1: 左の inc() → 右の inc() → +
        // result = 1 + 2 = 3

        // ケース 2: 右の inc() → 左の inc() → +
        // result = 1 + 2 = 3 (偶然同じ)

        // ケース 3: コンパイラが勝手に最適化
        // result = 2 * counter.inc() = 2 * 1 = 2 (バグ！)
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        local counter = new Counter()

        // ✅ ANF: 評価順序明確（左→右）
        local _t1 = counter.inc()  // Step 1: value = 1
        local _t2 = counter.inc()  // Step 2: value = 2
        local result = _t1 + _t2   // Step 3: result = 1 + 2 = 3

        // 保証: 常に result = 3
    }
}
```

**Scenario 2: File I/O**

```hako
// Source code:
static box Main {
    main() {
        local file = new FileBox("data.txt")

        // ❌ 評価順序未定義！
        local data = file.read() + file.read()

        // ケース 1: 1行目 + 2行目
        // ケース 2: 2行目 + 1行目（逆順？）
        // ケース 3: 1行目 + 1行目（重複読み？）
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        local file = new FileBox("data.txt")

        // ✅ ANF: 評価順序明確（左→右）
        local _t1 = file.read()  // Step 1: 1行目
        local _t2 = file.read()  // Step 2: 2行目
        local data = _t1 + _t2   // Step 3: concat

        // 保証: 常に "1行目2行目"
    }
}
```

### 4.2 Exception Ordering（例外の発生順序）

**Scenario 3: Division by zero**

```hako
// Source code:
static box Main {
    main() {
        // ❌ 評価順序未定義！
        local result = divide(10, 0) + divide(20, 0)

        // ケース 1: 左の divide() が先 → ZeroDivisionError (10/0)
        // ケース 2: 右の divide() が先 → ZeroDivisionError (20/0)

        // どちらの例外が投げられるか不定！
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        // ✅ ANF: 評価順序明確（左→右）
        local _t1 = divide(10, 0)  // Step 1: ZeroDivisionError (10/0)
        // この時点で例外が発生するため、以降は実行されない
        local _t2 = divide(20, 0)  // (unreachable)
        local result = _t1 + _t2   // (unreachable)

        // 保証: 常に ZeroDivisionError (10/0) が投げられる
    }
}
```

**Scenario 4: Null pointer dereference**

```hako
// Source code:
static box Main {
    main() {
        local obj1 = getObject1()  // null を返す可能性
        local obj2 = getObject2()  // null を返す可能性

        // ❌ 評価順序未定義！
        local result = obj1.method() + obj2.method()

        // ケース 1: obj1.method() が先 → NullPointerError (obj1)
        // ケース 2: obj2.method() が先 → NullPointerError (obj2)

        // どちらの例外が投げられるか不定！
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        local obj1 = getObject1()
        local obj2 = getObject2()

        // ✅ ANF: 評価順序明確（左→右）
        local _t1 = obj1.method()  // Step 1: obj1 が null なら NullPointerError
        local _t2 = obj2.method()  // Step 2: (Step 1 が成功した場合のみ実行)
        local result = _t1 + _t2   // Step 3: (Step 2 が成功した場合のみ実行)

        // 保証: obj1 が null なら常に NullPointerError (obj1)
        // 保証: obj1 が valid で obj2 が null なら常に NullPointerError (obj2)
    }
}
```

### 4.3 Resource Acquisition（リソース獲得の順序）

**Scenario 5: File locking**

```hako
// Source code:
static box Main {
    main() {
        // ❌ 評価順序未定義！
        local result = openFile("a.txt") + openFile("b.txt")

        // ケース 1: a.txt → b.txt の順で open
        // → a.txt がロックされた後、b.txt を open

        // ケース 2: b.txt → a.txt の順で open
        // → b.txt がロックされた後、a.txt を open

        // デッドロックのリスク！（他のプロセスが逆順で開く場合）
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        // ✅ ANF: 評価順序明確（左→右）
        local _t1 = openFile("a.txt")  // Step 1: a.txt を open
        local _t2 = openFile("b.txt")  // Step 2: b.txt を open
        local result = _t1 + _t2       // Step 3: (何らかの処理)

        // 保証: 常に a.txt → b.txt の順で open
        // デッドロック防止: すべてのコードで同じ順序を強制
    }
}
```

**Scenario 6: Database transaction**

```hako
// Source code:
static box Main {
    main() {
        local db = new DatabaseBox()

        // ❌ 評価順序未定義！
        local result = db.insert("user1") + db.insert("user2")

        // ケース 1: user1 → user2 の順で insert
        // → user1 の ID=1, user2 の ID=2

        // ケース 2: user2 → user1 の順で insert
        // → user2 の ID=1, user1 の ID=2

        // データベースの状態が不定！
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        local db = new DatabaseBox()

        // ✅ ANF: 評価順序明確（左→右）
        local _t1 = db.insert("user1")  // Step 1: user1 を insert (ID=1)
        local _t2 = db.insert("user2")  // Step 2: user2 を insert (ID=2)
        local result = _t1 + _t2        // Step 3: (何らかの処理)

        // 保証: 常に user1 (ID=1), user2 (ID=2)
    }
}
```

### 4.4 Loop Condition with Impure（ループ条件の impure 式）

**Scenario 7: Iterator hasNext()**

```hako
// Source code:
static box Main {
    main() {
        local iter = new Iterator()

        // ❌ ループ条件が impure！
        loop(iter.hasNext()) {
            local item = iter.next()
            process(item)
        }

        // 問題: iter.hasNext() がいつ評価されるか不定
        // - ループ開始前に1回？
        // - 各イテレーションの前に毎回？
        // - 各イテレーションの後に毎回？
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        local iter = new Iterator()

        // ✅ ANF: ループ preheader で hoist
        local _cond = iter.hasNext()  // Preheader: 最初に評価
        loop(_cond) {
            local item = iter.next()
            process(item)
            _cond = iter.hasNext()    // Latch: ループ末尾で再評価
        }

        // 保証: hasNext() は "preheader + 各イテレーション末尾" で評価
    }
}
```

**Scenario 8: Counter with side effect**

```hako
// Source code:
static box Main {
    main() {
        local counter = new Counter()

        // ❌ ループ条件が impure！
        loop(counter.incrementAndCheck()) {
            doSomething()
        }

        // 問題: incrementAndCheck() が毎回呼ばれると無限ループ？
    }
}
```

**ANF 変換後（✅ 正しい）**:

```hako
static box Main {
    main() {
        local counter = new Counter()

        // ✅ ANF: ループ preheader で hoist
        local _cond = counter.incrementAndCheck()  // Preheader: 1回目
        loop(_cond) {
            doSomething()
            _cond = counter.incrementAndCheck()    // Latch: 2回目以降
        }

        // 保証: incrementAndCheck() は "preheader + 各イテレーション末尾" で評価
    }
}
```

---

## 5. ANF Contract for Normalized JoinIR

### 5.1 Expression Classification (Pure vs Impure)

**分類規則（Classification Rules）**:

| Expression Type | Pure/Impure | Reasoning |
|-----------------|-------------|-----------|
| Variable        | Pure        | 単なる参照（副作用なし） |
| Literal         | Pure        | 定数（副作用なし） |
| UnaryOp         | Pure        | 算術/論理演算（副作用なし） |
| BinaryOp        | Pure        | 算術演算（副作用なし） |
| Compare         | Pure        | 比較演算（副作用なし） |
| **Call**        | **Impure**  | **関数呼び出し（副作用の可能性）** |
| **MethodCall**  | **Impure**  | **メソッド呼び出し（状態変更の可能性）** |
| NewBox          | Impure      | オブジェクト生成（メモリ割り当て） |
| ExternCall      | Impure      | 外部関数（I/O 等） |

**Phase 144-anf の Scope**:

- **Call/MethodCall のみ** を impure として扱う
- NewBox/ExternCall は Phase 147+ で対応

**将来の拡張（Phase 150+）**:

- **Pure annotation**: `@pure fn f(x) { ... }` で関数を pure として明示
- **Effect system**: `fn f(x): IO<Int>` で副作用を型レベルで追跡

### 5.2 Evaluation Order Rules

**ルール 1（Left-to-Right）**:

Binary operation の引数は **左から右** の順で評価する。

```hako
// Source:
x = f() + g()

// Evaluation order: f() → g() → +
// ANF:
local _t1 = f()   // Left first
local _t2 = g()   // Right second
x = _t1 + _t2     // Pure operation last
```

**ルール 2（Depth-First）**:

Nested call の引数は **深さ優先（depth-first）** で評価する。

```hako
// Source:
x = f(g(), h())

// Evaluation order: g() → h() → f()
// ANF:
local _t1 = g()      // First arg (depth-first)
local _t2 = h()      // Second arg
x = f(_t1, _t2)      // Outer call last
```

**ルール 3（Hoist Impure）**:

Impure 式は immediate position に出現せず、必ず **hoist** する。

```hako
// ❌ Non-ANF (impure in immediate position):
x = f(g())

// ✅ ANF (hoisted):
local _t = g()
x = f(_t)
```

**ルール 4（Loop Condition Hoist）**:

Loop condition が impure な場合、**preheader + latch** で評価する。

```hako
// ❌ Non-ANF (impure in loop condition):
loop(iter.hasNext()) {
    doSomething()
}

// ✅ ANF (hoisted to preheader + latch):
local _cond = iter.hasNext()  // Preheader
loop(_cond) {
    doSomething()
    _cond = iter.hasNext()    // Latch
}
```

**ルール 5（If Condition Hoist）**:

If condition が impure な場合、**条件評価を先行** させる。

```hako
// ❌ Non-ANF (impure in if condition):
if f() {
    doThen()
} else {
    doElse()
}

// ✅ ANF (hoisted):
local _cond = f()
if _cond {
    doThen()
} else {
    doElse()
}
```

### 5.3 JoinIR Lowering Contract

**契約（Contract）**:

1. **ExprLowererBox** が ANF 変換を実施
2. **Scope parameter** で pure-only を強制
3. **JoinInst generation** で temporary を作成
4. **Out-of-scope handling** で graceful fallback

**Scope parameter**:

```rust
pub enum ExprLoweringScope {
    /// Pure-only scope: impure 式を検出したら fail-fast or out-of-scope
    PureOnly,

    /// Allow impure: ANF 変換を試みる
    AllowImpure,
}
```

**Usage examples**:

```rust
// Pure-only scope (loop condition, if condition):
NormalizedExprLowererBox::lower_expr_with_scope(
    ExprLoweringScope::PureOnly,  // ← impure 式を許さない
    cond_ast,
    env,
    body,
    next_value_id,
)

// Allow impure (assignment, return):
NormalizedExprLowererBox::lower_expr_with_scope(
    ExprLoweringScope::AllowImpure,  // ← ANF 変換を試みる
    value_ast,
    env,
    body,
    next_value_id,
)
```

**JoinInst generation（Conceptual）**:

```rust
// Source AST:
// x = f() + g()

// Step 1: Lower f() (impure)
let t1_vid = alloc_local(&mut next_value_id);
body.push(JoinInst::Call {
    func: f_id,
    args: vec![],
    k_next: Some(k_cont1),
    dst: Some(t1_vid),
});

// k_cont1: Lower g() (impure)
let t2_vid = alloc_local(&mut next_value_id);
body.push(JoinInst::Call {
    func: g_id,
    args: vec![],
    k_next: Some(k_cont2),
    dst: Some(t2_vid),
});

// k_cont2: Lower x = _t1 + _t2 (pure)
body.push(JoinInst::Compute(MirLikeInst::BinOp {
    dst: x_vid,
    op: BinaryOp::Add,
    lhs: t1_vid,  // ANF temporary
    rhs: t2_vid,  // ANF temporary
}));
```

### 5.4 ValueId Allocation Strategy

**ANF temporary の ValueId 割り当て**:

- **Region**: Local region (1000+) を使用
- **SSOT**: `NormalizedHelperBox::alloc_value_id(&mut next_value_id)`
- **Lifetime**: JoinInst の scope 内でのみ有効

**Example**:

```rust
// Allocate ANF temporary for f()
let t1_vid = NormalizedHelperBox::alloc_value_id(&mut next_value_id);
// → ValueId(1001)

// Allocate ANF temporary for g()
let t2_vid = NormalizedHelperBox::alloc_value_id(&mut next_value_id);
// → ValueId(1002)

// Use temporaries in pure expression
body.push(JoinInst::Compute(MirLikeInst::BinOp {
    dst: result_vid,
    op: BinaryOp::Add,
    lhs: t1_vid,  // ValueId(1001)
    rhs: t2_vid,  // ValueId(1002)
}));
```

**注意（Caution）**:

- ANF temporary は **SSA form** を維持（1回のみ代入）
- PHI 命令は生成しない（Normalized JoinIR は PHI-free）

---

## 6. Diagnostic Strategy (Strict Mode)

### 6.1 Diagnostic Tags (SSOT)

**診断タグの設計（Diagnostic Tag Design）**:

ANF 関連のエラーは `[joinir/anf/*]` タグで統一する。

**Tag family**:

1. **`[joinir/anf/order_violation]`**: 非 ANF 式検出（impure in immediate position）
2. **`[joinir/anf/pure_required]`**: Impure 式が pure-only scope に出現
3. **`[joinir/anf/hoist_failed]`**: Loop/If condition の hoist 失敗

**実装場所（Implementation Location）**:

`src/mir/join_ir/lowering/error_tags.rs` に追加（Phase 145+）

**Signature（Conceptual）**:

```rust
/// ANF order violation - Impure expression in immediate position
///
/// Used when an impure expression appears in an immediate position
/// (e.g., binary operation operand, if condition, loop condition).
///
/// # Example
/// ```rust,ignore
/// return Err(error_tags::anf_order_violation(
///     "f() + g()",
///     "impure subexpression f() not hoisted"
/// ));
/// // Output: "[joinir/anf/order_violation] f() + g(): impure subexpression f() not hoisted"
/// ```
pub fn anf_order_violation(expr: &str, reason: &str) -> String {
    format!("[joinir/anf/order_violation] {}: {}", expr, reason)
}

/// ANF pure required - Impure expression in pure-only scope
///
/// Used when an impure expression appears in a pure-only scope
/// (e.g., loop condition, if condition with PureOnly scope).
///
/// # Example
/// ```rust,ignore
/// return Err(error_tags::anf_pure_required(
///     "iter.hasNext()",
///     "impure expression in loop condition (pure-only scope)"
/// ));
/// // Output: "[joinir/anf/pure_required] iter.hasNext(): impure expression in loop condition (pure-only scope)"
/// ```
pub fn anf_pure_required(expr: &str, reason: &str) -> String {
    format!("[joinir/anf/pure_required] {}: {}", expr, reason)
}

/// ANF hoist failed - Loop/If condition hoist failed
///
/// Used when ANF transformation fails to hoist an impure expression
/// from a loop/if condition.
///
/// # Example
/// ```rust,ignore
/// return Err(error_tags::anf_hoist_failed(
///     "loop",
///     "iter.hasNext()",
///     "complex nested call cannot be hoisted"
/// ));
/// // Output: "[joinir/anf/hoist_failed] loop(iter.hasNext()): complex nested call cannot be hoisted"
/// ```
pub fn anf_hoist_failed(construct: &str, expr: &str, reason: &str) -> String {
    format!("[joinir/anf/hoist_failed] {}({}): {}", construct, expr, reason)
}
```

### 6.2 Verification Points

**検証ポイント（Verification Points）**:

ANF 契約を以下のポイントで検証する：

**Point 1: ExprLowererBox (Pure-only scope)**

```rust
// Location: src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs
// Phase 145+

pub fn lower_expr_with_scope(
    scope: ExprLoweringScope,
    expr: &ASTNode,
    env: &BTreeMap<String, ValueId>,
    body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
) -> Result<Option<ValueId>, String> {
    match scope {
        ExprLoweringScope::PureOnly => {
            // ✅ Verification: impure 式を検出したら fail-fast
            if is_impure_expr(expr) {
                if crate::config::env::anf_strict_enabled() {
                    return Err(error_tags::anf_pure_required(
                        &expr.to_string(),
                        "impure expression in pure-only scope"
                    ));
                } else {
                    return Ok(None);  // Graceful fallback
                }
            }
        }
        ExprLoweringScope::AllowImpure => {
            // ANF 変換を試みる
        }
    }
    // ... Pure 式の lowering ...
}
```

**Point 2: BinaryOp lowering (Immediate position check)**

```rust
// Location: src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs
// Phase 145+

fn lower_binop(
    op: BinaryOp,
    lhs: &ASTNode,
    rhs: &ASTNode,
    env: &BTreeMap<String, ValueId>,
    body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
) -> Result<Option<ValueId>, String> {
    // ✅ Verification: LHS/RHS が impure なら hoist
    let lhs_vid = if is_impure_expr(lhs) {
        // Hoist LHS
        let t_vid = NormalizedHelperBox::alloc_value_id(next_value_id);
        lower_impure_expr(lhs, env, body, next_value_id, Some(t_vid))?;
        t_vid
    } else {
        // Pure: direct lowering
        lower_expr(lhs, env, body, next_value_id)?.unwrap()
    };

    let rhs_vid = if is_impure_expr(rhs) {
        // Hoist RHS
        let t_vid = NormalizedHelperBox::alloc_value_id(next_value_id);
        lower_impure_expr(rhs, env, body, next_value_id, Some(t_vid))?;
        t_vid
    } else {
        // Pure: direct lowering
        lower_expr(rhs, env, body, next_value_id)?.unwrap()
    };

    // Generate BinOp with hoisted operands
    let dst_vid = NormalizedHelperBox::alloc_value_id(next_value_id);
    body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: dst_vid,
        op,
        lhs: lhs_vid,
        rhs: rhs_vid,
    }));

    Ok(Some(dst_vid))
}
```

**Point 3: Loop condition hoist (Preheader generation)**

```rust
// Location: src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
// Phase 146+

fn hoist_loop_condition(
    cond_ast: &ASTNode,
    env: &BTreeMap<String, ValueId>,
    preheader_body: &mut Vec<JoinInst>,
    latch_body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
) -> Result<ValueId, String> {
    // ✅ Verification: impure 式なら hoist
    if is_impure_expr(cond_ast) {
        if crate::config::env::anf_strict_enabled() {
            return Err(error_tags::anf_hoist_failed(
                "loop",
                &cond_ast.to_string(),
                "impure expression in loop condition requires hoisting"
            ));
        } else {
            // Graceful fallback: out-of-scope
            eprintln!("[anf/debug] Loop condition hoist failed, fallback to legacy");
            return Err("out-of-scope".to_string());
        }
    }

    // Generate condition evaluation in preheader
    let cond_vid = lower_expr(cond_ast, env, preheader_body, next_value_id)?;

    // Copy condition evaluation to latch (for re-evaluation)
    latch_body.push(/* ... re-evaluate condition ... */);

    Ok(cond_vid)
}
```

### 6.3 Strict Mode Contract

**環境変数（Environment Variable）**:

```bash
# Strict mode: ANF 違反で fail-fast
HAKO_ANF_STRICT=1

# Default: ANF 違反で graceful fallback (out-of-scope)
# (環境変数未設定)
```

**実装（Implementation）**:

```rust
// Location: src/config/env/anf_flags.rs (Phase 145+ で新規作成)

/// Check if ANF strict mode is enabled
///
/// Strict mode: ANF violations cause compilation errors (fail-fast)
/// Default: ANF violations fall back to legacy lowering (graceful)
pub fn anf_strict_enabled() -> bool {
    std::env::var("HAKO_ANF_STRICT")
        .map(|v| v == "1")
        .unwrap_or(false)
}
```

**Usage**:

```rust
if crate::config::env::anf_strict_enabled() {
    // Fail-fast: ANF 違反でエラー
    return Err(error_tags::anf_order_violation(...));
} else {
    // Graceful fallback: out-of-scope
    return Ok(None);
}
```

**診断例（Diagnostic Examples）**:

**Example 1: Impure in immediate position**

```bash
$ HAKO_ANF_STRICT=1 ./target/release/hakorune test.hako

[ERROR] ❌ MIR compilation error:
[joinir/anf/order_violation] f() + g(): impure subexpression f() not hoisted

Hint: Split into multiple statements:
  local _t1 = f()
  local _t2 = g()
  local result = _t1 + _t2
```

**Example 2: Impure in pure-only scope**

```bash
$ HAKO_ANF_STRICT=1 ./target/release/hakorune test.hako

[ERROR] ❌ MIR compilation error:
[joinir/anf/pure_required] iter.hasNext(): impure expression in loop condition (pure-only scope)

Hint: Hoist condition to loop preheader:
  local _cond = iter.hasNext()
  loop(_cond) {
    ...
    _cond = iter.hasNext()
  }
```

**Example 3: Hoist failed**

```bash
$ HAKO_ANF_STRICT=1 ./target/release/hakorune test.hako

[ERROR] ❌ MIR compilation error:
[joinir/anf/hoist_failed] loop(f(g(), h())): complex nested call cannot be hoisted

Hint: Simplify nested calls:
  local _t1 = g()
  local _t2 = h()
  local _cond = f(_t1, _t2)
  loop(_cond) { ... }
```

---

## 7. Implementation Roadmap

### 7.1 Phase 145: ANF Transformation Core

**目標（Goal）**:

BinaryOp with impure operands の ANF 変換を実装する。

**Scope**:

- `x = f() + g()` → ANF 変換
- Left-to-right evaluation order 保証
- Hoist strategy 実装

**Implementation tasks**:

1. **`is_impure_expr()` helper** (1 file)
   - Location: `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
   - 役割: AST ノードが impure かを判定
   - 実装: Call/MethodCall を impure として分類

2. **`lower_binop_with_anf()` core** (1 file)
   - Location: `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
   - 役割: BinaryOp の ANF 変換
   - 実装: LHS/RHS が impure なら hoist

3. **ANF diagnostic tags** (1 file)
   - Location: `src/mir/join_ir/lowering/error_tags.rs`
   - 役割: ANF 診断タグを追加
   - 実装: `anf_order_violation()`, `anf_pure_required()`, `anf_hoist_failed()`

4. **ANF strict mode flag** (1 file)
   - Location: `src/config/env/anf_flags.rs` (新規)
   - 役割: `HAKO_ANF_STRICT` を読む
   - 実装: `anf_strict_enabled()` 関数

5. **Test fixtures** (2 files)
   - `apps/tests/phase145_anf_binop_impure_min.hako` (exit code 15)
   - `apps/tests/phase145_anf_binop_nested_min.hako` (exit code 16)

6. **Smoke tests** (4 files)
   - VM/LLVM variants for each fixture

**Acceptance criteria**:

- ✅ `x = f() + g()` が ANF 変換される（f() → g() → +）
- ✅ Left-to-right order が保証される
- ✅ Strict mode で ANF 違反が fail-fast
- ✅ Default mode で graceful fallback
- ✅ Test fixtures が VM/LLVM で pass

### 7.2 Phase 146: Loop Condition Hoisting

**目標（Goal）**:

Loop condition が impure な場合の hoist を実装する。

**Scope**:

- `loop(iter.hasNext()) { ... }` → ANF 変換
- Preheader + latch での評価
- Pure-only scope での fail-fast

**Implementation tasks**:

1. **`hoist_loop_condition()` core** (1 file)
   - Location: `src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs`
   - 役割: Loop condition の ANF 変換
   - 実装: Preheader で評価、latch で再評価

2. **Preheader generation** (1 file)
   - Location: `src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs`
   - 役割: Preheader block を生成
   - 実装: JoinFunction で preheader を追加

3. **Latch re-evaluation** (1 file)
   - Location: `src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs`
   - 役割: Latch での条件再評価
   - 実装: Loop body 末尾で条件を再計算

4. **Test fixtures** (2 files)
   - `apps/tests/phase146_anf_loop_cond_impure_min.hako` (exit code 17)
   - `apps/tests/phase146_anf_loop_cond_nested_min.hako` (exit code 18)

5. **Smoke tests** (4 files)
   - VM/LLVM variants for each fixture

**Acceptance criteria**:

- ✅ `loop(f()) { ... }` が ANF 変換される（preheader + latch）
- ✅ Preheader で1回目の評価
- ✅ Latch で2回目以降の評価
- ✅ Pure-only scope で impure 検出
- ✅ Test fixtures が VM/LLVM で pass

### 7.3 Phase 147: If Condition ANF

**目標（Goal）**:

If condition が impure な場合の hoist を実装する。

**Scope**:

- `if f() { ... } else { ... }` → ANF 変換
- 条件評価の先行
- Pure-only scope での fail-fast

**Implementation tasks**:

1. **`hoist_if_condition()` core** (1 file)
   - Location: `src/mir/control_tree/normalized_shadow/post_if_post_k.rs`
   - 役割: If condition の ANF 変換
   - 実装: 条件を先に評価、結果を変数に束縛

2. **Condition pre-evaluation** (1 file)
   - Location: `src/mir/control_tree/normalized_shadow/post_if_post_k.rs`
   - 役割: If より前に条件評価
   - 実装: JoinFunction で条件評価ブロックを追加

3. **Test fixtures** (2 files)
   - `apps/tests/phase147_anf_if_cond_impure_min.hako` (exit code 19)
   - `apps/tests/phase147_anf_if_cond_nested_min.hako` (exit code 20)

4. **Smoke tests** (4 files)
   - VM/LLVM variants for each fixture

**Acceptance criteria**:

- ✅ `if f() { ... }` が ANF 変換される（条件先行評価）
- ✅ 条件評価が if の前に実行される
- ✅ Pure-only scope で impure 検出
- ✅ Test fixtures が VM/LLVM で pass

### 7.4 Phase 148+: NewBox/ExternCall ANF (Future Work)

**Scope**:

- `new SomeBox()` の ANF 対応
- `print(f())` 等の ExternCall の ANF 対応

**Implementation outline**:

- NewBox を impure として分類
- ExternCall を impure として分類
- Hoist strategy を適用

**Note**: Phase 148+ は Phase 147 完了後に計画を詳細化する。

---

## 8. Acceptance Criteria

### 8.1 Design Acceptance (Phase 144-anf)

**このドキュメント（Phase 144-anf）の完成条件**:

- ✅ **ANF 定義** with examples（Section 2）
  - Pure vs Impure の分類
  - 評価順序の規則（left-to-right, depth-first）
  - Hoist strategy の原則

- ✅ **Problem scenarios**（Section 4）
  - Observable side effects（3+ シナリオ）
  - Exception ordering（2+ シナリオ）
  - Resource acquisition（2+ シナリオ）

- ✅ **ANF contract**（Section 5）
  - Expression classification
  - Evaluation order rules
  - JoinIR lowering contract

- ✅ **Diagnostic tag design**（Section 6）
  - 3+ tags（order_violation, pure_required, hoist_failed）
  - Verification points
  - Strict mode contract

- ✅ **Implementation roadmap**（Section 7）
  - 3+ phases（Phase 145-147）
  - Concrete tasks per phase
  - Acceptance criteria per phase

- ✅ **References** accurate（Section 10）
  - ExprLowerer SSOT
  - error_tags patterns
  - JoinIR invariants

- ✅ **Out-of-scope handling**（Section 9）
  - `Ok(None)` fallback strategy
  - Graceful degradation

### 8.2 Implementation Acceptance (Phase 145-147)

**Phase 145（BinaryOp ANF）の完成条件**:

- ✅ `is_impure_expr()` 実装完了
- ✅ `lower_binop_with_anf()` 実装完了
- ✅ ANF diagnostic tags 追加完了
- ✅ `anf_strict_enabled()` 実装完了
- ✅ Test fixtures 2個 + smoke tests 4個 作成完了
- ✅ All smoke tests pass (VM + LLVM)
- ✅ Strict mode で ANF 違反が fail-fast
- ✅ Default mode で graceful fallback

**Phase 146（Loop Condition Hoist）の完成条件**:

- ✅ `hoist_loop_condition()` 実装完了
- ✅ Preheader generation 実装完了
- ✅ Latch re-evaluation 実装完了
- ✅ Test fixtures 2個 + smoke tests 4個 作成完了
- ✅ All smoke tests pass (VM + LLVM)
- ✅ Pure-only scope で impure 検出

**Phase 147（If Condition ANF）の完成条件**:

- ✅ `hoist_if_condition()` 実装完了
- ✅ Condition pre-evaluation 実装完了
- ✅ Test fixtures 2個 + smoke tests 4個 作成完了
- ✅ All smoke tests pass (VM + LLVM)
- ✅ Pure-only scope で impure 検出

---

## 9. Out-of-Scope Handling

### 9.1 Graceful Fallback Strategy

**原則（Principle）**:

ANF 変換が失敗した場合、**サイレント退避は禁止**し、**`Ok(None)` で out-of-scope** として扱う。

**フォールバックの分類（Fallback Classification）**:

| Fallback Type | Allowed? | Logging | Example |
|---------------|----------|---------|---------|
| **Soft fallback** | ✅ 許容 | Debug log 必須 | ANF 変換失敗 → legacy lowering |
| **Prohibited fallback** | ❌ 禁止 | - | サイレント退避、契約違反の握りつぶし |

**Soft fallback の条件**:

1. **`Ok(None)` を返す**（out-of-scope を明示）
2. **Debug log を出力**（理由を記録）
3. **Strict mode では fail-fast**（`HAKO_ANF_STRICT=1`）

**実装例（Conceptual）**:

```rust
pub fn try_lower_with_anf(
    expr: &ASTNode,
    scope: ExprLoweringScope,
    env: &BTreeMap<String, ValueId>,
    body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
) -> Result<Option<ValueId>, String> {
    // Step 1: Detect impure expression
    if is_impure_expr(expr) {
        match scope {
            ExprLoweringScope::PureOnly => {
                // Pure-only scope: fail-fast or out-of-scope
                if crate::config::env::anf_strict_enabled() {
                    return Err(error_tags::anf_pure_required(
                        &expr.to_string(),
                        "impure expression in pure-only scope"
                    ));
                } else {
                    // Graceful fallback: out-of-scope
                    eprintln!("[anf/debug] Pure-only scope violation, fallback to legacy");
                    return Ok(None);
                }
            }
            ExprLoweringScope::AllowImpure => {
                // Try ANF transformation
                match try_anf_transform(expr, env, body, next_value_id) {
                    Ok(vid) => return Ok(Some(vid)),
                    Err(e) => {
                        if crate::config::env::anf_strict_enabled() {
                            return Err(e);
                        } else {
                            // Graceful fallback: legacy lowering
                            eprintln!("[anf/debug] ANF transformation failed: {}", e);
                            eprintln!("[anf/debug] Fallback to legacy lowering");
                            return Ok(None);
                        }
                    }
                }
            }
        }
    }

    // Step 2: Pure expression lowering (existing logic)
    // ... existing pure lowering code ...
}
```

### 9.2 Debug Logging

**Debug log の形式**:

```bash
[anf/debug] {context}: {reason}
```

**Examples**:

```bash
[anf/debug] Pure-only scope violation, fallback to legacy
[anf/debug] ANF transformation failed: complex nested call cannot be hoisted
[anf/debug] Fallback to legacy lowering
```

**実装場所（Implementation Location）**:

- ExprLowererBox の各メソッド内
- Strict mode check の前後

**条件（Condition）**:

- `HAKO_JOINIR_DEBUG=1` または `NYASH_JOINIR_DEV=1` で出力

### 9.3 Legacy Lowering Compatibility

**原則（Principle）**:

ANF 変換が失敗しても、**既定挙動不変**（legacy lowering で動作）を維持する。

**Legacy lowering の責務**:

- ANF 非対応の式を処理
- 評価順序は **実装依存**（non-deterministic）
- 将来的には ANF に移行する（Phase 150+ で完全置き換え）

**実装状況（Implementation Status）**:

- Phase 140 時点: Pure 式のみ対応（`NormalizedExprLowererBox`）
- Phase 145+: Impure 式の ANF 変換を段階投入
- Phase 150+: Legacy lowering を段階的に削除（ANF 完全移行）

**移行戦略（Migration Strategy）**:

1. **Phase 145-147**: ANF 変換を opt-in（default: legacy）
2. **Phase 148-149**: ANF 変換を default（legacy は fallback）
3. **Phase 150+**: Legacy lowering を削除（ANF 必須）

---

## 10. References

### 10.1 Internal Documentation

**設計図（Design SSOT）**:

- **JoinIR Architecture Overview**:
  `docs/development/current/main/joinir-architecture-overview.md`
  不変条件（Invariants）、箱の責務、Fail-Fast 原則

- **JoinIR Design Map**:
  `docs/development/current/main/design/joinir-design-map.md`
  実装導線の地図（どのファイルを触るか）

- **Normalized Expression Lowering**:
  `docs/development/current/main/design/normalized-expr-lowering.md`
  ExprLowererBox SSOT、Pure expression lowering

- **Docs Layout**:
  `docs/development/current/main/DOCS_LAYOUT.md`
  ドキュメント配置ルール（Phase/design/investigations）

**実装 SSOT**:

- **ExprLowererBox**:
  `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
  Pure expression lowering core

- **ExprLowering Contract**:
  `src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs`
  ExprLoweringScope, OutOfScopeReason

- **Error Tags**:
  `src/mir/join_ir/lowering/error_tags.rs`
  診断タグ生成の SSOT

- **Environment Variables**:
  `docs/reference/environment-variables.md`
  HAKO_JOINIR_DEBUG, NYASH_JOINIR_DEV, etc.

### 10.2 External References

**A-Normal Form（ANF）**:

- Flanagan, C., Sabry, A., Duba, B. F., & Felleisen, M. (1993).
  "The Essence of Compiling with Continuations"
  PLDI '93: Proceedings of the ACM SIGPLAN 1993 conference on Programming language design and implementation
  https://dl.acm.org/doi/10.1145/155090.155113

- Sabry, A., & Felleisen, M. (1992).
  "Reasoning about Programs in Continuation-Passing Style"
  LISP and Symbolic Computation, 6(3-4), 289-360
  https://doi.org/10.1007/BF01019462

**Evaluation Order Specifications**:

- JavaScript ECMAScript Specification:
  "12.15.4 Runtime Semantics: Evaluation" (Left-to-right order)
  https://tc39.es/ecma262/

- Python Language Reference:
  "6.3.1 Evaluation order" (Left-to-right order)
  https://docs.python.org/3/reference/expressions.html#evaluation-order

- Rust Reference:
  "Evaluation order and side-effects" (Sequenced before)
  https://doc.rust-lang.org/reference/expressions.html#evaluation-order-of-operands

**Static Single Assignment (SSA)**:

- Cytron, R., Ferrante, J., Rosen, B. K., Wegman, M. N., & Zadeck, F. K. (1991).
  "Efficiently computing static single assignment form and the control dependence graph"
  ACM Transactions on Programming Languages and Systems (TOPLAS), 13(4), 451-490
  https://doi.org/10.1145/115372.115320

### 10.3 Related Phases

**Phase 140（Pure Expression Lowering）**:

- ExprLowererBox 導入
- Pure-only scope の実装
- Out-of-scope handling の確立

**Phase 141 P1.5（Known Intrinsic SSOT）**:

- `KnownIntrinsicRegistryBox` 実装
- MethodCall の out-of-scope 理由精密化
- `OutOfScopeReason::IntrinsicNotWhitelisted` 追加

**Phase 143 P2（Else Symmetry）**:

- `loop(true){ if(cond){break}else{continue} }` 対応
- 4-way match（B-C, C-B, B-B, C-C）
- Condition inversion（`UnaryOp::Not`）

**Phase 145-147（ANF Implementation）**:

- Phase 145: BinaryOp ANF
- Phase 146: Loop condition hoist
- Phase 147: If condition ANF

---

## 11. Revision History

**2025-12-19（Phase 144-anf Initial Draft）**:

- ANF の定義・契約・診断タグを SSOT として確立
- Problem scenarios（副作用・例外・リソース）を明確化
- Diagnostic strategy（Strict mode, Fail-Fast）を設計
- Implementation roadmap（Phase 145-147）を作成
- Out-of-scope handling（Graceful fallback）を規定
- References（Internal/External）を整理

**変更履歴の管理（Change Management）**:

- このファイルは **SSOT** として扱う（Phase ログより優先）
- 変更があった場合は Revision History に記録
- 実装（Phase 145+）で契約に変更が必要な場合は、このファイルを更新してから実装に着手

**関連 Phase ドキュメント**:

- Phase 145: `docs/development/current/main/phases/phase-145-anf-binop/README.md`（作成予定）
- Phase 146: `docs/development/current/main/phases/phase-146-anf-loop/README.md`（作成予定）
- Phase 147: `docs/development/current/main/phases/phase-147-anf-if/README.md`（作成予定）

---

## Appendix A: ANF Transformation Examples

### A.1 Simple Binary Operation

**Source**:
```hako
x = f() + g()
```

**ANF**:
```hako
local _t1 = f()
local _t2 = g()
x = _t1 + _t2
```

**JoinIR（Conceptual）**:
```
main():
  t1 = Call f()
  t2 = Call g()
  x = BinOp(Add, t1, t2)
  Return
```

### A.2 Nested Calls

**Source**:
```hako
x = f(g(), h())
```

**ANF**:
```hako
local _t1 = g()
local _t2 = h()
x = f(_t1, _t2)
```

**JoinIR（Conceptual）**:
```
main():
  t1 = Call g()
  t2 = Call h()
  x = Call f(t1, t2)
  Return
```

### A.3 Method Chain

**Source**:
```hako
x = obj.method1().method2()
```

**ANF**:
```hako
local _t1 = obj.method1()
x = _t1.method2()
```

**JoinIR（Conceptual）**:
```
main():
  t1 = MethodCall obj.method1()
  x = MethodCall t1.method2()
  Return
```

### A.4 Loop Condition Hoist

**Source**:
```hako
loop(iter.hasNext()) {
    process()
}
```

**ANF**:
```hako
local _cond = iter.hasNext()
loop(_cond) {
    process()
    _cond = iter.hasNext()
}
```

**JoinIR（Conceptual）**:
```
main():
  cond = MethodCall iter.hasNext()  // Preheader
  Jump loop_header(cond)

loop_header(cond_param):
  Branch cond_param, loop_body, exit

loop_body():
  Call process()
  cond_new = MethodCall iter.hasNext()  // Latch
  Jump loop_header(cond_new)

exit():
  Return
```

### A.5 If Condition Hoist

**Source**:
```hako
if f() {
    doThen()
} else {
    doElse()
}
```

**ANF**:
```hako
local _cond = f()
if _cond {
    doThen()
} else {
    doElse()
}
```

**JoinIR（Conceptual）**:
```
main():
  cond = Call f()
  Branch cond, then_block, else_block

then_block():
  Call doThen()
  Jump exit

else_block():
  Call doElse()
  Jump exit

exit():
  Return
```

---

## Appendix B: Diagnostic Message Examples

### B.1 Order Violation

**Code**:
```hako
x = f() + g()
```

**Error**:
```
[ERROR] ❌ MIR compilation error:
[joinir/anf/order_violation] f() + g(): impure subexpression f() not hoisted

Hint: Split into multiple statements:
  local _t1 = f()
  local _t2 = g()
  local result = _t1 + _t2
```

### B.2 Pure Required

**Code**:
```hako
loop(iter.hasNext()) {
    process()
}
```

**Error**:
```
[ERROR] ❌ MIR compilation error:
[joinir/anf/pure_required] iter.hasNext(): impure expression in loop condition (pure-only scope)

Hint: Hoist condition to loop preheader:
  local _cond = iter.hasNext()
  loop(_cond) {
    process()
    _cond = iter.hasNext()
  }
```

### B.3 Hoist Failed

**Code**:
```hako
loop(f(g(), h())) {
    process()
}
```

**Error**:
```
[ERROR] ❌ MIR compilation error:
[joinir/anf/hoist_failed] loop(f(g(), h())): complex nested call cannot be hoisted

Hint: Simplify nested calls:
  local _t1 = g()
  local _t2 = h()
  local _cond = f(_t1, _t2)
  loop(_cond) {
    process()
    _cond = f(_t1, _t2)
  }
```

---

**End of Phase 144-anf INSTRUCTIONS.md**
