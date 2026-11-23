"""Shared deterministic workloads for m365 PGO training and measurement.

Every workload runs offline. The release binary has no HTTP mock, so the
profiles cover what m365 does on the local machine: argument parsing and
validation, help, output formatting, configuration, and the SharePoint
Framework rule engine (`spfx project doctor` and `upgrade`), which is the
largest CPU cost in the tool.
"""

from __future__ import annotations

import gzip
import json
import os
import platform
import statistics
import subprocess
from collections.abc import Iterable
from dataclasses import dataclass
from pathlib import Path

REPOSITORY_ROOT = Path(__file__).resolve().parent.parent
SENSITIVE_ENV_MARKERS = ('API_KEY', 'CREDENTIAL', 'PASSWORD', 'SECRET', 'TOKEN')
TRAINING_PROJECT = 'spfx-training'
HELD_OUT_PROJECT = 'spfx-held-out'


@dataclass(frozen=True, slots=True)
class Workload:
    """One process invocation, its exit code and expected output."""

    name: str
    arguments: tuple[str, ...]
    expected_output: str
    repetitions: int = 1
    exit_code: int = 0
    project: str | None = None


def _write(path: Path, content: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    text = content if isinstance(content, str) else json.dumps(content, indent=2)
    path.write_text(text + '\n', encoding='utf-8', newline='\n')


def write_spfx_project(root: Path, version: str, framework: str) -> None:
    """A minimal SharePoint Framework web part project of one version."""

    spfx = {
        '@microsoft/sp-core-library': version,
        '@microsoft/sp-lodash-subset': version,
        '@microsoft/sp-office-ui-fabric-core': version,
        '@microsoft/sp-property-pane': version,
        '@microsoft/sp-webpart-base': version,
    }
    dependencies = dict(spfx)
    if framework == 'react':
        dependencies |= {'react': '16.8.5', 'react-dom': '16.8.5'}
    _write(
        root / '.yo-rc.json',
        {
            '@microsoft/generator-sharepoint': {
                'version': version,
                'libraryName': 'pgo-fixture',
                'libraryId': '6c0a6a5e-3e8a-4d55-9f32-7f2d1b7c9a10',
                'environment': 'spo',
                'packageManager': 'npm',
                'solutionName': 'pgo-fixture',
                'componentType': 'webpart',
                'framework': framework,
                'isCreatingSolution': True,
            }
        },
    )
    _write(
        root / 'package.json',
        {
            'name': 'pgo-fixture',
            'version': '0.0.1',
            'private': True,
            'main': 'lib/index.js',
            'engines': {'node': '>=0.10.0'},
            'scripts': {
                'build': 'gulp bundle',
                'clean': 'gulp clean',
                'test': 'gulp test',
            },
            'dependencies': dependencies,
            'devDependencies': {
                '@microsoft/rush-stack-compiler-3.3': '0.3.5',
                '@microsoft/sp-build-web': version,
                '@microsoft/sp-module-interfaces': version,
                '@microsoft/sp-tslint-rules': version,
                '@microsoft/sp-webpart-workbench': version,
                '@types/chai': '3.4.34',
                '@types/mocha': '2.2.38',
                'ajv': '~5.2.2',
                'gulp': '~3.9.1',
            },
        },
    )
    _write(
        root / 'tsconfig.json',
        {
            'extends': './node_modules/@microsoft/rush-stack-compiler-3.3/includes/tsconfig-web.json',
            'compilerOptions': {
                'target': 'es5',
                'forceConsistentCasingInFileNames': True,
                'module': 'esnext',
                'moduleResolution': 'node',
                'jsx': 'react',
                'declaration': True,
                'sourceMap': True,
                'outDir': 'lib',
                'lib': ['es5', 'dom', 'es2015.collection'],
            },
            'include': ['src/**/*.ts', 'src/**/*.tsx'],
            'exclude': ['node_modules', 'lib'],
        },
    )
    _write(
        root / 'gulpfile.js',
        "'use strict';\n\nconst build = require('@microsoft/sp-build-web');\n\n"
        'build.initialize(require(\'gulp\'));',
    )
    _write(
        root / 'config' / 'config.json',
        {
            '$schema': 'https://developer.microsoft.com/json-schemas/spfx-build/config.2.0.schema.json',
            'version': '2.0',
            'bundles': {
                'hello-world-web-part': {
                    'components': [
                        {
                            'entrypoint': './lib/webparts/helloWorld/HelloWorldWebPart.js',
                            'manifest': './src/webparts/helloWorld/HelloWorldWebPart.manifest.json',
                        }
                    ]
                }
            },
            'externals': {},
            'localizedResources': {
                'HelloWorldWebPartStrings': 'lib/webparts/helloWorld/loc/{locale}.js'
            },
        },
    )
    _write(
        root / 'config' / 'package-solution.json',
        {
            '$schema': 'https://developer.microsoft.com/json-schemas/spfx-build/package-solution.schema.json',
            'solution': {
                'name': 'pgo-fixture-client-side-solution',
                'id': '6c0a6a5e-3e8a-4d55-9f32-7f2d1b7c9a10',
                'version': '1.0.0.0',
                'includeClientSideAssets': True,
                'isDomainIsolated': False,
            },
            'paths': {'zippedPackage': 'solution/pgo-fixture.sppkg'},
        },
    )
    _write(
        root / 'config' / 'serve.json',
        {
            '$schema': 'https://developer.microsoft.com/json-schemas/core-build/serve.schema.json',
            'port': 4321,
            'https': True,
            'initialPage': 'https://localhost:5432/workbench',
            'api': {'port': 5432, 'entryPath': 'node_modules/@microsoft/sp-webpart-workbench/lib/api/'},
        },
    )
    _write(
        root / 'config' / 'write-manifests.json',
        {
            '$schema': 'https://developer.microsoft.com/json-schemas/spfx-build/write-manifests.schema.json',
            'cdnBasePath': '<!-- PATH TO CDN -->',
        },
    )
    _write(
        root / 'config' / 'copy-assets.json',
        {
            '$schema': 'https://developer.microsoft.com/json-schemas/spfx-build/copy-assets.schema.json',
            'deployCdnPath': 'temp/deploy',
        },
    )
    webpart = root / 'src' / 'webparts' / 'helloWorld'
    _write(
        webpart / 'HelloWorldWebPart.manifest.json',
        {
            '$schema': 'https://developer.microsoft.com/json-schemas/spfx/client-side-web-part-manifest.schema.json',
            'id': '0f7d4c3e-8b6a-4f0e-9a1d-2c5b7e9f1a20',
            'alias': 'HelloWorldWebPart',
            'componentType': 'WebPart',
            'version': '*',
            'manifestVersion': 2,
            'requiresCustomScript': False,
            'preconfiguredEntries': [
                {
                    'groupId': '5c03119e-3074-46fd-976b-c60198311f70',
                    'group': {'default': 'Other'},
                    'title': {'default': 'HelloWorld'},
                    'description': {'default': 'HelloWorld description'},
                    'officeFabricIconFontName': 'Page',
                    'properties': {'description': 'HelloWorld'},
                }
            ],
        },
    )
    _write(
        webpart / 'HelloWorldWebPart.ts',
        "import { Version } from '@microsoft/sp-core-library';\n"
        "import { BaseClientSideWebPart } from '@microsoft/sp-webpart-base';\n\n"
        'export default class HelloWorldWebPart extends BaseClientSideWebPart<{}> {\n'
        '  public render(): void {\n'
        "    this.domElement.innerHTML = 'Hello';\n"
        '  }\n\n'
        '  protected get dataVersion(): Version {\n'
        "    return Version.parse('1.0');\n"
        '  }\n'
        '}',
    )


def prepare_fixture(root: Path) -> tuple[Path, dict[str, str]]:
    """Create an isolated, logged-out m365 home and two SPFx projects."""

    home = root / 'home'
    work = root / 'work'
    _write(
        home / '.config' / 'configstore' / 'cli-m365-config.json',
        {'prompt': False, 'disableTelemetry': True},
    )
    work.mkdir(parents=True, exist_ok=True)
    write_spfx_project(root / TRAINING_PROJECT, '1.10.0', 'react')
    write_spfx_project(root / HELD_OUT_PROJECT, '1.9.1', 'none')

    environment = {
        key: value
        for key, value in os.environ.items()
        if not any(marker in key.upper() for marker in SENSITIVE_ENV_MARKERS)
        and not key.upper().startswith(('M365_', 'CLIMICROSOFT365_'))
    }
    environment.update(
        {
            'HOME': str(home),
            'USERPROFILE': str(home),
            'XDG_CONFIG_HOME': str(home / '.config'),
            'APPDATA': str(home / 'AppData' / 'Roaming'),
            'LOCALAPPDATA': str(home / 'AppData' / 'Local'),
            'M365_NO_UPDATE_NOTIFIER': '1',
            'NO_COLOR': '1',
        }
    )
    return work, environment


def training_workloads() -> tuple[Workload, ...]:
    return (
        Workload('version', ('--version',), 'm365', repetitions=4),
        Workload('version_text', ('version', '--output', 'text'), 'v', repetitions=2),
        Workload('help', ('--help',), 'Usage:', repetitions=3),
        Workload('suite_help', ('spo', '--help'), 'SharePoint', repetitions=2),
        Workload('group_help', ('spo', 'web', '--help'), 'spo web', repetitions=2),
        Workload('command_help', ('spo', 'web', 'get', '--help'), 'Usage:', repetitions=3),
        Workload('help_options', ('entra', 'user', 'get', '-h', 'options'), 'Options:', repetitions=2),
        Workload(
            'agent_help',
            ('teams', 'message', 'send', '--help', '--agent'),
            '"ok":true',
            repetitions=2,
        ),
        Workload(
            'invalid_uuid', ('entra', 'user', 'get', '--id', 'x'), 'Invalid UUID',
            repetitions=3, exit_code=1,
        ),
        Workload(
            'invalid_url', ('spo', 'web', 'get', '--url', 'notaurl'), 'not a valid',
            repetitions=2, exit_code=1,
        ),
        Workload(
            'option_set',
            ('spo', 'list', 'get', '--webUrl', 'https://contoso.sharepoint.com', '--id', '1', '--title', 'x'),
            'option',
            repetitions=2, exit_code=1,
        ),
        Workload('logged_out', ('entra', 'user', 'list'), 'Log in', repetitions=2, exit_code=1),
        Workload('config_list', ('cli', 'config', 'list', '--output', 'json'), '{', repetitions=2),
        Workload('status', ('status',), 'Logged out', repetitions=2),
        Workload(
            'doctor_json', ('spfx', 'project', 'doctor', '--output', 'json'), '[',
            repetitions=3, project=TRAINING_PROJECT,
        ),
        Workload(
            'upgrade_json', ('spfx', 'project', 'upgrade', '--output', 'json'), 'FN001001',
            repetitions=3, project=TRAINING_PROJECT,
        ),
        Workload(
            'upgrade_md', ('spfx', 'project', 'upgrade', '--output', 'md'), 'FN001001',
            repetitions=2, project=TRAINING_PROJECT,
        ),
    )


def held_out_workloads() -> tuple[Workload, ...]:
    return (
        Workload('startup_version', ('--version',), 'm365'),
        Workload('root_help', ('--help',), 'Usage:'),
        Workload('command_help', ('outlook', 'mail', 'send', '--help'), 'Usage:'),
        Workload(
            'validation_error', ('planner', 'task', 'get', '--id', 'x', '--title', 'y'), 'Error',
            exit_code=1,
        ),
        Workload(
            'spfx_doctor', ('spfx', 'project', 'doctor', '--output', 'text'), 'npm dedupe',
            project=HELD_OUT_PROJECT,
        ),
        Workload(
            'spfx_upgrade',
            ('spfx', 'project', 'upgrade', '--toVersion', '1.18.0', '--output', 'json'),
            'FN001001',
            project=HELD_OUT_PROJECT,
        ),
    )


def run_workload(
    binary: Path,
    workload: Workload,
    *,
    cwd: Path,
    environment: dict[str, str],
    profile_directory: Path | None = None,
) -> subprocess.CompletedProcess[str]:
    child_environment = environment.copy()
    if profile_directory is not None:
        child_environment['LLVM_PROFILE_FILE'] = str(
            profile_directory / f'm365-{workload.name}-%m-%p.profraw'
        )
    if workload.project is not None:
        cwd = cwd.parent / workload.project
    completed = subprocess.run(
        [str(binary), *workload.arguments],
        cwd=cwd,
        env=child_environment,
        stdin=subprocess.DEVNULL,
        text=True,
        encoding='utf-8',
        capture_output=True,
        check=False,
        timeout=60,
    )
    output = completed.stdout + completed.stderr
    if completed.returncode != workload.exit_code:
        raise RuntimeError(
            f'{workload.name} returned {completed.returncode}, '
            f'expected {workload.exit_code}: {output.strip()[:500]}'
        )
    if workload.expected_output not in output:
        raise RuntimeError(
            f'{workload.name} did not contain {workload.expected_output!r}: '
            f'{output.strip()[:500]}'
        )
    return completed


def executable_size(path: Path) -> tuple[int, int]:
    data = path.read_bytes()
    return len(data), len(gzip.compress(data, compresslevel=9, mtime=0))


def host_description() -> str:
    return f'{platform.system()} {platform.machine()}'


def geometric_mean(values: Iterable[float]) -> float:
    values = tuple(values)
    if not values or any(value <= 0 for value in values):
        raise ValueError('geometric mean needs positive values')
    return statistics.geometric_mean(values)
