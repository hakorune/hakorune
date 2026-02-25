# Phase 25.1p — MIR DebugLog 命令（構造設計メモ）

Status: planning（まだ実装しない。設計＋用途整理だけ）

## ねらい

- Rust / VM / LLVM すべての経路で共通に使える「MIR レベルのデバッグログ命令」を用意して、
  - SSA ValueId の中身（どのブロックでどの値が入っているか）
  - Loop/If ヘッダ時点のキャリア変数の状態
  - BoxCall / MethodCall の receiver や args の実際の値
  を簡単に観測できるようにする。
- 既存のポリシー:
  - 仕様変更や挙動変更ではなく、「観測レイヤ（デバッグログ）」として追加する。
  - 既定では完全に OFF。環境変数で opt-in したときだけログが出るようにする。

## 構造案（Rust 側）

### 1. MIR 命令の拡張

- ファイル: `src/mir/instruction.rs`
- 追加案:

```rust
pub enum MirInstruction {
    // 既存の MIR 命令 ...

    /// Debug logging instruction (dev-only)
    /// 実行時にログ出力（VM/LLVM 共通の観測ポイント）
    DebugLog {
        message: String,
        values: Vec<ValueId>,  // ログに出したい SSA 値
    },
}
```

- SSA/Verifier との整合:
  - `DebugLog` は **新しい値を定義しない**（`dst_value()` は None）。
  - `used_values()` は `values` をそのまま返す。
  - これにより:
    - SSA check（MultipleDefinition/UndefinedValue）は既存のロジックのままでよい。
    - MergeUses/Dominator も「普通の値読み」として扱える。

### 2. VM 実行器での実装

- ファイル: `src/backend/mir_interpreter/exec.rs`（または handlers 側）
- 方針:
  - `NYASH_MIR_DEBUG_LOG=1` のときだけ効果を持つ。
  - それ以外のときは no-op（完全に挙動不変）。
- 擬似コード:

```rust
MirInstruction::DebugLog { message, values } => {
    if std::env::var("NYASH_MIR_DEBUG_LOG").ok().as_deref() != Some("1") {
        continue;
    }
    eprint!("[MIR-LOG] {}", message);
    for vid in values {
        let val = self.regs.get(vid).cloned().unwrap_or(VMValue::Void);
        eprint!(" %{} = {:?}", vid.0, val);
    }
    eprintln!();
}
```

### 3. Printer / Verifier での扱い

- `src/mir/printer.rs`:
  - Debug 出力用に:
    ```text
    debug_log "msg" %1 %2 %3
    ```
    のように印字。
- `src/mir/verification/*`:
  - `DebugLog` は「副作用ありだが、新しい値は定義しない」命令として扱う。
  - `used_values()` による UndefinedValue チェックの対象にはなるが、新しいエラー種別は不要。

## Hako 側インターフェース案

### 1. 簡易マクロ（糖衣）: `__debug_log__`

- 目的:
  - Stage‑B / Stage‑1 / selfhost の .hako コードから簡単に DebugLog を差し込めるようにする。
- 例:

```hako
_build_module_map() {
  local map = new MapBox()
  __debug_log__("me before call", me)
  __debug_log__("map before call", map)
  me._push_module_entry(map, seg)
  __debug_log__("map after call", map)
}
```

- 降下イメージ:
  - `__debug_log__("msg", x, y)` → `MirInstruction::DebugLog { message: "msg".into(), values: [id_of(x), id_of(y)] }`
  - json_v0_bridge / MirBuilder 双方から同じ命令を使えるようにする。

### 2. Bridge / LoopBuilder 側での自動挿入案（オプション）

これは「実装するかどうかは後で決める」拡張案として残しておく。

- json_v0_bridge:
  - `NYASH_AUTO_DEBUG_LOG=1` のときだけ:
    - BoxCall や MethodCall の直前に `DebugLog` を挿入し、receiver や主要変数をログ出力。
  - 例:
    ```rust
    MirInstruction::DebugLog {
        message: format!("BoxCall recv at {}.{}()", box_name, method_name),
        values: vec![recv_id],
    }
    ```

- LoopBuilder / LoopForm v2:
  - `NYASH_LOOP_DEBUG_LOG=1` のときだけ:
    - header ブロックの先頭に `DebugLog` を挿入し、carrier 変数の値を 1 行でダンプ。
  - 例:
    ```rust
    MirInstruction::DebugLog {
        message: "Loop header carriers".to_string(),
        values: carrier_value_ids.clone(),
    }
    ```

## フェーズ内タスク（まだ実装しないメモ）

1. 設計固め
   - [ ] MirInstruction への `DebugLog` 追加仕様を最終確定（used_values / dst_value の扱い）。
   - [ ] Verifier への影響（特に MergeUses / RetBlockPurity）を整理（ログ命令を許可するポリシー）。
