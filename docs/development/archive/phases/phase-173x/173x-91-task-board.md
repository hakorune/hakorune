# 173x-91: return-boundary publication sink task board

## Board

- [x] `173xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `173xB` narrow sink consumer
  - same-block `return` plan collection
  - same-block helper sink rewrite
  - copy-only alias deletion
- [x] `173xC` focused contract
  - add unit test for helper-result direct `return`
  - add focused unit smoke wrapper
- [x] `173xD` verify
  - rerun focused string unit tests
  - rerun `phase137x_string_publication_return_unit.sh`
  - rerun string guardrail smokes
  - rerun exact asm/perf on `kilo_micro_substring_concat`
  - rerun `tools/checks/dev_gate.sh quick`
- [x] `173xE` closeout
  - sync README / `CURRENT_TASK` / `10-Now`
  - record the direct-return route as landed or hold it if the acceptance contract stays red

## Notes

- this is a same-block `return` publication sink cut only
- it must use existing plan metadata
- broader `store` / host-boundary publication stays separate
- final emitted-MIR return-carrier cleanup stays separate
