# Phase 25.1l — Region/GC 観測レイヤー（LoopForm v2 × RefSlotKind）

Status: completed（Rust 側観測レイヤーの最小実装まで完了／挙動変更なし）

## ゴール

- 既に導入済みの **LoopForm v2 / ControlForm** を「Region Box（寿命管理の箱）」として扱い、  
  Rust MIR 側に **スコープと参照種別（RefKind）の観測レイヤー**を追加するフェーズだよ。
- このフェーズではあくまで:
  - `RefSlotKind` / `Region` / `SlotMetadata` といった **型と観測器（Observer）** を定義し、
  - `NYASH_REGION_TRACE=1` のときだけ Region 単位の live スロットと RefKind をログ出力する。
- **挙動（実行結果・SSA・PHI）は一切変えない**ことが前提。GC の retain/release 挿入や `.hako` 側のロジック変更は後続フェーズ（25.1m 以降）の仕事にする。

## 背景と位置づけ

- これまでの 25.1d〜25.1k で:
  - LoopForm v2 / ControlForm / Conservative PHI Box によって、If/Loop の SSA/PHI は Rust 側で安定。
  - Stage‑B/LoopSSA 由来の SSA 問題の多くは `.hako` 側（特に `StageBBodyExtractorBox.build_body_src/2`）の  
    「巨大なループ＋if 内での一時値の扱い」に起因することが分かってきた。
- いっぽう GC 観点では:
  - 変数スロットごとに「StrongRoot / WeakRoot / Borrowed / NonRef」を区別し、
  - **Region（= LoopForm/IfForm）境界を GC スコープ境界として扱う**設計が箱理論的にきれいにハマる。
- 25.1l はそのための **観測フェーズ** であり:
  - LoopForm v2 = Region Box という概念を Rust 型として明示し、
  - Stage‑B など複雑な関数で「どの Region でどのスロットが生きているか」をログで確認できるようにする。

## スコープ（25.1l でやること）

### L‑A: RefSlotKind / SlotMetadata / Region 型の導入（実装済み）

- 新規モジュール: `src/mir/region/mod.rs`
- 型設計:
  - `RefSlotKind`:
    - `StrongRoot` — GC root 候補となる強参照スロット。
    - `WeakRoot` — 弱参照（GC root としては数えない）。
    - `Borrowed` — 借用（SSA 寿命のみ管理、GC root ではない）。
    - `NonRef` — プリミティブなど、GC 対象外の値。
  - `SlotMetadata`:
    - `name: String` — 変数スロット名（`i`, `body_src`, `args` など）。
    - `ref_kind: RefSlotKind` — 上記の種別。
  - `RegionKind`:
    - `Function` — 関数スコープ。
    - `Loop` — ループ構造（LoopForm v2 由来）。
    - `If` — if/else 構造。
  - `Region`:
    - `id: RegionId`（`u32` ラッパ）。
    - `kind: RegionKind` — 上記 3 種。
    - `parent: Option<RegionId>` — 親 RegionId（FunctionRegion がルート、Loop/If はその子）。
    - `entry_block: BasicBlockId` / `exit_blocks: Vec<BasicBlockId>` — ControlForm から引き継ぐ。
    - `slots: Vec<SlotMetadata>` — その Region で live とみなすスロット一覧（観測用）。
- RefKind 判定は 25.1l では **簡易ヒューリスティック** に留める:
  - `MirType::Box(_)` / `Array(_)` / `Future(_)` → `StrongRoot`
  - 明らかなプリミティブ（整数/bool/文字列）→ `NonRef`
  - Weak 系/借用系の精密な分類は後続フェーズで詰める。

### L‑B: RegionObserver の実装と ControlForm/Function からの接続（実装済み）

