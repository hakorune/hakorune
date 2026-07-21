#!/usr/bin/env python3
"""HEADERPORT0-HDR0-M0 production ``current_module`` reader census.

This is an inventory guard, not a migration.  It records the exact legacy
sites that still observe the Builder-owned module and rejects new sites until
HDR0-P0 assigns them an explicit replacement.  Test fixtures and comments are
kept as named non-reader observations so a broad source scan cannot silently
lose coverage.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class ReaderRow:
    path: str
    anchor: str
    family: str
    owner: str


ROWS = (
    # Header-only readers that must receive LoweringHeaderPortV1 in HDR0-P0.
    ReaderRow(
        "src/mir/builder/located_legacy_lowering.rs",
        "module as &dyn FunctionSignatureLookupV1",
        "diagnostic_observation",
        "disconnected located legacy call/finalizer hint",
    ),
    ReaderRow(
        "src/mir/builder/recursive_child_lowering.rs",
        "module as &dyn FunctionSignatureLookupV1",
        "route_header",
        "raw recursive child call/finalizer hint",
    ),
    ReaderRow(
        "src/mir/builder/calls/annotation.rs",
        "FunctionSignatureLookupV1::signature(module, func_name.as_ref())",
        "route_header",
        "call-result signature annotation",
    ),
    ReaderRow(
        "src/mir/builder/calls/lowering.rs",
        "let module = self.current_module.take()",
        "route_header",
        "finalizer Call/Await header loan",
    ),
    ReaderRow(
        "src/mir/builder/rewrite/known.rs",
        ".and_then(|module| module.functions.get(fname))",
        "route_header",
        "known rewrite signature arity",
    ),
    ReaderRow(
        "src/mir/builder/rewrite/known.rs",
        ".is_some_and(|module| module.functions.contains_key(&fname))",
        "route_header",
        "known rewrite static-presence policy",
    ),
    ReaderRow(
        "src/mir/builder/builder_method_index.rs",
        "names.extend(module.functions.keys().cloned())",
        "route_header",
        "method-tail index projection",
    ),
    ReaderRow(
        "src/mir/builder/builder_method_index.rs",
        "self.comp_ctx.method_tail_index_source_len != refmod.functions.len()",
        "route_header",
        "method-tail index freshness",
    ),
    # These three direct checks are retained only as explicit legacy/fallback
    # inventory.  They are not permitted to become a HeaderPort authority.
    ReaderRow(
        "src/mir/builder/builder_build.rs",
        "module.functions.contains_key(&lowered)",
        "forbidden_fallback",
        "constructor/birth lowered-function probe",
    ),
    ReaderRow(
        "src/mir/builder/calls/static_resolution.rs",
        ".functions\n                    .keys()",
        "forbidden_fallback",
        "dev-only suffix tail resolver",
    ),
    ReaderRow(
        "src/mir/builder/calls/materializer.rs",
        ".is_some_and(|module| module.functions.contains_key(name))",
        "forbidden_fallback",
        "direct module-function materializer probe",
    ),
    # Module storage/lifecycle observations are not header queries.
    ReaderRow(
        "src/mir/builder/builder_metadata.rs",
        "self.current_module\n            .as_mut()\n            .map(|module| module.intern_closure_body(body))",
        "shell_lifecycle",
        "closure-body interning",
    ),
    ReaderRow(
        "src/mir/builder/indexing.rs",
        "crate::mir::static_data_plan::find_static_data_plan(",
        "shell_lifecycle",
        "static-data plan lookup",
    ),
    ReaderRow(
        "src/mir/builder/module_lifecycle.rs",
        "self.current_module = Some(module)",
        "shell_lifecycle",
        "module invocation installation",
    ),
    ReaderRow(
        "src/mir/builder/module_lifecycle.rs",
        "let mut module = self.current_module.take().unwrap()",
        "shell_lifecycle",
        "module finalization take",
    ),
    ReaderRow(
        "src/mir/builder/calls/function_session.rs",
        "publish_function_draft(\n            self.builder.current_module.as_mut()",
        "shell_lifecycle",
        "function-draft publication",
    ),
    ReaderRow(
        "src/mir/builder/calls/lowering.rs",
        "self.current_module = module",
        "shell_lifecycle",
        "finalizer module restore",
    ),
    ReaderRow(
        "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
        "fn publish_recursive_callable_drafts",
        "canonical_catalog",
        "recursive callable batch publication",
    ),
    ReaderRow(
        "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
        "fn publish_callable_drafts",
        "canonical_catalog",
        "acyclic/trivial callable batch publication",
    ),
    ReaderRow(
        "src/mir/builder/resolved_lowering/mod.rs",
        ".try_add_function(draft)",
        "canonical_catalog",
        "single resolved-function publication",
    ),
)

NON_READER_ANCHORS = (
    ("src/mir/builder/builder_init.rs", "current_module: None"),
    ("src/mir/builder/indexing.rs", "builder.current_module = Some(module)"),
    ("src/mir/builder/module_completion_candidate.rs", "all_current_module_products"),
    ("src/mir/builder/module_lowering_access_port.rs", "reach through `current_module`"),
    ("src/mir/builder/module_lowering_invocation_candidate.rs", "a `current_module` view"),
    ("src/mir/builder/module_lowering_shell.rs", "reach through `current_module`"),
)

# Exact occurrence counts keep this census fail-fast without hard-coding line
# numbers.  A new file or an extra occurrence in an existing file requires a
# new HDR0 row instead of silently entering the legacy surface.
EXPECTED_OCCURRENCE_COUNTS = {
    "src/mir/builder/builder_build.rs": 1,
    "src/mir/builder/builder_init.rs": 1,
    "src/mir/builder/builder_metadata.rs": 1,
    "src/mir/builder/builder_method_index.rs": 2,
    "src/mir/builder/calls/annotation.rs": 1,
    "src/mir/builder/calls/function_session.rs": 1,
    "src/mir/builder/calls/lowering.rs": 2,
    "src/mir/builder/calls/materializer.rs": 1,
    "src/mir/builder/calls/static_resolution.rs": 1,
    "src/mir/builder/indexing.rs": 2,
    "src/mir/builder/located_legacy_lowering.rs": 1,
    "src/mir/builder/module_completion_candidate.rs": 2,
    "src/mir/builder/module_lifecycle.rs": 3,
    "src/mir/builder/module_lowering_access_port.rs": 1,
    "src/mir/builder/module_lowering_invocation_candidate.rs": 1,
    "src/mir/builder/module_lowering_shell.rs": 1,
    "src/mir/builder/recursive_child_lowering.rs": 1,
    "src/mir/builder/resolved_lowering/callable_module_transaction.rs": 2,
    "src/mir/builder/resolved_lowering/mod.rs": 1,
    "src/mir/builder/rewrite/known.rs": 3,
}


def _production_lines(root: Path) -> list[tuple[str, int, str]]:
    lines: list[tuple[str, int, str]] = []
    for path in sorted((root / "src/mir/builder").rglob("*.rs")):
        if path.name.endswith("_tests.rs") or path.name == "tests.rs":
            continue
        relative = path.relative_to(root).as_posix()
        for number, line in enumerate(path.read_text().splitlines(), 1):
            if "current_module" in line:
                lines.append((relative, number, line.strip()))
    return lines


def verify_header_reader_census(root: Path) -> None:
    source_by_path = {
        path.relative_to(root).as_posix(): path.read_text()
        for path in (root / "src/mir/builder").rglob("*.rs")
        if not (path.name.endswith("_tests.rs") or path.name == "tests.rs")
    }
    for row in ROWS:
        text = source_by_path.get(row.path)
        if text is None:
            raise AssertionError(f"HDR0-M0 missing source file: {row.path}")
        if row.anchor not in text:
            raise AssertionError(f"HDR0-M0 missing {row.family} anchor: {row.path}: {row.anchor}")

    occurrences = _production_lines(root)
    actual_counts: dict[str, int] = {}
    for path, _, _ in occurrences:
        actual_counts[path] = actual_counts.get(path, 0) + 1
    if actual_counts != EXPECTED_OCCURRENCE_COUNTS:
        raise AssertionError(
            "HDR0-M0 current_module occurrence drift: "
            f"expected={EXPECTED_OCCURRENCE_COUNTS!r} actual={actual_counts!r}"
        )
    for path, _, line in occurrences:
        if any(path == exempt_path and exempt_anchor in line
               for exempt_path, exempt_anchor in NON_READER_ANCHORS):
            continue
        if path not in {row.path for row in ROWS}:
            raise AssertionError(f"HDR0-M0 uncategorized current_module occurrence: {path}: {line}")

    families = {row.family for row in ROWS}
    expected = {
        "route_header",
        "canonical_catalog",
        "shell_lifecycle",
        "forbidden_fallback",
        "diagnostic_observation",
    }
    if families != expected:
        raise AssertionError(f"HDR0-M0 family vocabulary drift: {sorted(families)}")
    counts = {family: sum(row.family == family for row in ROWS) for family in sorted(families)}
    print(
        "[headerport-hdr0-m0] ok "
        f"rows={len(ROWS)} route_header={counts['route_header']} "
        f"canonical_catalog={counts['canonical_catalog']} "
        f"shell_lifecycle={counts['shell_lifecycle']} "
        f"forbidden_fallback={counts['forbidden_fallback']} "
        f"diagnostic_observation={counts['diagnostic_observation']}"
    )


if __name__ == "__main__":
    verify_header_reader_census(Path(__file__).resolve().parents[3])
