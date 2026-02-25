# Phase 25.1f — ControlForm（Loop/If 共通ビュー）& Stage‑B ハーネス準備

Status: planning（設計＋観測レイヤ追加／挙動は変えない）

## ゴール

- LoopForm v2（ループ）と If まわり（`phi_core::if_phi`）に対して、**制御構造の共通ビュー `ControlForm`** を用意する。
- すでに導入済みの Conservative PHI Box（If/Loop 用 SSA/PHI ロジック）を、
  - ループ専用／If 専用でバラバラに持つのではなく、
  - `ControlForm` を単一の SSOT（Single Source of Truth）として参照する方向に寄せる。
- Stage‑B 最小ハーネスや Stage‑1 Resolver など、「複雑な if + loop」を含む経路で、
  - ループと条件分岐の形を **構造として観測・検証できる足場** を整える（いきなり本番導線は切り替えない）。

※ このフェーズでは Rust の挙動は変えず、「制御構造の箱（LoopShape / IfShape）＋ ControlForm の定義」と、
  デバッグ／テスト用の観測導線（トレース）までに留める。

## 背景（25.1d / 25.1e までの位置づけ）

- 25.1d:
  - Rust MIR ビルダーの SSA/PHI バグ（ValueId 二重定義／non‑dominating use／undefined Value）を、  
    小さな Rust テスト（`mir_*_verifies`）で炙り出して修正済み。
  - CalleeBoxKind / BoxCompilationContext によって Stage‑B / Stage‑1 の静的 Box とランタイム Box の混線も構造的に解消。
- 25.1e:
  - ループの PHI 生成の SSOT を LoopForm v2 + `phi_core` に寄せ、
    - Exit PHI、break/continue、pinned/carrier/invariant 変数を整理。
  - Conservative PHI Box（If/Loop 両方で「安全側に倒す PHI 生成」を行い、不要な PHI は将来の削除で最適化する方針）を実装。
  - Stage‑1 UsingResolver 系テストと Program v0 PHI スモーク、Stage‑B 風ループテストはすべて緑。

ここまでで「LoopForm v2 + Conservative PHI」の根本バグはだいたい取り切れたが、

- PHI ロジックが If 用と Loop 用で散在している。
- Stage‑B ハーネス側で「どの loop/if がどう組み合わさっているか」を横断的に眺める手段がまだ弱い。

→ 25.1f では、**Loop / If を一段上から包む ControlForm レイヤ**を定義し、Stage‑B ハーネスに進む前準備として構造を固める。

## 方針 — ControlForm / Shape の設計

### 型と場所

- 新規モジュール案:
  - `src/mir/control_form.rs`
- 基本構造:
  - `LoopShape`:
    - `preheader: BasicBlockId`
    - `header: BasicBlockId`
    - `body: BasicBlockId`（代表 body ブロック）
    - `latch: BasicBlockId`
    - `exit: BasicBlockId`
    - `continue_targets: Vec<BasicBlockId>`
    - `break_targets: Vec<BasicBlockId>`
  - `IfShape`:
    - `cond_block: BasicBlockId`
    - `then_block: BasicBlockId`
    - `else_block: Option<BasicBlockId>`
    - `merge_block: BasicBlockId`
  - `ControlKind`:
    - `Loop(LoopShape)` / `If(IfShape)`（将来 `Switch` / `Match` も追加可能な余地だけ確保）
  - `ControlForm`:
    - `entry: BasicBlockId`（構造全体の入口）
    - `exits: Vec<BasicBlockId>`（構造を抜けた先のブロック群）
    - `kind: ControlKind`

### 生成パス（from_loop / from_if）

- `LoopForm` → `ControlForm`:
  - 既存の LoopForm v2（`LoopFormBuilder` が返す構造）から `LoopShape` を構築し、
  - `ControlForm::from_loop(loop_form: &LoopForm)` で共通ビューに変換する。
  - `entry = preheader`, `exits = [exit]` というルールで統一。
