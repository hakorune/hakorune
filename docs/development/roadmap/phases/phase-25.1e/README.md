# Phase 25.1e — LoopForm PHI v2 Migration (Rust MIR)

Status: completed（Rust MIR 側の LoopForm/PHI 正規化は実装済み。LoopForm v2 + LoopSnapshotMergeBox を SSOT として運用中）

## ゴール

- ループまわりの SSA / PHI 生成の「SSOT（単一の正）」を **LoopForm v2 + phi_core** に寄せて、Legacy 経路との二重管理を解消する。
- Stage‑1 / Stage‑B / selfhost で見えている以下の問題を、LoopForm 側の設計として整理して直す:
  - 複雑ループ（`Stage1UsingResolverFull._find_from/3` など）での「同一 ValueId の二重定義（PHI vs 既存値）」。
  - Merge block（ヘッダ／合流ブロック）で、predecessor 定義値を PHI なしで読むことによる non‑dominating use。
  - pinned 受信箱（`__pin$*@recv`）や Loop carrier 変数の PHI 対象範囲が曖昧で、legacy/local_ssa/LoopForm の責務が重なっている問題。
- 既存テスト／Stage‑B 最小ハーネス／selfhost CLI から見える「SSA/PHI 赤ログ」を、LoopForm v2 経路を正とすることで構造的に潰す。

## 前提 / これまでにやったこと（25.1d まで）

- Local 変数の SSA 化:
  - `build_local_statement` を修正し、`local a = expr` ごとに新しい ValueId を払い出して `Copy` 命令で初期化値をコピーするように統一。
  - `src/tests/mir_locals_ssa.rs` で `local a,b,c` パターンを固定し、Const/NewBox とローカル変数レジスタが分離されることを確認済み。
- Callee 解決/ガード:
  - `CalleeBoxKind` / `CalleeResolverBox` / `CalleeGuardBox` の導入により、Stage‑B / Stage‑1 の static compiler Box と runtime Box の混線を構造的に防止。
  - `StageBArgsBox.resolve_src/1` 内の `args.get(i)` が `Stage1UsingResolverBox.get` に化ける問題は解消済み。
- Loop/PHI まわりの scaffolding:
  - `phi_core::loop_phi::{prepare_loop_variables_with, seal_incomplete_phis_with, build_exit_phis_with}` と `LoopPhiOps` を導入し、LoopBuilder から PHI 生成を委譲可能な構造は整備済み（現在は legacy 互換用のみ）。
  - LoopForm v2 (`LoopFormBuilder` + `LoopFormOps`) は Rust AST ルートの既定実装として常時有効で、legacy 経路（`build_loop_legacy`）は Rust 側では撤去済み。
  - Phase 25.2 では `LoopSnapshotMergeBox` を導入し、continue / break / exit スナップショットのマージと PHI 入力構成を一元管理している（詳細は `phase-25.2/README.md` を参照）。

残っていた問題は、主に legacy LoopBuilder / loop_phi / LoopForm v2 の責務が重なっていたところだよ。現在は LoopForm v2 + LoopSnapshotMergeBox を「正」とし、legacy 側は互換レイヤとして閉じ込めている。

## 方針（25.1e）

- **SSOT を決める**:
  - ループの PHI 生成の「正」は LoopForm v2 (`LoopFormBuilder` + `LoopFormOps` + `phi_core::loop_phi/if_phi`) に置く。
  - `build_loop_legacy` + `prepare_loop_variables_with` は「互換レイヤ/移行レイヤ」と位置づけ、最終的には LoopForm v2 の薄いラッパに縮退させる。
- **Feature Flag で段階導入（※この段階の設計メモ）**:
  - 当初は `NYASH_LOOPFORM_PHI_V2=1` のときだけ LoopForm v2 経路を使う案だったが、
    現在は LoopForm v2 が既定実装となっており、legacy 経路は撤去済み。
  - `NYASH_LOOPFORM_PHI_V2` は互換性のために残っているが、挙動切り替えには使われない。
