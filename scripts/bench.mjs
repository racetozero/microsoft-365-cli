// Start-up and run-time comparison of the Node.js CLI for Microsoft 365 and
// the Rust port on commands that need no network.
//
//   cargo build --release
//   node scripts/bench.mjs [--upstream /tmp/cli-microsoft365] [--bin target/release/m365] [--runs 15]
//
// Each command runs `--runs` times per CLI with an isolated HOME; the table
// shows the median wall time.
import { spawnSync } from 'child_process';
import fs from 'fs';
import os from 'os';
import path from 'path';
import { fileURLToPath } from 'url';

const here = path.dirname(fileURLToPath(import.meta.url));
const argv = process.argv.slice(2);
const flag = (n, d) => { const i = argv.indexOf(`--${n}`); return i >= 0 ? argv[i + 1] : d; };
const upstream = path.resolve(flag('upstream', '/tmp/cli-microsoft365'));
const bin = path.resolve(flag('bin', path.join(here, '..', 'target', 'release', 'm365')));
const runs = Number(flag('runs', '15'));

const home = fs.mkdtempSync(path.join(os.tmpdir(), 'm365-bench-'));
fs.mkdirSync(path.join(home, '.config', 'configstore'), { recursive: true });
fs.writeFileSync(path.join(home, '.config', 'configstore', 'cli-m365-config.json'), JSON.stringify({ disableTelemetry: true, prompt: false }));
const project = fs.mkdtempSync(path.join(os.tmpdir(), 'm365-bench-spfx-'));
fs.cpSync(path.join(upstream, 'src/m365/spfx/commands/project/test-projects/spfx-1100-webpart-react'), project, { recursive: true });

const cases = [
  ['m365 version', ['version']],
  ['m365 --help', ['--help']],
  ['m365 spo web get --help', ['spo', 'web', 'get', '--help']],
  ['m365 entra user get (validation error)', ['entra', 'user', 'get', '--id', 'x']],
  ['m365 cli config list', ['cli', 'config', 'list']],
  ['m365 spfx project upgrade --output json', ['spfx', 'project', 'upgrade', '--output', 'json'], project],
  ['m365 spfx project doctor --output json', ['spfx', 'project', 'doctor', '--output', 'json'], project]
];

const env = { ...process.env, HOME: home, USERPROFILE: home, CLIMICROSOFT365_NOUPDATE: '1', NO_COLOR: '1' };
function time(cmd, args, cwd) {
  const t = [];
  for (let i = 0; i < runs; i++) {
    const s = process.hrtime.bigint();
    spawnSync(cmd, args, { cwd: cwd || home, env, stdio: 'ignore' });
    t.push(Number(process.hrtime.bigint() - s) / 1e6);
  }
  t.sort((a, b) => a - b);
  return t[Math.floor(t.length / 2)];
}

console.log(`| Command | Node.js CLI 12.1.0 | m365 | Speed-up |`);
console.log(`| --- | --- | --- | --- |`);
for (const [name, args, cwd] of cases) {
  const node = time(process.execPath, [path.join(upstream, 'dist', 'index.js'), ...args], cwd);
  const rust = time(bin, args, cwd);
  console.log(`| \`${name}\` | ${node.toFixed(0)} ms | ${rust.toFixed(1)} ms | ${(node / rust).toFixed(0)}x |`);
}
fs.rmSync(home, { recursive: true, force: true });
fs.rmSync(project, { recursive: true, force: true });
