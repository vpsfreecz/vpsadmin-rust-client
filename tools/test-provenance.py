#!/usr/bin/env python3
"""Check that retained source exports cannot silently lose their identity."""

import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parent.parent


class SourceProvenance(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="rust-provenance-")
        self.addCleanup(self.temp.cleanup)
        self.client = Path(self.temp.name)
        files = ["tools/check-provenance", "generator.toml", "src/generated/.haveapi-generated.json"]
        files += ["schema/" + name for name in (
            "api-7.0.json", "inventory.json", "provenance.json", "generator.Gemfile.lock",
            "source-provenance.json", "tooling.Gemfile.lock",
        )]
        for relative in files:
            destination = self.client / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)

    def check(self, message=None):
        result = subprocess.run([str(self.client / "tools/check-provenance")],
                                capture_output=True, text=True)
        if message is None:
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        else:
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(message, result.stdout + result.stderr)

    def change_source(self, mutate, *, retain_digest=True):
        path = self.client / "schema/source-provenance.json"
        source = json.loads(path.read_text())
        mutate(source)
        path.write_text(json.dumps(source))
        if retain_digest:
            record_path = self.client / "schema/provenance.json"
            record = json.loads(record_path.read_text())
            record["source_export"]["provenance_sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
            record_path.write_text(json.dumps(record))

    def test_saved_export_passes(self):
        self.check()

    def test_changed_source_receipt(self):
        self.change_source(lambda source: source.update(locale="cs"), retain_digest=False)
        self.check("source provenance digest differs")

    def test_changed_exporter_lock(self):
        path = self.client / "schema/tooling.Gemfile.lock"
        path.write_bytes(path.read_bytes() + b"\n")
        self.check("exporter dependency lock differs")

    def test_wrong_api_revision(self):
        self.change_source(lambda source: source["source"].update(revision="0" * 40))
        self.check("API source revision differs")

    def test_wrong_source_schema(self):
        self.change_source(lambda source: source.update(schema_sha256="0" * 64))
        self.check("source schema digest differs")

    def test_missing_plugin(self):
        self.change_source(lambda source: source["plugins"].pop(next(iter(source["plugins"]))))
        self.check("source plugin inventory differs")

    def test_invalid_fixture_hash(self):
        self.change_source(lambda source: source["inputs"].update({"api/db/schema.rb": "invalid"}))
        self.check("invalid source input hashes")


if __name__ == "__main__":
    unittest.main()