- **1 バグ 1 パターンで前進**:
  - `mir_stage1_using_resolver_full_collect_entries_verifies` や Stage‑B 最小ハーネスで見えている赤ログは、それぞれ最小 Hako に絞って LoopForm v2 側で再現・修正する。
  - 同時に legacy 側からは同じ責務（PHI 生成ロジック）を抜いていき、二重管理を減らす。

## タスク粒度（やることリスト）

### 1. LoopForm v2 の足場をテストで固める

1.1 LoopForm v2 専用テストモードの追加（→ 現在は「LoopForm v2 が既定」の前提で完了）  
- `mir_stage1_using_resolver_full_collect_entries_verifies` は LoopForm v2 前提で緑になっており、
  もはやフラグは不要（テスト内の `NYASH_LOOPFORM_PHI_V2` 設定も削除済み）。

1.2 Stage‑B 最小ハーネス用ループ抜き出しテスト  
- `lang/src/compiler/tests/stageb_min_sample.hako` から、代表的なループだけを抜き出した Hako を作り、LoopForm v2 経路（`NYASH_LOOPFORM_PHI_V2=1`）で `MirVerifier` を通すテストを追加。
- 目的: Stage‑B / selfhost CLI で見えているループ系のバグを、純粋な LoopForm/PHI 問題として Rust テストに落とし込む。

### 2. Legacy / v2 の責務切り分け

2.1 prepare_loop_variables の責務縮小（実施中の変更の整備）  
- 既に導入したフィルタリング:
  - `prepare_loop_variables` が preheader の `variable_map` 全体ではなく、「ループキャリア変数（body で再代入されるもの）＋ pinned 変数（`__pin$*`）」だけを PHI 対象にするように変更。
  - 効果: `text_len` / `pattern_len` などループ不変なローカルに PHI を張らないことで、ValueId の二重定義/UseBeforeDef が起きにくくなる。
- 25.1e では、この変更を LoopForm v2 側の設計として明文化し、legacy 側のコメントやドキュメントもそれに揃える。

2.2 LoopForm v2 での PHI 生成を SSOT にする  
- `LoopFormBuilder::prepare_structure` / `emit_preheader` / `emit_header_phis` の挙動をドキュメント化し、「どの変数に PHI を張るか」のルールを固定:
  - 関数パラメータ ＋ 明示的なループキャリア変数（body 内で再代入される）＋ pinned 変数のみ。
  - ループ不変変数は preheader の値をそのまま使い、PHI を作らない。
- `build_loop_legacy` の PHI 補助ロジック（header/exit での snapshot + PHI 生成）は、LoopForm v2 のロジックと重複しないように段階的に削る。

### 3. mir_stage1_using_resolver_full_collect_entries の赤ログ解消

- 現状の代表的なエラー:
  - `Value %24 / %25 / %26 defined multiple times (bb53 vs bb54)`  
  - `Merge block bb54 uses predecessor-defined value %28/%29/%27 from bb59/bb61 without Phi`
- タスク:
  1. `_find_from` ループに対応する Hako 断片を LoopForm v2 経路でミニテスト化。
  2. LoopForm v2 側で:
     - preheader/header/latch/exit の各ブロックに対して、どの変数が carrier/pinned なのかを明示的に計算。
     - PHI dst の ValueId 割り当てを `MirFunction::next_value_id` に完全委譲し、既存 SSA 値と衝突しないようにする。
     - header での PHI 再定義（`%24` copy + `phi %24` のような形）を避けるため、古い値と新しい値のバインディングを LoopForm 内部で完結させる。
  3. 修正後、`mir_stage1_using_resolver_full_collect_entries_verifies` が LoopForm v2 経路で緑になることを確認。

### 4. Stage‑B / selfhost CLI への波及

