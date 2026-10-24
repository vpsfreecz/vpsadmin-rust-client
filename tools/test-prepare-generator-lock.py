#!/usr/bin/env python3
"""Input-integrity checks using disposable local Git repositories; no downloads."""

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


class PrepareGeneratorLockTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="vpsadmin-generator-lock-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.sdk = self.root / "sdk"
        (self.sdk / "tools").mkdir(parents=True)
        (self.sdk / "schema").mkdir()
        self.helper = self.sdk / "tools/prepare-generator-lock"
        shutil.copyfile(Path(__file__).with_name("prepare-generator-lock"), self.helper)
        self.source = self.root / "haveapi"
        (self.source / "clients/rust").mkdir(parents=True)
        (self.source / ".gitignore").write_text("/clients/rust/Gemfile.lock\n")
        (self.source / "clients/rust/Gemfile").write_text("source 'https://rubygems.org'\n")
        self.git("init", "-q")
        self.git("add", ".gitignore", "clients/rust/Gemfile")
        self.git("-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid",
                 "commit", "-qm", "Fixture")
        revision = self.git("rev-parse", "HEAD").strip()
        self.lock = b"GEM\n  remote: https://rubygems.org/\n\nBUNDLED WITH\n   4.0.23\n"
        self.manifest = {"schema": 1, "source": revision, "version": "0.30.1", "tag": "v0.30.1",
                         "dependency_locks": {"clients/rust/Gemfile.lock": self.lock.decode()}}
        self.generator = {"revision": revision, "version": "0.30.1",
                          "ruby_dependency_lock_sha256": hashlib.sha256(self.lock).hexdigest(),
                          "release_manifest": {"url": "https://github.com/vpsfreecz/haveapi/releases/"
                                               "download/v0.30.1/release-manifest.json"}}
        self.manifest_path = self.root / "release-manifest.json"
        self.write_manifest()

    def git(self, *argv):
        return subprocess.check_output(["git", "-C", str(self.source), *argv], text=True)

    def write_manifest(self):
        data = (json.dumps(self.manifest) + "\n").encode()
        self.manifest_path.write_bytes(data)
        self.generator["release_manifest"]["sha256"] = hashlib.sha256(data).hexdigest()
        self.write_provenance()

    def write_provenance(self):
        (self.sdk / "schema/provenance.json").write_text(json.dumps({"generator": self.generator}))

    def run_helper(self, success, message=""):
        result = subprocess.run([sys.executable, "-B", str(self.helper), str(self.manifest_path)],
                                env={**os.environ, "HAVEAPI_SOURCE": str(self.source)},
                                capture_output=True, text=True)
        self.assertEqual(result.returncode == 0, success, result.stdout + result.stderr)
        if message:
            self.assertIn(message, result.stderr)
        return result

    def test_restores_exact_bytes_and_is_idempotent(self):
        self.run_helper(True)
        self.run_helper(True)
        self.assertEqual((self.source / "clients/rust/Gemfile.lock").read_bytes(), self.lock)
        self.assertEqual(self.git("status", "--porcelain"), "")

    def test_rejects_wrong_manifest_hash(self):
        self.manifest_path.write_bytes(self.manifest_path.read_bytes() + b" ")
        self.run_helper(False, "Release manifest digest differs")

    def test_rejects_wrong_manifest_source_and_version(self):
        for key, value in (("source", "0" * 40), ("version", "0.30.0"), ("tag", "v0.30.0")):
            with self.subTest(key=key):
                original = self.manifest[key]
                self.manifest[key] = value
                self.write_manifest()
                self.run_helper(False, "Release manifest source or version differs")
                self.manifest[key] = original

    def test_rejects_wrong_lock_digest(self):
        self.manifest["dependency_locks"]["clients/rust/Gemfile.lock"] += "changed\n"
        self.write_manifest()
        self.run_helper(False, "Released Ruby lock digest differs")

    def test_rejects_wrong_checkout(self):
        self.generator["revision"] = self.manifest["source"] = "0" * 40
        self.write_manifest()
        self.run_helper(False, "HaveAPI source revision differs")

    def test_rejects_dirty_source(self):
        (self.source / "clients/rust/Gemfile").write_text("changed\n")
        self.run_helper(False, "HaveAPI source must be clean")

    def test_preserves_conflicting_file(self):
        lock = self.source / "clients/rust/Gemfile.lock"
        lock.write_bytes(b"conflict\n")
        self.run_helper(False, "Existing generator lock conflicts")
        self.assertEqual(lock.read_bytes(), b"conflict\n")

    def test_rejects_symlink(self):
        (self.source / "clients/rust/Gemfile.lock").symlink_to(self.manifest_path)
        self.run_helper(False, "Existing generator lock must be a regular file")


if __name__ == "__main__":
    unittest.main()
