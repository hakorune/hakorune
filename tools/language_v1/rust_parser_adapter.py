#!/usr/bin/env python3
"""Process boundary for independent Rust parser grammar evidence."""

from __future__ import annotations

import json
import pathlib
import subprocess
import tempfile
from typing import Any

if __package__:
    from .rust_witness_projection import RustProjectionError, project_rust_normalized_form
else:
    from rust_witness_projection import RustProjectionError, project_rust_normalized_form


ROOT = pathlib.Path(__file__).resolve().parents[2]


def reject_tag(stderr: str) -> str:
    for token in stderr.replace("[", " ").replace("]", " ").split():
        if token.startswith("parser/"):
            return token.rstrip(".,:;")
    return "parser/implementation_rejected"


def observe_rust_fixture(
    binary: pathlib.Path, fixture: dict[str, Any]
) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="grammar-contract-rust-") as temp_dir:
        root = pathlib.Path(temp_dir)
        source = root / "fixture.hako"
        ast = root / "ast.json"
        source.write_text(
            fixture.get("parser_inventory_source", "") + fixture["source"],
            encoding="utf-8",
        )
        completed = subprocess.run(
            [
                str(binary),
                "--emit-parser-evidence-ast-json",
                str(ast),
                *(
                    ["--grammar-profile", "compat2025"]
                    if fixture["profile"] == "Compat2025"
                    else []
                ),
                str(source),
            ],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=False,
        )
        ast_payload = (
            json.loads(ast.read_text(encoding="utf-8"))
            if completed.returncode == 0
            else None
        )
    if completed.returncode != 0:
        return {
            "accepted": False,
            "normalized_form": None,
            "stable_reject_tag": reject_tag(completed.stderr),
        }
    try:
        normalized_form = project_rust_normalized_form(fixture["row_id"], ast_payload)
    except RustProjectionError as error:
        return {
            "accepted": False,
            "normalized_form": None,
            "stable_reject_tag": error.stable_reject_tag,
        }
    return {
        "accepted": True,
        "normalized_form": normalized_form,
        "stable_reject_tag": "",
    }
