import unittest

from src.llvm_py.phi_wiring.tagging import _trivial_phi_alias


class TestPhiTrivialAlias(unittest.TestCase):
    def test_identical_incomings_are_aliasable(self):
        incoming = [[7, 0], [7, 3], [7, 9]]
        self.assertEqual(_trivial_phi_alias(42, incoming), 7)

    def test_self_carry_invariant_is_not_aliasable(self):
        incoming = [[12, 0], [13, 3]]
        self.assertIsNone(_trivial_phi_alias(13, incoming))

    def test_self_only_is_not_aliasable(self):
        incoming = [[5, 0], [5, 1]]
        self.assertIsNone(_trivial_phi_alias(5, incoming))


if __name__ == "__main__":
    unittest.main()
