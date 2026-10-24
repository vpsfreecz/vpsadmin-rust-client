#!/usr/bin/env python3
"""Exercise the shared lock restore helper without Cargo or provider access."""

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


class RestoreCargoLockTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="vpsadmin-cargo-lock-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = self.root / "artifact"
        self.source.mkdir()
        self.checkout = self.root / "checkout"
        self.checkout.mkdir()
        self.lock = b"version = 4\n"
        self.digest = hashlib.sha256(self.lock).hexdigest()
        (self.source / "Cargo.lock").write_bytes(self.lock)
        self.identity = {"source": "a" * 40, "run_id": "123", "run_attempt": "1",
                         "lock_sha256": self.digest}
        self.write_identity()
        self.env = {**os.environ, "GITHUB_SHA": "a" * 40, "GITHUB_RUN_ID": "123",
                    "GITHUB_RUN_ATTEMPT": "1"}

    def write_identity(self):
        (self.source / "identity.json").write_text(json.dumps(self.identity))

    def run_helper(self, success, expected_sha=None):
        result = subprocess.run(
            [sys.executable, "-B", str(Path(__file__).with_name("restore-cargo-lock")),
             str(self.source), "--sha256", self.digest if expected_sha is None else expected_sha],
            cwd=self.checkout, env=self.env, capture_output=True, text=True)
        self.assertEqual(result.returncode == 0, success, result.stdout + result.stderr)
        return result

    def assert_no_destination(self):
        self.assertFalse((self.checkout / "Cargo.lock").exists())

    def test_valid_transfer_preserves_original_artifact(self):
        self.run_helper(True)
        self.assertEqual((self.checkout / "Cargo.lock").read_bytes(), self.lock)
        self.assertEqual((self.source / "Cargo.lock").read_bytes(), self.lock)

    def test_rejects_payload_tampering(self):
        (self.source / "Cargo.lock").write_bytes(self.lock + b"changed\n")
        self.run_helper(False)
        self.assert_no_destination()

    def test_rejects_wrong_expected_digest(self):
        self.run_helper(False, expected_sha="0" * 64)
        self.assert_no_destination()

    def test_rejects_each_wrong_identity_field(self):
        for key in self.identity:
            with self.subTest(key=key):
                original = self.identity[key]
                self.identity[key] = "wrong"
                self.write_identity()
                self.run_helper(False)
                self.assert_no_destination()
                self.identity[key] = original

    def test_rejects_extra_identity_field(self):
        self.identity["unexpected"] = True
        self.write_identity()
        self.run_helper(False)
        self.assert_no_destination()

    def test_rejects_extra_artifact_entry(self):
        (self.source / "unexpected").write_text("extra")
        self.run_helper(False)
        self.assert_no_destination()

    def test_rejects_nonregular_artifact_entry(self):
        (self.source / "Cargo.lock").unlink()
        (self.source / "Cargo.lock").mkdir()
        self.run_helper(False)
        self.assert_no_destination()

    def test_rejects_each_artifact_symlink(self):
        for name in ("Cargo.lock", "identity.json"):
            with self.subTest(name=name):
                entry = self.source / name
                original = self.root / name
                entry.rename(original)
                entry.symlink_to(original)
                self.run_helper(False)
                self.assert_no_destination()
                entry.unlink()
                original.rename(entry)

    def test_preserves_any_existing_destination(self):
        for data in (self.lock, b"existing different lock\n"):
            with self.subTest(data=data):
                destination = self.checkout / "Cargo.lock"
                destination.write_bytes(data)
                self.run_helper(False)
                self.assertEqual(destination.read_bytes(), data)

    def test_preserves_symlink_destination(self):
        original = self.root / "existing.lock"
        original.write_bytes(b"untouched\n")
        destination = self.checkout / "Cargo.lock"
        destination.symlink_to(original)
        self.run_helper(False)
        self.assertTrue(destination.is_symlink())
        self.assertEqual(original.read_bytes(), b"untouched\n")


if __name__ == "__main__":
    unittest.main()