- 新規モジュール: `src/mir/region/observer.rs`
- 現実装の責務:
  - `observe_function_region(builder: &mut MirBuilder)`:
    - `NYASH_REGION_TRACE=1` かつ 関数名に `"StageB"` を含む場合のみ、`RegionKind::Function` を 1 つ作成。
    - `RegionId` を `MirBuilder.current_region_stack` に push し、ルート Region としてログ出力。
  - `observe_control_form(builder: &mut MirBuilder, form: &ControlForm)`:
    - `ControlForm` から `entry`/`exits`/Loop/If の形を読む。
    - `builder.current_region_stack.last()` を `parent: Option<RegionId>` として保持し、FunctionRegion の子として Loop/IfRegion をぶら下げる。
    - スロット列挙は FunctionSlotRegistry（後述）を優先し、なければ `variable_map` + `value_types` で暫定推定。
    - `NYASH_REGION_TRACE=1` のときだけ:
      - `[region/observe] fn=StageBBodyExtractorBox.build_body_src/2 id=RegionId(..) kind=Loop entry=bb.. exits=[..] slots=[..]`
        のようなログを eprintln する（メモリ蓄積はしない）。
- Hook の置き場所（いずれも観測専用）:
  - `src/mir/builder/lifecycle.rs`:
    - main 関数生成後に `observe_function_region(self)` を呼び出し（関数名フィルタにより、多くはスキップされる）。
  - `src/mir/builder/calls/lowering.rs`:
    - static 関数用: `create_function_skeleton` で新関数を作成後、`observe_function_region(self)` を呼び出す。
    - instance method 用: `create_method_skeleton` 後に同様に `observe_function_region(self)` を呼び出す。
    - finalize (`lower_static_method_as_function` / `lower_method_as_function`) の最後で `pop_function_region(self)` を呼び、Region スタックを 1 段戻す。
  - `src/mir/loop_builder.rs`:
    - ループ構築完了後、`LoopShape`→`ControlForm::from_loop` 生成直後に `observe_control_form(self.parent_builder, &form)` を呼ぶ。
    - `lower_if_in_loop` でも `IfShape`→`ControlForm::from_if` 生成直後に `observe_control_form(self.parent_builder, &form)` を呼ぶ。
  - dev フィルタ:
    - 現時点では Stage‑B 周辺の観測に絞るため、`func_name.contains("StageB")` の関数のみログ対象にしている（ログ爆発防止）。

### L‑C: 関数スコープ Slot 管理箱（FunctionSlotRegistry）との関係（実装済み・観測専用）

- 新規モジュール: `src/mir/region/function_slot_registry.rs`
  - `SlotId(u32)` / `SlotInfo { name, ty: Option<MirType>, ref_kind: Option<RefSlotKind> }`。
  - `FunctionSlotRegistry { slots: Vec<SlotInfo>, name_to_slot: HashMap<String, SlotId> }`。
  - API:
    - `new()` — 空レジストリ。
    - `ensure_slot(name, ty)` — スロットが存在しなければ作成し、`SlotId` を返す。
    - `set_ref_kind(slot, RefSlotKind)` — 後から RefKind を埋める。
    - `iter_slots()` / `get_slot(name)` / `get_slot_info(slot)` — 観測用読み出し。
- `MirBuilder` への統合:
  - フィールド:
    - `current_slot_registry: Option<FunctionSlotRegistry>` — `current_function` と同じライフサイクルで生存。
    - `current_region_stack: Vec<RegionId>` — FunctionRegion/Loop/IfRegion の親子関係を維持する dev 用スタック。
  - 関数開始・終了での管理:
    - main 関数生成時（`prepare_module`）で `FunctionSlotRegistry::new()` を作成。
    - static 関数/instance method lowering 時（`calls/lowering.rs`）で:
      - `create_function_skeleton` / `create_method_skeleton` 内で新しいレジストリをセット。
      - `LoweringContext` に `saved_slot_registry` を追加し、呼び出し元のレジストリを退避・復元。
    - `finalize_module` / `lower_static_method_as_function` / `lower_method_as_function` の終了時に `current_slot_registry = None`（または `saved_slot_registry` を復元）。
