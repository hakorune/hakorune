# runtime/kernel/array

Place `.hako` array kernel logic here.

Examples:
- current first narrow op: `ArrayBox.length/len/size` observer path in `lang/src/runtime/collections/array_core_box.hako`
- keep a new kernel module deferred until a concrete policy difference appears
  - promotion is trigger-based: move here only when the collections ring1 wrapper is no longer thin enough to stay transport-only (owner-local policy / normalization / birth handling, or a dedicated acceptance row + smoke that cannot stay in ring1)
- get/set fast-path policy
- bounds and shape contract handling
