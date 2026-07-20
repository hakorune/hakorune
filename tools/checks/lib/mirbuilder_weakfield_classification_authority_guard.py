#!/usr/bin/env python3
"""Freeze WEAKFIELD-CLASSIFY0's one-route authority boundary."""

from __future__ import annotations

import sys
from pathlib import Path


TAG = "mirbuilder-weakfield-classification-authority"
ROUTE = "src/mir/builder/weak_field_write_route.rs"
ISSUER = "src/mir/builder/weak_field_write.rs"
FIELDS = "src/mir/builder/fields.rs"
SELF = "tools/checks/lib/mirbuilder_weakfield_classification_authority_guard.py"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] ERROR: {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def production(source: str) -> str:
    return source.split("#[cfg(test)]", maxsplit=1)[0]


def count(source: str, needle: str, expected: int, label: str) -> None:
    actual = source.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def absent(source: str, needle: str, label: str) -> None:
    if needle in source:
        fail(f"{label}: forbidden token remains: {needle}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    route = production(read(root, ROUTE))
    issuer = production(read(root, ISSUER))
    fields = production(read(root, FIELDS))

    count(route, "pub(super) fn prepare_field_write_route_v1(", 1, "route classifier")
    count(fields, "prepare_field_write_route_v1(", 1, "production classifier consumer")
    count(issuer, "pub(super) fn emit_prepared_known_weak_field_write(", 1, "prepared issuer")
    count(fields, "emit_prepared_known_weak_field_write(", 1, "production issuer consumer")
    absent(issuer, "user_box_field_decls", "issuer declaration-registry re-query")
    absent(fields, "emit_known_weak_field_write(", "old bool weak emitter consumer")

    for forbidden, label in (
        ("#[derive(Clone", "prepared route Clone implementation"),
        ("MirBuilder", "prepared route Builder reference"),
        ("WeakFieldWriteSiteId", "prepared route site ID"),
        ("weak_fields_by_box", "legacy weak cache as classifier authority"),
        ("record_field_access_site", "route product metadata mutation"),
    ):
        absent(route, forbidden, label)

    oversized = [
        relative
        for relative in (ROUTE, ISSUER, FIELDS, SELF)
        if len(read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"[{TAG}] ok classifier=1 consumer=1 issuer=1 "
        "registry_requery=0 old_bool_consumer=0"
    )


if __name__ == "__main__":
    main()
