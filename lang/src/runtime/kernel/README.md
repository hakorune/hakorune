# lang/src/runtime/kernel — .hako Runtime Kernel

Scope:
- Home for `.hako` runtime kernel logic (`*_hk` bench line).
- Keep behavior/policy here; keep host-call transport in `../host/`.

Subdirectories:
- `string/`: string kernel routines and policies.
- `array/`: array/list kernel routines and policies.
- `numeric/`: numeric kernel routines and policies.

Current narrow pilot:
- `string.search` (`string/search.hako`)
  - current public surface: `find_index` / `contains` / `starts_with` / `ends_with` / `split_once_index`

Non-goals:
- No direct host category routing.
- No plugin/loader process orchestration.