- If 用:
  - 既存の `phi_core::if_phi` / builder 側の if lowering から、必要な情報をまとめた薄い `IfForm`（もしくは `IfShape` 直構築）を定義。
  - `ControlForm::from_if(if_form: &IfForm)` で共通ビューに変換。
  - `entry = cond_block`, `exits = [merge_block]` を基本とし、将来 `else if` や switch なども扱える形で設計。

### Invariant / Debug 用の Trait

- 25.1f では「挙動を変えない」ため、ControlForm 自体は **観測・検証専用** として設計する:
  - `debug_dump(&self)`:
    - Loop / If の各ブロック ID を一括でログ出力し、Stage‑1 / Stage‑B テストから CFG の形を視覚的に確認できるようにする。
  - `CfgLike` トレイト（任意）:
    - `fn has_edge(&self, from: BasicBlockId, to: BasicBlockId) -> bool;`
    - `fn predecessors_len(&self, block: BasicBlockId) -> usize;`
    - `LoopShape::debug_validate<C: CfgLike>(&self, cfg: &C)` などを `#[cfg(debug_assertions)]` で用意し、
      Loop/If の形が想定どおりかを assert できるようにする。

## タスク粒度（25.1f TODO）

### F‑1: ControlForm / Shape の設計ドキュメント

- 本 README で:
  - `LoopShape` / `IfShape` / `ControlKind` / `ControlForm` の責務とフィールドを明文化。
  - LoopForm v2 との対応関係（preheader/header/latch/exit/continue/break）を図解レベルで通るところまで整理。
- 25.1e の LoopForm 仕様（Carrier/Pinned/Invariant / break/continue / exit PHI）と整合していることを確認。

### F‑2: Rust 側のスケルトン実装（導線のみ）

- `src/mir/control_form.rs` を追加し、以下のみ実装:
  - 型定義（`LoopShape` / `IfShape` / `ControlKind` / `ControlForm`）。
  - `ControlForm::from_loop(&LoopForm)` / `ControlForm::from_if(&IfForm)` の雛形。
  - `debug_dump()` と `debug_validate()`（`#[cfg(debug_assertions)]` 前提、もしくはテスト専用）。
- 既存のビルダー/PHI ロジックは **まだ ControlForm を使わない**:
  - 25.1e で安定した Conservative PHI 実装を壊さないため、Phase 25.1f では「観測オンリー」に留める。

### F‑3: Stage‑1 / Stage‑B テストへの観測フック

- 対象テスト:
  - `src/tests/mir_stage1_using_resolver_verify.rs` 系（Stage‑1 UsingResolver の min/full）。
  - Stage‑B 風ループ／Program v0 PHI スモーク（`mir_loopform_exit_phi.rs` / `mir_stageb_loop_break_continue.rs` / `tools/smokes/v2/...`）。
- 方針:
  - `NYASH_CONTROL_FORM_TRACE=1` などの dev フラグが立っているときだけ、
    - LoopForm v2 から `ControlForm` を生成し、
    - `debug_dump()` で構造をトレースログに出す。
  - これにより、Stage‑B ハーネスを組む前に「実際にどんな Loop/If 形が発生しているか」を CFG レベルで確認できる。

### F‑4: Conservative PHI との接続計画（設計のみ）

- 25.1e で実装した Conservative PHI Box（If/Loop）のロジックを整理し、将来像を設計として固定する:
  - 目標: `merge_modified_at_merge_with` / LoopForm v2 Exit PHI 生成などを、
    - `ControlForm` ベースの API で呼べるようにする（例: `build_phi_for_control(form: &ControlForm, ...)`）。
  - 25.1f では:
    - どの関数をどこまで ControlForm 受けにするか、
    - 既存 API との互換性をどう保つか、
    を README 上で設計するところまで。
  - 実際の実装切り替え（PHI ロジックを ControlForm ベースに差し替える）は、Stage‑B ハーネスと合わせて **別フェーズ（25.1g または 25.2 以降）** に送る。

