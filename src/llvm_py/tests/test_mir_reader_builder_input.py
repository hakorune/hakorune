import unittest

from src.llvm_py.mir_reader import build_builder_input, normalize_functions_payload


class TestMirReaderBuilderInput(unittest.TestCase):
    def test_normalize_functions_payload_accepts_dict_shape(self):
        mir_json = {
            "functions": {
                "main": {
                    "params": [],
                    "blocks": [{"id": 0, "instructions": []}],
                }
            }
        }
        functions = normalize_functions_payload(mir_json)
        self.assertEqual(len(functions), 1)
        self.assertEqual(functions[0]["name"], "main")

    def test_build_builder_input_collects_functions_user_box_decls_and_call_arities(self):
        mir_json = {
            "user_box_decls": [{"name": "Main"}],
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [{"id": 0, "instructions": []}],
                }
            ],
        }

        def fake_scan(functions):
            self.assertEqual(functions[0]["name"], "main")
            return {"main": 0}

        builder_input = build_builder_input(mir_json, fake_scan)
        self.assertEqual(builder_input.user_box_decls, [{"name": "Main"}])
        self.assertEqual(builder_input.functions[0]["name"], "main")
        self.assertEqual(builder_input.call_arities, {"main": 0})


if __name__ == "__main__":
    unittest.main()
