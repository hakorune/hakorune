# ADR-004: 失敗の書き方 — `Result` を 1 つの道にして、`catch` と `cleanup` の形は残す

Status: 採用(2026-11-16、ユーザー決定)。実装は、まだ無い。
Date: 2026-11-16
Scope: 回復できる失敗の表し方、後ろに付ける `catch`・`cleanup` の扱い、捨てた `Result` の扱い、古い例外の形の実装の退役の順。

[language-result-propagation-and-exit-transaction-ssot.md](../current/main/design/language-result-propagation-and-exit-transaction-ssot.md)(決定 `LANGUAGE-RESULT-EXIT-C-PRIME0-D0`、2026-08-05。以下 C′)の一部を、この ADR が直す。C′ の実行の形(`Result`、`?`、1 つの出口の手順、Fault)は、そのまま使う。直すのは、「source の `catch` と、後ろに付ける `cleanup` を、v1 に入れない」という所。

## なぜ決め直したか

- 掃除の中で、「古い `try/catch/throw` を MIR に下ろす code を消す」が候補に挙がった。調べると、C′ が「`catch` は v1 に入れない」と決めていた。
- メンテナは、この決定を覚えていなかった。「`try` を書かずに、block や method の後ろに `catch` と `cleanup` を付ける」形は、メンテナがこの言語の特徴として考えた物である。
- 2 つの相談の答え(2026-11-16)は、同じ事を言った: **C′ が捨てる必要があったのは、中身(例外。`Result` でも Fault でもない 3 つ目の失敗の道)で、形(後ろに付ける書き方)ではない。** 形は、`Result` の上に定義し直せる。

## 決めた事(ユーザー決定 2026-11-16)

1. **回復できる失敗の道は、`Result<T, E>` の 1 つ。** `throw`、例外、`RecoverableFailure` は戻さない。失敗を起こすのは `return Result::Err(e)`、そのまま上へ渡すのは後ろに付ける `?`(C′ のまま)。回復できない失敗は Fault(`panic` など)で、`catch` では受けられない。
2. **後ろに付ける `catch` と `cleanup` は、この言語に残す。**
   - `catch` は、「`Result::Err` を受けて直す書き方」として定義し直す。例外を捕まえる物ではない。失敗は勝手には上がらない。`?` を書いた所と、`return` した `Err` だけが、`catch` に届く。
   - `cleanup` は、片付け。成功でも失敗でも走る。単独の文 `cleanup { ... }`(C′ のまま)が基本で、method と block の後ろに付ける形も残す。
   - 2 つは役目が違う(`catch` は直す、`cleanup` は片付ける)。片方で、もう片方を代用しない。
   - **作るのは、`?` と単独の `cleanup` と出口の手順が動いた後。** それまで、後ろに付ける形は、名前付きの拒否にする(綴りは予約)。
   - 細かい決まりは、作る時に決める(下の「まだ決めていない事」)。
3. **捨てた `Result` は、compile error にする。** 例外は、誰も受けなければ上へ伝わる。`Result` は、誰も見なければ黙って消える。式の文で `Result` の値を捨てる事を、認めない。捨てる時は、その意思が source で分かる書き方をする(書き方は、`?` の回で決める)。
4. **古い例外の形の実装は、この ADR を書いた後に、1 つの family として消す回を作る。** 掃除の回 5 では触らない。
   - 相手は、builder の下ろし方(約 280 行)だけではない。`MirInstruction` の `Catch`・`Throw`、`TryCatch` に触れる Rust の file、`.hako` 側の parser と handler、古い `try/catch` と後ろの `cleanup` を使う test を、まとめて扱う。
   - 古い実装は、新しい形の土台にしない(下ろし方に Fault の辺が無い。C′ の `cleanup` の意味を持たない)。
   - C′ の「この作業が止まっている間は、実装の退役を認めない」は、この ADR で外す。ただし、消す回は、メンテナが範囲を見てから始める。
