#!/usr/bin/env python3
"""Stub/fixture tests of publication capture logic, not native Cargo acceptance."""

import argparse
import gzip
import importlib.machinery
import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch


loader = importlib.machinery.SourceFileLoader(
    "publish_crate", str(Path(__file__).with_name("publish-crate")))
spec = importlib.util.spec_from_loader(loader.name, loader)
helper = importlib.util.module_from_spec(spec)
loader.exec_module(helper)


class PublishCrateTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="vpsadmin-publish-helper-")
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.root = self.base / "checkout"
        self.root.mkdir()
        self.source = "a" * 40
        self.head = self.source
        self.version = helper.CARGO_VERSION
        self.lock = b"version = 4\n# actual root input\n"
        self.manifest = (b'[package]\nname = "vpsadmin-client"\nversion = "0.1.0"\n'
                         b'edition = "2024"\nrust-version = "1.95"\n')
        (self.root / "Cargo.toml").write_bytes(self.manifest)
        (self.root / "Cargo.lock").write_bytes(self.lock)
        for name in ["src/lib.rs", "examples/sample.rs"] + [
                "src/generated/runtime/" + str(i) + ".rs" for i in range(8)]:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"// fixture " + name.encode() + b"\n")
        (self.root / "src/generated/.haveapi-generated.json").write_bytes(b"{}\n")
        self.cargo = self.base / "cargo"
        self.cargo.write_bytes(b"stub cargo binary identity\n")
        self.rustc = self.base / "rustc"
        self.rustc.write_bytes(b"stub rustc binary identity\n")
        self.digest = helper.digest(self.lock)
        self.members = {p.relative_to(self.root).as_posix(): p.read_bytes()
                        for p in self.root.rglob("*") if p.is_file()}
        self.members.update({"Cargo.toml.orig": self.manifest,
                             "Cargo.lock": b"version = 4\n# normalized package lock\n",
                             ".cargo_vcs_info.json": json.dumps({
                                 "git": {"sha1": self.source}, "path_in_vcs": ""}).encode()})
        self.archive = self.make_archive()
        self.child_exit = 0
        self.child_calls = []
        self.child_action = None
        self.env = patch.dict(os.environ, {"GITHUB_RUN_ID": "123", "GITHUB_RUN_ATTEMPT": "1"},
                              clear=True)
        self.env.start()
        self.addCleanup(self.env.stop)
        for item in (patch.object(helper.Path, "cwd", return_value=self.root),
                     patch.object(helper, "output", side_effect=self.output),
                     patch.object(helper.shutil, "which", side_effect=lambda n: str(self.base / n)),
                     patch.object(helper.subprocess, "run", side_effect=self.native)):
            item.start()
            self.addCleanup(item.stop)

    def make_archive(self, special=None):
        stream = io.BytesIO()
        with tarfile.open(fileobj=stream, mode="w") as archive:
            for name, data in self.members.items():
                entry = tarfile.TarInfo("vpsadmin-client-0.1.0/" + name)
                entry.size = len(data)
                archive.addfile(entry, io.BytesIO(data))
            if special:
                archive.addfile(special)
        return gzip.compress(stream.getvalue(), mtime=0)

    def output(self, argv):
        if argv == ["git", "rev-parse", "HEAD"]:
            return self.head
        if argv == ["git", "status", "--porcelain", "--untracked-files=all"]:
            return ""
        if argv == ["cargo", "--version"]:
            return self.version
        if argv == ["rustc", "--version"]:
            return "rustc fixture identity"
        self.fail("Unexpected subprocess metadata call")

    def native(self, argv, check):
        self.assertFalse(check)
        self.assertEqual(argv[:2], ["cargo", "--config"])
        build = Path(json.loads(argv[2].split("=", 1)[1]))
        self.assertEqual(argv[3:5], ["publish", "--locked"])
        scratch = build / "package/tmp-crate/vpsadmin-client-0.1.0.crate"
        scratch.parent.mkdir(parents=True, exist_ok=True)
        self.child_calls.append(argv)
        if self.archive is not None:
            scratch.write_bytes(self.archive)
        if self.child_action:
            self.child_action(scratch)
        return subprocess.CompletedProcess(argv, self.child_exit)

    def args(self, mode="candidate", suffix=""):
        return argparse.Namespace(mode=mode, source=self.source, lock_sha256=self.digest,
                                  work_dir=self.base / ("work" + suffix),
                                  evidence_dir=self.base / (mode + suffix),
                                  candidate_dir=self.base / ("candidate" + suffix)
                                  if mode == "publish" else None)

    def run_helper(self, args, expected):
        self.assertEqual(helper.execute(args), expected)
        return json.loads((args.evidence_dir / "identity.json").read_text())

    def candidate(self):
        args = self.args()
        report = self.run_helper(args, 0)
        return args, report

    def test_candidate_and_publish_retain_separate_exact_native_bytes(self):
        candidate, first = self.candidate()
        self.assertFalse((candidate.work_dir / "build/package/tmp-crate" /
                          first["archive_name"]).exists())
        second = self.run_helper(self.args("publish"), 0)
        self.assertEqual(first["crate_sha256"], second["crate_sha256"])
        self.assertEqual(first["archive_validation"]["packaged_rust_files"], 10)
        self.assertNotEqual(first["lock_sha256"],
                            first["archive_validation"]["members_sha256"]["Cargo.lock"])
        for mode in ("candidate", "publish"):
            self.assertEqual((self.base / mode / first["archive_name"]).read_bytes(), self.archive)
        self.assertIn("--dry-run", self.child_calls[0])
        self.assertNotIn("--dry-run", self.child_calls[1])
        self.assertEqual(self.child_calls[0][:-1], self.child_calls[1])
        self.assertNotIn("--no-verify", self.child_calls[0])

    def test_missing_archive_after_success_fails_with_absence_report(self):
        self.archive = None
        report = self.run_helper(self.args(), 1)
        self.assertEqual(report["native_exit"], 0)
        self.assertFalse(report["actual_archive_present"])

    def test_child_failure_present_or_missing_preserves_native_exit(self):
        for present in (True, False):
            with self.subTest(present=present):
                self.archive = self.make_archive() if present else None
                self.child_exit = 23
                args = self.args(suffix=str(present))
                report = self.run_helper(args, 23)
                self.assertEqual(report["native_exit"], 23)
                self.assertEqual(report["actual_archive_present"], present)
                self.assertEqual(report["validation"], "failed")
                if present:
                    self.assertEqual((args.evidence_dir / report["archive_name"]).read_bytes(),
                                     self.archive)

    def test_capture_failure_keeps_original_scratch_and_child_failure(self):
        self.child_exit = 29
        with patch.object(helper.Path, "rename", side_effect=OSError("fixture capture failure")):
            report = self.run_helper(self.args(), 29)
        self.assertEqual(report["native_exit"], 29)
        self.assertEqual(Path(report["scratch_path"]).read_bytes(), self.archive)
        self.assertEqual(report["error"], "OSError")

    def test_report_write_failure_does_not_mask_child_failure(self):
        self.child_exit = 31
        original = helper.Path.open

        def failing(path, *args, **kwargs):
            if path.name == "identity.json" and args and args[0] == "x":
                raise OSError("fixture report failure")
            return original(path, *args, **kwargs)

        with patch.object(helper.Path, "open", failing):
            self.assertEqual(helper.execute(self.args()), 31)
        self.assertTrue((self.base / "candidate/vpsadmin-client-0.1.0.crate").is_file())

    def test_malformed_archive_preserved_before_validation(self):
        self.archive = b"partial native output"
        report = self.run_helper(self.args(), 1)
        self.assertEqual((self.base / "candidate" / report["archive_name"]).read_bytes(),
                         self.archive)
        self.assertEqual(report["crate_sha256"], helper.digest(self.archive))

    def test_rejects_symlink_archive_member_and_preserves_bytes(self):
        link = tarfile.TarInfo("vpsadmin-client-0.1.0/hostile")
        link.type = tarfile.SYMTYPE
        link.linkname = "/outside"
        self.archive = self.make_archive(link)
        report = self.run_helper(self.args(), 1)
        self.assertEqual(report["reason"], "Archive member must be regular")
        self.assertEqual((self.base / "candidate" / report["archive_name"]).read_bytes(),
                         self.archive)

    def test_wrong_source_tool_or_lock_prevents_native_launch(self):
        for field in ("source", "tool", "lock"):
            with self.subTest(field=field):
                self.head = "b" * 40 if field == "source" else self.source
                self.version = "cargo unknown" if field == "tool" else helper.CARGO_VERSION
                args = self.args(suffix=field)
                if field == "lock":
                    args.lock_sha256 = "0" * 64
                report = self.run_helper(args, 1)
                self.assertFalse(report["native_attempted"])
        self.assertEqual(self.child_calls, [])

    def test_candidate_refuses_supplied_publish_credentials(self):
        for name in ("CARGO_REGISTRY_TOKEN", "CARGO_REGISTRIES_CRATES_IO_TOKEN"):
            with self.subTest(name=name), patch.dict(os.environ, {name: "fixture-never-a-token"}):
                self.run_helper(self.args(suffix=name), 1)
        self.assertEqual(self.child_calls, [])

    def test_existing_work_or_evidence_not_overwritten(self):
        args = self.args()
        args.work_dir.mkdir()
        (args.work_dir / "original").write_bytes(b"preserve")
        self.run_helper(args, 1)
        self.assertEqual((args.work_dir / "original").read_bytes(), b"preserve")
        with self.assertRaises(FileExistsError):
            helper.execute(args)
        self.assertEqual(self.child_calls, [])

    def test_publish_rejects_stale_or_symlink_scratch_without_following(self):
        args, report = self.candidate()
        scratch = args.work_dir / "build/package/tmp-crate" / report["archive_name"]
        original = self.base / "untouched"
        original.write_bytes(b"preserve")
        for symlink in (False, True):
            with self.subTest(symlink=symlink):
                if symlink:
                    scratch.symlink_to(original)
                else:
                    scratch.write_bytes(b"stale")
                publish = self.args("publish")
                publish.evidence_dir = self.base / ("publish" + str(symlink))
                self.run_helper(publish, 1)
                scratch.unlink()
        self.assertEqual(len(self.child_calls), 1)
        self.assertEqual(original.read_bytes(), b"preserve")

    def test_symlink_work_path_refused(self):
        args = self.args()
        other = self.base / "outside"
        other.mkdir()
        args.work_dir.symlink_to(other, target_is_directory=True)
        self.run_helper(args, 1)
        self.assertEqual(self.child_calls, [])
        self.assertEqual(list(other.iterdir()), [])

    def test_native_replacing_scratch_with_symlink_is_not_adopted(self):
        original = self.base / "untouched"
        original.write_bytes(b"preserve")

        def replace(scratch):
            scratch.unlink()
            scratch.symlink_to(original)

        self.child_action = replace
        report = self.run_helper(self.args(), 1)
        self.assertEqual(report["reason"], "Owned paths must not contain symlinks")
        self.assertEqual(original.read_bytes(), b"preserve")
        self.assertFalse((self.base / "candidate/vpsadmin-client-0.1.0.crate").exists())

    def test_candidate_source_tool_lock_run_and_name_tampering_prevents_publish(self):
        args, report = self.candidate()
        path = args.evidence_dir / "identity.json"
        for key in ("source", "tool", "lock_sha256", "run_id", "run_attempt", "archive_name"):
            with self.subTest(key=key):
                changed = {**report, key: "wrong"}
                path.write_text(json.dumps(changed))
                publish = self.args("publish")
                publish.evidence_dir = self.base / ("publish-" + key)
                self.run_helper(publish, 1)
        self.assertEqual(len(self.child_calls), 1)

    def test_changed_candidate_bytes_prevent_publish(self):
        args, report = self.candidate()
        (args.evidence_dir / report["archive_name"]).write_bytes(b"changed")
        self.run_helper(self.args("publish"), 1)
        self.assertEqual(len(self.child_calls), 1)

    def test_actual_mismatch_retained_without_replacing_candidate(self):
        args, report = self.candidate()
        original = (args.evidence_dir / report["archive_name"]).read_bytes()
        self.members["extra.txt"] = b"new native bytes"
        self.archive = self.make_archive()
        actual = self.run_helper(self.args("publish"), 1)
        self.assertEqual(actual["reason"], "Native publish archive differs from candidate")
        self.assertEqual((args.evidence_dir / report["archive_name"]).read_bytes(), original)
        self.assertEqual((self.base / "publish" / report["archive_name"]).read_bytes(), self.archive)

    def test_source_change_during_child_fails_after_capture(self):
        self.child_action = lambda _: setattr(self, "head", "b" * 40)
        report = self.run_helper(self.args(), 1)
        self.assertTrue((self.base / "candidate" / report["archive_name"]).is_file())
        self.assertEqual(report["validation"], "failed")

    def test_archive_oracles_reject_vcs_dependency_rust_and_duplicate_member(self):
        original = self.members.copy()
        for bad in ("vcs", "dependency", "rust", "duplicate", "traversal"):
            with self.subTest(oracle=bad):
                self.members = original.copy()
                extra = None
                if bad == "vcs":
                    self.members[".cargo_vcs_info.json"] = b'{"git":{"sha1":"wrong"},"path_in_vcs":""}'
                elif bad == "dependency":
                    self.members["Cargo.toml"] += b'\n[dependencies]\nhaveapi = "1"\n'
                elif bad == "rust":
                    self.members["src/lib.rs"] += b"changed"
                else:
                    extra = tarfile.TarInfo("vpsadmin-client-0.1.0/" +
                                            ("Cargo.toml" if bad == "duplicate" else "../outside"))
                self.archive = self.make_archive(extra)
                self.run_helper(self.args(suffix=bad), 1)


if __name__ == "__main__":
    unittest.main()
