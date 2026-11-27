"""Static contract tests for the optional Metal backend.

These tests run on non-Apple CI hosts and do not import PyObjC or execute GPU code.
"""

import ast
from pathlib import Path
import re
import unittest


ROOT = Path(__file__).resolve().parents[1]
DISPATCHER = ROOT / "model_training" / "Dispatcher.py"
SHADER = ROOT / "model_training" / "kernels" / "lmlm_hash.metal"


class MetalDispatchContractTests(unittest.TestCase):
    def test_dispatcher_is_valid_python(self):
        source = DISPATCHER.read_text(encoding="utf-8")
        ast.parse(source, filename=str(DISPATCHER))

    def test_default_shader_is_packaged_beside_dispatcher(self):
        source = DISPATCHER.read_text(encoding="utf-8")
        self.assertIn('"kernels" / "lmlm_hash.metal"', source)
        self.assertTrue(SHADER.is_file())

    def test_shader_exposes_expected_buffer_indices(self):
        source = SHADER.read_text(encoding="utf-8")
        insert = source.split("kernel void hash_insert(", 1)[1].split(
            ") {", 1
        )[0]
        lookup = source.split("kernel void hash_lookup(", 1)[1].split(
            ") {", 1
        )[0]

        self.assertEqual(
            sorted(map(int, re.findall(r"\[\[buffer\((\d+)\)\]\]", insert))),
            list(range(7)),
        )
        self.assertEqual(
            sorted(map(int, re.findall(r"\[\[buffer\((\d+)\)\]\]", lookup))),
            list(range(7)),
        )

    def test_dispatcher_binds_result_buffers_at_shader_indices(self):
        tree = ast.parse(DISPATCHER.read_text(encoding="utf-8"))
        calls = [
            node for node in ast.walk(tree)
            if isinstance(node, ast.Call)
            and isinstance(node.func, ast.Attribute)
            and node.func.attr == "_dispatch"
        ]
        self.assertEqual(len(calls), 2)

        # The fifth positional argument is the indexed-buffer list.
        bindings = []
        for call in calls:
            pairs = ast.literal_eval(call.args[1])
            bindings.append([index for index, _ in pairs])

        self.assertIn([0, 1, 2, 3, 6], bindings)
        self.assertIn([0, 1, 2, 5, 6], bindings)


if __name__ == "__main__":
    unittest.main()
