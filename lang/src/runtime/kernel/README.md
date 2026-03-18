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

Next family order:
- `array/`
- `numeric/`
- `map` is kept in `lang/src/runtime/collections/` ring1, not in this kernel lane

Array first narrow op:
- `ArrayBox.length/len/size` observer path stays in `lang/src/runtime/collections/array_core_box.hako` first
- defer a new `lang/src/runtime/kernel/array/` module until a concrete policy difference appears

Non-goals:
- No direct host category routing.
- No plugin/loader process orchestration.
