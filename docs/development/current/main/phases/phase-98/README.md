# Phase 98: Plugin loader fail-fast + LLVM parityの持続化

- 目的: Phase 97 で復旧した FileBox/MapBox plugin を「存在チェック＋strict fail-fast」で固め、LLVM EXE parity を日常運用で維持する。
- ポイント: HAKO_JOINIR_STRICT=1 で missing .so を即座に止める／strict=0 では best-effort 継続＋[plugin/missing] ログを出す。新しい env は増やさない。
- 成果物: Phase 97 の2本 smoke（LLVM EXE）が plugin ビルド済みなら高速通過、欠落時は build だけ走らせて PASS まで持っていく。
- AOT/LLVM EXE exit code: IntegerBox 返却時のみその値を exit code にし、それ以外は 0（VM と整合）。***
