#!/usr/bin/env python3
"""Thin entrypoint for the VariableContext explicit carrier snapshot Hako artifact."""

from __future__ import annotations

import argparse

from mirbuilder_carrier_snapshot_artifacts import run_variable_context_explicit_carrier_snapshot_artifact_generator


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()
    run_variable_context_explicit_carrier_snapshot_artifact_generator(check=args.check)


if __name__ == "__main__":
    main()