### F‑5: Stage‑B ハーネスへの橋渡し準備

- Stage‑B 最小ハーネス（`tools/test_stageb_min.sh` / `lang/src/compiler/tests/stageb_min_sample.hako`）における:
  - ループパターン（`skip_ws` などの単純ループ、複数 break/continue を含むループ）。
  - if + loop のネスト構造。
  を洗い出し、LoopForm v2 + IfShape + ControlForm ですべて表現できることを確認する。
- 将来のタスク（このフェーズでは設計のみ）:
  - Stage‑B ハーネスの SSA/PHI 検証を `ControlForm` 経由で行う Rust テストをどう設計するかをまとめる。
  - Stage‑B 側の .hako LoopSSA（`lang/src/compiler/builder/ssa/loopssa.hako`）が、LoopForm v2 / ControlForm の設計と齟齬を起こさないように、必要な制約・前提条件を明記する。

### F‑6: .hako 側 ControlFormBox（Layer 2 の箱）定義

- 目的:
  - Rust 側 `LoopShape` / `IfShape` / `ControlForm` に対応する **Hakorune 実装版の箱** を用意し、  
    Stage‑B / LoopSSA が Rust と同じモデルでループ/if を扱えるようにする。
  - 将来の LoopSSA 実装は、この箱経由で構造を受け取ることを前提に設計する。
- 現行構文版（Layer 2）:
  - ファイル案: `lang/src/shared/mir/control_form_box.hako`
  - 定義案（BlockId は現状 i64/IntegerBox で表現）:
    ```hako
    static box ControlFormBox {
      kind_name: StringBox           // "loop" or "if"
      entry: IntegerBox              // BasicBlockId 相当
      exits: ArrayBox                // of IntegerBox (BlockId)

      // Loop fields (kind_name == "loop" の時のみ有効)
      loop_preheader: IntegerBox
      loop_header: IntegerBox
      loop_body: IntegerBox
      loop_latch: IntegerBox
      loop_exit: IntegerBox

      // If fields (kind_name == "if" の時のみ有効)
      if_cond: IntegerBox
      if_then: IntegerBox
      if_else: IntegerBox
      if_merge: IntegerBox

      birth(kind) {
        me.kind_name = kind
        me.exits = new ArrayBox()
      }

      is_loop() {
        return me.kind_name == "loop"
      }

      is_if() {
        return me.kind_name == "if"
      }
    }
    ```
  - 対応表（Rust ↔ .hako）:
    - Rust `LoopShape.preheader/header/body/latch/exit`  
      ↔ Hako `loop_preheader/loop_header/loop_body/loop_latch/loop_exit`
    - Rust `IfShape.cond_block/then_block/else_block/merge_block`  
      ↔ Hako `if_cond/if_then/if_else/if_merge`
    - Rust `ControlForm.entry/exits`  
      ↔ Hako `entry` / `exits`（`exits` は BlockId の配列）
- 将来構文版（Layer 3 の理想形・variant 導入前提）:
  - 将来の Nyash variant 構文が入ったら、`ControlKind` を enum 的に表現し、
    ```hako
    box ControlKind {
      variant Loop { preheader: BlockIdBox, header: BlockIdBox, ... }
      variant If   { cond_block: BlockIdBox, then_block: BlockIdBox, ... }
    }

    static box ControlFormBox {
      kind: ControlKind
      entry: BlockIdBox
      exits: ArrayBox
    }
    ```
    のような形に寄せる（現段階では設計メモのみ）。
- 25.1f でのスコープ:
  - `control_form_box.hako` に上記 Layer 2 版 `ControlFormBox` を実装する。
  - ただし Stage‑B / LoopSSA からはまだ使用しない（コンパイルが通る最小限の実装＋将来用の箱）。
  - Stage‑B LoopSSA 統合（ControlFormBox を実際に使って JSON v0→MIR を整理する）は、次フェーズに回す。

