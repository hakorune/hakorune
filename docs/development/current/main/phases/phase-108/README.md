# Phase 108 — PostLoopEarlyReturn 一般化 + Pattern2 policy router SSOT（Active）

目的: balanced_depth_scan_policy 由来の post-loop early return を policy 共通の plan として独立させ、Pattern2 の ApplyPolicyStepBox を policy router 1 本に統一する（入口SSOT）。
受け入れ基準: Phase 107（find_balanced_array/object）の VM/LLVM EXE parity が意味論不変のまま PASS、かつ Phase 97/100/104/94 の代表 integration smoke が退行しない。
メモ: 新しい環境変数は追加しない（既存 `HAKO_JOINIR_STRICT` / `NYASH_JOINIR_DEV` のみ）。

