# pagequeue-mixed-base-helper-proof

Purpose: fixture for the PageQueue helper-extraction homework.

This proof pins the mixed-base same-module helper shape that MIM-059 kept
parked:

```text
receiver writes:
  me.last_selected_*
foreign read:
  page.page_id
foreign handle publication:
  me.last_selected_page = page
```

Decision:
- Do not widen generic `@rune Inline(required)` for this mixed-base shape.
- Non-inline same-module lowering must not coredump. It may either lower
  correctly or fail at compile time with a clear diagnostic.
- A future inline route must come through a narrow publication recipe, not a
  generic multi-base inline verifier.

Acceptance:
- `test.sh` runs VM parity, MIR route proof, pure-first EXE build, and EXE run.
- The EXE path must consume the same-module method route and exit with `0`.
- This fixture is not a request to inline the helper.
