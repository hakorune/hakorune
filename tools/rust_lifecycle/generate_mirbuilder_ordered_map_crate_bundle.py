#!/usr/bin/env python3
"""Generate the ordered-map crate-level bundle artifact."""

from __future__ import annotations

import argparse

from mirbuilder_ordered_map_crate_bundle_artifacts import run_ordered_map_crate_bundle_generator


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_ordered_map_crate_bundle_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
