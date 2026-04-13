import unittest

from src.llvm_py.builders.ipo_build_policy import (
    IpoBuildPolicy,
    apply_ipo_build_policy,
    resolve_ipo_build_policy,
)


class TestIpoBuildPolicy(unittest.TestCase):
    def test_resolve_ipo_build_policy_defaults_off(self):
        policy = resolve_ipo_build_policy()
        self.assertEqual(policy, IpoBuildPolicy(lto_mode="off", pgo_mode="off"))

    def test_apply_ipo_build_policy_is_noop_for_phase272x(self):
        kwargs = {"opt": 2, "cpu": "native"}
        policy = IpoBuildPolicy(lto_mode="off", pgo_mode="off")

        applied = apply_ipo_build_policy(dict(kwargs), policy)

        self.assertEqual(applied, kwargs)


if __name__ == "__main__":
    unittest.main()
