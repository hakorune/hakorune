Status: Planned  
Scope: Phase 43‑NORM‑CANON‑P2‑MID — JsonParser 本命 P2（`_parse_number` / `_atoi` 本体 / `_atof_loop`）を Normalized 基準に寄せるための事前設計メモ。
Update (Phase 43-A): `_atoi` 本体を Program(JSON) フィクスチャ `jsonparser_atoi_real` で dev Normalized→MIR(direct) 経路に載せ、Structured 直経路との VM 出力一致を確認済み（符号あり/なしの簡易パスまで対応。canonical 化は後続で検討）。
Update (Phase 43-C): `_parse_number` 本体を Program(JSON) フィクスチャ `jsonparser_parse_number_real` で dev Normalized→MIR(direct) 経路に載せ、Structured 直経路との VM 出力一致を dev テストで固定（num_str は現状仕様のまま据え置き）。

# Phase 43‑NORM‑CANON‑P2‑MID 設計メモ（JsonParser P2‑Mid 向け Normalized 拡張）

## 0. ゴールと前提

- ゴール
  - JsonParser の本命 P2 ループ（P2‑Mid）を、既存の Normalized インフラの延長で扱えるようにするための設計方針を固める。
  - 特に `_parse_number` / `_atoi` 本体 / `_atof_loop` について、
    - どのキャリア・body‑local を EnvLayout に載せるか
    - どの JpInst / JpOp パターンが追加で必要か
    - StepScheduleBox / DigitPos / NumberAccumulation との責務分担
    を整理する。
- 前提
  - P2‑Core（P2 ミニ + JP skip_ws mini/real + JP atoi mini）は Phase 41 までで Normalized→MIR(direct) が canonical 済み。
  - P2‑Mid は **JoinIR(Structured) までは載っているが Normalized は未対応** の状態（Phase 245/246 系）。
  - 本メモは「設計レベル」で止め、実装・テスト追加は後続フェーズ（43 実装回）で扱う。

---

## 1. 対象ループと P2‑Mid クラスの整理

- 対象とする P2‑Mid（JsonParser 側）
  - `_parse_number`（数値文字列の収集 + digit_pos break）
  - `_atoi` 本体（範囲チェック + digit_pos + NumberAccumulation）
  - `_atof_loop`（構造的には `_atoi` と同型の浮動小数点版）
- すべて Pattern2 Break で、既に JoinIR(Structured) には載っている：
  - `_parse_number` → Phase 245‑EX で header / break / `p` 更新を Pattern2 に統合済み（`num_str` は当面対象外）。
  - `_atoi` → Phase 246‑EX で DigitPos dual 値 + NumberAccumulation パターンとして JoinIR 経路に統合済み。
  - `_atof_loop` → 設計上 `_atoi` と同型とみなし、P2‑Mid クラスに含める。
- P2‑Core との差分
  - P2 ミニ / skip_ws / atoi ミニに比べて：
    - Carrier の本数（`p` + `result` + 場合によっては `num_str`）が増える。
    - body‑local / Derived Carrier（`digit_pos`, `is_digit_pos`, `digit_value` 等）の依存関係が複雑。
    - 一部で文字列連結（`num_str = num_str + ch`）や Range チェック（`"0" <= ch <= "9"`）が入る。

---

## 2. Normalized IR に必要な拡張（EnvLayout / JpInst / JpOp）

### 2.1 EnvLayout / フィールド設計

- P2‑Mid で EnvLayout に載せる候補
  - LoopState キャリア
    - `_parse_number`: `p`（必須）、`num_str`（Phase 43 では **オプション**、まず `p` 単独で正規化する案も許容）。
    - `_atoi` 本体 / `_atof_loop`: `i`, `result`（NumberAccumulation キャリア）。
  - Condition 専用 / FromHost
    - `len` / `s` / `digits` などの不変値は ParamRole::Condition として EnvLayout 側に持たない（現状どおり）。
  - Derived LoopState（DigitPos 系）
    - P2‑Core で既に導入済みの `digit_value` / `is_digit_pos` と同じ方針：
      - EnvLayout に「FromHost ではない LoopState キャリア」として載せる。
      - host_slot を持たず、ExitBinding には出さない（ループ内部完結キャリア）。

### 2.2 JpInst / JpOp 側の必要パターン

- 既存の Normalized が既に扱っているもの
  - `Let { dst, op: Const / BinOp / Unary / Compare / BoxCall, args }`
  - `If { cond, then_target, else_target, env }`
  - `TailCallFn` / `TailCallKont`（loop_step / k_exit のみ）
- P2‑Mid で追加検証・明文化が必要なパターン
  - `_parse_number`
    - `substring` / `indexOf` の BoxCall パターン（P2‑Core でも使用済みだが、`num_str` 周辺の利用を含めてドキュメントで SSOT 化する）。
    - 文字列連結 `num_str = num_str + ch` を
      - 当面は「扱わない」（`num_str` を Carrier から外す）案
      - もしくは `BinOp(Add)` として Normalized→MIR 直ブリッジに追加する案
      のどちらにするかを Phase 43 実装メモで最終決定する。
  - `_atoi` 本体 / `_atof_loop`
    - Range チェック `ch < "0" || ch > "9"`：
      - ExprLowerer / ConditionEnv 側で既に対応済みであれば、Normalized には Compare + BinOp(or) の形で入る。
      - Normalized では追加の JpOp は不要（Compare / BinOp を利用）。
    - NumberAccumulation パターン：
      - Structured 側では `UpdateRhs::NumberAccumulation` として扱っているので、
        - Normalized → MIR 直ブリッジ側で「Mul + Add + digit_value」の形を既に対応済み。
      - Phase 43 では `_atoi` 本体 / `_atof_loop` でも同じシーケンスになることを前提とし、JpInst 種別追加は行わない。

