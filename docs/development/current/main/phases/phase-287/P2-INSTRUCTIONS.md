# Phase 287 P2（optional）: quick を「45秒」へ寄せる指示書

## 目的

- `tools/smokes/v2/run.sh --profile quick` を **~45秒以内**へ寄せる。
- quick の責務を「最小ゲート」に寄せ、網羅・重い導線は integration/full/plugins に移す。
- 意味論は不変（テスト分類/配置のみ）。

## 前提（P1の到達点）

- quick は **55秒 / 447 tests** まで改善済み。
- ここからの削減は「残っている遅いテストをさらに移す」だけで狙う（runner改造や並列化はP3）。

## 成功条件（受け入れ基準）

- quick が **45秒以内**（目安）で完走する。
- 速さ優先。テスト本数 ~100 は“理想”だが、P2 では **時間を第一**にする。
- `--filter` の導線を壊さない（移動は相対パス維持を基本にする）。

## 手順

### 0) まずP1の成果をコミット（推奨）

P2 を始める前に、P1 を1コミットに固める（後戻りを容易にする）。

```bash
git add -A
git commit -m "smokes(v2): slim quick profile (Phase 287 P1)"
```

### 1) quick の再計測（現状の遅い上位を出す）

```bash
./tools/smokes/v2/measure_test_times.sh quick /tmp/smoke_test_times_quick_p2.txt
head -50 /tmp/smoke_test_times_quick_p2.txt.sorted
```

ここで「上位（遅い順）に何が残っているか」を確認する。

### 2) “遅いファミリー” を integration へ寄せる（構造的に解決）

基本方針:
- まず **ディレクトリ単位で `git mv`**（最小手数・効果大）。
- 例外的に“その中の1本だけ quick に残したい”場合のみ個別に戻す。
- 移動先は `tools/smokes/v2/profiles/integration/` で、**相対パス階層を維持**する。

例（イメージ）:

```bash
git mv tools/smokes/v2/profiles/quick/core/phaseXXXX tools/smokes/v2/profiles/integration/core/
```

判定基準（P2で移す寄せ方）:
- 1本が重い（>0.5s 以上が繰り返し出る）
- crate exe / S3 / selfhost / 長尺の外部I/O / 大量fixture を含む
- “壊れたら困る入口” ではない（quickの責務外）

### 3) 45秒に届くまで「上位を削る」を繰り返す

```bash
time ./tools/smokes/v2/run.sh --profile quick
```

- 45秒を切ったらP2は終了。
- 切らない場合は、計測の上位から追加で移す。

### 4) docs を更新（SSOTを合わせる）

- `docs/development/current/main/phases/phase-287/README.md` に P2 の Before/After（秒・本数）を追記。
- `tools/smokes/v2/README.md` の目安（quick=~45s）は“達成/未達”を明記（未達なら次の方針も書く）。

## 非ゴール（P2ではやらない）

- runner の並列化（`--jobs` を実装して速くする）: **Phase 287 P3**
- quick を “マニフェスト（明示リスト）” 管理に変える: 必要なら別提案（runner変更を伴う）

