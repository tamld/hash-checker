import sys
import tempfile
import unittest
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[1]
SRC_PATH = PROJECT_ROOT / "src"
if str(SRC_PATH) not in sys.path:
    sys.path.insert(0, str(SRC_PATH))

from hash_checker.core import compute_hash, verify_hash


class TestHashChecker(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.TemporaryDirectory()
        self.test_file_path = Path(self.temp_dir.name) / "sample.txt"
        self.test_file_path.write_text("This is a test file for the hash checker.")

    def tearDown(self):
        self.temp_dir.cleanup()

    def test_compute_hash_sha256(self):
        expected_hash = "19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811"
        actual_hash = compute_hash(self.test_file_path, "sha256")
        self.assertEqual(actual_hash, expected_hash)

    def test_compute_hash_md5(self):
        expected_hash = "109788a70f52a60437d3c8867124ca72"
        actual_hash = compute_hash(self.test_file_path, "md5")
        self.assertEqual(actual_hash, expected_hash)

    def test_verify_hash_success(self):
        expected_hash = "109788a70f52a60437d3c8867124ca72"
        matches, computed = verify_hash(self.test_file_path, expected_hash, "md5")
        self.assertTrue(matches)
        self.assertEqual(computed, expected_hash)

    def test_verify_hash_file_not_found(self):
        with self.assertRaises(FileNotFoundError):
            verify_hash(Path(self.temp_dir.name) / "missing.txt", "deadbeef")


if __name__ == "__main__":
    unittest.main()
