import unittest

from src.llvm_py.phi_wiring.fact_propagation import should_mark_phi_arrayish


class DummyResolver:
    def __init__(self):
        self.array_ids = set()
        self.value_types = {}

    def is_arrayish(self, vid: int) -> bool:
        return int(vid) in self.array_ids


class TestPhiFactPropagation(unittest.TestCase):
    def test_should_mark_phi_arrayish_accepts_raw_json_incoming_order(self):
        resolver = DummyResolver()
        resolver.array_ids.add(7)
        resolver.value_types[7] = {"kind": "handle", "box_type": "ArrayBox"}

        self.assertTrue(should_mark_phi_arrayish(resolver, None, [(7, 0)]))

    def test_should_mark_phi_arrayish_accepts_normalized_incoming_order(self):
        resolver = DummyResolver()
        resolver.array_ids.add(7)
        resolver.value_types[7] = {"kind": "handle", "box_type": "ArrayBox"}

        self.assertTrue(should_mark_phi_arrayish(resolver, None, [(0, 7)]))


if __name__ == "__main__":
    unittest.main()
