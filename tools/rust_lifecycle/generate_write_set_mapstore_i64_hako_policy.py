#!/usr/bin/env python3
"""Compatibility delegator for the common MapStore artifact generator."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
from generate_write_set_mapstore_route_policy import (  # noqa: E402
    GENERATED,
    SOURCE,
    read_rows,
    render_all,
)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path)
    args = parser.parse_args()
    source = args.source or SOURCE
    print(render_all(read_rows(source))[GENERATED["i64_policy"]], end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