- Stage‑B 最小ハーネス（`tools/test_stageb_min.sh`）と selfhost CLI スモークを、LoopForm v2 経路で試験的に実行:
  - `NYASH_LOOPFORM_PHI_V2=1` にした状態で Test2（`compiler_stageb.hako` 経由）と selfhost CLI を実行し、ValueId 未定義や PHI 不整合が減っていることを確認。
- 25.1e のスコープでは、「v2 経路での挙動が legacy より悪化しない（少なくとも同程度、可能なら改善）」ことを目標に、必要最小限の修正に留める。

## 設計図 — LoopForm v2 Scope モデル

25.1e では「すべてのループ／if/else を LoopForm v2 のスコープとして見る」という前提で設計を固める。
ここでは **スコープ単位の入出力と break/continue の扱い** を明文化する。

### 1. 基本モデル（LoopScope / IfScope）

- すべての制御構造は「スコープ」として扱う:
  - `LoopScope`: `while/loop` 相当（Nyash の `loop (cond) { ... }`）。
  - `IfScope`: `if (cond) { then } else { else }`。
- 各スコープは次の情報を持つ:
  - 入力: `Env_in` … スコープ入口時点の `variable_map`（名前→ValueId）。
  - 出力: `Env_out` … スコープ出口時点の `variable_map`。
  - 内部状態:
    - `Carriers`: ループ本体で再代入される変数名の集合。
    - `Pinned`: ループをまたいで保持する必要がある値（`__pin$*@recv` や `me` の一部など）。
    - `BreakSnaps`: break 到達時の `Env` スナップショットの集合。
    - `ContinueSnaps`: continue 到達時の `Env` スナップショットの集合。

LoopForm v2 は「各スコープの `Env_in` と `Env_out` を定義し、SSA/PHI をその範囲で完結させる」ことを目標にする。

### 2. LoopScope の形状とブロック

LoopScope は LLVM の canonical form に従う:

```text
preheader → header (PHI) → body → latch → header
                      ↘ exit
```

- preheader:
  - ループに入る直前のブロック。
  - `Env_in(loop)` をそのまま保持し、loop entry 用の Copy をここで emit する（Carrier/Pinned のみ）。
- header:
  - ループ条件・合流点。
  - Entry 時点では preheader からの Copy を入力に PHI を seed するだけ（latch/continue は後で seal）。
- body:
  - ループ本体。`Carriers` に対する再代入や break/continue が発生する。
- latch:
  - body の末尾ブロック（continue でヘッダに戻る前に通る最後のブロック）。
  - `Env_latch` として LoopForm v2 に引き継がれ、header PHI の backedge 値に使われる。
- exit:
  - ループ脱出ブロック。`BreakSnaps` と header fall-through をマージして exit PHI を作る。

### 3. 変数分類 — Carrier / Pinned / Invariant

- Carrier:
  - ループ本体（body）で **再代入される** 変数。
  - 例: `i`, `a`, `b` などのインデックスや累積値。
  - LoopScope では:
    - header entry で `phi(entry, latch)` を必ず持つ。
    - exit でも header 値と break 値を PHI でマージする。
- Pinned:
  - `me` レシーバや `__pin$*@recv` のように、ループをまたいで同じ Box を指し続ける必要がある値。
  - ループ内の再代入はほとんどないが、「PHI に乗せておかないと次のイテレーションで UseBeforeDef になる」種類の値。
  - LoopScope では:
    - header PHI の入力として preheader の Copy を用意する（`prepare_structure` 相当）。
    - break/continue/exit でも pinned 値が破綻しないように header/exit PHI に含める。
- Invariant:
  - ループ内で再代入されない、純粋な不変ローカル・パラメータ。
  - 例: `text_len`, `pattern_len` のような長さ。
  - LoopScope では:
    - preheader 値をそのまま使い、PHI には乗せない（ValueId の二重定義を避ける）。

