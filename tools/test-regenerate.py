#!/usr/bin/env python3
"""Exercise source refusals in disposable SDK and HaveAPI checkout copies."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parent.parent
SOURCE = Path(os.environ["HAVEAPI_SOURCE"]).resolve()


class RegenerationRefusals(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="rust-regenerate-")
        self.addCleanup(self.temp.cleanup)
        base = Path(self.temp.name)
        self.client = base / "client"
        self.source = base / "haveapi"
        (self.client / "tools").mkdir(parents=True)
        (self.client / "schema").mkdir()
        (self.client / "src/generated").mkdir(parents=True)
        shutil.copy2(ROOT / "tools/regenerate", self.client / "tools/regenerate")
        for name in ("provenance.json", "generator.Gemfile.lock"):
            shutil.copy2(ROOT / "schema" / name, self.client / "schema" / name)
        self.output = self.client / "src/generated/sentinel"
        self.output.write_bytes(b"preserve generated output\n")
        subprocess.run(["git", "clone", "--quiet", "--shared", "--no-checkout", str(SOURCE), str(self.source)],
                       check=True, capture_output=True)
        revision = self.record()["generator"]["revision"]
        subprocess.run(["git", "-C", str(self.source), "checkout", "--quiet", "--detach", revision],
                       check=True, capture_output=True)
        self.retained = self.client / "schema/generator.Gemfile.lock"
        self.installed = self.source / "clients/rust/Gemfile.lock"
        shutil.copy2(self.retained, self.installed)

    def record(self):
        return json.loads((self.client / "schema/provenance.json").read_text())

    def change_record(self, section, field):
        record = self.record()
        record[section][field] = "0" * (40 if field == "revision" else 64)
        (self.client / "schema/provenance.json").write_text(json.dumps(record))

    def assert_refuses(self, message):
        locks = [p.read_bytes() if p.exists() else None for p in (self.retained, self.installed)]
        env = dict(os.environ, HAVEAPI_SOURCE=str(self.source))
        result = subprocess.run([str(self.client / "tools/regenerate"), "--check"], cwd=self.client,
                                env=env, capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(message, result.stdout + result.stderr)
        self.assertEqual(list(self.output.parent.iterdir()), [self.output])
        self.assertEqual(self.output.read_bytes(), b"preserve generated output\n")
        self.assertEqual(locks, [p.read_bytes() if p.exists() else None
                                 for p in (self.retained, self.installed)])

    def test_wrong_revision(self):
        self.change_record("generator", "revision")
        self.assert_refuses("HaveAPI revision differs from provenance")

    def test_dirty_source(self):
        with (self.source / "clients/rust/lib/haveapi/rust_client/version.rb").open("a") as f:
            f.write("\n# changed source\n")
        self.assert_refuses("HaveAPI generator sources are dirty")

    def test_missing_retained_lock(self):
        self.retained.unlink()
        self.assert_refuses("prepare the recorded regular lock file")

    def test_different_retained_lock(self):
        self.retained.write_bytes(self.retained.read_bytes() + b"\n")
        self.assert_refuses("Ruby dependency lock differs from provenance")

    def test_missing_installed_lock(self):
        self.installed.unlink()
        self.assert_refuses("prepare the recorded regular lock file")

    def test_different_installed_lock(self):
        self.installed.write_bytes(self.installed.read_bytes() + b"\n")
        self.assert_refuses("Ruby dependency lock differs from provenance")

    def test_symlink_installed_lock(self):
        self.installed.unlink()
        self.installed.symlink_to(self.retained)
        self.assert_refuses("prepare the recorded regular lock file")

    def test_generator_digest(self):
        self.change_record("generator", "source_sha256")
        self.assert_refuses("loaded generator/shared schema sources differ from provenance")

    def test_shared_schema_digest(self):
        record = self.record()
        record["generator"]["shared_schema"]["source_sha256"] = "0" * 64
        (self.client / "schema/provenance.json").write_text(json.dumps(record))
        self.assert_refuses("loaded generator/shared schema sources differ from provenance")

    def test_runtime_digest(self):
        self.change_record("runtime", "sha256")
        self.assert_refuses("maintained runtime differs from provenance")


if __name__ == "__main__":
    unittest.main()