---

## 3. StepScheduleBox / DigitPos / NumberAccumulation の役割分担

### 3.1 StepScheduleBox（評価順）の適用範囲

- Phase 39 で導入した StepScheduleBox（Pattern2 用）は、P2‑Mid に対しても「どの StepKind をどの順番で評価するか」を決める SSOT として使う。
- `_parse_number` / `_atoi` 本体 / `_atof_loop` では、以下のようなフラグを想定：
  - `has_digitpos_body_local`（DigitPos 二重値を使うか）
  - `has_number_accumulation`（結果キャリアに Mul+Add があるか）
  - `has_bodylocal_break`（break 条件が body‑local に依存するか）
- StepScheduleBox は、これらのフラグだけを見て
  - 標準 P2: `[HeaderCond, BreakCheck, BodyInit, Updates, Tail]`
  - DigitPos / atoi 系: `[HeaderCond, BodyInit, BreakCheck, Updates, Tail]`
  など、評価順のバリエーションを返す「薄い箱」のまま保つ。
- Pattern2 lowerer / Normalized 変換側は、この StepSchedule に従って
  - header 条件
  - body‑local init（DigitPos / Range check 等）
  - break 条件
  - carrier 更新（NumberAccumulation / i++ 等）
  を「並べるだけ」にし、条件式の詳細や body‑local の構造には踏み込まない。

### 3.2 DigitPos / NumberAccumulation との接続

- DigitPos 系
  - `digit_pos` → `is_digit_pos` / `digit_value` の二重値設計は、P2‑Core と同じく「LoopState キャリア（FromHost なし）」として EnvLayout に載せる。
  - ExitBinding には出さず、Normalized→MIR 直ブリッジでも Loop 内部だけで完結させる。
  - Break 条件 `digit_pos < 0` は Phase 224 の DigitPosConditionNormalizer（AST→`!is_digit_pos`）を前提にし、Normalized 側は `!is_digit_pos` という bool 条件だけを受け取る。
- NumberAccumulation 系
  - Structured 側の LoopUpdateAnalyzer / CarrierUpdateEmitter が `result = result * 10 + digit_value` パターンを `UpdateRhs::NumberAccumulation` として検出・JoinIR 生成済み。
  - Normalized→MIR 直ブリッジは、P2‑Core と同じ Mul + Add シーケンスで MIR を吐く設計を維持し、P2‑Mid でも追加ロジックを増やさずに流用する。

---

## 4. Bridge / ShapeGuard / canonical 切り替え方針

- ShapeGuard 拡張
  - 既存の P2‑Core 判定（P2 ミニ / skip_ws mini/real / atoi mini）に加えて、
    - `_parse_number` 本体
    - `_atoi` 本体
    - `_atof_loop`
    を P2‑Mid として検出できる Shape 種別を追加する（例: `ShapeKind::JsonparserParseNumber`, `ShapeKind::JsonparserAtoiCore`）。
  - Phase 43 実装フェーズでは、まず P2‑Mid ループを **dev only** の Normalized 対象にする（canonical 切り替えは Phase 43 後半〜Phase 44 相当で検討）。
- Bridge 側の経路
  - `bridge_joinir_to_mir` の入口で：
    - P2‑Core: 既に canonical Normalized→MIR(direct)（Phase 41 の状態を維持）。
    - P2‑Mid: `normalized_dev_enabled()` が true のときに限り Structured→Normalized→MIR(direct) を試し、テストで Structured 直経路と比較。
  - Fail‑Fast 方針
    - P2‑Mid では「Normalized が未対応の領域」がまだ多いため、
      - dev / debug ビルドでは invariant 破壊・未対応命令で panic（Normalized 実装の穴を早期検出）。
      - release / canonical OFF 時は Structured→MIR 直経路に落とす（サイレントフォールバックではなく「Normalized をそもそも使わない」構成にする）。

---

## 5. テストと完了条件（Phase 43 実装フェーズ向けメモ）

- テスト戦略（概要）
  - `_parse_number`:
    - 代表ケース（"42", "7z" など）で
      - Structured→MIR→VM
      - Structured→Normalized→MIR(direct)→VM
      の stdout / RC を比較する dev テストを追加。
    - `num_str` をまだ Normalized キャリアに載せない場合でも、「p / break 条件 / DigitPos 周り」が齟齬なく動くことを確認する。
  - `_atoi` 本体 / `_atof_loop`:
    - 既存の `_atoi` mini dev fixture と同じ観点で、NumberAccumulation / DigitPos / Range check が Normalized 経路で再現できるかをチェック。
    - Mini / 本体 / `_atof_loop` が同じ normalized helper / bridge ロジックを共有できることを確認する。
- 完了条件（Phase 43 実装のためのチェックリスト）
  - EnvLayout / JpInst / JpOp / StepScheduleBox / DigitPos / NumberAccumulation の役割分担が本メモの通りに整理されている。
  - P2‑Mid（`_parse_number` / `_atoi` 本体 / `_atof_loop`）に対して、どこまでを Phase 43 で扱い、どこから先を後続フェーズに回すかの線引きが明文化されている。
  - `joinir-architecture-overview.md` の Phase 43 セクション（3.20）と、この設計メモの内容が矛盾していない。