LoopForm v2 のルール（Rust 実装ベースの確定版）:
- header PHI の対象は **Carrier + Pinned**。Invariant は preheader の値を直接参照し、header では新しい ValueId を割り当てない。
- exit PHI の対象は **Carrier + Pinned +「ループ内 new だが exit で live な body-local」**（BodyLocalInOut）とし、header fall-through と break 経路の値を統合する。
  - ループ内部で完結する一時変数（BodyLocalInternal）は exit PHI に参加しない。

### 4. break / continue の扱い

#### 4.1 continue

- continue は「現在のループの latch ブロックにジャンプして header に戻る」。
- LoopScope では:
  - `ContinueSnaps` に `(block_id, VarSnapshot)` を記録する（block_id は continue が現れたブロック）。
  - `seal_phis` 実行時に:
    - `ContinueSnaps` のスナップショットから Carrier/Pinned 変数の値を集め、
    - header の IncompletePhi（`IncompletePhi { var_name, phi_id, known_inputs }`）に `(continue_block, value)` を追加する。
- 条件: continue は「現在のループスコープ」からのみ脱出し、外側のスコープには影響しない。

#### 4.2 break

- break は「現在のループを脱出し、exit ブロックへ遷移する」。
- LoopScope では:
  - `BreakSnaps` に `(block_id, VarSnapshot)` を記録する。
  - `build_exit_phis` 実行時に:
    - header fall-through の Snapshot （header_exit_snapshot）と `BreakSnaps` をマージし、
    - exit ブロックで PHI を生成する:
      - 1 predecessor のみ → 直接 bind。
      - 2 つ以上 → `phi(header, break1, break2, ...)` を作る。
- ここでも PHI の中心は Carrier + Pinned だが、25.2 以降は「exit で live な body-local」も対象に含める。
  - Invariant は preheader/header の値で十分であり、PHI には乗せない。

### 5. スコープ入出力と変数の「渡し方」

#### 5.1 LoopScope の Env 入出力

- 入力: `Env_in(loop)` = ループ直前（preheader 手前）の `variable_map`。
  - LoopForm v2 はここから:
    - Carrier/Pinned を抽出して PHI 用の構造を準備。
    - preheader に必要な Copy を emit。
- 出力: `Env_out(loop)` = exit ブロック直後の `variable_map`。
  - Carrier/Pinned は exit PHI の結果に更新される。
  - Invariant は `Env_in(loop)` の値をそのまま引き継ぐ。

LoopScope の契約:
- 「ループの外側から見える変数」は Carrier/Pinned/BodyLocalInOut/Invariant すべてだが、
  - ループ内でキャリーされるのは Carrier。
  - ループ内で「箱を固定」するのは Pinned。
  - ループ内で new されつつ exit まで生きるものは BodyLocalInOut として exit PHI に乗る。
  - ループ内で決して変わらない Invariant は `Env_in(loop)` と `Env_out(loop)` で同じ ValueId になる。

#### 5.2 IfScope の Env 入出力

- IfScope も同様に:
  - `Env_in(if)` = pre-if スナップショット。
  - `Env_then_end`, `Env_else_end` を計算し、
  - 変化した変数についてのみ merge PHI を生成（`phi_core::if_phi::merge_modified_at_merge_with`）。
- LoopScope と組み合わせる場合（loop header 内の if など）は:
  - LoopForm が header/body/latch の枠を作り、
  - その中の if は IfScope として φ を 張る。
  - LoopScope は「if によって更新された Carrier/Pinned の最終値」を snapshot として扱い、次の header/latch PHI の入力に使う。

### 6. break/continue を含む複雑パターン

代表的な難パターン:
- ループ内 if の中で break/continue が出るケース:

```hako
loop (i < n) {
  local ch = text.substring(i, i+1)
  if ch == " " {
    i = i + 1
    continue
  }
  if ch == "," {
    break
  }
  i = i + 1
}
```

LoopForm v2 での扱い:
- Carrier: `i`
- Pinned: `text`, `n`、必要に応じて pinned recv（`__pin$*@recv`）。
- IfScope 内で:
  - `continue` → `ContinueSnaps` に i/text/n のスナップショットを保存。
  - `break` → `BreakSnaps` に同様のスナップショットを保存。
