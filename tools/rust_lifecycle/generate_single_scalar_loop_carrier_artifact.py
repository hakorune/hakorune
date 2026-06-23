#!/usr/bin/env python3
"""Thin entrypoint for the single-scalar-loop-carrier Hako artifact."""

from __future__ import annotations

import argparse

from mirbuilder_single_scalar_loop_carrier_artifacts import run_single_scalar_loop_carrier_artifact_generator


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()
    run_single_scalar_loop_carrier_artifact_generator(check=args.check)


if __name__ == "__main__":
    main()
