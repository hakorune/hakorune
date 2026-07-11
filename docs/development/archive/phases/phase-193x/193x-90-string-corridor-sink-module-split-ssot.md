# 193x-90 String Corridor Sink Module Split SSOT

Decision
- `src/mir/passes/string_corridor_sink` is split by responsibility, not by benchmark or route.
- the top-level sink remains a thin orchestration facade.
- shared helper logic, retained-len rewrites, concat corridor rewrites, fusion rules, publication handling, and tests live in separate files.

Rules
- this phase is BoxShape-only.
- no new string semantics are introduced here.
- no publication barrier policy changes are introduced here.
- no benchmark-specific helpers are added.

Module boundaries
- `mod.rs`: orchestration entry and public constants
- `shared.rs`: generic sink helpers shared by multiple rewrite families
- `retained_len.rs`: retained-slice len rewrite path
- `concat_corridor.rs`: concat corridor rewrite path
- `fusion.rs`: complementary helper fusion path
- `publication.rs`: publication sink path
- `tests/`: topic-scoped sink test modules

Acceptance
- compile and tests remain green with the new module seam
- later work refers to `src/mir/passes/string_corridor_sink/` instead of the retired monolithic file