## 移行ステップ（Option A → B → C のロードマップ）

このフェーズ（25.1f）では、あくまで **Option A = 観測レイヤーの完成** までをスコープにするよ。その上で、将来の B/C も含めて合意しておきたい移行順序はこうだよ:

1. **Option A: 観測レイヤーを完璧にする（Phase 25.1f）**
   - Rust:
     - `LoopShape` / `IfShape` / `ControlForm` 実装済み。
     - `LoopBuilder` から LoopForm v2 / `lower_if_in_loop` の出口で `ControlForm::Loop/If` を生成して `debug_dump` 済み。
     - `NYASH_CONTROL_FORM_TRACE` は「未設定=ON, 0/false=OFF」のヘルパーに統一済み。
   - .hako:
     - `ControlFormBox`（kind_name + loop_* / if_* + entry/exits）の箱だけ定義（まだ未使用）。
   - テスト/スモーク:
     - `mir_stage1_using_resolver_*` / `mir_loopform_exit_phi.rs` / `mir_stageb_loop_break_continue.rs` / `tools/test_stageb_min.sh` を回しつつ、
       ControlForm トレースが panic せずに構造ログだけを出すことを確認する。

2. **Option B: Conservative PHI ↔ ControlForm の統合（別フェーズ: 25.1g 想定）**
   - ねらい:
     - いま `merge_modified_at_merge_with`（If）や LoopForm v2 Exit PHI が直接 BlockId/ValueId を持っている部分を、
       `ControlForm` 経由で読めるように段階的に寄せていく。
   - 方針:
     - まず If から（`ControlKind::If`）→ ループ Exit PHI（`ControlKind::Loop`）の順に小さく進める。
     - 各ステップごとに:
       - 対応するテスト（Stage‑1 / Program v0 / Stage‑B 風）＋ `test_stageb_min.sh` を回し、  
         SSA/PHI の赤ログが増えていないことを確認しながら移行する。
   - 注意:
     - これは 25.1f のスコープ外。Phase 25.1g（仮）として小さな差分に分割しながら実装する。

3. **Option C: LoopBuilder/If 降下の ControlForm ベース正規化（さらに後ろのフェーズ）**
   - ねらい:
     - 最終的には「LoopBuilder/If 降下がまず ControlForm を組み立てる」スタイルに寄せ、  
       Conservative PHI / LoopSSA / .hako 側 LoopSSA が同じ ControlForm を揃って見る状態にする。
   - 規模:
     - 数百行単位のリファクタになるため、Phase 25.2 以降（Selfhost/Stage‑B の安定を見ながら）に分割してやる。
   - ガード:
     - 各ステップで代表スモーク（Stage‑1 / Stage‑B / Program v0 / `test_stageb_min.sh`）を流しながら進める。
     - `NYASH_CONTROL_FORM_TRACE` と `.hako ControlFormBox` を使って、常に「形」が崩れていないかを観測できるようにしておく。

このフェーズ（25.1f）は「Option A をやり切って足場を固める」ことに専念し、その上で 25.1g 以降で Option B/C に進む、というロードマップで進めるよ。

## このフェーズで「やらないこと」

- if を loop に書き換えるような、意味論レベルの正規化は **行わない**:
  - If は IfShape、Loop は LoopShape として、それぞれの形を尊重する。
- Stage‑B ハーネス本体の導入・既存パイプラインの切り替え:
  - `test_stageb_min.sh` の経路変更や `.hako` コンパイラ側の LoopSSA 実装変更は、25.1f では触らず、  
    ControlForm の設計と観測レイヤ整備が完了してから別フェーズで扱う。
- Conservative PHI ロジックの大規模リライト:
  - 25.1e で安定させた If/Loop PHI 実装を、いきなり ControlForm ベースに書き換えることはしない。
  - まずは「どう書き換えるのが構造的に美しいか」を、このフェーズの README に設計として落とす。  
