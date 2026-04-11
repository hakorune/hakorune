# 171x-91: substring concat exact-seed loop-shape task board

## Board

- [x] `171xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `171xB` exact seed loop-shape cut
  - keep the matcher contract unchanged
  - recut `hako_llvmc_emit_substring_concat_loop_ir(...)` to a bottom-tested loop
- [x] `171xC` verify
  - rerun direct emit shape smoke
  - rerun phi-merge contract smoke
  - rerun exact asm/perf on `kilo_micro_substring_concat`
  - rerun `tools/checks/dev_gate.sh quick`
- [ ] `171xD` closeout
  - sync README / `CURRENT_TASK` / `10-Now`
  - record the exact reread after the loop-shape cut

## Notes

- this is an exact-front keeper cut, not a new generic string-family widening
- broader `return` / `store` / host-boundary publication stays separate
- `phi_merge` widening remains a separate metadata-contract phase
- current result is green but still below the keeper threshold only by a small margin; the next follow-on should stay exact-route-local