- seal/build_exit 時:
  - header PHI: `i` と pinned recv のみを対象に、preheader/latch/continue からの値を統合。
  - exit PHI: `i` や pinned recv を header fall-through と break ブロックから統合。

これにより、「どのスコープからどの変数が外に出るか」「break/continue でどの値が生き残るか」が LoopForm v2 の規則で明示される。

### 7. この設計図の適用範囲

- 対象:
  - Rust MIR builder (`MirBuilder` + `LoopBuilder` + `LoopFormBuilder`) が生成するすべての loop/if/else 構造。
  - JSON v0 Bridge の loop lowering（Bridge 側にも LoopFormOps 実装を追加して同じアルゴリズムを使う）。
- スコープ外（25.1e 時点）:
  - Nyash `.hako` 側 MirBuilder（selfhost builder）の loop/if 実装。
  - try/catch/finally の完全な LoopForm への統合（現状は独自の cf_try_catch ルールで SSA を保っている）。

25.1e では、この設計図をベースに「_find_from ループ」「Stage‑B 最小ループ」などの代表ケースから LoopForm v2 に寄せていき、  
legacy LoopBuilder 側から重複ロジックを削っていくのが次のステップになる。

実装メモ（25.1q 以降の接続）:
- Rust AST ルートでは、Phase 25.1q の作業として LoopBuilder 側に canonical `continue_merge` ブロックを導入し、
  すべての `continue` を一度 `continue_merge` に集約してから `header` に戻す形に正規化済み。
- LoopFormBuilder 側では `continue_snapshots` を `continue_merge` 起点に集約して header PHI を構成しており、
  25.1e で描いた「LoopScope/IfScope の Env_in/out と Carrier/Pinned/Break/ContinueSnaps によるスコープモデル」を、
  実装レベルで Rust AST → MIR 経路に反映し始めている。

### 8. 用語と Rust 実装の対応表（2025-Phase 25.2 時点）

25.1e で定義した用語が、現在どの構造体・フィールドで実装されているかを整理しておくよ。

- LoopScope
  - `src/mir/loop_builder.rs:build_loop_with_loopform` 全体。
  - ループ基本ブロック構造（preheader/header/body/latch/exit/continue_merge）は `LoopShape` として `src/mir/control_form.rs` に記録される。
- IfScope
  - `phi_core::if_phi` 系（`src/mir/phi_core/if_phi.rs`）と、それを呼び出す `MirBuilder::build_if_*` 系。
  - LoopScope 内に現れる if は IfScope として扱われ、LoopScope はその結果の `variable_map` を snapshot して次の PHI 入力に使う。
- Env_in(loop) / Env_out(loop)
  - `Env_in(loop)`:
    - `loop_builder.rs:build_loop_with_loopform` 冒頭の `let current_vars = self.get_current_variable_map();` が LoopScope の入力スナップショット。
    - これが `LoopFormBuilder::prepare_structure(self, &current_vars)` に渡される。
  - `Env_out(loop)`:
    - `LoopFormBuilder::build_exit_phis` 内で exit PHI を構成し、`LoopFormOps::update_var` によって exit ブロック直後の `variable_map` に書き戻された状態。
    - VM 視点では、この `variable_map` が次の文の実行時環境になる。
- Carriers / Pinned / Invariant / BodyLocalInOut
  - `LoopFormBuilder` 内のフィールド:
    - `carriers: Vec<CarrierVariable>` / `pinned: Vec<PinnedVariable>` が Carrier/Pinned に対応。
  - BodyLocalInOut:
    - `LoopFormBuilder::build_exit_phis` 内で `body_local_names` として検出される「exit スナップショットに現れるが carriers/pinned ではない」変数。
    - これらは header での値を `header_vals` に追加した上で、`LoopSnapshotMergeBox::merge_exit` に渡される。
  - Invariant:
    - 上記いずれにも属さず、header/exit で新しい ValueId を割り当てられない変数。
    - preheader での ValueId が exit まで生き残る（MirBuilder の `variable_map` 上で再束縛されない）。
