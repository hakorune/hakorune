# Phase 93x Task Board

| Step | Status | Notes |
| --- | --- | --- |
| `93xA1` archive-later helper inventory lock | landed | worker inventory を確定 |
| `93xA2` archive target ranking / boundary freeze | landed | `keep-now` / `archive-later` を切り分け |
| `93xB1` archive move and doc repoint | landed | helper scripts を archive bucket へ退避し current/docs を repoint |
| `93xC1` proof refresh | landed | `bash -n` / `git diff --check` / live-ref audit PASS |
| `93xD1` closeout | landed | current pointers を thin に保って終了 |

## Done Criteria

- live surface に残る legacy engineering helpers が archive path だけになる
- current docs の stale wording が増えない
- `CURRENT_TASK.md` が pointer のまま薄い
- closeout complete; next lane is the existing `phase-94` task
