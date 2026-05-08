#!/usr/bin/env python3
"""Generate Hako static-data defaults from the backend-private TOML manifest."""

from __future__ import annotations

import argparse
import pathlib
import sys
import tomllib


ROOT = pathlib.Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/development/current/main/design/static-data-manifest-v0.toml"
OUT = ROOT / "lang/src/shared/backend/ll_emit/generated/static_data_defaults.hako"


def q(text: str) -> str:
    return text.replace("\\", "\\\\").replace('"', '\\"')


def emit_string_array_lines(var_name: str, values: list[str], indent: str) -> list[str]:
    out = [f"{indent}local {var_name} = new ArrayBox()"]
    for value in values:
        out.append(f'{indent}{var_name}.push("{q(str(value))}")')
    return out


def render_output() -> str:
    data = tomllib.loads(MANIFEST.read_text())
    rows = data.get("rows", [])
    lines: list[str] = [
        "// Generated from docs/development/current/main/design/static-data-manifest-v0.toml",
        "// Generator: tools/backend_static_data_manifest_codegen.py",
        "",
        "static box StaticDataDefaultsBox {",
        "  rows() {",
        "    local rows = new ArrayBox()",
    ]
    for idx, row in enumerate(rows):
        row_var = f"row_{idx}"
        lines.append(f"    local {row_var} = new MapBox()")
        lines.append(f'    {row_var}.set("symbol", "{q(row["symbol"])}")')
        lines.append(f'    {row_var}.set("element", "{q(row["element"])}")')
        lines.append(f'    {row_var}.set("align", "{int(row["align"])}")')
        lines.append(f'    {row_var}.set("linkage", "{q(row.get("linkage", "private"))}")')
        lines.append(
            f'    {row_var}.set("unnamed_addr", "{1 if row.get("unnamed_addr", False) else 0}")'
        )
        lines.append(f'    {row_var}.set("purpose", "{q(row.get("purpose", ""))}")')
        lines.extend(emit_string_array_lines(f"values_{idx}", row.get("values", []), "    "))
        lines.append(f'    {row_var}.set("values", values_{idx})')
        lines.extend(emit_string_array_lines(f"lanes_{idx}", row.get("lanes", []), "    "))
        lines.append(f'    {row_var}.set("lanes", lanes_{idx})')
        lines.append(f"    rows.push({row_var})")
    lines.extend(
        [
            "    return rows",
            "  }",
            "}",
            "",
            "static box StaticDataDefaultsMain { main(args) { return 0 } }",
        ]
    )
    return "\n".join(lines) + "\n"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if the generated output differs from the checked-in file",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    rendered = render_output()
    if args.check:
        if not OUT.exists():
            raise SystemExit(
                f"[static-data-manifest/check-missing] generated file missing: {OUT}"
            )
        current = OUT.read_text()
        if current != rendered:
            sys.stderr.write(
                "[static-data-manifest/check-drift] "
                f"run {pathlib.Path(__file__).name} to refresh {OUT}\n"
            )
            raise SystemExit(1)
        return
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(rendered)


if __name__ == "__main__":
    main()
