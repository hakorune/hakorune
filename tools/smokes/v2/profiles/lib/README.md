# profiles/lib compatibility shims

`tools/smokes/v2/profiles/lib/*.sh` are thin compatibility wrappers.

- Purpose: keep legacy profile scripts working while the shared implementation lives in `tools/smokes/v2/lib/`.
- Contract: wrappers must only `source` the matching file from `tools/smokes/v2/lib/` and should not add behavior.
- Migration rule: new logic goes to `tools/smokes/v2/lib/`; wrappers stay stable and minimal.

