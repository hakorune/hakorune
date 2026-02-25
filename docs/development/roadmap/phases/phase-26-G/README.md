# Phase 26-G — Exit Liveness MIR Scan (実装フェーズ)

Status: planning

## ゴール（何を達成するフェーズか）

- ExitLivenessProvider の本実装（MIR 命令列スキャン版）を作り、FuncScanner のカナリア
  - `mir_funcscanner_skip_ws_direct_vm`
  - `mir_funcscanner_parse_params_trim_min_verify_and_vm`
  を緑にする。
- 26-F で用意した差し替え口（MirScanExitLiveness）に、use/def ベースの liveness を実装する。
- `NYASH_EXIT_LIVE_ENABLE=1` で MIR スキャン版を有効化し、デフォルトは従来挙動のまま安全側。

## スコープ（このフェーズでやること）

- LoopFormOps / LoopBuilder / JSON bridge (LoopFormJsonOps) に「MirFunction への参照を返す API」を追加する設計と最小実装。
- MirScanExitLiveness を `MirQuery` ベースの use/def スキャンに置き換える。
- 代表テスト（FuncScanner カナリア）で緑を確認するところまで。

## 実装ステップ（順序）

### Step 1: API 設計と LoopFormOps 拡張

- 目的: Exit 側から MirFunction（または MirQuery）に到達できるようにする。
- 具体案:
  - `LoopFormOps` に「MirFunction への参照を返すメソッド」を追加する:
    - 例: `fn mir_function(&self) -> &MirFunction;`
  - 実装箇所:
    - `impl<'a> LoopFormOps for LoopBuilder<'a>`:
      - `self.parent_builder.current_function.as_ref().expect(...)` 経由で &MirFunction を返す。
    - `impl LoopFormOps for LoopFormJsonOps<'_>`:
      - 既に `f: &mut MirFunction` を保持しているので、その参照を返す。
    - テスト用 `MockOps` など:
      - 最小限の MirFunction スタブを持つ or `unimplemented!()` でテストを段階的に更新。

### Step 2: ExitPhiBuilder に MirQuery フックを追加

- 目的: Exit PHI 生成時に MirQuery にアクセスできるようにする。
- 具体案:
  - `ExitPhiBuilder::build_exit_phis` のシグネチャを拡張:
    - 現状: `fn build_exit_phis<O: LoopFormOps>(..., exit_snapshots, pinned, carrier) -> Result<(), String>`
    - 追加: `mir_query: &dyn MirQuery` を引数に足す、または内部で `ops.mir_function()` から `MirQueryBox` を組み立てる。
  - 検討:
    - a) `build_exit_phis` に `&dyn MirQuery` を直渡し → テストで差し替えやすい。
    - b) `LoopFormOps` に `mir_function()` を足し、`ExitPhiBuilder` 側で `MirQueryBox::new(ops.mir_function())` を作る。
  - このフェーズでは b) 案を優先（既に LoopBuilder/JSON bridge は MirFunction を握っているため）。

### Step 3: MirScanExitLiveness の本実装（use/def スキャン）

- 前提: Step 2 で `MirQueryBox` を構築できる。
- 実装方針:
  - 入口:
    - `MirScanExitLiveness::compute_live_at_exit(header_vals, exit_snapshots)` の中で
      - `MirQueryBox` と `exit_blocks` を受け取れるようにする（必要なら `ExitLivenessProvider` トレイトのシグネチャ拡張も検討）。
  - 最小アルゴリズム:
    1. スキャン対象ブロック集合:
       - exit ブロック（LoopShape.exit）＋ exit_snapshots に現れる break 元ブロック。
    2. 初期 live 集合:
       - 対象ブロックの命令を後ろから走査し、`reads_of(inst)` を live に追加。
    3. 1-step backward 伝播:
       - それぞれのブロックで `writes_of(inst)` で kill、`reads_of(inst)` で add。
       - `succs(bb)` が対象集合に含まれている場合、succ の live を bb に流す。
    4. 固定点反復:
       - live 集合が変わらなくなるまで 2〜3 を繰り返す。
    5. 名前へのマッピング:
       - header_vals / exit_snapshots に現れる `(name, ValueId)` を逆引きテーブルに集約し、
         live に含まれる ValueId に対応する name だけを `live_at_exit` に含める。
  - 返り値:
    - `BTreeSet<String>`（BodyLocalPhiBuilder からそのまま使える）。
  - 既存ロジックとの整合:
    - BodyLocalInternal でも「exit後で本当に live」なものだけが rescue されることが期待される。

### Step 4: ExitPhiBuilder / BodyLocalPhiBuilder 統合確認

- 目的: 新しい live_at_exit を使っても PhiInvariantsBox で落ちないことを確認する。
- 作業:
  - `BodyLocalPhiBuilder::filter_exit_phi_candidates` が
    - `class.needs_exit_phi()` (Pinned/Carrier/BodyLocalExit) と
    - `live_at_exit` + `is_available_in_all` の OR を行っていることを再確認。
  - `PhiInvariantsBox::ensure_exit_phi_availability` が
    - 「選ばれた変数はすべての exit pred で定義済み」であることを保証し、
    - それでも穴があれば即 Fail-Fast で教えてくれることを前提に、MirScan 側のロジックを調整。

### Step 5: カナリア検証

- コマンド:
  - `NYASH_EXIT_LIVE_ENABLE=1 cargo test --release --lib mir_funcscanner_skip_ws_direct_vm`
  - `NYASH_EXIT_LIVE_ENABLE=1 cargo test --release --lib mir_funcscanner_parse_params_trim_min_verify_and_vm`
- 期待:
  - MirVerifier が `use of undefined value` を報告しない。
  - VM 実行で RC が期待通り（skip_ws の sentinel が正しく動き、trim/parse_params も undefined を出さない）。

### Step 6: ドキュメント更新

- `loopform_ssot.md` に:
  - MirQuery / MirQueryBox の役割と ExitLiveness との関係。
  - 「MirScanExitLiveness は 26-G で use/def スキャン実装済み」と記載。
- `phase-26-G/README.md` 自体も「実装完了」セクションを追記。

## やらないこと

- Loop 形状や PHI 生成ロジックの意味変更（ExitPhiBuilder/BodyLocalPhiBuilder のアルゴリズム変更はしない）。
- env 名の変更や追加（既存の `NYASH_EXIT_LIVE_ENABLE` を継続利用）。

## 受け入れ条件

- `NYASH_EXIT_LIVE_ENABLE=0/未設定` で従来のテスト結果を維持。
- `NYASH_EXIT_LIVE_ENABLE=1` で FuncScanner カナリアが緑（MirVerifier/VM）。
- docs 更新: 26-F/loopform_ssot に「MIR スキャン実装済み」を追記。