- SlotRegistry の更新ポイント（挙動不変・観測のみ）:
  - パラメータ（static 関数）:
    - `setup_function_params` で `params` とシグネチャの `MirType` をローカルベクタに集約し、ループ後に `reg.ensure_slot(name, ty)` を呼ぶ。
  - パラメータ（instance method）:
    - `setup_method_params` で `me` と通常パラメータを `(name, None)` として集約し、同様に `ensure_slot`。
  - static Main ラッパー:
    - `build_static_main_box` で `self.variable_map.insert(p.clone(), pid)` の直後に `value_types.get(&pid)` を使って `ensure_slot(p, ty)`。
  - ローカル変数/nowait/me:
    - `build_local_statement` で `variable_map` 登録後に `ensure_slot(&var_name, value_types.get(&var_id))`。
    - `build_nowait_statement` で Future を束ねる `variable` 名を `ensure_slot(&variable, None)`。
    - `build_me_expression` で `me` を初回生成したときに `ensure_slot("me", None)`。
- RegionObserver との接続:
  - `observe_control_form` では、`current_slot_registry` が存在する場合はそれを優先:
    - `classify_slots_from_registry(reg)` で:
      - SlotInfo.ty から `Region::classify_ref_kind` を使って RefKind を決定。
      - ty が無い場合は `classify_slot_name_only`（`args/src/body_src/...` 系を StrongRoot とみなす簡易ヒューリスティック）。
      - 各 SlotInfo に `set_ref_kind` で RefKind を埋めてから `Region.slots` を構築。
  - これにより、RefKind 判定は SlotRegistry 側に一元化され、Region 側は `SlotMetadata { name, ref_kind }` を参照するだけになる。

### L‑D: Region メタデータの足場（将来の JSON 拡張のための入口だけ）

- 25.1l では **まだ MIR JSON への出力は行わない**が、将来のために:
  - `Region::to_json()` 相当のメソッドを `#[cfg(feature = "region-meta")]` 等のガード付きで用意しておく。
  - `MirCompiler` 側に「RegionObserver を渡しておけば、あとでメタデータを JSON に差し込める」拡張ポイントだけ作っておく。
- GC 統合フェーズ（25.1m 以降）では、このメタデータを:
  - `Program(JSON v0) → MIR(JSON)` 変換結果の横に `"regions":[...]` として添付する形を想定している。

## このフェーズで「やらない」こと

- **GC の retain/release 挿入**:
  - Region 情報を使って `retain(slot)` / `release(slot)` 命令を実際に MIR に埋め込むのは、25.1m 以降のタスクとし、このフェーズでは一切行わない。
- **LoopForm v2 / Conservative PHI / ControlForm の設計変更**:
  - 25.1l はあくまで「読み取り専用の観測レイヤー」を追加するだけで、既存の SSA/PHI 実装には手を入れない。
- **.hako 側の GC 実装や Stage‑B 本体の大規模リファクタ**:
  - Stage‑B/LoopSSA の箱分解（`StageBBodyExtractorBox` のサブ箱化）や、`.hako` 側 GC API の導入は、Region 観測結果を見ながら次フェーズで設計する。
- **関数スコープ SlotRegistry の本実装**:
  - 現段階では `variable_map` ベースの暫定観測に留め、SlotId ベースの SSOT 化は次のフェーズ（Region 木 + FunctionRegion 導入時）に回す。

## 受け入れ条件（25.1l）

- `NYASH_REGION_TRACE=0`（既定）のとき:
  - すべての既存テスト（LoopForm v2 / Stage‑1 resolver / Stage‑B Rust テスト）が挙動一切変化せず緑のまま。
- `NYASH_REGION_TRACE=1` のとき:
  - 代表関数（特に `StageBBodyExtractorBox.build_body_src/2`）に対して Region/Slot のログが出力される。
  - ログは「Region id / kind(If/Loop) / entry/exit blocks / slots + RefKind」を含み、  
    今後の GC/寿命設計の議論に耐えうる解像度になっている。
- 変更差分は Rust 側に限定され、`.hako` 側コードや Stage‑B 本体には影響を与えない。*** End Patch***"/>
