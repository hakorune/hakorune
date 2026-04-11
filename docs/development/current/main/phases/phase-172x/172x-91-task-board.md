# 172x-91: substring concat stable-length exact-route task board

## Board

- [x] `172xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `172xB` relation helper
  - add narrow `string_corridor_relations` JSON reader
- [x] `172xC` exact-route switch
  - keep the current matcher contract
  - consume `stable_length_scalar`
  - switch the exact seed to `substring_concat_len_ir(...)`
  - keep the old text-rotation route as fallback
- [x] `172xD` verify
  - rerun direct emit shape smoke
  - rerun phi-merge contract smoke
  - rerun exact asm/perf on `kilo_micro_substring_concat`
  - rerun `tools/checks/dev_gate.sh quick`
- [x] `172xE` closeout
  - sync README / `CURRENT_TASK` / `10-Now`
  - record the exact reread after the stable-length route switch

## Notes

- this is an exact-route-local consumer cut
- it consumes already-landed metadata; it does not widen metadata
- broader `return` / `store` / host-boundary publication stays separate
