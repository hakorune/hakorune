#!/usr/bin/env python3
"""Guard the single ordinary FieldSet receipt owner."""

from pathlib import Path
import sys


def main(root: Path) -> int:
    fields = (root / "src/mir/builder/fields.rs").read_text()
    receipt = (root / "src/mir/builder/fields/store_post_success.rs").read_text()

    product = "PreparedOrdinaryFieldStoreAccessSiteV1"
    checks = {
        "product": receipt.count(f"struct {product}"),
        "prepare": receipt.count(f"impl {product}") and receipt.count("pub(super) fn prepare("),
        "commit": receipt.count("pub(super) fn commit("),
        "consumer": fields.count("PreparedOrdinaryFieldStoreAccessSiteV1::prepare("),
        "ordinary_receipt": fields.count("ordinary_receipt"),
        "old_failure_witness": fields.count(
            "ordinary_fieldset_failure_leaves_no_access_site_after_receipt_cutover"
        ),
    }
    if checks["product"] != 1 or checks["prepare"] != 1 or checks["commit"] != 1:
        raise SystemExit(f"receipt owner count mismatch: {checks}")
    if checks["consumer"] != 1 or checks["ordinary_receipt"] < 2:
        raise SystemExit(f"receipt consumer count mismatch: {checks}")
    if checks["old_failure_witness"] != 1:
        raise SystemExit(f"post-success failure witness missing: {checks}")

    for path in [
        root / "src/mir/builder/fields.rs",
        root / "src/mir/builder/fields/store_post_success.rs",
        Path(__file__),
    ]:
        if sum(1 for _ in path.open()) >= 800:
            raise SystemExit(f"source/check file too long: {path}")
    print(
        "[mirbuilder-fieldstore-observe-authority] ok "
        f"product={checks['product']} commit={checks['commit']} "
        f"consumer={checks['consumer']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()))
