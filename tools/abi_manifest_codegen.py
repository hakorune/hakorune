#!/usr/bin/env python3
"""
Generate AbiAdapterRegistry default table from the docs-side TOML manifest.

Usage:
  python tools/abi_manifest_codegen.py --out lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako
  python tools/abi_manifest_codegen.py --check

The manifest vocabulary follows value-repr-and-abi-manifest-ssot.md.
Only the adapter-default rows needed by AbiAdapterRegistryBox are consumed here.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path
import sys
import tomllib


ROOT = Path(__file__).resolve().parent.parent
DEFAULT_MANIFEST = ROOT / "docs/development/current/main/design/abi-export-manifest-v0.toml"
DEFAULT_OUT = ROOT / "lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako"


@dataclass
class AbiRow:
    box_type: str
    method: str
    symbol: str
    args: list[str]
    ret: str
    arg_ownership: str
    ret_ownership: str
    failure_contract: str
    compat_status: str
    call_shape: str
    unbox: str


def _load_manifest(path: Path) -> list[AbiRow]:
    try:
        data = tomllib.loads(path.read_text())
    except FileNotFoundError as exc:
        raise SystemExit(f"manifest not found: {path}") from exc

    rows = data.get("rows")
    if not isinstance(rows, list):
        raise SystemExit("manifest must contain an array table 'rows'")

    required = [
        "box_type",
        "method",
        "symbol",
        "args",
        "ret",
        "arg_ownership",
        "ret_ownership",
        "failure_contract",
        "compat_status",
    ]

    out: list[AbiRow] = []
    for idx, row in enumerate(rows):
        if not isinstance(row, dict):
            raise SystemExit(f"row {idx} is not a table")
        for key in required:
            if key not in row:
                raise SystemExit(f"row {idx} missing required key '{key}'")
        args_field = row["args"]
        if not isinstance(args_field, list) or not all(isinstance(x, str) for x in args_field):
            raise SystemExit(f"row {idx} field 'args' must be a list of strings")
        call_shape = row.get("call_shape", "h")
        unbox = row.get("unbox", "none")
        out.append(
            AbiRow(
                box_type=str(row["box_type"]),
                method=str(row["method"]),
                symbol=str(row["symbol"]),
                args=list(args_field),
                ret=str(row["ret"]),
                arg_ownership=str(row["arg_ownership"]),
                ret_ownership=str(row["ret_ownership"]),
                failure_contract=str(row["failure_contract"]),
                compat_status=str(row["compat_status"]),
                call_shape=str(call_shape),
                unbox=str(unbox),
            )
        )
    return out


def _render(rows: list[AbiRow], manifest_path: Path) -> str:
    header = """// GENERATED FILE - DO NOT EDIT
// Source: {source}
// Generator: tools/abi_manifest_codegen.py

""".format(source=manifest_path)

    body_lines = ["static box AbiAdapterRegistryDefaultsBox {", "  populate(reg) {"]
    for row in rows:
        body_lines.append(
            f'    reg._put("{row.box_type}", "{row.method}", "{row.symbol}", "{row.call_shape}", "{row.unbox}")'
        )
    body_lines.append("  }")
    body_lines.append("}")
    body_lines.append("")
    return header + "\n".join(body_lines)


def _write_if_changed(dest: Path, content: str) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists() and dest.read_text() == content:
        return
    dest.write_text(content)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Generate AbiAdapterRegistry defaults from manifest")
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--check", action="store_true", help="Fail if generated output differs from --out")
    args = parser.parse_args(argv)

    rows = _load_manifest(args.manifest)
    rendered = _render(rows, args.manifest)

    if args.check:
        if not args.out.exists():
            print(f"missing generated file: {args.out}", file=sys.stderr)
            return 1
        if args.out.read_text() != rendered:
            print("generated output differs; re-run tools/abi_manifest_codegen.py to update", file=sys.stderr)
            return 1
        return 0

    _write_if_changed(args.out, rendered)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
