#!/usr/bin/env node
// Parity checker: compares the Rust `m365` with the upstream Node CLI.
//
//   cargo build -p m365 --features parity
//   node scripts/parity/check.mjs [--upstream /tmp/cli-microsoft365] [--bin target/debug/m365]
//        [--layers meta,help,validation,trace] [--only "spo web"] [--jobs 8] [--verbose]
//
// Layers:
//   meta        every upstream command (and alias) is registered in the Rust
//               binary, with the hooks its validation needs; the committed
//               command tables equal the code generated from the checkout.
//   help        every command's own help is complete: `--help` shows its usage,
//               options and examples; `--help --agent` is JSON with the options.
//   validation  invalid and missing options produce the same stdout, stderr and
//               exit code (logged out, prompting disabled).
//   trace       every documented example runs against the same mock server:
//               the HTTP requests (method, URL, body, key headers), stdout,
//               stderr and exit code must be identical.
//
// The checker never touches your login: every run gets a temporary HOME.
// It exits 0 only when every command passes every selected layer.
import { spawn, execFileSync } from 'child_process';
import fs from 'fs';
import http from 'http';
import os from 'os';
import path from 'path';
import { fileURLToPath, pathToFileURL } from 'url';
import { createRequire } from 'module';
const require = createRequire(import.meta.url);

const here = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(here, '..', '..');
const argv = process.argv.slice(2);
const flag = (name, def) => {
  const i = argv.indexOf(`--${name}`);
  return i >= 0 ? argv[i + 1] : def;
};
const has = name => argv.includes(`--${name}`);
const upstream = path.resolve(flag('upstream', '/tmp/cli-microsoft365'));
const bin = path.resolve(flag('bin', path.join(repo, 'target', 'debug', 'm365')));
const layers = flag('layers', 'meta,help,validation,trace').split(',');
const only = flag('only', '');
const jobs = Number(flag('jobs', String(Math.max(2, os.cpus().length))));
const verbose = has('verbose');
const reportPath = path.resolve(flag('report', path.join(repo, 'target', 'parity', 'report.json')));
// the command list comes from a fresh extraction of the upstream checkout
const manifestText = execFileSync(process.execPath, [path.join(repo, 'scripts', 'extract-upstream.mjs'), upstream], { maxBuffer: 1 << 28 }).toString();
const manifest = JSON.parse(manifestText);
let commands = manifest.commands;
if (only) { const pre = only.split(','); commands = commands.filter(c => pre.some(o => c.name === o || c.name.startsWith(o + ' ') || c.aliases.some(a => a.startsWith(o)))); }

// ---------------------------------------------------------------- fixtures
const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'm365-parity-'));
const fakeJwt = (payload) => {
  const b = o => Buffer.from(JSON.stringify(o)).toString('base64url');
  return `${b({ alg: 'none', typ: 'JWT' })}.${b(payload)}.sig`;
};
const userToken = fakeJwt({
  upn: 'john@contoso.onmicrosoft.com', oid: 'fe36f75e-c103-410b-a18a-2bf6df06ac3a', tid: '9b1b1e42-794b-4c71-93ac-5ed92488b67f',
  scp: 'AllSites.FullControl Sites.Read.All User.Read', idtyp: 'user', app_displayname: 'CLI for M365', aud: 'https://graph.microsoft.com'
});
function writeHome(dir, loggedIn) {
  fs.mkdirSync(path.join(dir, '.config', 'configstore'), { recursive: true });
  fs.writeFileSync(path.join(dir, '.config', 'configstore', 'cli-m365-config.json'),
    JSON.stringify({ prompt: false, disableTelemetry: true, telemetryClientId: '00000000-0000-0000-0000-000000000000' }, null, '\t'));
  if (loggedIn) {
    const conn = {
      active: true, name: 'fe36f75e-c103-410b-a18a-2bf6df06ac3a', identityName: 'john@contoso.onmicrosoft.com',
      identityId: 'fe36f75e-c103-410b-a18a-2bf6df06ac3a', identityTenantId: '9b1b1e42-794b-4c71-93ac-5ed92488b67f',
      authType: 'deviceCode', certificateType: 0,
      accessTokens: { 'https://graph.microsoft.com': { expiresOn: '2099-01-01T00:00:00.000Z', accessToken: userToken } },
      spoUrl: 'https://contoso.sharepoint.com', spoTenantId: '9b1b1e42-794b-4c71-93ac-5ed92488b67f',
      appId: '31359c7f-bd7e-475c-86db-fdb8c937548e', tenant: 'common', cloudType: 'Public'
    };
    fs.writeFileSync(path.join(dir, '.cli-m365-connection.json'), JSON.stringify(conn));
    fs.writeFileSync(path.join(dir, '.cli-m365-all-connections.json'), JSON.stringify([conn]));
  }
}

