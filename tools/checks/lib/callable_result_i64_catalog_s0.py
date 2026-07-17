#!/usr/bin/env python3
"""Guard disconnected local-body exact-i64 callable result catalog S0a."""

from __future__ import annotations

import argparse
from pathlib import Path
import re


MODULE = Path("src/mir/callable_result_representation")
PRODUCT = "VerifiedSameModuleCallableResultCatalogV1"


class GuardFailure(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GuardFailure(message)


def code_only(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    text = re.sub(r"//.*", "", text)
    return re.sub(r'"(?:\\.|[^"\\])*"', '""', text)


def production_rust(root: Path) -> str:
    rows: list[str] = []
    for path in (root / "src").rglob("*.rs"):
        if MODULE in path.relative_to(root).parents:
            continue
        if "tests" in path.parts or path.name.endswith("_tests.rs"):
            continue
        rows.append(code_only(path.read_text(encoding="utf-8")))
    return "\n".join(rows)


def verify(root: Path) -> dict[str, int]:
    module_root = root / MODULE
    require(module_root.is_dir(), f"missing module: {MODULE}")
    rust_files = sorted(module_root.rglob("*.rs"))
    require(bool(rust_files), "result catalog has no Rust sources")
    sources = {path: path.read_text(encoding="utf-8") for path in rust_files}
    module_code = "\n".join(code_only(text) for text in sources.values())
    production = production_rust(root)
    solver = sources[module_root / "solver.rs"]
    disposition = sources[module_root / "disposition.rs"]

    require(
        module_code.count(f"struct {PRODUCT}") == 1,
        "result catalog product definition count drift",
    )
    require(
        not re.search(r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct " + PRODUCT, solver),
        "sealed result catalog must remain non-Clone",
    )
    require(
        production.count(PRODUCT) == 0,
        "S0 result catalog gained a production producer or consumer",
    )
    require(
        solver.count(".static_declarations()") == 1,
        "solver must derive rows from the single static declaration view",
    )
    require(
        "InstanceBoxMethod" not in code_only("\n".join(sources[path] for path in rust_files if "tests" not in path.parts)),
        "instance namespace entered production result rows",
    )
    require(
        "rows_by_key:" in solver and "body:" not in solver and "body:" not in disposition,
        "result product must retain keys/dispositions, not duplicate bodies",
    )
    for forbidden in (
        "MirBuilder",
        "MirFunction",
        "MirType",
        "ValueId",
        "type_ctx",
        "value_origin_newbox",
        "current_module",
        "function.metadata",
        "GenericLoop",
    ):
        require(forbidden not in module_code, f"forbidden S0 authority entered module: {forbidden}")
    require(
        "expect(" not in code_only(solver) and "unwrap(" not in code_only(solver),
        "solver must close structural drift through typed errors",
    )
    require(
        module_code.count("ExactTrivialScalarAbiV1::classify") == 2,
        "exact-i64 spelling must reuse the existing scalar ABI classifier twice",
    )
    require(
        module_code.count("BareStaticRecoveryDecisionV1::decide") == 0,
        "S0a must not infer final call targets from declaration recovery",
    )
    require(
        "StaticCallTargetAuthorityUnavailable" in module_code,
        "explicit call-target authority boundary is missing",
    )
    require(
        "actual_string_helpers_keeps_skip_ws_exact_and_records_to_i64_design_boundary" in module_code,
        "actual StringHelpers boundary fixture is missing",
    )
    for path, text in sources.items():
        lines = len(text.splitlines())
        require(lines < 800, f"source reached 800 lines: {path.relative_to(root)} ({lines})")
    self_path = root / "tools/checks/lib/callable_result_i64_catalog_s0.py"
    require(
        len(self_path.read_text(encoding="utf-8").splitlines()) < 800,
        "S0 guard reached 800 lines",
    )

    return {
        "product_definitions": 1,
        "production_producers_consumers": 0,
        "static_declaration_views": 1,
        "bare_static_policy_consumers": 0,
        "forbidden_authority_occurrences": 0,
        "line_cap_violations": 0,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    root = Path(parser.parse_args().root).resolve()
    report = verify(root)
    for key, value in report.items():
        print(f"{key}={value}")
    print("summary=green")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GuardFailure as error:
        print(f"[callable-result-i64-catalog-s0] ERROR: {error}")
        raise SystemExit(1)