- ContinueSnaps / BreakSnaps
  - ContinueSnaps:
    - `LoopBuilder` のフィールド `continue_snapshots: Vec<(BasicBlockId, HashMap<String, ValueId>)>`。
    - `do_loop_exit(LoopExitKind::Continue)` から登録され、Phase 25.2 では continue_merge ブロック上の PHI を通じて 1 つの `merged_snapshot` に統合されてから `LoopFormBuilder::seal_phis` に渡される。
  - BreakSnaps:
    - `LoopBuilder` のフィールド `exit_snapshots: Vec<(BasicBlockId, HashMap<String, ValueId>)>`。
    - `do_loop_exit(LoopExitKind::Break)` から登録され、`LoopFormBuilder::build_exit_phis` で `LoopSnapshotMergeBox::merge_exit` に渡される。
- LoopSnapshotMergeBox（Phase 25.2）
  - ファイル: `src/mir/phi_core/loop_snapshot_merge.rs`。
  - 25.1e の「4. break / continue の扱い」で定義した:
    - ContinueSnaps → header PHI 入力
    - BreakSnaps + header fallthrough → exit PHI 入力
    のルールを、実装として引き受ける箱。
  - header 側:
    - `LoopBuilder::build_loop_with_loopform` で continue_merge ブロックの PHI 入力構成に `optimize_same_value` / `sanitize_inputs` を使用し、その結果を 1 つの snapshot にまとめて `LoopFormBuilder::seal_phis` に渡す。
  - exit 側:
    - `LoopFormBuilder::build_exit_phis` で header 値 + filtered exit_snapshots + body-local を `LoopSnapshotMergeBox::merge_exit` に渡し、PHI 入力ベクタを構成したうえで `optimize_same_value` / `sanitize_inputs` → PHI emit を行う。
- Legacy ルート
  - `phi_core::loop_phi`（`prepare_loop_variables_with` / `seal_incomplete_phis_with` / `build_exit_phis_with`）は、
    - Rust AST ルートでは既に廃止済みで、
    - JSON v0 Bridge (`src/runner/json_v0_bridge/lowering/loop_.rs`) からのみ互換 API として利用されている。
  - 新規ループ実装はすべて LoopForm v2 + LoopSnapshotMergeBox 経由で SSA/PHI を構成し、legacy API は削除候補として閉じ込めている。

この対応表により、「25.1e の設計用語」と「2025-Phase 25.2 時点の Rust 実装」の差分がゼロになるようにしているよ。

## スコープ外 / 後続フェーズ候補

- Nyash 側 MirBuilder（.hako 実装）の LoopForm 対応:
  - ここでは Rust MIR builder 側の LoopForm/PHI を整えることに集中し、`.hako` 側 MirBuilder への LoopForm 移植は Phase 25.1f 以降のタスクとする。
- ループ最適化（unrolling / strength reduction など）:
  - 25.1e はあくまで「正しい SSA/PHI を作る」のが目的であり、性能最適化は Phase 26+ で扱う。

## メモ（現状の観測ログ）

- `mir_stage1_using_resolver_full_collect_entries_verifies` 実行時:
  - `_find_from` / `collect_entries` 内で多数の PHI が生成されているが、header/merge ブロックで既存の ValueId と衝突して `Value %N defined multiple times` が発生。
  - merge ブロック（bb54 など）で predecessor（bb59, bb61）由来の値を PHI なしで読んでいる箇所があり、`non‑dominating use` / `Merge block uses predecessor-defined value without Phi` がレポートされている。
- `LoopBuilder::prepare_loop_variables` による「全変数 PHI 化」は、LocalSSA＋LoopForm の両方が入った状態では過剰であり、キャリア＋pinned のみに制限する必要があることが分かっている。
