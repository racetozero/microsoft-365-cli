"""Tests for the m365 PGO build and benchmark programs."""

from __future__ import annotations

import tempfile
import unittest
from argparse import Namespace
from pathlib import Path
from unittest.mock import patch

import benchmark_m365_pgo
import build_m365_pgo
from pgo_common import (
    HELD_OUT_PROJECT,
    TRAINING_PROJECT,
    geometric_mean,
    held_out_workloads,
    prepare_fixture,
    training_workloads,
)


class BuildPgoTests(unittest.TestCase):
    def test_native_and_linux_musl_targets_can_run(self) -> None:
        self.assertTrue(
            build_m365_pgo.target_runs_on_host(
                'aarch64-apple-darwin', 'aarch64-apple-darwin'
            )
        )
        self.assertTrue(
            build_m365_pgo.target_runs_on_host(
                'x86_64-unknown-linux-musl', 'x86_64-unknown-linux-gnu'
            )
        )
        self.assertFalse(
            build_m365_pgo.target_runs_on_host(
                'aarch64-unknown-linux-gnu', 'x86_64-unknown-linux-gnu'
            )
        )

    def test_hot_count_parser_requires_positive_95th_percentile(self) -> None:
        summary = '123 functions with count >= 41 account for 95% of the total counts.'
        self.assertEqual(build_m365_pgo.parse_hot_count(summary), 41)
        with self.assertRaises(RuntimeError):
            build_m365_pgo.parse_hot_count('no detailed summary')
        with self.assertRaises(RuntimeError):
            build_m365_pgo.parse_hot_count(
                '1 functions with count >= 0 account for 95% of the total counts.'
            )

    def test_optimized_flags_keep_existing_linker_flags(self) -> None:
        profile = Path('profiles') / 'm365.profdata'
        flags = build_m365_pgo.optimized_rustflags('-C linker=rust-lld', profile, 27)
        self.assertIn('-C linker=rust-lld', flags)
        self.assertIn(f'-Cprofile-use={profile}', flags)
        self.assertIn('--profile-summary-hot-count=27', flags)

    def test_github_environment_is_appended(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'github-env'
            path.write_text('EXISTING=1\n', encoding='utf-8')
            build_m365_pgo.write_github_environment(path, '-Cprofile-use=profile')
            self.assertEqual(
                path.read_text(encoding='utf-8'),
                'EXISTING=1\nRUSTFLAGS=-Cprofile-use=profile\n',
            )


class FixtureTests(unittest.TestCase):
    def test_fixture_does_not_forward_credentials_or_m365_settings(self) -> None:
        with (
            tempfile.TemporaryDirectory() as temporary,
            patch.dict(
                'os.environ',
                {
                    'GH_TOKEN': 'private',
                    'M365_SELF_UPDATE_COMMAND': 'evil',
                    'CLIMICROSOFT365_ENTRAAPPID': 'private',
                    'PATH': '/usr/bin',
                },
                clear=True,
            ),
        ):
            _, environment = prepare_fixture(Path(temporary))
        self.assertNotIn('GH_TOKEN', environment)
        self.assertNotIn('M365_SELF_UPDATE_COMMAND', environment)
        self.assertNotIn('CLIMICROSOFT365_ENTRAAPPID', environment)
        self.assertEqual(environment['M365_NO_UPDATE_NOTIFIER'], '1')
        self.assertEqual(environment['PATH'], '/usr/bin')

    def test_fixture_writes_both_projects(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            prepare_fixture(Path(temporary))
            for project in (TRAINING_PROJECT, HELD_OUT_PROJECT):
                self.assertTrue((Path(temporary) / project / '.yo-rc.json').is_file())

    def test_held_out_workloads_are_not_training_workloads(self) -> None:
        # plain start-up is measured in both sets, as in kiss
        startup = {('--version',), ('--help',)}
        trained = {workload.arguments for workload in training_workloads()}
        for workload in held_out_workloads():
            if workload.arguments not in startup:
                self.assertNotIn(workload.arguments, trained, workload.name)


class BenchmarkTests(unittest.TestCase):
    def test_summary_and_change(self) -> None:
        summary = benchmark_m365_pgo.summarize([1, 2, 3, 4, 5])
        self.assertEqual(summary.median_ns, 3)
        self.assertEqual(summary.p95_ns, 5)
        self.assertAlmostEqual(benchmark_m365_pgo.percent_change(100, 90), -10.0)

    def test_target_directory_resolves_matched_binaries(self) -> None:
        arguments = Namespace(
            target_dir=Path('target/pgo'),
            target='x86_64-pc-windows-msvc',
            baseline=None,
            pgo=None,
        )
        baseline, pgo = benchmark_m365_pgo.resolve_binaries(arguments)
        root = arguments.target_dir.resolve()
        target = arguments.target
        self.assertEqual(baseline, root / 'baseline' / target / 'dist' / 'm365.exe')
        self.assertEqual(pgo, root / 'optimized' / target / 'dist' / 'm365.exe')

    def test_geometric_mean(self) -> None:
        self.assertAlmostEqual(geometric_mean([1.0, 4.0]), 2.0)
        with self.assertRaises(ValueError):
            geometric_mean([])


if __name__ == '__main__':
    unittest.main()
