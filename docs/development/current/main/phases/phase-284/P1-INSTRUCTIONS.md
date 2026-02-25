# Phase 284 P1（code）: Return as ExitKind SSOT（実装）

目的: `return` を pattern 固有の特例にせず、`ExitKind::Return` と `compose::*` / `emit_frag()` へ収束させる。

前提SSOT（P0）:
- `docs/development/current/main/phases/phase-284/README.md`
- Phase 282 の境界ルール（SSOT=extract / close-but-unsupported=Err）: `docs/development/current/main/phases/phase-282/README.md`

## 実装方針（最小）

### 1) 返り値の運搬（ExitKind::Return + args）

- `return <expr>` は **`ExitKind::Return` の edge**として表現する。
- Return edge が持つ値は `EdgeArgs` で運ぶ（Return terminator の operand）。
- terminator は `emit_frag()` が生成する（pattern/box が直に Return 命令を生やさない）。

### 2) 「移行期間の穴」を消す

現状は Pattern4/5 などが `return` を `Err(close-but-unsupported)` にしている。
P1 のゴールは:
- `return` を含む loop-body が “別パターンへ静かに流れる” 状態をなくす
- SSOT 経路で `ExitKind::Return` に落ちるようにする

補足（設計意図）:
- Phase 284 は “return だけ” を特別扱いするのではなく、**Exit の語彙（ExitKind）を SSOT 化**するフェーズでもある。
- `return` を「条件付き Jump の一種」として扱えるようにしておくと、将来の `break/continue` / `throw` も同じ導線に乗る。

## 実装タスク（推奨順）

### Step 1: 現状の `return` ハンドリングを棚卸し（read-only）

- joinir patterns extractors:
  - `src/mir/builder/control_flow/joinir/patterns/extractors/pattern4.rs`
  - `src/mir/builder/control_flow/joinir/patterns/extractors/pattern5.rs`
  - `return` を Err にしている箇所（close-but-unsupported の根拠）を列挙する

- control-flow lowering:
  - `emit_frag()` が Return edge をどう生成しているか確認する（target=None の Return wire/exit）
  - `compose::cleanup()` の Return wiring が想定どおりか確認する

成果物: `docs/development/current/main/phases/phase-284/P1-NOTES.md`（短い箇条書きでOK）

### Step 2: `return` を ExitKind に落とす “単一入口” を作る（root fix）

狙い:
- loop body のどの位置でも `return` が現れたら `ExitKind::Return` で外へ出せること
- これを **1 箇所**に寄せる（pattern 側に増やさない）

重要: Phase 284 で一番の迷子ポイントは「どこに寄せるか」なので、先に経路を固定する。

#### A) Plan line と JoinIR line を混同しない（必須）

- **Plan line（Pattern6/7）**: `src/mir/builder/control_flow/plan/normalizer.rs` が Frag 構築 SSOT。
  `compose::cleanup()` / `emit_frag()` へ寄せるのが正しい。
- **JoinIR line（Pattern1–5,9）**: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs` が共通入口。
  Pattern4/5 の root fix を `plan/normalizer.rs` に寄せても効かない（経路が違う）。

#### B) root fix の候補（JoinIR line を対象にする）

実装候補（どれか 1 つに決める）:
- B1) JoinIR lowerer 側に “Return collector” を 1 箇所だけ追加し、Pattern4/5 はそれを呼ぶだけにする
  - 方針: まずは **fixture で使う形だけ**を対応（例: top-level if の then に return）
  - 未対応形は Err（silent fallback 禁止）
- B2) JoinIR→MIR 変換/merge の共通入口へ正規化を寄せる（`conversion_pipeline.rs` / `merge/mod.rs`）
  - ただし、JoinModule に return 相当が無いと後段で作れないので、ここは “最後に出る地点” にしないこと
  - 実体としては B1 が先に必要になるケースが多い

重要:
- `conversion_pipeline.rs` は JoinModule 構築の後で走るため、「return の配線を作る」責務をそこへ押し込むと設計が歪みやすい。
  P1 の最小は「JoinIR lowering の入口（JoinInst を作る地点）で return を ExitKind に落とす」こと。

要件:
- Fail-Fast: “表現できない return” は Err（silent fallback 禁止）
- 既定挙動は変えない（return を含む既存 fixture があれば、その期待値は明示して更新）

### Step 3: extractor の `return` ポリシーを更新（穴を埋める）

P1 で Return SSOT が通るようになったら、以下を更新する:
- Pattern4/5 の extractor で `return` を **Err にしない**（close-but-unsupported ではなくなるため）
- ただし “return があるせいでパターン形状が曖昧になる” 場合は Err を維持（Fail-Fast）

### Step 4: fixture + smoke（VM/LLVM）で SSOT を固定

最小 fixture の要件:
- `return` が loop の then/else どちらかに現れる
- exit code が安定（stdout 抑制の LLVM でも確認できる）

例（案）:
- `apps/tests/phase284_p1_return_in_loop_min.hako`
  - loop 内で条件により `return 7` / `continue` 等
  - 最終 exit code を 7 に固定

smoke:
- VM: stdout/exit code を検証
- LLVM: exit code + harness の `Result: <code>` を検証（stdout が出ない想定）

## 受け入れ基準

- `return` を含む loop fixture が VM/LLVM で同一動作
- pattern 側に “return の特例 if” が増えていない（root fix のみ）
- `Ok(None)` / `Err` の境界が崩れていない（silent fallback なし）
