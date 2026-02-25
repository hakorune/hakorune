# AotPrep Passes (Pre‑AOT Normalization)

目的: MIR(JSON v0) を安全な範囲で前処理し、LLVM/AOT に渡す前の負荷を軽減します。

方針
- 既定は挙動不変・最小。すべて opt‑in の ENV トグルで有効化します。
- バックエンド非依存の“構造正規化”のみを行い、意味論を変えません。
- 将来的にファイル分割（passes/*）へ移行できるよう、パスの役割を明確化します。

主なパス（現在）
- StrlenFold（`NYASH_LLVM_FAST=1` or `NYASH_MIR_LOOP_HOIST=1`）
  - 固定文字列由来の `length/len` を i64 即値に置換
- LoopHoist（`NYASH_MIR_LOOP_HOIST=1`）
  - mod/div/compare の右辺にぶら下がる定数 `const` をブロック先頭へ移動
  - 同一ブロック i64 の `const` をデデュープ
- CollectionsHot（`NYASH_AOT_COLLECTIONS_HOT=1`）
  - Array/Map の単純 `boxcall` を `externcall`（`nyash.array.*` / `nyash.map.*`）へ張替え
  - Map キー戦略は `NYASH_AOT_MAP_KEY_MODE={h|i64|hh|auto}`（既定: `h`/`i64`）
  - **型確定強化（NEW!）**:
    - **Backpropagation Pass**（`tmap_backprop`）: コールサイトからの型シグナル逆伝播
      - `push` メソッド → 受信=Array と確定
      - `get`/`set`/`has` + stringy key（toString/StringBox const/文字列連結）→ 受信=Map
      - `get`/`set`/`has` + linear index → 受信=Array
      - fixpoint反復（max 2周）で新事実を追加
      - ENV制御: `NYASH_AOT_CH_BACKPROP=1`（既定=1）
  - 型確定の優先順位（上から順に試行）:
    1. 型テーブル（`type_table.has(bvid)`）
    2. PHI型推論（`peek_phi_type`）
    3. 後方MIR解析（`resolve_recv_type_backward`）
    4. **メソッド専用判別（push は Array のみ）** - 強化!
    5. 直接マップ（`arr_recv`/`map_recv`）
    6. **キー文脈ヒューリスティック（`is_stringy_key_in_block`）** - 強化!
    7. newbox 逆スキャン
- BinopCSE（`NYASH_MIR_LOOP_HOIST=1`）
  - 同一 binop（加算/乗算を含む）を1回だけ発行し、`copy` で再利用することで線形インデックス（例: `i*n+k`）の再計算を抑制
  - `LoopHoist` と組み合わせることで `const` + `binop` の共通部分を前出しする汎用CSEになっている

ENV トグル一覧
- `NYASH_MIR_LOOP_HOIST=1` … StrlenFold + LoopHoist + ConstDedup を有効化
- `NYASH_AOT_COLLECTIONS_HOT=1` … Array/Map hot-path 置換を有効化
- `NYASH_AOT_MAP_KEY_MODE` … `h`/`i64`（既定）, `hh`, `auto`（将来拡張）
- `auto` モードでは `_is_const_or_linear` で args を走査し、単純な i64 定数/線形表現と判定できる場合に `nyash.map.*_h` を選ぶ（それ以外は `*_hh`）。
- `NYASH_AOT_CH_BACKPROP=1` … Backpropagation Pass有効化（既定=1、0で無効化）
- `NYASH_AOT_CH_TRACE=1` … CollectionsHot の詳細診断出力（型推論・書き換え情報・backprop）
- `NYASH_VERIFY_RET_PURITY=1` … Return 直前の副作用を Fail-Fast で検出。ベンチはこのトグルをオンにした状態で回しています。

使い方（例）
```
export NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1 \
       NYASH_LLVM_SKIP_BUILD=1 NYASH_LLVM_FAST=1 NYASH_LLVM_FAST_INT=1 \
       NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_COLLECTIONS_HOT=1
tools/perf/microbench.sh --case arraymap --exe --runs 3
```

注意
- すべて opt‑in。CI/既定ユーザ挙動は従来どおりです。
- Return 純化ガード（`NYASH_VERIFY_RET_PURITY=1`）と併用してください。

Stage-3 キーワード要件
- AotPrep 各パスは Stage-B（Nyash自身で書かれた自己ホストコード）のため、`local`/`flow`/`try`/`catch`/`throw` などの Stage-3 キーワードを使用します。
- これらのキーワードは `NYASH_PARSER_STAGE3=1` および `HAKO_PARSER_STAGE3=1` が必要です。
- **推奨**: `tools/hakorune_emit_mir.sh` を使うこと。このスクリプトは必要なENVを自動設定します。
- **手動実行時**: 以下のENVを明示的に付与してください。
  ```bash
  NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  ./target/release/hakorune --backend vm your_file.hako
  ```
- **診断**: `NYASH_TOK_TRACE=1` を追加すると、Stage-3キーワードが識別子に降格される様子が `[tok-stage3]` ログで確認できます。