5. **言語の文書にある、「ユーザー決定」の印の無い決定を、一覧にして、メンテナが 1 回見る。** 言語の表に出る物(語、文法、利用者に見える動き)を足す・消す決定には、これから、誰が決めたかを書く。

## 形の見込み(説明の例。まだ動かない)

```hako
// 失敗を起こす: Err を返す
read(path: String): Result<Data, String> {
    if !exists(path) { return Result::Err("not found") }
    ...
}

// 後ろに付ける catch と、単独の cleanup
load(path: String): Result<Data, String> {
    local file = File.open(path)?      // 失敗したら、下の catch へ
    cleanup { file.close() }           // 必ず走る片付け
    return file.read()?
} catch (e) {
    log("load failed: " + e)
    return Result::Ok(Data.empty())
}

// 式の後ろに付ける catch(一番小さい形)
local count = parse_count(text) catch (e) { 0 }
```

| 語 | 役目 | いつ走るか |
|---|---|---|
| `return Result::Err(e)` | 失敗を起こす | — |
| `?` | そのまま上(または下の `catch`)へ渡す | `Err` の時 |
| `catch` | 失敗を直す | 失敗した時だけ |
| `cleanup` | 片付ける | 成功でも失敗でも |
| `panic(...)` | 止まる(Fault) | 捕まえられない |

## まだ決めていない事(作る時に決める)

- method の後ろの `catch` を、`Result` を返さない method にも付けられるか(付けられるなら、その method の中の `?` の行き先は、その `catch` になる)。最初は、式の後ろと、`Result` を返す method からでよい。
- block の後ろの `catch`(`?` の行き先を、関数の出口ではなく block の終わりにする、新しい出口が要る)。
- handler から見える名前(引数と `me` と error は見える。本体の local は見せない、が案)。handler 自身の失敗を、同じ `catch` で受け直さない事。
- 後ろの `cleanup` から見える名前(外側の束縛・引数・`me` だけ、が案。本体の中で作った物は、作った直後の単独の `cleanup` で片付ける)。本体 → `catch` → 後ろの `cleanup` の順。
- `local x = e cleanup { ... }` と、式の後ろの `cleanup`: 残さない方向(2 つの答えとも)。
- 本体に `?` が 1 つも無いのに `catch` が後ろにある時を、compile error にするか。
- `catch` という語が、ほかの言語の例外の意味を思わせる事への手当て(診断の文、文書)。
- error の型の変換(C′ は「完全に同じ `E`、暗黙の変換なし」)。層の境目での書き方。
- `birth` は `Result` を返せない。失敗しうる構築は、`Result` を返す static の関数に書く、という書き方の決まり。
- Fault を止める場所(今は process。後で task の境目)。`catch` を Fault へ広げない。
- 失敗の時だけ走る片付け(`errdefer` に当たる物)は、今は入れない。

## 作業の順

掃除と selfhost の順(ADR-003 の「作業の順」)の中に置く。この ADR のためだけに、順を前へ出さない。

1. この ADR と、C′ の文書への注記(済み)。
2. 印の無い決定の一覧を、メンテナが見る。
3. 古い例外の形の family を消す回(メンテナが範囲を見てから)。後ろに付ける形は、名前付きの拒否になる。
4. `?` の回(C′ の順)。`Result` を返す関数、型付きの `?`、単独の `cleanup`、出口の手順、捨てた `Result` の拒否。
5. 後ろに付ける `catch` と `cleanup` を作る。式の後ろと、`Result` を返す method から。

## 根拠

- 相談と 2 つの答えは、公開の repo の外にある(2026-11-16)。要点は、上の「なぜ決め直したか」と「まだ決めていない事」に入れた。
- 確かめた事: `.hako` のコンパイラは、`try`・`catch`・`throw`・`cleanup` を使っていない。古い形は、今の既定の道では拒否される(試した 7 本)。`?`・`Result` を返す関数・単独の `cleanup` も、今の既定の道では拒否される。
- 確かめていない事: 後ろに付ける形を `Result` の上に定義できる、は紙の上の話で、作っていない。
