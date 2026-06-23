#!/usr/bin/env python3
"""Thin entrypoint for the canonical explicit PHI Hako artifact."""

from __future__ import annotations

import argparse

from mirbuilder_explicit_phi_artifacts import run_explicit_phi_artifact_generator


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()
    run_explicit_phi_artifact_generator(check=args.check)


if __name__ == "__main__":
    main()