2. Rust 実装（最小）
   - [ ] `src/mir/instruction.rs` に DebugLog variant を追加。
   - [ ] `src/backend/mir_interpreter/exec.rs` に dev-only 実装を追加（`NYASH_MIR_DEBUG_LOG=1` ガード）。
   - [ ] `src/mir/printer.rs` に印字サポートを追加。
3. Hako からの利用導線
   - [ ] Hako パーサ / MirBuilder 側に `__debug_log__` 的な糖衣マクロを追加（構文をどうするかは別途検討）。
   - [ ] json_v0_bridge / MirBuilder のどこで DebugLog を使うか「観測ポイント候補」を CURRENT_TASK 側にメモ。
4. 拡張（任意）
   - [ ] `NYASH_AUTO_DEBUG_LOG=1` / `NYASH_LOOP_DEBUG_LOG=1` などのデバッグ専用トグルを検討。
   - [ ] LLVM ライン（PyVM/llvmlite ハーネス）での対応方法（printf など）を検討。

### 5. build_me_expression / static box との関係（検証タスクに含める）

- 現状:
  - `build_me_expression()` は、`variable_map["me"]` があればそれを返し、なければ `Const String(me_tag)` を生成してプレースホルダとして扱う実装になっている。
  - インスタンスメソッド（`lower_method_as_function` 経由）では params に `me` が含まれるため、`variable_map["me"]` から正しい Box パラメータが返る。
  - 一方で、static box / static 関数経路では `me` が文字列プレースホルダになるケースがあり、言語仕様上の「静的Boxでも暗黙 self が存在する」規約とはズレがある。
- 25.1p でやること（設計＋観測）:
  - [ ] DebugLog を使って、static box 内での `me` の実際の ValueId/VMValue をログし、「どこで文字列プレースホルダが使われているか」を可視化する。
  - [ ] `lower_static_method_as_function` と `lower_method_as_function` の責務を比較し、
        static box メソッドに対しても暗黙 receiver をパラメータとして扱うべきかどうかを設計レベルで判断する。
  - [ ] 必要であれば、別フェーズ（例: 25.1q）で「static box メソッドの me 取り扱い」を Box 理論ベースで揃える（DebugLog はそのための観測レイヤとして使う）。

### 6. Static box / me セマンティクス統一（部分完了メモ）

- 25.1c/25.1m までに判明したこと:
  - `static box StringHelpers` のような「純粋ユーティリティ箱」で、`me.starts_with(src, i, kw)` のように
    同一箱内ヘルパーを receiver 経由で呼ぶと、Stage‑3 降下で引数ずれ（`i` にソース全文が入る）が発生しうる。
  - 実際、`StringHelpers.starts_with_kw/3` → `StringHelpers.starts_with/3` 経路で
    `StringHelpers.starts_with("StringHelpers", src, i, kw)` のような形になり、
    `i + m > n` が `String > Integer(13)` の比較に化けていた。
- 25.1m での暫定対応（完了済み）:
  - `StringHelpers.starts_with_kw` 内を `me.starts_with(src, i, kw)` ではなく、素の `starts_with(src, i, kw)` 呼び出しに変更し、
    static box ユーティリティに対する `me` 依存を排除した。
  - これにより、`starts_with` 内の比較は全て整数同士となり、`String > Integer` 型エラーは解消済み。
- 25.1p 以降でやること:
  - DebugLog を使って、static box 全般（`StringHelpers` 以外も含む）の `me` の振る舞いを観測し、
    「本当にインスタンスとして扱いたい static box」と「名前空間としての static box」を切り分ける。
  - 必要に応じて、前述の `build_me_expression` / `lower_static_method_as_function` / `FunctionDefBuilder::is_instance_method`
    の SSOT 設計を詰め、「static box における me セマンティクス」を Rust 側に反映するタスクを別フェーズでまとめて行う。

## いつやるか（優先度メモ）

- 今回は **フォルダ＋設計メモだけ** で、実装はまだ行わない。
- 実装タイミングの候補:
  - Stage‑B / selfhost の SSA バグ（BreakFinderBox / ParserStringScanBox / Stage‑B Test2）がもう一段深いところで詰まったとき、
    「DebugLog を先に入れて ValueId の中身を観測しながら直す」フェーズとして 25.1p を前倒しする。
  - あるいは 25.1c/25.1k の大きな山が一段落してから、「開発者向けのデバッグ基盤」としてまとめて導入する。

このフェーズは「やることの設計だけ先に箱にしておく」位置づけだよ。  
本当に必要になったタイミングで 25.1p をアクティブにして、ここに書いたタスクを少しずつ実装していく想定。***
 End Patch ***!
  ```
   I apologize, but I can't apply this patch because it isn't formatted correctly. Could you fix it and try again?  ```
   Testing code: ```md
  # Phase 25.1p — MIR DebugLog 命令（構造設計メモ）
  ...
  本当に必要になったタイミングで 25.1p をアクティブにして、ここに書いたタスクを少しずつ実装していく想定。***  ->  ```  workdir=/home/tomoaki/git/hakorune-selfhost"]} ***!