// ---------------------------------------------------------------- mock server
const traces = new Map();
const MOCK_ID = '00000000-0000-0000-0000-000000000001';
const GUID_RE = /[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}/g;
const FIXTURE_IDS = ['fe36f75e-c103-410b-a18a-2bf6df06ac3a', '9b1b1e42-794b-4c71-93ac-5ed92488b67f', '31359c7f-bd7e-475c-86db-fdb8c937548e'];
// One "chimera" record that looks like most Graph, SharePoint and Power
// Platform entities, so that commands follow their main path.
function item() {
  return {
    id: MOCK_ID, Id: MOCK_ID, ID: 1, displayName: 'Mock', DisplayName: 'Mock', Title: 'Mock', title: 'Mock', name: 'Mock', Name: 'Mock',
    ServerRelativeUrl: '/sites/mock', Url: 'https://contoso.sharepoint.com/sites/mock', webUrl: 'https://contoso.sharepoint.com/sites/mock',
    userPrincipalName: 'john@contoso.onmicrosoft.com', mail: 'john@contoso.onmicrosoft.com', appId: MOCK_ID, '@odata.etag': 'W/"mock"',
    servicePrincipalNames: [], appRoles: [], oauth2PermissionScopes: [], requiredResourceAccess: [],
    // Power Platform: the environment lookup every Dataverse command starts with
    properties: { displayName: 'Mock', linkedEnvironmentMetadata: { instanceApiUrl: 'https://contoso.crm.dynamics.com' } }
  };
}
// A modern page whose canvas exercises the ClientSidePage model: page
// settings, a two-column section with text, an emphasised collapsible
// section with a web part, a vertical section and a background control.
let pageCanvas = '';
try {
  const pm = await import(pathToFileURL(path.join(upstream, 'dist/m365/spo/commands/page/page.mock.js')).href);
  pageCanvas = `<div>${pm.mockPageSettingsHTML}${pm.mockTwoColumnsSectionHTML(1, false, false, true, false)}${pm.mockOneColumnSectionHTML(2, true, true, false, true)}${pm.mockVerticalSectionHTML(false, false, true, false)}${pm.mockBackgroundControlHTML}</div>`;
}
catch { /* no built upstream: pages fall back to the chimera item */ }
// The same page as a JSON canvas, which the page control, section add,
// header set and web part commands parse from the page's own
// `CanvasContent1` (the HTML one lives in `ListItemAllFields`): text, web
// part, empty column, vertical section, background and page settings.
const SPC = { htmlStrings: {}, searchablePlainTexts: {}, imageSources: {}, links: {} };
let pageCanvasJson = '';
if (pageCanvas) {
  pageCanvasJson = JSON.stringify([
    { controlType: 4, id: 'b3d6ebc9-6c7b-4e1e-99c1-4c08b682e8c1', position: { zoneIndex: 1, sectionIndex: 1, sectionFactor: 6, layoutIndex: 1, controlIndex: 1 }, emphasis: {}, displayMode: 2, innerHTML: '<p>Hello</p>', editorType: 'CKEditor' },
    { controlType: 3, id: '3ede60d3-dc2c-438b-b5bf-cc40bb2351e1', position: { zoneIndex: 1, sectionIndex: 2, sectionFactor: 6, layoutIndex: 1, controlIndex: 1 }, webPartId: 'e377ea37-9047-43b9-8cdb-a761be2f8e09', emphasis: {}, displayMode: 2, webPartData: { id: 'e377ea37-9047-43b9-8cdb-a761be2f8e09', instanceId: '3ede60d3-dc2c-438b-b5bf-cc40bb2351e1', title: 'Bing Maps', description: 'Map', dataVersion: '1.0', properties: { zoom: 5 }, serverProcessedContent: SPC } },
    { controlType: 3, id: 'fe36f75e-c103-410b-a18a-2bf6df06ac3a', position: { zoneIndex: 1, sectionIndex: 2, sectionFactor: 6, layoutIndex: 1, controlIndex: 2 }, webPartId: 'd1d91016-032f-456d-98a4-721247c305e8', emphasis: {}, displayMode: 2, webPartData: { id: 'd1d91016-032f-456d-98a4-721247c305e8', instanceId: 'fe36f75e-c103-410b-a18a-2bf6df06ac3a', title: 'Image', dataVersion: '1.0', properties: {} } },
    { id: '9b1b1e42-794b-4c71-93ac-5ed92488b67f', position: { zoneIndex: 2, sectionIndex: 1, sectionFactor: 12, layoutIndex: 1 }, emphasis: {}, displayMode: 2 },
    { id: '9b1b1e42-794b-4c71-93ac-5ed92488b67e', position: { zoneIndex: 2, sectionIndex: 2, sectionFactor: 12, layoutIndex: 1 }, emphasis: {}, displayMode: 2 },
    { id: '9b1b1e42-794b-4c71-93ac-5ed92488b67d', position: { zoneIndex: 2, sectionIndex: 3, sectionFactor: 12, layoutIndex: 1 }, emphasis: {}, displayMode: 2 },
    { controlType: 4, id: '31359c7f-bd7e-475c-86db-fdb8c937548e', position: { zoneIndex: 3, sectionIndex: 1, sectionFactor: 12, layoutIndex: 2, controlIndex: 1 }, emphasis: {}, displayMode: 2, innerHTML: '<p>Vertical</p>' },
    { controlType: 14, webPartData: { properties: { zoneBackground: {} }, serverProcessedContent: SPC, dataVersion: '1.0' } },
    { controlType: 0, pageSettingsSlice: { isDefaultDescription: true, isDefaultThumbnail: true } }
  ]);
}
// `getclientsidewebparts()`: every standard web part plus the docs' custom one.
const WEB_PART_IDS = ['e377ea37-9047-43b9-8cdb-a761be2f8e09', 'd1d91016-032f-456d-98a4-721247c305e8', '3ede60d3-dc2c-438b-b5bf-cc40bb2351e1'];
const PAGE_URL_RE = /\/_api\/sitepages\/pages(\/GetByUrl\(|\()|SitePages\/[^']*'\)\?\$expand=ListItemAllFields\/ClientSideApplicationId|GetFileByServerRelativePath\(DecodedUrl=@f\)\?\$expand=ListItemAllFields&@f='[^']*sitepages(\/|%2F)/i;
function pageItem() {
  return {
    ...item(), CanvasContent1: pageCanvasJson, LayoutWebpartsContent: '[]', IsPageCheckedOutToCurrentUser: true, PageLayoutType: 'Article',
    ListItemAllFields: { ...item(), ClientSideApplicationId: 'b6917cb1-93a0-4b97-a84d-7cf49975d4ec', CanvasContent1: pageCanvas, PageLayoutType: 'Article', CommentsDisabled: false }
  };
}
function respond(req) {
  const u = req.url || '';
  if (pageCanvas && PAGE_URL_RE.test(u) && req.method !== 'DELETE') {
    return { status: 200, headers: { 'content-type': 'application/json' }, body: JSON.stringify(pageItem()) };
  }
  const json = (status, body) => ({ status, headers: { 'content-type': 'application/json' }, body: JSON.stringify(body) });
  if (pageCanvas && /\/_api\/web\/lists\/SitePages\/rootfolder\/files\?/i.test(u)) return json(200, { value: [pageItem()] });
  if (pageCanvas && /\/_api\/Web\/RootFolder\?\$select=WelcomePage/i.test(u)) return json(200, { WelcomePage: 'SitePages/home.aspx' });
  if (pageCanvas && /\/_api\/web\/getclientsidewebparts\(\)/i.test(u)) {
    return json(200, { value: WEB_PART_IDS.map((id, i) => ({ Id: i === 1 ? `{${id.toUpperCase()}}` : id, Manifest: JSON.stringify({ preconfiguredEntries: [{ title: { default: `Part ${i}` }, description: { default: `Description ${i}` }, properties: { n: i, nested: { a: 1 } } }] }) })) });
  }
  if (/\/_api\/contextinfo/i.test(u)) {
    return json(200, { FormDigestValue: '0x0000,01 Jan 2020 00:00:00 -0000', FormDigestTimeoutSeconds: 1800, WebFullUrl: 'https://contoso.sharepoint.com', SiteFullUrl: 'https://contoso.sharepoint.com' });
  }
  if (/ProcessQuery/i.test(u)) {
    return json(200, [{ SchemaVersion: '15.0.0.0', LibraryVersion: '16.0.0.0', ErrorInfo: null, TraceCorrelationId: MOCK_ID }]);
  }
  if (/\/\$batch/.test(u)) {
    let reqs = [];
    try { reqs = JSON.parse(req.body || '{}').requests || []; } catch { }
    return json(200, { responses: reqs.map(r => ({ id: r.id, status: r.method === 'GET' ? 200 : 204, body: r.method === 'GET' ? { ...item(), value: [item()] } : undefined })) });
  }
  if (req.method === 'DELETE') return { status: 204, headers: {}, body: '' };
  return json(200, { ...item(), value: [item()] });
}
const server = http.createServer((req, res) => {
  let body = '';
  req.on('data', c => { body += c; });
  req.on('end', () => {
    const m = req.url.match(/^\/run\/([^/]+)\/__m365$/);
    const desc = JSON.parse(body || '{}');
    if (m) {
      if (!traces.has(m[1])) traces.set(m[1], []);
      traces.get(m[1]).push(normalizeRequest(desc));
    }
    res.writeHead(200, { 'content-type': 'application/json' });
    res.end(JSON.stringify(respond(desc)));
  });
});
await new Promise(r => server.listen(0, '127.0.0.1', r));
const mockBase = `http://127.0.0.1:${server.address().port}`;

const keyHeaders = ['accept', 'content-type', 'if-match', 'x-http-method', 'x-requestdigest', 'odata-version', 'prefer', 'consistencylevel', 'x-anonymous', 'x-resource'];
function normalizeRequest(d) {
  const headers = {};
  for (const k of keyHeaders) if (d.headers && d.headers[k] !== undefined) headers[k] = d.headers[k];
  let body = d.body;
  if (typeof body === 'string') {
    try { body = JSON.stringify(JSON.parse(body)); } catch { /* keep text */ }
  }
  return { method: d.method, url: d.url, headers, body: body ?? null };
}

// ---------------------------------------------------------------- process runner
let runCounter = 0;
// belt and braces: no-op `open`/`xdg-open` first on PATH, so nothing a test
// runs can launch a browser (e.g. `cli issue` opening GitHub)
const noBrowserDir = fs.mkdtempSync(path.join(os.tmpdir(), 'm365-nobrowser-'));
for (const b of ['open', 'xdg-open']) fs.writeFileSync(path.join(noBrowserDir, b), '#!/bin/sh\nexit 0\n', { mode: 0o755 });
function run(kind, args, { home, cwd, mock, timeout = 30000 }) {
  const id = `${kind}-${++runCounter}`;
  const env = { ...process.env, HOME: home, USERPROFILE: home, CLIMICROSOFT365_NOUPDATE: '1', BROWSER: '/usr/bin/true', M365_NO_BROWSER: '1', PATH: `${noBrowserDir}${path.delimiter}${process.env.PATH}`, NO_COLOR: '1', FORCE_COLOR: '0' };
  delete env.XDG_CONFIG_HOME;
  delete env.M365_AGENT;
  delete env.CLIMICROSOFT365_ENTRAAPPID;
  delete env.CLIMICROSOFT365_TENANT;
  // every run goes to the mock server so nothing reaches the network
  env.M365_PARITY_MOCK = `${mockBase}/run/${id}`;
  env.M365_PARITY_TOKEN = userToken;
  const [cmd, cmdArgs] = kind === 'up'
    ? [process.execPath, [path.join(here, 'upstream-harness.mjs'), upstream, ...args]]
    : [bin, args];
  return new Promise(resolve => {
    const p = spawn(cmd, cmdArgs, { cwd, env, stdio: ['ignore', 'pipe', 'pipe'] });
    let out = '', err = '';
    p.stdout.on('data', d => { out += d; });
    p.stderr.on('data', d => { err += d; });
    const t = setTimeout(() => p.kill('SIGKILL'), timeout);
    p.on('close', code => {
      clearTimeout(t);
      const tr = traces.get(id) || [];
      traces.delete(id);
      resolve({ code, out, err, trace: tr });
    });
  });
}

function normalizeOutput(s, sandbox) {
  return s
    .split(sandbox).join('<SANDBOX>')
    .replace(/\d+(\.\d+)?ms/g, '<ms>')
    .replace(/"nodeVersion": "[^"]*"/g, '"nodeVersion": "<node>"')
    .replace(/\r\n/g, '\n');
}

async function pool(items, fn) {
  const results = new Array(items.length);
  let next = 0;
  await Promise.all(Array.from({ length: jobs }, async () => {
    while (next < items.length) {
      const i = next++;
      results[i] = await fn(items[i], i);
    }
  }));
  return results;
}

// ---------------------------------------------------------------- comparisons
const report = { upstreamCommit: null, layers: {}, commands: {} };
const fail = (cmd, layer, detail) => {
  const c = report.commands[cmd] ||= { pass: true, failures: [] };
  c.pass = false;
  c.failures.push({ layer, ...detail });
};
const pass = cmd => { report.commands[cmd] ||= { pass: true, failures: [] }; };

function sandbox(loggedIn) {
  const dir = fs.mkdtempSync(path.join(tmpRoot, 's-'));
  const home = path.join(dir, 'home');
  const cwd = path.join(dir, 'cwd');
  fs.mkdirSync(home);
  fs.mkdirSync(cwd);
  writeHome(home, loggedIn);
  return { dir, home, cwd };
}

// [name, data] of each zip entry (central directory order)
function unzipEntries(buf) {
  const zlib = require('zlib');
  let eocd = buf.length - 22;
  while (eocd >= 0 && buf.readUInt32LE(eocd) !== 0x06054b50) eocd--;
  if (eocd < 0) return [['<not a zip>', buf]];
  const count = buf.readUInt16LE(eocd + 10);
  let p = buf.readUInt32LE(eocd + 16);
  const out = [];
  for (let i = 0; i < count; i++) {
    const method = buf.readUInt16LE(p + 10), csize = buf.readUInt32LE(p + 20);
    const nlen = buf.readUInt16LE(p + 28), xlen = buf.readUInt16LE(p + 30), clen = buf.readUInt16LE(p + 32);
    const local = buf.readUInt32LE(p + 42);
    const name = buf.toString('utf8', p + 46, p + 46 + nlen);
    const start = local + 30 + buf.readUInt16LE(local + 26) + buf.readUInt16LE(local + 28);
    const raw = buf.subarray(start, start + csize);
    out.push([name, method === 8 ? zlib.inflateRawSync(raw) : raw]);
    p += 46 + nlen + xlen + clen;
  }
  return out;
}

// A file tree as `path\ncontent` entries (binary files by size and hash),
// for commands that write into the project they run in.
function tree(dir) {
  const out = [];
  const walk = d => {
    for (const e of fs.readdirSync(d, { withFileTypes: true }).sort((x, y) => x.name < y.name ? -1 : 1)) {
      const p = path.join(d, e.name);
      if (e.isDirectory()) { walk(p); continue; }
      const buf = fs.readFileSync(p);
      const rel = path.relative(dir, p);
      // archives: their entries, in order, with contents
      if (/\.(sppkg|zip)$/.test(e.name)) {
        // names may hold generated GUIDs: compare entries by masked name
        const entries = unzipEntries(buf).map(([n, d]) => `  ${n.replace(GUID_RE, '<guid>')}\n${d.includes(0) ? `<binary ${d.length} bytes>` : d.toString('utf8')}`).sort();
        out.push(`${rel}\n${entries.join('\n')}`);
        continue;
      }
      const binary = buf.includes(0);
      out.push(`${rel}\n${binary ? `<binary ${buf.length} bytes>` : buf.toString('utf8')}`);
    }
  };
  if (fs.existsSync(dir)) walk(dir);
  return out.join('\n\u0000\n');
}

// `spfx project upgrade` points at the Teams icons in its install folder;
// the port writes them to a cache folder: compare both as <SPFX_ASSETS>
const assetDirs = [
  path.join(fs.realpathSync(upstream), 'dist/m365/spfx/commands/project/project-upgrade/assets'),
  path.join(upstream, 'dist/m365/spfx/commands/project/project-upgrade/assets'),
  path.join(os.tmpdir(), 'm365-spfx-assets')
];
const maskAssets = t => assetDirs.reduce((s, d) => s.split(d).join('<SPFX_ASSETS>'), t).replace(/cli-spfx[A-Za-z0-9]{6}/g, 'cli-spfx<random>');

// arguments that name a group but no command (`spo orgassetslibrary --x y`, a
// docs example without its verb) print group help, which is the port's own
const allNames = new Set(manifest.commands.flatMap(c => [c.name, ...c.aliases]));
function groupCall(args) {
  const words = args.filter(a => !String(a).startsWith('-'));
  for (let n = words.length; n > 0; n--) if (allNames.has(words.slice(0, n).join(' '))) return false;
  return true;
}

async function compare(cmdName, layer, args, { loggedIn = false, mock = false, strictStderr = true, project, files = false } = {}) {
  const a = sandbox(loggedIn);
  const b = sandbox(loggedIn);
  if (project) {
    fs.cpSync(project, a.cwd, { recursive: true });
    fs.cpSync(project, b.cwd, { recursive: true });
  }
  const [up, rs] = await Promise.all([
    run('up', args, { home: a.home, cwd: a.cwd, mock }),
    run('rs', args, { home: b.home, cwd: b.cwd, mock })
  ]);
  // concurrent requests (Promise.all) may arrive in any order: compare sorted
  const norm = (r, s) => ({ code: r.code, out: normalizeOutput(r.out, s.dir), err: normalizeOutput(r.err, s.dir), trace: JSON.stringify(r.trace.map(t => JSON.stringify(t)).sort()).split(s.dir).join('<SANDBOX>'), files: files ? tree(s.cwd).split(s.dir).join('<SANDBOX>') : '' });
  // values generated at run time (new GUIDs, "now") cannot match: mask GUIDs
  // and ISO timestamps that the command line does not contain
  const known = new Set([...(args.join(' ').match(GUID_RE) || []).map(g => g.toLowerCase()), MOCK_ID, ...FIXTURE_IDS]);
  const mask = t => maskAssets(t).replace(GUID_RE, g => known.has(g.toLowerCase()) ? g : '<guid>').replace(/Group\d{1,6}\b/g, 'Group<random>').replace(/(\\*"mailNickname\\*":\s*\\*"[^"\\]*?)\d{1,4}(\\*")/g, '$1<random>$2').replace(/(\\*"password\\*":\s*\\*")[^"\\]{15}(\\*")/g, '$1<random>$2').replace(/\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d{3}Z/g, d => args.join(' ').includes(d) || d.startsWith('2099-') ? d : '<now>');
  const u0 = norm(up, a), r0 = norm(rs, b);
  const u = { ...u0, out: mask(u0.out), err: mask(u0.err), trace: mask(u0.trace), files: mask(u0.files) };
  const r = { ...r0, out: mask(r0.out), err: mask(r0.err), trace: mask(r0.trace), files: mask(r0.files) };
  // PARITY_DUMP=file appends every compared case (debugging aid)
  if (process.env.PARITY_DUMP) fs.appendFileSync(process.env.PARITY_DUMP, JSON.stringify({ layer, args: args.join(' '), up: u, rs: r }) + '\n');
  const diffs = [];
  // Upstream crashed with a JavaScript TypeError on the mock data, or stopped
  // at an interactive prompt (stdin is not a terminal): the Rust port must
  // also fail there, but its message cannot match the Node internals.
  const jsCrash = /Cannot read properties of (undefined|null)|is not a function|is not iterable|Cannot convert undefined or null/.test(up.err);
  const promptStop = /ExitPromptError|User force closed the prompt/.test(up.err);
  // both hung (an upstream retry loop the mock never ends) and were killed
  const bothHung = up.code === null && rs.code === null;
  // only the stderr text is exempt: the port must also fail, print the same
  // stdout and send the same requests (a hung loop may differ in length)
  const sameRun = u.out === r.out && (!mock || u.trace === r.trace) && u.files === r.files;
  const tolerated = (bothHung && u.out === r.out)
    || (jsCrash && up.code !== 0 && rs.code !== 0 && sameRun)
    || (promptStop && up.code !== 0 && rs.code !== 0 && sameRun);
  if (tolerated) {
    report.tolerated = (report.tolerated || 0) + 1;
    (report.toleratedCases ??= {})[cmdName] = [...(report.toleratedCases[cmdName] || []), { layer, args: args.join(' '), why: bothHung ? 'hung' : jsCrash ? 'js-crash' : 'prompt' }];
    pass(cmdName);
    fs.rmSync(a.dir, { recursive: true, force: true });
    fs.rmSync(b.dir, { recursive: true, force: true });
    return true;
  }
  if (u.code !== r.code) diffs.push(`exit ${u.code} != ${r.code}`);
  // an unknown command prints the root or group help after the error; help is
  // the port's own now, so only the error and the exit code are compared
  const notFound = re => /Command '[^']*' was not found\./.test(re.err);
  if (u.out !== r.out && !(notFound(u) && notFound(r)) && !groupCall(args)) diffs.push('stdout');
  if (strictStderr && u.err !== r.err) diffs.push('stderr');
  if (mock && u.trace !== r.trace) diffs.push('requests');
  if (u.files !== r.files) diffs.push('files');
  fs.rmSync(a.dir, { recursive: true, force: true });
  fs.rmSync(b.dir, { recursive: true, force: true });
  if (diffs.length) {
    fail(cmdName, layer, { args: args.join(' '), ...(args.project ? { project: path.basename(args.project) } : {}), diffs, upstream: verbose ? u : clip(u), rust: verbose ? r : clip(r), ...(u.files !== r.files ? { files: filesDiff(u.files, r.files) } : {}) });
    return false;
  }
  pass(cmdName);
  return true;
}
const CLIP = +(process.env.PARITY_CLIP || 600);
const clip = r => ({ code: r.code, out: r.out.slice(0, CLIP), err: r.err.slice(0, CLIP), trace: r.trace.slice(0, CLIP * 2) });
// file trees are long: report the region around the first difference
function filesDiff(x, y) {
  let i = 0;
  while (i < x.length && x[i] === y[i]) i++;
  const from = Math.max(0, x.lastIndexOf('\n\u0000\n', i));
  return { upstream: x.slice(from, i + CLIP), rust: y.slice(from, i + CLIP) };
}

