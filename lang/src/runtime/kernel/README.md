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
- `numeric` (`numeric/matrix_i64.hako`)
  - current public surface: `MatI64.mul_naive`
  - ring1 numeric wrapper stays in `lang/src/runtime/numeric/` and delegates into this kernel owner

Next family order:
- `array/`
- `numeric/`
- `map` is kept in `lang/src/runtime/collections/` ring1, not in this kernel lane

Array first narrow op:
- `ArrayBox.length/len/size` observer path stays in `lang/src/runtime/collections/array_core_box.hako` first
- defer a new `lang/src/runtime/kernel/array/` module until a concrete policy difference appears
  - promotion is trigger-based, not calendar-based: move only when the ring1 wrapper is no longer thin enough (owner-local policy / normalization / birth handling, or a dedicated acceptance case that cannot stay as a wrapper-only lane)

Non-goals:
- No direct host category routing.
- No plugin/loader process orchestration.