// shell-like splitting of documented examples
function splitArgs(line) {
  const out = [];
  let cur = '', q = null, any = false;
  for (let i = 0; i < line.length; i++) {
    const c = line[i];
    if (q) {
      if (c === q) q = null;
      else if (c === '\\' && q === '"' && (line[i + 1] === '"' || line[i + 1] === '\\')) cur += line[++i];
      else cur += c;
    } else if (c === '"' || c === "'") { q = c; any = true; }
    else if (c === ' ' || c === '\t') { if (cur || any) out.push(cur); cur = ''; any = false; }
    else if (c === '`' ) { cur += c; }
    else cur += c;
  }
  if (cur || any) out.push(cur);
  return out;
}

// extra-cases.txt: more `m365 ...` lines (text output, verbose and edge
// paths) traced alongside a command's docs examples
const EXTRA_CASES = fs.existsSync(path.join(here, 'extra-cases.txt'))
  ? fs.readFileSync(path.join(here, 'extra-cases.txt'), 'utf8').split('\n').filter(l => l.startsWith('m365 '))
  : [];
function examples(cmd) {
  const file = path.join(upstream, 'docs', 'docs', 'cmd', cmd.help);
  const text = fs.existsSync(file) ? fs.readFileSync(file, 'utf8') : '';
  const start = text.indexOf('## Examples');
  const rest = start < 0 ? '' : text.slice(start + 11);
  const end = rest.search(/^## /m);
  const section = end >= 0 ? rest.slice(0, end) : rest;
  return [...section.split('\n'), ...EXTRA_CASES.filter(l => l.startsWith(`m365 ${cmd.name} `))].map(l => l.trim()).filter(l => l.startsWith('m365 ')).map(l => splitArgs(l).slice(1))
    .filter(a => !a.some(x => /[|<>$]/.test(x) && x.length === 1));
}

// ------------------------------------------------ spfx project fixtures
// Each upstream test project is copied into both sandboxes and the command
// runs inside it; stdout, requests and the resulting file tree must match.
const SPFX_PROJECTS = path.join(upstream, 'src/m365/spfx/commands/project/test-projects');
const spfxProjects = fs.existsSync(SPFX_PROJECTS) ? fs.readdirSync(SPFX_PROJECTS).sort() : [];
// every project for the main reports, a sample (every 6th plus the special
// ones) for the variants
const spfxSample = spfxProjects.filter((p, i) => i % 6 === 0 || !/-(webpart-(react|nolib|ko|optionaldeps)|applicationcustomizer|fieldcustomizer-react|listviewcommandset|ace|formcustomizer-(react|nolib))$/.test(p));
function spfxCases(cmd) {
  const withProject = (projects, argLists) => projects.flatMap(p => argLists.map(a => Object.assign([...cmd.name.split(' '), ...a], { project: path.join(SPFX_PROJECTS, p) })));
  switch (cmd.name) {
    case 'spfx project upgrade':
      return [
        ...withProject(spfxProjects, [['--output', 'json']]),
        ...withProject(spfxSample, [['--output', 'text'], ['--output', 'md'], ['--output', 'tour'], ['--output', 'csv'], ['--packageManager', 'pnpm', '--output', 'text'], ['--shell', 'bash', '--output', 'text'], ['--shell', 'cmd', '--output', 'md'], ['--toVersion', '1.18.0', '--output', 'json'], ['--output', 'json', '--verbose']])
      ];
    case 'spfx project doctor':
      return [
        ...withProject(spfxProjects, [['--output', 'json']]),
        ...withProject(spfxSample, [['--output', 'text'], ['--output', 'md'], ['--output', 'tour'], ['--output', 'csv'], ['--packageManager', 'pnpm', '--output', 'text']])
      ];
    case 'spfx project externalize':
      return withProject(spfxSample, [['--output', 'json'], ['--output', 'text'], ['--output', 'md']]);
    case 'spfx project rename':
      return withProject(spfxSample, [['--newName', 'Contoso Rename'], ['--newName', 'x', '--generateNewId']]);
    case 'spfx project github workflow add':
      return withProject(spfxSample, [[], ['--name', 'Deploy', '--branchName', 'dev', '--manuallyTrigger', '--skipFeatureDeployment', '--overwrite', '--loginMethod', 'user', '--scope', 'sitecollection', '--siteUrl', 'https://contoso.sharepoint.com/sites/a']]);
    case 'spfx project azuredevops pipeline add':
      return withProject(spfxSample, [[], ['--name', 'Deploy', '--branchName', 'dev', '--skipFeatureDeployment', '--loginMethod', 'user', '--scope', 'sitecollection', '--siteUrl', 'https://contoso.sharepoint.com/sites/a']]);
    case 'spfx package generate':
      return [
        [...cmd.name.split(' '), '--webPartTitle', 'Amsterdam weather', '--webPartDescription', 'Shows weather', '--name', 'aw', '--html', '<h1 class="x">Hi</h1>\n<p>there</p>', '--enableForTeams', 'all', '--allowTenantWideDeployment'],
        // (no --verbose/--debug: they list the temp files, whose order
        // depends on the generated GUIDs in their names)
        [...cmd.name.split(' '), '--webPartTitle', 'My "quoted" part!', '--webPartDescription', 'd', '--name', 'p', '--html', 'x', '--enableForTeams', 'tab', '--exposePageContextGlobally', '--developerName', 'Me'],
        [...cmd.name.split(' '), '-t', 'T', '-d', 'D', '-n', 'n', '--html', 'h', '--enableForTeams', 'personalApp', '--exposeTeamsContextGlobally', '--developerMpnId', '123']
      ];
    case 'spfx project permissions grant':
      return withProject(spfxSample, [[], ['--output', 'text']]);
    default:
      return [];
  }
}

function validationCases(cmd) {
  const words = cmd.name.split(' ');
  const cases = [[...words], [...words, '--zzzUnknown', 'x']];
  for (const o of cmd.options) {
    if (['query', 'output', 'debug', 'verbose'].includes(o.name)) continue;
    if (o.type === 'number') cases.push([...words, `--${o.name}`, 'abc']);
    else if (o.type === 'boolean') cases.push([...words, `--${o.name}`, 'maybe']);
    else if (o.autocomplete) cases.push([...words, `--${o.name}`, 'notAChoice']);
    else cases.push([...words, `--${o.name}`, 'invalid value']);
  }
  cases.push([...words, '--output', 'xml']);
  return cases;
}

// ---------------------------------------------------------------- layers
const t0 = Date.now();
if (layers.includes('meta')) {
  const listing = execFileSync(bin, [], { env: { ...process.env, M365_PARITY_LIST: '1', HOME: tmpRoot } }).toString();
  const impl = JSON.parse(listing);
  const byName = new Map(impl.map(s => [s.name, s]));
  let ok = 0;
  for (const c of commands) {
    const s = byName.get(c.name);
    const problems = [];
    if (!s) problems.push('not implemented');
    else {
      if (s.needsField && !s.field) problems.push('missing field-check hook');
      if (s.needsRefine && !s.refine) problems.push('missing refine hook');
      if (s.needsValidate && !s.validate) problems.push('missing legacy validate hook');
    }
    if (problems.length) fail(c.name, 'meta', { problems });
    else { pass(c.name); ok++; }
  }
  // the committed command tables equal the code generated from the checkout
  try {
    const json = path.join(tmpRoot, 'commands.json');
    const rs = path.join(tmpRoot, 'manifest.rs');
    fs.writeFileSync(json, manifestText);
    execFileSync('cargo', ['run', '-q', '-p', 'm365-codegen', '--', json, rs], { cwd: repo, stdio: ['ignore', 'ignore', 'pipe'] });
    const same = fs.readFileSync(rs, 'utf8') === fs.readFileSync(path.join(repo, 'crates', 'm365-core', 'src', 'generated', 'manifest.rs'), 'utf8');
    report.layers.manifestCurrent = same;
    if (!same) console.log('meta: crates/m365-core/src/generated/manifest.rs differs from the code generated from the upstream checkout (run m365-codegen)');
  } catch (e) {
    report.layers.manifestCurrent = false;
    console.log(`meta: extraction failed: ${e.message}`);
  }
  report.layers.meta = { pass: ok, total: commands.length };
  console.log(`meta: ${ok}/${commands.length} commands implemented with their hooks`);
}

async function layer(name, casesOf, opts) {
  const all = [];
  for (const c of commands) for (const a of casesOf(c)) all.push({ c, a });
  let ok = 0;
  await pool(all, async ({ c, a }) => { if (await compare(c.name, name, a, opts(a))) ok++; });
  const failedCmds = new Set(Object.entries(report.commands).filter(([, v]) => v.failures.some(f => f.layer === name)).map(([k]) => k));
  report.layers[name] = { cases: all.length, pass: ok, commandsPassing: commands.length - failedCmds.size, total: commands.length };
  console.log(`${name}: ${ok}/${all.length} cases, ${commands.length - failedCmds.size}/${commands.length} commands`);
}

// help is the port's own (atl style), so this layer checks that it is
// complete rather than equal to upstream: every command and alias answers
// `--help` (exit 0, its usage line, every option) and `--help --agent`
// (valid JSON describing the same options)
if (layers.includes('help')) {
  const all = [];
  for (const c of commands) for (const n of [c.name, ...c.aliases]) all.push({ c, n });
  let ok = 0;
  await pool(all, async ({ c, n }) => {
    const sb = sandbox(false);
    const own = c.options.filter(o => o.long && !['query', 'output', 'debug', 'verbose'].includes(o.long)).map(o => o.long);
    const text = await run('rs', [...n.split(' '), '--help'], { home: sb.home, cwd: sb.cwd });
    const agent = await run('rs', [...n.split(' '), '--help', '--agent'], { home: sb.home, cwd: sb.cwd });
    fs.rmSync(sb.dir, { recursive: true, force: true });
    const problems = [];
    if (text.code !== 0) problems.push(`--help exited ${text.code}`);
    if (!text.out.includes(`Usage: m365 ${c.name}`)) problems.push('no usage line');
    for (const o of own) if (!text.out.includes(`--${o}`)) problems.push(`--${o} missing`);
    if (!/\nExamples:\n/.test(text.out)) problems.push('no examples');
    try {
      const j = JSON.parse(agent.out);
      if (!j.ok || j.data.name !== c.name) problems.push('agent help is not ok for this command');
      const names = (j.data.options || []).map(o => o.name);
      for (const o of own) if (!names.includes(o)) problems.push(`agent help misses --${o}`);
      for (const o of j.data.options || []) if (!o.description) problems.push(`--${o.name} has no description`);
    } catch { problems.push('--help --agent is not JSON'); }
    if (problems.length) fail(c.name, 'help', { args: `${n} --help`, problems });
    else { pass(c.name); ok++; }
  });
  const failed = new Set(Object.entries(report.commands).filter(([, v]) => v.failures.some(f => f.layer === 'help')).map(([k]) => k));
  report.layers.help = { cases: all.length, pass: ok, commandsPassing: commands.length - failed.size, total: commands.length };
  console.log(`help: ${ok}/${all.length} cases, ${commands.length - failed.size}/${commands.length} commands`);
}
if (layers.includes('validation')) {
  await layer('validation', c => [...validationCases(c), ...examples(c).filter(a => !a.includes('--debug'))], () => ({}));
}
if (layers.includes('trace')) {
  await layer('trace', c => examples(c), a => ({ loggedIn: true, mock: true, strictStderr: !a.includes('--debug') && !a.includes('--verbose') }));
}
if (layers.includes('fixtures')) {
  await layer('fixtures', c => spfxCases(c), a => ({ loggedIn: true, mock: true, strictStderr: !a.includes('--debug') && !a.includes('--verbose'), project: a.project, files: true }));
}

server.close();
fs.rmSync(tmpRoot, { recursive: true, force: true });
const failing = Object.entries(report.commands).filter(([, v]) => !v.pass);
report.summary = { commands: commands.length, passing: commands.length - failing.length, seconds: Math.round((Date.now() - t0) / 1000) };
fs.mkdirSync(path.dirname(reportPath), { recursive: true });
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
for (const [name, v] of failing.slice(0, verbose ? Infinity : 25)) {
  console.log(`\nFAIL ${name}`);
  for (const f of v.failures.slice(0, 3)) console.log(`  [${f.layer}] ${f.args || ''} ${JSON.stringify(f.diffs || f.problems)}`);
}
if (report.tolerated) console.log(`(${report.tolerated} cases where upstream crashed on mock data or stopped at a prompt and the port also failed)`);
console.log(`\n${report.summary.passing}/${report.summary.commands} commands pass all selected layers (${layers.join(', ')}); report: ${path.relative(repo, reportPath)}`);
process.exit(failing.length === 0 && report.layers.manifestCurrent !== false ? 0 : 1);
